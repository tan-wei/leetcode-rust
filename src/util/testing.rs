#[macro_export]
macro_rules! assert_eq_sorted {
    ($left:expr_2021, $right:expr_2021) => ({
        let (mut v1, mut v2) = ($left, $right);
        v1.sort();
        v2.sort();
        assert_eq!(v1, v2)
    });
    ($left:expr_2021, $right:expr_2021,) => ({
        assert_eq_sorted!($left, $right)
    });
    ($left:expr_2021, $right:expr_2021, $($arg:tt)+) => ({
        let (mut v1, mut v2) = ($left, $right);
        v1.sort();
        v2.sort();
        assert_eq!(v1, v2, $($arg)+)
    });
}
