//! Useful macros

#[cfg(feature = "std")]
pub use crate::hashmap;
pub use crate::unwrap;

#[cfg(feature = "std")]
/// [`vec!`][_std::vec!] for [`HashMap`][_std::collections::HashMap]
///
/// # Examples
///
/// ```
/// use std2::hashmap;
///
/// let id1 = "a";
/// let id2 = "b";
///
/// let mut users = hashmap! {
///   id1 => "User 1",
///   id2 => "User 2",
///   "c" => "User 3",
/// };
///
/// assert_eq!(users.get("a"), Some(&"User 1"));
/// assert_eq!(users.get("b"), Some(&"User 2"));
/// assert_eq!(users.get("c"), Some(&"User 3"));
/// assert_eq!(users.get("d"), None);
///
/// users.insert("d", "User 4");
///
/// assert_eq!(users.get("d"), Some(&"User 4"));
/// ```
#[macro_export]
macro_rules! hashmap {
  (
    $($k:expr => $v:expr),*
    $(,)?
  ) => {
    $crate::_std::collections::HashMap::from([
      $(($k, $v)),*
    ])
  };
}

/// <code>{[Option][Option::unwrap],[Result][Result::unwrap]}::unwrap()</code> while handling errors
///
/// This has an advantage over other methods such as
/// [`Option::unwrap_or_else()`][Option::unwrap_or_else] and
/// [`Result::unwrap_or_else()`][Result::unwrap_or_else] because it expands the
/// provided code into the same scope as the call, allowing you to do things like `return`, `break`,
/// or `continue` that would not be possible inside of a closure.
///
/// <div class="warning">
///
/// **While the syntax might look like a closure, it is NOT**. The reason this syntax was chosen was
/// so that `rustfmt` is able to format the macro body because it sees it as valid rust code (an
/// expression + a closure).
///
/// </div>
///
/// # Examples
///
/// ```
/// use std2::unwrap;
///
/// fn do_something(res: Result<u8, &'static str>) -> u8 {
///   let x = match res {
///     Ok(x) => x,
///     Err(err) => {
///       println!("error: {err}");
///       return 0;
///     }
///   };
///
///   x + 2
/// }
///
/// fn do_something2(res: Result<u8, &'static str>) -> u8 {
///   let x = unwrap!(res, |err| {
///     println!("error: {err}");
///     // this returns 0 from the function (`do_something2`)
///     // it does NOT assign 0 to `x`
///     return 0;
///   });
///
///   x + 2
/// }
///
/// assert_eq!(do_something(Ok(2)), 4);
/// assert_eq!(do_something2(Ok(2)), 4);
///
/// assert_eq!(do_something(Err("oops")), 0);
/// assert_eq!(do_something2(Err("oops")), 0);
/// ```
#[macro_export]
macro_rules! unwrap {
  ($expr:expr, |$binding:pat_param| $action:expr $(,)?) => {
    match $crate::macros::IntoResult::into_result($expr) {
      ::core::result::Result::Ok(x) => x,
      ::core::result::Result::Err($binding) => $action,
    }
  };

  ($expr:expr, $action:expr $(,)?) => {
    $crate::unwrap!($expr, |_| $action)
  };
}

// <@credit(2026-2-18)>
// SabrinaJewson (sabrinajewson) <970372552432177163>
// Rust Programming Language Community <273534239310479360>
// #rust-discussions-1 <273541522815713281>
// https://discord.com/channels/273534239310479360/273541522815713281/1473790310156075068
#[doc(hidden)]
pub trait IntoResult {
  type Output;
  type Error;

  fn into_result(self) -> Result<Self::Output, Self::Error>;
}

impl<T, E> IntoResult for Result<T, E> {
  type Output = T;
  type Error = E;

  fn into_result(self) -> Result<Self::Output, Self::Error> {
    self
  }
}

impl<T> IntoResult for Option<T> {
  type Output = T;
  type Error = ();

  fn into_result(self) -> Result<Self::Output, Self::Error> {
    match self {
      Option::Some(x) => Result::Ok(x),
      Option::None => Result::Err(()),
    }
  }
}
// </@credit>

#[cfg(test)]
mod tests {
  #[cfg(feature = "std")]
  #[test]
  fn hashmap() {
    let h = hashmap! {
      0 => 10,
      23 => 20,
    };

    assert_eq!(h.get(&0), Some(&10));
    for i in 1..23 {
      assert_eq!(h.get(&i), None);
    }
    assert_eq!(h.get(&23), Some(&20));
  }

  // @todo unwrap
}
