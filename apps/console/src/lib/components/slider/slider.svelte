<script lang="ts">
	import { Slider } from "bits-ui"
	import cn from "clsx"

	let value = $state(50)
</script>

<div class="w-full md:max-w-[280px]">
	<Slider.Root
		type="single"
		bind:value
		class="relative flex w-full touch-none items-center select-none"
	>
		<span class="bg-dark-10 relative h-2 w-full grow cursor-pointer overflow-hidden rounded-full">
			<Slider.Range class="bg-foreground absolute h-full" />
		</span>
		<Slider.Thumb
			index={0}
			class={cn(
				"border-border-input bg-background hover:border-dark-40 focus-visible:ring-foreground dark:bg-foreground dark:shadow-card data-active:border-dark-40 block size-[25px] cursor-pointer rounded-full border shadow-sm transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-hidden disabled:pointer-events-none disabled:opacity-50 data-active:scale-[0.98]"
			)}
		/>
	</Slider.Root>
</div>
