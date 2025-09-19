import type { Actions, PageServerLoad } from "./$types"
import { fail, redirect } from "@sveltejs/kit"
import { submitFeedback } from "$lib/server/feedback"

export const load: PageServerLoad = async ({ locals }) => {
	return {
		user: locals.user || null
	}
}

export const actions: Actions = {
	default: async (event) => {
		const formData = await event.request.formData()
		const subject = formData.get("subject")
		const message = formData.get("message")
		const name = formData.get("name")
		const email = formData.get("email")
		const rating = formData.get("rating")

		// Validation
		if (!subject || typeof subject !== "string" || subject.trim().length === 0) {
			return fail(400, {
				error: "Subject is required",
				values: { subject, message, name, email, rating }
			})
		}

		if (!message || typeof message !== "string" || message.trim().length === 0) {
			return fail(400, {
				error: "Message is required",
				values: { subject, message, name, email, rating }
			})
		}

		if (subject.length > 200) {
			return fail(400, {
				error: "Subject must be less than 200 characters",
				values: { subject, message, name, email, rating }
			})
		}

		if (message.length > 2000) {
			return fail(400, {
				error: "Message must be less than 2000 characters",
				values: { subject, message, name, email, rating }
			})
		}

		// Validate email if provided
		if (email && typeof email === "string" && email.trim().length > 0) {
			const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
			if (!emailRegex.test(email)) {
				return fail(400, {
					error: "Please enter a valid email address",
					values: { subject, message, name, email, rating }
				})
			}
		}

		// Validate rating if provided
		let ratingValue: number | undefined
		if (rating && typeof rating === "string" && rating.trim().length > 0) {
			ratingValue = parseInt(rating)
			if (isNaN(ratingValue) || ratingValue < 1 || ratingValue > 5) {
				return fail(400, {
					error: "Rating must be between 1 and 5",
					values: { subject, message, name, email, rating }
				})
			}
		}

		try {
			await submitFeedback({
				userId: event.locals.user?.id,
				name: name && typeof name === "string" ? name.trim() || undefined : undefined,
				email: email && typeof email === "string" ? email.trim() || undefined : undefined,
				subject: subject.trim(),
				message: message.trim(),
				rating: ratingValue,
				ipAddress: event.getClientAddress(),
				userAgent: event.request.headers.get("user-agent") || undefined
			})

			return {
				success: true,
				message:
					"Thank you for your feedback! We appreciate you taking the time to help us improve."
			}
		} catch (error) {
			console.error("Feedback submission error:", error)
			return fail(500, {
				error: "Failed to submit feedback. Please try again.",
				values: { subject, message, name, email, rating }
			})
		}
	}
}
