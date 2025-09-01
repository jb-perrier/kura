use koto::runtime::{KNativeFunction, KValue};

#[must_use]
pub fn make_module() -> KValue {
    KValue::NativeFunction(KNativeFunction::new(|_| {
            println!("Hello from the demo library!");
            Ok(KValue::Null)
        },
    ))
}

