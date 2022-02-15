#[derive(Debug)]
struct X {
}

impl std::ops::Add<&X> for &X {
    type Output = X;
    fn add(self, _rhs: &X) -> X {
        X { }
    }
}

fn main() {
    let a = X { };
    let b = X { };
    println!("x: {:?}", a+b); //~ ERROR cannot add `X` to `X`
}
