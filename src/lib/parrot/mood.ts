/** What the parrot is up to, which drives its animation. */
export type Mood =
  /** Waiting for the user: breathes and blinks. */
  | 'idle'
  /** Recording: leans in and moves its beak with the user's voice. */
  | 'listening'
  /** Transcribing the last words. */
  | 'thinking'
  /** The text was delivered: flaps its wings. */
  | 'happy'
  /** Something went wrong: ruffles its feathers. */
  | 'error'
  /** The speech model is not available yet. */
  | 'sleeping';
