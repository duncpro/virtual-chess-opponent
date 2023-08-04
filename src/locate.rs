use std::cmp::{max, min};
use crate::{Antidiagonal, Diagonal, split_so, SquareOrdinal};

/// Represents either a *antidiagonal square coordinate* or a *diagonal square coordinate*.
pub(crate) struct DiagonalSquareCoordinate {
    pub(crate) diagonal: usize,
    pub(crate) offset: usize
}

/// Calculates the *antidiagonal square coordinate* which is associated with the given [SquareOrdinal].
/// Every square ordinal has exactly one *anti-diagonal square coordinate* describing the square's
/// position in terms of the [Antidiagonal] intersecting the square and the distance (measured in
/// squares) from the beginning of that [Antidiagonal] to the square.
pub(crate) fn locate_ad(sordinal: SquareOrdinal) -> DiagonalSquareCoordinate {
    let (rank, file) = split_so(sordinal);
    let antidiagonal: Antidiagonal = rank + file;
    let origin_sordinal = antidiagonal + (8 * max(antidiagonal - 8, 0));
    return DiagonalSquareCoordinate {
        diagonal: antidiagonal,
        offset: (sordinal - origin_sordinal) / 8
    };
}

/// Calculates the *diagonal square coordinate* which is associated with the given [SquareOrdinal].
/// Evey square ordinal has exactly one *diagonal square coordinate* describing the square's
/// position in terms of the [Diagonal] intersecting the square and the distance (measured in
/// squares) from the beginning of that [Diagonal] to the square.
pub(crate) fn locate_d(sordinal: SquareOrdinal) -> DiagonalSquareCoordinate {
    let rank: usize = sordinal / 8;
    let file: usize = 7 - (sordinal % 8);
    let diagonal: Diagonal = rank + file;
    let reflected_sordinal = rank * 8 + file;
    let origin_sordinal = diagonal + (8 * max((diagonal as isize) - 8isize, 0isize)) as usize;
    return DiagonalSquareCoordinate {
        diagonal,
        offset: (reflected_sordinal - origin_sordinal) / 8
    };
}

/// Calculates the [SquareOrdinal] of a *diagonal square coordinate*.
pub(crate) fn reverse_locate_d(dordinal: DiagonalSquareCoordinate) -> SquareOrdinal {
    let base_file = (7usize - min(7usize, dordinal.diagonal));
    let base_rank = max(0isize, dordinal.diagonal as isize - 7isize) as usize;
    let offset = (base_rank * 8) + (dordinal.offset * 9usize);
    return (base_file + offset) as usize;
}

/// Calculates the [SquareOrdinal] of a *antidiagonal square coordinate*.
pub(crate) fn reverse_locate_ad(dordinal: DiagonalSquareCoordinate) -> SquareOrdinal {
    let base_file = min(dordinal.diagonal, 7);
    let base_rank = usize::saturating_sub(dordinal.diagonal, 7usize);
    return base_file + (base_rank * 8) + (dordinal.offset * 7);
}