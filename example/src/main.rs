use all_eq::{AllEq, all_eq};

#[derive(Debug, AllEq, PartialEq)]
struct Ex {
    r: String,
    num: i32,
    is_real: IsReal
}

#[derive(Debug, PartialEq)]
enum IsReal {
    Real,
    Fake
}

#[derive(Debug, AllEq, PartialEq)]
struct Ex2((String, usize));

fn main() {
    let ex1 = Ex { 
        r: String::from("ma"), 
        num: 22,
        is_real: IsReal::Real
        };
    let ex2 = Ex {
         r: String::from("ma"),
         num: 22,
         is_real: IsReal::Fake
         };
    all_eq!(ex1, ex2);

    let ex3 = Ex2((String::from("fantasia"), 3usize));
    let ex4 = Ex2((String::from("fantasia"), 3usize));
    all_eq!(ex3, ex4);
}
