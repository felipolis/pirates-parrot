//! Geometry of the floating widget, in physical pixels.
//!
//! The widget is either compact (just the parrot) or expanded (the parrot
//! plus a speech bubble). When it expands, the parrot stays exactly where it
//! was and the bubble grows towards the middle of the screen, so it never
//! spills off the edge the user parked the parrot against.

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    fn right(self) -> i32 {
        self.x + dim(self.width)
    }

    fn bottom(self) -> i32 {
        self.y + dim(self.height)
    }

    fn center(self) -> (i32, i32) {
        (self.x + dim(self.width) / 2, self.y + dim(self.height) / 2)
    }

    pub fn contains(self, (x, y): (i32, i32)) -> bool {
        (self.x..self.right()).contains(&x) && (self.y..self.bottom()).contains(&y)
    }
}

/// Corner of the expanded widget that the parrot occupies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Corner {
    fn is_right(self) -> bool {
        matches!(self, Self::TopRight | Self::BottomRight)
    }

    fn is_bottom(self) -> bool {
        matches!(self, Self::BottomLeft | Self::BottomRight)
    }
}

/// Picks the corner for the parrot so the bubble opens towards the center of
/// the work area.
pub fn parrot_corner(compact: Rect, work_area: Rect) -> Corner {
    let (x, y) = compact.center();
    let (center_x, center_y) = work_area.center();
    match (y > center_y, x > center_x) {
        (true, true) => Corner::BottomRight,
        (true, false) => Corner::BottomLeft,
        (false, true) => Corner::TopRight,
        (false, false) => Corner::TopLeft,
    }
}

/// The expanded widget that keeps `compact` pinned at `corner`, moved back
/// inside the work area if it would overflow it.
pub fn expand(compact: Rect, size: (u32, u32), corner: Corner, work_area: Rect) -> Rect {
    let (width, height) = size;
    let x = if corner.is_right() {
        compact.right() - dim(width)
    } else {
        compact.x
    };
    let y = if corner.is_bottom() {
        compact.bottom() - dim(height)
    } else {
        compact.y
    };
    clamp_into(Rect::new(x, y, width, height), work_area)
}

/// The compact widget sitting at `corner` of `expanded`.
pub fn collapse(expanded: Rect, size: (u32, u32), corner: Corner) -> Rect {
    let (width, height) = size;
    let x = if corner.is_right() {
        expanded.right() - dim(width)
    } else {
        expanded.x
    };
    let y = if corner.is_bottom() {
        expanded.bottom() - dim(height)
    } else {
        expanded.y
    };
    Rect::new(x, y, width, height)
}

/// Default spot for a compact widget: the bottom-right of the work area.
pub fn default_position(work_area: Rect, size: (u32, u32), margin: u32) -> Rect {
    let (width, height) = size;
    Rect::new(
        work_area.right() - dim(width) - dim(margin),
        work_area.bottom() - dim(height) - dim(margin),
        width,
        height,
    )
}

/// Moves `rect` the least amount needed to fit inside `area`.
pub fn clamp_into(rect: Rect, area: Rect) -> Rect {
    let x = rect.x.min(area.right() - dim(rect.width)).max(area.x);
    let y = rect.y.min(area.bottom() - dim(rect.height)).max(area.y);
    Rect { x, y, ..rect }
}

/// Window dimensions are far below `i32::MAX`; saturate rather than wrap if a
/// platform ever reports something absurd.
fn dim(value: u32) -> i32 {
    i32::try_from(value).unwrap_or(i32::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCREEN: Rect = Rect {
        x: 0,
        y: 0,
        width: 1920,
        height: 1040,
    };
    const COMPACT: (u32, u32) = (100, 100);
    const EXPANDED: (u32, u32) = (400, 250);

    #[test]
    fn bubble_opens_towards_screen_center() {
        let bottom_right = Rect::new(1800, 900, 100, 100);
        let top_left = Rect::new(20, 20, 100, 100);

        assert_eq!(parrot_corner(bottom_right, SCREEN), Corner::BottomRight);
        assert_eq!(parrot_corner(top_left, SCREEN), Corner::TopLeft);
    }

    #[test]
    fn expanding_keeps_the_parrot_in_place() {
        let compact = Rect::new(1800, 900, 100, 100);

        let expanded = expand(compact, EXPANDED, Corner::BottomRight, SCREEN);

        assert_eq!(expanded, Rect::new(1500, 750, 400, 250));
        assert_eq!(collapse(expanded, COMPACT, Corner::BottomRight), compact);
    }

    #[test]
    fn expanding_near_an_edge_stays_on_screen() {
        let compact = Rect::new(10, 10, 100, 100);

        let expanded = expand(compact, EXPANDED, Corner::BottomRight, SCREEN);

        assert_eq!((expanded.x, expanded.y), (0, 0));
    }

    #[test]
    fn default_position_is_bottom_right_with_margin() {
        assert_eq!(
            default_position(SCREEN, COMPACT, 24),
            Rect::new(1796, 916, 100, 100)
        );
    }

    #[test]
    fn work_area_offset_is_respected() {
        let second_monitor = Rect::new(1920, 0, 1280, 1000);
        let offscreen = Rect::new(-500, 2000, 100, 100);

        assert_eq!(
            clamp_into(offscreen, second_monitor),
            Rect::new(1920, 900, 100, 100)
        );
    }
}
