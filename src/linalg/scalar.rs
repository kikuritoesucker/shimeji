pub fn lerp<T: num_traits::Float>(lhs: T, rhs: T, t: T) -> T {
    lhs * (T::one() - t) + rhs * t
}

