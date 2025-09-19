export default interface Toast {
	id: number
	role: "info" | "error" | "success"
	title: string
	message: string
}
