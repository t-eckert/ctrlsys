import type { Actions, PageServerLoad } from "./$types"
import { fail, redirect } from "@sveltejs/kit"
import { verify } from "@node-rs/argon2"
import { eq } from "drizzle-orm"
import * as auth from "$lib/server/auth"
import { db } from "$lib/server/db"
import * as table from "$lib/server/db/schema"
import { logAuditEvent } from "$lib/server/audit"

export const load: PageServerLoad = async ({ locals }) => {
	// If user is already logged in, redirect to home
	if (locals.user) {
		throw redirect(302, "/")
	}

	return {
		message: "Please log in to continue"
	}
}

export const actions: Actions = {
	default: async (event) => {
		const formData = await event.request.formData()
		const username = formData.get("username")
		const password = formData.get("password")

		if (!validateUsername(username)) {
			return fail(400, { message: "Invalid username" })
		}
		if (!validatePassword(password)) {
			return fail(400, { message: "Invalid password" })
		}

		try {
			// Find user by username
			const [existingUser] = await db
				.select()
				.from(table.user)
				.where(eq(table.user.username, username))

			if (!existingUser) {
				// Log failed login attempt
				await logAuditEvent({
					action: "LOGIN_FAILED",
					resource: "user",
					details: { username, reason: "User not found" },
					ipAddress: event.getClientAddress(),
					userAgent: event.request.headers.get("user-agent") || undefined
				})
				return fail(400, { message: "Invalid username or password" })
			}

			// Verify password
			const validPassword = await verify(existingUser.passwordHash, password, {
				memoryCost: 19456,
				timeCost: 2,
				outputLen: 32,
				parallelism: 1
			})

			if (!validPassword) {
				// Log failed login attempt
				await logAuditEvent({
					userId: existingUser.id,
					action: "LOGIN_FAILED",
					resource: "user",
					resourceId: existingUser.id,
					details: { username, reason: "Invalid password" },
					ipAddress: event.getClientAddress(),
					userAgent: event.request.headers.get("user-agent") || undefined
				})
				return fail(400, { message: "Invalid username or password" })
			}

			// Create session
			const sessionToken = auth.generateSessionToken()
			const session = await auth.createSession(sessionToken, existingUser.id)
			auth.setSessionTokenCookie(event, sessionToken, session.expiresAt)

			// Log successful login
			await logAuditEvent({
				userId: existingUser.id,
				action: "LOGIN",
				resource: "session",
				resourceId: session.id,
				details: { username, method: "password" },
				ipAddress: event.getClientAddress(),
				userAgent: event.request.headers.get("user-agent") || undefined
			})

			return redirect(302, "/")
		} catch (e) {
			console.error("Login error:", e)

			// Log system error
			await logAuditEvent({
				action: "LOGIN_ERROR",
				resource: "system",
				details: { username, error: "System error during login" },
				ipAddress: event.getClientAddress(),
				userAgent: event.request.headers.get("user-agent") || undefined
			})

			return fail(500, { message: "An error occurred during login" })
		}
	}
}

function validateUsername(username: unknown): username is string {
	return (
		typeof username === "string" &&
		username.length >= 3 &&
		username.length <= 31 &&
		/^[a-z0-9_-]+$/.test(username)
	)
}

function validatePassword(password: unknown): password is string {
	return typeof password === "string" && password.length >= 6 && password.length <= 255
}

/*
// Page rendering options
export const prerender = true
export const ssr = true
export const csr = true
//*/
