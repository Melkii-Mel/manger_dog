pub trait Entity<E> {
    fn table_name() -> &'static str;
    fn api_location() -> &'static str;
    fn validate(&self) -> Result<(), E>;
}