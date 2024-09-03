use raf_stable_enum::stable_enum;

pub struct Foo(bool);

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
}

#[test]
fn test_stable_enum_kfoo() {
    let value = MyStableEnumBuilder::create_KFOO(Foo(true));
    assert_eq!(value.key(), MyStableEnumKey::KFOO);
    let foo = unsafe { value.KFOO() };
    assert!(foo.0);
}

#[test]
fn test_stable_enum_kbaz() {
    let baz = Baz { value: 5, second: 1.32 };
    let value = MyStableEnumBuilder::create_KBAZ(baz.clone());
    assert_eq!(value.key(), MyStableEnumKey::KBAZ);
    let baz_result = unsafe { value.KBAZ() };
    assert_eq!(baz_result.value, baz.value);
    assert_eq!(baz_result.second, baz.second);
}

