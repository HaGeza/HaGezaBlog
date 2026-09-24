#[macro_export]
macro_rules! vec3_relative_eq {
    ($lhs:expr, $rhs:expr, epsilon = $eps:expr) => {
        approx::relative_eq!($lhs.x, $rhs.x, epsilon = $eps)
            && approx::relative_eq!($lhs.y, $rhs.y, epsilon = $eps)
            && approx::relative_eq!($lhs.z, $rhs.z, epsilon = $eps)
    };

    ($lhs:expr, $rhs:expr) => {
        approx::relative_eq!($lhs.x, $rhs.x)
            && approx::relative_eq!($lhs.y, $rhs.y)
            && approx::relative_eq!($lhs.z, $rhs.z)
    };
}

#[macro_export]
macro_rules! assert_vec3_relative_eq {
    ($lhs:expr, $rhs:expr, epsilon = $eps:expr) => {
        assert!(crate::vec3_relative_eq!($lhs, $rhs, epsilon = $eps));
    };

    ($lhs:expr, $rhs:expr) => {
        assert!(crate::vec3_relative_eq!($lhs, $rhs));
    };
}
