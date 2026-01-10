#[macro_export]
macro_rules! vec_string {
    ($($e:expr_2021),*) => {vec![$($e.to_owned()), *]};
    ($($e:expr_2021,)*) => {vec![$($e.to_owned()), *]};
}
