import styles from './NightSky.module.css';

/** Twinkling stars and a glowing moon, painted behind a scene. */
export function NightSky() {
  return (
    <div className={styles.sky} aria-hidden="true">
      <div className={`${styles.stars} ${styles.near}`} />
      <div className={`${styles.stars} ${styles.far}`} />
      <div className={styles.moon} />
    </div>
  );
}
