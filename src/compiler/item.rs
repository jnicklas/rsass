use css;
use error::Error;
use file_context::FileContext;
use parser::parse_scss_file;
use sass;
use selectors::Selectors;
use value::Quotes;
use variablescope::{Scope, ScopeImpl};

pub fn compile_root_items(file_context: &FileContext,
                          scope: &mut Scope,
                          sass_items: &[sass::Item])
                          -> Result<Vec<css::Item>, Error> {
    let mut css_items = Vec::with_capacity(sass_items.len());

    for sass_item in sass_items {
        css_items.extend(compile_root_item(file_context, scope, sass_item)?
                             .into_iter());
    }

    css_items.sort();

    Ok(css_items)
}

pub fn compile_root_item(file_context: &FileContext,
                         scope: &mut Scope,
                         item: &sass::Item)
                         -> Result<Vec<css::Item>, Error> {
    match *item {
        sass::Item::Import(ref name) => {
            let name = name.evaluate(scope);
            if let css::Value::Literal(ref x, _) = name {
                if let Some((sub_context, file)) =
                    file_context.find_file(x.as_ref()) {
                    let sass_items = parse_scss_file(&file)?;
                    compile_root_items(&sub_context, scope, &sass_items)
                } else {
                    let value = css::Value::Literal(x.clone(), Quotes::None);
                    let call_args = css::CallArgs::new(vec![(None, value)]);
                    let call = css::Value::Call(String::from("url"), call_args);
                    Ok(vec![css::Item::Import(call)])
                }
            } else {
                Ok(vec![css::Item::Import(name)])
            }
        }
        sass::Item::VariableDeclaration {
            ref name,
            ref val,
            ref default,
            ref global,
        } => {
            let val = val.evaluate(scope);
            if *default {
                scope.define_default(name, &val, *global);
            } else if *global {
                scope.define_global(name, &val);
            } else {
                scope.define(name, &val);
            }
            Ok(vec![])
        }
        sass::Item::AtRule { ref name, ref args, ref body } => {
            let args = args.evaluate(scope);

            let body = match body {
                &Some(ref body) => {
                    Some(compile_body_items(&file_context,
                                            &mut ScopeImpl::sub(scope),
                                            &Selectors::root(),
                                            &body)?)
                }
                &None => None,
            };

            let at_rule = css::AtRule::new(name.clone(), args, body);
            Ok(vec![css::Item::AtRule(at_rule)])
        }
        sass::Item::MixinDeclaration { ref name, ref args, ref body } => {
            scope.define_mixin(name, args, body);
            Ok(vec![])
        }
        sass::Item::MixinCall { ref name, ref args, ref body } => {
            if let Some((m_args, m_body)) = scope.get_mixin(name) {
                let mut scope = m_args.eval(scope, &args.evaluate(scope));
                scope.define_mixin("%%BODY%%",
                                   &sass::FormalArgs::default(),
                                   body);
                compile_root_items(file_context, &mut scope, &m_body)
            } else {
                panic!(format!("Unknown mixin {}({:?})", name, args))
            }
        }
        sass::Item::Content => {
            panic!("@content not allowed in global context");
        }
        sass::Item::FunctionDeclaration { ref name, ref func } => {
            scope.define_function(name, func.clone());
            Ok(vec![])
        }
        sass::Item::Return(_) => {
            panic!("Return not allowed in global context");
        }
        sass::Item::IfStatement(ref cond, ref do_if, ref do_else) => {
            let cond = cond.evaluate(scope).is_true();
            let items = if cond { do_if } else { do_else };
            compile_root_items(file_context, scope, items)
        }
        sass::Item::Each(ref name, ref values, ref body) => {
            let values = match values.evaluate(scope) {
                css::Value::List(v, _) => v,
                v => vec![v],
            };

            let mut css_items = Vec::with_capacity(values.len());

            for value in values {
                scope.define(name, &value);
                css_items.extend(compile_root_items(file_context,
                                                    scope,
                                                    body)?);
            }

            Ok(css_items)
        }
        sass::Item::For { ref name, ref from, ref to, inclusive, ref body } => {
            let from = from.evaluate(scope).integer_value()?;
            let to = to.evaluate(scope).integer_value()?;
            let to = if inclusive { to + 1 } else { to };

            let range = from..to;

            let mut css_items = Vec::with_capacity(range.len());

            for value in range {
                scope.define(name, &css::Value::scalar(value));
                css_items.extend(compile_root_items(file_context,
                                                    scope,
                                                    body)?);
            }

            Ok(css_items)
        }
        sass::Item::While(ref cond, ref body) => {
            let mut css_items = vec![];

            while cond.evaluate(scope).is_true() {
                css_items.extend(compile_root_items(file_context,
                                                    scope,
                                                    body)?);
            }

            Ok(css_items)
        }
        sass::Item::Rule(ref s, ref body) => {
            let root = Selectors::root();
            let selectors = s.inside(Some(&root));
            let mut scope = ScopeImpl::sub(scope);
            let css_items =
                compile_body_items(file_context, &mut scope, &selectors, body)?;
            if css_items.len() > 0 {
                Ok(vec![css::Item::Rule(css::Rule::new(selectors, css_items))])
            } else {
                Ok(vec![])
            }
        }
        sass::Item::NamespaceRule(..) => {
            panic!("Global namespaced property not allowed");
        }
        sass::Item::Property(..) => {
            panic!("Global property not allowed");
        }
        sass::Item::Comment(ref c) => Ok(vec![css::Item::Comment(c.clone())]),
        sass::Item::None => Ok(vec![]),
    }
}

pub fn compile_body_items(file_context: &FileContext,
                          scope: &mut Scope,
                          selectors: &Selectors,
                          sass_items: &[sass::Item])
                          -> Result<Vec<css::Item>, Error> {
    let mut css_items = Vec::with_capacity(sass_items.len());

    for sass_item in sass_items {
        css_items.extend(compile_body_item(file_context,
                                           scope,
                                           selectors,
                                           sass_item)?
                                 .into_iter());
    }

    css_items.sort();

    Ok(css_items)
}

pub fn compile_body_item(file_context: &FileContext,
                         scope: &mut Scope,
                         selectors: &Selectors,
                         item: &sass::Item)
                         -> Result<Vec<css::Item>, Error> {
    match *item {
        sass::Item::Import(ref name) => {
            let name = name.evaluate(scope);

            if let css::Value::Literal(ref x, _) = name {
                let (sub_context, file) = file_context.file(x.as_ref());
                let sass_items = parse_scss_file(&file)?;
                compile_body_items(&sub_context, scope, selectors, &sass_items)
            } else {
                // TODO writeln!(direct, "@import {};", name)?;
                Ok(vec![])
            }
        }
        sass::Item::VariableDeclaration {
            ref name,
            ref val,
            default,
            global,
        } => {
            let val = val.evaluate(scope);
            if default {
                scope.define_default(name, &val, global);
            } else if global {
                scope.define_global(name, &val);
            } else {
                scope.define(name, &val);
            }
            Ok(vec![])
        }
        sass::Item::AtRule { ref name, ref args, ref body } => {
            let args = args.evaluate(scope);

            let body = match body {
                &Some(ref body) => {
                    Some(compile_body_items(&file_context,
                                            &mut ScopeImpl::sub(scope),
                                            &selectors,
                                            &body)?)
                }
                &None => None,
            };

            let at_rule = css::AtRule::new(name.clone(), args, body);
            Ok(vec![css::Item::AtRule(at_rule)])
        }
        sass::Item::MixinDeclaration { ref name, ref args, ref body } => {
            scope.define_mixin(name, args, body);
            Ok(vec![])
        }
        sass::Item::MixinCall { ref name, ref args, ref body } => {
            if let Some((m_args, m_body)) = scope.get_mixin(name) {
                let mut argscope = m_args.eval(scope, &args.evaluate(scope));
                argscope.define_mixin("%%BODY%%",
                                      &sass::FormalArgs::default(),
                                      body);
                compile_body_items(file_context,
                                   &mut argscope,
                                   selectors,
                                   &m_body)
            } else {
                Ok(vec![css::Item::Comment(format!("Unknown mixin {}({:?})",
                                                   name,
                                                   args))])
            }
        }
        sass::Item::Content => {
            if let Some((_args, m_body)) = scope.get_mixin("%%BODY%%") {
                compile_body_items(file_context, scope, selectors, &m_body)
            } else {
                Ok(vec![css::Item::Comment("Mixin @content not found."
                                               .to_string())])
            }
        }
        sass::Item::FunctionDeclaration { ref name, ref func } => {
            scope.define_function(name, func.clone());
            Ok(vec![])
        }
        sass::Item::Return(_) => {
            panic!("Return not allowed in plain context");
        }
        sass::Item::IfStatement(ref cond, ref do_if, ref do_else) => {
            let cond = cond.evaluate(scope).is_true();
            let items = if cond { do_if } else { do_else };
            compile_body_items(file_context, scope, selectors, items)
        }
        sass::Item::Each(ref name, ref values, ref body) => {
            let values = match values.evaluate(scope) {
                css::Value::List(v, _) => v,
                v => vec![v],
            };

            let mut css_items = Vec::with_capacity(values.len());

            for value in values {
                scope.define(name, &value);
                css_items.extend(compile_body_items(file_context,
                                                    scope,
                                                    selectors,
                                                    body)?);
            }

            Ok(css_items)
        }
        sass::Item::For { ref name, ref from, ref to, inclusive, ref body } => {
            let from = from.evaluate(scope).integer_value()?;
            let to = to.evaluate(scope).integer_value()?;
            let to = if inclusive { to + 1 } else { to };

            let range = from..to;

            let mut css_items = Vec::with_capacity(range.len());

            for value in range {
                scope.define(name, &css::Value::scalar(value));
                css_items.extend(compile_body_items(file_context,
                                                    scope,
                                                    selectors,
                                                    body)?);
            }

            Ok(css_items)
        }
        sass::Item::While(ref cond, ref body) => {
            let mut css_items = vec![];

            while cond.evaluate(scope).is_true() {
                css_items.extend(compile_body_items(file_context,
                                                    scope,
                                                    selectors,
                                                    body)?);
            }

            Ok(css_items)
        }
        sass::Item::Rule(ref s, ref body) => {
            let selectors = s.inside(Some(selectors));
            let mut scope = ScopeImpl::sub(scope);

            let css_items =
                compile_body_items(file_context, &mut scope, &selectors, body)?;
            if css_items.len() > 0 {
                Ok(vec![css::Item::Rule(css::Rule::new(selectors, css_items))])
            } else {
                Ok(vec![])
            }
        }
        sass::Item::NamespaceRule(ref name, ref value, ref body) => {
            let mut css_items = Vec::with_capacity(body.len() + 1);

            let value = value.evaluate(scope);

            if !value.is_null() {
                css_items.push(css::Item::Property(name.clone(), value, false));
            }

            let body_items =
                compile_body_items(file_context, scope, selectors, body)?;
            for item in body_items {
                let prefixed_item = match item {
                    css::Item::Property(n, v, i) => {
                        css::Item::Property(format!("{}-{}", name, n), v, i)
                    }
                    c => c,
                };
                css_items.push(prefixed_item);
            }

            Ok(css_items)
        }
        sass::Item::Property(ref name, ref value, important) => {
            let v = value.evaluate(scope);
            if !v.is_null() {
                Ok(vec![css::Item::Property(name.clone(), v, important)])
            } else {
                Ok(vec![])
            }
        }
        sass::Item::Comment(ref c) => Ok(vec![css::Item::Comment(c.clone())]),
        sass::Item::None => Ok(vec![]),
    }
}
