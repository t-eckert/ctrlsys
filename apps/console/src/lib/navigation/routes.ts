/* Routes
 *
 * This file provides
 */

export type Route = {
	name: string
	path: string
}

const routes: Route[] = [
	{
		name: "Feedback",
		path: "/feedback"
	},
	{
		name: "About",
		path: "/about"
	},
	{
		name: "Validators",
		path: "/demo/validators"
	},
	{
		name: "Login",
		path: "/auth/login"
	},
	{
		name: "Admin",
		path: "/admin"
	}
]

export default routes
