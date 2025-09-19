import { dev } from "$app/environment"

// Config is a public manifest of configuration for the site.
export interface Config {
	title: string
	description: string
	url: string
}

const development: Config = {
	title: "SvelteKit Kickstart",
	description: "A starter template for building web applications with SvelteKit and TypeScript.",
	url: "https://sveltekitkickstart.local"
}

const production: Config = {
	title: "SvelteKit Kickstart",
	description: "A starter template for building web applications with SvelteKit and TypeScript.",
	url: "https://sveltekitkickstart.local"
}

const config = dev ? development : production

export default config
