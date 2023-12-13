/// ```compile_fail
/// fn check_seal_trait() {
///     use step_2_6::my_iterator_ext::private::MyIteratorExt;
/// }
/// ```
pub mod my_iterator_ext;

/// ```compile_fail
/// fn check_seal_method(err: impl step_2_6::my_error::MyError) {
///     let _ = err.type_id(step_2_6::my_error::private::Internal);
/// }
/// ```
pub mod my_error;

pub use self::my_error::MyError;
