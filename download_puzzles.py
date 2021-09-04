import requests
import tqdm

from main import save_puzzle

url = "https://sugoku.herokuapp.com/board"
params = {"difficulty": "random"}

for _ in tqdm.trange(50):
    board = requests.get(url, params=params).json()["board"]
    save_puzzle(board)
