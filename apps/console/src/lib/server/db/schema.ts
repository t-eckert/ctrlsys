import { sqliteTable, text, integer } from "drizzle-orm/sqlite-core"

export const user = sqliteTable("user", {
	id: text("id").primaryKey(),
	age: integer("age"),
	username: text("username").notNull().unique(),
	passwordHash: text("password_hash").notNull()
})

export type User = typeof user.$inferSelect

export const session = sqliteTable("session", {
	id: text("id").primaryKey(),
	userId: text("user_id")
		.notNull()
		.references(() => user.id),
	expiresAt: integer("expires_at", { mode: "timestamp" }).notNull()
})

export type Session = typeof session.$inferSelect

export const keyValue = sqliteTable("key_value", {
	id: integer("id", { mode: "number" }).primaryKey({ autoIncrement: true }),
	key: text("key").notNull().unique(),
	value: text("value").notNull()
})

export type KeyValue = typeof keyValue.$inferSelect

export const auditLog = sqliteTable("audit_log", {
	id: integer("id", { mode: "number" }).primaryKey({ autoIncrement: true }),
	userId: text("user_id").references(() => user.id),
	action: text("action").notNull(),
	resource: text("resource").notNull(),
	resourceId: text("resource_id"),
	details: text("details"), // JSON string for additional data
	ipAddress: text("ip_address"),
	userAgent: text("user_agent"),
	createdAt: integer("created_at", { mode: "timestamp" })
		.notNull()
		.$defaultFn(() => new Date())
})

export type AuditLog = typeof auditLog.$inferSelect

export const feedback = sqliteTable("feedback", {
	id: integer("id", { mode: "number" }).primaryKey({ autoIncrement: true }),
	userId: text("user_id").references(() => user.id),
	name: text("name"),
	email: text("email"),
	subject: text("subject").notNull(),
	message: text("message").notNull(),
	rating: integer("rating"), // 1-5 star rating (optional)
	status: text("status").notNull().$default("open"), // open, reviewing, closed
	ipAddress: text("ip_address"),
	userAgent: text("user_agent"),
	createdAt: integer("created_at", { mode: "timestamp" })
		.notNull()
		.$defaultFn(() => new Date()),
	updatedAt: integer("updated_at", { mode: "timestamp" })
})

export type Feedback = typeof feedback.$inferSelect
