#!/usr/bin/env python3

from threading import Thread

import chess
import pygame
import pygame_gui
from pygame.event import Event
from pygame_gui.core import ObjectID
from chess import PAWN, QUEEN, KNIGHT, BISHOP, ROOK
from pygame_gui.elements import UIButton, UIPanel, UITextBox, UIWindow
from chess import PIECE_NAMES, COLOR_NAMES, square_name, parse_square, Move, Piece

from core.logger import logger
from scenes.scene import BaseScene
from core.state import ChessStateArgs
from engine.engine import CustomEngine


class BoardScene(BaseScene):
    """Board scene for the chess board"""

    def __init__(self, core, manager):

        super().__init__(core, manager)

        self.chess_pieces = {}
        self.engine = CustomEngine()
        self.board = self.engine.board.board

        self.square_size = 64
        self.board_offset = (0, 0)

        self.board_img = self.create_board_surface(
            (self.container.get_abs_rect().width, self.container.get_abs_rect().height),
            "#ebecd0",
            "#779556",
        )
        self.container.set_image(self.board_img)
        self.chess_pieces_elements = []

        self.selected = None
        self.valid_moves = []

        self.container_two = UIPanel(
            relative_rect=pygame.Rect(-5, 510, 700, 40),
            manager=self.manager,
            starting_height=600,
        )

        self.info_text = UITextBox(
            relative_rect=pygame.Rect(0, 0, 700, 40),
            manager=self.manager,
            container=self.container_two,
            html_text="INFO: Starting Game ",
        )

        self.container_three = UIPanel(
            relative_rect=pygame.Rect(510, -10, 130, 525),
            manager=self.manager,
            starting_height=600,
        )
        self.history_info = UITextBox(
            relative_rect=pygame.Rect(0, 0, 130, 525),
            manager=self.manager,
            container=self.container_three,
            html_text=" Welcome to Dirty Chess",
        )

        self.promo_window = None
        self.selected_promotion = None
        self.selected_for_promotion = None

        self.move_sound = pygame.mixer.Sound("assets/sounds/interface-124464.mp3")
        self.move_sound.set_volume(0.5)

    def update_history(self) -> None:
        text = "History <br/>"
        for no, value in enumerate(self.board.move_stack):
            text += f"<font color='#06F71A'>{no+1}. {value}</font> <br/>"
        self.history_info.set_text(text)

    def update_info(self, text: str, status: int = 1) -> None:
        # color="#FFFB04"
        match status:
            case 1:
                text = f"INFO: <font color='#06F71A'>{text}</font>"
            case 2:
                text = f"WARNING: <font color='#F70606'>{text}</font>"
            case 3:
                text = f"ERROR: <font color='#FFFB04'>{text}</font>"
            case _:
                text = f"INFO: <font color='#06F71A'>{text}</font>"
        self.info_text.set_text(text)

    def spawn_piece(self, square, coord, piece_type, color):
        pos_x, pos_y = self.__coord_to_pixels(coord)

        piece_btn = UIButton(
            relative_rect=pygame.Rect(
                (pos_x, pos_y), (self.square_size, self.square_size)
            ),
            text="",
            manager=self.manager,
            container=self.container,
            object_id=ObjectID(
                class_id=f"@{color}_{piece_type}",
                object_id=f"#light_square" if square % 2 == 0 else f"#dark_square",
            ),
        )

        self.chess_pieces_elements.append(piece_btn)
        return piece_btn

    def __coord_to_pixels(self, coord):
        """Converts 'e4' to (x, y) pixels relative to the board panel."""
        files = "abcdefgh"
        ranks = "87654321"

        col = files.index(coord[0])
        row = ranks.index(coord[1])

        x = col * self.square_size
        y = row * self.square_size
        return (x, y)

    def __pixels_to_coord(self, col, row) -> str:
        """Converts (x, y) pixels relative to the board panel back to 'e4'."""
        files = "abcdefgh"
        ranks = "87654321"

        # col = int(x // self.square_size)
        # row = int(y // self.square_size)

        if 0 <= col < 8 and 0 <= row < 8:
            file_char = files[col]
            rank_char = ranks[row]
            return f"{file_char}{rank_char}"

        return None  # type: ignore

    def draw(self, surface):
        super().draw(surface)
        self.back_button.hide()
        self.board_img.fill((0, 0, 0))
        # self.container.set_image(self.board_img)
        self.board_img = self.create_board_surface(
            (self.container.get_abs_rect().width, self.container.get_abs_rect().height),
            "#ebecd0",
            "#779556",
        )
        self.container.set_image(self.board_img)

        for squares in chess.SQUARES:
            chess_piece = self.board.piece_at(squares)
            if chess_piece:
                self.spawn_piece(
                    squares,
                    coord=square_name(squares),
                    piece_type=PIECE_NAMES[chess_piece.piece_type],
                    color=COLOR_NAMES[chess_piece.color],
                )
            # else:
        for i in self.valid_moves:
            i = square_name(i)
            # self.update_info(f"Selected move: {i}")
            # print(f"Valid moves: {self.valid_moves}")
            self.draw_move_dot(self.board_img, i)
            # surface.blit(self.board_img, (0, 0))

    def create_board_surface(self, size, light_color, dark_color):
        """Generates a single image of a chessboard."""
        surface = pygame.Surface(size)
        sq_size = self.square_size
        for row in range(8):
            for col in range(8):
                color = light_color if (row + col) % 2 == 0 else dark_color
                pygame.draw.rect(
                    surface, color, (col * sq_size, row * sq_size, sq_size, sq_size)
                )

        return surface

    def draw_move_dot(self, surface: pygame.Surface, coord, color=(0, 170, 255)):
        pos = self.__coord_to_pixels(coord)

        rect = pygame.Rect(pos, (self.square_size, self.square_size)).inflate(-2, -2)

        pygame.draw.rect(surface, color, rect, width=4)

    def process_events(self, event: pygame.Event):
        super().process_events(event)

        if self.board.is_checkmate():
            self.update_info("Checkmate!", 2)
            self.change_history(title="CheakMate!!!", body=f"{self.winner_msg()}")
            self.handle_event_after_game_conclusion(event)
            return

        if self.board.is_game_over():
            self.update_info("Game Over!", 2)
            self.change_history(title="Game Over!!!", body=f"{self.winner_msg()}")
            self.handle_event_after_game_conclusion(event)
            return

        if self.board.is_stalemate():
            self.update_info("Stalemate!", 2)
            self.change_history(title="StaleMate!!!", body=f"{self.winner_msg()}")
            self.handle_event_after_game_conclusion(event)
            return

        if self.board.is_insufficient_material():
            self.update_info("Insufficient material!", 2)
            self.change_history(
                title="Insufficient Material <br> Game Over!!!",
                body=f"{self.winner_msg()}",
            )
            self.handle_event_after_game_conclusion(event)
            return

        if self.promo_window:
            if event.type == pygame_gui.UI_BUTTON_PRESSED:
                print("Button clicked")
                if event.ui_element.ui_container == self.promo_window.get_container():
                    print("it passed here")
                    chosen_type = event.ui_element.most_specific_combined_id.split("_")[
                        -1
                    ]
                    self.apply_promotion(chosen_type)

                    # CLEANUP
                    self.promo_window.kill()
                    self.container.enable()
                    self.promo_window = None
            return
        if event.type == pygame.MOUSEBUTTONDOWN:
            if event.button == 1:
                mouse_pos = pygame.mouse.get_pos()
                size = int(mouse_pos[0] // self.square_size), int(
                    mouse_pos[1] // self.square_size
                )
                if self.selected:

                    piece_square = parse_square(self.__pixels_to_coord(*size))

                    if piece_square in self.valid_moves:
                        # self.board.push_uci(f"{self.selected}{piece_square}")
                        move = Move(self.selected, piece_square)
                        current_piece = self.board.piece_at(self.selected)
                        if (current_piece and current_piece.piece_type == PAWN) and (
                            move.to_square // 8 == 7 or move.to_square // 8 == 0
                        ):
                            self.selected_for_promotion = self.selected
                            self.initiate_promotion(current_piece, move)

                        else:
                            self.board.push(move)
                            self.move_sound.play()

                        move_str = (
                            f"{square_name(self.selected)}{square_name(piece_square)}"
                        )
                        self.update_info(f"Selected move: {move_str}")
                        print(f"Valid moves: {self.valid_moves}")
                        print(self.board)
                        self.update_history()
                    else:
                        self.update_info(
                            f"Invalid move: {square_name(piece_square)}", 3
                        )

                    self.selected = None
                    self.valid_moves.clear()
                elif not self.selected:
                    print("Not selected")
                    piece_square = parse_square(self.__pixels_to_coord(*size))
                    self.update_info(f"Selected piece: {piece_square}")
                    piece = self.board.piece_at(piece_square)

                    if piece:
                        if piece.color != self.board.turn:
                            self.update_info(
                                f"Not your piece: {piece_square} and color. Time for {COLOR_NAMES[self.board.turn]} to play",
                                2,
                            )
                            return
                        self.selected = piece_square
                        for i in self.board.legal_moves:
                            if i.from_square == self.selected:
                                self.valid_moves.append(i.to_square)

    def update(self, time_delta):
        super().update(time_delta)

        def push_into(self):
            for i in self.chess_pieces_elements:
                i.kill()
            self.chess_pieces_elements.clear()

        main = Thread(target=push_into, args=(self,))
        main.start()
        main.join()

    def initiate_promotion(self, current_piece, move) -> None:
        self.promo_coord = move.to_square
        color = COLOR_NAMES[current_piece.color]

        win_w = self.square_size + 100
        win_h = (self.square_size * 5) + 50

        self.promo_window = UIWindow(
            rect=pygame.Rect(0, 0, win_w, win_h),
            manager=self.manager,
            window_display_title="Select Piece",
            object_id="#promo_window",
        )
        self.promo_window.set_position(
            (
                (self.container.get_abs_rect().width - win_w) // 2,
                (self.container.get_abs_rect().height - win_h) // 2,
            )
        )

        pieces = ["queen", "rook", "bishop", "knight", "pawn"]
        for i, p_type in enumerate(pieces):
            UIButton(
                relative_rect=pygame.Rect(
                    0, i * self.square_size, win_w, self.square_size
                ),
                text=f"{p_type}",
                manager=self.manager,
                container=self.promo_window,
                object_id=ObjectID(class_id=f"@{color}_{p_type}"),

            )

    def apply_promotion(self, piece_type: str) -> None:
        print("Piece Type: ", piece_type)
        pieces = {
            "queen": QUEEN,
            "rook": ROOK,
            "bishop": BISHOP,
            "knight": KNIGHT,
            "pawn": None,
        }
        self.selected_promotion = pieces.get(piece_type.strip())
        print("Selected Promotion: ", self.selected_promotion)
        print("Selected for Promotion: ", self.selected)
        if self.selected_for_promotion is None:
            return
        move = Move(
            self.selected_for_promotion, self.promo_coord, self.selected_promotion
        )
        self.board.push(move)
        self.move_sound.play()
        self.selected_promotion = None

    def change_history(self, *args, **kwargs) -> None:
        title = kwargs.get("title")
        body = kwargs.get("body")
        self.history_info.set_text(
            f"{title} <br/> {body} <br/> <ul><li><br/>Press r to reset.</li><br/><li>Press esc to return to menu</li></ul> "
        )

    def winner_msg(
        self,
    ) -> str:
        outcome = self.board.outcome()

        if outcome:
            if outcome.winner == chess.WHITE:
                return "White won!"
            elif outcome.winner == chess.BLACK:
                return "Black won!"
            elif outcome.winner == None:
                return "It's a draw."

        return f"Reason for game end: {outcome.termination.name}"  # type: ignore

    def handle_event_after_game_conclusion(self, event: Event) -> None:
        if event.type == pygame.KEYDOWN:
            if event.key == pygame.K_r:
                self.board.reset_board()
                self.board.reset()
            if event.key == pygame.K_ESCAPE:
                self.core.state.current_state = ChessStateArgs.MENU
