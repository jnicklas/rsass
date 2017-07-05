extern crate rsass;
use rsass::*;
use std::sync::Arc;

fn compile_with_scope(scope: &mut Scope,
                      input: &[u8])
                      -> Result<String, Error> {
    let sass_items = parse_scss_data(input)?;

    let file_context = FileContext::new();
    let css_items =
        compiler::compile_in_scope(&file_context, scope, &sass_items)?;

    let mut buffer = Vec::new();
    writer::write(&mut buffer, OutputStyle::Compressed, &css_items)?;

    Ok(String::from_utf8(buffer)?)
}

#[test]
fn simple_value() {
    let mut scope = GlobalScope::new();
    scope.define("color", &css::Value::black());

    let output = compile_with_scope(&mut scope, b"p { color: $color }")
        .unwrap();

    assert_eq!(output, "p{color:black}\n");
}

#[test]
fn simple_function() {
    let mut scope = GlobalScope::new();
    scope.define_function("get_answer",
                          SassFunction::builtin(vec![],
                                                false,
                                                Arc::new(|_| {
        Ok(css::Value::scalar(42))
    })));
    let output = compile_with_scope(&mut scope, b"p { x: get_answer(); }")
        .unwrap();

    assert_eq!(output, "p{x:42}\n");
}

#[test]
fn function_with_args() {
    let mut scope = GlobalScope::new();
    scope.define_function("halfway",
                          SassFunction::builtin(vec![("a".into(),
                                                     sass::Value::Null),
                                                    ("b".into(),
                                                     sass::Value::scalar(0))],
                                                false,
                                                Arc::new(|s| {
        let half = Rational::new(1, 2);
        match (s.get("a"), s.get("b")) {
            (css::Value::Numeric(a, au, ..),
             css::Value::Numeric(b, bu, ..)) => {
                if au == bu || bu == Unit::None {
                    Ok(css::Value::Numeric((a + b) * half, au, false, true))
                } else if au == Unit::None {
                    Ok(css::Value::Numeric((a + b) * half, bu, false, true))
                } else {
                    Err(Error::BadArguments("Incopatible args".into()))
                }
            }
            (a, b) => Err(Error::badargs(&["number", "number"], &[&a, &b])),
        }
    })));
    let output = compile_with_scope(&mut scope, b"p { x: halfway(10, 18); }")
        .unwrap();
    assert_eq!(output, "p{x:14}\n");
}
