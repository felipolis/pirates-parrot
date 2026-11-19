import type { ReactNode } from 'react';
import styles from './HowItWorks.module.css';
import { Keycaps } from './Keycaps';
import { Section } from './Section';

const STEPS: { title: string; body: ReactNode }[] = [
  {
    title: 'Press the shortcut',
    body: (
      <>
        <Keycaps keys={['Ctrl', 'Shift', 'Space']} /> from any app, or click the parrot. On a Mac
        it’s <Keycaps keys={['⌘', '⇧', 'Space']} />.
      </>
    ),
  },
  {
    title: 'Say what you need',
    body: 'Your words show up in the parrot’s speech bubble as you talk, so you can see it keeping up.',
  },
  {
    title: 'It lands where you type',
    body: 'When you stop, the text is pasted at your cursor and copied to the clipboard, ready for your agent.',
  },
];

export function HowItWorks() {
  return (
    <Section id="how-it-works" title="How it works">
      <ol className={styles.steps}>
        {STEPS.map((step) => (
          <li key={step.title} className={styles.step}>
            <h3 className={styles.title}>{step.title}</h3>
            <p className={styles.body}>{step.body}</p>
          </li>
        ))}
      </ol>
    </Section>
  );
}
