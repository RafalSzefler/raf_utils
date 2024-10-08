use std::sync::{Arc, Mutex};

use raf_array::array::Array;

#[test]
fn doc_test() {
    let index = &mut 1;
    let factory = || {
        let value = *index;
        *index = value * 2;
        value
    };
    let arr = Array::from_factory(5, factory).unwrap();
    assert_eq!(arr.as_slice(), &[1, 2, 4, 8, 16]);
}

#[test]
fn test_u8_array_1() {
    let array = Array::<u8>::new_default(0).unwrap();
    let slice = array.as_slice();
    assert_eq!(slice.len(), 0);
    assert!(slice.get(0).is_none());
}

#[test]
fn test_u8_array_2() {
    let mut array = Array::<u8>::new_default(3).unwrap();
    let slice = array.as_slice_mut();
    slice[0] = 1;
    slice[1] = 15;
    slice[2] = 3;
    assert_eq!(array.as_slice(), &[1, 15, 3]);
}

struct Baz {
    pub value: i32,
}

impl Default for Baz {
    fn default() -> Self {
        Self { value: 7 }
    }
}

#[test]
fn test_baz_array() {
    let array = Array::<Baz>::new_default(3).unwrap();
    let slice = array.as_slice();
    for item in slice {
        assert_eq!(item.value, 7);
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
struct CloneTest {
    pub value: i32,
}

#[test]
fn test_array_clone() {
    let mut idx = 0;
    let factory = || {
        let data = CloneTest { value: idx };
        idx += 1;
        data
    };

    let array = Array::<CloneTest>::from_factory(4, factory).unwrap();
    
    let clone = array.clone();
    let clone2 = clone.clone();

    assert_eq!(array, clone);
    assert_eq!(array, clone2);
    assert_eq!(clone, clone2);
    drop(array);
    assert_eq!(clone, clone2);
}


struct Droppable {
    counter: Arc<Mutex<usize>>
}

impl Drop for Droppable {
    fn drop(&mut self) {
        let mut guard = self.counter.lock().unwrap();
        *guard += 1;
    }
}

#[test]
fn test_drop() {
    const ARRAY_LEN: usize = 4;
    let counter = Arc::new(Mutex::new(0usize));

    {
        let clone = counter.clone();
        let factory = || {
            Droppable { counter: clone.clone() }
        };
        let _array = Array::<Droppable>::from_factory(ARRAY_LEN, factory);
    }

    {
        let guard = counter.lock().unwrap();
        assert_eq!(*guard, ARRAY_LEN);
    }
}
