/* Routes
 *
 * Application navigation routes for the Console
 */

import type { Component } from "svelte"
import { House, Clock, Briefcase, Heart } from "phosphor-svelte"

export type Route = {
	name: string
	path: string
	icon: Component
}

const routes: Route[] = [
	{
		name: "Home",
		path: "/",
		icon: House
	},
	{
		name: "Jobs",
		path: "/jobs",
		icon: Briefcase
	},
	{
		name: "Timers",
		path: "/timers",
		icon: Clock
	},
	{
		name: "Health",
		path: "/health",
		icon: Heart
	}
]

export default routes
