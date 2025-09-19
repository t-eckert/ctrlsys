import { describe, it, expect } from "vitest"
import {
	alwaysValid,
	isRequired,
	isEmail,
	minLength,
	maxLength,
	isNumeric,
	minValue,
	maxValue,
	isUrl,
	isPhoneNumber,
	isAlphanumeric,
	isUsername,
	isPassword,
	isStrongPassword,
	isDate,
	isFutureDate,
	isPastDate,
	dateRange,
	isPostalCode,
	isCreditCard,
	isIPAddress,
	isHexColor,
	containsOnly,
	doesNotContain,
	matchesRegex,
	isSlug,
	isAge,
	isWithinRange,
	requiredIf,
	validIf,
	combineValidators
} from "./validators"

describe("Basic Validators", () => {
	it("alwaysValid should always return true", () => {
		expect(alwaysValid("anything")).toEqual([true])
		expect(alwaysValid("")).toEqual([true])
		expect(alwaysValid(123)).toEqual([true])
	})

	it("isRequired should validate required fields", () => {
		expect(isRequired("test")).toEqual([true])
		expect(isRequired("")).toEqual([false, "This field is required"])
		expect(isRequired(null)).toEqual([false, "This field is required"])
		expect(isRequired(undefined)).toEqual([false, "This field is required"])
	})

	it("isEmail should validate email addresses", () => {
		expect(isEmail("test@example.com")).toEqual([true])
		expect(isEmail("")).toEqual([true]) // Empty is valid for optional fields
		expect(isEmail("invalid-email")).toEqual([false, "Must be a valid email address"])
		expect(isEmail("test@")).toEqual([false, "Must be a valid email address"])
	})
})

describe("Length Validators", () => {
	it("minLength should validate minimum length", () => {
		const validator = minLength(5)
		expect(validator("12345")).toEqual([true])
		expect(validator("123456")).toEqual([true])
		expect(validator("1234")).toEqual([false, "Must be at least 5 characters long"])
	})

	it("maxLength should validate maximum length", () => {
		const validator = maxLength(5)
		expect(validator("12345")).toEqual([true])
		expect(validator("1234")).toEqual([true])
		expect(validator("123456")).toEqual([false, "Must be no more than 5 characters long"])
	})
})

describe("Numeric Validators", () => {
	it("isNumeric should validate numbers", () => {
		expect(isNumeric("123")).toEqual([true])
		expect(isNumeric("123.45")).toEqual([true])
		expect(isNumeric("-123")).toEqual([true])
		expect(isNumeric("abc")).toEqual([false, "Must be a valid number"])
		expect(isNumeric("12a")).toEqual([false, "Must be a valid number"])
	})

	it("minValue should validate minimum value", () => {
		const validator = minValue(10)
		expect(validator("15")).toEqual([true])
		expect(validator("10")).toEqual([true])
		expect(validator("5")).toEqual([false, "Must be at least 10"])
		expect(validator("abc")).toEqual([false, "Must be at least 10"])
	})

	it("maxValue should validate maximum value", () => {
		const validator = maxValue(100)
		expect(validator("50")).toEqual([true])
		expect(validator("100")).toEqual([true])
		expect(validator("150")).toEqual([false, "Must be no more than 100"])
	})

	it("isWithinRange should validate number ranges", () => {
		const validator = isWithinRange(10, 20)
		expect(validator("15")).toEqual([true])
		expect(validator("10")).toEqual([true])
		expect(validator("20")).toEqual([true])
		expect(validator("5")).toEqual([false, "Must be between 10 and 20"])
		expect(validator("25")).toEqual([false, "Must be between 10 and 20"])
		expect(validator("abc")).toEqual([false, "Must be a valid number"])
	})
})

describe("Pattern Validators", () => {
	it("isUrl should validate URLs", () => {
		expect(isUrl("https://example.com")).toEqual([true])
		expect(isUrl("http://test.org")).toEqual([true])
		expect(isUrl("ftp://files.example.com")).toEqual([true])
		expect(isUrl("invalid-url")).toEqual([false, "Must be a valid URL"])
		expect(isUrl("http://")).toEqual([false, "Must be a valid URL"])
	})

	it("isPhoneNumber should validate phone numbers", () => {
		expect(isPhoneNumber("+1234567890")).toEqual([true])
		expect(isPhoneNumber("1234567890")).toEqual([true])
		expect(isPhoneNumber("+1 (234) 567-890")).toEqual([true])
		expect(isPhoneNumber("abc123")).toEqual([false, "Must be a valid phone number"])
		expect(isPhoneNumber("123")).toEqual([false, "Must be a valid phone number"])
	})

	it("isAlphanumeric should validate alphanumeric strings", () => {
		expect(isAlphanumeric("abc123")).toEqual([true])
		expect(isAlphanumeric("ABC")).toEqual([true])
		expect(isAlphanumeric("123")).toEqual([true])
		expect(isAlphanumeric("abc-123")).toEqual([false, "Must contain only letters and numbers"])
		expect(isAlphanumeric("abc 123")).toEqual([false, "Must contain only letters and numbers"])
	})

	it("isUsername should validate usernames", () => {
		expect(isUsername("user123")).toEqual([true])
		expect(isUsername("test_user")).toEqual([true])
		expect(isUsername("ab")).toEqual([false, "Username must be between 3 and 31 characters"])
		expect(isUsername("a".repeat(32))).toEqual([
			false,
			"Username must be between 3 and 31 characters"
		])
		expect(isUsername("user-name")).toEqual([
			false,
			"Username can only contain letters, numbers, and underscores"
		])
	})
})

describe("Password Validators", () => {
	it("isPassword should validate basic passwords", () => {
		expect(isPassword("password")).toEqual([true])
		expect(isPassword("12345")).toEqual([false, "Password must be between 6 and 255 characters"])
		expect(isPassword("a".repeat(256))).toEqual([
			false,
			"Password must be between 6 and 255 characters"
		])
	})

	it("isStrongPassword should validate strong passwords", () => {
		expect(isStrongPassword("Password123!")).toEqual([true])
		expect(isStrongPassword("short")).toEqual([
			false,
			"Password must be at least 8 characters long"
		])
		expect(isStrongPassword("password")).toEqual([
			false,
			"Password must contain at least one uppercase letter"
		])
		expect(isStrongPassword("PASSWORD123!")).toEqual([
			false,
			"Password must contain at least one lowercase letter"
		])
		expect(isStrongPassword("password123!")).toEqual([
			false,
			"Password must contain at least one uppercase letter"
		])
		expect(isStrongPassword("Password!")).toEqual([
			false,
			"Password must contain at least one number"
		])
		expect(isStrongPassword("Password123")).toEqual([
			false,
			"Password must contain at least one special character"
		])
	})
})

describe("Date Validators", () => {
	it("isDate should validate dates", () => {
		expect(isDate("2023-12-25")).toEqual([true])
		expect(isDate("December 25, 2023")).toEqual([true])
		expect(isDate("invalid-date")).toEqual([false, "Must be a valid date"])
	})

	it("isFutureDate should validate future dates", () => {
		const futureDate = new Date()
		futureDate.setDate(futureDate.getDate() + 1)

		const pastDate = new Date()
		pastDate.setDate(pastDate.getDate() - 1)

		expect(isFutureDate(futureDate.toISOString())).toEqual([true])
		expect(isFutureDate(pastDate.toISOString())).toEqual([false, "Date must be in the future"])
	})

	it("isPastDate should validate past dates", () => {
		const futureDate = new Date()
		futureDate.setDate(futureDate.getDate() + 1)

		const pastDate = new Date()
		pastDate.setDate(pastDate.getDate() - 1)

		expect(isPastDate(pastDate.toISOString())).toEqual([true])
		expect(isPastDate(futureDate.toISOString())).toEqual([false, "Date must be in the past"])
	})

	it("isAge should validate age based on birth date", () => {
		const eighteenYearsAgo = new Date()
		eighteenYearsAgo.setFullYear(eighteenYearsAgo.getFullYear() - 18)
		eighteenYearsAgo.setDate(eighteenYearsAgo.getDate() - 1) // Make sure they're past their birthday

		const tenYearsAgo = new Date()
		tenYearsAgo.setFullYear(tenYearsAgo.getFullYear() - 10)

		const validator = isAge(18)
		expect(validator(eighteenYearsAgo.toISOString())).toEqual([true])
		expect(validator(tenYearsAgo.toISOString())).toEqual([false, "Must be at least 18 years old"])
	})
})

describe("Specialized Pattern Validators", () => {
	it("isPostalCode should validate postal codes", () => {
		expect(isPostalCode("12345")).toEqual([true]) // US ZIP
		expect(isPostalCode("12345-6789")).toEqual([true]) // US ZIP+4
		expect(isPostalCode("A1A 1A1")).toEqual([true]) // Canadian
		expect(isPostalCode("SW1A 1AA")).toEqual([true]) // UK
		expect(isPostalCode("invalid")).toEqual([false, "Must be a valid postal code"])
	})

	it("isCreditCard should validate credit card numbers using Luhn algorithm", () => {
		expect(isCreditCard("4111111111111111")).toEqual([true]) // Valid test card
		expect(isCreditCard("4111-1111-1111-1111")).toEqual([true]) // With hyphens
		expect(isCreditCard("1234567890123456")).toEqual([false, "Must be a valid credit card number"])
		expect(isCreditCard("123")).toEqual([false, "Credit card number must be 13-19 digits"])
	})

	it("isIPAddress should validate IP addresses", () => {
		expect(isIPAddress("192.168.1.1")).toEqual([true])
		expect(isIPAddress("255.255.255.255")).toEqual([true])
		expect(isIPAddress("2001:0db8:85a3:0000:0000:8a2e:0370:7334")).toEqual([true])
		expect(isIPAddress("256.1.1.1")).toEqual([false, "Must be a valid IP address"])
		expect(isIPAddress("invalid-ip")).toEqual([false, "Must be a valid IP address"])
	})

	it("isHexColor should validate hex colors", () => {
		expect(isHexColor("#FF0000")).toEqual([true])
		expect(isHexColor("#f00")).toEqual([true])
		expect(isHexColor("#123ABC")).toEqual([true])
		expect(isHexColor("FF0000")).toEqual([
			false,
			"Must be a valid hex color (e.g., #FF0000 or #F00)"
		])
		expect(isHexColor("#GG0000")).toEqual([
			false,
			"Must be a valid hex color (e.g., #FF0000 or #F00)"
		])
	})

	it("isSlug should validate URL slugs", () => {
		expect(isSlug("my-url-slug")).toEqual([true])
		expect(isSlug("simple")).toEqual([true])
		expect(isSlug("multi-word-slug")).toEqual([true])
		expect(isSlug("MySlug")).toEqual([
			false,
			"Must be a valid slug (lowercase letters, numbers, and hyphens only)"
		])
		expect(isSlug("my_slug")).toEqual([
			false,
			"Must be a valid slug (lowercase letters, numbers, and hyphens only)"
		])
		expect(isSlug("my slug")).toEqual([
			false,
			"Must be a valid slug (lowercase letters, numbers, and hyphens only)"
		])
	})
})

describe("Custom Pattern Validators", () => {
	it("containsOnly should validate allowed characters", () => {
		const validator = containsOnly("a-z", "Only lowercase letters")
		expect(validator("abc")).toEqual([true])
		expect(validator("ABC")).toEqual([false, "Only lowercase letters"])
		expect(validator("123")).toEqual([false, "Only lowercase letters"])
	})

	it("doesNotContain should validate forbidden characters", () => {
		const validator = doesNotContain(" ", "No spaces allowed")
		expect(validator("nospaces")).toEqual([true])
		expect(validator("has spaces")).toEqual([false, "No spaces allowed"])
	})

	it("matchesRegex should validate custom patterns", () => {
		const validator = matchesRegex(/^[A-Z]{3}-\d{3}$/, "Format must be ABC-123")
		expect(validator("ABC-123")).toEqual([true])
		expect(validator("XYZ-789")).toEqual([true])
		expect(validator("abc-123")).toEqual([false, "Format must be ABC-123"])
		expect(validator("AB-123")).toEqual([false, "Format must be ABC-123"])
	})
})

describe("Conditional Validators", () => {
	it("requiredIf should validate conditional requirements", () => {
		const validator = requiredIf((value) => value !== "test") // Require if not "test"
		expect(validator("test")).toEqual([true]) // Condition false, so not required
		expect(validator("other")).toEqual([true]) // Has value, condition true but field has value
		expect(validator("")).toEqual([false, "This field is required"]) // Condition true and field empty
	})

	it("validIf should apply validation conditionally", () => {
		const validator = validIf(
			(value) => typeof value === "string" && value.startsWith("test"),
			isEmail
		)
		expect(validator("not-test")).toEqual([true]) // Condition not met, so validation skipped
		expect(validator("test@example.com")).toEqual([true]) // Condition met and valid email
		expect(validator("test-invalid-email")).toEqual([false, "Must be a valid email address"])
	})
})

describe("Combined Validators", () => {
	it("combineValidators should run all validators in sequence", () => {
		const validator = combineValidators(isRequired, minLength(5), isEmail)

		expect(validator("test@example.com")).toEqual([true])
		expect(validator("")).toEqual([false, "This field is required"])
		expect(validator("test")).toEqual([false, "Must be at least 5 characters long"])
		expect(validator("invalid-email")).toEqual([false, "Must be a valid email address"])
	})

	it("combineValidators should stop at first failure", () => {
		const validator = combineValidators(
			isRequired,
			minLength(10), // This will fail first
			isEmail // This won't be reached
		)

		expect(validator("short")).toEqual([false, "Must be at least 10 characters long"])
	})
})
