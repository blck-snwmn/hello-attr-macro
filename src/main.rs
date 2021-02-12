use hello_attr_macro::hello;

#[hello]
pub enum State {
    ToDo,
    Doing,
    Done,
    Pending,
}
#[hello(world)]
pub struct Foo {}

#[hello(world = "rust")]
pub fn do_func() {}

#[hello(world => go)]
pub fn do_func2() {}

// マクロの定義でコンパイルエラーになるようにしているので、
// これはコンパイルできない
// #[hello]
// pub trait Sample {}

fn main() {
    println!("Hello, world!");
}
