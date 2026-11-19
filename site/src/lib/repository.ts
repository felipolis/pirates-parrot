// The GitHub repository the site links to and takes its downloads from.

const REPOSITORY_PATTERN = /^[\w.-]+\/[\w.-]+$/;

function readRepository(): string {
  const repository = process.env.GITHUB_REPOSITORY;
  if (!repository || !REPOSITORY_PATTERN.test(repository)) {
    throw new Error(
      'Set GITHUB_REPOSITORY to the repository as owner/name (see site/.env.example).',
    );
  }
  return repository;
}

/** The repository as `owner/name`. */
export const REPOSITORY = readRepository();

export const REPOSITORY_URL = `https://github.com/${REPOSITORY}`;

export const RELEASES_URL = `${REPOSITORY_URL}/releases`;
