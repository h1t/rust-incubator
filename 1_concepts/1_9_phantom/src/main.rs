use rand::seq::SliceRandom;
use std::marker::PhantomData;

struct Fact<T>(PhantomData<T>);

impl<T> Fact<T> {
    fn new() -> Self {
        Self(PhantomData)
    }

    fn fact(&self) -> &'static str {
        ["Vec is heap-allocated.", "Vec may re-allocate on growing"]
            .choose(&mut rand::thread_rng())
            .expect("slice is not empty")
    }
}

fn main() {
    let f: Fact<Vec<()>> = Fact::new();
    println!("Fact about Vec: {}", f.fact());
    println!("Fact about Vec: {}", f.fact());
}
