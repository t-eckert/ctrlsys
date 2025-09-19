<script lang="ts">
	import { cva } from "cva"

	type Props = {
		href?: string
		target?: string
		rel?: string
		download?: string | boolean
		ping?: string
		hreflang?: string
		type?: string
		referrerpolicy?: string
		id?: string
		role?: string
		tabindex?: number
		title?: string
		ariaLabel?: string
		ariaDescribedBy?: string
		ariaLabelledBy?: string
		ariaExpanded?: boolean
		ariaPressed?: boolean
		ariaSelected?: boolean
		ariaCurrent?: string
		ariaHidden?: boolean
		onClick?: (event: MouseEvent) => void
		onFocus?: (event: FocusEvent) => void
		onBlur?: (event: FocusEvent) => void
		onMouseEnter?: (event: MouseEvent) => void
		onMouseLeave?: (event: MouseEvent) => void
		children?: import("svelte").Snippet
	}

	const {
		href,
		target,
		rel,
		download,
		ping,
		hreflang,
		type,
		referrerpolicy,
		id,
		role,
		tabindex,
		title,
		ariaLabel,
		ariaDescribedBy,
		ariaLabelledBy,
		ariaExpanded,
		ariaPressed,
		ariaSelected,
		ariaCurrent,
		ariaHidden,
		onClick,
		onFocus,
		onBlur,
		onMouseEnter,
		onMouseLeave,
		children
	}: Props = $props()

	// Determine if link is external
	const isExternal = $derived.by(() => {
		if (!href) return false
		return href.startsWith("http://") || href.startsWith("https://") || href.startsWith("//")
	})

	// Auto-configure target and rel for external links
	const linkTarget = $derived(target || (isExternal ? "_blank" : undefined))
	const linkRel = $derived(rel || (isExternal ? "noopener noreferrer" : undefined))

	const linkStyle = cva([
		"underline",
		"underline-offset-2",
		"decoration-2",
		"text-blue-600",
		"hover:text-blue-800",
		"focus:text-blue-800",
		"focus:outline-none",
		"focus:ring-2",
		"focus:ring-blue-500",
		"focus:ring-offset-1",
		"transition-colors",
		"duration-150",
		"dark:text-blue-400",
		"dark:hover:text-blue-200",
		"dark:focus:text-blue-200",
		"dark:focus:ring-blue-400"
	])
</script>

<a
	{href}
	target={linkTarget}
	rel={linkRel}
	{download}
	{ping}
	{hreflang}
	{type}
	{referrerpolicy}
	{id}
	{role}
	{tabindex}
	{title}
	aria-label={ariaLabel}
	aria-describedby={ariaDescribedBy}
	aria-labelledby={ariaLabelledBy}
	aria-expanded={ariaExpanded}
	aria-pressed={ariaPressed}
	aria-selected={ariaSelected}
	aria-current={ariaCurrent}
	aria-hidden={ariaHidden}
	class={linkStyle()}
	onclick={onClick}
	onfocus={onFocus}
	onblur={onBlur}
	onmouseenter={onMouseEnter}
	onmouseleave={onMouseLeave}
>
	{@render children?.()}
</a>
