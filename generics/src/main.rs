use generics::DoubleDrop;

struct Container(i32, i32);

trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

// explicitly annotate type in <> bracket
impl Contains<i32, i32> for Container {
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    // Grab the first number
    fn first(&self) -> i32 {
        self.0
    }

    // Grab the last number
    fn last(&self) -> i32 {
        self.1
    }
}

// `C` contains `A` and `B`. In the light of that to express `A` and
// `B` again in nusiance.
fn _different<A, B, C>(container: &C) -> i32
where
    C: Contains<A, B>,
{
    container.last() - container.first()
}

fn main() {
    let _empty = generics::Empty;
    let null = generics::Null {
        s: String::from("hello"),
    };
    let container = Container(5, 5);

    null.double_drop(container);
    println!("End!");
}
