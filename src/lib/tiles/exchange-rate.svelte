<script lang="ts">
	interface BankAPIResponse {
		terms: {
			url: string;
		};
		seriesDetail: {
			[key: string]: {
				label: string;
				description: string;
				dimension: {
					key: string;
					name: string;
				};
			};
		};
		observations: Array<{
			d: string;
			FXCADUSD: {
				v: string;
			};
		}>;
	}

	let exchangeRate = fetch(
		'https://www.bankofcanada.ca/valet/observations/FXCADUSD/json?recent=1&order_dir=desc'
	)
		.then((x) => x.json())
		.then((x: BankAPIResponse) => x.observations.at(0)?.FXCADUSD.v);
</script>

<section class="col-span-1 rounded-sm shadow">
	<div class="w-full rounded-t-sm bg-zinc-950 px-0.5 text-xs text-white">
		<h1>CAD to USD Exchange Rate</h1>
	</div>
	<div
		class="flex w-full flex-row items-center justify-between gap-0.5 rounded-b-sm border-x border-b border-zinc-950 bg-white p-0.5 font-mono"
	>
		{#await exchangeRate then data}
			<div class="relative">
				<span
					class={`absolute -top-4 left-0 text-2xl ${Number(data) > 1 ? 'scale-110' : 'scale-90'}`}
					>🇨🇦</span
				>
				<span
					class={`absolute -top-4 left-7 text-2xl ${Number(data) > 1 ? 'scale-90' : 'scale-110'}`}
					>🇺🇸</span
				>
			</div>
			<span class="rounded-sm bg-zinc-200 pl-3 pr-1 text-sm">{data}</span>
		{/await}
	</div>
</section>
