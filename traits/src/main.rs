mod basket;
mod stack;
mod container;

use basket::Basket;
use stack::Stack;
use container::Container;

fn add_string_to_container<T: Container<String>>(container: &mut T, item: String) {
    c.put(item);
}

fn main() {
    let mut b1 = Basket::new(String::from("Hi there"));
    let b2 = Basket::new(10 as i8);
    let b3 = Basket::new(true);

    let mut s1 = Stack::new(vec![String::from("hi")]);
    let s2 = Stack::new(vec![1, 2, 3]);

    add_string_to_container(&mut b1, String::from("Hi basket"));
    add_string_to_container(&mut s1, String::from("Hi stack"));
}
