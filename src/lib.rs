pub use all_eq_derive::*;

pub trait AllEq : PartialEq {
    fn all_eq(&self, other: &Self) -> bool;
}

/// Extracted for stdlib `assert_eq!` macro
    #[macro_export]
    macro_rules! all_eq {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(all_eq::AllEq::all_eq(&*left_val, &*right_val)) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left == right)`
  left: `{:#?}`,
 right: `{:#?}`"#, &*left_val, &*right_val)
                }
            }
        }
    });
    ($left:expr, $right:expr,) => ({
        test_eq!($left, $right)
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(all_eq::AllEq::all_eq(&*left_val, &*right_val)) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left == right)`
  left: `{:#?}`,
 right: `{:#?}`: {}"#, &*left_val, &*right_val,
                           format_args!($($arg)+))
                }
            }
        }
    });
}