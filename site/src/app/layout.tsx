import type { Metadata, Viewport } from 'next';
import { IM_Fell_English, IM_Fell_English_SC, Nunito, Pirata_One } from 'next/font/google';
import type { ReactNode } from 'react';
import './globals.css';

// The desktop app's typefaces, self-hosted by Next.js at build time.
const body = Nunito({ subsets: ['latin'], variable: '--font-body' });
const print = IM_Fell_English({
  subsets: ['latin'],
  weight: '400',
  style: ['normal', 'italic'],
  variable: '--font-print',
});
const printCaps = IM_Fell_English_SC({
  subsets: ['latin'],
  weight: '400',
  variable: '--font-print-caps',
});
const flag = Pirata_One({ subsets: ['latin'], weight: '400', variable: '--font-flag' });

export const metadata: Metadata = {
  title: "Pirate's Parrot — talk to your terminal",
  description:
    'A little desktop parrot that turns your voice into text as you speak and pastes it wherever you are typing. Runs entirely on your computer. Free and open source.',
  icons: { icon: '/parrots/captain-scarlet.svg' },
};

export const viewport: Viewport = {
  themeColor: '#07121c',
};

export default function RootLayout({ children }: { children: ReactNode }) {
  return (
    <html
      lang="en"
      className={`${body.variable} ${print.variable} ${printCaps.variable} ${flag.variable}`}
    >
      <body>{children}</body>
    </html>
  );
}
