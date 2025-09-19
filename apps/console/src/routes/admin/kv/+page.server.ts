import type { Actions, PageServerLoad } from "./$types"
import { error } from "@sveltejs/kit"
import { kv } from "$lib/server/kv"

export const load: PageServerLoad = async () => {
	try {
		const keyValuePairs = await kv.list()
		return {
			keyValuePairs
		}
	} catch (err) {
		console.error("Failed to load key-value pairs:", err)
		throw error(500, "Failed to load key-value pairs")
	}
}

export const actions: Actions = {
	add: async ({ request }) => {
		const formData = await request.formData()
		const key = formData.get("key")?.toString()
		const value = formData.get("value")?.toString()
		const type = formData.get("type")?.toString()

		if (!key || !value) {
			return {
				error: "Key and value are required"
			}
		}

		try {
			let parsedValue: any = value

			// Parse value based on selected type
			if (type === "number") {
				parsedValue = Number(value)
				if (isNaN(parsedValue)) {
					return { error: "Invalid number format" }
				}
			} else if (type === "boolean") {
				parsedValue = value.toLowerCase() === "true"
			} else if (type === "json") {
				try {
					parsedValue = JSON.parse(value)
				} catch {
					return { error: "Invalid JSON format" }
				}
			}
			// string type keeps the value as-is

			await kv.set(key, parsedValue)
			return { success: true }
		} catch (err) {
			console.error("Failed to add key-value pair:", err)
			return { error: "Failed to add key-value pair" }
		}
	}
}

/*
// Page rendering options
export const prerender = true
export const ssr = true
export const csr = true
//*/
