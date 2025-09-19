const wordsPerMinute = 200

// This function calculates the reading time of a given text in minutes.
// It takes the text as input and returns the estimated reading time based on a default reading speed of 200 words per minute.
export default function readingTime(text: string, wpm: number = wordsPerMinute): number {
	return Math.ceil(text.split(" ").length / wpm)
}
