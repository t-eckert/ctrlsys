<script lang="ts">
	import { H1, H2, P } from "$lib/typography"
	import Card from "$lib/components/card/card.svelte"
	import Input from "$lib/components/input/input.svelte"
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
		combineValidators
	} from "$lib/components/input/validators"
</script>

<div class="mx-auto max-w-6xl px-4 py-12">
	<div class="mb-8 text-center">
		<H1 text="Input Validators Demo" />
		<P class="mt-3 text-gray-600 dark:text-gray-300">
			{#snippet children()}
				Comprehensive validation examples for the Input component
			{/snippet}
		</P>
	</div>

	<div class="space-y-8">
		<!-- Basic Validators -->
		<Card class="p-6">
			{#snippet children()}
				<H2 text="Basic Validators" />
				<div class="mt-6 grid grid-cols-1 gap-6 md:grid-cols-2">
					<Input
						id="required-field"
						label="Required Field"
						validator={isRequired}
						helpText="This field cannot be empty"
						required
					/>

					<Input
						id="email-field"
						label="Email Address"
						type="email"
						validator={combineValidators(isRequired, isEmail)}
						helpText="Enter a valid email address"
						required
					/>

					<Input
						id="min-length"
						label="Minimum 5 Characters"
						validator={minLength(5)}
						helpText="Must be at least 5 characters long"
					/>

					<Input
						id="max-length"
						label="Maximum 10 Characters"
						validator={maxLength(10)}
						helpText="Must be no more than 10 characters"
					/>
				</div>
			{/snippet}
		</Card>

		<!-- Numeric Validators -->
		<Card class="p-6">
			{#snippet children()}
				<H2 text="Numeric Validators" />
				<div class="mt-6 grid grid-cols-1 gap-6 md:grid-cols-2">
					<Input
						id="numeric-field"
						label="Numeric Only"
						validator={isNumeric}
						helpText="Only numbers allowed"
					/>

					<Input
						id="min-value"
						label="Minimum Value (10)"
						type="number"
						validator={minValue(10)}
						helpText="Must be at least 10"
					/>

					<Input
						id="max-value"
						label="Maximum Value (100)"
						type="number"
						validator={maxValue(100)}
						helpText="Must be no more than 100"
					/>

					<Input
						id="range-value"
						label="Range (1-50)"
						type="number"
						validator={isWithinRange(1, 50)}
						helpText="Must be between 1 and 50"
					/>
				</div>
			{/snippet}
		</Card>

		<!-- Text Pattern Validators -->
		<Card class="p-6">
			{#snippet children()}
				<H2 text="Text Pattern Validators" />
				<div class="mt-6 grid grid-cols-1 gap-6 md:grid-cols-2">
					<Input
						id="url-field"
						label="Website URL"
						validator={isUrl}
						helpText="Enter a valid URL (e.g., https://example.com)"
						placeholder="https://example.com"
					/>

					<Input
						id="phone-field"
						label="Phone Number"
						validator={isPhoneNumber}
						helpText="Enter a valid phone number"
						placeholder="+1234567890"
					/>

					<Input
						id="alphanumeric"
						label="Alphanumeric Only"
						validator={isAlphanumeric}
						helpText="Only letters and numbers allowed"
					/>

					<Input
						id="username-field"
						label="Username"
						validator={isUsername}
						helpText="3-31 characters, letters, numbers, and underscores only"
					/>
				</div>
			{/snippet}
		</Card>

		<!-- Password Validators -->
		<Card class="p-6">
			{#snippet children()}
				<H2 text="Password Validators" />
				<div class="mt-6 grid grid-cols-1 gap-6 md:grid-cols-2">
					<Input
						id="basic-password"
						label="Basic Password"
						type="password"
						validator={isPassword}
						helpText="6-255 characters required"
					/>

					<Input
						id="strong-password"
						label="Strong Password"
						type="password"
						validator={isStrongPassword}
						helpText="8+ chars with uppercase, lowercase, number, and special character"
					/>
				</div>
			{/snippet}
		</Card>

		<!-- Date Validators -->
		<Card class="p-6">
			{#snippet children()}
				<H2 text="Date Validators" />
				<div class="mt-6 grid grid-cols-1 gap-6 md:grid-cols-2">
					<Input
						id="date-field"
						label="Valid Date"
						type="date"
						validator={isDate}
						helpText="Enter any valid date"
					/>

					<Input
						id="future-date"
						label="Future Date"
						type="date"
						validator={isFutureDate}
						helpText="Date must be in the future"
					/>

					<Input
						id="past-date"
						label="Past Date (Birth Date)"
						type="date"
						validator={isPastDate}
						helpText="Date must be in the past"
					/>

					<Input
						id="age-field"
						label="Birth Date (18+ years)"
						type="date"
						validator={isAge(18)}
						helpText="Must be at least 18 years old"
					/>
				</div>
			{/snippet}
		</Card>

		<!-- Specialized Pattern Validators -->
		<Card class="p-6">
			{#snippet children()}
				<H2 text="Specialized Patterns" />
				<div class="mt-6 grid grid-cols-1 gap-6 md:grid-cols-2">
					<Input
						id="postal-code"
						label="Postal Code"
						validator={isPostalCode}
						helpText="US ZIP, Canadian, or UK postal code"
						placeholder="12345 or A1A 1A1"
					/>

					<Input
						id="credit-card"
						label="Credit Card Number"
						validator={isCreditCard}
						helpText="Valid credit card number (Luhn algorithm)"
						placeholder="4111111111111111"
					/>

					<Input
						id="ip-address"
						label="IP Address"
						validator={isIPAddress}
						helpText="IPv4 or IPv6 address"
						placeholder="192.168.1.1"
					/>

					<Input
						id="hex-color"
						label="Hex Color"
						validator={isHexColor}
						helpText="Valid hex color code"
						placeholder="#FF0000"
					/>
				</div>
			{/snippet}
		</Card>

		<!-- Custom Pattern Validators -->
		<Card class="p-6">
			{#snippet children()}
				<H2 text="Custom Patterns" />
				<div class="mt-6 grid grid-cols-1 gap-6 md:grid-cols-2">
					<Input
						id="slug-field"
						label="URL Slug"
						validator={isSlug}
						helpText="Lowercase letters, numbers, and hyphens only"
						placeholder="my-url-slug"
					/>

					<Input
						id="letters-only"
						label="Letters Only"
						validator={containsOnly("a-zA-Z", "Only letters allowed")}
						helpText="No numbers or special characters"
					/>

					<Input
						id="no-spaces"
						label="No Spaces Allowed"
						validator={doesNotContain(" ", "Spaces are not allowed")}
						helpText="Cannot contain any spaces"
					/>

					<Input
						id="custom-regex"
						label="Product Code (ABC-123)"
						validator={matchesRegex(/^[A-Z]{3}-\d{3}$/, "Format must be ABC-123")}
						helpText="Three letters, dash, three numbers"
						placeholder="ABC-123"
					/>
				</div>
			{/snippet}
		</Card>

		<!-- Combined Validators -->
		<Card class="p-6">
			{#snippet children()}
				<H2 text="Combined Validators" />
				<div class="mt-6 grid grid-cols-1 gap-6 md:grid-cols-2">
					<Input
						id="required-email"
						label="Required Email"
						type="email"
						validator={combineValidators(isRequired, isEmail)}
						helpText="Required and must be valid email"
						required
					/>

					<Input
						id="strong-username"
						label="Strong Username"
						validator={combineValidators(isRequired, minLength(4), maxLength(20), isUsername)}
						helpText="Required, 4-20 chars, alphanumeric + underscores"
						required
					/>

					<Input
						id="secure-password"
						label="Secure Password"
						type="password"
						validator={combineValidators(isRequired, isStrongPassword, minLength(12))}
						helpText="Required, strong password, min 12 characters"
						required
					/>

					<Input
						id="adult-age"
						label="Adult Birth Date"
						type="date"
						validator={combineValidators(isRequired, isPastDate, isAge(18, 120))}
						helpText="Required, past date, age 18-120"
						required
					/>
				</div>
			{/snippet}
		</Card>

		<!-- Validation Examples -->
		<Card class="p-6">
			{#snippet children()}
				<H2 text="Usage Examples" />
				<div class="mt-6 space-y-4">
					<div class="rounded-md bg-gray-50 p-4 dark:bg-gray-900">
						<h3 class="mb-2 font-semibold">Basic Usage:</h3>
						<pre class="overflow-x-auto text-sm"><code
								>{`<Input 
  label="Email" 
  validator={isEmail} 
  helpText="Enter a valid email" 
/>`}</code
							></pre>
					</div>

					<div class="rounded-md bg-gray-50 p-4 dark:bg-gray-900">
						<h3 class="mb-2 font-semibold">Combined Validators:</h3>
						<pre class="overflow-x-auto text-sm"><code
								>{`<Input 
  label="Username" 
  validator={combineValidators(
    isRequired,
    minLength(3),
    isUsername
  )}
  required 
/>`}</code
							></pre>
					</div>

					<div class="rounded-md bg-gray-50 p-4 dark:bg-gray-900">
						<h3 class="mb-2 font-semibold">Custom Validator:</h3>
						<pre class="overflow-x-auto text-sm"><code
								>{`<Input 
  label="Product Code" 
  validator={matchesRegex(
    /^[A-Z]{3}-\\d{3}$/,
    "Format must be ABC-123"
  )}
/>`}</code
							></pre>
					</div>
				</div>
			{/snippet}
		</Card>
	</div>
</div>
