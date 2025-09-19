<script lang="ts">
	import { H1, P } from "$lib/typography"
	import Button from "$lib/components/button/button.svelte"
	import Card from "$lib/components/card/card.svelte"
	import config from "$lib/config"
	import { ArrowRight, Code, Sparkle, Rocket } from "phosphor-svelte"
</script>

<!-- Splash Component - Easy to remove when building your own site -->
<div class="flex min-h-[80vh] items-center justify-center px-4">
	<Card class="mx-auto max-w-4xl p-8 text-center">
		{#snippet children()}
			<!-- Project Title -->
			<div class="mb-6">
				<H1 text={config.title} />
				<P class="mt-3 text-lg text-gray-600 dark:text-gray-300">
					{#snippet children()}
						A production-ready SvelteKit starter with 40+ components, authentication, and modern
						tooling
					{/snippet}
				</P>
			</div>

			<!-- Quick Features -->
			<div class="mb-8 grid grid-cols-1 gap-4 md:grid-cols-3">
				<div class="flex items-center justify-center space-x-2 text-sm">
					<Code size={20} class="text-blue-600 dark:text-blue-400" />
					<span>40+ Components</span>
				</div>
				<div class="flex items-center justify-center space-x-2 text-sm">
					<Sparkle size={20} class="text-green-600 dark:text-green-400" />
					<span>Auth & Database</span>
				</div>
				<div class="flex items-center justify-center space-x-2 text-sm">
					<Rocket size={20} class="text-purple-600 dark:text-purple-400" />
					<span>Production Ready</span>
				</div>
			</div>

			<!-- Action Buttons -->
			<div class="flex flex-wrap justify-center gap-4">
				<Button role="primary" href="/about">
					{#snippet children()}
						Learn More
						<ArrowRight size={16} class="ml-1" />
					{/snippet}
				</Button>
				<Button role="secondary" href="/auth/register">
					{#snippet children()}
						Try Demo
					{/snippet}
				</Button>
				<Button role="tertiary" href="/feedback">
					{#snippet children()}
						Give Feedback
					{/snippet}
				</Button>
				<Button role="tertiary" href="https://github.com/t-eckert/sveltekit-kickstart">
					{#snippet children()}
						GitHub
					{/snippet}
				</Button>
			</div>

			<!-- Developer Note -->
			<div class="mt-8 border-t pt-6">
				<P class="text-xs text-gray-500 dark:text-gray-400">
					{#snippet children()}
						👋 <strong>For developers:</strong> This splash component is in
						<code class="rounded bg-gray-100 px-1 py-0.5 dark:bg-gray-800"
							>src/lib/components/splash/splash.svelte</code
						>
						— remove it to build your own site!
					{/snippet}
				</P>
			</div>
		{/snippet}
	</Card>
</div>
