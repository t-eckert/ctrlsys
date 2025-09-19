<script lang="ts">
	let { addr }: { addr: string } = $props()
</script>

<a href="mailto:{addr}" class="text-decoration-none lowercase">
	{addr}
</a>
