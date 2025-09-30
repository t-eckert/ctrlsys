import { setContext, getContext } from "svelte"

class NavigationStore {
	currentPath: string = $state("/")

	setCurrentPath(path: string) {
		this.currentPath = path
	}

	isActive(path: string): boolean {
		if (path === "/") {
			return this.currentPath === "/"
		}
		return this.currentPath.startsWith(path)
	}
}

const NAVIGATION_KEY = Symbol("navigation")

export function initNavigationStore() {
	return setContext(NAVIGATION_KEY, new NavigationStore())
}

export function getNavigationStore() {
	return getContext<ReturnType<typeof initNavigationStore>>(NAVIGATION_KEY)
}