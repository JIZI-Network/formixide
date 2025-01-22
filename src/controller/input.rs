pub trait InputSource {
    fn get_value(&self, key: &str) -> Option<Vec<u8>>;
}