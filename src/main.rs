use ::rand::{seq::IteratorRandom, thread_rng, Rng};
use macroquad::{
    prelude::*,
    ui::{
        root_ui,
        widgets::{Button, Checkbox},
        Skin,
    },
};

const W: i32 = 630;
const H: i32 = 630;
const SQ_SIZE: i32 = W / 9;

pub struct Sudoku {
    pub board: Vec<Vec<u8>>,
    pub initial_board: Vec<Vec<u8>>,
    pub solved_board: Vec<Vec<u8>>,
    pub help_player: bool,
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

    fn create_board() -> Sudoku {
        let mut sudoku = Sudoku {
            initial_board: vec![vec![0; 9]; 9],
            solved_board: vec![vec![0; 9]; 9],
            help_player: false,
            board: vec![vec![0; 9]; 9],
        };
        let mut rng = thread_rng();
        for _ in 0..10 {
            let x: u8 = rng.gen_range(0..=8);
            let y: u8 = rng.gen_range(0..=8);
            let num = sudoku.get_valid_nums(x, y);
            let num = num.iter().choose(&mut rng).unwrap();
            sudoku.board[y as usize][x as usize] = *num;
            sudoku.initial_board[y as usize][x as usize] = *num;
        }

        sudoku.solve();
        sudoku.solved_board = sudoku.board.clone();
        sudoku.initial_board = sudoku.board.clone();
        loop {
            let x: u8 = rng.gen_range(0..=8);
            let y: u8 = rng.gen_range(0..=8);
            sudoku.board[y as usize][x as usize] = 0;
            sudoku.initial_board[y as usize][x as usize] = 0;
            if sudoku.no_of_solns() != 1 {
                sudoku.board[y as usize][x as usize] = sudoku.solved_board[y as usize][x as usize];
                sudoku.initial_board[y as usize][x as usize] =
                    sudoku.solved_board[y as usize][x as usize];
                break;
            }
        }
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

    fn get_valid_nums(&self, x: u8, y: u8) -> Vec<u8> {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        nums.retain(|&n| self.is_valid(n, x, y));
        if nums.len() == 0 {
            return vec![0];
        }
        return nums;
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

    fn no_of_solns(&mut self) -> usize {
        let empty_pos = self.get_empty();
        if empty_pos.is_none() {
            return 1; // One solution found
        }

        let (x, y) = empty_pos.unwrap();
        let mut num_solutions = 0;
        for n in 1..10 {
            if self.is_valid(n, x, y) {
                self.board[y as usize][x as usize] = n;
                num_solutions += self.no_of_solns();
                self.board[y as usize][x as usize] = 0;
            }
        }
        num_solutions
    }

    fn set_cell(&mut self, num: u8, x: u8, y: u8) {
        if num > 9 {
            return;
        }

        if self.initial_board[y as usize][x as usize] == 0 {
            self.board[y as usize][x as usize] = num;
        }
    }

    fn draw_selected(&self, x: i32, y: i32) {
        draw_rectangle(
            (x * SQ_SIZE) as f32,
            (y * SQ_SIZE) as f32,
            SQ_SIZE as f32,
            SQ_SIZE as f32,
            Color::from_hex(0xbbdefb),
        );
    }

    fn draw(&self, selected: &Vec<u8>) {
        for y in 0..9 {
            for x in 0..9 {
                let cell = self.board[y][x];

                if x as u8 == selected[0] && y as u8 == selected[1] {
                    self.draw_selected(x as i32, y as i32);
                }

                if cell == 0 {
                    continue;
                }

                let text_length = measure_text(&cell.to_string(), None, SQ_SIZE as u16, 1.0);
                let text_color = if self.initial_board[y as usize][x as usize] == 0 {
                    if self.help_player && self.solved_board[y as usize][x as usize] != cell {
                        Color::from_hex(0xff7777)
                    } else {
                        Color::from_hex(0x777777)
                    }
                } else {
                    BLACK
                };
                draw_text(
                    &cell.to_string(),
                    x as f32 * SQ_SIZE as f32 + text_length.width / 2.0,
                    y as f32 * SQ_SIZE as f32 + text_length.height * 1.5,
                    SQ_SIZE as f32,
                    text_color,
                );
            }
        }

        for i in 0..10 {
            let t = if i % 3 == 0 { 5.0 } else { 2.0 };
            let color = Color::from_hex(0x344861);
            draw_line(
                (i * SQ_SIZE) as f32,
                0.0,
                (i * SQ_SIZE) as f32,
                H as f32,
                t,
                color,
            );
            draw_line(
                0.0,
                (i * SQ_SIZE) as f32,
                W as f32,
                (i * SQ_SIZE) as f32,
                t,
                color,
            );
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Sudoku".to_owned(),
        fullscreen: false,
        window_width: W + 200,
        window_height: H,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut sudoku = Sudoku::create_board();
    let mut selected = vec![0, 0];
    let mut need_assistance = false;

    Sudoku::create_board();

    let button_style = root_ui()
        .style_builder()
        .font(include_bytes!(
            "../fonts/CaskaydiaCoveNerdFontMono-Regular.ttf"
        ))
        .unwrap()
        .font_size((SQ_SIZE / 3) as u16)
        .color(Color::from_hex(0x9ca0b0))
        .color_hovered(Color::from_hex(0x8c8fa1))
        .color_clicked(Color::from_hex(0x7287fd))
        .build();

    let label_style = root_ui()
        .style_builder()
        .font(include_bytes!(
            "../fonts/CaskaydiaCoveNerdFontMono-Regular.ttf"
        ))
        .unwrap()
        .font_size((SQ_SIZE as f32 / 3.5) as u16)
        .build();
    let checkbox_style = root_ui()
        .style_builder()
        .color(Color::from_hex(0x9ca0b0))
        .color_hovered(Color::from_hex(0x8c8fa1))
        .color_selected_hovered(Color::from_hex(0x7287fd))
        .color_selected(Color::from_hex(0x04a5e5))
        .build();

    let skin1 = Skin {
        button_style,
        checkbox_style,
        label_style,
        ..root_ui().default_skin()
    };

    root_ui().push_skin(&skin1);

    let big_label_style = root_ui()
        .style_builder()
        .font_size(50)
        .text_color(Color::from_hex(0xe64553))
        .build();
    let skin2 = Skin {
        label_style: big_label_style,
        ..skin1
    };

    let mut time = 0.0;

    loop {
        clear_background(Color::from_hex(0xeff1f5));

        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
            if 0.0 <= x && x <= W as f32 && 0.0 <= y && y <= H as f32 {
                selected[0] = (x / SQ_SIZE as f32) as u8;
                selected[1] = (y / SQ_SIZE as f32) as u8;
            }
        }

        match get_char_pressed() {
            Some('0') => sudoku.set_cell(0, selected[0], selected[1]),
            Some('1') => sudoku.set_cell(1, selected[0], selected[1]),
            Some('2') => sudoku.set_cell(2, selected[0], selected[1]),
            Some('3') => sudoku.set_cell(3, selected[0], selected[1]),
            Some('4') => sudoku.set_cell(4, selected[0], selected[1]),
            Some('5') => sudoku.set_cell(5, selected[0], selected[1]),
            Some('6') => sudoku.set_cell(6, selected[0], selected[1]),
            Some('7') => sudoku.set_cell(7, selected[0], selected[1]),
            Some('8') => sudoku.set_cell(8, selected[0], selected[1]),
            Some('9') => sudoku.set_cell(9, selected[0], selected[1]),
            _ => {}
        };

        if is_key_pressed(KeyCode::Space) {
            sudoku = Sudoku::create_board();
        }
        if is_key_pressed(KeyCode::Left) && selected[0] > 0 {
            selected[0] -= 1;
        }
        if is_key_pressed(KeyCode::Right) && selected[0] < 8 {
            selected[0] += 1;
        }
        if is_key_pressed(KeyCode::Up) && selected[1] > 0 {
            selected[1] -= 1;
        }
        if is_key_pressed(KeyCode::Down) && selected[1] < 8 {
            selected[1] += 1;
        }

        let mut time_str = String::new();
        time_str += &format!("{:0>2}", (time as i32 / 60).to_string());
        time_str += ":";
        time_str += &format!("{:0>2}", (time as i32 % 60).to_string());
        root_ui().push_skin(&skin2);
        root_ui().label(Vec2::new((W + 10) as f32, 10.0), &time_str);
        root_ui().pop_skin();
        time += get_frame_time();

        Checkbox::new(1)
            .label("Help")
            .pos(Vec2::new((W + 45) as f32, 60.0))
            .size(Vec2::new(0.0, 0.0))
            .ui(&mut root_ui(), &mut need_assistance);

        if Button::new("Solve")
            .size(Vec2::new(100.0, 30.0))
            .position(Vec2::new((W + 10) as f32, 90.0))
            .ui(&mut root_ui())
        {
            sudoku.solve();
        }
        if Button::new("Reset")
            .size(Vec2::new(100.0, 30.0))
            .position(Vec2::new((W + 10) as f32, 130.0))
            .ui(&mut root_ui())
        {
            time = 0.0;
            sudoku.reset();
        }

        sudoku.help_player = need_assistance;
        sudoku.draw(&selected);
        next_frame().await
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
