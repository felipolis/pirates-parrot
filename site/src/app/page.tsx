import { Closing } from '@/components/Closing';
import { Crew } from '@/components/Crew';
import { Hero } from '@/components/Hero';
import { HowItWorks } from '@/components/HowItWorks';
import { Privacy } from '@/components/Privacy';
import { SiteFooter } from '@/components/SiteFooter';
import { SiteHeader } from '@/components/SiteHeader';
import { getCrew } from '@/lib/crew';
import { getLatestRelease } from '@/lib/release';
import { RELEASES_URL, REPOSITORY_URL } from '@/lib/repository';

/** The parrot that greets visitors: the app's default. */
const HOST_PARROT = 'captain-scarlet';

export default async function HomePage() {
  const [release, crew] = await Promise.all([getLatestRelease(), getCrew()]);
  const host = crew.find((parrot) => parrot.id === HOST_PARROT);
  if (!host) throw new Error(`The exported crew has no "${HOST_PARROT}".`);

  return (
    <>
      <SiteHeader repositoryUrl={REPOSITORY_URL} logoSrc={host.src} />
      <main>
        <Hero release={release} repositoryUrl={REPOSITORY_URL} parrotSrc={host.src} />
        <HowItWorks />
        <Privacy />
        <Crew parrots={crew} />
        <Closing release={release} repositoryUrl={REPOSITORY_URL} />
      </main>
      <SiteFooter repositoryUrl={REPOSITORY_URL} releasesUrl={RELEASES_URL} />
    </>
  );
}
