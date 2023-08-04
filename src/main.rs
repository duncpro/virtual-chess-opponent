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
use seq_macro::seq;
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

pub(crate) struct Occupancy {
    boards: [Bitboard; variant_count::<PieceColor>()]
}

pub(crate) struct RotatableOccupancy {
    boards: [Occupancy; variant_count::<BoardLayout>()]
}

pub(crate) fn rotate(rboard: &RotatableOccupancy, orientation: BoardLayout) -> &Occupancy {
    let index = orientation as usize;
    return &(*rboard).boards[index];
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

pub(crate) fn scan_bitlane(bitlane: Bitlane, f: impl FnMut(u32)) {
    seq!(i in 0..=8 {
        match bitlane.count_ones() {
            #(i => { scan_bitlane_n::<i>(bitlane, f); },)*
            _ => {}
        }
    })
}

fn scan_bitlane_n<const N: u32>(mut bitlane: Bitlane, mut f: impl FnMut(u32)) {
    for _ in 0..N {
        let i = Bitlane::leading_zeros(bitlane);
        bitlane = bitlanes::exclude(i as usize, bitlane);
        f(i);
    }
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
