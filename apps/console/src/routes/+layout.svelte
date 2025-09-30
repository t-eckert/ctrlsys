<script lang="ts">
	import "@fontsource/libre-caslon-text"
	import "@fontsource-variable/jetbrains-mono"
	import { type LayoutProps } from "./$types"

	import "../app.css"
	import config from "$lib/config"
	import Head from "$lib/components/head/head.svelte"
	import ThemeProvider from "$lib/theme/theme-provider.svelte"

	import { ToolbarProvider, Toolbar } from "$lib/toolbar"
	import { StatusBarProvider, StatusBar } from "$lib/statusbar"
	import { NavigationProvider, Sidebar } from "$lib/navigation"

	let { children, data }: LayoutProps = $props()

	let theme = data.root.theme || "system"
</script>

<Head
	title={config.title}
	description={config.description}
	url={config.url}
	favicon="/favicon.ico"
/>

<ThemeProvider {theme}>
	<ToolbarProvider>
		<StatusBarProvider>
			<NavigationProvider>
				<div class="app-layout grid h-screen w-screen grid-rows-[auto_1fr_auto] overscroll-none">
					<!-- Toolbar -->
					<Toolbar />

					<!-- Main Content Area: Navigation + App Pane -->
					<div class="grid grid-cols-[auto_1fr] overflow-hidden">
						<!-- Navigation Sidebar -->
						<Sidebar />

						<!-- App Pane (Scrollable Content) -->
						<main class="app-pane overflow-y-auto overscroll-none bg-white dark:bg-neutral-950" id="main-content">
							{@render children()}
						</main>
					</div>

					<!-- StatusBar -->
					<StatusBar />
				</div>
			</NavigationProvider>
		</StatusBarProvider>
	</ToolbarProvider>
</ThemeProvider>
