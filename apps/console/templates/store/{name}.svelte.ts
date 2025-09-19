import { getContext, setContext } from "svelte"

class Store {
	constructor() {}
}

const KEY = Symbol("key")

export function initStore() {
	return setContext(KEY, new Store())
}

export function getStore() {
	return getContext<ReturnType<typeof initStore>>(KEY)
}
