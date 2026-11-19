import Link from 'next/link';
import styles from './not-found.module.css';

export default function NotFound() {
  return (
    <main className={styles.lost}>
      <p className={styles.code}>404</p>
      <h1 className={styles.title}>Lost at sea.</h1>
      <p className={styles.text}>This page sank, or it never existed.</p>
      <Link className={styles.home} href="/">
        Back to the ship
      </Link>
    </main>
  );
}
