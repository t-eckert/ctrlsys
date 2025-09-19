import { db } from "./db"
import { keyValue } from "./db/schema"
import { eq } from "drizzle-orm"

async function set(key: string, value: object | string | number | boolean): Promise<void> {
	const serializedValue = typeof value === "string" ? value : JSON.stringify(value)

	await db
		.insert(keyValue)
		.values({ key, value: serializedValue })
		.onConflictDoUpdate({
			target: keyValue.key,
			set: { value: serializedValue }
		})
}

async function get(key: string): Promise<object | string | number | boolean | null> {
	const result = await db.select().from(keyValue).where(eq(keyValue.key, key)).limit(1)

	if (result.length === 0) return null

	const value = result[0].value

	// Try to parse as JSON, fallback to string if it fails
	try {
		return JSON.parse(value)
	} catch {
		return value
	}
}

async function remove(key: string): Promise<void> {
	await db.delete(keyValue).where(eq(keyValue.key, key))
}

async function list(): Promise<Array<{ key: string; value: object | string | number | boolean }>> {
	const results = await db.select().from(keyValue)

	return results.map((row) => ({
		key: row.key,
		value: (() => {
			try {
				return JSON.parse(row.value)
			} catch {
				return row.value
			}
		})()
	}))
}

export const kv = {
	set,
	get,
	remove,
	list
}
