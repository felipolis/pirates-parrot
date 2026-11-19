/** Formats a byte count as megabytes, e.g. `453 MB`. */
export function formatMegabytes(bytes: number): string {
  return `${Math.round(bytes / 1_000_000)} MB`;
}

/** Formats a short duration, e.g. `560 ms` or `1.12 s`. */
export function formatDuration(milliseconds: number): string {
  return milliseconds < 1000 ? `${milliseconds} ms` : `${milliseconds / 1000} s`;
}
