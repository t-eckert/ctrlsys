import { cva } from "cva"

const badgeStyle = cva(
	[
		"text-xs",
		"min-h-6",
		"w-fit",
		"font-medium",
		"px-1.75",
		"py-0.25",
		"rounded-full",
		"flex",
		"select-none",
		"justify-center",
		"items-center",
		"border",
		"transition-all"
	],
	{
		variants: {
			color: {
				neutral: [
					"bg-neutral-100",
					"text-neutral-800",
					"border-neutral-200",
					"dark:bg-neutral-800",
					"dark:text-neutral-200",
					"dark:border-neutral-700"
				],
				red: [
					"bg-rose-100",
					"text-rose-800",
					"border-rose-200",
					"dark:bg-rose-900",
					"dark:text-rose-200",
					"dark:border-rose-800"
				],
				orange: [
					"bg-orange-50",
					"text-orange-800",
					"border-orange-200",
					"dark:bg-orange-900",
					"dark:text-orange-200",
					"dark:border-orange-800"
				],
				yellow: [
					"bg-yellow-50",
					"text-yellow-800",
					"border-yellow-200",
					"dark:bg-yellow-900",
					"dark:text-yellow-200",
					"dark:border-yellow-800"
				],
				green: [
					"bg-green-50",
					"text-green-800",
					"border-green-200",
					"dark:bg-green-900",
					"dark:text-green-200",
					"dark:border-green-800"
				],
				blue: [
					"bg-blue-50",
					"text-blue-800",
					"border-blue-200",
					"dark:bg-blue-900",
					"dark:text-blue-200",
					"dark:border-blue-800"
				],
				indigo: [
					"bg-indigo-50",
					"text-indigo-800",
					"border-indigo-200",
					"dark:bg-indigo-900",
					"dark:text-indigo-200",
					"dark:border-indigo-800"
				],
				purple: [
					"bg-purple-50",
					"text-purple-800",
					"border-purple-200",
					"dark:bg-purple-900",
					"dark:text-purple-200",
					"dark:border-purple-800"
				],
				pink: [
					"bg-pink-50",
					"text-pink-800",
					"border-pink-200",
					"dark:bg-pink-900",
					"dark:text-pink-200",
					"dark:border-pink-800"
				]
			}
		},
		defaultVariants: {
			color: "neutral"
		}
	}
)

export default badgeStyle
