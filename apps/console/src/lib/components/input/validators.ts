import * as EmailValidator from "email-validator"

export type Validator = (value: string | number | File | null | undefined) => [boolean, string?]

export const alwaysValid: Validator = () => [true]

export const isEmail: Validator = (value) => {
	if (value == null || value === "") {
		return [true]
	}

	if (typeof value !== "string" || !EmailValidator.validate(value)) {
		return [false, "Must be a valid email address"]
	}

	return [true]
}

export const isRequired: Validator = (value) => {
	if (value === "" || value == null) {
		return [false, "This field is required"]
	}
	return [true]
}

export const minLength =
	(min: number): Validator =>
	(value) => {
		const str = String(value)
		if (str.length < min) {
			return [false, `Must be at least ${min} characters long`]
		}
		return [true]
	}

export const maxLength =
	(max: number): Validator =>
	(value) => {
		const str = String(value)
		if (str.length > max) {
			return [false, `Must be no more than ${max} characters long`]
		}
		return [true]
	}

export const isNumeric: Validator = (value) => {
	const num = Number(value)
	if (isNaN(num)) {
		return [false, "Must be a valid number"]
	}
	return [true]
}

export const minValue =
	(min: number): Validator =>
	(value) => {
		const num = Number(value)
		if (isNaN(num) || num < min) {
			return [false, `Must be at least ${min}`]
		}
		return [true]
	}

export const maxValue =
	(max: number): Validator =>
	(value) => {
		const num = Number(value)
		if (isNaN(num) || num > max) {
			return [false, `Must be no more than ${max}`]
		}
		return [true]
	}

export const isUrl: Validator = (value) => {
	try {
		new URL(String(value))
		return [true]
	} catch {
		return [false, "Must be a valid URL"]
	}
}

export const isPhoneNumber: Validator = (value) => {
	const cleaned = String(value).replace(/[\s\-\(\)]/g, "")
	const phoneRegex = /^[\+]?[1-9][\d]{9,15}$/
	if (!phoneRegex.test(cleaned)) {
		return [false, "Must be a valid phone number"]
	}
	return [true]
}

export const isAlphanumeric: Validator = (value) => {
	const alphanumericRegex = /^[a-zA-Z0-9]+$/
	if (typeof value !== "string" || !alphanumericRegex.test(value)) {
		return [false, "Must contain only letters and numbers"]
	}
	return [true]
}

export const isUsername: Validator = (value) => {
	if (typeof value !== "string") {
		return [false, "Username must be a string"]
	}
	if (value.length < 3 || value.length > 31) {
		return [false, "Username must be between 3 and 31 characters"]
	}
	if (!/^[a-zA-Z0-9_]+$/.test(value)) {
		return [false, "Username can only contain letters, numbers, and underscores"]
	}
	return [true]
}

export const isPassword: Validator = (value) => {
	if (typeof value !== "string") {
		return [false, "Password must be a string"]
	}
	if (value.length < 6 || value.length > 255) {
		return [false, "Password must be between 6 and 255 characters"]
	}
	return [true]
}

export const isStrongPassword: Validator = (value) => {
	if (typeof value !== "string") {
		return [false, "Password must be a string"]
	}
	if (value.length < 8) {
		return [false, "Password must be at least 8 characters long"]
	}
	if (!/(?=.*[a-z])/.test(value)) {
		return [false, "Password must contain at least one lowercase letter"]
	}
	if (!/(?=.*[A-Z])/.test(value)) {
		return [false, "Password must contain at least one uppercase letter"]
	}
	if (!/(?=.*\d)/.test(value)) {
		return [false, "Password must contain at least one number"]
	}
	if (!/(?=.*[@$!%*?&])/.test(value)) {
		return [false, "Password must contain at least one special character"]
	}
	return [true]
}

export const combineValidators =
	(...validators: Validator[]): Validator =>
	(value) => {
		for (const validator of validators) {
			const result = validator(value)
			if (!result[0]) {
				return result
			}
		}
		return [true]
	}

// Date and time validators
export const isDate: Validator = (value) => {
	const date = new Date(String(value))
	if (isNaN(date.getTime())) {
		return [false, "Must be a valid date"]
	}
	return [true]
}

export const isFutureDate: Validator = (value) => {
	const date = new Date(String(value))
	if (isNaN(date.getTime())) {
		return [false, "Must be a valid date"]
	}
	if (date <= new Date()) {
		return [false, "Date must be in the future"]
	}
	return [true]
}

export const isPastDate: Validator = (value) => {
	const date = new Date(String(value))
	if (isNaN(date.getTime())) {
		return [false, "Must be a valid date"]
	}
	if (date >= new Date()) {
		return [false, "Date must be in the past"]
	}
	return [true]
}

export const dateRange =
	(minDate: Date, maxDate: Date): Validator =>
	(value) => {
		const date = new Date(String(value))
		if (isNaN(date.getTime())) {
			return [false, "Must be a valid date"]
		}
		if (date < minDate || date > maxDate) {
			return [
				false,
				`Date must be between ${minDate.toLocaleDateString()} and ${maxDate.toLocaleDateString()}`
			]
		}
		return [true]
	}

// Pattern validators
export const isPostalCode: Validator = (value) => {
	// US ZIP code (5 digits or 5+4 format)
	const usZipRegex = /^\d{5}(-\d{4})?$/
	// Canadian postal code
	const caPostalRegex = /^[A-Za-z]\d[A-Za-z] \d[A-Za-z]\d$/
	// UK postal code
	const ukPostalRegex = /^[A-Za-z]{1,2}\d[A-Za-z\d]? \d[A-Za-z]{2}$/

	const str = String(value).trim()
	if (!usZipRegex.test(str) && !caPostalRegex.test(str) && !ukPostalRegex.test(str)) {
		return [false, "Must be a valid postal code"]
	}
	return [true]
}

export const isCreditCard: Validator = (value) => {
	// Luhn algorithm for credit card validation
	const cardNumber = String(value).replace(/\D/g, "")
	if (cardNumber.length < 13 || cardNumber.length > 19) {
		return [false, "Credit card number must be 13-19 digits"]
	}

	let sum = 0
	let isEven = false

	for (let i = cardNumber.length - 1; i >= 0; i--) {
		let digit = parseInt(cardNumber.charAt(i), 10)

		if (isEven) {
			digit *= 2
			if (digit > 9) {
				digit -= 9
			}
		}

		sum += digit
		isEven = !isEven
	}

	if (sum % 10 !== 0) {
		return [false, "Must be a valid credit card number"]
	}
	return [true]
}

export const isIPAddress: Validator = (value) => {
	const ipv4Regex =
		/^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/
	const ipv6Regex = /^(?:[0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}$/

	const str = String(value)
	if (!ipv4Regex.test(str) && !ipv6Regex.test(str)) {
		return [false, "Must be a valid IP address"]
	}
	return [true]
}

export const isHexColor: Validator = (value) => {
	const hexColorRegex = /^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$/
	if (!hexColorRegex.test(String(value))) {
		return [false, "Must be a valid hex color (e.g., #FF0000 or #F00)"]
	}
	return [true]
}

// Text content validators
export const containsOnly =
	(chars: string, description?: string): Validator =>
	(value) => {
		const regex = new RegExp(`^[${chars}]+$`)
		if (!regex.test(String(value))) {
			return [false, description || `Must contain only: ${chars}`]
		}
		return [true]
	}

export const doesNotContain =
	(chars: string, description?: string): Validator =>
	(value) => {
		const regex = new RegExp(`[${chars}]`)
		if (regex.test(String(value))) {
			return [false, description || `Must not contain: ${chars}`]
		}
		return [true]
	}

export const matchesRegex =
	(regex: RegExp, errorMessage: string): Validator =>
	(value) => {
		if (!regex.test(String(value))) {
			return [false, errorMessage]
		}
		return [true]
	}

export const isSlug: Validator = (value) => {
	const slugRegex = /^[a-z0-9]+(?:-[a-z0-9]+)*$/
	if (!slugRegex.test(String(value))) {
		return [false, "Must be a valid slug (lowercase letters, numbers, and hyphens only)"]
	}
	return [true]
}

// File validators
export const isFileSize =
	(maxSizeMB: number): Validator =>
	(value) => {
		if (typeof value === "object" && value instanceof File) {
			const sizeMB = value.size / (1024 * 1024)
			if (sizeMB > maxSizeMB) {
				return [false, `File size must be less than ${maxSizeMB}MB`]
			}
		}
		return [true]
	}

export const isFileType =
	(allowedTypes: string[]): Validator =>
	(value) => {
		if (typeof value === "object" && value instanceof File) {
			if (!allowedTypes.includes(value.type)) {
				return [false, `File type must be one of: ${allowedTypes.join(", ")}`]
			}
		}
		return [true]
	}

// Custom business logic validators
export const isAge =
	(minAge: number, maxAge?: number): Validator =>
	(value) => {
		const birthDate = new Date(String(value))
		if (isNaN(birthDate.getTime())) {
			return [false, "Must be a valid birth date"]
		}

		const today = new Date()
		const age = today.getFullYear() - birthDate.getFullYear()
		const monthDiff = today.getMonth() - birthDate.getMonth()
		const dayDiff = today.getDate() - birthDate.getDate()

		const actualAge = monthDiff < 0 || (monthDiff === 0 && dayDiff < 0) ? age - 1 : age

		if (actualAge < minAge) {
			return [false, `Must be at least ${minAge} years old`]
		}

		if (maxAge && actualAge > maxAge) {
			return [false, `Must be no more than ${maxAge} years old`]
		}

		return [true]
	}

export const isWithinRange =
	(min: number, max: number): Validator =>
	(value) => {
		const num = Number(value)
		if (isNaN(num)) {
			return [false, "Must be a valid number"]
		}
		if (num < min || num > max) {
			return [false, `Must be between ${min} and ${max}`]
		}
		return [true]
	}

// Conditional validators
export const requiredIf =
	(condition: (value: any) => boolean, message?: string): Validator =>
	(value) => {
		if (condition(value) && (value === "" || value == null)) {
			return [false, message || "This field is required"]
		}
		return [true]
	}

export const validIf =
	(condition: (value: any) => boolean, validator: Validator): Validator =>
	(value) => {
		if (condition(value)) {
			return validator(value)
		}
		return [true]
	}
