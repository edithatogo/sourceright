import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

export default defineConfig({
  site: 'https://edithatogo.github.io/sourceright',
  base: '/sourceright/',
  integrations: [
    starlight({
      title: 'Sourceright',
      description:
        'Reference verification for academic, legal, and provenance-sensitive workflows.',
      sidebar: [
        {
          label: 'Guides',
          items: [{ autogenerate: { directory: 'guides' } }],
        },
        {
          label: 'Reference',
          items: [{ autogenerate: { directory: 'reference' } }],
        },
      ],
    }),
  ],
});
