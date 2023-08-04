use std::simd::{LaneCount, Simd, SimdInt, SimdUint, SupportedLaneCount};
use crate::Bitboard;
use crate::bitboards;
use crate::PieceColor;
use crate::locate::RankwiseSquareOrdinal;

pub(crate) struct Pattern<const N: usize> {
    pub(crate) ranks: [i8; N],
    pub(crate) files: [i8; N]
}

/// Produces [N], [RankwiseSquareOrdinal]-like quantities, by applying each offset in the given `pattern`
/// to the given `origin`. The return value will be < 0 or > 63 if the origin is sufficiently
/// close to the ends of the board and the pattern contains sufficiently large offsets.
#[inline]
pub(crate) fn translate_n<const N: usize>(origin: RankwiseSquareOrdinal, pattern: &Pattern<N>) -> Simd<isize, N>
    where LaneCount<N>: SupportedLaneCount {
    let files: Simd<isize, N> = Simd::<i8, N>::cast(Simd::<i8, N>::from_array(pattern.files));
    let ranks: Simd<isize, N> = Simd::<i8, N>::cast(Simd::<i8, N>::from_array(pattern.ranks));
    return Simd::<isize, N>::splat(origin as isize) + (Simd::<isize, N>::splat(8) * ranks) + files;
}

#[inline]
pub(crate) fn instantiate_pattern<const N: usize>(origin: RankwiseSquareOrdinal, pattern: &Pattern<N>) -> Bitboard
    where LaneCount<N>: SupportedLaneCount {

    let boards = bitboards::only_n(translate_n(origin, pattern));
    return Simd::<u64, N>::reduce_and(boards)
}

const fn compile_knight_pattern() -> Pattern<8> {
    let mut files: [i8; 8] = [0; 8];
    let mut ranks: [i8; 8] = [0; 8];

    files[0] = -1; ranks[0] = 2;  // Rankwise, Queenside, Black
    files[1] = -2; ranks[1] = 1;  // Filewise, Queenside, Black
    files[2] = -1; ranks[2] = -2; // Rankwise, Queenside, White
    files[3] = -2; ranks[3] = -1; // Filewise, Queenside, White
    files[4] = 1; ranks[4] = 2;   // Rankwise, Kingside, Black
    files[5] = 2; ranks[5] = 1;   // Filewise, Kingside, Black
    files[6] = 2; ranks[6] = -1;  // Filewise, Kingside, White
    files[7] = 1; ranks[7] = -2;  // Rankwise, Kingside, White

    return Pattern { files, ranks }
}

pub(crate) const KNIGHT_PATTERN: Pattern<8> = compile_knight_pattern();

const fn compile_king_pattern() -> Pattern<8> {
    let mut files: [i8; 8] = [0; 8];
    let mut ranks: [i8; 8] = [0; 8];

    // Cardinal Direction (4)

    // Queenside
    // * * *
    // X K *
    // * * *
    files[0] = -1; ranks[0] = 0;

    // Kingside
    // * * *
    // * K X
    // * * *
    files[1] = 1; ranks[1] = 0;

    // Black
    // * X *
    // * K *
    // * * *
    files[2] = 0; ranks[2] = 1;

    // White
    // * * *
    // * K *
    // * X *
    files[3] = 0; ranks[3] = -1;

    // Diagonals (4)

    // Queenside, White
    // * * *
    // * K *
    // X * *
    files[4] = -1; ranks[4] = -1;

    // Queenside, Black
    // X * *
    // * K *
    // * * *
    files[5] = -1; ranks[5] = 1;


    // Kingside, Black
    // * * X
    // * K *
    // * * *
    files[6] = 1; ranks[6] = 1;

    // Kingside, White
    // * * *
    // * K *
    // * * X
    files[7] = 1; ranks[7] = -1;

    return Pattern { files, ranks };
}

pub(crate) const KING_PATTERN: Pattern<8> = compile_king_pattern();

const fn compile_pawn_capture_pattern(color: PieceColor) -> Pattern<2> {
    let mut files: [i8; 2] = [0; 2];
    let mut ranks: [i8; 2] = [0; 2];

    let direction: i8 = match color {
        PieceColor::Black => -1,
        PieceColor::White => 1
    };

    files[0] = -1; ranks[0] = direction;
    files[1] = 1; ranks[1] = direction;

    return Pattern { files, ranks };
}

const BLACK_PAWN_CAPTURE_PATTERN: Pattern<2> =
    compile_pawn_capture_pattern(PieceColor::Black);

const WHITE_PAWN_CAPTURE_PATTERN: Pattern<2> =
    compile_pawn_capture_pattern(PieceColor::Black);

const PAWN_CAPTURE_PATTERN: [Pattern<2>; 2] = [
    BLACK_PAWN_CAPTURE_PATTERN,
    WHITE_PAWN_CAPTURE_PATTERN
];

pub(crate) fn lookup_pawn_capture_pattern(color: PieceColor) -> &'static Pattern<2> {
    return &PAWN_CAPTURE_PATTERN[color as usize];
}