#!/usr/bin/env node
/**
 * Script to create a new page in the SvelteKit routes directory.
 * Uses templates from templates/page/ to generate the page files.
 */

import { mkdir, readFile, writeFile } from "fs/promises"
import { existsSync } from "fs"
import { join, dirname } from "path"
import { fileURLToPath } from "url"

const __filename = fileURLToPath(import.meta.url)
const __dirname = dirname(__filename)

async function createPage(pagePath: string, templateDir: string, routesDir: string): Promise<void> {
	// Normalize the page path (remove leading/trailing slashes)
	pagePath = pagePath.replace(/^\/+|\/+$/g, "")

	// Create the target directory
	const targetDir = join(routesDir, pagePath)
	await mkdir(targetDir, { recursive: true })

	// Copy template files
	const templateFiles = ["+layout.svelte", "+page.server.ts", "+page.svelte"]

	for (const templateFile of templateFiles) {
		const templatePath = join(templateDir, templateFile)
		if (existsSync(templatePath)) {
			const targetPath = join(targetDir, templateFile)

			// Read template content
			const content = await readFile(templatePath, "utf-8")

			// Write to target
			await writeFile(targetPath, content)

			console.log(`Created: ${targetPath}`)
		} else {
			console.log(`Warning: Template ${templateFile} not found`)
		}
	}
}

async function main() {
	const args = process.argv.slice(2)
	if (args.length === 0) {
		console.log("Usage: node add_page.ts <path>")
		console.log("Example: node add_page.ts about")
		console.log("Example: node add_page.ts blog/post")
		process.exit(1)
	}

	const pagePath = args[0]

	// Get script directory and project root
	const scriptDir = __dirname
	const projectRoot = dirname(scriptDir)

	// Define paths
	const templateDir = join(projectRoot, "templates", "page")
	const routesDir = join(projectRoot, "src", "routes")

	// Validate paths exist
	if (!existsSync(templateDir)) {
		console.error(`Error: Template directory not found: ${templateDir}`)
		process.exit(1)
	}

	if (!existsSync(routesDir)) {
		console.error(`Error: Routes directory not found: ${routesDir}`)
		process.exit(1)
	}

	try {
		await createPage(pagePath, templateDir, routesDir)
		console.log(`\nPage created successfully at: src/routes/${pagePath}`)
	} catch (error) {
		console.error(`Error creating page: ${error}`)
		process.exit(1)
	}
}

if (import.meta.url === `file://${process.argv[1]}`) {
	main()
}
