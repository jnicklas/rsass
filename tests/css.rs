//! These are from the "css" directory in the sass specification.
//! See https://github.com/sass/sass-spec for source material.
//! I add one a test function for one specification at a time and then
//! try to implement that functionality without breaking those already
//! added.
extern crate rsass;
use rsass::{OutputStyle, compile_scss};

#[test]
fn multi_star_comments() {
    // Note, actual expected has single newlines, but the sass-spec
    // test runner succeeds my implementation.
    check(b"a /***/ b {x: y}\n\
            a /****/ b {x: y}\n\
            a /* **/ b {x: y}\n\
            a /** */ b {x: y}\n",
          "a b {\n  x: y;\n}\n\n\
           a b {\n  x: y;\n}\n\n\
           a b {\n  x: y;\n}\n\n\
           a b {\n  x: y;\n}\n")
}

fn check(input: &[u8], expected: &str) {
    let mut buffer = Vec::new();
    compile_scss(input, &mut buffer, OutputStyle::Normal).unwrap();
    let actual = String::from_utf8(buffer).unwrap();
    assert_eq!(&actual, expected);
}
