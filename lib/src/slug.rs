use slugify::slugify;

pub fn slug(name: &str) -> String {
    slugify!(name)
}
