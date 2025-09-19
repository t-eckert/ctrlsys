<script lang="ts">
	import "@fontsource/libre-caslon-text"
	import "@fontsource-variable/jetbrains-mono"
	import { type LayoutProps } from "./$types"

	import "../app.css"
	import config from "$lib/config"
	import Head from "$lib/components/head/head.svelte"
	import Header from "./header.svelte"
	import Footer from "./footer.svelte"
	import SkipToContent from "$lib/utils/components/skip-to-content.svelte"
	import ThemeProvider from "$lib/theme/theme-provider.svelte"
	import Analytics from "$lib/components/analytics/analytics.svelte"
	import Json from "$lib/utils/components/json.svelte"

	let { children, data }: LayoutProps = $props()

	let theme = data.root.theme || "system"
</script>

<Head
	title={config.title}
	description={config.description}
	url={config.url}
	favicon="/favicon.ico"
/>

<Analytics publicSiteCode="ASDF" />

<ThemeProvider {theme}>
	<div class="min-h-screen">
		<SkipToContent />
		<Header />
		<main class="mx-auto w-full max-w-7xl px-4" id="main-content">
			{@render children()}
		</main>
	</div>
	<Footer />
</ThemeProvider>
