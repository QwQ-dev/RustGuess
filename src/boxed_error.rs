#[allow(dead_code)]
pub type BoxedError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[allow(dead_code)]
pub fn get_concrete_error<E>(boxed_error: &BoxedError) -> Option<&E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    boxed_error.downcast_ref::<E>()
}
