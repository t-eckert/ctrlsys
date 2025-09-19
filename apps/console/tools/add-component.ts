#!/usr/bin/env node
/**
 * Script to create a new component in the SvelteKit components directory.
 * Uses templates from templates/component/ to generate the component files.
 */

import { mkdir, readFile, writeFile } from "fs/promises"
import { existsSync } from "fs"
import { join, dirname } from "path"
import { fileURLToPath } from "url"

const __filename = fileURLToPath(import.meta.url)
const __dirname = dirname(__filename)

async function createComponent(
	componentName: string,
	templateDir: string,
	componentsDir: string
): Promise<void> {
	// Create the target directory
	const targetDir = join(componentsDir, componentName)
	await mkdir(targetDir, { recursive: true })

	// Copy template files and replace placeholders
	const templateFiles = ["{name}.svelte", "{name}.stories.svelte"]

	for (const templateFile of templateFiles) {
		const templatePath = join(templateDir, templateFile)
		if (existsSync(templatePath)) {
			const actualFileName = templateFile.replace("{name}", componentName)
			const targetPath = join(targetDir, actualFileName)

			// Read template content
			let content = await readFile(templatePath, "utf-8")

			// Replace placeholders
			content = content.replace(/{name}/g, componentName)

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
		console.log("Usage: node add_component.ts <name>")
		console.log("Example: node add_component.ts Button")
		console.log("Example: node add_component.ts UserProfile")
		process.exit(1)
	}

	const componentName = args[0]

	// Get script directory and project root
	const scriptDir = __dirname
	const projectRoot = dirname(scriptDir)

	// Define paths
	const templateDir = join(projectRoot, "templates", "component")
	const componentsDir = join(projectRoot, "src", "lib", "components")

	// Validate paths exist
	if (!existsSync(templateDir)) {
		console.error(`Error: Template directory not found: ${templateDir}`)
		process.exit(1)
	}

	if (!existsSync(componentsDir)) {
		console.error(`Error: Components directory not found: ${componentsDir}`)
		process.exit(1)
	}

	try {
		await createComponent(componentName, templateDir, componentsDir)
		console.log(`\nComponent created successfully at: src/lib/components/${componentName}`)
	} catch (error) {
		console.error(`Error creating component: ${error}`)
		process.exit(1)
	}
}

if (import.meta.url === `file://${process.argv[1]}`) {
	main()
}
