<script lang="ts">
	import { page } from "$app/state"
	import { onMount } from "svelte"
	import * as Fathom from "fathom-client"
	import config from "$lib/config"

	interface Props {
		publicSiteCode: string
	}

	let { publicSiteCode }: Props = $props()

	onMount(async () => {
		Fathom.load(publicSiteCode, {
			url: "https://cdn.usefathom.com/script.js",
			includedDomains: [config.url]
		})
	})

	$effect(() => {
		page.url.pathname
		console.log("Tracking pageview")
		Fathom.trackPageview()
	})
</script>
