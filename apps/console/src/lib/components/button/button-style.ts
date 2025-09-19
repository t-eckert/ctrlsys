import { cva } from "cva"

const buttonStyle = cva(
	[
		"select-none",
		"cursor-pointer",
		"group",
		"flex",
		"flex-row",
		"gap-1",
		"items-center",
		"justify-center",
		"transition-all",
		"disabled:cursor-not-allowed",
		"focus:outline-blue-500",
		"focus:ring-2",
		"focus:ring-offset-1",
		"focus:ring-blue-500",
		"active:outline-blue-500"
	],
	{
		variants: {
			role: {
				primary: [
					"border",
					"shadow-inner",
					"disabled:text-neutral-100",
					"disabled:bg-neutral-200",
					"disabled:outline-neutral-200",
					"disabled:border-neutral-200",
					"disabled:dark:text-neutral-700",
					"disabled:dark:bg-neutral-800",
					"disabled:dark:outline-neutral-800",
					"disabled:dark:border-neutral-800"
				],
				secondary: [
					"backdrop-blur-2xl",
					"disabled:text-neutral-400",
					"disabled:bg-neutral-300/20",
					"disabled:outline-neutral-300/20",
					"disabled:hover:bg-neutral-300/20",
					"disabled:dark:text-neutral-400",
					"disabled:dark:bg-neutral-300/20",
					"disabled:dark:outline-neutral-300/20",
					"disabled:hover:dark:bg-neutral-300/20"
				],
				tertiary: [
					"bg-none",
					"disabled:text-stone-300",
					"hover:disabled:text-stone-300",
					"hover:disabled:bg-none"
				]
			},
			tone: {
				neutral: "",
				positive: "",
				negative: ""
			},
			shape: {
				rectangle: "rounded-md px-2 py-1",
				circle: "rounded-full aspect-square p-1.5",
				square: "rounded-md px-1 py-1 aspect-square",
				rounded: "rounded-full px-1 py-1"
			}
		},
		compoundVariants: [
			{
				role: "primary",
				tone: "neutral",
				class: [
					"text-neutral-50",
					"bg-neutral-900",
					"border-neutral-800",
					"shadow-neutral-800",
					"hover:bg-neutral-950",
					"dark:text-neutral-300",
					"dark:bg-neutral-950",
					"dark:border-neutral-800",
					"dark:shadow-neutral-800"
				]
			},
			{
				role: "primary",
				tone: "positive",
				class: [
					"text-sky-100",
					"bg-sky-600",
					"shadow-sky-500",
					"border-sky-500",
					"hover:bg-sky-500",
					"dark:text-sky-100",
					"dark:bg-sky-800",
					"dark:border-sky-700",
					"dark:shadow-sky-700"
				]
			},
			{
				role: "primary",
				tone: "negative",
				class: [
					"bg-rose-600",
					"text-rose-50",
					"outline-rose-700",
					"shadow-rose-500",
					"active:outline-blue-500",
					"disabled:text-red-200",
					"disabled:bg-red-500",
					"disabled:outline-red-500",
					"disabled:border-red-500",
					"dark:bg-red-800",
					"dark:text-red-300",
					"hover:bg-rose-700"
				]
			},
			{
				role: "secondary",
				tone: "neutral",
				class: [
					"text-neutral-800",
					"bg-neutral-300/40",
					"hover:text-neutral-900",
					"hover:bg-neutral-300/60",
					"dark:text-neutral-300",
					"dark:bg-neutral-700/40",
					"dark:hover:text-neutral-200",
					"dark:hover:bg-neutral-700/60"
				]
			},
			{
				role: "secondary",
				tone: "positive",
				class: ["bg-green-200", "text-green-800", "hover:text-green-900", "hover:bg-green-300"]
			},
			{
				role: "secondary",
				tone: "negative",
				class: ["bg-red-200", "text-red-800", "hover:text-red-900", "hover:bg-red-300"]
			},
			{
				role: "tertiary",
				tone: "neutral",
				class: [
					"text-neutral-700",
					"hover:bg-neutral-100",
					"hover:text-neutral-900",
					"outline",
					"outline-transparent",
					"focus:outline-blue-500",
					"focus:ring-4",
					"focus:ring-blue-500",
					"disabled:text-neutral-100",
					"disabled:hover:bg-transparent",
					"dark:text-neutral-300",
					"dark:hover:bg-zinc-900",
					"dark:hover:text-zinc-300"
				]
			},
			{
				role: "tertiary",
				tone: "positive",
				class: ["text-green-700", "hover:bg-green-100", "hover:text-green-900"]
			},
			{
				role: "tertiary",
				tone: "negative",
				class: ["text-red-700", "hover:bg-red-100", "hover:text-red-900"]
			}
		]
	}
)

export default buttonStyle
