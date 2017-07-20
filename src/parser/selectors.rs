use nom::is_alphanumeric;
use parser::util::{opt_spacelike, spacelike2};
use parser::value::value_expression;
use sass::{InterpolationString, InterpolationStringPart, Selector,
           SelectorPart, Selectors};
use std::str::from_utf8;

named!(pub selectors<Selectors>,
       map!(separated_nonempty_list!(
           do_parse!(tag!(",") >> opt!(is_a!(", \t\n")) >> ()),
           selector),
            |s| Selectors(s)));

named!(pub complete_selectors<Selectors>,
        do_parse!(selectors: selectors >> tag!(";") >> (selectors)));

named!(pub selector<Selector>,
       map!(many1!(selector_part),
            |s: Vec<SelectorPart>| {
                let mut s = s;
                if s.last() == Some(&SelectorPart::Descendant) {
                    s.pop();
                }
                Selector(s)
            }));

named!(selector_part<&[u8], SelectorPart>,
       alt_complete!(
           map!(selector_string, |s| SelectorPart::Simple(s)) |
           value!(SelectorPart::Simple("*".into()), tag!("*")) |
           do_parse!(tag!("::") >>
                     name: selector_string >>
                     (SelectorPart::PseudoElement(name))) |
           do_parse!(tag!(":") >>
                     name: selector_string >>
                     arg: opt!(delimited!(tag!("("), selectors,
                                          tag!(")"))) >>
                     (SelectorPart::Pseudo {
                         name: name,
                         arg: arg,
                     })) |
           do_parse!(tag!("[") >> opt_spacelike >>
                     name: selector_string >> opt_spacelike >>
                     op: alt_complete!(tag!("*=") | tag!("|=") | tag!("=")) >>
                     opt_spacelike >>
                     val: alt_complete!(
                         map!(delimited!(tag!("\""),
                                         escaped!(is_not!("\\\""), '\\',
                                                  one_of!("\"\\")),
                                         tag!("\"")),
                              |s| format!("\"{}\"", from_utf8(s).unwrap())) |
                         map!(delimited!(tag!("'"),
                                         escaped!(is_not!("\\'"), '\\',
                                                  one_of!("'\\")),
                                         tag!("'")),
                              |s| format!("'{}'", from_utf8(s).unwrap()))) >>
                     opt_spacelike >>
                     tag!("]") >>
                     (SelectorPart::Attribute {
                         name: name,
                         op: from_utf8(op).unwrap().into(),
                         val: val,
                     })) |
           do_parse!(tag!("[") >> opt_spacelike >>
                     name: selector_string >> opt_spacelike >>
                     tag!("]") >>
                     (SelectorPart::Attribute {
                         name: name,
                         op: "".into(),
                         val: "".into(),
                     })) |
           value!(SelectorPart::BackRef, tag!("&")) |
           delimited!(opt_spacelike,
                      alt!(value!(SelectorPart::RelOp(b'>'), tag!(">")) |
                           value!(SelectorPart::RelOp(b'+'), tag!("+")) |
                           value!(SelectorPart::RelOp(b'~'), tag!("~")) |
                           value!(SelectorPart::RelOp(b'\\'), tag!("\\"))),
                      opt_spacelike) |
           value!(SelectorPart::Descendant, spacelike2)
           ));


named!(selector_string<InterpolationString>,
       fold_many1!(alt_complete!(
                       selector_interpolation_part |
                       selector_plain_part |
                       selector_escaped_part |
                       selector_hash_part),
                   InterpolationString(vec![]),
                   |mut acc: InterpolationString, item| {
                       acc.0.push(item);
                       acc
                   }));
named!(selector_plain_part<InterpolationStringPart>,
       map!(take_while1!(is_selector_char),
       |v| InterpolationStringPart::Simple(from_utf8(v).unwrap().into())));
named!(selector_escaped_part<InterpolationStringPart>,
       map!(recognize!(preceded!(tag!("\\"), many_m_n!(1, 3, hexpair))),
       |v| InterpolationStringPart::Simple(from_utf8(v).unwrap().into())));
named!(selector_interpolation_part<InterpolationStringPart>,
       map!(delimited!(tag!("#{"), value_expression, tag!("}")),
            |v| InterpolationStringPart::Value(v)));
named!(selector_hash_part<InterpolationStringPart>,
       map!(tag!("#"),
            |_| InterpolationStringPart::Simple("#".into())));

named!(hexpair,
       recognize!(do_parse!(one_of!("0123456789ABCDEFabcdef") >>
                            one_of!("0123456789ABCDEFabcdef") >> ())));

fn is_selector_char(chr: u8) -> bool {
    is_alphanumeric(chr) || chr == b'_' || chr == b'-' || chr == b'.'
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult::*;
    use sass::Value;

    #[test]
    fn simple_selector() {
        assert_eq!(selector(b"foo "),
                   Done(&b""[..],
                        Selector(vec![SelectorPart::Simple("foo".into())])))
    }
    #[test]
    fn escaped_simple_selector() {
        let string = InterpolationString(vec!["\\E9".into(), "m".into()]);
        let actual = Selector(vec![SelectorPart::Simple(string)]);
        assert_eq!(selector(b"\\E9m "), Done(&b""[..], actual))
    }

    #[test]
    fn simple_selector_with_interpolation() {
        let value = InterpolationStringPart::Value(Value::scalar(12));
        let string =
            InterpolationString(vec!["foo".into(), value, "bar".into()]);
        let actual = Selector(vec![SelectorPart::Simple(string)]);
        assert_eq!(selector(b"foo#{12}bar"), Done(&b""[..], actual))
    }

    #[test]
    fn selector2() {
        assert_eq!(selector(b"foo bar "),
                   Done(&b""[..],
                        Selector(vec![SelectorPart::Simple("foo".into()),
                                      SelectorPart::Descendant,
                                      SelectorPart::Simple("bar".into())])))
    }

    #[test]
    fn child_selector() {
        assert_eq!(selector(b"foo > bar "),
                   Done(&b""[..],
                        Selector(vec![SelectorPart::Simple("foo".into()),
                                      SelectorPart::RelOp(b'>'),
                                      SelectorPart::Simple("bar".into())])))
    }

    #[test]
    fn foo1_selector() {
        assert_eq!(selector(b"[data-icon='test-1'] "),
                   Done(&b""[..],
                        Selector(vec![SelectorPart::Attribute {
                            name: "data-icon".into(),
                            op: "=".into(),
                            val: "'test-1'".into(),
                        }])))
    }

    #[test]
    fn pseudo_selector() {
        assert_eq!(selector(b":before "),
                   Done(&b""[..],
                        Selector(vec![SelectorPart::Pseudo {
                                          name: "before".into(),
                                          arg: None,
                                      }])))
    }
    #[test]
    fn pseudo_on_simple_selector() {
        assert_eq!(selector(b"figure:before "),
                   Done(&b""[..],
                        Selector(vec![SelectorPart::Simple("figure".into()),
                                      SelectorPart::Pseudo {
                                          name: "before".into(),
                                          arg: None,
                                      }])))
    }
}
