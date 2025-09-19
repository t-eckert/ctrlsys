import { describe, it, expect } from "vitest"
import readingTime from "./readingTime"

describe("reading time", () => {
	it("calculates reading time for a single word", () => {
		expect(readingTime("hello")).toBe(1)
	})

	it("calculates reading time for multiple words under 200", () => {
		const text = "This is a simple test with ten words total"
		expect(readingTime(text)).toBe(1)
	})

	it("calculates reading time for exactly 200 words", () => {
		const words = Array(200).fill("word").join(" ")
		expect(readingTime(words)).toBe(1)
	})

	it("calculates reading time for 201 words", () => {
		const words = Array(201).fill("word").join(" ")
		expect(readingTime(words)).toBe(2)
	})

	it("calculates reading time for 400 words", () => {
		const words = Array(400).fill("word").join(" ")
		expect(readingTime(words)).toBe(2)
	})

	it("calculates reading time for 1000 words", () => {
		const words = Array(1000).fill("word").join(" ")
		expect(readingTime(words)).toBe(5)
	})

	it("handles empty string", () => {
		expect(readingTime("")).toBe(1)
	})

	it("handles string with only spaces", () => {
		expect(readingTime("   ")).toBe(1)
	})

	it("handles custom words per minute", () => {
		const text = "This is exactly six words total"
		expect(readingTime(text, 6)).toBe(1)
		expect(readingTime(text, 3)).toBe(2)
		expect(readingTime(text, 1)).toBe(6)
	})

	it("handles text with multiple spaces between words", () => {
		const text = "word1  word2   word3    word4"
		expect(readingTime(text)).toBe(1)
	})

	it("handles text with newlines and tabs", () => {
		const text = "word1\nword2\tword3 word4"
		expect(readingTime(text)).toBe(1)
	})

	it("handles punctuation correctly", () => {
		const text = "Hello, world! How are you? I'm fine, thanks."
		expect(readingTime(text)).toBe(1)
	})

	it("rounds up partial minutes", () => {
		const words = Array(199).fill("word").join(" ")
		expect(readingTime(words)).toBe(1)
	})

	it("handles very large text", () => {
		const words = Array(10000).fill("word").join(" ")
		expect(readingTime(words)).toBe(50)
	})

	it("handles zero words per minute gracefully", () => {
		expect(() => readingTime("test", 0)).not.toThrow()
		expect(readingTime("test", 0)).toBe(Infinity)
	})

	it("handles negative words per minute", () => {
		expect(readingTime("test", -100)).toBe(-0)
	})

	it("handles decimal words per minute", () => {
		const text = "word1 word2 word3"
		expect(readingTime(text, 1.5)).toBe(2)
	})

	it("maintains default wpm when not specified", () => {
		const text = Array(200).fill("word").join(" ")
		expect(readingTime(text)).toBe(readingTime(text, 200))
	})
})
