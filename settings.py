import pygame

pygame.init()

WIDTH, HEIGHT = 595, 595
SQ_SIZE = WIDTH // 9

font = pygame.font.SysFont("Arial", SQ_SIZE)
small_font_size = SQ_SIZE // 4
font_small = pygame.font.SysFont("Arial", small_font_size)
font_timer = pygame.font.SysFont("Arial", 35)
offset = 50

PUZZLE_DIR = "puzzles"
