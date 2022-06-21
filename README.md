# Template.rs

A rust library for making idiomatic, declarative, 
builder-like patterns that use the struct literal syntax.

No function-like macros required!

## How to use

To make a template, just as you would make a builder, 
you have to consider the following:

1. All the parameters
2. The default state of the template
3. What the template builds into.
4. Its relationship with other objects.

You can define a template using the `#[template]` annotation
to your template model struct, and the `Template` trait.

### 1. Defining the parameters

Simply use the the `#[template]` annotation on a struct with
all the parameters as its fields

```rust
#[template]
pub struct Box {
    orientation: Orientation,
    spacing: i32,
    padding: i32,
    margin: i32,
}

#[template]
pub struct Button {
    padding: i32,
    margin: i32,
    style: StyleDescriptor,
    text: String
}
```

### 2. The default state of the template

By default, this library defines the default state of your
template by `#[derive(Default)]`. Custom `Default` implementations
are also planned.

### 3. What the template builds into

Use the `Template` trait to define what the template builds into, and how

```rust

impl Template for Box {
    type Output = some_other_lib::Box;
    
    fn define(self) -> Self::Output {
        let mut this = some_other_lib::Box::new();
        this.padding = self.padding;
        //...
        this
    }
}

impl Template for Button {
    type Output = some_other_lib::Button;
    
    fn define(self) -> Self::Output {
        let mut this = some_other_lib::Button::new();
        this.padding = self.padding;
        //...
        this
    }
}

```

### 3. What the template builds into

Define different default states by dependency injection by objects
relative to the template's target.

```rust
trait Container {
    fn child<T, W>(&self) -> T
        where T: Template<Output = W>,
              W: some_other_lib::Widget;
}

impl Container for some_other_lib::Box {
    fn child<T, W>(&self) -> T
        where T: Template<Output = W>,
              W: some_other_lib::Widget 
    {
        let this = self.clone();
        let out = Button::default();            // The template
        out.on_create(move |w| this.add(w));    // provided by the #[template] annotation
        out
    }
}
```

### Using the template

All annotated templates implement the following traits for building the target types:
- `for<A, F: FnOnce(Self::Output) -> A> FnOnce(F) -> A` 
- `FnOnce() -> Self::Output`

Those traits can be invoked right after the struct literal in a "currying" fashion.

This is a simple example of idiomatic templates:

```rust
Box {
    orientation: Orientation::HORIZONTAL,
    padding: 6,
    spacing: 6,
    ..Default::default()
} (|w| {
    Box {
        orientation: Orientation::VERTICAL,
        spacing: 6,
        ..w.child()
    } (|w| {

        Button {
            text: "Column btn 1",
            ..w.child()
        }();    
        // Function call constructs button 
        // and adds it to the box as per the
        // ..w.child() injection directive

        Button {
            text: "Column btn 2"
            ..w.child()
        }();

    }); // function call runs the lambda argument
        // on the output and then returns it

    Button {
        text: "Big btn",
        ..w.child()
    }();
})

// expected result:
// |--------------|---------|
// | Column btn 1 |         |
// |--------------| Big Btn |
// | Column btn 2 |         |
// |--------------|---------|
```

In case you don't want to use direct function calls, 
you can use the following equivalent methods:
- `fn build<A>(self, F: impl FnOnce(Self::Output) -> A) -> A`
- `fn create(self) -> Self::Output`

```rust
Box {
    orientation: Orientation::HORIZONTAL,
    padding: 6,
    spacing: 6,
    ..Default::default()
}.build(|w| {       // creations with lambdas use the "build" method
    Button {
        text: "Hello World",
        ..w.child()
    }.create();     // creations without arguments use the "create" method
})
```

## Current Problems and Future Plans

1. Allow the user to define custom default states for templates
2. Create a `Templatable` trait associates the Output of a template with its template,
so that you don't have to associate it yourself. Eg.:
```rust
trait Templatable {
    type New: Template<Output = Self>;
}

// ...so that

use some_other_lib::Box;

Box::New {
    //...
} ();
```
3. Currently some type analysers do not consider `std::ops::FnOnce` implementations,
so they automatically label `Struct { /*...*/ } ( /*...*/ )` syntaxes as wrong
when they are perfectly legal and fine as per the `std::ops::FnOnce` trait definition.
For example, in the jetbrains Rust Plugin. Until such problems are solved, either use a
different analyser, or use the alternative `.build()` and `.create()` methods.