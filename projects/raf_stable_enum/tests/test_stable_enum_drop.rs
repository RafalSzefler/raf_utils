use std::sync::{Arc, Mutex};

use raf_stable_enum::stable_enum;

pub struct Foo {
    pub value: Arc<Mutex<i32>>,
}

impl Drop for Foo {
    fn drop(&mut self) {
        let mut guard = self.value.lock().unwrap();
        *guard += 1;
    }
}


#[stable_enum]
pub enum MyDroppableEnum {
    UNKNOWN = 0,
    FOO(Foo) = 1,
    BAZ(usize) = 3,
}

fn get_value<T: Clone>(arc: &Arc<Mutex<T>>) -> T {
    let guard = arc.lock().unwrap();
    guard.clone()
}

#[test]
fn test_drop() {
    const DATA_SIZE: usize = 333;
    let value = Arc::new(Mutex::new(0));
    let value2 = Arc::new(Mutex::new(5));
    let mut data = Vec::<MyDroppableEnum>::with_capacity(DATA_SIZE);

    for idx in 0..DATA_SIZE {
        data.push(MyDroppableEnumBuilder::create_UNKNOWN());
        data.push(MyDroppableEnumBuilder::create_BAZ(idx));
        let foo = Foo {
            value: value.clone()
        };
        data.push(MyDroppableEnumBuilder::create_FOO(foo));
        let foo2 = Foo {
            value: value2.clone()
        };
        data.push(MyDroppableEnumBuilder::create_FOO(foo2));
    }

    assert_eq!(get_value(&value), 0);
    assert_eq!(get_value(&value2), 5);

    drop(data);

    assert_eq!(get_value(&value), 333);
    assert_eq!(get_value(&value2), 338);
}

