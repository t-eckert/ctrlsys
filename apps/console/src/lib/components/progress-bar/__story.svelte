<script lang="ts">
	import ProgressBar from "./progress-bar.svelte"
</script>

<div class="w-full max-w-2xl space-y-8">
	<div class="space-y-6">
		<ProgressBar value={25} label="Download Progress" animated={true} />

		<ProgressBar value={50} label="Installation Progress" animated={true} />

		<ProgressBar value={75} label="Upload Progress" animated={true} />

		<ProgressBar value={100} label="Complete" animated={true} />
	</div>

	<div class="space-y-6">
		<h3 class="text-lg font-medium">Different Configurations</h3>

		<ProgressBar value={30} label="Without Animation" animated={false} />

		<ProgressBar value={80} label="Hidden Percentage" showPercentage={false} animated={true} />

		<ProgressBar value={60} max={200} label="Custom Max Value (60/200)" animated={true} />
	</div>
</div>
