extern crate js_sys;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn random_index(size: usize, without: usize) -> usize {
  let result = js_sys::Math::random() * size as f64;
  let idx = result.floor() as usize;
  if idx == without {
    random_index(size, without)
  } else {
    idx
  }
}
