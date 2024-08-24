use macroquad::prelude::*;

#[macroquad::main("Sudoku")]
async fn main() {
    let block_size: f32 = 50.0;

    let mut row = None;
    let mut col = None;

    let mut board: [[Option<u8>; 9]; 9] = [[None; 9]; 9];

    loop {
        // clear background
        clear_background(WHITE);
        // draw board
        draw_board(&board, row, col, block_size);
        // draw buttons
        draw_buttons();
        // handle events
        handle_events(&mut board, &mut row, &mut col, block_size);
        // next frame
        next_frame().await;
    }
}

fn draw_board(board: &[[Option<u8>; 9]; 9], row: Option<i32>, col: Option<i32>, block_size: f32) {
    // horizontal lines
    let n: u8 = 10;
    for i in 0..n {
        let x1 = 50.;
        let x2 = 50. + (n - 1) as f32 * block_size;
        let y = 75. + i as f32 * block_size;

        if i % 3 == 0 {
            draw_line(x1, y, x2, y, 5.0, BLACK); // line width = 5.0
        } else {
            draw_line(x1, y, x2, y, 3., BLACK); // line width = 5.0
        }
    }
    // vertical lines
    for j in 0..n {
        let x = 50.0 + j as f32 * block_size;
        let y1 = 75.;
        let y2 = 75. + (n - 1) as f32 * block_size;

        if j % 3 == 0 {
            draw_line(x, y1, x, y2, 5.0, BLACK); // line width = 5.0
        } else {
            draw_line(x, y1, x, y2, 3.0, BLACK); // line width = 3.0
        }
    }
    // numbers
    for r in 0..9 {
        for c in 0..9 {
            let value = board[r][c];
            if let Some(value) = value {
                draw_text(
                    &value.to_string(),
                    50.0 + 15.0 + c as f32 * block_size,
                    75.0 + 37.0 + r as f32 * block_size,
                    50.0,
                    BLACK,
                );
            }
        }
    }
    // draw red lines around selected square
    if let (Some(row), Some(col)) = (row, col) {
        draw_rectangle_lines(
            50.0 + col as f32 * block_size,
            75.0 + row as f32 * block_size,
            block_size,
            block_size,
            10.0,
            RED,
        );
    }
}

fn clear_board(board: &mut [[Option<u8>; 9]; 9]) {
    *board = [[None; 9]; 9];
}

fn draw_buttons() {
    // top button
    draw_rectangle_lines(550.0, 100.0, 200.0, 100.0, 10.0, BLACK);
    draw_text("Solve", 595.0, 160.0, 50.0, BLACK);
    // middle button
    draw_rectangle_lines(550.0, 250.0, 200.0, 100.0, 10.0, BLACK);
    draw_text("Clear", 595.0, 310.0, 50.0, BLACK);
    // bottom button
    draw_rectangle_lines(550.0, 400.0, 200.0, 100.0, 10.0, BLACK);
    draw_text("Exit", 605.0, 460.0, 50.0, BLACK);
}

fn handle_events(
    board: &mut [[Option<u8>; 9]; 9],
    row: &mut Option<i32>,
    col: &mut Option<i32>,
    block_size: f32,
) {
    // LMB
    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_pos: (f32, f32) = mouse_position();

        // click on board
        if mouse_pos.0 > 50.0
            && mouse_pos.0 < 50.0 + 9.0 * block_size
            && mouse_pos.1 > 75.0
            && mouse_pos.1 < 75.0 + 9.0 * block_size
        {
            *col = Some(((mouse_pos.0 - 50.0) / block_size).floor() as i32);
            *row = Some(((mouse_pos.1 - 75.0) / block_size).floor() as i32);

            is_valid_entry(*board, row.unwrap() as usize, col.unwrap() as usize, 1);
        } else {
            // deselect grid block if not clicked on the board
            *row = None;
            *col = None;
        }

        // click on one of buttons
        if mouse_pos.0 > 550.0 && mouse_pos.0 < 750.0 {
            // solve
            if mouse_pos.1 > 100.0 && mouse_pos.1 < 200.0 {
                solve(board, 0, 0);
            }
            // clear
            else if mouse_pos.1 > 250.0 && mouse_pos.1 < 350.0 {
                clear_board(board);
            }
            // exit
            else if mouse_pos.1 > 400.0 && mouse_pos.1 < 500.0 {
                exit();
            }
        }
    }
    // escape
    if is_key_pressed(KeyCode::Escape) {
        exit();
    }
    // input mode
    if let Some(ref mut row) = row {
        if let Some(ref mut col) = col {
            // arrow keys
            if is_key_pressed(KeyCode::Up) && *row > 0 {
                *row -= 1;
            }
            if is_key_pressed(KeyCode::Down) && *row < 8 {
                *row += 1;
            }
            if is_key_pressed(KeyCode::Left) && *col > 0 {
                *col -= 1;
            }
            if is_key_pressed(KeyCode::Right) && *col < 8 {
                *col += 1;
            }

            // backspace/delete
            if is_key_pressed(KeyCode::Backspace) || is_key_pressed(KeyCode::Delete) {
                board[*row as usize][*col as usize] = None;
            }
            // numbers
            macro_rules! update_board_based_on_input {
                ($($keycode1: ident, $keycode2: ident)*) => {
                    let mut i = 0;
                    $(
                        i += 1;

                        if (is_key_pressed(KeyCode::$keycode1) || is_key_pressed(KeyCode::$keycode2)) && is_valid_entry(*board, *row as usize, *col as usize, i) {
                            board[*row as usize][*col as usize] = Some(i);
                        }
                    )*
                };
            }
            update_board_based_on_input!(Key1,Kp1  Key2,Kp2  Key3,Kp3 Key4,Kp4  Key5,Kp5 Key6,Kp6 Key7,Kp7 Key8,Kp8  Key9,Kp9);
        }
    }
}

fn is_valid_entry(board: [[Option<u8>; 9]; 9], row: usize, col: usize, num: u8) -> bool {
    // check row
    // get nums already in row
    let mut nums_in_row = Vec::new();
    for n in 0..9 {
        if let Some(iter_num) = board[row][n] {
            nums_in_row.push(iter_num);
        }
    }
    // check if num is in vec
    if nums_in_row.contains(&num) {
        //println!("num already in row");
        return false;
    }

    // check column
    // get nums already in column
    let mut nums_in_col: Vec<u8> = Vec::new();
    for n in 0..9 {
        if let Some(iter_num) = board[n][col] {
            nums_in_col.push(iter_num);
        }
    }
    // check if num is in vec
    if nums_in_col.contains(&num) {
        //println!("num already in col");
        return false;
    }

    // check box
    // get nums already in box
    let box_row = row / 3;
    let box_col = col / 3;
    let mut nums_in_box: Vec<u8> = Vec::new();
    for j in 0..3 {
        for i in 0..3 {
            if let Some(iter_num) = board[box_row * 3 + j][box_col * 3 + i] {
                nums_in_box.push(iter_num);
            }
        }
    }
    // check if num is in vec
    if nums_in_box.contains(&num) {
        //println!("num already in box");
        return false;
    }

    return true;
}

fn solve(board: &mut [[Option<u8>; 9]; 9], row: usize, col: usize) -> bool {
    // board is full
    if row == 9 {
        return true;
    }
    // column is full, go to next row
    else if col == 9 {
        return solve(board, row + 1, 0);
    }
    // if the square already has a number in it
    else if board[row][col] != None {
        return solve(board, row, col + 1);
    }
    // empty square on the board
    else {
        // try all numbers (1 - 9)
        for n in 1..10 {
            // check is number is valid
            if is_valid_entry(*board, row, col, n) {
                // make move
                board[row][col] = Some(n);
                // go to next column
                if solve(board, row, col + 1) {
                    return true;
                }
                // undo move if its wrong
                board[row][col] = None;
            }
        }
    }

    // the sudoku is unsolvable
    return false;
}

fn exit() {
    std::process::exit(0)
}
