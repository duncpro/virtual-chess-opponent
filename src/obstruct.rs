use crate::Bitlane;
use crate::bitlanes;

/// Computes a [Bitlane] where all reachable destinations from the given `origin` are marked.
/// A square is reachable if it is not obstructed by a piece on a square between the destination and
/// the origin.
const fn search_unobstructed(origin: usize, occupancy: Bitlane) -> Bitlane {
    let mut destinations = bitlanes::EMPTY;

    let mut check_i: usize = origin;
    loop {
        if check_i >= 7 { break; }
        check_i += 1;
        let check_bl = bitlanes::only(check_i);
        destinations |= check_bl;
        let occupied = (check_bl & occupancy) > 0;
        if occupied { break; }
    }

    let mut check_i: usize = origin;
    loop {
        if check_i == 0 { break; }
        check_i -= 1;
        let check_bl = bitlanes::only(check_i);
        destinations |= check_bl;
        let occupied = (check_bl & occupancy) > 0;
        if occupied { break; }
    }

    return destinations;
}

const OBSTRUCTION_LOOKUP_TABLE_SIZE: usize = 256 /* 2^8 */ * 8 /* origins */;

const fn compile_obstruction_lookup_table() -> [u8; OBSTRUCTION_LOOKUP_TABLE_SIZE] {
    let mut table: [u8; OBSTRUCTION_LOOKUP_TABLE_SIZE] = [0; OBSTRUCTION_LOOKUP_TABLE_SIZE];
    let mut i = 0;
    loop {
        if i >= OBSTRUCTION_LOOKUP_TABLE_SIZE { break; }
        let origin = i / 256;
        let occupancy = (i % 256) as u8;
        table[i] = search_unobstructed(origin, occupancy);
        i += 1;
    }
    return table;
}

const OBSTRUCTION_LOOKUP_TABLE: [u8; OBSTRUCTION_LOOKUP_TABLE_SIZE] =
    compile_obstruction_lookup_table();

/// Produces a [Bitlane] where each marked square indicates an unobstructed square.
/// That is, a square which a sliding piece can reach, assuming the piece on the square
/// is of the opponent's color.
pub(crate) fn lookup_unobstructed_squares(origin: usize, occupancy: Bitlane) -> Bitlane {
    let key = origin * 256 + (occupancy as usize);
    return OBSTRUCTION_LOOKUP_TABLE[key];
}