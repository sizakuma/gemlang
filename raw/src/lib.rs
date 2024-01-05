#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MutRaw<T> {
    None,
    Some(*mut T)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ConstRaw<T> {
    None,
    Some(*const T)
}

impl<T> MutRaw<T> {
    pub fn unwrap(&self) -> &mut T {
        match *self {
            MutRaw::Some(ptr) => unsafe { &mut *ptr },
            _ => panic!("Trying to unwrap a MutLink::None!"),
        }
    }
}

impl<T> ConstRaw<T> {
    pub fn unwrap(&self) -> &T {
        match *self {
            ConstRaw::Some(ptr) => unsafe { &*ptr },
            _ => panic!("Trying to unwrap a MutLink::None!"),
        }
    }
}

#[macro_export]
macro_rules! raw {
    (&mut $expr:expr) => {
        MutRaw::Some(&mut $expr)
    };
    (&const $expr:expr) => {
        ConstRaw::Some(&$expr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct Test {
        x: i64,
        y: i32,
    }

    #[test]
    fn it_works() {
        fn testf(raw: MutRaw<Test>) {
            let r = raw.unwrap();
            assert_eq!(r.x, 10);
            assert_eq!(r.y, 12);
            r.x = 16;
        }

        let mut test = Test { x: 10, y: 12 };
        let raw = raw!(&mut test);
        testf(raw);
        assert_eq!(test.x, 16);
    }
}
