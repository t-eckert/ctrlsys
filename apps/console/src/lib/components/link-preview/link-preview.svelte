<script lang="ts">
	import { Avatar, LinkPreview } from "bits-ui"
	import CalendarBlank from "phosphor-svelte/lib/CalendarBlank"
	import MapPin from "phosphor-svelte/lib/MapPin"

	type Props = {
		href?: string
		target?: string
		rel?: string
		avatarSrc?: string
		avatarAlt?: string
		fallback?: string
		username?: string
		description?: string
		location?: string
		joinDate?: string
	}

	const {
		href = "https://x.com/huntabyte",
		target = "_blank",
		rel = "noreferrer noopener",
		avatarSrc = "/avatar-1.png",
		avatarAlt = "@huntabyte",
		fallback = "HB",
		username = "@huntabyte",
		description = "I do things on the internet.",
		location = "FL, USA",
		joinDate = "Joined May 2020"
	}: Props = $props()
</script>

<LinkPreview.Root>
	<LinkPreview.Trigger
		{href}
		{target}
		{rel}
		class="rounded-xs underline-offset-4 hover:underline focus-visible:outline-2 focus-visible:outline-offset-8 focus-visible:outline-black"
	>
		<Avatar.Root
			class="data-[status=loaded]:border-foreground bg-muted text-muted-foreground h-12 w-12 rounded-full border border-transparent text-[17px] font-medium uppercase"
		>
			<div
				class="flex h-full w-full items-center justify-center overflow-hidden rounded-full border-2 border-transparent"
			>
				<Avatar.Image src={avatarSrc} alt={avatarAlt} />
				<Avatar.Fallback class="border-muted border">{fallback}</Avatar.Fallback>
			</div>
		</Avatar.Root>
	</LinkPreview.Trigger>
	<LinkPreview.Content
		class="border-muted bg-background shadow-popover w-[331px] rounded-xl border p-[17px]"
		sideOffset={8}
	>
		<div class="flex space-x-4">
			<Avatar.Root
				class="data-[status=loaded]:border-foreground bg-muted text-muted-foreground h-12 w-12 rounded-full border border-transparent text-[17px] font-medium uppercase"
			>
				<div
					class="flex h-full w-full items-center justify-center overflow-hidden rounded-full border-2 border-transparent"
				>
					<Avatar.Image src={avatarSrc} alt={avatarAlt} />
					<Avatar.Fallback class="border-muted border">{fallback}</Avatar.Fallback>
				</div>
			</Avatar.Root>
			<div class="space-y-1 text-sm">
				<h4 class="font-medium">{username}</h4>
				<p>{description}</p>
				<div class="text-muted-foreground flex items-center gap-[21px] pt-2 text-xs">
					{#if location}
						<div class="flex items-center text-xs">
							<MapPin class="mr-1 size-4" />
							<span>{location}</span>
						</div>
					{/if}
					{#if joinDate}
						<div class="flex items-center text-xs">
							<CalendarBlank class="mr-1 size-4" />
							<span>{joinDate}</span>
						</div>
					{/if}
				</div>
			</div>
		</div>
	</LinkPreview.Content>
</LinkPreview.Root>
