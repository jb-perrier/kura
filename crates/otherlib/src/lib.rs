use koto::runtime::{unexpected_args, KMap, KValue};

#[must_use]
pub fn make_module() -> impl Into<KValue> {
    let map = KMap::new();

    map.insert("my_number", 42);
    map.add_fn("say_hello_to", |ctx| match ctx.args() {
            [KValue::Str(name)] => {
                Ok(KValue::Str(format!("Hello, {name}!").into()))
            }
            unexpected => unexpected_args("|string|", unexpected)
    });

    map
}

