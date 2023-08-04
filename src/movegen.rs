use std::simd::{LaneCount, SupportedLaneCount};
use crate::{Bitboard, bitboards, bitlanes, enumerate_bitboard, enumerate_bitlane, is_occupied, PieceColor, RotatableOccupancy, rotate, select_color, select_occupied, Translation};
use crate::locate::{BoardLayout, DiagonalSquareCoordinate, FilewiseSquareOrdinal, locate_ad, locate_d,
                    Rank, RankwiseSquareOrdinal, reverse_locate_ad, reverse_locate_d, split_rwc};
use crate::misc::measure_diagonal;
use crate::move_patterns::{instantiate_pattern, KING_PATTERN, KNIGHT_PATTERN, lookup_pawn_capture_pattern, Pattern};
use crate::obstruct::lookup_unobstructed_squares;

#[derive(Copy, Clone)]
pub(crate) struct MovingPiece {
    origin: RankwiseSquareOrdinal,
    color: PieceColor
}

/// Generates all pseudo-legal moves for a Bishop fixed at `mpiece.origin` of color `mpiece.color`.
/// A Bishop of that color need not actually exist. This function simply assumes one does.
/// In other words, the presence of such a Bishop at the given origin **is not** a precondition.
pub(crate) fn bishop(mpiece: MovingPiece, board: &RotatableOccupancy, moves: &mut Vec<Translation>) {
    // Diagonals
    {
        let diagonal_coordinate = locate_d(mpiece.origin);
        let diagonal_board = rotate(&board, BoardLayout::Diagonal);
        let diagonal_occupancy = bitlanes::slice_d(diagonal_coordinate.diagonal,
            select_occupied(diagonal_board));

        let diagonal_destinations = lookup_unobstructed_squares(diagonal_coordinate.offset,
            diagonal_occupancy);

        // Filter out extraneous squares.
        let diagonal_destinations = bitlanes::trim_to(diagonal_destinations,
            measure_diagonal(8, diagonal_coordinate.diagonal) as u8);

        // Filter out all squares where the occupant's color is equal to the color of the bishop.
        let diagonal_destinations = diagonal_destinations & !bitlanes::slice_d(
            diagonal_coordinate.diagonal, select_color(diagonal_board, mpiece.color));

        for dest_offset in enumerate_bitlane(diagonal_destinations) {
            let destination = reverse_locate_d(DiagonalSquareCoordinate {
                diagonal: diagonal_coordinate.diagonal,
                offset: usize::from(dest_offset),
            });
            let translation = Translation::new(mpiece.origin, destination);
            Vec::push(moves, translation)
        }
    }

    // Antidiagonals
    {
        let antidiagonal_coordinate = locate_ad(mpiece.origin);
        let antidiagonal_board = rotate(&board, BoardLayout::Antidiagonal);
        let antidiagonal_occupancy = bitlanes::slice_d(antidiagonal_coordinate.diagonal,
            select_occupied(antidiagonal_board));
        let antidiagonal_destinations = lookup_unobstructed_squares(
            antidiagonal_coordinate.offset, antidiagonal_occupancy);

        // Filter out extraneous squares.
        let antidiagonal_destinations = bitlanes::trim_to(antidiagonal_destinations,
            measure_diagonal(8, antidiagonal_coordinate.diagonal) as u8);

        // Filter out all squares where the occupant's color is equal to the color of the bishop.
        let antidiagonal_destinations = antidiagonal_destinations & !bitlanes::slice_d(
            antidiagonal_coordinate.diagonal, select_color(antidiagonal_board, mpiece.color));

        for dest_offset in enumerate_bitlane(antidiagonal_destinations) {
            let destination = reverse_locate_ad(DiagonalSquareCoordinate {
                diagonal: antidiagonal_coordinate.diagonal,
                offset: usize::from(dest_offset)
            });
            let translation = Translation::new(mpiece.origin, destination);
            Vec::push(moves, translation);
        }
    }
}

/// Generates all pseudo-legal moves for a Rook fixed at `mpiece.origin` of color `mpiece.color`.
/// A Rook of that color need not actually exist. This function simply assumes one does.
/// In other words, the presence of such a Rook at the given origin **is not** a precondition.
pub(crate) fn rook(mpiece: MovingPiece, board: &RotatableOccupancy, moves: &mut Vec<Translation>) {
    // Ranks
    {
        let (rank, file) = split_rwc(mpiece.origin);
        let rank_occupancy = bitlanes::slice(rank * 8, select_occupied(rotate(board, BoardLayout::Rankwise)));
        let destinations = lookup_unobstructed_squares(file, rank_occupancy);
        let destinations = destinations & !bitlanes::slice(rank * 8,
            !select_color(rotate(board, BoardLayout::Rankwise), mpiece.color));
        for dest_file in enumerate_bitlane(destinations) {
            let destination: RankwiseSquareOrdinal = rank * 8 + usize::from(dest_file);
            let translation = Translation::new(mpiece.origin, destination);
            Vec::push(moves, translation);
        }
    }

    // Files
    {
        let (rank, file) = split_rwc(mpiece.origin);
        let file_occupancy = bitlanes::slice(file * 8, select_occupied(rotate(board, BoardLayout::Filewise)));
        let destinations = lookup_unobstructed_squares(rank, file_occupancy);
        let destinations = destinations & !bitlanes::slice(file,
            select_color(rotate(board, BoardLayout::Filewise), mpiece.color));
        for dest_rank in enumerate_bitlane(destinations) {
            let destination: FilewiseSquareOrdinal = usize::from(dest_rank) * 8 + file;
            let translation = Translation::new(mpiece.origin, destination);
            Vec::push(moves, translation);
        }
    }
}

pub(crate) fn queen(mpiece: MovingPiece, board: &RotatableOccupancy, moves: &mut Vec<Translation>) {
    rook(mpiece, board, moves);
    bishop(mpiece, board, moves);
}

fn pattern<const N: usize>(mpiece: MovingPiece, board: &RotatableOccupancy, moves: &mut Vec<Translation>,
                           pattern: &Pattern<N>) where LaneCount<N>: SupportedLaneCount {
    let destinations = instantiate_pattern(mpiece.origin, pattern)
        & !select_color(rotate(board, BoardLayout::Rankwise), mpiece.color);

    for destination in enumerate_bitboard(destinations) {
        let translation = Translation::new(mpiece.origin, usize::from(destination));
        Vec::push(moves, translation);
    }
}

pub(crate) fn knight(mpiece: MovingPiece, board: &RotatableOccupancy, moves: &mut Vec<Translation>) {
    pattern(mpiece, board, moves, &KNIGHT_PATTERN);
}


pub(crate) fn king(mpiece: MovingPiece, board: &RotatableOccupancy, moves: &mut Vec<Translation>) {
    pattern(mpiece, board, moves, &KING_PATTERN);
}

pub(crate) fn pawn_step(mpiece: MovingPiece, board: &RotatableOccupancy, moves: &mut Vec<Translation>) {
    let (origin_rank, origin_file) = split_rwc(mpiece.origin);
    let destination_rank: Rank = (origin_rank as i8 + lookup_pawn_direction(mpiece.color)) as Rank;
    let destination: RankwiseSquareOrdinal = 8 * destination_rank + origin_file;
    let bb = bitboards::only(destination) & !select_occupied(rotate(board, BoardLayout::Rankwise));
    if bb > 0 { Vec::push(moves, Translation::new(mpiece.origin, destination)); }
}


fn lookup_pawn_direction(color: PieceColor) -> i8 {
    const DIRECTION: [i8; 2] = [-1, 1];
    return DIRECTION[color as usize];
}

fn lookup_pawn_birth_rank(color: PieceColor) -> i8 {
    const BIRTH_RANK: [i8; 2] = [6, 1];
    return BIRTH_RANK[color as usize];
}

pub(crate) fn pawn_2step(mpiece: MovingPiece, board: &RotatableOccupancy, moves: &mut Vec<Translation>) {
    let rw_board = rotate(board, BoardLayout::Rankwise);
    let direction = lookup_pawn_direction(mpiece.color);
    let birth_rank = lookup_pawn_birth_rank(mpiece.color);
    let lava_rank: Rank = Rank::try_from(birth_rank + direction).unwrap();
    let destination_rank: Rank = Rank::try_from(birth_rank + (2 * direction)).unwrap();

    let (origin_rank, origin_file) = split_rwc(mpiece.origin);
    let destination: RankwiseSquareOrdinal = destination_rank * 8 + origin_file;
    if origin_rank != Rank::try_from(birth_rank).unwrap() { return; }

    // Ensure lava square isn't blocked.
    if is_occupied(rw_board, lava_rank * 8 + origin_file) { return; }
    // Ensure destination square isn't blocked.
    if is_occupied(rw_board, destination) { return; }

    Vec::push(moves, Translation::new(mpiece.origin, destination))
}

pub(crate) fn pawn_capture(mpiece: MovingPiece, board: &RotatableOccupancy, moves: &mut Vec<Translation>) {
    let rw_board = rotate(board, BoardLayout::Rankwise);

    let bb: Bitboard =
        instantiate_pattern(mpiece.origin, lookup_pawn_capture_pattern(mpiece.color))
        & !select_color(rw_board, mpiece.color)
        & select_occupied(rw_board);

    for destination in enumerate_bitboard(bb) {
        let translation = Translation::new(mpiece.origin, usize::from(destination));
        Vec::push(moves, translation);
    }
}

// TODO: Enpassant
// TODO: Castling