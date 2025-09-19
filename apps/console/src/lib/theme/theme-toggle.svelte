<script lang="ts">
	import { Sun, Moon, Monitor } from "phosphor-svelte"
	import { getThemeStore, type Theme } from "./theme-store.svelte"
	import { enhance } from "$app/forms"
	import Button from "$lib/components/button/button.svelte"
	import type { SubmitFunction } from "@sveltejs/kit"

	let store = getThemeStore()

	const submitUpdateTheme: SubmitFunction = async ({ action }) => {
		console.log("submitUpdateTheme")
		console.dir(action)
		const theme = action.searchParams.get("theme")

		if (theme) {
			store.setTheme(theme as Theme)
		}
	}
</script>

<form method="POST" use:enhance={submitUpdateTheme}>
	{#if store.theme === "dark"}
		<Button
			shape="square"
			role="tertiary"
			formaction="/?/setTheme&theme=light"
			ariaLabel="set theme to light"
		>
			<Moon />
		</Button>
	{:else if store.theme === "light"}
		<Button
			shape="square"
			role="tertiary"
			formaction="/?/setTheme&theme=dark"
			ariaLabel="set theme to dark"
		>
			<Sun />
		</Button>
	{:else if store.theme === "system"}
		<Button
			shape="square"
			role="tertiary"
			formaction="/?/setTheme&theme=light"
			ariaLabel="set theme to light"
		>
			<Monitor />
		</Button>
	{/if}
</form>
