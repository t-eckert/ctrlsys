<script lang="ts">
	interface Props {
		value: number
		currency?: string
		class?: string
	}

	const { value, currency = "USD", class: className }: Props = $props()

	const formatMoney = (amount: number, currency: string): string => {
		return new Intl.NumberFormat("en-US", {
			style: "currency",
			currency: currency
		}).format(amount)
	}
</script>

<span class={className}>
	{formatMoney(value, currency)}
</span>
