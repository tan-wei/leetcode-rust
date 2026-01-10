#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

#[macro_export]
macro_rules! point {
    ($($e:expr_2021),*) => {
        {
            let vec = vec![$($e.to_owned()), *];
            Point::new(vec[0], vec[1])
        }
    };
    ($($e:expr_2021,)*) => (point![$($x),*])
}
