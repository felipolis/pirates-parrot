import Image from 'next/image';
import type { Parrot } from '@/lib/crew';
import styles from './Crew.module.css';
import { Section } from './Section';

export function Crew({ parrots }: { parrots: Parrot[] }) {
  return (
    <Section
      id="crew"
      title="Pick a shipmate"
      intro="Every parrot ships with the app. They breathe, blink and talk along while you dictate."
    >
      <ul className={`${styles.crew} reveal`}>
        {parrots.map((parrot) => (
          <li key={parrot.id} className={styles.member}>
            <div className={styles.perch}>
              <Image className={styles.portrait} src={parrot.src} alt="" width={120} height={120} />
            </div>
            <h3 className={styles.plate}>{parrot.name}</h3>
            <p className={styles.tagline}>{parrot.tagline}</p>
          </li>
        ))}
      </ul>
    </Section>
  );
}
