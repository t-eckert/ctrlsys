<script lang="ts">
	type Props = {
		src: string
		alt: string
		width?: number | string
		height?: number | string
		loading?: "eager" | "lazy"
	}

	const { src, alt, width, height, loading }: Props = $props()
</script>

<img {src} {alt} {width} {height} {loading} class="" />
