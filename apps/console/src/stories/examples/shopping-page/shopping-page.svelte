<script lang="ts">
	import { H1, H2, H3, P } from "$lib/typography"
	import { Star, Leaf, Shield, ArrowRight } from "phosphor-svelte"
	import Image from "$lib/components/image/image.svelte"
	import Money from "$lib/components/money/money.svelte"
	import Button from "$lib/components/button/button.svelte"
	import Badge from "$lib/components/badge/badge.svelte"
	import Card from "$lib/components/card/card.svelte"

	// Mock product images - in real app these would be actual product photos
	const productImages = [
		"https://images.unsplash.com/photo-1521572163474-6864f9cf17ab?w=800",
		"https://images.unsplash.com/photo-1583743814966-8936f37f8302?w=800",
		"https://images.unsplash.com/photo-1576566588028-4147f3842f27?w=800"
	]

	let selectedColor = "uniform-black"
	let selectedSize = "M"
	let selectedImageIndex = 0

	const colors = [
		{ id: "uniform-black", name: "Uniform Black", hex: "#1a1a1a" },
		{ id: "navy", name: "Navy", hex: "#1e3a8a" },
		{ id: "white", name: "White", hex: "#ffffff" },
		{ id: "olive", name: "Olive", hex: "#65a30d" },
		{ id: "burgundy", name: "Burgundy", hex: "#991b1b" }
	]

	const sizes = ["XS", "S", "M", "L", "XL", "XXL"]
</script>

<!-- Navigation breadcrumb -->
<nav class="border-b border-neutral-200 bg-white">
	<div class="mx-auto max-w-7xl px-4 py-3">
		<div class="flex items-center space-x-2 text-sm text-neutral-600">
			<a href="#" class="hover:text-neutral-900">Men</a>
			<span>/</span>
			<a href="#" class="hover:text-neutral-900">Tees</a>
			<span>/</span>
			<span class="text-neutral-900">The Essential Organic Crew</span>
		</div>
	</div>
</nav>

<main class="mx-auto max-w-7xl px-4 py-8">
	<div class="grid grid-cols-1 gap-8 lg:grid-cols-2">
		<!-- Product Images -->
		<div class="space-y-4">
			<div class="aspect-square overflow-hidden rounded-lg bg-neutral-100">
				<Image
					src={productImages[selectedImageIndex]}
					alt="The Essential Organic Crew in {colors.find((c) => c.id === selectedColor)?.name}"
					class="h-full w-full object-cover object-center"
				/>
			</div>
			<div class="grid grid-cols-3 gap-4">
				{#each productImages as image, index}
					<button
						class="aspect-square overflow-hidden rounded-lg bg-neutral-100 {selectedImageIndex ===
						index
							? 'ring-2 ring-neutral-900'
							: ''}"
						on:click={() => (selectedImageIndex = index)}
					>
						<Image
							src={image}
							alt="Product view {index + 1}"
							class="h-full w-full object-cover object-center"
						/>
					</button>
				{/each}
			</div>
		</div>

		<!-- Product Details -->
		<div class="space-y-6">
			<div>
				<H1 text="The Essential Organic Crew" />
				<div class="mt-2 flex items-center space-x-4">
					<Money value={38} currency="USD" class="text-2xl font-semibold text-neutral-900" />
					<Badge color="green" text="Organic Cotton" />
				</div>
			</div>

			<!-- Bundle offer -->
			<Card>
				<div class="flex items-center justify-between">
					<div>
						<P class="font-medium">Bundle & Save</P>
						<P class="text-sm text-neutral-600">3 for $69 (save $45)</P>
					</div>
					<Badge color="blue" text="Best Value" />
				</div>
			</Card>

			<!-- Color Selection -->
			<div class="space-y-3">
				<div class="flex items-center justify-between">
					<H3 text="Color" />
					<span class="text-sm text-neutral-600"
						>{colors.find((c) => c.id === selectedColor)?.name}</span
					>
				</div>
				<div class="flex space-x-3">
					{#each colors as color}
						<button
							class="h-8 w-8 rounded-full border-2 {selectedColor === color.id
								? 'border-neutral-900'
								: 'border-neutral-300'}"
							style="background-color: {color.hex}"
							on:click={() => (selectedColor = color.id)}
							aria-label="Select {color.name}"
						></button>
					{/each}
				</div>
			</div>

			<!-- Size Selection -->
			<div class="space-y-3">
				<div class="flex items-center justify-between">
					<H3 text="Size" />
					<button class="text-sm text-neutral-600 underline hover:text-neutral-900">
						Size Guide
					</button>
				</div>
				<div class="grid grid-cols-6 gap-2">
					{#each sizes as size}
						<button
							class="border py-2 text-center text-sm font-medium {selectedSize === size
								? 'border-neutral-900 bg-neutral-900 text-white'
								: 'border-neutral-300 hover:border-neutral-400'}"
							on:click={() => (selectedSize = size)}
						>
							{size}
						</button>
					{/each}
				</div>
			</div>

			<!-- Add to Cart -->
			<div class="space-y-3">
				<Button role="primary" class="w-full">
					{#snippet children()}
						<span class="text-base font-medium">Add to Cart</span>
					{/snippet}
				</Button>
				<Button role="secondary" class="w-full">
					{#snippet children()}
						<span class="text-base font-medium">Add to Favorites</span>
					{/snippet}
				</Button>
			</div>

			<!-- Sustainability Features -->
			<div class="space-y-4 border-t border-neutral-200 pt-6">
				<H3 text="Why We Love This" />
				<div class="space-y-3">
					<div class="flex items-start space-x-3">
						<Leaf class="mt-0.5 h-5 w-5 text-green-600" />
						<div>
							<P class="font-medium">100% Organic Cotton</P>
							<P class="text-sm text-neutral-600"
								>Grown without synthetic pesticides or fertilizers</P
							>
						</div>
					</div>
					<div class="flex items-start space-x-3">
						<Shield class="mt-0.5 h-5 w-5 text-blue-600" />
						<div>
							<P class="font-medium">Better Factories</P>
							<P class="text-sm text-neutral-600">Made in our ethical partner factory in Vietnam</P>
						</div>
					</div>
					<div class="flex items-start space-x-3">
						<Star class="mt-0.5 h-5 w-5 text-yellow-600" />
						<div>
							<P class="font-medium">Customer Favorite</P>
							<P class="text-sm text-neutral-600">4.8/5 stars from 2,847 reviews</P>
						</div>
					</div>
				</div>
			</div>

			<!-- Product Details -->
			<div class="space-y-4 border-t border-neutral-200 pt-6">
				<H3 text="Details" />
				<div class="space-y-2 text-sm">
					<P
						>Our Essential Organic Crew is cut from 100% organic cotton jersey for an easy,
						comfortable fit that gets softer with every wash.</P
					>
					<P
						>This tee features a classic crew neck, short sleeves, and a relaxed fit that's perfect
						for layering or wearing on its own.</P
					>
				</div>
				<div class="space-y-1 text-sm">
					<P class="font-medium">Fabric & Care:</P>
					<P>• 100% Organic Cotton</P>
					<P>• Machine wash cold, tumble dry low</P>
					<P>• Relaxed fit</P>
					<P>• Made in Vietnam</P>
				</div>
			</div>
		</div>
	</div>

	<!-- Reviews Section -->
	<section class="mt-16 border-t border-neutral-200 pt-16">
		<div class="mb-8">
			<H2 text="Customer Reviews" />
			<div class="mt-2 flex items-center space-x-4">
				<div class="flex items-center">
					{#each Array(5) as _, i}
						<Star class="h-5 w-5 text-yellow-400" weight="fill" />
					{/each}
				</div>
				<P class="text-neutral-600">4.8 out of 5 stars (2,847 reviews)</P>
			</div>
		</div>

		<div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
			{#each Array(6) as _, i}
				<Card>
					<div class="space-y-3">
						<div class="flex items-center space-x-2">
							{#each Array(5) as _, j}
								<Star class="h-4 w-4 text-yellow-400" weight="fill" />
							{/each}
						</div>
						<P class="font-medium">Perfect fit and quality</P>
						<P class="text-sm text-neutral-600">
							"This is my go-to tee. The organic cotton is so soft and the fit is exactly what I
							wanted. Will definitely buy more colors."
						</P>
						<P class="text-xs text-neutral-500">- Verified Buyer</P>
					</div>
				</Card>
			{/each}
		</div>
	</section>
</main>
