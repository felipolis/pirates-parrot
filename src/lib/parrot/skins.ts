// The crew of parrots the user can pick from. A skin is just a palette plus a
// set of accessories layered on the same SVG parrot, so adding a new parrot
// is a matter of adding an entry here.

export type Accessory = 'tricorne' | 'bandana' | 'eyepatch' | 'earring';

export interface ParrotPalette {
  body: string;
  belly: string;
  wing: string;
  wingTip: string;
  tail: string;
  tailTip: string;
  crest: string;
  face: string;
  cheek: string;
  eye: string;
  beak: string;
  beakShade: string;
  feet: string;
  hat: string;
  hatTrim: string;
  bandana: string;
}

export interface ParrotSkin {
  id: string;
  name: string;
  tagline: string;
  palette: ParrotPalette;
  accessories: readonly Accessory[];
  /** Translucent and floating. */
  ghostly?: boolean;
}

const pirateGear = {
  hat: '#3b2a22',
  hatTrim: '#e2b13c',
  bandana: '#d8343d',
} as const;

const CAPTAIN_SCARLET: ParrotSkin = {
  id: 'captain-scarlet',
  name: 'Captain Scarlet',
  tagline: 'Scarlet macaw. Has commanded more ships than she has lost.',
  accessories: ['tricorne', 'eyepatch'],
  palette: {
    ...pirateGear,
    body: '#e0412f',
    belly: '#f2683a',
    wing: '#2f7fd6',
    wingTip: '#f5c431',
    tail: '#d93a2b',
    tailTip: '#2f7fd6',
    crest: '#e0412f',
    face: '#fff4ea',
    cheek: '#f7a6a0',
    eye: '#231c1a',
    beak: '#f1e9df',
    beakShade: '#3b3633',
    feet: '#8e8a86',
  },
};

export const SKINS: readonly ParrotSkin[] = [
  CAPTAIN_SCARLET,
  {
    id: 'blue-jack',
    name: 'Blue Jack',
    tagline: 'Blue-and-gold macaw. Never without his lucky bandana.',
    accessories: ['bandana', 'earring'],
    palette: {
      ...pirateGear,
      body: '#2c8be0',
      belly: '#f7c736',
      wing: '#2273c2',
      wingTip: '#1b5c9e',
      tail: '#2c8be0',
      tailTip: '#1b5c9e',
      crest: '#3fa04a',
      face: '#ffffff',
      cheek: '#9fd0f5',
      eye: '#1c1a19',
      beak: '#33302e',
      beakShade: '#1b1918',
      feet: '#6e6a66',
    },
  },
  {
    id: 'cabin-boy',
    name: 'Cabin Boy',
    tagline: 'Green parrot. First voyage, learns every word.',
    accessories: [],
    palette: {
      ...pirateGear,
      body: '#46b05a',
      belly: '#b8e067',
      wing: '#3a9a4c',
      wingTip: '#2f7dc6',
      tail: '#46b05a',
      tailTip: '#e8c33a',
      crest: '#3a9a4c',
      face: '#5cc46e',
      cheek: '#f29c6b',
      eye: '#1c1a19',
      beak: '#f2a93b',
      beakShade: '#b86f16',
      feet: '#9a8f86',
    },
  },
  {
    id: 'old-salt',
    name: 'Old Salt',
    tagline: 'Cockatoo. Has a story for every scar.',
    accessories: ['eyepatch', 'earring'],
    palette: {
      ...pirateGear,
      body: '#f4f1e8',
      belly: '#ffffff',
      wing: '#e8e3d6',
      wingTip: '#d9d2c1',
      tail: '#e8e3d6',
      tailTip: '#f6c93c',
      crest: '#f6c93c',
      face: '#f4f1e8',
      cheek: '#f7c8b5',
      eye: '#1c1a19',
      beak: '#3a3633',
      beakShade: '#1d1b1a',
      feet: '#6e6a66',
    },
  },
  {
    id: 'phantom-polly',
    name: 'Phantom Polly',
    tagline: 'Haunts the ship. Still repeats everything you say.',
    accessories: ['tricorne'],
    ghostly: true,
    palette: {
      body: '#cfe9f2',
      belly: '#e8f6fa',
      wing: '#b4dce9',
      wingTip: '#9ccfe0',
      tail: '#b4dce9',
      tailTip: '#9ccfe0',
      crest: '#b4dce9',
      face: '#f2fbfd',
      cheek: '#bfe3ee',
      eye: '#2b5c6b',
      beak: '#ddeff4',
      beakShade: '#8dbccb',
      feet: '#9ccfe0',
      hat: '#7fb3c4',
      hatTrim: '#e8f6fa',
      bandana: '#9ccfe0',
    },
  },
];

/** The skin with `id`, or the default one if it no longer exists. */
export function skinById(id: string | undefined): ParrotSkin {
  return SKINS.find((skin) => skin.id === id) ?? CAPTAIN_SCARLET;
}
