import {
  faCode,
  faComments,
  faEnvelope,
  faGlobe,
  faRobot,
  faTerminal,
  type IconDefinition,
} from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import styles from './WorksWith.module.css';

/** Places the text can land; the app pastes into whatever has the cursor. */
const DESTINATIONS: { label: string; icon: IconDefinition }[] = [
  { label: 'Claude Code', icon: faTerminal },
  { label: 'Codex', icon: faRobot },
  { label: 'Your editor', icon: faCode },
  { label: 'Chat apps', icon: faComments },
  { label: 'The browser', icon: faGlobe },
  { label: 'Email', icon: faEnvelope },
];

export function WorksWith() {
  return (
    <section className={`${styles.strip} reveal`} aria-labelledby="works-with-title">
      <h2 id="works-with-title" className={styles.title}>
        Lands wherever you type
      </h2>
      <ul className={styles.destinations}>
        {DESTINATIONS.map(({ label, icon }) => (
          <li key={label} className={styles.destination}>
            <FontAwesomeIcon icon={icon} className={styles.icon} />
            {label}
          </li>
        ))}
      </ul>
    </section>
  );
}
