import { describe, it, expect, vi } from "vitest"
import { render, fireEvent } from "@testing-library/svelte"
import ButtonTestWrapper from "./button-test-wrapper.svelte"

describe("Button Component", () => {
	it("renders button with children content", () => {
		const { getByRole } = render(ButtonTestWrapper, {
			props: {
				text: "Click me"
			}
		})

		const button = getByRole("button")
		expect(button).toBeInTheDocument()
		expect(button).toHaveTextContent("Click me")
	})

	it("applies correct default attributes", () => {
		const { getByRole } = render(ButtonTestWrapper, {
			props: {
				text: "Test"
			}
		})

		const button = getByRole("button")
		expect(button).toHaveAttribute("type", "button")
		expect(button).not.toBeDisabled()
	})

	it("handles disabled state correctly", () => {
		const { getByRole } = render(ButtonTestWrapper, {
			props: {
				text: "Test",
				disabled: true
			}
		})

		const button = getByRole("button")
		expect(button).toBeDisabled()
	})

	it("calls onclick handler when clicked", async () => {
		const mockClick = vi.fn()
		const { getByRole } = render(ButtonTestWrapper, {
			props: {
				text: "Test",
				onclick: mockClick
			}
		})

		const button = getByRole("button")
		await fireEvent.click(button)
		expect(mockClick).toHaveBeenCalledTimes(1)
	})

	it("renders as link when href is provided", () => {
		const { getByRole } = render(ButtonTestWrapper, {
			props: {
				text: "Link",
				href: "/test"
			}
		})

		const link = getByRole("button")
		expect(link.tagName).toBe("A")
		expect(link).toHaveAttribute("href", "/test")
	})

	it("applies accessibility attributes correctly", () => {
		const { getByRole } = render(ButtonTestWrapper, {
			props: {
				text: "Test",
				ariaLabel: "Custom label",
				ariaDescribedBy: "help-text",
				ariaExpanded: true
			}
		})

		const button = getByRole("button")
		expect(button).toHaveAttribute("aria-label", "Custom label")
		expect(button).toHaveAttribute("aria-describedby", "help-text")
		expect(button).toHaveAttribute("aria-expanded", "true")
	})

	it("supports different button types", () => {
		const { getByRole } = render(ButtonTestWrapper, {
			props: {
				text: "Submit",
				type: "submit"
			}
		})

		const button = getByRole("button")
		expect(button).toHaveAttribute("type", "submit")
	})

	it("includes form attributes when provided", () => {
		const { getByRole } = render(ButtonTestWrapper, {
			props: {
				text: "Test",
				name: "test-button",
				value: "test-value",
				formaction: "/submit"
			}
		})

		const button = getByRole("button")
		expect(button).toHaveAttribute("name", "test-button")
		expect(button).toHaveAttribute("value", "test-value")
		expect(button).toHaveAttribute("formaction", "/submit")
	})
})
