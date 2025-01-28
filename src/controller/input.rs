use uuid::Uuid;

pub trait InputSource {
    fn get_value(&self, key: &Uuid) -> Option<Vec<u8>>;
}
