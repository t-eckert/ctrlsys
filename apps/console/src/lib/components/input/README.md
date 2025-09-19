# Input Component Validators

This directory contains a comprehensive validation system for the Input component with 30+ built-in validators.

## Basic Usage

```svelte
<script>
	import Input from "$lib/components/input/input.svelte"
	import { isRequired, isEmail, combineValidators } from "$lib/components/input/validators"
</script>

<Input label="Email" validator={combineValidators(isRequired, isEmail)} required />
```

## Available Validators

### Basic Validators

- `alwaysValid` - Always passes (default)
- `isRequired` - Field cannot be empty
- `minLength(n)` - Minimum character length
- `maxLength(n)` - Maximum character length

### Numeric Validators

- `isNumeric` - Must be a valid number
- `minValue(n)` - Minimum numeric value
- `maxValue(n)` - Maximum numeric value
- `isWithinRange(min, max)` - Number within range

### Text Pattern Validators

- `isEmail` - Valid email address
- `isUrl` - Valid URL
- `isPhoneNumber` - Valid phone number (international formats)
- `isAlphanumeric` - Letters and numbers only
- `isUsername` - Valid username (3-31 chars, alphanumeric + underscore)
- `isSlug` - URL-safe slug format

### Password Validators

- `isPassword` - Basic password (6-255 characters)
- `isStrongPassword` - Strong password with uppercase, lowercase, number, and special character

### Date Validators

- `isDate` - Valid date
- `isFutureDate` - Date must be in the future
- `isPastDate` - Date must be in the past
- `dateRange(min, max)` - Date within specified range
- `isAge(minAge, maxAge?)` - Age validation based on birth date

### Specialized Patterns

- `isPostalCode` - US ZIP, Canadian, or UK postal codes
- `isCreditCard` - Valid credit card number (Luhn algorithm)
- `isIPAddress` - IPv4 or IPv6 address
- `isHexColor` - Hex color code (#FF0000 or #F00)

### Custom Pattern Validators

- `containsOnly(chars, description?)` - Only specified characters allowed
- `doesNotContain(chars, description?)` - Specified characters forbidden
- `matchesRegex(regex, errorMessage)` - Custom regex pattern

### File Validators

- `isFileSize(maxSizeMB)` - File size validation
- `isFileType(allowedTypes)` - File type validation

### Conditional Validators

- `requiredIf(condition, message?)` - Required based on condition
- `validIf(condition, validator)` - Apply validator conditionally

### Combining Validators

- `combineValidators(...validators)` - Run multiple validators in sequence

## Examples

### Basic Email Validation

```svelte
<Input label="Email" validator={combineValidators(isRequired, isEmail)} required />
```

### Strong Password

```svelte
<Input
	type="password"
	label="Password"
	validator={combineValidators(isRequired, isStrongPassword, minLength(12))}
	required
/>
```

### Custom Pattern

```svelte
<Input
	label="Product Code"
	validator={matchesRegex(/^[A-Z]{3}-\d{3}$/, "Format must be ABC-123")}
	placeholder="ABC-123"
/>
```

### Age Validation

```svelte
<Input
	type="date"
	label="Birth Date"
	validator={combineValidators(isRequired, isPastDate, isAge(18, 120))}
	required
/>
```

## Demo

Visit `/demo/validators` to see all validators in action with live examples.

## Testing

The validators are thoroughly tested with 31 test cases covering all functionality. Run tests with:

```bash
npm run test:unit -- validators.test.ts
```

## Custom Validators

Create custom validators by following the `Validator` type:

```typescript
type Validator = (value: string | number) => [boolean, string?]

const isCustom: Validator = (value) => {
  if (/* your condition */) {
    return [false, "Custom error message"]
  }
  return [true]
}
```
