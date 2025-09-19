<script lang="ts">
	import Button from "$lib/components/button/button.svelte"
	import routes from "./routes"
</script>

<nav>
	<ul class="flex flex-row items-center">
		{#each routes as route}
			<li>
				<Button href={route.path} role="tertiary"
					><span class="text-sm font-medium">{route.name}</span></Button
				>
			</li>
		{/each}
	</ul>
</nav>
