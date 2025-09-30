import { setContext, getContext } from "svelte"

export type SystemStatus = "healthy" | "warning" | "error" | "unknown"

class StatusBarStore {
	systemStatus: SystemStatus = $state("unknown")
	serverTime: Date = $state(new Date())
	notifications: string[] = $state([])

	setSystemStatus(status: SystemStatus) {
		this.systemStatus = status
	}

	updateServerTime() {
		this.serverTime = new Date()
	}

	addNotification(message: string) {
		this.notifications = [...this.notifications, message]
	}

	clearNotifications() {
		this.notifications = []
	}
}

const STATUSBAR_KEY = Symbol("statusbar")

export function initStatusBarStore() {
	return setContext(STATUSBAR_KEY, new StatusBarStore())
}

export function getStatusBarStore() {
	return getContext<ReturnType<typeof initStatusBarStore>>(STATUSBAR_KEY)
}