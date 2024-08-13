use std::{hash::Hash, marker::PhantomData, sync::{Arc, Mutex}};

use raf_readonly::readonly;

use rstest::rstest;

pub struct Baz<'a> {
    phantom: PhantomData<&'a ()>,
}

#[readonly(ctr_vis=pub, ctr_name=new)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct User<'a, T>
    where T: PartialEq + Eq + Hash
{
    pub name: String,
    pub age: i32,
    pub misc: T,
    pub x: &'a usize,
}

#[rstest]
fn test_user() {
    let name = "john".to_owned();
    let age = 32;
    let tmp = 15;
    let user = User::new(name.clone(), age, true, &tmp);
    assert_eq!(user.name(), &name);
    assert_eq!(user.age(), age);
    assert_eq!(user.x(), &tmp);
    assert!(user.misc());
    let second_user = User::new(name.clone(), age, true, &tmp);
    assert_eq!(user, second_user);
}

pub struct CustomDrop {
    pub text: String,
    pub vec: Arc<Mutex<Vec<String>>>,
}

impl Drop for CustomDrop {
    fn drop(&mut self) {
        let mut guard = self.vec.lock().unwrap();
        guard.push(self.text.clone());
    }
}


#[readonly(with_release=true)]
pub struct DroppableWrapper {
    pub droppable: CustomDrop,
    pub value: u32,
}


#[rstest]
fn test_droppable_wrapper() {
    let real_vec = Arc::new(Mutex::new(Vec::new()));

    fn validate(vec: &Arc<Mutex<Vec<String>>>, func: impl FnOnce(&Vec<String>) -> bool, msg: &str) {
        let guard = vec.lock().unwrap();
        assert!(func(guard.as_ref()), "{}", msg)
    }    

    let cd = CustomDrop {
        text: "foo".to_owned(),
        vec: real_vec.clone(),
    };
    let wrapper = DroppableWrapper::new(cd, 15);
    assert_eq!(wrapper.value(), 15);
    assert_eq!(wrapper.droppable().text, "foo");
    validate(&wrapper.droppable().vec, |v| { v == &Vec::<String>::new() }, "Invalid wrapper");

    let released = wrapper.release();
    validate(&released.droppable.vec, |v| { v == &Vec::<String>::new() }, "Invalide released");
    assert_eq!(released.droppable.text, "foo");
    assert_eq!(released.value, 15);
    drop(released);
    validate(&real_vec, |v| { v == &Vec::from(["foo"]) }, "Invalid final");
}
