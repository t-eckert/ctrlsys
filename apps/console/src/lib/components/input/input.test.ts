import { describe, it, expect } from "vitest"
import { render, fireEvent } from "@testing-library/svelte"
import Input from "./input.svelte"

describe("Input Component", () => {
	it("renders input with label", () => {
		const { getByLabelText } = render(Input, {
			props: {
				id: "test-input",
				name: "Test Input"
			}
		})

		const input = getByLabelText("Test Input")
		expect(input).toBeInTheDocument()
		expect(input).toHaveAttribute("id", "test-input")
	})

	it("shows optional indicator for non-required fields", () => {
		const { getByText } = render(Input, {
			props: {
				id: "optional-input",
				name: "Optional Field",
				required: false
			}
		})

		expect(getByText("Optional")).toBeInTheDocument()
	})

	it("does not show optional indicator for required fields", () => {
		const { queryByText } = render(Input, {
			props: {
				id: "required-input",
				name: "Required Field",
				required: true
			}
		})

		expect(queryByText("Optional")).not.toBeInTheDocument()
	})

	it("displays help text when provided", () => {
		const helpText = "This is helpful information"
		const { getByText } = render(Input, {
			props: {
				id: "help-input",
				name: "Input with help",
				helpText
			}
		})

		expect(getByText(helpText)).toBeInTheDocument()
		expect(getByText(helpText)).toHaveAttribute("id", "help-input-help-text")
	})

	it("handles initial value correctly", () => {
		const { getByDisplayValue } = render(Input, {
			props: {
				id: "value-input",
				name: "Input with value",
				initialValue: "Initial text"
			}
		})

		expect(getByDisplayValue("Initial text")).toBeInTheDocument()
	})

	it("applies input attributes correctly", () => {
		const { getByLabelText } = render(Input, {
			props: {
				id: "attr-input",
				name: "Input with attributes",
				type: "email",
				placeholder: "Enter email",
				disabled: true,
				readonly: true,
				min: "0",
				max: "100",
				step: "1"
			}
		})

		const input = getByLabelText("Input with attributes")
		expect(input).toHaveAttribute("type", "email")
		expect(input).toHaveAttribute("placeholder", "Enter email")
		expect(input).toBeDisabled()
		expect(input).toHaveAttribute("readonly")
		expect(input).toHaveAttribute("min", "0")
		expect(input).toHaveAttribute("max", "100")
		expect(input).toHaveAttribute("step", "1")
	})

	it("associates help text with aria-describedby", () => {
		const { getByLabelText } = render(Input, {
			props: {
				id: "aria-input",
				name: "Input with ARIA",
				helpText: "Help text"
			}
		})

		const input = getByLabelText("Input with ARIA")
		expect(input).toHaveAttribute("aria-describedby", "aria-input-help-text")
	})

	it("handles validation errors with aria-invalid", () => {
		const invalidValidator = () => [false, "This field is invalid"]
		const { getByLabelText } = render(Input, {
			props: {
				id: "invalid-input",
				name: "Invalid Input",
				validator: invalidValidator,
				initialValue: "invalid"
			}
		})

		const input = getByLabelText("Invalid Input")
		expect(input).toHaveAttribute("aria-invalid", "true")
	})

	it("updates value on user input", async () => {
		const { getByLabelText } = render(Input, {
			props: {
				id: "interactive-input",
				name: "Interactive Input"
			}
		})

		const input = getByLabelText("Interactive Input") as HTMLInputElement
		await fireEvent.input(input, { target: { value: "new value" } })
		expect(input.value).toBe("new value")
	})

	it("displays validation error messages", () => {
		const invalidValidator = () => [false, "Custom error message"]
		const { getByText } = render(Input, {
			props: {
				id: "error-input",
				name: "Error Input",
				validator: invalidValidator,
				initialValue: "invalid"
			}
		})

		expect(getByText("Custom error message")).toBeInTheDocument()
	})
})
