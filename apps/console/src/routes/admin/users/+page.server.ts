import type { Actions, PageServerLoad } from "./$types"
import { error } from "@sveltejs/kit"
import { db } from "$lib/server/db"
import { user } from "$lib/server/db/schema"

export const load: PageServerLoad = async () => {
	try {
		const users = await db
			.select({
				id: user.id,
				username: user.username,
				age: user.age
			})
			.from(user)

		return {
			users
		}
	} catch (err) {
		console.error("Failed to load users:", err)
		throw error(500, "Failed to load users")
	}
}

export const actions: Actions = {
	default: async (event) => {
		// Default action
	}
}

/*
// Page rendering options
export const prerender = true
export const ssr = true
export const csr = true
//*/
