//! Useful macros

#[cfg(feature = "std")]
pub use crate::hashmap;

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
}
