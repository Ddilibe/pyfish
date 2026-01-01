#!/usr/bin/env python3

import pygame
import pygame_gui
from pygame_gui.core import ObjectID
from pygame_gui.elements import UIButton, UILabel, UIImage

# from pygame_gui import UI_BUTTON_PRESSED

from scenes.scene import BaseScene
from core.state import ChessStateArgs


class WelcomeScene(BaseScene):
    def __init__(self, core, manager):

        super().__init__(core, manager)

    def draw(self, surface):
        super().draw(surface)
        text = "Start Game"
        text_rect, text_surface = self._get_font_rect(text, fontsize=33)

        self.start_button = UIButton(
            relative_rect=pygame.Rect((10, 50), (180, 50)),
            text="Start Game",
            manager=self.manager,
            container=self.container,
            object_id=ObjectID(class_id="@start_game"),
        )
        self.start_button.hide()

        self.text_box = UILabel(
            relative_rect=pygame.Rect((100, 300), (400, 200)),
            text="Click here to start game",
            manager=self.manager,
            container=self.container,
            object_id=ObjectID(class_id="@text_box"),
            anchors={
                "left": "left",
                "right": "right",
                "top": "top",
                "bottom": "bottom",
            },
        )
        self.text_box.set_active_effect(pygame_gui.TEXT_EFFECT_SHAKE)

        logo = pygame.image.load("assets/ChatGPT Image Dec 31, 2025, 10_30_52 PM.png")
        logo = pygame.transform.scale(logo, (1000, 1000))

        self.logo = UIImage(
            relative_rect=pygame.Rect((70, 10), (500, 400)),
            manager=self.manager,
            container=self.container,
            object_id=ObjectID(class_id="@logo"),
            anchors={
                "left": "left",
                "right": "right",
                "top": "top",
                "bottom": "bottom",
            },
            image_surface=logo,
        )

    def process_events(self, event: pygame.Event):
        super().process_events(event)
        # if event.type == pygame_gui.UI_BUTTON_PRESSED:
        #     self._log(
        #         f"{event.ui_element}, {pygame_gui.UI_BUTTON_PRESSED} {self.start_button} {event.ui_element.most_specific_combined_id}"
        #     )
        #     print(
        #         self.container.most_specific_combined_id,
        #         self.start_button.most_specific_combined_id,
        #     )

        #     if (
        #         event.ui_element.most_specific_combined_id
        #         == self.start_button.most_specific_combined_id
        #     ):
        #         self.core.state.current_state = ChessStateArgs.MENU
        #         self._log("Called the Login")
        if event.type == pygame.MOUSEBUTTONDOWN and event.button == 1:
            self.core.state.current_state = ChessStateArgs.MENU
