#!/usr/bin/env python3
from typing import Optional

import pygame
from pygame import locals
from pygame_gui import UIManager

from core.texts import Text
from scenes.menu import MenuScene
from scenes.scene import BaseScene
from scenes.board import BoardScene
from core.config import load_config
from scenes.welcome import WelcomeScene
from core.state import customstate as state, ChessStateArgs


class App:
    """Create a single-window app with multiple scenes."""

    def __init__(self):
        """Initialize pygame and the application."""
        pygame.mixer.pre_init(44100, -16, 2, 512) 
        pygame.init()
        self.flags = pygame.DOUBLEBUF | pygame.HWSURFACE | pygame.NOFRAME
        self.screen = pygame.display.set_mode((640, 550), self.flags)
        self.clock = pygame.time.Clock()
        self.running = True

        self.background = pygame.Surface((640, 550))
        self.background.fill(pygame.Color("black"))

        self.cfg = load_config()

        # themes
        self.manager = UIManager((640, 550), "theme.json")
        self.manager.get_theme().load_theme("themes/buttons.json")
        self.manager.get_theme().load_theme("themes/labels.json")
        self.manager.get_theme().load_theme("themes/chess_pieces.json")

        # self.text = Text("I am running in the hills", (20, 20), self.manager)

        self.state = state
        self.states = [ChessStateArgs.WELCOME]

        self.scenes = []
        self.scene: BaseScene = WelcomeScene(self, self.manager)

        pygame.mixer.init()
        pygame.mixer.music.load(self.cfg.assets.music_path)


    def run(self):
        """Run the main event loop."""
        pygame.mixer.music.play(-1, 0.0)
        while self.running:
            time_delta = self.clock.tick(60) / 1000.0

            self.clock.tick(60)
            self.handle_events()
            self.screen.fill((0, 0, 0))

            # self.text.draw(self.manager)
            if self.states[-1] != self.state.current_state:
                self.__switch_scene()

            self.manager.update(time_delta)
            self.scene.update(time_delta)

            self.screen.blit(self.background, (0, 0))

            self.scene.draw(self.screen)
            self.manager.draw_ui(self.screen)

            pygame.display.update()
        pygame.quit()

    def handle_events(self):
        """Handle events."""
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                self.running = False

            self.manager.process_events(event)
            self.scene.process_events(event)

    def __switch_scene(self):
        if (
            self.states[-1] != self.state.current_state
            and self.state.current_state != ChessStateArgs.WELCOME
        ):
            print(
                f"switching scene from {self.states[-1]} to {self.state.current_state}"
            )
            self.states.append(self.state.current_state)
            self.scene.cleanup()
            self.manager.clear_and_reset()
            print(self.states)

        match self.state.current_state:
            case ChessStateArgs.PLAY:
                self.scene = BoardScene(self, self.manager)
            case ChessStateArgs.MENU:
                self.scene = MenuScene(self, self.manager)
            case ChessStateArgs.PAUSE:
                pass
            case ChessStateArgs.QUIT:
                self.running = False
            case ChessStateArgs.SETTINGS:
                pass
            case ChessStateArgs.HELP:
                pass
            case ChessStateArgs.WELCOME:
                self.scene = WelcomeScene(self, self.manager)
            case _:
                pass

    def switch_scenes(self):
        self.__switch_scene()


if __name__ == "__main__":
    App().run()
