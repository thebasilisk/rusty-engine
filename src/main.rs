use std::{i32, ops::AddAssign, str::FromStr, time::Instant};

use chess::{BitBoard, Board, BoardStatus, CacheTable, ChessMove, Color, MoveGen, Piece, EMPTY};




// fn count_material(board : Board, color : Color) -> i32 {
//     let mut total = 0;

//     let pieces = board.color_combined(color);
//     let pawns = board.pieces(Piece::Pawn);
//     let knights = board.pieces(Piece::Knight);
//     let bishops = board.pieces(Piece::Bishop);
//     let rooks = board.pieces(Piece::Rook);
//     let queens = board.pieces(Piece::Queen);

//     total += (pieces & pawns).popcnt() * 1;
//     total += (pieces & knights).popcnt() * 3;
//     total += (pieces & bishops).popcnt() * 3;
//     total += (pieces & rooks).popcnt() * 5;
//     total += (pieces & queens).popcnt() * 8;
    
//     return total as i32
// }

const BLACK_PAWN_TABLE : [i8; 64] = [
0, 0, 0, 0, 0, 0, 0, 0,
50, 50, 50, 50, 50, 50, 50, 50,
15, 15, 20, 25, 25, 20, 15, 15,
5, 5, 5, 15, 15, 5, 5, 5,
0, 0, 0, 20, 20, 0, 0, 0,
5, 5, 5, 10, 10, 5, 5, 5,
5, 5, 10, -5, -5, 10, 5, 5,
0, 0, 0, 0, 0, 0, 0, 0,
];
const WHITE_PAWN_TABLE : [i8; 64] = [
0, 0, 0, 0, 0, 0, 0, 0,
5, 5, 10, -5, -5, 10, 5, 5,
5, 5, 5, 10, 10, 5, 5, 5,
0, 0, 0, 20, 20, 0, 0, 0,
5, 5, 5, 15, 15, 5, 5, 5,
15, 15, 20, 25, 25, 20, 15, 15,
50, 50, 50, 50, 50, 50, 50, 50,
0, 0, 0, 0, 0, 0, 0, 0,
];

const BLACK_KNIGHT_TABLE : [i8; 64] = [
-25, -20, -15, -10, -10, -15, -20, -25,
-20, -5, 0, 0, 0, 0, -5, -20,
-15, 0, 5, 10, 10, 5, 0, -15,
-10, 0, 0, 20, 20, 0, 0, -10,
-10, 0, 5, 20, 20, 5, 5, -10,
-15, 5, 15, 10, 10, 15, 5, -15,
-20, -5, 0, 5, 5, 0, -5, -20,
-25, -5, 0, 0, 0, 0, -5, -25,
];
const WHITE_KNIGHT_TABLE : [i8; 64] = [
-25, -5, 0, 0, 0, 0, -5, -25,
-20, -5, 0, 5, 5, 0, -5, -20,
-15, 5, 15, 10, 10, 15, 5, -15,
-10, 0, 5, 20, 20, 5, 5, -10,
-10, 0, 0, 20, 20, 0, 0, -10,
-15, 0, 5, 10, 10, 5, 0, -15,
-20, -5, 0, 0, 0, 0, -5, -20,
-25, -20, -15, -10, -10, -15, -20, -25,
];
const BLACK_BISHOP_TABLE : [i8; 64] = [
0, 0, 0, 0, 0, 0, 0, 0,
0, 0, 0, 0, 0, 0, 0, 0,
-10, 0, -5, 0, 0, -5, 0, -10,
-5, 10, 5, 10, 10, 5, 10, -5,
-5, 0, 15, 10, 10, 15, 0, -5,
0, 5, 5, 10, 10, 5, 5, 0,
5, 10, 0, 5, 5, 0, 10, 5,
0, -5, -10, 0, 0, -10, -5, 0,
];
const WHITE_BISHOP_TABLE : [i8; 64] = [
0, -5, -10, 0, 0, -10, -5, 0,
5, 10, 0, 5, 5, 0, 10, 5,
0, 5, 5, 10, 10, 5, 5, 0,
-5, 0, 15, 10, 10, 15, 0, -5,
-5, 10, 5, 10, 10, 5, 10, -5,
-10, 0, -5, 0, 0, -5, 0, -10,
0, 0, 0, 0, 0, 0, 0, 0,
0, 0, 0, 0, 0, 0, 0, 0,
];
// fn get_pawn_evals(pawns : BitBoard, color : Color) -> f32 {
//     let mut eval = 0.0f32;
//     match color {
//         Color::White => for square in pawns {
//             eval += match square.get_rank() {
//                 chess::Rank::First => 1.0,
//                 chess::Rank::Second => 1.0,
//                 chess::Rank::Third => 1.1,
//                 chess::Rank::Fourth => 1.2,
//                 chess::Rank::Fifth => 1.2,
//                 chess::Rank::Sixth => 1.1,
//                 chess::Rank::Seventh => 1.0,
//                 chess::Rank::Eighth => 1.0,
//             } * match square.get_file() {
//                 chess::File::A => 0.80,
//                 chess::File::B => 0.85,
//                 chess::File::C => 0.90,
//                 chess::File::D => 0.95,
//                 chess::File::E => 0.95,
//                 chess::File::F => 0.90,
//                 chess::File::G => 0.85,
//                 chess::File::H => 0.80,
//             }
//         },
//         Color::Black => for square in pawns {
//             eval += match square.get_rank() {
//                 chess::Rank::First => 1.0,
//                 chess::Rank::Second => 1.0,
//                 chess::Rank::Third => 1.1,
//                 chess::Rank::Fourth => 1.2,
//                 chess::Rank::Fifth => 1.2,
//                 chess::Rank::Sixth => 1.1,
//                 chess::Rank::Seventh => 1.0,
//                 chess::Rank::Eighth => 1.0,
//             } * match square.get_file() {
//                 chess::File::A => 0.80,
//                 chess::File::B => 0.85,
//                 chess::File::C => 0.90,
//                 chess::File::D => 0.95,
//                 chess::File::E => 0.95,
//                 chess::File::F => 0.90,
//                 chess::File::G => 0.85,
//                 chess::File::H => 0.80,
//             }
//         },
//     }
//     return eval
// }
fn eval(board : Board) -> i32 {
    // let legal_moves = MoveGen::new_legal(&board);
    // let move_count = legal_moves.len();
    let mut eval= 0i32;

    let white_pieces = board.color_combined(Color::White);
    let white_pawns = board.pieces(Piece::Pawn) & white_pieces;
    let white_knights = board.pieces(Piece::Knight) & white_pieces;
    let white_bishops = board.pieces(Piece::Bishop) & white_pieces;
    let white_rooks = board.pieces(Piece::Rook) & white_pieces;
    let white_queens = board.pieces(Piece::Queen) & white_pieces;

    // eval += get_pawn_evals(white_pawns, Color::White);
    // eval += get_pawn_evals(white_knights, Color::White) * 3.0;
    // eval += get_pawn_evals(white_bishops, Color::White) * 3.0;
    // eval += get_pawn_evals(white_rooks, Color::White) * 5.0;
    // eval += get_pawn_evals(white_queens, Color::White) * 8.0;

    for square in white_pawns {
        eval += 100 + WHITE_PAWN_TABLE[square.to_index()] as i32;
    }
    for square in white_knights {
        eval += 300 + WHITE_KNIGHT_TABLE[square.to_index()] as i32;
    }
    for square in white_bishops {
        eval += 300 + WHITE_BISHOP_TABLE[square.to_index()] as i32;
    }
    eval += white_rooks.popcnt() as i32 * 500;
    eval += white_queens.popcnt() as i32 * 800;

    let black_pieces = board.color_combined(Color::Black);
    let black_pawns = board.pieces(Piece::Pawn) & black_pieces;
    let black_knights = board.pieces(Piece::Knight) & black_pieces;
    let black_bishops = board.pieces(Piece::Bishop) & black_pieces;
    let black_rooks = board.pieces(Piece::Rook) & black_pieces;
    let black_queens = board.pieces(Piece::Queen) & black_pieces;

    // eval -= get_pawn_evals(black_pawns, Color::Black);
    // eval -= get_pawn_evals(black_knights, Color::Black) * 3.0;
    // eval -= get_pawn_evals(black_bishops, Color::Black) * 3.0;
    // eval -= get_pawn_evals(black_rooks, Color::Black) * 5.0;
    // eval -= get_pawn_evals(black_queens, Color::Black) * 8.0;

    for square in black_pawns {
        eval -= 100 + BLACK_PAWN_TABLE[square.to_index()] as i32;
    }
    for square in black_knights {
        eval -= 300 + BLACK_KNIGHT_TABLE[square.to_index()] as i32;
    }
    for square in black_bishops {
        eval -= 300 + BLACK_BISHOP_TABLE[square.to_index()] as i32;
    }
    eval -= black_rooks.popcnt() as i32 * 500;
    eval -= black_queens.popcnt() as i32 * 800;

    // let white_material = count_material(board, Color::White);
    // let black_material = count_material(board, Color::Black);

    // let material_difference = white_material - black_material;
    // let eval = material_difference;

    return eval
}

fn minimax(board : Board, depth : i8, alpha : Option<i32>, beta : Option<i32>, prev_best : ChessMove, color : Color, time : Instant, target: u128, capture : bool, cache : &mut CacheTable<(ChessMove, i32, i8)>, i : &mut u32) -> (ChessMove, i32) {
    
    // let bs = board.to_string();
    // let debug = bs.as_str();
    // if depth == 0 {
    //     println!("{debug}");
    // }
    if let Some((c_mv, c_eval, c_depth)) = cache.get(board.get_hash()) {
        // i.add_assign(1);
        // let new_depth_eval = negamax(board.make_move_new(mv), depth - 1, alpha, beta, ChessMove::default(), color, time, target, false, cache, i);
        if c_depth == depth {
            return (ChessMove::default(), c_eval);
        }
    }
    if board.status() == BoardStatus::Checkmate {
        // println!("{}", board.to_string());
        let sign = match color {
            Color::White => 1,
            Color::Black => -1,
        };
        match board.side_to_move() {
            Color::White => return (ChessMove::default(), sign * -999),
            Color::Black => return (ChessMove::default(), sign * 999),
        }
    }
    if board.status() == BoardStatus::Stalemate {
        return (ChessMove::default(), 0)
    }
    if (depth <= 0 && !capture) || time.elapsed().as_millis() > 1000000 {
        match color {
            Color::White => return (ChessMove::default(), eval(board)),
            Color::Black => return (ChessMove::default(), -eval(board)),
        }
    }

    let mut moves = MoveGen::new_legal(&board);

    match board.side_to_move() {
        Color::White => {
            moves.set_iterator_mask(*board.color_combined(Color::Black));
        },
        Color::Black => {
            moves.set_iterator_mask(*board.color_combined(Color::White));
        }
    }

    let mut maxxed = -999;
    let mut candidate = ChessMove::default();
    let mut minned;
    let mut best_response = ChessMove::default();
    let mut response;
    for mv in moves.by_ref() {
        // println!("{}", mv.to_string());
        i.add_assign(1);
        let res_board = board.make_move_new(mv);
        (response, minned) = negamax(res_board, depth - 1, Some(maxxed), beta, best_response, color, time, target, true, cache, i);
        if minned > maxxed {
            maxxed = minned;
            candidate = mv;
            best_response = response;
            if beta.is_some_and(|val| maxxed > val) {
                // print!("A");
                return (best_response, maxxed)
            }
        }
    }
    if depth > 0 {
        moves.set_iterator_mask(!EMPTY);
    }
    let mut rem  = moves.collect::<Vec<ChessMove>>();
    if rem.contains(&prev_best) {
        rem.push(prev_best);
    }
    rem.reverse();
    best_response = ChessMove::default();
    for mv in rem {
        // println!("{}", mv.to_string());
        i.add_assign(1);
        (response, minned) = negamax(board.make_move_new(mv), depth - 1, Some(maxxed), beta, best_response, color, time, target, false, cache, i);
        if minned > maxxed {
            maxxed = minned;
            candidate = mv;
            best_response = response;
            if beta.is_some_and(|val| maxxed > val) {
                // print!("A");
                return (best_response, maxxed)
            }
        }
    }
    cache.add(board.get_hash(), (candidate, maxxed, depth));
    // let s = debug.as_ptr();
    return (candidate, maxxed)
}

fn negamax(board : Board, depth : i8, alpha : Option<i32>, beta : Option<i32>, prev_best : ChessMove, color : Color, time : Instant, target: u128, capture : bool, cache : &mut CacheTable<(ChessMove, i32, i8)>, i : &mut u32) -> (ChessMove, i32) {
    // let bs = board.to_string();
    // let debug = bs.as_str();
    if let Some((c_mv, c_eval, c_depth)) = cache.get(board.get_hash()) {
        // i.add_assign(1);
        if c_depth == depth {
            return (ChessMove::default(), c_eval);
        }
    }
    if board.status() == BoardStatus::Checkmate {
        // println!("{}", board.to_string());
        let sign = match color {
            Color::White => 1,
            Color::Black => -1,
        };
        match board.side_to_move() {
            Color::White => return (ChessMove::default(), sign * -999),
            Color::Black => return (ChessMove::default(), sign * 999),
        }
    }
    if board.status() == BoardStatus::Stalemate {
        return (ChessMove::default(), 0)
    }
    if (depth <= 0 && !capture) || time.elapsed().as_millis() > target {
        match color {
            Color::White => return (ChessMove::default(), eval(board)),
            Color::Black => return (ChessMove::default(), -eval(board)),
        }
    }
    if depth <= 0 {
        // println!("{}", board.to_string());
    }

    let mut moves = MoveGen::new_legal(&board);

    match board.side_to_move() {
        Color::White => {
            moves.set_iterator_mask(*board.color_combined(Color::Black));
        },
        Color::Black => {
            moves.set_iterator_mask(*board.color_combined(Color::White));
        }
    }
    
    let mut minned = 999;
    let mut candidate = ChessMove::default();
    let mut maxxed;
    let mut best_response = ChessMove::default();
    let mut response;

    for mv in moves.by_ref() {
        i.add_assign(1);
        // println!("{}", mv.to_string());
        (response, maxxed) = minimax(board.make_move_new(mv), depth - 1, alpha, Some(minned), best_response, color, time, target, true, cache, i);
        if maxxed < minned {
            minned = maxxed;
            candidate = mv;
            best_response = response;
            // if minned <= -500.0 {
            //     println!("{}", mv.to_string());
            // }
            if alpha.is_some_and(|val| minned < val) {
                // print!("B");
                return (candidate, minned)
            }
        }
    }
    if depth > 0 {
        moves.set_iterator_mask(!EMPTY);
    }
    let mut rem  = moves.collect::<Vec<ChessMove>>();
    if rem.contains(&prev_best) {
        rem.push(prev_best);
    }
    rem.reverse();
    best_response = ChessMove::default();
    for mv in rem {
        i.add_assign(1);
        // println!("{}", mv.to_string());
        (response, maxxed) = minimax(board.make_move_new(mv), depth - 1, alpha, Some(minned), best_response, color, time, target, false, cache, i);
        if maxxed < minned {
            minned = maxxed;
            candidate = mv;
            best_response = response;
            // if minned > 3.4 && minned < 3.5 {
            //     println!("{}, {}", mv.to_string(), board.to_string());
            // }
            if alpha.is_some_and(|val| minned < val) {
                // print!("B");
                return (candidate, minned)
            }
        }
    }
    cache.add(board.get_hash(), (candidate, minned, depth));
    // let s = debug.as_ptr();
    return (candidate, minned);
}

// fn recurse (depth : i8, i : &mut u32, prev : u32) -> u32 {
//     i.add_assign(1);
//     if depth == 0 {
//         return prev + 1
//     }
//     return ercuse(depth - 1, i, prev)
// }

// fn ercuse (depth : i8, i : &mut u32, prev : u32) -> u32 {
//     i.add_assign(1);
//     if depth == 0 {
//         return prev + 1
//     }
//     return recurse(depth - 1, i, prev)
// }

fn iter_deepen (board : Board, target: u128) -> ChessMove {
    let now = Instant::now();
    let mut table = CacheTable::new(65536 * 2 * 2 * 2, (ChessMove::default(), 0i32, 0i8));
    let mut i = 0;
    let mut depth = 0;
    
    let mut eval;
    let mut prev_best = ChessMove::default();
    let mut candidate = prev_best;
    loop {
        depth += 1;
        (prev_best, eval) = minimax(board, depth, Some(i32::MIN), Some(i32::MAX), prev_best, board.side_to_move(), now, target, false, &mut table, &mut i);
        // println!("next: {next}");
        if now.elapsed().as_millis() > target {
            println!("depth: {depth}");
            break;
        }
        candidate = prev_best;
    }
    // while let (mv, temp) = minimax(board, depth, Some(f32::NEG_INFINITY), Some(f32::INFINITY), prev_best, board.side_to_move(), now, target, false, &mut table, &mut i) {
    //     println!("depth: {depth}");
    //     if now.elapsed().as_millis() > target {
    //         break;
    //     }
    //     prev_best = mv;
    //     eval = temp;
    //     depth += 1;
    // }
    println!("eval: {}", eval);
    println!("i: {i}");
    return candidate
}

// fn best_first (board : Board, depth1 : i8, depth2 : i8, color : Color, cache : &mut CacheTable<f32>) -> ChessMove {
//     let mut moves = MoveGen::new_legal(&board).collect::<Vec<ChessMove>>();
//     moves.reverse();
//     let mut ordered_moves: Vec<(ChessMove, ChessMove, f32)> = Vec::with_capacity(moves.len());
//     let mut i = 0;
//     let mut j = 0;
//     let mut prev_best = ChessMove::from_str("a2a3").unwrap();
//     for mv in moves {
//         i +=1;
//         j += 1;
//         println!("Checking move {j}");
//         println!("{}", mv.to_string());
//         let result_board = board.make_move_new(mv);
//         let mut responses = MoveGen::new_legal(&result_board).collect::<Vec<ChessMove>>();
//         responses.push(prev_best);
//         responses.reverse();

//         if !result_board.legal(responses[0]) {
//             responses.remove(0);
//         }
//         let mut mini = 999.0f32;
//         let mut best_response = ChessMove::default();
//         for response in responses {
//             i += 1;
//             // println!("so far: {i}");
//             let now = Instant::now();
//             let eval = minimax(result_board.make_move_new(response), depth1, Some(mini), !color, now, false, cache, &mut i);
//             if eval < mini {
//                 mini = eval;
//                 best_response = response;
//                 // println!("New best response: {}", response.to_string());
//                 println!("New Eval: {mini}");
//             }
//         }
//         ordered_moves.push((mv, best_response, mini));
//         prev_best = best_response;
//     }

//     ordered_moves.sort_by(|a,b| b.2.total_cmp(&a.2));
//     ordered_moves.reverse();
//     let mut best_eval = ordered_moves[0].2;
//     println!("{best_eval}");
//     let mut candidate = ChessMove::default();
//     for n in 0..ordered_moves.len() {
//         let mv = ordered_moves[n];
//         let now = Instant::now();
//         let eval = minimax(board.make_move_new(mv.0).make_move_new(mv.1), depth2, Some(best_eval), color, now, false, cache, &mut i);
//         if eval < ordered_moves[n + 1].2 {
//             continue;
//         } else {
//             best_eval = eval;
//             candidate = mv.0;
//             break;
//         }
//     }

//     println!("Nodes searched: {i}");
//     println!("Best Eval: {best_eval}");
//     return candidate
// }

fn main() {
    let board = Board::default();
    // let move_gen = MoveGen::new_legal(&board);
    let depth = 3;
    
    let mut table = CacheTable::new(4194304 * 2 * 2, (ChessMove::default(), 0i32, 0i8));
    let total = Instant::now();
    // // println!("{}", candidate.to_string());
    // let mut candidates : Vec<(ChessMove, f32)> = Vec::with_capacity(move_gen.len());
    // let mut candidate = move_gen.next().unwrap();
    let now = Instant::now();
    let target = 3000;
    let mut i = 0;
    // let mut mini_eval = minimax(board.make_move_new(candidate), depth, None, oppose_color, now, false, &mut table);
    // for mv in move_gen {
    //     now = Instant::now();
    //     let new_board = board.make_move_new(mv);
    //     let eval = minimax(new_board, depth, None, oppose_color, now, false, &mut table);
    //     candidates.push((mv, eval));
    // }
    // candidates.sort_by(|a, b| a.1.total_cmp(&b.1));

    // let mut best_moves = candidates.iter();
    // let (mut candidate, mut mini_eval) = best_moves.next().unwrap();
    
    // for (mv, _) in best_moves {
    //     now = Instant::now();
    //     let new_board = board.make_move_new(*mv);
    //     let oppose_mini = 999.0;
    //     let oppose_moves = MoveGen::new_legal(&new_board);
    //     for mv2 in oppose_moves {
    //         let oppose_eval = minimax(new_board, depth, Some(mini_eval), !oppose_color, now, false, &mut table);
    //         if oppose_eval < oppose_mini
    //     }
    //     let eval = minimax(new_board, depth, Some(mini_eval), !oppose_color, now, false, &mut table);

    //     if eval < mini_eval {
    //         mini_eval = eval;
    //         candidate = *mv;
    //     }
    // }
    // println!("mini: {}", mini_eval);
    // println!("{}", candidate.to_string());
    let test_board = board.make_move_new(ChessMove::from_str("d2d4").unwrap()).make_move_new(ChessMove::from_str("g8f6").unwrap()).make_move_new(ChessMove::from_str("d4d5").unwrap()).make_move_new(ChessMove::from_str("d7d6").unwrap());
    // let test_board = Board::from_str("rnb1k2r/ppp2ppp/4qn2/2b1p3/3P4/2N2N2/PPP1BPPP/R1BQ1RK1 b kq - 0 7").unwrap();
    // println!("{}", minimax(board.make_move_new(ChessMove::from_str("d2d4").unwrap()).make_move_new(ChessMove::from_str("g8f6").unwrap()).make_move_new(ChessMove::from_str("d4d5").unwrap()).make_move_new(ChessMove::from_str("d7d6").unwrap()), depth, None, Color::White, now, false, &mut table, &mut i));
    // let test_board = board.make_move_new(ChessMove::from_str("d2d4").unwrap());
    // println!("{}", minimax(board.make_move_new(ChessMove::from_str("e6f5").unwrap()).make_move_new(ChessMove::from_str("g2g4").unwrap()), 5, -1000.0, Color::Black));

    // println!("{}", MoveGen::new_legal(&test_board).len());
    // println!("{}", minimax(test_board.make_move_new(ChessMove::from_str("d1d4").unwrap()), 1, None, Color::Black, now, false, &mut table, &mut i));
    println!("{}", minimax(test_board, depth, Some(i32::MIN), Some(i32::MAX), ChessMove::default(), Color::White, now, target, false, &mut table, &mut i).1);
    // println!("{}", best_first(test_board, 1, 2, Color::White, &mut table));
    // for i in 0.. 35989401 {
    //     board.legal(mv);
    // }
    println!("{}", i);
    
    // println!("{}", now.elapsed().as_millis());
    println!("{}", total.elapsed().as_millis());

    let now = Instant::now();
    println!("{}", iter_deepen(test_board, target));

    // let now = Instant::now();
    // let mut table = CacheTable::new(65536 * 2 * 2 * 2, 0.0f32);
    // let mut i = 0;
    // let mut depth = 0;

    // let mut eval = 0.0;
    // let mut prev_best = ChessMove::from_str("f6d5").unwrap();
    // loop {
    //     depth += 1;
    //     (prev_best, eval) = minimax(test_board, depth, Some(f32::NEG_INFINITY), Some(f32::INFINITY), prev_best, board.side_to_move(), now, target, false, &mut table, &mut i);
    //     // println!("eval: {eval}");
    //     if now.elapsed().as_millis() > target || depth <= 0 {
    //         println!("depth: {depth}");
    //         break;
    //     }
    // }

    // println!("{}", eval);
    println!("{}", now.elapsed().as_millis());
    // while board.status() == BoardStatus::Ongoing {
    //     let mut move_gen = MoveGen::new_legal(&board);
    //     // println!("#moves:{}", move_gen.len());
    //     if move_gen.len() == 0 {
    //         break;
    //     }
    //     if move_gen.len() == 1 {
    //         let random_move = move_gen.next();
    //         board = board.make_move_new(random_move.expect("Move broked"));
    //     } else {
    //         let random_move = move_gen.nth(thread_rng().gen_range(0..move_gen.len()-1));
    //         board = board.make_move_new(random_move.expect("Move broked"));
    //     }
    // }

    // println!("{}", mv_final.to_string());
}
