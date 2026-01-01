#!/usr/bin/env python3
import os
import crypt
import pickle
from uuid import uuid4
from typing import Optional
from datetime import datetime

from chess import Board, Move


class CustomBoard:

    def __init__(
        self,
        custom: bool = False,
        string: Optional[str] = None,
        chess360: bool = False,
        name: str = "",
    ) -> None:
        self.all_moves, self.reversed_move = [], []
        if custom:
            self.board = Board(fen=string)
        elif chess360:
            self.board = Board(chess960=chess360)
        else:
            self.board = Board()
        self.location_name = name

    def _move(self, move) -> bool:
        move = Move.from_uci(move)
        if move in self.board.legal_moves:
            self.board.push(move)
            self.all_moves.append(move)
            self.reversed_move.clear()
            return True
        return False


    def _undo(self):
        self.reversed_move.append(self.all_moves.pop())

    def _redo(self):
        self.board.push(self.reversed_move.pop())

    def _save(self):
        values = {"class": self, "moves": self.all_moves, "board": self.board}
        # crypt.crypt(v)
        os.mkdir("storage")
        name = (
            f"storage/{str(uuid4())[:10]}_{datetime.year}_{datetime.day}_{datetime.month}_{datetime.minute}.pkl"
            if self.location_name.strip() == ""
            else self.location_name
        )
        with open(name, "wb") as file:
            pickle.dump(values, file)

    @staticmethod
    def _load(location: str) -> "CustomBoard":
        with open(location, "rb") as file:
            instance = pickle.load(file)
            new_board: CustomBoard = instance.get("class")
            new_board.all_moves = instance.get("moves")
            new_board.board = instance.get("board")
            new_board.reversed_move = []
            new_board.location_name = instance.get("name")
            return new_board

    def _display(self):
        board_str = str(self.board).split("\n")

        for i, row in enumerate(board_str):
            print(f"{8 - i} {row}")

        print("  h g f e d c b a")
