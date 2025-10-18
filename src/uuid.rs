use uuid::Uuid;

pub fn create_uuid() -> Uuid {
    Uuid::new_v4()
}
