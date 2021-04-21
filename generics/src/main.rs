use generics::DoubleDrop;

// rewrite the example with associative type
struct Container(i32, i32);

trait Contains {
    // define generic types here which methods will be able to utilize
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    // Specify what types `A` and `B` are. if the input type
    // is `Container(i32, i32) the output types are determined 
    // as `i32` and `i32`
    type A = i32;
    type B = i32;

    // `&Self::A` and `&Self::B` are also vaild here
    fn contains(&self ,number_1: &i32, number_2: &i32) -> bool {
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

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let _empty = generics::Empty;
    let _null = generics::Null {
        s: String::from("hello"),
    };
    let _container = Container(5, 5);

    println!("End!");
}
