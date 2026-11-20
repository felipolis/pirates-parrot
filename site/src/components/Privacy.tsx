import {
  faHandHoldingHeart,
  faLanguage,
  faLaptop,
  faLock,
  faMicrochip,
  type IconDefinition,
} from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import styles from './Privacy.module.css';

const FACTS: { term: string; detail: string; icon: IconDefinition }[] = [
  { term: 'Runs on', detail: 'Your processor. No graphics card needed.', icon: faMicrochip },
  {
    term: 'Speaks',
    detail: 'English, Portuguese, Spanish, French, German, Japanese and more, detected on its own.',
    icon: faLanguage,
  },
  { term: 'Works on', detail: 'Windows, macOS and Linux.', icon: faLaptop },
  {
    term: 'Costs',
    detail: 'Nothing. It’s open source under the MIT license.',
    icon: faHandHoldingHeart,
  },
];

export function Privacy() {
  return (
    <section id="privacy" className={styles.section} aria-labelledby="privacy-title">
      <div className={`${styles.panel} reveal`}>
        <div className={styles.statement}>
          <div className={styles.seal} aria-hidden="true">
            <FontAwesomeIcon icon={faLock} className={styles.sealIcon} />
            <span>Local only</span>
          </div>
          <div className={styles.text}>
            <h2 id="privacy-title" className={styles.title}>
              Your voice never leaves your computer.
            </h2>
            <p className={styles.intro}>
              Speech recognition runs on your own machine with NVIDIA’s Nemotron 3.5 ASR Streaming
              model. There’s no account, no cloud service and nothing to subscribe to. After a
              one-time 475 MB model download, it even works offline.
            </p>
          </div>
        </div>
        <dl className={styles.facts}>
          {FACTS.map(({ term, detail, icon }) => (
            <div key={term} className={styles.fact}>
              <dt className={styles.term}>
                <FontAwesomeIcon icon={icon} className={styles.factIcon} />
                {term}
              </dt>
              <dd className={styles.detail}>{detail}</dd>
            </div>
          ))}
        </dl>
      </div>
    </section>
  );
}
