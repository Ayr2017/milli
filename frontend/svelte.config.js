import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),
	kit: {
		adapter: adapter({
			// Указываем правильную папку - static вместо build
			pages: '../static',
			assets: '../static',
			fallback: 'index.html', // Для SPA режима можно установить 'index.html'
			precompress: false,
			strict: true
		})
	}
};

export default config;