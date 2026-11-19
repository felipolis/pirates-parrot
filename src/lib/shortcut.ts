// Conversions between keyboard events, Tauri accelerator strings such as
// `CommandOrControl+Shift+Space`, and labels shown to the user.

const isMac = navigator.userAgent.includes('Mac');

const MODIFIER_KEYS = new Set(['Control', 'Shift', 'Alt', 'Meta', 'AltGraph', 'OS']);

const MODIFIER_LABELS: Record<string, string> = isMac
  ? {
      CommandOrControl: '⌘',
      Control: '⌃',
      Super: '⌘',
      Alt: '⌥',
      Shift: '⇧',
    }
  : {
      CommandOrControl: 'Ctrl',
      Control: 'Ctrl',
      Super: 'Win',
      Alt: 'Alt',
      Shift: 'Shift',
    };

const KEY_LABELS: Record<string, string> = {
  ArrowUp: '↑',
  ArrowDown: '↓',
  ArrowLeft: '←',
  ArrowRight: '→',
  Backquote: '`',
  Minus: '-',
  Equal: '=',
  BracketLeft: '[',
  BracketRight: ']',
  Backslash: '\\',
  Semicolon: ';',
  Quote: "'",
  Comma: ',',
  Period: '.',
  Slash: '/',
};

/** The keys of an accelerator as the user would read them, e.g. `['Ctrl', 'Shift', 'Space']`. */
export function shortcutKeys(accelerator: string): string[] {
  return accelerator.split('+').map(keyLabel);
}

function keyLabel(part: string): string {
  const modifier = MODIFIER_LABELS[part];
  if (modifier) return modifier;
  if (part.startsWith('Key')) return part.slice('Key'.length);
  if (part.startsWith('Digit')) return part.slice('Digit'.length);
  return KEY_LABELS[part] ?? part;
}

/**
 * Turns a key press into an accelerator. Returns `null` while only modifiers
 * are held, and for combinations without a modifier (other than function
 * keys), which would hijack normal typing.
 */
export function acceleratorFromEvent(event: KeyboardEvent): string | null {
  if (MODIFIER_KEYS.has(event.key)) return null;

  const modifiers: string[] = [];
  if (event.ctrlKey) modifiers.push(isMac ? 'Control' : 'CommandOrControl');
  if (event.metaKey) modifiers.push(isMac ? 'CommandOrControl' : 'Super');
  if (event.altKey) modifiers.push('Alt');
  if (event.shiftKey) modifiers.push('Shift');

  const isFunctionKey = /^F\d{1,2}$/.test(event.code);
  if (modifiers.length === 0 && !isFunctionKey) return null;
  return [...modifiers, event.code].join('+');
}
