import styles from './Waves.module.css';

const TILE_WIDTH = 2880;
const SWELLS_PER_TILE = 6;

/**
 * A row of smooth swells across a tile twice as wide as the scene. Every swell
 * starts and ends at mid-height with the same slope, so sliding the tile by
 * half its width loops without a seam.
 */
function swellPath(): string {
  const width = TILE_WIDTH / SWELLS_PER_TILE;
  const curves = Array.from({ length: SWELLS_PER_TILE }, (_, index) => {
    const start = index * width;
    return `C${start + width / 4} 0 ${start + (width * 3) / 4} 100 ${start + width} 50`;
  });
  return `M0 50 ${curves.join(' ')} V100 H0 Z`;
}

const SWELL = swellPath();

const LAYERS = ['far', 'middle', 'near'] as const;

/**
 * Layers of drifting waves along the bottom edge of a scene. The nearest layer
 * is painted in the page's color, so the scene flows into what comes below.
 */
export function Waves() {
  return (
    <div className={styles.waves} aria-hidden="true">
      {LAYERS.map((layer) => (
        <svg
          key={layer}
          className={`${styles.layer} ${styles[layer]}`}
          viewBox={`0 0 ${TILE_WIDTH} 100`}
          preserveAspectRatio="none"
        >
          <path d={SWELL} />
        </svg>
      ))}
    </div>
  );
}
