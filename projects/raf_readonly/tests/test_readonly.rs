use std::{hash::Hash, marker::PhantomData};

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
