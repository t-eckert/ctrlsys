use rand::Rng;

pub fn name() -> String {
    let mut rng = rand::rng();

    let adjective = ADJECTIVES[rng.random_range(0..ADJECTIVES.len())];
    let noun = NOUNS[rng.random_range(0..NOUNS.len())];

    format!("{}-{}", adjective, noun)
}

const ADJECTIVES: [&str; 50] = [
    "agile",
    "brave",
    "calm",
    "delightful",
    "eager",
    "fierce",
    "gentle",
    "happy",
    "inventive",
    "jolly",
    "keen",
    "lively",
    "mighty",
    "noble",
    "optimistic",
    "peaceful",
    "quick",
    "radiant",
    "sincere",
    "thoughtful",
    "upbeat",
    "vibrant",
    "witty",
    "youthful",
    "zealous",
    "bold",
    "curious",
    "daring",
    "elegant",
    "fearless",
    "graceful",
    "humble",
    "inquisitive",
    "joyful",
    "kind",
    "loyal",
    "magnificent",
    "nurturing",
    "observant",
    "playful",
    "quiet",
    "resilient",
    "spirited",
    "tender",
    "understanding",
    "valiant",
    "warm",
    "xenial",
    "yearning",
    "zesty",
];

const NOUNS: [&str; 50] = [
    "adventure",
    "beacon",
    "compass",
    "dream",
    "echo",
    "flame",
    "glimmer",
    "harbor",
    "inspiration",
    "journey",
    "keeper",
    "legacy",
    "memory",
    "nexus",
    "oasis",
    "path",
    "quest",
    "realm",
    "spirit",
    "treasure",
    "universe",
    "voyage",
    "whisper",
    "yonder",
    "zephyr",
    "alliance",
    "bridge",
    "charm",
    "discovery",
    "ember",
    "fortress",
    "guardian",
    "horizon",
    "insight",
    "jewel",
    "key",
    "labyrinth",
    "melody",
    "network",
    "origin",
    "pillar",
    "radiance",
    "sanctuary",
    "talisman",
    "unity",
    "venture",
    "wonder",
    "xenon",
    "yearning",
    "zenith",
];

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_name() {
        let name = name();
        let (adjective, noun) = name.split_once("-").unwrap();

        assert!(ADJECTIVES.contains(&adjective));
        assert!(NOUNS.contains(&noun));
    }
}
