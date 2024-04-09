
pub fn get_type<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
