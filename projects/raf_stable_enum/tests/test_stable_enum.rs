use std::sync::{Arc, Mutex};

use raf_stable_enum::stable_enum;

pub struct Foo;

#[derive(Clone)]
pub struct Baz {
    pub value: i32,
    pub second: f64,
}


#[stable_enum]
pub enum MyStableEnum {
    UNKNOWN = 1,
    KFOO(Foo) = 2,
    KBAZ(Baz) = 3,
}


#[test]
fn test_stable_enum_unknown() {
    let value = MyStableEnumBuilder::create_UNKNOWN();
    assert_eq!(value.key(), MyStableEnumKey::UNKNOWN);
    let matched_id = Arc::new(Mutex::new(false));
    value.match_it(
        || {
            let mut guard = matched_id.lock().unwrap();
            *guard = true;
        },
        |_| {},
        |_| {},
    );

    let guard = matched_id.lock().unwrap();
    assert!(*guard);
}

#[test]
fn test_stable_enum_kfoo() {
    let value = MyStableEnumBuilder::create_KFOO(Foo);
    assert_eq!(value.key(), MyStableEnumKey::KFOO);
    let matched_id = Arc::new(Mutex::new(false));
    value.match_it(
        || {},
        |_| {
            let mut guard = matched_id.lock().unwrap();
            *guard = true;
        },
        |_| {},
    );

    let guard = matched_id.lock().unwrap();
    assert!(*guard);
}

#[test]
fn test_stable_enum_kbaz() {
    let baz = Baz { value: 5, second: 1.32 };
    let baz_clone = baz.clone();
    let value = MyStableEnumBuilder::create_KBAZ(baz);
    assert_eq!(value.key(), MyStableEnumKey::KBAZ);
    let matched_id = Arc::new(Mutex::new(false));
    value.match_it(
        || {},
        |_| {},
        |baz| {
            let mut guard = matched_id.lock().unwrap();
            *guard = true;
            assert_eq!(baz.value, baz_clone.value);
            assert_eq!(baz.second, baz_clone.second);
        },
    );

    let guard = matched_id.lock().unwrap();
    assert!(*guard);
}

