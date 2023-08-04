/// Computes the <i>n</i>th triangular [number](https://en.wikipedia.org/wiki/Triangular_number).
pub(crate) fn triangle(n: usize) -> usize { return (n * (n + 1)) / 2; }

/// Incrementally computes the [square figurate number](https://en.wikipedia.org/wiki/Figurate_number)
/// <i>n<sup>2</sup></i>, beginning at its corner, and continuing to the <i>i</i>th diagonal.
///
/// The <i>i</i>th partial square figurate is the sum of the values of the diagonals `[1, i]`.
///
/// A square number <i>n<sup>2</sup></i> has exactly 2<i>n</i> - 1 diagonals.
/// The size of the <i>d</i>th diagonal is equal to <i>n</i> - |<i>d</i> - <i>n</i>|.
/// It can be calculated via [measure_diagonal].
pub(crate) fn partial_diamond_figurate(n: usize, i: usize) -> usize {
    use std::cmp::min;
    let total_diagonals = 2 * n - 1;
    triangle(min(n, i)) + triangle(n - 1) - triangle(min(total_diagonals - i, n - 1))
}

pub(crate) fn measure_diagonal(n: usize, d: usize) -> usize {
    let distance_from_principal_diagonal = isize::abs(d as isize - n as isize);
    return n - (distance_from_principal_diagonal as usize);
}