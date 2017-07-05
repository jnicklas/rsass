extern crate rsass;
use rsass::{OutputStyle, compile_scss};

#[test]
fn t01_arg_eval() {
    check(b"
        @function foo() {
          @return 1+2 3/4 5+6;
        }

        @mixin bar($x: 3/4) {
          bar-content: $x;
        }

        div {
          content: foobar(1+2 3/4 5+6, orange);
          content: append(1+2 2/3 5+6, orange);
          content: 1+2 2/3 5+6;
          content: type-of(2/3);
          content: type-of(orange);
          content: foo();
          @include bar();
        }
    ",
          "div {\n  \
      content: foobar(3 3/4 11, orange);\n  \
      content: 3 2/3 11 orange;\n  content: 3 2/3 11;\n  \
      content: number;\n  content: color;\n  content: 3 3/4 11;\n  \
      bar-content: 0.75;\n}\n")
}

fn check(input: &[u8], expected: &str) {
    let mut buffer = Vec::new();
    compile_scss(input, &mut buffer, OutputStyle::Expanded(0)).unwrap();
    let actual = String::from_utf8(buffer).unwrap();
    assert_eq!(&actual, expected);
}
