<script lang="ts">
	import Toast from "./toast.svelte"
	import { getToastStore } from "../toast-store.svelte"

	let toastStore = getToastStore()

	let toasts = $derived(toastStore.get())
</script>

<div
	class="pointer-events-none fixed right-4 bottom-4 z-50 flex flex-col-reverse gap-3 transition-all"
>
	{#each toasts as toast (toast.id)}
		<div class="pointer-events-auto">
			<Toast {toast} />
		</div>
	{/each}
</div>
