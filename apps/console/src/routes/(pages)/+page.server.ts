import type { Actions, PageServerData } from "./$types"
import { error } from "@sveltejs/kit"

export const actions: Actions = {
	setTheme: async ({ url, cookies }) => {
		const theme = url.searchParams.get("theme")
		if (!theme) {
			console.error("No theme provided")
			return
		}

		if (!["dark", "light", "system"].includes(theme)) {
			console.error("Invalid theme:", theme)
			return
		}

		cookies.set("theme", theme, { path: "/", maxAge: 60 * 60 * 24 * 365 })
	}
}
