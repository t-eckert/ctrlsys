#!/usr/bin/env node
/**
 * Script to create a new store in the SvelteKit stores directory.
 * Uses templates from templates/store/ to generate the store files.
 */

import { mkdir, readFile, writeFile } from "fs/promises"
import { existsSync } from "fs"
import { join, dirname } from "path"
import { fileURLToPath } from "url"

const __filename = fileURLToPath(import.meta.url)
const __dirname = dirname(__filename)

async function createStore(
	storeName: string,
	templateDir: string,
	storesDir: string
): Promise<void> {
	// Copy template files and replace placeholders
	const templateFiles = ["{name}.svelte.ts"]

	for (const templateFile of templateFiles) {
		const templatePath = join(templateDir, templateFile)
		if (existsSync(templatePath)) {
			const actualFileName = templateFile.replace("{name}", storeName)
			const targetPath = join(storesDir, actualFileName)

			// Read template content
			let content = await readFile(templatePath, "utf-8")

			// Replace placeholders
			content = content.replace(/{name}/g, storeName)

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
		console.log("Usage: node add_store.ts <name>")
		console.log("Example: node add_store.ts user")
		console.log("Example: node add_store.ts shoppingCart")
		process.exit(1)
	}

	const storeName = args[0]

	// Get script directory and project root
	const scriptDir = __dirname
	const projectRoot = dirname(scriptDir)

	// Define paths
	const templateDir = join(projectRoot, "templates", "store")
	const storesDir = join(projectRoot, "src", "lib", "stores")

	// Validate paths exist
	if (!existsSync(templateDir)) {
		console.error(`Error: Template directory not found: ${templateDir}`)
		process.exit(1)
	}

	if (!existsSync(storesDir)) {
		console.error(`Error: Stores directory not found: ${storesDir}`)
		process.exit(1)
	}

	try {
		await createStore(storeName, templateDir, storesDir)
		console.log(`\nStore created successfully at: src/lib/stores/${storeName}.svelte.ts`)
	} catch (error) {
		console.error(`Error creating store: ${error}`)
		process.exit(1)
	}
}

if (import.meta.url === `file://${process.argv[1]}`) {
	main()
}
