export function generateId(prefix = "id"): string {
	return `${prefix}-${Math.random().toString(36).substr(2, 9)}`
}

export function mergeAriaAttributes(
	baseAttributes: Record<string, any>,
	...additionalAttributes: (Record<string, any> | undefined)[]
): Record<string, any> {
	const merged = { ...baseAttributes }

	for (const attrs of additionalAttributes) {
		if (!attrs) continue

		for (const [key, value] of Object.entries(attrs)) {
			if (key.startsWith("aria-") && value !== undefined) {
				merged[key] = value
			}
		}
	}

	return merged
}

export function combineIds(...ids: (string | undefined)[]): string | undefined {
	const validIds = ids.filter(Boolean)
	return validIds.length > 0 ? validIds.join(" ") : undefined
}

export function announceToScreenReader(
	message: string,
	priority: "polite" | "assertive" = "polite"
): void {
	const announcement = document.createElement("div")
	announcement.setAttribute("aria-live", priority)
	announcement.setAttribute("aria-atomic", "true")
	announcement.className = "sr-only"
	announcement.textContent = message

	document.body.appendChild(announcement)

	setTimeout(() => {
		document.body.removeChild(announcement)
	}, 1000)
}

export function trapFocus(element: HTMLElement): () => void {
	const focusableElements = element.querySelectorAll(
		'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
	) as NodeListOf<HTMLElement>

	const firstElement = focusableElements[0]
	const lastElement = focusableElements[focusableElements.length - 1]

	const handleTabKey = (e: KeyboardEvent) => {
		if (e.key !== "Tab") return

		if (e.shiftKey) {
			if (document.activeElement === firstElement) {
				e.preventDefault()
				lastElement.focus()
			}
		} else {
			if (document.activeElement === lastElement) {
				e.preventDefault()
				firstElement.focus()
			}
		}
	}

	element.addEventListener("keydown", handleTabKey)
	firstElement?.focus()

	return () => {
		element.removeEventListener("keydown", handleTabKey)
	}
}

export const visuallyHiddenStyles =
	"sr-only absolute w-px h-px p-0 -m-px overflow-hidden whitespace-nowrap border-0"

export function createScreenReaderOnlyText(text: string): HTMLSpanElement {
	const span = document.createElement("span")
	span.className = visuallyHiddenStyles
	span.textContent = text
	return span
}
