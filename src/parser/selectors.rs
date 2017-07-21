use nom::is_alphanumeric;
use parser::util::{opt_spacelike, spacelike2};
use sass::{Selector, SelectorPart, Selectors};
use std::str::from_utf8;

named!(pub selectors<Selectors>,
       map!(separated_nonempty_list!(
           do_parse!(tag!(",") >> opt!(is_a!(", \t\n")) >> ()),
           selector),
            |s| Selectors(s)));

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
           value!(SelectorPart::Simple("*".to_string()), tag!("*")) |
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
                         op: "".to_string(),
                         val: "".to_string(),
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


named!(selector_string<String>,
       fold_many1!(alt_complete!(selector_plain_part | selector_escaped_part),
                   String::new(),
                   |mut acc: String, item: &[u8]| {
                       acc.push_str(from_utf8(item).unwrap());
                       acc
                   }));
named!(selector_plain_part<&[u8]>,
       take_while1!(is_selector_char));
named!(selector_escaped_part<&[u8]>,
       recognize!(preceded!(tag!("\\"), many_m_n!(1, 3, hexpair))));
named!(hexpair,
       recognize!(do_parse!(one_of!("0123456789ABCDEFabcdef") >>
                            one_of!("0123456789ABCDEFabcdef") >> ())));

fn is_selector_char(chr: u8) -> bool {
    is_alphanumeric(chr) || chr == b'_' || chr == b'-' || chr == b'.' ||
    chr == b'#'
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult::*;

    #[test]
    fn simple_selector() {
        assert_eq!(selector(b"foo "),
                   Done(&b""[..],
                        Selector(vec![SelectorPart::Simple("foo".into())])))
    }
    #[test]
    fn escaped_simple_selector() {
        assert_eq!(selector(b"\\E9m "),
                   Done(&b""[..],
                        Selector(vec![SelectorPart::Simple("\\E9m".into())])))
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
