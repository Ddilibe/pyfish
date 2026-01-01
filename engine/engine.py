#!/usr/bin/env python3
from enum import StrEnum
from typing import Literal, Optional

from chess import Move, WHITE, BLACK
from chess.engine import SimpleEngine, Limit

from engine.board import CustomBoard


class Level(StrEnum):
    BEGINNER = "BEGINNER"
    INTERMEDIATE = "INTERMEDIATE"
    ADVANCED = "ADVANCED"
    EXPERT = "EXPERT"


class RunMode(StrEnum):
    AIVSAI = "ai vs ai"
    AIVSHUMAN = "ai vs human"
    HUMANVSHUMAN = "human vs human"


class CustomEngine:
    """Holds the attribute for a custom engine"""

    def __init__(self, runmode: RunMode = RunMode.AIVSHUMAN):
        self.board = CustomBoard()
        # self.aiengine = AIEngine(Level.BEGINNER)
        self.runmode = runmode
        self.first, self.second, current = None, None, None
        self.ai = False
        self.setup_colors()

    class Meta:
        pass

    # def ai_vs_ai(self):

    def run(self):
        while True:
            try:
                self.board._display()
                move = input("Make your Move: ")
                if move == "q":
                    self.board._save()
                    break
                self.board._move(move)
                self.board._display()
                if self.ai:
                    print("AI thinking")
                    self.ai_move()
                else:
                    self.board._move(move)
            except Exception as e:
                print("Invalid Move")
                print(e)

    def setup_colors(self) -> None:
        pass

    def ai_move(self):
        move = str(self.aiengine.move(self.board))
        print(move)
        self.board._move(move)


class AIEngine:
    levels = {
        "BEGINNER": {"skill": 0, "depth": 2, "time": 0.01},
        "INTERMEDIATE": {"skill": 10, "depth": 8, "time": 0.05},
        "ADVANCED": {"skill": 15, "depth": 12, "time": 0.1},
        "EXPERT": {"skill": 20, "depth": 20, "time": 0.5},
    }

    def __init__(self, level: Level) -> None:
        self.skill = self.levels[level]["skill"]
        self.depth = self.levels[level]["depth"]
        self.time = self.levels[level]["time"]
        self.engine = SimpleEngine.popen_uci(
            "assets/engine/stockfish-ubuntu-x86-64-avx2"
        )

    def move(self, board: CustomBoard) -> Optional[Move]:
        self.engine.analyse(board.board, Limit(depth=self.depth, time=self.time))
        result = self.engine.play(board.board, Limit(depth=self.depth, time=self.time))
        return result.move
