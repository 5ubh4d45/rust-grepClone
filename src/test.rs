use std::str;
use clap::error::ContextValue::String;
use crate::find_matches;

#[test]
fn find_matches_test() {
    let res = find_matches(("lorem ipsum\ndolor sit amet").to_string(), "lorem");
    assert_eq!(res[0], "lorem ipsum\n");
}