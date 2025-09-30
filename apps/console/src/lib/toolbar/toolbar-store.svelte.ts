import { setContext, getContext } from "svelte"

class ToolbarStore {
	searchQuery: string = $state("")

	setSearchQuery(query: string) {
		this.searchQuery = query
	}

	clearSearch() {
		this.searchQuery = ""
	}
}

const TOOLBAR_KEY = Symbol("toolbar")

export function initToolbarStore() {
	return setContext(TOOLBAR_KEY, new ToolbarStore())
}

export function getToolbarStore() {
	return getContext<ReturnType<typeof initToolbarStore>>(TOOLBAR_KEY)
}