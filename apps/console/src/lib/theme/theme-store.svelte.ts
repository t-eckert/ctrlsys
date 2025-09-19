import { setContext, getContext } from "svelte"

export type Theme = "dark" | "light" | "system"

class ThemeStore {
	theme: Theme = $state("system")

	constructor(theme: Theme = "system") {
		this.theme = theme
	}

	setTheme(newTheme: Theme) {
		this.theme = newTheme

		// Return early if not in a browser environment.
		if (typeof document === "undefined") {
			return
		}

		// Update the document's data-theme attribute.
		document.documentElement.setAttribute("data-theme", newTheme)
	}
}

const THEME_KEY = Symbol("theme")

export function initThemeStore(theme: Theme = "system") {
	return setContext(THEME_KEY, new ThemeStore(theme))
}

export function getThemeStore() {
	return getContext<ReturnType<typeof initThemeStore>>(THEME_KEY)
}
