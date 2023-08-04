#![feature(variant_count)]
#![feature(exclusive_range_pattern)]
#![feature(portable_simd)]

mod misc;
mod bitboards;
mod locate;
mod obstruct;
mod bitlanes;
mod movegen;
mod move_patterns;

use std::mem::variant_count;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Shl;
use std::ops::ShlAssign;
use std::simd::SimdInt;
use std::simd::SimdOrd;
use std::simd::SimdUint;
use std::simd::SupportedLaneCount;
use std::u16;
use locate::BoardLayout;

#[derive(Copy, Clone)]
enum PieceKind { Rook = 0, Knight = 1, Bishop = 2, Queen = 3, King = 4, Pawn = 5 }

#[derive(Copy, Clone)]
enum PieceColor { Black = 0, White = 1 }

fn opponent(color: PieceColor) -> PieceColor {
    return match color {
        PieceColor::Black => PieceColor::White,
        PieceColor::White => PieceColor::Black
    }
}

type Bitlane = u8;

type Bitboard = u64;

struct Piece { color: PieceColor, kind: PieceKind }

struct Square { occupant: Option<Piece> }

pub(crate) struct Occupancy {
    boards: [Bitboard; variant_count::<PieceColor>()]
}

/// Describes the occupancy of the chessboard (which squares have white pieces, which squares have
/// black pieces). Specifically, this struct encapsulates multiple [Bitboard], each describing the
/// occupancy of the chess board in a different [BoardLayout]. See also, [layout].
pub(crate) struct CompositeOccupancy {
    boards: [Occupancy; variant_count::<BoardLayout>()]
}


/// Narrows the given [CompositeOccupancy] into [Occupancy] under the given layout.
pub(crate) fn layout(rboard: &CompositeOccupancy, layout: BoardLayout) -> &Occupancy {
    return &((*rboard).boards[layout as usize]);
}

struct Position {
    ctm: PieceColor, // color to move

}

type Diagonal = usize;
type Antidiagonal = usize;

pub(crate) fn select_color(board: &Occupancy, color: PieceColor) -> Bitboard {
    return board.boards[color as usize]
}

pub(crate) fn select_occupied(board: &Occupancy) -> Bitboard {
    return select_color(board, PieceColor::White) | select_color(board, PieceColor::Black)
}
pub(crate) fn is_occupied(board: &Occupancy, origin: usize) -> bool {
    return select_occupied(board) & bitboards::only(origin) > 0;
}

/// Describes a pair of locations on the board. Namely, `origin` and `destination`.
/// The actual format of each coordinate is dependent on the context.
/// In other words, the coordinates could be in any [BoardLayout].
pub(crate) struct Translation { data: u16 }

impl Translation {
    pub(crate) fn origin(self) -> usize { usize::from(self.data & 0b111111) }
    pub(crate) fn destination(self) -> usize { usize::from((self.data >> 6) & 0b111111) }
    pub(crate) fn new(origin: usize, destination: usize) -> Self {
        let mut data = 0u16;
        data |= (origin as u16 & 0b111111);
        data |= ((destination as u16 & 0b111111) << 6);
        return Translation { data }
    }
}


fn main() {
    println!("Hello, world!");
}
