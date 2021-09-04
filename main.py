#!/usr/bin/python3.9
import os
import pickle
from random import choice, randint, sample

import pygame
import pygame.display
import pygame.event
import pygame.time

from settings import (
    HEIGHT,
    PUZZLE_DIR,
    SQ_SIZE,
    WIDTH,
    font,
    font_small,
    font_timer,
    offset,
    small_font_size,
)

pygame.init()


def save_puzzle(board):
    file = os.path.join(PUZZLE_DIR, "puzzles.pkl")
    try:
        with open(file, "rb") as f:
            boards = pickle.load(f)
    except FileNotFoundError:
        boards = []

    boards.append(board)

    with open(file, "wb") as f:
        pickle.dump(boards, f)


def how_many_levels():
    with open(os.path.join(PUZZLE_DIR, "puzzles.pkl"), "rb") as f:
        return len(pickle.load(f))


def load_puzzle(level):
    file = os.path.join(PUZZLE_DIR, "puzzles.pkl")
    try:
        with open(file, "rb") as f:
            return Sudoku(pickle.load(f)[level])
    except IndexError:
        print(f"No puzzle with the level: {level+1}")
    except FileNotFoundError:
        print(f"{file} was not found")
    return Sudoku([[0 for _ in range(9)] for _ in range(9)])


class Sudoku:
    def __init__(self, board):
        self.board = board
        self.fixed: list = [
            (x, y)
            for x, row in enumerate(board)
            for y, num in enumerate(row)
            if num != 0
        ]
        self.selected: list = []
        self.penciled = {}

    @classmethod
    def convert_81_string_to_arr(cls, string):
        sudoku = []
        for i in range(9):
            sudoku.append([])
            for j in range(9):
                sudoku[i].append(int(string[i * 9 + j]))

        return cls(sudoku)

    @classmethod
    def random_board(cls):
        grid = [[0 for _ in range(9)] for _ in range(9)]
        randindex = randint(0, 8)
        grid[randindex] = sample(range(1, 10), 9)
        empty_board: Sudoku = cls(grid)
        empty_board.solve()

        num_of_non_empty_cells = randint(35, 45)
        options = [(x, y) for x in range(9) for y in range(9)]

        while len(options) > num_of_non_empty_cells:
            x, y = choice(options)
            empty_board.board[y][x] = 0
            options.remove((x, y))

        empty_board.fixed = [
            (x, y)
            for x, row in enumerate(empty_board.board)
            for y, num in enumerate(row)
            if num != 0
        ]

        return empty_board

    @classmethod
    def random_premade_puzzle(cls):
        level = randint(0, how_many_levels() - 1)
        return load_puzzle(level)

    def print(self):
        print("|-----------------------|")
        for i in range(9):
            print("|", end=" ")
            for j in range(9):
                end = " "
                cur_num = self.board[i][j]
                if j % 3 == 2:
                    end = " | "
                if cur_num == 0:
                    print(".", end=end)
                else:
                    print(cur_num, end=end)

            print()
            if (i + 1) % 3 == 0:
                print("|-----------------------|")

    def display(self, win):
        if self.selected:
            i, j = self.selected

            # Vertical Line
            pygame.draw.rect(win, "#e2ebf3", (i * SQ_SIZE, 0, SQ_SIZE, HEIGHT))

            # Horizontal Line
            pygame.draw.rect(win, "#e2ebf3", (0, j * SQ_SIZE, WIDTH, SQ_SIZE))

            # Box
            box_x_start = i // 3 * 3
            box_y_start = j // 3 * 3
            pygame.draw.rect(
                win,
                "#e2ebf3",
                (
                    box_x_start * SQ_SIZE,
                    box_y_start * SQ_SIZE,
                    SQ_SIZE * 3,
                    SQ_SIZE * 3,
                ),
            )

            pygame.draw.rect(
                win, "#bbdefb", (i * SQ_SIZE, j * SQ_SIZE, SQ_SIZE, SQ_SIZE)
            )

        for i, row in enumerate(self.board):
            for j, cell in enumerate(row):
                pygame.draw.rect(
                    win, "#cad0dc", (i * SQ_SIZE, j * SQ_SIZE, SQ_SIZE, SQ_SIZE), 1
                )

                if cell != 0:
                    text = font.render(
                        str(cell),
                        True,
                        "#344861" if (i, j) in self.fixed else "#0072e3",
                    )
                    text_rect = text.get_rect(
                        center=(j * SQ_SIZE + SQ_SIZE / 2, i * SQ_SIZE + SQ_SIZE / 1.7)
                    )
                    win.blit(text, text_rect)

        # Draw the thick lines
        for i in range(3):
            pygame.draw.line(
                win,
                "#344861",
                (i * SQ_SIZE * 3, 0),
                (i * SQ_SIZE * 3, HEIGHT),
                3,
            )

        for j in range(3):
            pygame.draw.line(
                win,
                "#344861",
                (0, j * SQ_SIZE * 3),
                (WIDTH, j * SQ_SIZE * 3),
                3,
            )

        # draw penciled text
        for x, y in self.penciled:
            for num in self.penciled[x, y]:
                text = font_small.render(str(num), True, "black")
                text_rect = text.get_rect(
                    center=(
                        x * SQ_SIZE + (num - 1) % 3 * small_font_size + small_font_size,
                        y * SQ_SIZE
                        + (num - 1) // 3 * small_font_size
                        + small_font_size,
                    )
                )
                win.blit(text, text_rect)

    def find_empty(self):
        for i, row in enumerate(self.board):
            for j, cell in enumerate(row):
                if cell == 0:
                    return (i, j)
        return None

    def solve(self):
        find = self.find_empty()
        if not find:
            return True
        else:
            row, col = find

        for i in range(1, 10):
            if self.is_valid_on((row, col), i, "solving"):
                self.board[row][col] = i

                if self.solve():
                    return True

                self.board[row][col] = 0

        return False

    def is_valid_on(self, pos, num, for_what="checking"):
        num = int(num)
        x, y = pos

        if for_what == "checking":
            if num in self.board[y]:
                return False

            for i in range(9):
                if num == self.board[i][x]:
                    return False

            for ybox in range(y // 3 * 3, y // 3 * 3 + 3):
                for xbox in range(x // 3 * 3, x // 3 * 3 + 3):
                    if num == self.board[ybox][xbox]:
                        return False

            return True
        elif for_what == "solving":
            for i in range(9):
                if self.board[x][i] == num and y != i:
                    return False

            for i in range(9):
                if self.board[i][y] == num and x != i:
                    return False

            box_x = y // 3
            box_y = x // 3
            for i in range(box_y * 3, box_y * 3 + 3):
                for j in range(box_x * 3, box_x * 3 + 3):
                    if self.board[i][j] == num and (i, j) != pos:
                        return False
            return True

    def put_text(self, text):
        x, y = self.selected
        num = int(text)

        if num == 0:
            self.board[y][x] = 0
            self.penciled[x, y] = []
            return

        if (y, x) in self.fixed or self.board[y][x] != 0:
            return

        if self.is_valid_on(self.selected, num):
            if num in self.penciled.get((x, y), []):
                self.board[y][x] = num
                self.penciled[x, y] = []
            else:
                self.penciled[x, y] = self.penciled.get((x, y), []) + [num]
        elif num in self.penciled.get((x, y), []):
            self.penciled[x, y].remove(num)

    def directly_put_text(self, text):
        x, y = self.selected
        num = int(text)

        if num == 0:
            self.board[y][x] = 0
            return

        if (y, x) in self.fixed or self.board[y][x] != 0:
            return

        if self.is_valid_on(self.selected, num):
            self.board[y][x] = num

    def update(self):
        if pygame.mouse.get_pressed()[0]:
            pos = pygame.mouse.get_pos()
            x = pos[0] // SQ_SIZE
            y = (pos[1] - offset) // SQ_SIZE
            self.selected = [x, y]

    def change_selected(self, key):
        if not self.selected:
            return
        if key == pygame.K_UP and self.selected[1] > 0:
            self.selected[1] -= 1
        elif key == pygame.K_DOWN and self.selected[1] < 8:
            self.selected[1] += 1
        elif key == pygame.K_LEFT and self.selected[0] > 0:
            self.selected[0] -= 1
        elif key == pygame.K_RIGHT and self.selected[0] < 8:
            self.selected[0] += 1

    def clear(self):
        self.penciled = {}
        self.board = [
            [self.board[j][i] if (j, i) in self.fixed else 0 for i in range(9)]
            for j in range(9)
        ]


board = [
    [5, 3, 0, 0, 7, 0, 0, 0, 0],
    [6, 0, 0, 1, 9, 5, 0, 0, 0],
    [0, 9, 8, 0, 0, 0, 0, 6, 0],
    [8, 0, 0, 0, 6, 0, 0, 0, 3],
    [4, 0, 0, 8, 0, 3, 0, 0, 1],
    [7, 0, 0, 0, 2, 0, 0, 0, 6],
    [0, 6, 0, 0, 0, 0, 2, 8, 0],
    [0, 0, 0, 4, 1, 9, 0, 0, 5],
    [0, 0, 0, 0, 8, 0, 0, 7, 9],
]

if __name__ == "__main__":
    offset = 50
    win = pygame.display.set_mode((WIDTH, HEIGHT + offset))
    screen = pygame.Surface((WIDTH, HEIGHT))

    clock = pygame.time.Clock()

    # sudoku = Sudoku.random_board()
    sudoku = Sudoku(board)
    elapsed_time = 0

    def milli_to_minutes(millis):
        millis = int(millis)
        seconds = (millis // 1000) % 60
        minutes = (millis // (1000 * 60)) % 60
        return f"{minutes:02}:{seconds:02}"

    while True:
        elapsed_time += clock.tick(60)
        win.fill("#ffffff")
        screen.fill("#ffffff")
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                pygame.quit()
                exit()
            if event.type == pygame.KEYDOWN:
                if 48 <= event.key <= 58:
                    sudoku.put_text(event.unicode)
                elif event.key == pygame.K_RETURN:
                    sudoku.solve()
                elif event.key == pygame.K_BACKSPACE:
                    sudoku.clear()
                elif event.key == pygame.K_r:
                    # sudoku = Sudoku.random_board()
                    sudoku = Sudoku.random_premade_puzzle()
                else:
                    sudoku.change_selected(event.key)

        sudoku.display(screen)
        sudoku.update()

        text = font_timer.render(
            f"Time: {milli_to_minutes(elapsed_time)}", True, "#3c4f67"
        )
        text_rect = text.get_rect(midright=(WIDTH, offset // 2))
        win.blit(text, text_rect)

        win.blit(screen, (0, offset))

        pygame.display.flip()
