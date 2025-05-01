use handlebars::{Handlebars, HelperDef, RenderError, ScopedJson};

/// Custom implementation of `#eq` that treats missing values as false-y
#[derive(Debug, Clone)]
pub struct CustomEqHelper;

impl HelperDef for CustomEqHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &handlebars::Helper<'reg, 'rc>,
        _r: &'reg Handlebars<'reg>,
        _ctx: &'rc handlebars::Context,
        _rc: &mut handlebars::RenderContext<'reg, 'rc>,
    ) -> std::result::Result<ScopedJson<'reg, 'rc>, handlebars::RenderError> {
        let params = h.params();
        if params.len() < 2 {
            return Err(RenderError::new(
                "Arity Error: `eq` helper requires at least two parameters",
            ));
        }
        let result = params.windows(2).all(|w| w[0].value() == w[1].value());

        Ok(ScopedJson::Derived(result.into()))
    }
}

/// Custom implementation of `#and` that treats missing values as false-y
#[derive(Debug, Clone)]
pub struct CustomAndHelper;

impl HelperDef for CustomAndHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &handlebars::Helper<'reg, 'rc>,
        _r: &'reg Handlebars<'reg>,
        _ctx: &'rc handlebars::Context,
        _rc: &mut handlebars::RenderContext<'reg, 'rc>,
    ) -> std::result::Result<ScopedJson<'reg, 'rc>, handlebars::RenderError> {
        let params = h.params();
        if params.len() < 2 {
            return Err(RenderError::new(
                "Arity Error: `and` helper requires at least two parameters",
            ));
        }

        let include_zero = h
            .hash_get("includeZero")
            .and_then(|v| v.value().as_bool())
            .unwrap_or(false);

        let result = params
            .iter()
            .all(|param| param.value().is_truthy(include_zero));

        Ok(ScopedJson::Derived(result.into()))
    }
}

trait JsonTruthy {
    fn is_truthy(&self, include_zero: bool) -> bool;
}

impl JsonTruthy for serde_json::Value {
    fn is_truthy(&self, include_zero: bool) -> bool {
        match self {
            serde_json::Value::Bool(b) => *b,
            serde_json::Value::Number(n) => {
                if include_zero {
                    n.as_f64().is_some_and(|f| !f.is_nan())
                } else {
                    n.as_f64().is_some_and(f64::is_normal)
                }
            }
            serde_json::Value::String(s) => !s.is_empty(),
            serde_json::Value::Array(a) => !a.is_empty(),
            serde_json::Value::Object(o) => !o.is_empty(),
            serde_json::Value::Null => false,
        }
    }
}

#[cfg(test)]
mod test_custom_bool_ops {
    use super::*;
    use handlebars::RenderError;
    use maplit::hashmap;
    use serde::Serialize;

    // A thin wrapper around the handlebars render_template method that includes
    // setup and registration of helpers
    fn setup_and_render_template(tmpl: &str, data: impl Serialize) -> Result<String, RenderError> {
        let mut registry = Handlebars::new();
        registry.set_strict_mode(true);

        registry.register_helper("eq", Box::new(CustomEqHelper));
        registry.register_helper("and", Box::new(CustomAndHelper));

        registry.render_template(tmpl, &data)
    }

    #[test]
    fn test_custom_eq_val_present() {
        let template = r#"{{#if (eq settings.hello "there") }}yep!{{else}}nope!{{/if}}"#;
        assert_eq!(
            setup_and_render_template(
                &template,
                hashmap! {
                    "settings" => hashmap! { "hello" => "there" }
                }
            )
            .unwrap(),
            "yep!"
        );
    }

    #[test]
    fn test_custom_eq_val_missing() {
        let template = r#"{{#if (eq settings.no-hello "there") }}yep!{{else}}nope!{{/if}}"#;
        assert_eq!(
            setup_and_render_template(
                &template,
                hashmap! {
                    "settings" => hashmap! { "hello" => "there" }
                }
            )
            .unwrap(),
            "nope!"
        );
    }

    #[test]
    fn test_custom_eq_infinite_arity() {
        let template =
            r#"{{#if (eq settings.hello settings.other-hello "there") }}yep!{{else}}nope!{{/if}}"#;
        assert_eq!(
            setup_and_render_template(
                &template,
                hashmap! {
                    "settings" => hashmap! { "hello" => "there", "other-hello" => "there" }
                }
            )
            .unwrap(),
            "yep!"
        );

        assert_eq!(
            setup_and_render_template(
                &template,
                hashmap! {
                    "settings" => hashmap! { "hello" => "there", "other-hello" => "not there" }
                }
            )
            .unwrap(),
            "nope!"
        );
    }
}
