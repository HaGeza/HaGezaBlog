import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
    preprocess: vitePreprocess(),

    kit: {
        // adapter-auto only supports some environments, see https://svelte.dev/docs/kit/adapters for more information.
        // adapter-static will create the 'build' folder you need.
        adapter: adapter({
            pages: 'build',
            assets: 'build',
            fallback: '404.html',
            precompress: false,
            strict: true
        }),
        paths: {
			// use repo name as base path in production
            base: process.env.NODE_ENV === 'production' ? '/HaGezaBlog' : '',
        }
    }
};

export default config;
