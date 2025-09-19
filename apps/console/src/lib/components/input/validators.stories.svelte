<script module>
	import { defineMeta } from "@storybook/addon-svelte-csf"
	import Input from "./input.svelte"
	import {
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
		combineValidators
	} from "./validators"

	const { Story } = defineMeta({
		title: "Components/Input/Validators",
		component: Input,
		tags: ["autodocs"],
		parameters: {
			docs: {
				description: {
					component: `
# Input Validators

Comprehensive validation system for the Input component with 30+ built-in validators.

## Basic Usage

\`\`\`svelte
<Input 
  label="Email" 
  validator={combineValidators(isRequired, isEmail)} 
  required 
/>
\`\`\`

Try typing in the inputs below to see validation in action!
					`
				}
			}
		}
	})
</script>

<!-- BASIC VALIDATORS -->

<Story
	name="Required Field"
	parameters={{
		docs: {
			description: {
				story: "Field cannot be empty. Try submitting without entering anything."
			}
		}
	}}
	args={{
		id: "required-field",
		label: "Required Field",
		validator: isRequired,
		helpText: "This field cannot be empty",
		placeholder: "Enter something...",
		required: true
	}}
/>

<Story
	name="Email Validation"
	parameters={{
		docs: {
			description: {
				story:
					"Validates email format. Try entering invalid emails like 'test@' or 'invalid-email'."
			}
		}
	}}
	args={{
		id: "email-field",
		label: "Email Address",
		type: "email",
		validator: combineValidators(isRequired, isEmail),
		helpText: "Enter a valid email address",
		placeholder: "user@example.com",
		required: true
	}}
/>

<Story
	name="Minimum Length"
	args={{
		id: "min-length",
		label: "Minimum 5 Characters",
		validator: minLength(5),
		helpText: "Must be at least 5 characters long",
		placeholder: "Enter at least 5 chars..."
	}}
/>

<Story
	name="Maximum Length"
	args={{
		id: "max-length",
		label: "Maximum 10 Characters",
		validator: maxLength(10),
		helpText: "Must be no more than 10 characters",
		placeholder: "Max 10 chars..."
	}}
/>

<!-- NUMERIC VALIDATORS -->

<Story
	name="Numeric Only"
	args={{
		id: "numeric-field",
		label: "Numeric Only",
		validator: isNumeric,
		helpText: "Only numbers allowed",
		placeholder: "123.45"
	}}
/>

<Story
	name="Minimum Value"
	args={{
		id: "min-value",
		label: "Minimum Value (10)",
		type: "number",
		validator: minValue(10),
		helpText: "Must be at least 10",
		placeholder: "Enter number >= 10"
	}}
/>

<Story
	name="Maximum Value"
	args={{
		id: "max-value",
		label: "Maximum Value (100)",
		type: "number",
		validator: maxValue(100),
		helpText: "Must be no more than 100",
		placeholder: "Enter number <= 100"
	}}
/>

<Story
	name="Number Range"
	args={{
		id: "range-value",
		label: "Range (1-50)",
		type: "number",
		validator: isWithinRange(1, 50),
		helpText: "Must be between 1 and 50",
		placeholder: "1-50"
	}}
/>

<!-- TEXT PATTERN VALIDATORS -->

<Story
	name="URL Validation"
	args={{
		id: "url-field",
		label: "Website URL",
		validator: isUrl,
		helpText: "Enter a valid URL",
		placeholder: "https://example.com"
	}}
/>

<Story
	name="Phone Number"
	args={{
		id: "phone-field",
		label: "Phone Number",
		validator: isPhoneNumber,
		helpText: "Enter a valid phone number",
		placeholder: "+1234567890"
	}}
/>

<Story
	name="Alphanumeric Only"
	args={{
		id: "alphanumeric",
		label: "Alphanumeric Only",
		validator: isAlphanumeric,
		helpText: "Only letters and numbers allowed",
		placeholder: "abc123"
	}}
/>

<Story
	name="Username Validation"
	args={{
		id: "username-field",
		label: "Username",
		validator: isUsername,
		helpText: "3-31 characters, letters, numbers, and underscores only",
		placeholder: "user_name123"
	}}
/>

<Story
	name="URL Slug"
	args={{
		id: "slug-field",
		label: "URL Slug",
		validator: isSlug,
		helpText: "Lowercase letters, numbers, and hyphens only",
		placeholder: "my-url-slug"
	}}
/>

<!-- PASSWORD VALIDATORS -->

<Story
	name="Basic Password"
	args={{
		id: "basic-password",
		label: "Basic Password",
		type: "password",
		validator: isPassword,
		helpText: "6-255 characters required",
		placeholder: "Enter password..."
	}}
/>

<Story
	name="Strong Password"
	args={{
		id: "strong-password",
		label: "Strong Password",
		type: "password",
		validator: isStrongPassword,
		helpText: "8+ chars with uppercase, lowercase, number, and special character",
		placeholder: "Password123!"
	}}
/>

<!-- DATE VALIDATORS -->

<Story
	name="Date Validation"
	args={{
		id: "date-field",
		label: "Valid Date",
		type: "date",
		validator: isDate,
		helpText: "Enter any valid date"
	}}
/>

<Story
	name="Future Date"
	args={{
		id: "future-date",
		label: "Future Date",
		type: "date",
		validator: isFutureDate,
		helpText: "Date must be in the future"
	}}
/>

<Story
	name="Past Date"
	args={{
		id: "past-date",
		label: "Birth Date",
		type: "date",
		validator: isPastDate,
		helpText: "Date must be in the past"
	}}
/>

<Story
	name="Age Validation"
	args={{
		id: "age-field",
		label: "Birth Date (18+ years)",
		type: "date",
		validator: isAge(18),
		helpText: "Must be at least 18 years old"
	}}
/>

<!-- SPECIALIZED PATTERNS -->

<Story
	name="Postal Code"
	args={{
		id: "postal-code",
		label: "Postal Code",
		validator: isPostalCode,
		helpText: "US ZIP, Canadian, or UK postal code",
		placeholder: "12345 or A1A 1A1"
	}}
/>

<Story
	name="Credit Card"
	args={{
		id: "credit-card",
		label: "Credit Card Number",
		validator: isCreditCard,
		helpText: "Valid credit card number (Luhn algorithm)",
		placeholder: "4111111111111111"
	}}
/>

<Story
	name="IP Address"
	args={{
		id: "ip-address",
		label: "IP Address",
		validator: isIPAddress,
		helpText: "IPv4 or IPv6 address",
		placeholder: "192.168.1.1"
	}}
/>

<Story
	name="Hex Color"
	args={{
		id: "hex-color",
		label: "Hex Color",
		validator: isHexColor,
		helpText: "Valid hex color code",
		placeholder: "#FF0000"
	}}
/>

<!-- CUSTOM PATTERN VALIDATORS -->

<Story
	name="Letters Only"
	args={{
		id: "letters-only",
		label: "Letters Only",
		validator: containsOnly("a-zA-Z", "Only letters allowed"),
		helpText: "No numbers or special characters",
		placeholder: "OnlyLetters"
	}}
/>

<Story
	name="No Spaces"
	args={{
		id: "no-spaces",
		label: "No Spaces Allowed",
		validator: doesNotContain(" ", "Spaces are not allowed"),
		helpText: "Cannot contain any spaces",
		placeholder: "NoSpacesHere"
	}}
/>

<Story
	name="Product Code"
	args={{
		id: "custom-regex",
		label: "Product Code (ABC-123)",
		validator: matchesRegex(/^[A-Z]{3}-\d{3}$/, "Format must be ABC-123"),
		helpText: "Three letters, dash, three numbers",
		placeholder: "ABC-123"
	}}
/>

<!-- COMBINED VALIDATORS -->

<Story
	name="Required Email"
	args={{
		id: "required-email",
		label: "Required Email",
		type: "email",
		validator: combineValidators(isRequired, isEmail),
		helpText: "Required and must be valid email",
		placeholder: "user@example.com",
		required: true
	}}
/>

<Story
	name="Strong Username"
	args={{
		id: "strong-username",
		label: "Strong Username",
		validator: combineValidators(isRequired, minLength(4), maxLength(20), isUsername),
		helpText: "Required, 4-20 chars, alphanumeric + underscores",
		placeholder: "strong_user123",
		required: true
	}}
/>

<Story
	name="Secure Password"
	args={{
		id: "secure-password",
		label: "Secure Password",
		type: "password",
		validator: combineValidators(isRequired, isStrongPassword, minLength(12)),
		helpText: "Required, strong password, min 12 characters",
		placeholder: "VerySecure123!",
		required: true
	}}
/>

<Story
	name="Adult Birth Date"
	args={{
		id: "adult-age",
		label: "Adult Birth Date",
		type: "date",
		validator: combineValidators(isRequired, isPastDate, isAge(18, 120)),
		helpText: "Required, past date, age 18-120",
		required: true
	}}
/>
