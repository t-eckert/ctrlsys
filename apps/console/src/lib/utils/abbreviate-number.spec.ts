import { describe, it, expect } from "vitest"
import abbreviateNumber from "./abbreviate-number"

describe("abbreviate number", () => {
	it("returns full number below 1,000", () => {
		expect(abbreviateNumber(325)).toBe("325")
	})
	it("appends K to numbers between 1,000 and 1,000,000", () => {
		expect(abbreviateNumber(129320)).toBe("129K")
	})
	it("appends M to numbers between 1,000,000 and 1,000,000,000", () => {
		expect(abbreviateNumber(38000000)).toBe("38M")
	})
	it("appends B to numbers between 1,000,000,000 and 1,000,000,000,000", () => {
		expect(abbreviateNumber(2500000000)).toBe("2B")
	})
	it("appends T to numbers between 1,000,000,000,000 and 1,000,000,000,000,000", () => {
		expect(abbreviateNumber(1500000000000)).toBe("1T")
	})
	it("returns full number for negative values", () => {
		expect(abbreviateNumber(-325)).toBe("-325")
	})
	it("handles zero correctly", () => {
		expect(abbreviateNumber(0)).toBe("0")
	})
	it("handles large numbers correctly", () => {
		expect(abbreviateNumber(1234567890123)).toBe("1T")
		expect(abbreviateNumber(9876543210987654321)).toBe("9T")
	})
	it("returns empty string for NaN", () => {
		expect(abbreviateNumber(NaN)).toBe("NaN")
	})
	it("returns empty string for Infinity", () => {
		expect(abbreviateNumber(Infinity)).toBe("Infinity")
	})
	it("returns empty string for -Infinity", () => {
		expect(abbreviateNumber(-Infinity)).toBe("-Infinity")
	})
	it("returns empty string for non-numeric input", () => {
		expect(abbreviateNumber(Number("not a number"))).toBe("NaN")
	})
	it("returns empty string for null", () => {
		expect(abbreviateNumber(null as any)).toBe("NaN")
	})
	it("returns empty string for undefined", () => {
		expect(abbreviateNumber(undefined as any)).toBe("NaN")
	})
	it("returns empty string for empty string input", () => {
		expect(abbreviateNumber(Number(""))).toBe("NaN")
	})
	it("returns empty string for whitespace string input", () => {
		expect(abbreviateNumber(Number("   "))).toBe("NaN")
	})
	it("returns empty string for boolean true input", () => {
		expect(abbreviateNumber(Number(true))).toBe("1")
	})
	it("returns empty string for boolean false input", () => {
		expect(abbreviateNumber(Number(false))).toBe("0")
	})
	it("returns empty string for very small numbers", () => {
		expect(abbreviateNumber(0.0001)).toBe("0")
		expect(abbreviateNumber(0.9999)).toBe("0")
	})
	it("returns empty string for very large numbers", () => {
		expect(abbreviateNumber(1e30)).toBe("1e+30")
		expect(abbreviateNumber(1e100)).toBe("1e+100")
	})
})
