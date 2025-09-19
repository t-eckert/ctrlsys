// Slugify a string by replacing spaces with hyphens, converting to lowercase, and removing apostrophes.
export default function slug(text: string): string {
	return text.replaceAll(" ", "-").toLowerCase().replaceAll("'", "")
}
