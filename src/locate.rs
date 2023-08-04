use std::cmp::{max, min};
use crate::{Antidiagonal, Diagonal};

pub(crate) type Rank = usize;
pub(crate) type File = usize;
pub(crate) type FilewiseSquareOrdinal = usize;

#[derive(Copy, Clone)]
pub(crate) enum BoardLayout {
    /// A board arranged in terms of its [Antidiagonal]s. Under this layout, square ordinal + 1 equals
    /// the next square along the antidiagonal.
    Antidiagonal = 0,

    /// A board arranged in terms of its [Diagonal]s. Under this layout, square ordinal + 1 equals
    /// the next square along the diagonal.
    Diagonal = 1,

    /// A board arranged in terms of its [Rank]s. Under this layout, square ordinal + 1 equals
    /// the next square along the rank.
    Rankwise = 2,

    /// A board arranged in terms of its [File]s. Under this layout, square ordinal + 1 equals
    /// the next square along the file.
    Filewise = 3
}


/// A natural number in the range 0 <= i <= 63 denoting a particular
/// square's location under [BoardLayout::Rankwise].
///
/// ```
///                      White
///            0   1   2   3   4   5   6   7
///          ────────────────────────────── F
///  Q   0 │   0   1   2   3   4   5   6   7    K
///  u   1 │   8   9  10  11  12  13  14  15    i
///  e   2 │  16  17  18  19  20  21  22  23    n
///  e   3 │  24  25  26  27  28  29  30  31    g
///  n   4 │  32  33  34  35  36  37  38  39    s
///  s   5 │  40  41  42  43  44  45  46  47    i
///  i   6 │  48  49  50  51  52  53  54  55    d
///  d   7 │  56  57  58  59  60  61  62  63    e
///  e     R
///                      Black
/// ```
///
/// This is the standard coordinate system. All other coordinate systems can be converted
/// from/to the this system using the locate, and reverse_locate functions respectively.
///
pub type RankwiseSquareOrdinal = usize;


/// Represents either a *antidiagonal square coordinate* or a *diagonal square coordinate*.
pub(crate) struct DiagonalSquareCoordinate {
    pub(crate) diagonal: usize,
    pub(crate) offset: usize
}

/// Calculates the *antidiagonal square coordinate* which is associated with the given [RankwiseSquareOrdinal].
/// Every square ordinal has exactly one *anti-diagonal square coordinate* describing the square's
/// position in terms of the [Antidiagonal] intersecting the square and the distance (measured in
/// squares) from the beginning of that [Antidiagonal] to the square.
pub(crate) fn locate_ad(sordinal: RankwiseSquareOrdinal) -> DiagonalSquareCoordinate {
    let (rank, file) = split_rwc(sordinal);
    let antidiagonal: Antidiagonal = rank + file;
    let origin_sordinal = antidiagonal + (8 * max(antidiagonal - 8, 0));
    return DiagonalSquareCoordinate {
        diagonal: antidiagonal,
        offset: (sordinal - origin_sordinal) / 8
    };
}

/// Calculates the *diagonal square coordinate* which is associated with the given [RankwiseSquareOrdinal].
/// Evey square ordinal has exactly one *diagonal square coordinate* describing the square's
/// position in terms of the [Diagonal] intersecting the square and the distance (measured in
/// squares) from the beginning of that [Diagonal] to the square.
pub(crate) fn locate_d(sordinal: RankwiseSquareOrdinal) -> DiagonalSquareCoordinate {
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

/// Calculates the [RankwiseSquareOrdinal] of a *diagonal square coordinate*.
pub(crate) fn reverse_locate_d(dordinal: DiagonalSquareCoordinate) -> RankwiseSquareOrdinal {
    let base_file = (7usize - min(7usize, dordinal.diagonal));
    let base_rank = max(0isize, dordinal.diagonal as isize - 7isize) as usize;
    let offset = (base_rank * 8) + (dordinal.offset * 9usize);
    return base_file + offset;
}

/// Calculates the [RankwiseSquareOrdinal] of a *antidiagonal square coordinate*.
pub(crate) fn reverse_locate_ad(dordinal: DiagonalSquareCoordinate) -> RankwiseSquareOrdinal {
    let base_file = min(dordinal.diagonal, 7);
    let base_rank = usize::saturating_sub(dordinal.diagonal, 7usize);
    return base_file + (base_rank * 8) + (dordinal.offset * 7);
}

pub(crate) fn locate_fw(ordinal: RankwiseSquareOrdinal) -> FilewiseSquareOrdinal {
    let (rank, file) = split_rwc(ordinal);
    return file * 8 + rank;
}

pub(crate) fn reverse_locate_fw(ordinal: FilewiseSquareOrdinal) -> RankwiseSquareOrdinal {
    let (rank, file) = split_fwc(ordinal);
    return rank * 8 + file;
}

pub(crate) fn split_rwc(sordinal: RankwiseSquareOrdinal) -> (Rank, File) {
    let rank: usize = sordinal / 8;
    let file: usize = sordinal % 8;
    return (rank, file);
}

pub(crate) fn split_fwc(sordinal: FilewiseSquareOrdinal) -> (Rank, File) {
    let rank: usize = sordinal % 8;
    let file: usize = sordinal / 8;
    return (rank, file);
}