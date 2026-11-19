'use client';

import { useSyncExternalStore } from 'react';

/**
 * Whether the media query matches, kept current as it changes. Static HTML is
 * rendered as if it does not match.
 */
export function useMediaQuery(query: string): boolean {
  return useSyncExternalStore(
    (onChange) => {
      const media = window.matchMedia(query);
      media.addEventListener('change', onChange);
      return () => media.removeEventListener('change', onChange);
    },
    () => window.matchMedia(query).matches,
    () => false,
  );
}
