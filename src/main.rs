use raylib::prelude::*;

const W: i32 = 630;
const H: i32 = 630;
const SQ_SIZE: i32 = W / 9;

struct Sudoku {
    board: Vec<Vec<u8>>,
    initial_board: Vec<Vec<u8>>,
    solved_board: Vec<Vec<u8>>,
    help_player: bool,
}

impl Sudoku {
    pub fn new(board: Vec<Vec<u8>>) -> Sudoku {
        let mut sudoku = Sudoku {
            initial_board: board.clone(),
            solved_board: vec![vec![0; 9]; 9],
            help_player: false,
            board,
        };
        sudoku.solve();
        sudoku.solved_board = sudoku.board.clone();
        sudoku.board = sudoku.initial_board.clone();
        return sudoku;
    }

    fn reset(&mut self) {
        self.board = self.initial_board.clone();
    }

    fn get_empty(&self) -> Option<(u8, u8)> {
        for (y, row) in self.board.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == 0 {
                    return Some((x.try_into().unwrap(), y.try_into().unwrap()));
                }
            }
        }
        None
    }

    fn is_valid(&self, num: u8, x: u8, y: u8) -> bool {
        for i in 0..9 {
            if self.board[y as usize][i] == num {
                // Horizontal check
                return false;
            }

            if self.board[i][x as usize] == num {
                // Vertical check
                return false;
            }
        }

        let x = x / 3 * 3;
        let y = y / 3 * 3;

        for j in y..y + 3 {
            for i in x..x + 3 {
                if self.board[j as usize][i as usize] == num {
                    return false;
                }
            }
        }

        true
    }

    fn solve(&mut self) -> bool {
        let empty_pos = self.get_empty();
        if empty_pos.is_none() {
            return true;
        }

        let (x, y) = empty_pos.unwrap();
        for n in 1..10 {
            if self.is_valid(n, x, y) {
                self.board[y as usize][x as usize] = n;
                if self.solve() {
                    return true;
                }
                self.board[y as usize][x as usize] = 0;
            }
        }
        return false;
    }

    fn set_cell(&mut self, num: u8, x: u8, y: u8) {
        if num > 9 {
            return;
        }

        if self.initial_board[y as usize][x as usize] == 0 {
            self.board[y as usize][x as usize] = num;
        }
    }

    fn draw_selected(&self, d: &mut RaylibDrawHandle, x: i32, y: i32) {
        d.draw_rectangle(
            x * SQ_SIZE,
            y * SQ_SIZE,
            SQ_SIZE,
            SQ_SIZE,
            Color::new(0xbb, 0xde, 0xfb, 0xff),
        );

        // for i in 0..9 {
        //     if i != y {
        //         d.draw_rectangle(
        //             x * SQ_SIZE,
        //             i * SQ_SIZE,
        //             SQ_SIZE,
        //             SQ_SIZE,
        //             Color::new(0xe2, 0xeb, 0xf3, 0xff),
        //         );
        //     }

        //     if i != x {
        //         d.draw_rectangle(
        //             i * SQ_SIZE,
        //             y * SQ_SIZE,
        //             SQ_SIZE,
        //             SQ_SIZE,
        //             Color::new(0xe2, 0xeb, 0xf3, 0xff),
        //         );
        //     }
        // }
    }

    fn draw(&self, d: &mut RaylibDrawHandle, selected: &mut Vec<u8>) {
        for y in 0..9 {
            for x in 0..9 {
                let cell = self.board[y][x];

                if x as u8 == selected[0] && y as u8 == selected[1] {
                    self.draw_selected(d, x as i32, y as i32);
                }

                if cell == 0 {
                    continue;
                }

                let text_length = d.measure_text(&cell.to_string(), SQ_SIZE);
                let text_color = if self.initial_board[y as usize][x as usize] == 0 {
                    if self.help_player && self.solved_board[y as usize][x as usize] != cell {
                        Color::new(0xff, 0x77, 0x77, 0xff)
                    } else {
                        Color::new(0x77, 0x77, 0x77, 0xff)
                    }
                } else {
                    Color::BLACK
                };
                d.draw_text(
                    &cell.to_string(),
                    x as i32 * SQ_SIZE + text_length / 2,
                    y as i32 * SQ_SIZE,
                    SQ_SIZE,
                    text_color,
                );
            }
        }

        for i in 0..10 {
            let t = if i % 3 == 0 { 5.0 } else { 2.0 };
            let color = Color::new(0x34, 0x48, 0x61, 0xff);
            d.draw_line_ex(
                Vector2::new((i * SQ_SIZE) as f32, 0.0),
                Vector2::new((i * SQ_SIZE) as f32, H as f32),
                t,
                color,
            );
            d.draw_line_ex(
                Vector2::new(0.0, (i * SQ_SIZE) as f32),
                Vector2::new(W as f32, (i * SQ_SIZE) as f32),
                t,
                color,
            );
        }
    }
}

fn main() {
    let mut sudoku = Sudoku::new(vec![
        vec![5, 0, 0, 9, 0, 0, 0, 0, 1],
        vec![9, 0, 4, 7, 0, 1, 0, 0, 0],
        vec![0, 7, 2, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 5, 0, 0, 2, 0, 7],
        vec![0, 0, 7, 0, 0, 6, 0, 4, 0],
        vec![3, 0, 0, 0, 0, 0, 6, 0, 0],
        vec![0, 0, 6, 0, 0, 0, 9, 3, 0],
        vec![0, 1, 0, 0, 0, 9, 4, 0, 0],
        vec![8, 0, 0, 4, 2, 0, 0, 0, 0],
    ]);
    let mut selected = vec![0, 0];
    let mut need_assistance = false;

    let (mut rl, thread) = raylib::init().size(W + 200, H).title("Sudoku").build();

    while !rl.window_should_close() {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let (x, y) = (rl.get_mouse_x(), rl.get_mouse_y());
            if 0 <= x && x <= W && 0 <= y && y <= H {
                selected[0] = (rl.get_mouse_x() / SQ_SIZE) as u8;
                selected[1] = (rl.get_mouse_y() / SQ_SIZE) as u8;
            }
        }

        match rl.get_key_pressed() {
            Some(KeyboardKey::KEY_LEFT) => {
                if selected[0] > 0 {
                    selected[0] -= 1;
                }
            }
            Some(KeyboardKey::KEY_RIGHT) => {
                if selected[0] < 8 {
                    selected[0] += 1;
                }
            }
            Some(KeyboardKey::KEY_UP) => {
                if selected[1] > 0 {
                    selected[1] -= 1;
                }
            }
            Some(KeyboardKey::KEY_DOWN) => {
                if selected[1] < 8 {
                    selected[1] += 1;
                }
            }
            Some(v) => {
                let num = match v {
                    KeyboardKey::KEY_ZERO => 0,
                    KeyboardKey::KEY_ONE => 1,
                    KeyboardKey::KEY_TWO => 2,
                    KeyboardKey::KEY_THREE => 3,
                    KeyboardKey::KEY_FOUR => 4,
                    KeyboardKey::KEY_FIVE => 5,
                    KeyboardKey::KEY_SIX => 6,
                    KeyboardKey::KEY_SEVEN => 7,
                    KeyboardKey::KEY_EIGHT => 8,
                    KeyboardKey::KEY_NINE => 9,
                    _ => 10,
                };
                sudoku.set_cell(num, selected[0], selected[1]);
            }
            _ => {}
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        d.gui_set_style(
            GuiControl::DEFAULT,
            GuiDefaultProperty::TEXT_SIZE as i32,
            SQ_SIZE / 2,
        );
        d.gui_check_box(
            rrect(W + 10, 10, 50, 30),
            Some(rstr!("Help")),
            &mut need_assistance,
        );

        d.gui_set_style(
            GuiControl::DEFAULT,
            GuiDefaultProperty::TEXT_SIZE as i32,
            24,
        );
        if d.gui_button(rrect(W + 10, 50, 100, 30), Some(rstr!("Solve"))) {
            sudoku.reset();
            sudoku.solve();
        }
        if d.gui_button(rrect(W + 10, 90, 100, 30), Some(rstr!("Reset"))) {
            sudoku.reset();
        }

        sudoku.help_player = need_assistance;
        sudoku.draw(&mut d, &mut selected);
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn run_tests() {
        let sudoku = super::Sudoku::new(vec![
            vec![5, 0, 0, 9, 0, 0, 0, 0, 1],
            vec![9, 0, 4, 7, 0, 1, 0, 0, 0],
            vec![0, 7, 2, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 5, 0, 0, 2, 0, 7],
            vec![0, 0, 7, 0, 0, 6, 0, 4, 0],
            vec![3, 0, 0, 0, 0, 0, 6, 0, 0],
            vec![0, 0, 6, 0, 0, 0, 9, 3, 0],
            vec![0, 1, 0, 0, 0, 9, 4, 0, 0],
            vec![8, 0, 0, 4, 2, 0, 0, 0, 0],
        ]);
        assert_eq!(sudoku.get_empty(), Some((1, 0)));
        assert_eq!(sudoku.is_valid(5, 1, 0), false); // Horizontal checking
        assert_eq!(sudoku.is_valid(4, 2, 3), false); // Vertical checking
        assert_eq!(sudoku.is_valid(4, 4, 6), false); // Checking box
        assert_eq!(sudoku.is_valid(6, 1, 1), true);
    }
}
