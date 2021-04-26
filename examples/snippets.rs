use std::collections::HashMap;

// [md-bakery: begin @ snippet-a]
#[derive(Debug)]
struct Foo {
    bar: HashMap<String, usize>,
}
// [md-bakery: end]

fn main() {
    // [md-bakery: begin @ snippet-b]
    let foo = Foo {
        bar: {
            let mut result = HashMap::new();
            result.insert("answer".to_owned(), 42);
            result
        },
    };

    println!("{:?}", foo);
    // [md-bakery: end]
}
