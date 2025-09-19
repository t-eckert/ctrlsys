/**
 * Abbreviates a number to a more readable format.
 * For example, 1500 becomes "1K", 2500000 becomes "2M", etc.
 *
 * @param {number} num - The number to abbreviate.
 * @returns {string} The abbreviated number as a string.
 */
export default function abbreviateNumber(num: number): string {
	if (num < 1000) {
		return num.toString()
	}

	const units = ["K", "M", "B", "T"]
	let unitIndex = -1

	while (num >= 1000 && unitIndex < units.length - 1) {
		num /= 1000
		unitIndex++
	}

	return `${num.toFixed()}${units[unitIndex]}`
}
