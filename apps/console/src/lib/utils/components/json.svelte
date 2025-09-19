<script lang="ts">
	import Codeblock from "$lib/components/codeblock/codeblock.svelte"

	let props = $props()
</script>

<Codeblock lang="json" code={JSON.stringify(props, null, 2)} />
