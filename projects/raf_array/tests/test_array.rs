use raf_array::Array;

#[test]
fn test_u8_array_1() {
    let array = Array::<u8>::new(0);
    let slice = array.as_slice();
    assert_eq!(slice.len(), 0);
    assert!(slice.get(0).is_none());
}

#[test]
fn test_u8_array_2() {
    let mut array = Array::<u8>::new(3);
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
    let array = Array::<Baz>::new(3);
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
    let mut factory = || {
        let data = CloneTest { value: idx };
        idx += 1;
        data
    };

    let array = Array::<CloneTest>::new_with_fill(4, &mut factory);
    
    let clone = array.clone();
    let clone2 = clone.clone();

    assert_eq!(array, clone);
    assert_eq!(array, clone2);
    assert_eq!(clone, clone2);
    drop(array);
    assert_eq!(clone, clone2);
}
