<script lang="ts">
	interface Props {
		src: string
		alt: string
	}

	let { src, alt } = $props()
</script>

<img {src} {alt} class="rounded" />
