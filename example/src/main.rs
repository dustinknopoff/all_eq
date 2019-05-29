#![allow(dead_code)]
use all_eq::{AllEq, all_eq};

type UUID = String;

#[derive(Debug, AllEq)]
struct Order {
    product_id: UUID,
    quantity: i32,
    name: &'static str,
    category: Category
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.product_id == other.product_id
    }
}

#[derive(Debug, Eq)]
enum Category {
    Living,
    Cooking,
    Clothing,
    Electronics
}

fn main() {
    let order1 = Order {
      product_id: UUID::from("DCD20663-C1DB-434F-9A88-CABC5CDD7878"),
        quantity: 1,
        name: "Granite Pan",
        category: Category::Cooking
    };
    let order2 = Order {
        product_id: UUID::from("DCD20663-C1DB-434F-9A88-CABC5CDD7878"),
        quantity: 14,
        name: "Granite Pan",
        category: Category::Cooking
    };
    // This will pass. In terms of equality we usually only care if 2 sheep have the same `product_id`
    assert_eq!(order1, order2);
    // This will cause a panic. Useful when testing in order to verify that the two objects have the same fields
    // and not necessarily what they actually represent. Perhaps there is a side-effect in a hypothetical
    // method call earlier that caused `order2.quantity` to be mutated when it shouldn't have.
    all_eq!(order1, order2);
    /*
    thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `Order {
    product_id: "DCD20663-C1DB-434F-9A88-CABC5CDD7878",
    quantity: 1,
    name: "Granite Pan",
    category: Cooking,
}`,
 right: `Order {
    product_id: "DCD20663-C1DB-434F-9A88-CABC5CDD7878",
    quantity: 14,
    name: "Granite Pan",
    category: Cooking,
}`', example/src/main.rs:45:5
    */
}
