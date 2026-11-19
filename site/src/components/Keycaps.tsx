import styles from './Keycaps.module.css';

/** A keyboard shortcut drawn as brass keys, like in the app's settings. */
export function Keycaps({ keys }: { keys: string[] }) {
  return (
    <span className={styles.keys}>
      {keys.map((key) => (
        <kbd key={key} className={styles.key}>
          {key}
        </kbd>
      ))}
    </span>
  );
}
