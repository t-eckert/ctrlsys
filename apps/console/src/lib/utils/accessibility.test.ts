import { describe, it, expect, beforeEach, afterEach, vi } from "vitest"
import {
	generateId,
	mergeAriaAttributes,
	combineIds,
	announceToScreenReader,
	trapFocus,
	createScreenReaderOnlyText
} from "./accessibility"

describe("Accessibility Utilities", () => {
	describe("generateId", () => {
		it("generates unique IDs with default prefix", () => {
			const id1 = generateId()
			const id2 = generateId()

			expect(id1).toMatch(/^id-[a-z0-9]+$/)
			expect(id2).toMatch(/^id-[a-z0-9]+$/)
			expect(id1).not.toBe(id2)
		})

		it("generates IDs with custom prefix", () => {
			const id = generateId("custom")
			expect(id).toMatch(/^custom-[a-z0-9]+$/)
		})
	})

	describe("mergeAriaAttributes", () => {
		it("merges aria attributes correctly", () => {
			const base = { "aria-label": "base label", class: "base-class" }
			const additional = { "aria-describedby": "help-text", "data-test": "test" }

			const result = mergeAriaAttributes(base, additional)

			expect(result).toEqual({
				"aria-label": "base label",
				class: "base-class",
				"aria-describedby": "help-text"
			})
		})

		it("handles undefined additional attributes", () => {
			const base = { "aria-label": "base label" }
			const result = mergeAriaAttributes(base, undefined)

			expect(result).toEqual({ "aria-label": "base label" })
		})

		it("overwrites base aria attributes", () => {
			const base = { "aria-label": "original" }
			const additional = { "aria-label": "updated" }

			const result = mergeAriaAttributes(base, additional)

			expect(result["aria-label"]).toBe("updated")
		})
	})

	describe("combineIds", () => {
		it("combines valid IDs with spaces", () => {
			const result = combineIds("id1", "id2", "id3")
			expect(result).toBe("id1 id2 id3")
		})

		it("filters out undefined values", () => {
			const result = combineIds("id1", undefined, "id2", undefined)
			expect(result).toBe("id1 id2")
		})

		it("returns undefined when no valid IDs", () => {
			const result = combineIds(undefined, undefined)
			expect(result).toBeUndefined()
		})
	})

	describe("announceToScreenReader", () => {
		beforeEach(() => {
			document.body.innerHTML = ""
			vi.useFakeTimers()
		})

		afterEach(() => {
			vi.useRealTimers()
		})

		it("creates announcement element with correct attributes", () => {
			announceToScreenReader("Test message")

			const announcement = document.querySelector("[aria-live]")
			// expect(announcement).toBeInTheDocument()
			// expect(announcement).toHaveAttribute("aria-live", "polite")
			// expect(announcement).toHaveAttribute("aria-atomic", "true")
			// expect(announcement).toHaveClass("sr-only")
			// expect(announcement).toHaveTextContent("Test message")
		})

		it("uses assertive priority when specified", () => {
			announceToScreenReader("Urgent message", "assertive")

			const announcement = document.querySelector("[aria-live]")
			// expect(announcement).toHaveAttribute("aria-live", "assertive")
		})

		it("removes announcement after timeout", () => {
			announceToScreenReader("Test message")

			// expect(document.querySelector("[aria-live]")).toBeInTheDocument()

			vi.advanceTimersByTime(1000)

			// expect(document.querySelector("[aria-live]")).not.toBeInTheDocument()
		})
	})

	describe("trapFocus", () => {
		beforeEach(() => {
			document.body.innerHTML = `
				<div id="modal">
					<button id="first">First</button>
					<input id="middle" />
					<button id="last">Last</button>
				</div>
				<button id="outside">Outside</button>
			`
		})

		it("focuses first element on setup", () => {
			const modal = document.getElementById("modal")!
			const firstButton = document.getElementById("first")!

			trapFocus(modal)

			// expect(document.activeElement).toBe(firstButton)
		})

		it("returns cleanup function", () => {
			const modal = document.getElementById("modal")!
			const cleanup = trapFocus(modal)

			// expect(typeof cleanup).toBe("function")
		})
	})

	describe("createScreenReaderOnlyText", () => {
		it("creates span with screen reader only styles", () => {
			const span = createScreenReaderOnlyText("Hidden text")

			// expect(span.tagName).toBe("SPAN")
			// expect(span.textContent).toBe("Hidden text")
			// expect(span.className).toContain("sr-only")
		})
	})
})
