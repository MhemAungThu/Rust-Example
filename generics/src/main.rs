use std::ops::{Add, Mul, Sub};

// rewrite the example with associative type
struct Container(i32, i32);

trait Contains {
    // define generic types here which methods will be able to utilize
    type A: Sub + Add + Mul;

    fn contains(&self, _: &Self::A, _: &Self::A) -> bool;
    fn first(&self) -> Self::A;
    fn last(&self) -> Self::A;
}

impl Contains for Container {
    // Specify what types `A` and `B` are. if the input type
    // is `Container(i32, i32) the output types are determined
    // as `i32` and `i32`
    type A = i32;

    // `&Self::A` and `&Self::B` are also vaild here
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    // Grab the first number
    fn first(&self) -> i32 {
        self.0
    }

    // Grab the second number
    fn last(&self) -> i32 {
        self.1
    }
}

fn difference<C: Contains>(container: &C) -> <<C as Contains>::A as Sub>::Output {
    container.last() - container.first()
}

fn main() {
    let _empty = generics::Empty;
    let _null = generics::Null {
        s: String::from("hello"),
    };
    let container = Container(5, 5);
    let result = difference(&container);

    println!("{}", result + 100_i32);

    let float_result = difference(&MyContainer(3.14, 6.28));
    println!("{}", float_result + 0.001_f64);
    println!("End!");
}

// try to implemnt in my own type
struct MyContainer(f64, f64);

impl Contains for MyContainer {
    type A = f64;

    fn contains(&self, point_1: &f64, point_2: &f64) -> bool {
        (&self.0 == point_1) && (&self.1 == point_2)
    }

    fn first(&self) -> f64 {
        self.0
    }

    fn last(&self) -> f64 {
        self.1
    }
}
