#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(trace_macros)]

use crate::{Template, TemplateConstruction};
use prelude::template;

#[template]
struct MyComponent {
    field: i32
}

impl Template for MyComponent {
    type Output = i32;

    fn draw(&self) -> <Self as Template>::Output {
        self.field * 3
    }
}

trait Factor {
    fn factor(&self) -> MyComponent;
}

impl Factor for i32 {
    fn factor(&self) -> MyComponent {
        let mut out = MyComponent::default();
        let this = *self;
        out.on_create(move |w| println!("{this} * {w} = {prod}",
            prod = *w * this));
        out
    }
}

#[test]
fn example() {
    let a = MyComponent {
        field: 6,
        ..Default::default()
    } (|w| {
        MyComponent {
            field: 7,
            ..w.factor()
        } ();
        w
    });
    println!("{}", a);
}