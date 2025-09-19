<script lang="ts">
	let { text }: { text: string } = $props()
</script>

<div class="flex h-100 w-full items-center justify-center">
	<span class="text-5xl font-semibold text-neutral-800">{text}</span>
</div>
