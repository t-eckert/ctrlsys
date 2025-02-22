use uuid::Uuid;

pub struct Location {
    pub id: Uuid,
    pub name: String,
    pub longitude: f64,
    pub latitude: f64,
}

impl Location {
    pub fn new(name: &str, longitude: f64, latitude: f64) -> Location {
        Location {
            id: Uuid::new_v4(),
            name: name.to_string(),
            longitude,
            latitude,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_location() {
        let location = Location::new("Test", 12.0, 4.0);
        assert_eq!(location.name, "Test");
        assert_eq!(location.longitude, 12.0);
        assert_eq!(location.latitude, 4.0);
    }
}
