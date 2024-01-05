mod raw;

pub struct Test {
    x: i32
}

#[cfg(test)]
mod tests {
    use crate::raw::{MutRaw, Raw};
    use super::*;

    #[test]
    fn it_works() {
        let mut t = Test { x: 10 };
        let raw = raw!(&mut t);
        let mut _t = raw.unwrap();
        _t.x = 11;
        println!("{}", t.x);
    }
}
