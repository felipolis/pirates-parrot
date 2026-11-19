'use client';

// A looping re-enactment of a dictation: the words appear in the parrot's
// speech bubble as they are "spoken", then the text lands in the terminal.

import Image from 'next/image';
import { useEffect, useState } from 'react';
import { useMediaQuery } from '@/lib/use-media';
import styles from './DictationDemo.module.css';

const PHRASES = [
  'Refactor the login handler so it returns a 401 instead of throwing.',
  'Write tests for the date parser, and cover leap years.',
  'Explain why this query is slow and suggest an index.',
].map((phrase) => phrase.split(' '));

/** Roughly the pace of someone dictating. */
const WORD_DELAY_MS = 280;
const FIRST_WORD_DELAY_MS = 1100;
const PASTE_DELAY_MS = 750;
const PASTED_HOLD_MS = 2800;

interface Step {
  phrase: number;
  /** How many words of the phrase have been heard. */
  heard: number;
  pasted: boolean;
}

const FIRST_STEP: Step = { phrase: 0, heard: 0, pasted: false };

function wordsOf(phrase: number): string[] {
  return PHRASES[phrase] ?? [];
}

function nextStep(step: Step): Step {
  if (step.pasted) return { phrase: (step.phrase + 1) % PHRASES.length, heard: 0, pasted: false };
  if (step.heard < wordsOf(step.phrase).length) return { ...step, heard: step.heard + 1 };
  return { ...step, pasted: true };
}

function delayBefore(next: Step, current: Step): number {
  if (current.pasted) return PASTED_HOLD_MS;
  if (next.pasted) return PASTE_DELAY_MS;
  return current.heard === 0 ? FIRST_WORD_DELAY_MS : WORD_DELAY_MS;
}

interface Props {
  /** The parrot sitting next to the terminal. */
  parrotSrc: string;
}

export function DictationDemo({ parrotSrc }: Props) {
  const reducedMotion = useMediaQuery('(prefers-reduced-motion: reduce)');
  const [step, setStep] = useState(FIRST_STEP);

  useEffect(() => {
    if (reducedMotion) return;
    const next = nextStep(step);
    const timer = setTimeout(() => setStep(next), delayBefore(next, step));
    return () => clearTimeout(timer);
  }, [step, reducedMotion]);

  // Without motion, show a finished dictation instead of the loop.
  const shown = reducedMotion ? { phrase: 0, heard: wordsOf(0).length, pasted: true } : step;
  const heardText = wordsOf(shown.phrase).slice(0, shown.heard).join(' ');

  return (
    <figure className={styles.desktop} aria-hidden="true">
      <div className={styles.terminal}>
        <div className={styles.titleBar}>
          <span className={styles.lights} />
          <span className={styles.title}>~/projects/api</span>
        </div>
        <div className={styles.screen}>
          <p className={styles.dim}>Ready when you are. What should we work on?</p>
          <p className={styles.prompt}>
            <span className={styles.caret}>›</span>
            {shown.pasted ? <span className={styles.pasted}>{heardText}</span> : null}
            <span className={styles.cursor} />
          </p>
        </div>
      </div>

      <div className={styles.widget}>
        <div className={styles.bubble}>
          <p className={shown.pasted ? `${styles.status} ${styles.done}` : styles.status}>
            <span className={styles.dot} />
            {shown.pasted ? 'Pasted!' : 'Listening'}
          </p>
          <p className={heardText ? styles.words : `${styles.words} ${styles.placeholder}`}>
            {heardText || 'Speak up, I’m all ears…'}
          </p>
        </div>
        <Image className={styles.parrot} src={parrotSrc} alt="" width={92} height={92} />
      </div>
    </figure>
  );
}
