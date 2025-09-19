import type { Actions, PageServerLoad } from "./$types"
import { error } from "@sveltejs/kit"
import { getAuditLogs } from "$lib/server/audit"

export const load: PageServerLoad = async () => {
	try {
		const auditLogs = await getAuditLogs(200) // Get last 200 entries
		return {
			auditLogs
		}
	} catch (err) {
		console.error("Failed to load audit logs:", err)
		throw error(500, "Failed to load audit logs")
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
