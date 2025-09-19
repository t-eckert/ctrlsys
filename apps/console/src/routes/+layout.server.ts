import type { LayoutServerLoad } from "./$types"

export const load: LayoutServerLoad = async ({ locals }) => {
	return {
		root: {
			theme: locals.theme
		}
	}
}
