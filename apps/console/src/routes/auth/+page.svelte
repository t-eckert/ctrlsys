<script lang="ts">
	import type { PageProps } from "./$types"
	import Json from "$lib/utils/components/json.svelte"

	let props: PageProps = $props()
</script>

<Json {props} />
