<script lang="ts">
	import { goto } from "$app/navigation"
	import { onMount } from "svelte"

	interface Props {
		url: string
	}

	let { url }: Props = $props()

	function redirect() {
		goto(url)
	}

	onMount(() => {
		redirect()
	})
</script>
