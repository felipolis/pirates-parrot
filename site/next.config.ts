import type { NextConfig } from 'next';

const nextConfig: NextConfig = {
  // A static site: `next build` writes plain HTML, CSS and JS to `out/`,
  // which Cloudflare serves as is.
  output: 'export',
  // With no server, images cannot be optimized on request.
  images: { unoptimized: true },
};

export default nextConfig;
