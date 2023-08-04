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
use std::ops::{Add, Mul, Shl, ShlAssign};
use std::simd::{LaneCount, Simd, SimdInt, SimdOrd, SimdUint, SupportedLaneCount};
use std::u16;
use locate::{BoardLayout, RankwiseSquareOrdinal};

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

pub(crate) struct Board {
    color: [Bitboard; variant_count::<PieceColor>()],
    kind: [Bitboard; variant_count::<PieceKind>()]
}

pub(crate) struct RotatableBoard {
    board: [Board; variant_count::<BoardLayout>()]
}

pub(crate) fn rotate(rboard: &RotatableBoard, orientation: BoardLayout) -> &Board {
    let index = orientation as usize;
    return &(*rboard).board[index];
}

struct Position {
    ctm: PieceColor, // color to move

}

type Diagonal = usize;
type Antidiagonal = usize;

pub(crate) fn select_species(color: PieceColor, kind: PieceKind, board: &Board) -> Bitboard {
    return board.color[color as usize] & board.kind[kind as usize];
}

pub(crate) fn select_all(board: &Board) -> Bitboard {
    return board.color[PieceColor::White as usize] | board.color[PieceColor::Black as usize];
}

pub(crate) fn select_color(board: &Board, color: PieceColor) -> Bitboard { board.color[color as usize] }

pub(crate) fn is_occupied(board: &Board, origin: usize) -> bool {
    return select_all(board) & bitboards::only(origin) > 0;
}

pub(crate) fn enumerate_bitboard(mut board: Bitboard) -> Vec<u8> {
    let occupancy = u64::count_ones(board) as usize;
    let mut occupants: Vec<u8> = Vec::with_capacity(occupancy);

    loop {
        let next = u64::leading_zeros(board) as usize;
        if next == 64 { break; }
        board = bitboards::exclude(next, board);
        occupants.push(next as u8);
    }

    return occupants;
}

pub(crate) fn enumerate_bitlane(mut bitlane: Bitlane) -> Vec<u8> {
    let occupancy = u8::count_ones(bitlane) as usize;
    let mut occupants: Vec<u8> = Vec::with_capacity(occupancy);

    loop {
        let next = u8::leading_zeros(bitlane) as usize;
        if next == 64 { break; }
        bitlane = bitlanes::exclude(next, bitlane);
        occupants.push(next as u8);
    }

    return occupants;
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
