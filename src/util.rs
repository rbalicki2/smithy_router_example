use web_sys::{
  window,
  Location,
};

pub fn get_location() -> Location {
  window().unwrap().location()
}

pub fn get_current_hash() -> Option<String> {
  get_location()
    .hash()
    .ok()
    .map(|hash_with_hash| hash_with_hash.chars().skip(1).collect::<String>())
}
