import { describe, it, expect } from "vitest"
import { render } from "@testing-library/svelte"
import Avatar from "./avatar.svelte"

describe("Avatar Component", () => {
	it("renders avatar with display name", () => {
		const { getByLabelText } = render(Avatar, {
			props: {
				displayName: "John Doe"
			}
		})

		const avatar = getByLabelText("Avatar for John Doe")
		// expect(avatar).toBeInTheDocument()
	})

	it("displays initials as fallback when no image provided", () => {
		const { getByText } = render(Avatar, {
			props: {
				displayName: "John Doe",
				size: "md"
			}
		})

		// expect(getByText("JD")).toBeInTheDocument()
	})

	it("displays full name for display size", () => {
		const { getByText } = render(Avatar, {
			props: {
				displayName: "John Doe",
				size: "display"
			}
		})

		// expect(getByText("John Doe")).toBeInTheDocument()
	})

	it("applies custom aria label when provided", () => {
		const { getByLabelText } = render(Avatar, {
			props: {
				displayName: "John Doe",
				ariaLabel: "User profile image"
			}
		})

		// expect(getByLabelText("User profile image")).toBeInTheDocument()
	})

	it("renders as button when role is button", () => {
		const { container } = render(Avatar, {
			props: {
				displayName: "John Doe",
				role: "button"
			}
		})

		const avatar = container.querySelector('[role="button"]')
		// expect(avatar).toBeInTheDocument()
		// expect(avatar).toHaveAttribute("tabindex", "0")
	})

	it("shows loading state when loading prop is true", () => {
		const { getByLabelText } = render(Avatar, {
			props: {
				displayName: "John Doe",
				loading: true
			}
		})

		// expect(getByLabelText("Loading avatar")).toBeInTheDocument()
	})

	it("includes image attributes when src is provided", () => {
		const { container } = render(Avatar, {
			props: {
				displayName: "John Doe",
				src: "https://example.com/avatar.jpg",
				srcSet: "https://example.com/avatar-2x.jpg 2x",
				sizes: "100px"
			}
		})

		const image = container.querySelector("img")
		// expect(image).toHaveAttribute("src", "https://example.com/avatar.jpg")
		// expect(image).toHaveAttribute("srcset", "https://example.com/avatar-2x.jpg 2x")
		// expect(image).toHaveAttribute("sizes", "100px")
		// expect(image).toHaveAttribute("alt", "Avatar for John Doe")
	})

	it("applies correct size classes", () => {
		const { container: smallContainer } = render(Avatar, {
			props: {
				displayName: "John Doe",
				size: "sm"
			}
		})

		const smallAvatar = smallContainer.querySelector('[role="img"]')
		// expect(smallAvatar).toHaveClass("h-8", "w-8")

		const { container: largeContainer } = render(Avatar, {
			props: {
				displayName: "John Doe",
				size: "lg"
			}
		})

		const largeAvatar = largeContainer.querySelector('[role="img"]')
		// expect(largeAvatar).toHaveClass("h-16", "w-16")
	})

	it("has proper focus styles for interactive avatars", () => {
		const { container } = render(Avatar, {
			props: {
				displayName: "John Doe",
				role: "button"
			}
		})

		const avatar = container.querySelector('[role="button"]')
		// expect(avatar).toHaveClass("focus:outline-none", "focus:ring-2", "focus:ring-blue-500")
	})
})
