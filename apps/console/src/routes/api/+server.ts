import { json, type RequestHandler } from "@sveltejs/kit"

export const GET: RequestHandler = async ({ url, params, locals }) => {
	console.log("GET request received", { url: url.pathname, params })

	return json({
		message: "GET request received",
		timestamp: new Date().toISOString()
	})
}

export const POST: RequestHandler = async ({ request, params, locals }) => {
	const body = await request.json().catch(() => ({}))
	console.log("POST request received", { body, params })

	return json({
		message: "POST request received",
		received: body,
		timestamp: new Date().toISOString()
	})
}

export const PUT: RequestHandler = async ({ request, params, locals }) => {
	const body = await request.json().catch(() => ({}))
	console.log("PUT request received", { body, params })

	return json({
		message: "PUT request received",
		received: body,
		timestamp: new Date().toISOString()
	})
}

export const PATCH: RequestHandler = async ({ request, params, locals }) => {
	const body = await request.json().catch(() => ({}))
	console.log("PATCH request received", { body, params })

	return json({
		message: "PATCH request received",
		received: body,
		timestamp: new Date().toISOString()
	})
}

export const DELETE: RequestHandler = async ({ request, params, locals }) => {
	console.log("DELETE request received", { params })

	return json({
		message: "DELETE request received",
		timestamp: new Date().toISOString()
	})
}

export const HEAD: RequestHandler = async ({ url, params, locals }) => {
	console.log("HEAD request received", { url: url.pathname, params })

	return new Response(null, {
		status: 200,
		headers: {
			"Content-Type": "application/json"
		}
	})
}

export const OPTIONS: RequestHandler = async ({ request }) => {
	console.log("OPTIONS request received")

	return new Response(null, {
		status: 200,
		headers: {
			"Access-Control-Allow-Origin": "*",
			"Access-Control-Allow-Methods": "GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS",
			"Access-Control-Allow-Headers": "Content-Type, Authorization"
		}
	})
}
