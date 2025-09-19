import type { Actions, PageServerLoad } from "./$types"
import { error } from "@sveltejs/kit"

export const load: PageServerLoad = async ({ params, locals, fetch, route, request, cookies }) => {
	console.log("Params:", params)
	console.log("Locals:", locals)
	console.log("Route:", route)
	console.log("Request:", request)
	console.log("Cookies:", cookies.getAll())

	return {
		message: "Data loaded successfully"
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
