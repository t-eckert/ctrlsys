<script lang="ts">
	import { type Snippet } from "svelte"

	let { children }: { children?: Snippet } = $props()
</script>

<tfoot>{#if children}{@render children()}{/if}</tfoot>
