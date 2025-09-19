<script lang="ts">
	import Panel from "$lib/components/panel/panel.svelte"
	import PanelActions from "$lib/components/panel/panel-actions.svelte"
	import PanelBar from "$lib/components/panel/panel-bar.svelte"
	import PanelBody from "$lib/components/panel/panel-body.svelte"
	import Button from "$lib/components/button/button.svelte"

	interface Props {
		status: number
		message: string
	}

	const { status, message }: Props = $props()
</script>

<div class="w-full max-w-xs">
	<Panel>
		{#snippet bar()}<PanelBar title="Error" tone="negative" />{/snippet}
		{#snippet body()}
			<PanelBody>
				<div class="flex min-w-80 flex-col items-start gap-1">
					<div class="text-6xl font-semibold">{status}</div>
					<div>
						{message}
					</div>
				</div>
			</PanelBody>
		{/snippet}
		{#snippet actions()}
			<PanelActions>
				<Button href="/">
					<span class="text-xs font-medium">Return home</span>
				</Button>
				<Button role="tertiary" href="/feedback">
					<span class="text-xs font-medium">Give feedback</span>
				</Button>
			</PanelActions>
		{/snippet}
	</Panel>
</div>
