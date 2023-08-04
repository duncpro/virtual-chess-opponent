use seq_macro::seq;
use crate::Bitboard;
use crate::Bitlane;
use crate::misc::measure_diagonal;
use crate::misc::partial_diamond_figurate;

pub(crate) const EMPTY: Bitlane = 0;

pub(crate) const fn only(ordinal: usize) -> Bitlane {
    if ordinal > 7 {
        panic!("Bitlane has width of 8 bits. Ordinal fails bounds check.");
    }
    1 << (ordinal as u8)
}

pub(crate) const OPAQUE: Bitlane = 0b11111111;

pub(crate) fn except(ordinal: usize) -> Bitlane { return !only(ordinal); }


pub(crate) fn exclude(ordinal: usize, template: Bitlane) -> Bitlane {
    return template & except(ordinal);
}

/// Produces a [Bitlane] beginning at `ordinal`.
pub(crate) fn slice(ordinal: usize, board: Bitboard) -> Bitlane {
    return (board >> ordinal) as Bitlane;
}

pub(crate) fn trim_to(bitlane: Bitlane, length: u8) -> Bitlane {
    return (OPAQUE >> (8 - length)) & bitlane;
}


/// Produces a [Bitlane] encapsulating an entire diagonal.
/// The returned lane is right-aligned, such that the rightmost bit represents the first square on
/// the diagonal. Any extraneous bits/squares are unmarked.
pub(crate) fn slice_d(dordinal: usize, bitboard: Bitboard) -> Bitlane {
    if dordinal > 14 { panic!("Expected dordinal in rage 0 <= dordinal <= 14."); }
    let base = partial_diamond_figurate(8, dordinal);
    let lane = slice(base, bitboard);
    let length = u8::try_from(measure_diagonal(8, dordinal)).unwrap();
    let trimmed = trim_to(lane, length);
    return trimmed;
}

pub(crate) fn scan(bitlane: Bitlane, f: impl FnMut(u32)) {
    seq!(i in 0..=8 {
        match bitlane.count_ones() {
            #(i => { scan_n::<i>(bitlane, f); },)*
            _ => {}
        }
    })
}

fn scan_n<const N: u32>(mut bitlane: Bitlane, mut f: impl FnMut(u32)) {
    for _ in 0..N {
        let i = Bitlane::leading_zeros(bitlane);
        bitlane = exclude(i as usize, bitlane);
        f(i);
    }
}