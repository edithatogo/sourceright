import { defineConfig } from 'astro/config';
import mdx from '@astrojs/mdx';
import sitemap from '@astrojs/sitemap';
import starlight from '@astrojs/starlight';

export default defineConfig({
  site: 'https://edithatogo.github.io',
  base: '/sourceright/',
  integrations: [
    mdx(),
    sitemap(),
    starlight({
      title: 'SourceRight',
      description: 'Legal NZ documentation portal for SourceRight.',
      sidebar: [
        { label: 'Start', items: ['index', 'docs-tooling-audit'] },
      ],
    }),
  ],
});
