import { hash, verify } from "@node-rs/argon2"
import { encodeBase32LowerCase } from "@oslojs/encoding"
import { fail, redirect } from "@sveltejs/kit"
import { eq } from "drizzle-orm"
import * as auth from "$lib/server/auth"
import { db } from "$lib/server/db"
import * as table from "$lib/server/db/schema"
import type { Actions, PageServerLoad } from "./$types"
import { logAuditEvent } from "$lib/server/audit"

export const load: PageServerLoad = async (event) => {
	if (event.locals.user) {
		return redirect(302, "/demo/lucia")
	}
	return {}
}

export const actions: Actions = {
	login: async (event) => {
		const formData = await event.request.formData()
		const username = formData.get("username")
		const password = formData.get("password")

		if (!validateUsername(username)) {
			return fail(400, { message: "Invalid username" })
		}
		if (!validatePassword(password)) {
			return fail(400, { message: "Invalid password" })
		}

		const results = await db.select().from(table.user).where(eq(table.user.username, username))

		const existingUser = results.at(0)
		if (!existingUser) {
			// Log failed login attempt - user not found
			await logAuditEvent({
				action: "LOGIN_FAILED",
				resource: "user",
				details: { username, reason: "user_not_found" },
				ipAddress: event.getClientAddress(),
				userAgent: event.request.headers.get("user-agent") || undefined
			})
			return fail(400, { message: "Incorrect username or password" })
		}

		const validPassword = await verify(existingUser.passwordHash, password, {
			memoryCost: 19456,
			timeCost: 2,
			outputLen: 32,
			parallelism: 1
		})
		if (!validPassword) {
			// Log failed login attempt - wrong password
			await logAuditEvent({
				userId: existingUser.id,
				action: "LOGIN_FAILED",
				resource: "user",
				resourceId: existingUser.id,
				details: { username, reason: "invalid_password" },
				ipAddress: event.getClientAddress(),
				userAgent: event.request.headers.get("user-agent") || undefined
			})
			return fail(400, { message: "Incorrect username or password" })
		}

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

		return redirect(302, "/demo/lucia")
	},
	register: async (event) => {
		const formData = await event.request.formData()
		const username = formData.get("username")
		const password = formData.get("password")

		if (!validateUsername(username)) {
			return fail(400, { message: "Invalid username" })
		}
		if (!validatePassword(password)) {
			return fail(400, { message: "Invalid password" })
		}

		const userId = generateUserId()
		const passwordHash = await hash(password, {
			// recommended minimum parameters
			memoryCost: 19456,
			timeCost: 2,
			outputLen: 32,
			parallelism: 1
		})

		try {
			await db.insert(table.user).values({ id: userId, username, passwordHash })

			// Log successful registration
			await logAuditEvent({
				userId,
				action: "REGISTER",
				resource: "user",
				resourceId: userId,
				details: { username },
				ipAddress: event.getClientAddress(),
				userAgent: event.request.headers.get("user-agent") || undefined
			})

			const sessionToken = auth.generateSessionToken()
			const session = await auth.createSession(sessionToken, userId)
			auth.setSessionTokenCookie(event, sessionToken, session.expiresAt)

			// Log successful login after registration
			await logAuditEvent({
				userId,
				action: "LOGIN",
				resource: "session",
				resourceId: session.id,
				details: { username, method: "registration" },
				ipAddress: event.getClientAddress(),
				userAgent: event.request.headers.get("user-agent") || undefined
			})
		} catch (e) {
			// Log failed registration attempt
			await logAuditEvent({
				action: "REGISTER_FAILED",
				resource: "user",
				details: { username, error: "Database error" },
				ipAddress: event.getClientAddress(),
				userAgent: event.request.headers.get("user-agent") || undefined
			})
			return fail(500, { message: "An error has occurred" })
		}
		return redirect(302, "/demo/lucia")
	}
}

function generateUserId() {
	// ID with 120 bits of entropy, or about the same as UUID v4.
	const bytes = crypto.getRandomValues(new Uint8Array(15))
	const id = encodeBase32LowerCase(bytes)
	return id
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
