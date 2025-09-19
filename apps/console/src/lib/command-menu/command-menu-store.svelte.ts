import { getContext, setContext } from "svelte"

class CommandMenuStore {
	constructor() {}
}

const KEY = Symbol("COMMAND_MENU")

export function initStore() {
	return setContext(KEY, new CommandMenuStore())
}

export function getStore() {
	return getContext<ReturnType<typeof initStore>>(KEY)
}
