'use client';

import { useSyncExternalStore } from 'react';

export type OperatingSystem = 'windows' | 'macos' | 'linux';

/** Chromium's structured platform hint, not yet in TypeScript's DOM types. */
interface NavigatorWithUserAgentData extends Navigator {
  userAgentData?: { platform: string; mobile: boolean };
}

/** The desktop system the visitor is browsing from, or `null` on phones and unknown systems. */
function detect(): OperatingSystem | null {
  const { userAgentData, userAgent } = navigator as NavigatorWithUserAgentData;
  if (userAgentData?.mobile || /Android|iPhone|iPad/.test(userAgent)) return null;

  const platform = userAgentData?.platform ?? userAgent;
  if (/Win/i.test(platform)) return 'windows';
  if (/Mac/i.test(platform)) return 'macos';
  if (/Linux|X11/i.test(platform)) return 'linux';
  return null;
}

// The system never changes while the page is open.
const subscribe = () => () => {};

/** The visitor's operating system; static HTML renders with `null`. */
export function useVisitorOs(): OperatingSystem | null {
  return useSyncExternalStore(subscribe, detect, () => null);
}
