<script lang="ts">
	import { File } from "phosphor-svelte"
	import { cva } from "cva"
	import Button from "$lib/components/button/button.svelte"

	type Status = "ready" | "uploading" | "completed" | "failed"

	interface Props {
		acceptedFileTypes?: string[]
	}

	let { acceptedFileTypes }: Props = $props()

	let status: Status = $state("ready")
	let isDraggedOver: boolean = $state(false)
	let fileInput: HTMLInputElement = $state()

	function handleDragOver(event: DragEvent) {
		event.preventDefault()
		event.stopPropagation()
		isDraggedOver = true
	}

	function handleDragLeave(event: DragEvent) {
		event.preventDefault()
		event.stopPropagation()
		isDraggedOver = false
	}

	function handleDrop(event: DragEvent) {
		event.preventDefault()
		event.stopPropagation()
		isDraggedOver = false

		if (event.dataTransfer && event.dataTransfer.files.length > 0) {
			handleFileInput(event.dataTransfer.files)
		}
	}

	function handleFileInput(files: FileList | File[]) {
		if (files.length === 0) return

		// Process the files as needed
		console.log("Files selected:", files)
		status = "uploading"

		// Simulate file upload
		uploadFiles(files)
	}

	function uploadFiles(files: FileList | File[]) {
		if (files.length === 0) return

		// Process the files as needed
		console.log("Files selected:", files)
		status = "uploading"

		// Simulate file upload
		setTimeout(() => {
			status = "completed"
			console.log("Files uploaded successfully")
		}, 2000)
	}

	let containerStyle = cva([
		"group",
		"h-full",
		"w-full",
		"rounded",
		"border",
		"border-neutral-100",
		"bg-white/80",
		"p-1",
		"transition-all",
		"group-hover:shadow-md",
		"dark:border-neutral-700",
		"dark:bg-neutral-900",
		"dark:group-hover:shadow-neutral-800"
	])
	let backdropStyle = cva(
		[
			"flex",
			"h-full",
			"w-full",
			"items-center",
			"justify-center",
			"rounded-xs",
			"shadow-inner",
			"transition-all",
			"border",
			"border-neutral-100",
			"bg-neutral-50",
			"group-hover:bg-neutral-100",
			"dark:bg-neutral-950",
			"dark:border-neutral-800",
			"dark:group-hover:bg-neutral-900",
			"dark:group-hover:border-neutral-700"
		],
		{
			variants: {
				draggedOver: {
					true: [""],
					false: [""]
				}
			}
		}
	)
</script>

<div
	role="button"
	tabindex="0"
	class={containerStyle()}
	ondragover={handleDragOver}
	ondragexit={handleDragLeave}
	ondragleave={handleDragLeave}
	ondrop={handleDrop}
>
	<input bind:this={fileInput} type="file" class="hidden" multiple onchange={handleFileInput} />

	<div class={backdropStyle()}>
		<div class="flex flex-col items-center gap-1">
			<div class="mb-2 text-neutral-500 transition-transform group-hover:rotate-8">
				<File class="h-8 w-8" />
			</div>
			<div class="content text-sm font-medium text-neutral-600 select-none">
				<span>Drag and drop files here</span>
				<span>or</span>
			</div>
			<Button onclick={() => fileInput.click()}
				><span class="text-sm">Browse local files</span></Button
			>
		</div>
	</div>
</div>
