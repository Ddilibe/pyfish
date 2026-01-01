#!/usr/bin/env python3

import sys

import pygame
from pygame import Surface
from pygame_gui.core import ObjectID
from pygame_gui import UI_BUTTON_PRESSED
from pygame_gui.elements import UILabel, UIButton


from scenes.scene import BaseScene
from core.state import ChessStateArgs


class MenuScene(BaseScene):
    def __init__(self, core, manager):

        super().__init__(core, manager)

    def draw(self, surface: Surface):
        super().draw(surface)
        title_text = "DIRTY CHESS"
        title_rect, title_surface = self._get_font_rect(title_text, fontsize=64)

        self.title = UILabel(
            relative_rect=pygame.Rect((150, 10), (300, 300)),
            text=title_text,
            manager=self.manager,
            container=self.container,
            parent_element=self.container,
            object_id=ObjectID(class_id="@title"),
        )

        self.elements.append(self.title)

        self.start_button = UIButton(
            relative_rect=pygame.Rect((230, 200), (180, 50)),
            text="Play",
            manager=self.manager,
            container=self.container,
            object_id=ObjectID(class_id="@start_game"),
        )
        self.settings = UIButton(
            relative_rect=pygame.Rect((230, 250), (180, 50)),
            text="Settings",
            manager=self.manager,
            container=self.container,
            object_id=ObjectID(class_id="@settings"),
        )
        self.highscore = UIButton(
            relative_rect=pygame.Rect((230, 300), (180, 50)),
            text="Highscore",
            manager=self.manager,
            container=self.container,
            object_id=ObjectID(class_id="@highscore"),
        )
        self.exit = UIButton(
            relative_rect=pygame.Rect((230, 350), (180, 50)),
            text="Exit",
            manager=self.manager,
            container=self.container,
            object_id=ObjectID(class_id="@exit"),
        )

        self.elements += [self.settings, self.highscore, self.exit, self.start_button]

        # surface.blit(title_surface, title_rect)

    def process_events(self, event: pygame.Event):
        super().process_events(event)
        if event.type == UI_BUTTON_PRESSED:
            if (
                event.ui_element.most_specific_combined_id
                == self.start_button.most_specific_combined_id
            ):
                self.core.state.current_state = ChessStateArgs.PLAY
            
            if (
                event.ui_element.most_specific_combined_id
                == self.exit.most_specific_combined_id):
                sys.exit()
            

