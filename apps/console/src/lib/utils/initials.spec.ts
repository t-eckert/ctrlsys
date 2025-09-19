import { describe, it, expect } from "vitest"
import initials from "./initials"

describe("initials", () => {
	it("returns empty string for empty input", () => {
		expect(initials("")).toBe("")
	})

	it("returns empty string for null input", () => {
		expect(initials(null as any)).toBe("")
	})

	it("returns empty string for undefined input", () => {
		expect(initials(undefined as any)).toBe("")
	})

	it("returns single initial for single name", () => {
		expect(initials("John")).toBe("J")
	})

	it("returns single initial for single lowercase name", () => {
		expect(initials("john")).toBe("J")
	})

	it("returns two initials for first and last name", () => {
		expect(initials("John Doe")).toBe("JD")
	})

	it("returns two initials for lowercase first and last name", () => {
		expect(initials("john doe")).toBe("JD")
	})

	it("returns two initials for mixed case names", () => {
		expect(initials("jOhN dOe")).toBe("JD")
	})

	it("returns only first two initials for multiple names", () => {
		expect(initials("John Michael Doe")).toBe("JM")
	})

	it("returns only first two initials for many names", () => {
		expect(initials("John Michael Patrick William Doe")).toBe("JM")
	})

	it("handles names with extra spaces", () => {
		expect(initials("John  Doe")).toBe("J")
	})

	it("handles names with leading spaces", () => {
		expect(initials(" John Doe")).toBe("J")
	})

	it("handles names with trailing spaces", () => {
		expect(initials("John Doe ")).toBe("JD")
	})

	it("handles names with multiple spaces", () => {
		expect(initials("  John   Doe  ")).toBe("")
	})

	it("handles single character names", () => {
		expect(initials("J")).toBe("J")
	})

	it("handles single character first and last names", () => {
		expect(initials("J D")).toBe("JD")
	})

	it("handles names with special characters", () => {
		expect(initials("Jean-Luc Picard")).toBe("JP")
	})

	it("handles names with apostrophes", () => {
		expect(initials("O'Brien Connor")).toBe("OC")
	})

	it("handles names with periods", () => {
		expect(initials("Dr. House")).toBe("DH")
	})

	it("handles names with numbers", () => {
		expect(initials("John2 Doe3")).toBe("JD")
	})

	it("handles Unicode characters", () => {
		expect(initials("José María")).toBe("JM")
	})

	it("handles names with tabs", () => {
		expect(initials("John\tDoe")).toBe("J")
	})

	it("handles names that are just spaces", () => {
		expect(initials("   ")).toBe("")
	})

	it("handles empty parts after splitting", () => {
		expect(initials("John  ")).toBe("J")
	})

	it("handles names where second part is empty", () => {
		expect(initials("John ")).toBe("J")
	})
})
