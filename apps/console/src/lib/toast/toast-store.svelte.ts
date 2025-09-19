import { getContext, setContext } from "svelte"
import type Toast from "./Toast"

const duration = 5000

class ToastStore {
	private toasts: Toast[] = $state([])
	private idCounter = $state(0)

	add(toast: Toast) {
		this.toasts = [...this.toasts, toast]

		setTimeout(() => this.remove(toast), duration)

		return toast
	}

	remove(toast: Toast) {
		this.toasts = this.toasts.filter((t) => t.id !== toast.id)
	}

	removeById(id: number) {
		this.toasts = this.toasts.filter((t) => t.id !== id)
	}

	get() {
		return this.toasts
	}

	clear() {
		this.toasts = []
	}

	nextId() {
		return ++this.idCounter
	}
}

const TOASTS_KEY = Symbol("toasts")

export function initToastStore() {
	return setContext(TOASTS_KEY, new ToastStore())
}

export function getToastStore() {
	return getContext<ReturnType<typeof initToastStore>>(TOASTS_KEY)
}
