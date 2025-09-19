import { getContext, setContext } from "svelte"

type Status = "ready" | "uploading" | "completed" | "failed"

class FiledropStore {
	#status: Status = $state("ready")
	isDraggedOver: boolean = $state(false)

	// Get the readonly status of the filedrop store.
	status() {
		return this.#status
	}


	// Handle the user dragging a file over the filedrop component.
	handleDragOver(e: DragEvent) {
		e.preventDefault()
		e.stopPropagation()
		this.isDraggedOver = true
	}

	// Handle the user leaving the filedrop component while dragging.
	handleDragLeave(e: DragEvent) {
		e.preventDefault()
		e.stopPropagation()

		const target = e.currentTarget as HTMLElement;
		const relatedTarget = e.relatedTarget as HTMLElement;
		if (!target.contains(relatedTarget)) {
			this.isDraggedOver = false
		}
	}

	// Handle the user dropping a file on the filedrop component.
	// This reads the file and begins the upload.
	handleDrop(e: DragEvent) {
		e.preventDefault();
		e.stopPropagation();
		this.isDraggedOver = false

		if (!e.dataTransfer) return;

		const files = Array.from(e.dataTransfer.files);
		if (files.length > 0) {
			this.#uploadFiles(files);
		}
	}

	handleFileInput(e: Event) {
		const target = e.target as HTMLInputElement;
		if (!target.files) return;

		const files = Array.from(target.files);
		if (files.length > 0) {
			this.#uploadFiles(files);
		}
		// Reset the input so the same file can be selected again
		target.value = '';
	}

	#uploadFiles(files: File[]) {
		this.#status = "uploading"

		console.log("Uploading files:", files)

		setTimeout(() => {
			this.#status = "completed"
		}, 2000)
	}
}

export function initFiledropStore(key: string) {
	return setContext(key, new FiledropStore())
}

export function getFiledropStore(key: string) {
	return getContext<ReturnType<typeof initFiledropStore>>(key)
}
