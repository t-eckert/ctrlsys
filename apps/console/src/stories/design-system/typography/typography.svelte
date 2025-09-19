<script lang="ts">
	import { Title, Subtitle, H1, H2, H3, H4, H5, H6, Li, Ol, P, Strong, Ul } from "$lib/typography"

	interface Props {
		titleText: string
		subtitleText: string
	}

	let { titleText, subtitleText }: Props = $props()
</script>

<div class="flex w-full flex-col items-start gap-4">
	<div
		class="flex w-full flex-col items-start gap-8 border-t border-t-neutral-600 pt-1 pb-1 sm:pb-8"
	>
		<div class="flex w-full flex-row items-center gap-1">
			<div class="font-semibold">Title</div>
			<div class="text-neutral-500">Inter Sans Serif</div>
		</div>
		<Title fontStyle="sans">{titleText}</Title>
	</div>

	<div
		class="flex w-full flex-col items-start gap-8 border-t border-t-neutral-600 pt-1 pb-1 sm:pb-8"
	>
		<div class="flex w-full flex-row items-center gap-1">
			<div class="font-semibold">Title</div>
			<div class="text-neutral-500">Libre Caslon Serif</div>
		</div>
		<Title fontStyle="serif">{titleText}</Title>
	</div>

	<div
		class="flex w-full flex-col items-start gap-8 border-t border-t-neutral-600 pt-1 pb-1 sm:pb-8"
	>
		<div class="flex w-full flex-row items-center gap-1">
			<div class="font-semibold">Title</div>
			<div class="text-neutral-500">Jetbrains Mono</div>
		</div>
		<Title fontStyle="mono">{titleText}</Title>
	</div>

	<div
		class="flex w-full flex-col items-start gap-4 border-t border-t-neutral-600 pt-1 pb-1 sm:pb-4"
	>
		<div class="flex w-full flex-row items-center gap-1">
			<div class="font-semibold">Subtitle</div>
			<div class="text-neutral-500">Inter Sans Serif</div>
		</div>
		<Subtitle fontStyle="sans">{subtitleText}</Subtitle>
	</div>

	<div
		class="flex w-full flex-col items-start gap-4 border-t border-t-neutral-600 pt-1 pb-1 sm:pb-4"
	>
		<div class="flex w-full flex-row items-center gap-1">
			<div class="font-semibold">Subtitle</div>
			<div class="text-neutral-500">Libre Caslon Serif</div>
		</div>
		<Subtitle fontStyle="serif">{subtitleText}</Subtitle>
	</div>

	<div
		class="flex w-full flex-col items-start gap-4 border-t border-t-neutral-600 pt-1 pb-1 sm:pb-4"
	>
		<div class="flex w-full flex-row items-center gap-1">
			<div class="font-semibold">Subtitle</div>
			<div class="text-neutral-500">Jetbrains Mono</div>
		</div>
		<Subtitle fontStyle="mono">{subtitleText}</Subtitle>
	</div>

	<div class="flex w-full flex-row items-start gap-4">
		<div
			class="flex w-full flex-col items-start gap-4 border-t border-t-neutral-600 pt-1 pb-1 sm:pb-4"
		>
			<div class="flex w-full flex-row items-center gap-1">
				<div class="font-semibold">Headings</div>
				<div class="text-neutral-500">Inter Sans Serif</div>
			</div>
			<H1 text="Heading 1" fontStyle="sans" />
			<H2 text="Heading 2" fontStyle="sans" />
			<H3 text="Heading 3" fontStyle="sans" />
			<H4 text="Heading 4" fontStyle="sans" />
			<H5 text="Heading 5" fontStyle="sans" />
			<H6 text="Heading 6" fontStyle="sans" />
		</div>

		<div
			class="flex w-full flex-col items-start gap-4 border-t border-t-neutral-600 pt-1 pb-1 sm:pb-4"
		>
			<div class="flex w-full flex-row items-center gap-1">
				<div class="font-semibold">Headings</div>
				<div class="text-neutral-500">Caslon Libre Serif</div>
			</div>
			<H1 text="Heading 1" fontStyle="serif" />
			<H2 text="Heading 2" fontStyle="serif" />
			<H3 text="Heading 3" fontStyle="serif" />
			<H4 text="Heading 4" fontStyle="serif" />
			<H5 text="Heading 5" fontStyle="serif" />
			<H6 text="Heading 6" fontStyle="serif" />
		</div>

		<div
			class="flex w-full flex-col items-start gap-4 border-t border-t-neutral-600 pt-1 pb-1 sm:pb-4"
		>
			<div class="flex w-full flex-row items-center gap-1">
				<div class="font-semibold">Headings</div>
				<div class="text-neutral-500">Jetbrains Mono</div>
			</div>
			<H1 text="Heading 1" fontStyle="mono" />
			<H2 text="Heading 2" fontStyle="mono" />
			<H3 text="Heading 3" fontStyle="mono" />
			<H4 text="Heading 4" fontStyle="mono" />
			<H5 text="Heading 5" fontStyle="mono" />
			<H6 text="Heading 6" fontStyle="mono" />
		</div>
	</div>

	<div
		class="flex w-full flex-col items-start gap-4 border-t border-t-neutral-600 pt-1 pb-1 sm:pb-4"
	>
		<div class="flex w-full flex-row items-center gap-1">
			<div class="font-semibold">Body</div>
			<div class="text-neutral-500">Inter Sans Serif</div>
		</div>
		<P>
			One of the most famous lighthouses of antiquity, as I have already pointed out, was the pharos
			of Alexandria, which ancient writers included among the Seven Wonders of the World. It might
			naturally be supposed that the founder of so remarkable a monument of architectural skill
			would be well known; yet while Strabo and Pliny, Eusebius, Suidas, and Lucian ascribe its
			erection to Ptolemæus Philadelphus, the wisest and most benevolent of the Ptolemean kings of
			Egypt, by Tzetzes and Ammianus Marcellinus the honour is given to Cleopatra; and other
			authorities even attribute it to Alexander the Great.
		</P>
	</div>
</div>

<div class="mx-auto flex flex-col gap-2 p-4">
	<P>This is a paragraph element styled with the design system.</P>
	<Strong>This text is strong.</Strong>

	<Ol>
		<Li>Ordered List Item 1</Li>
		<Li>Ordered List Item 2</Li>
		<Li>Ordered List Item 3</Li>
	</Ol>

	<Ul>
		<Li>Unordered List Item A</Li>
		<Li>Unordered List Item B</Li>
		<Li>Unordered List Item C</Li>
	</Ul>
</div>
