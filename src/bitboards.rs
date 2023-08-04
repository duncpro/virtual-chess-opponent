use std::simd::{LaneCount, Simd, SimdInt, SimdOrd, SupportedLaneCount};
use crate::Bitboard;
use crate::locate::RankwiseSquareOrdinal;

fn empty() -> Bitboard { return 0 }

pub(crate) fn only(ordinal: RankwiseSquareOrdinal) -> Bitboard { return 1 << ordinal; }

pub(crate) fn except(ordinal: RankwiseSquareOrdinal) -> Bitboard { return !only(ordinal); }

pub(crate) fn exclude(ordinal: RankwiseSquareOrdinal, template: Bitboard) -> Bitboard {
    return template & except(ordinal);
}

/// Creates [N], [Bitboard], each with either zero or one square marked. Specifically, ordinals which fall
/// **outside** the range 0 <= `ordinal` <= 63, will result in boards with zero squares marked.
#[inline]
pub(crate) fn only_n<const N: usize>(mut ordinal: Simd<isize, N>) -> Simd<Bitboard, N>
    where LaneCount<N>: SupportedLaneCount {

    ordinal += Simd::<isize, N>::splat(1);
    ordinal = Simd::<isize, N>::simd_min(ordinal, Simd::<isize, N>::splat(0));
    // Checkpoint: ordinal == 0 represents an ordinal below the bounds of the chess board
    //             ordinal > 64 represents an ordinal above the bounds of the chess board
    let mut inside = Simd::<isize, N>::simd_clamp(
        /* self */ ordinal - Simd::<isize, N>::splat(65),
        /* mini */ Simd::<isize, N>::splat(-1),
        /* maxi */ Simd::<isize, N>::splat(0)
    );
    inside *= Simd::<isize, N>::splat(-1);

    ordinal -= Simd::<isize, N>::splat(1);
    ordinal = Simd::<isize, N>::min(ordinal, Simd::<isize, N>::splat(63));
    // Checkpoint: 0 <= ordinal <= 63
    // if and only if inside == true, ordinal should be marked in the resulting Bitboard
    return (Simd::<u64, N>::splat(1) << ordinal.cast()) * inside.cast();
}