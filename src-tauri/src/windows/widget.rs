//! The always-on-top parrot widget.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Mutex, PoisonError};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use tauri::{
    AppHandle, Manager, Monitor, PhysicalPosition, PhysicalSize, WebviewUrl, WebviewWindow,
    WebviewWindowBuilder, WindowEvent,
};

use super::layout::{self, Corner, Rect};
use crate::error::AppError;
use crate::persistence::JsonFile;

pub const LABEL: &str = "widget";

/// Logical sizes; converted to physical pixels with the monitor's scale.
const COMPACT_SIZE: (f64, f64) = (96.0, 96.0);
const EXPANDED_SIZE: (f64, f64) = (380.0, 224.0);
const SCREEN_MARGIN: f64 = 24.0;
/// Positions are saved once the user stops dragging for this long.
const SAVE_DELAY: Duration = Duration::from_millis(400);

/// Where the compact widget was last left, as stored in `widget.json`.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct WidgetPlacement {
    position: Option<(i32, i32)>,
}

pub type PlacementFile = JsonFile<WidgetPlacement>;

/// Layout reported to the widget page after a resize.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetLayout {
    /// Corner holding the parrot while expanded; `None` when compact.
    pub corner: Option<Corner>,
}

#[derive(Debug, Default)]
pub struct WidgetState {
    corner: Mutex<Option<Corner>>,
    pending_save: AtomicU64,
    /// The user put the parrot away; it only appears while dictating.
    hidden: AtomicBool,
}

impl WidgetState {
    fn corner(&self) -> Option<Corner> {
        *self.corner.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn set_corner(&self, corner: Option<Corner>) {
        *self.corner.lock().unwrap_or_else(PoisonError::into_inner) = corner;
    }
}

/// Creates the widget at its saved position, or at the bottom-right of the
/// primary screen on first launch.
///
/// The window never takes keyboard focus: clicking the parrot must leave the
/// focus in the app the user is dictating into.
pub fn create(app: &AppHandle) -> Result<(), AppError> {
    let window = WebviewWindowBuilder::new(app, LABEL, WebviewUrl::App("widget.html".into()))
        .title("Pirate's Parrot")
        .inner_size(COMPACT_SIZE.0, COMPACT_SIZE.1)
        .decorations(false)
        .transparent(true)
        .shadow(false)
        .resizable(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .visible_on_all_workspaces(true)
        .focusable(false)
        .focused(false)
        .visible(false)
        .build()?;

    let start = initial_position(app)?;
    window.set_position(PhysicalPosition::new(start.0, start.1))?;
    window.show()?;

    let app = app.clone();
    window.on_window_event(move |event| {
        if let WindowEvent::Moved(position) = event {
            remember_position(&app, *position);
        }
    });
    Ok(())
}

/// Grows the widget to show the speech bubble, or shrinks it back to the
/// parrot alone.
pub fn set_expanded(app: &AppHandle, expanded: bool) -> Result<WidgetLayout, AppError> {
    let window = widget_window(app)?;
    let monitor = monitor_of(app, &window)?;
    let scale = monitor.scale_factor();
    let work_area = work_area_of(&monitor);
    let state = app.state::<WidgetState>();
    let compact = compact_rect(&window, state.corner())?;

    let (corner, target) = if expanded {
        let corner = state
            .corner()
            .unwrap_or_else(|| layout::parrot_corner(compact, work_area));
        let target = layout::expand(compact, physical(EXPANDED_SIZE, scale), corner, work_area);
        (Some(corner), target)
    } else {
        (None, compact)
    };

    // The state is updated before touching the window so the move events the
    // resize triggers are interpreted with the new layout.
    state.set_corner(corner);
    window.set_size(PhysicalSize::new(target.width, target.height))?;
    window.set_position(PhysicalPosition::new(target.x, target.y))?;
    // A parrot that was only revealed for a dictation goes away with its bubble.
    if !expanded && state.hidden.load(Ordering::Relaxed) {
        window.hide()?;
    }
    Ok(WidgetLayout { corner })
}

/// Puts the parrot away or brings it back. A hidden parrot still appears while
/// it listens (see [`reveal`]), so the user can see what is being written.
pub fn set_hidden(app: &AppHandle, hidden: bool) -> Result<(), AppError> {
    let state = app.state::<WidgetState>();
    state.hidden.store(hidden, Ordering::Relaxed);
    let window = widget_window(app)?;
    if !hidden {
        window.show()?;
    } else if state.corner().is_none() {
        // With the bubble open, the parrot hides once the bubble closes.
        window.hide()?;
    }
    Ok(())
}

pub fn is_hidden(app: &AppHandle) -> bool {
    app.state::<WidgetState>().hidden.load(Ordering::Relaxed)
}

/// Shows the parrot for a dictation even if the user hid it; it hides again
/// when its speech bubble closes.
pub fn reveal(app: &AppHandle) -> Result<(), AppError> {
    widget_window(app)?.show()?;
    Ok(())
}

fn widget_window(app: &AppHandle) -> Result<WebviewWindow, AppError> {
    app.get_webview_window(LABEL)
        .ok_or(AppError::Tauri(tauri::Error::WindowNotFound))
}

/// Saved position if it is still on a connected screen, otherwise the default.
fn initial_position(app: &AppHandle) -> Result<(i32, i32), AppError> {
    let saved = app.state::<PlacementFile>().get().position;
    let monitors = app.available_monitors()?;
    if let Some(position) = saved
        && monitors
            .iter()
            .any(|monitor| work_area_of(monitor).contains(position))
    {
        return Ok(position);
    }

    let monitor = app
        .primary_monitor()?
        .or_else(|| monitors.into_iter().next())
        .ok_or(AppError::NoScreen)?;
    let scale = monitor.scale_factor();
    let rect = layout::default_position(
        work_area_of(&monitor),
        physical(COMPACT_SIZE, scale),
        (SCREEN_MARGIN * scale).round() as u32,
    );
    Ok((rect.x, rect.y))
}

/// Saves where the compact widget sits, debounced while the user drags.
fn remember_position(app: &AppHandle, position: PhysicalPosition<i32>) {
    let Ok(window) = widget_window(app) else {
        return;
    };
    let state = app.state::<WidgetState>();
    let Ok(compact) = compact_rect_at(&window, position, state.corner()) else {
        return;
    };
    let generation = state.pending_save.fetch_add(1, Ordering::Relaxed) + 1;

    let app = app.clone();
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(SAVE_DELAY).await;
        if app
            .state::<WidgetState>()
            .pending_save
            .load(Ordering::Relaxed)
            != generation
        {
            return;
        }
        let placement = WidgetPlacement {
            position: Some((compact.x, compact.y)),
        };
        if let Err(error) = app.state::<PlacementFile>().set(placement) {
            log::warn!("could not save the widget position: {error}");
        }
    });
}

fn compact_rect(window: &WebviewWindow, corner: Option<Corner>) -> Result<Rect, AppError> {
    compact_rect_at(window, window.outer_position()?, corner)
}

/// Rect of the compact widget given the window's current position and layout.
fn compact_rect_at(
    window: &WebviewWindow,
    position: PhysicalPosition<i32>,
    corner: Option<Corner>,
) -> Result<Rect, AppError> {
    let size = window.outer_size()?;
    let current = Rect::new(position.x, position.y, size.width, size.height);
    Ok(match corner {
        Some(corner) => layout::collapse(
            current,
            physical(COMPACT_SIZE, window.scale_factor()?),
            corner,
        ),
        None => current,
    })
}

fn monitor_of(app: &AppHandle, window: &WebviewWindow) -> Result<Monitor, AppError> {
    window
        .current_monitor()?
        .or(app.primary_monitor()?)
        .ok_or(AppError::NoScreen)
}

fn work_area_of(monitor: &Monitor) -> Rect {
    let area = monitor.work_area();
    Rect::new(
        area.position.x,
        area.position.y,
        area.size.width,
        area.size.height,
    )
}

fn physical((width, height): (f64, f64), scale: f64) -> (u32, u32) {
    (
        (width * scale).round() as u32,
        (height * scale).round() as u32,
    )
}
