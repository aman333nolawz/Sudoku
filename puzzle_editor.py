import pygame

from main import Sudoku, load_puzzle, save_puzzle
from settings import HEIGHT, PUZZLE_DIR, WIDTH, font_timer, offset

win = pygame.display.set_mode((WIDTH, HEIGHT + offset))
screen = pygame.Surface((WIDTH, HEIGHT))

clock = pygame.time.Clock()

board = [[0 for _ in range(9)] for _ in range(9)]
# sudoku = Sudoku.random_board()
sudoku = Sudoku(board)

level = 1


while True:
    clock.tick(60)
    win.fill("#ffffff")
    screen.fill("#ffffff")
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            pygame.quit()
            exit()
        if event.type == pygame.KEYDOWN:
            if 48 <= event.key <= 58:
                sudoku.directly_put_text(event.unicode)
            elif event.key == pygame.K_RETURN:
                sudoku.solve()
            elif event.key == pygame.K_BACKSPACE:
                sudoku.clear()
            elif event.key == pygame.K_r:
                board = [[0 for _ in range(9)] for _ in range(9)]
                sudoku = Sudoku(board)
            elif event.key == pygame.K_s:
                save_puzzle(sudoku.board)
            elif event.key == pygame.K_l:
                sudoku = load_puzzle(level - 1)
            elif pygame.key.get_mods() & pygame.KMOD_SHIFT:
                if event.key == pygame.K_UP:
                    level += 1
                    sudoku = load_puzzle(level - 1)
                elif event.key == pygame.K_DOWN:
                    level = max(1, level - 1)
                    sudoku = load_puzzle(level - 1)
            else:
                sudoku.change_selected(event.key)

    sudoku.display(screen)
    sudoku.update()

    text = font_timer.render(f"Level: {level}", True, "#3c4f67")
    text_rect = text.get_rect(midright=(WIDTH, offset // 2))
    win.blit(text, text_rect)

    win.blit(screen, (0, offset))

    pygame.display.flip()
