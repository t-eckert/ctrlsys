<script lang="ts">
	import { page } from "$app/state"
	import Error from "$lib/components/error/error.svelte"
</script>

{#if page.error}
	<div class="flex h-[80vh] w-full items-center justify-center">
		<Error status={page.status} message={page.error.message} />
	</div>
{/if}
