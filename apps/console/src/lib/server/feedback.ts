import { db } from "./db"
import { feedback } from "./db/schema"
import type { Feedback } from "./db/schema"
import { eq, desc } from "drizzle-orm"
import { logAuditEvent } from "./audit"

export interface FeedbackEntry {
	userId?: string
	name?: string
	email?: string
	subject: string
	message: string
	rating?: number
	ipAddress?: string
	userAgent?: string
}

export async function submitFeedback(entry: FeedbackEntry): Promise<Feedback> {
	try {
		const [newFeedback] = await db
			.insert(feedback)
			.values({
				userId: entry.userId,
				name: entry.name,
				email: entry.email,
				subject: entry.subject,
				message: entry.message,
				rating: entry.rating,
				status: "open",
				ipAddress: entry.ipAddress,
				userAgent: entry.userAgent,
				createdAt: new Date(),
				updatedAt: new Date()
			})
			.returning()

		// Log feedback submission
		await logAuditEvent({
			userId: entry.userId,
			action: "FEEDBACK_SUBMITTED",
			resource: "feedback",
			resourceId: newFeedback.id.toString(),
			details: {
				subject: entry.subject,
				rating: entry.rating,
				hasEmail: !!entry.email
			},
			ipAddress: entry.ipAddress,
			userAgent: entry.userAgent
		})

		return newFeedback
	} catch (error) {
		console.error("Failed to submit feedback:", error)

		// Log error
		await logAuditEvent({
			userId: entry.userId,
			action: "FEEDBACK_ERROR",
			resource: "feedback",
			details: { subject: entry.subject, error: "Database error" },
			ipAddress: entry.ipAddress,
			userAgent: entry.userAgent
		})

		throw new Error("Failed to submit feedback")
	}
}

export async function getFeedback(limit: number = 50, status?: string): Promise<Feedback[]> {
	try {
		if (status) {
			return await db
				.select()
				.from(feedback)
				.where(eq(feedback.status, status))
				.orderBy(desc(feedback.createdAt))
				.limit(limit)
		}

		return await db.select().from(feedback).orderBy(desc(feedback.createdAt)).limit(limit)
	} catch (error) {
		console.error("Failed to fetch feedback:", error)
		return []
	}
}

export async function getFeedbackById(id: number): Promise<Feedback | null> {
	try {
		const [result] = await db.select().from(feedback).where(eq(feedback.id, id))

		return result || null
	} catch (error) {
		console.error("Failed to fetch feedback by ID:", error)
		return null
	}
}

export async function updateFeedbackStatus(
	id: number,
	status: string,
	userId?: string
): Promise<boolean> {
	try {
		await db
			.update(feedback)
			.set({
				status,
				updatedAt: new Date()
			})
			.where(eq(feedback.id, id))

		// Log status update
		await logAuditEvent({
			userId,
			action: "FEEDBACK_STATUS_UPDATED",
			resource: "feedback",
			resourceId: id.toString(),
			details: { newStatus: status }
		})

		return true
	} catch (error) {
		console.error("Failed to update feedback status:", error)
		return false
	}
}

export async function getFeedbackStats(): Promise<{
	total: number
	open: number
	reviewing: number
	closed: number
	averageRating: number
}> {
	try {
		const allFeedback = await db.select().from(feedback)

		const stats = {
			total: allFeedback.length,
			open: allFeedback.filter((f) => f.status === "open").length,
			reviewing: allFeedback.filter((f) => f.status === "reviewing").length,
			closed: allFeedback.filter((f) => f.status === "closed").length,
			averageRating: 0
		}

		const ratingsWithValues = allFeedback.filter((f) => f.rating !== null)
		if (ratingsWithValues.length > 0) {
			const totalRating = ratingsWithValues.reduce((sum, f) => sum + (f.rating || 0), 0)
			stats.averageRating = Math.round((totalRating / ratingsWithValues.length) * 10) / 10
		}

		return stats
	} catch (error) {
		console.error("Failed to get feedback stats:", error)
		return {
			total: 0,
			open: 0,
			reviewing: 0,
			closed: 0,
			averageRating: 0
		}
	}
}
