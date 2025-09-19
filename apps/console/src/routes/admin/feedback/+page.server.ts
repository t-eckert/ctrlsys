import type { Actions, PageServerLoad } from "./$types"
import { fail } from "@sveltejs/kit"
import { getFeedback, getFeedbackStats, updateFeedbackStatus } from "$lib/server/feedback"

export const load: PageServerLoad = async ({ url, locals }) => {
	if (!locals.user) {
		throw new Error("Unauthorized")
	}

	const status = url.searchParams.get("status") || undefined
	const limit = parseInt(url.searchParams.get("limit") || "50")

	const [feedback, stats] = await Promise.all([getFeedback(limit, status), getFeedbackStats()])

	return {
		feedback,
		stats,
		currentFilter: status
	}
}

export const actions: Actions = {
	updateStatus: async ({ request, locals }) => {
		if (!locals.user) {
			return fail(401, { error: "Unauthorized" })
		}

		const formData = await request.formData()
		const feedbackId = formData.get("feedbackId")
		const newStatus = formData.get("status")

		if (!feedbackId || !newStatus) {
			return fail(400, { error: "Missing feedback ID or status" })
		}

		const id = parseInt(feedbackId.toString())
		if (isNaN(id)) {
			return fail(400, { error: "Invalid feedback ID" })
		}

		if (!["open", "reviewing", "closed"].includes(newStatus.toString())) {
			return fail(400, { error: "Invalid status" })
		}

		const success = await updateFeedbackStatus(id, newStatus.toString(), locals.user.id)

		if (!success) {
			return fail(500, { error: "Failed to update feedback status" })
		}

		return { success: true }
	}
}
