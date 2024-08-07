use std::sync::{Arc, Mutex};

use raf_array::atomic_array::StrongArray;

#[test]
fn test_atomic_array_1() {
    let counter = Arc::new(Mutex::new(0));

    let factory_counter = counter.clone();
    let factory = || {
        let mut guard = factory_counter.lock().unwrap();
        let tmp_value = *guard;
        *guard = tmp_value + 3;
        tmp_value
    };

    let array = StrongArray::new(4, factory).unwrap();
    let expected = &[0, 3, 6, 9];
    assert_eq!(array.as_slice(), expected);
    assert_eq!(array.strong_count(), 1);
    assert_eq!(array.weak_count(), 1);

    let array_clone = array.clone();
    assert_eq!(array_clone.as_slice(), expected);
    assert_eq!(array.id(), array_clone.id());

    assert_eq!(array.strong_count(), 2);
    assert_eq!(array_clone.strong_count(), 2);
    assert_eq!(array.weak_count(), 1);
    assert_eq!(array_clone.weak_count(), 1);
    drop(array_clone);
    assert_eq!(array.strong_count(), 1);
    assert_eq!(array.weak_count(), 1);

    let weak = array.downgrade();
    assert_eq!(array.strong_count(), 1);
    assert_eq!(array.weak_count(), 2);
    assert_eq!(weak.strong_count(), 1);
    assert_eq!(weak.weak_count(), 2);
    assert_eq!(array.id(), weak.id());
    drop(array);

    assert_eq!(weak.strong_count(), 0);
    assert_eq!(weak.weak_count(), 1);
}

#[test]
fn test_atomic_array_1_with_release() {
    let counter = Arc::new(Mutex::new(0));

    let factory_counter = counter.clone();
    let factory = || {
        let mut guard = factory_counter.lock().unwrap();
        let tmp_value = *guard;
        *guard = tmp_value + 3;
        tmp_value
    };

    let array = StrongArray::new(4, factory).unwrap();
    let expected = &[0, 3, 6, 9];
    assert_eq!(array.as_slice(), expected);
    assert_eq!(array.strong_count(), 1);
    assert_eq!(array.weak_count(), 1);

    let array_clone = array.clone();
    assert_eq!(array_clone.as_slice(), expected);
    assert_eq!(array.id(), array_clone.id());

    assert_eq!(array.strong_count(), 2);
    assert_eq!(array_clone.strong_count(), 2);
    assert_eq!(array.weak_count(), 1);
    assert_eq!(array_clone.weak_count(), 1);
    let array_clone_release = array_clone.release();
    assert!(array_clone_release.is_none());
    assert_eq!(array.strong_count(), 1);
    assert_eq!(array.weak_count(), 1);

    let weak = array.downgrade();
    assert_eq!(array.strong_count(), 1);
    assert_eq!(array.weak_count(), 2);
    assert_eq!(weak.strong_count(), 1);
    assert_eq!(weak.weak_count(), 2);
    assert_eq!(array.id(), weak.id());
    let array_release = array.release().unwrap();

    assert_eq!(weak.strong_count(), 0);
    assert_eq!(weak.weak_count(), 2);

    drop(array_release);

    assert_eq!(weak.strong_count(), 0);
    assert_eq!(weak.weak_count(), 1);
}

#[test]
fn test_atomic_array_2() {
    struct OnDrop {
        pub val: i32,
        pub vec: Arc<Mutex<Vec<i32>>>
    }

    impl Drop for OnDrop {
        fn drop(&mut self) {
            let mut guard = self.vec.lock().unwrap();
            guard.push(self.val);
        }
    }

    let counter = Arc::new(Mutex::new(0));
    let counter_clone = counter.clone();
    let vec = Arc::new(Mutex::new(Vec::<i32>::new()));
    let vec_clone = vec.clone();
    let factory = || {
        let ct = {
            let mut guard = counter_clone.lock().unwrap();
            let val = *guard;
            *guard += 1;
            val
        };
        OnDrop {
            val: ct,
            vec: vec_clone.clone(),
        }
    };

    let array = StrongArray::new(7, factory).unwrap();

    {
        let guard = vec.lock().unwrap();
        let vec_ref: &Vec<i32> = guard.as_ref();
        assert_eq!(vec_ref.as_slice(), []);
    }

    let clone = array.clone();

    assert_eq!(array.id(), clone.id());

    {
        let guard = vec.lock().unwrap();
        let vec_ref: &Vec<i32> = guard.as_ref();
        assert_eq!(vec_ref.as_slice(), []);
    }

    drop(clone);

    {
        let guard = vec.lock().unwrap();
        let vec_ref: &Vec<i32> = guard.as_ref();
        assert_eq!(vec_ref.as_slice(), []);
    }

    drop(array);

    {
        let guard = vec.lock().unwrap();
        let vec_ref: &Vec<i32> = guard.as_ref();
        assert_eq!(vec_ref.as_slice(), [0, 1, 2, 3, 4, 5, 6]);
    }
}


#[test]
fn test_atomic_array_default() {
    #[derive(Debug, PartialEq, Eq, Hash)]
    struct Defaulter {
        pub value: i32
    }

    impl Default for Defaulter {
        fn default() -> Self {
            Self { value: -5 }
        }
    }

    let strong = StrongArray::<Defaulter>::new_default(3).unwrap();
    assert_eq!(strong.as_slice(), [Defaulter { value: -5 }, Defaulter { value: -5 }, Defaulter { value: -5 }]);
}


#[test]
fn test_atomic_array_empty() {
    for _ in 0..5 {
        let strong1 = StrongArray::<i32>::default();
        assert_eq!(strong1.as_slice(), []);
        assert!(strong1.release().is_none());

        let strong2 = StrongArray::<bool>::default();
        assert_eq!(strong2.as_slice(), []);
        assert!(strong2.release().is_none());

        let strong3 = StrongArray::<usize>::default();
        assert_eq!(strong3.as_slice(), []);
        assert!(strong3.release().is_none());
    }
}
