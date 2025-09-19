<script lang="ts">
	import { type Snippet } from "svelte"

	interface Props {
		children: Snippet
		background?: string
	}

	let { children, background }: Props = $props()
</script>

<section
	class={`resize overflow-auto rounded-md border border-neutral-100 p-2 shadow ${background ?? "bg-white/80 backdrop-blur-2xl"}`}
>
	<div
		class="h-full w-full overflow-auto rounded border border-neutral-200 bg-white/80 backdrop-blur-2xl"
	>
		{@render children()}
	</div>
</section>
