#[alchem_macros::native_fn]
fn wrapped_function() {}

#[test]
fn works() {
  wrapped_function();
}
