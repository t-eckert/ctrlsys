import { expect, test } from "vitest"
import slug from "./slug"

test("slug correctly creates a slug for a name", () => {
	const given = "Thomas' Blog"
	const expected = "thomas-blog"

	const actual = slug(given)

	expect(actual).toBe(expected)
})
