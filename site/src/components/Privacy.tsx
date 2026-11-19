import styles from './Privacy.module.css';
import { Section } from './Section';

const FACTS: { term: string; detail: string }[] = [
  { term: 'Runs on', detail: 'Your processor. No graphics card needed.' },
  {
    term: 'Speaks',
    detail: 'English, Portuguese, Spanish, French, German, Japanese and more, detected on its own.',
  },
  { term: 'Works on', detail: 'Windows, macOS and Linux.' },
  { term: 'Costs', detail: 'Nothing. It’s open source under the MIT license.' },
];

export function Privacy() {
  return (
    <Section
      id="privacy"
      title="Your voice never leaves your computer."
      intro="Speech recognition runs on your own machine with NVIDIA’s Nemotron 3.5 ASR Streaming model. There’s no account, no cloud service and nothing to subscribe to. After a one-time 475 MB model download, it even works offline."
    >
      <dl className={styles.facts}>
        {FACTS.map(({ term, detail }) => (
          <div key={term} className={styles.fact}>
            <dt className={styles.term}>{term}</dt>
            <dd className={styles.detail}>{detail}</dd>
          </div>
        ))}
      </dl>
    </Section>
  );
}
