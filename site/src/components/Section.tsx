import type { ReactNode } from 'react';
import styles from './Section.module.css';

interface Props {
  /** Anchor for in-page links. */
  id: string;
  title: ReactNode;
  intro?: ReactNode;
  children: ReactNode;
}

/** A band of the page with the shared width, spacing and heading. */
export function Section({ id, title, intro, children }: Props) {
  return (
    <section id={id} className={styles.section} aria-labelledby={`${id}-title`}>
      <header className={styles.header}>
        <h2 id={`${id}-title`} className={styles.title}>
          {title}
        </h2>
        {intro ? <p className={styles.intro}>{intro}</p> : null}
      </header>
      {children}
    </section>
  );
}
