# AllEq

This crate defines 2 key pieces:

1. A derive macro for the `AllEq` trait
2. a `all_eq!` macro for comparisons based on the `AllEq` akin to the stdlib `assert_eq!` macro.

## Example

```rust
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

#[derive(Debug)]
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

```

## What's happening?

Let's say we are building an invoicing software. We are representing orders with the struct `Order`. For the purposes of 
our program, the equality of 2 orders is signified by the equality of their `product_id`s. However, particularly when testing
it's important that our orders are exactly the same in order to hopefully prevent bugs before production.

Let's say for example we add a field to `Order` order_date:

```rust
#[derive(Debug, AllEq)]
struct Order {
    product_id: UUID,
    quantity: i32,
    name: &'static str,
    category: Category,
    order_date: Date
}

// ...
#[test]
fn main() {
    let expected = Order {
      product_id: UUID::from("DCD20663-C1DB-434F-9A88-CABC5CDD7878"),
        quantity: 1,
        name: "Granite Pan",
        category: Category::Cooking,
        order_date: Date::now()
    };
    let actual = Order::new()
        .product_id("DCD20663-C1DB-434F-9A88-CABC5CDD7878")
        .quantity(1)
        .category(Category::Cooking);
     actual.send_order();
    /*
    let actual = Order {
        product_id: UUID::from("DCD20663-C1DB-434F-9A88-CABC5CDD7878"),
        quantity: 14,
        name: "Granite Pan",
        category: Category::Cooking,
        order_date: Date::now()
    };
    */
}
```

Somehow, in the `send_order()` function, the `quantity` field gets set to **14**!

```rust
//...
    // Now, our Orders both have the same product_id but an unforeseen side effect of the `send_order()` method is not caught
    assert_eq!(actual, expected);
    // But here, we catch this side effect.
    all_eq!(actual, expected);
```

### How does it work?

The `AllEq` trait is dependent on all of a struct's fields having implemented `PartialEq` or `Eq`.
For example if we removed the `PartialEq` derive from `Category`,

```bash
error[E0369]: binary operation `==` cannot be applied to type `Category`
  --> example/src/main.rs:10:5
   |
10 |     category: Category
   |     ^^^^^^^^
   |
   = note: an implementation of `std::cmp::PartialEq` might be missing for `Category`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0369`.
```

It uses a procedural macro to generate an implementation of `AllEq` that compares every field of the struct.

The `all_eq!` macro is the `assert_eq!` macro copied out of stdlib and replacing it's comparison with `AllEq` comparison.

## Try it out

```bash
git clone https://www.github.com/dustinknopoff/all_eq
cd all_eq
cargo run
```