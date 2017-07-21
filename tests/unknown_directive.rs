extern crate rsass;

use rsass::{OutputStyle, compile_scss};

/// Tests from `sass_spec/spec/css/unknown_directive`

// Unknown directives should support almost any sequence of valid tokens,
// including interpolation.

fn check(input: &str, expected: &str) {
    let mut buffer = Vec::new();
    compile_scss(input.as_bytes(), &mut buffer, OutputStyle::Expanded(0))
        .unwrap();
    let actual = String::from_utf8(buffer).unwrap();
    assert_eq!(&actual, expected);
}

#[test]
fn t01_characters_are_passed_through_unaltered() {
    check("@asdf .~@#$%^&*()_-+=[]|:<>,.?/;\n",
          "@asdf .~@#$%^&*()_-+=[]|:<>,.?/;\n")
}
#[test]
fn t02_strings_are_tokenized_as_strings() {
    check("@asdf \"f'o\" 'b\"r' url(baz) url(\"qux\");\n",
          "@asdf \"f'o\" 'b\"r' url(baz) url(\"qux\");\n")
}
#[test]
fn t03_comments_are_preserved() {
    check("@asdf foo //\n      bar;\n", "@asdf foo //\n      bar;\n")
}
#[test]
fn t04_comments_are_preserved() {
    check("@asdf foo /* bar */ baz;", "@asdf foo /* bar */ baz;\n")
}
#[test]
fn t05_interpolation_plain() {
    check("@asdf #{1 + 2};\n", "@asdf 3;\n")
}
#[test]
fn t06_interpolation_in_string() {
    check("@asdf \"foo #{\"bar\"} baz\";\n", "@asdf \"foo bar baz\";\n")
}
#[test]
#[ignore] // TODO The single quotes should not be converted to double.
fn t07_interpolation_in_string() {
    check("@asdf 'foo #{'bar'} baz';\n", "@asdf 'foo bar baz';\n")
}
#[test]
fn t08_interpolation_in_url() {
    check("@asdf url(http://#{\")\"}.com/);\n",
          "@asdf url(http://).com/);\n")
}
#[test]
fn t09_interpolation_in_url() {
    check("@asdf url(\"http://#{\")\"}.com/\");\n",
          "@asdf url(\"http://).com/\");\n")
}
