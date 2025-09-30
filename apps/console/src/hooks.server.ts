import type { Handle } from "@sveltejs/kit"
import handleTheme from "$lib/theme/theme-handler"

export const handle: Handle = handleTheme
