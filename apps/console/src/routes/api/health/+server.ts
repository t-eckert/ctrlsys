import { json, type RequestHandler } from "@sveltejs/kit"
import config from "$lib/config"

export const GET: RequestHandler = async ({ url, params, locals }) => {
	console.log("healthcheck request received", { url: url.pathname, params })

	return json({
		message: `${config.title} is healthy`,
		timestamp: new Date().toISOString()
	})
}
