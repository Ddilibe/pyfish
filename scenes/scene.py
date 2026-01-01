#!/usr/bin/env python3

import pygame
from pygame.event import Event
from pygame_gui import UIManager
from pygame_gui.elements import UIPanel, UIButton
from pygame_gui.core import ObjectID
from pygame_gui import UI_BUTTON_PRESSED


from core.logger import logger
from core.state import ChessStateArgs


class BaseScene:
    """Custom Base Scene"""

    def __init__(
        self,
        core,
        manager: UIManager,
    ) -> None:
        # core.scenes.append(self)
        self.core = core
        self.manager = manager
        self.container = UIPanel(
            pygame.Rect(0, 0, 640, 600),
            manager=self.manager,
            starting_height=0,
            object_id=ObjectID(class_id="@menu_container"),
        )
        logger.info(f"Initialized Scene: <{self.__class__.__name__}>")

        self.elements = list(self.container)

    def process_events(self, event: Event):

        if len(self.core.states) == 0:
            self._log("Uninitialized State")
            raise Exception("Uninitialized State")

        if len(self.core.states) > 1:

            logger.info(f"Processing Events for <{self.__class__.__name__}>")
            if event.type == pygame.USEREVENT:
                if event.user_type == UI_BUTTON_PRESSED:
                    if (
                        event.ui_element.most_specific_combined_id
                        == self.back_button.most_specific_combined_id
                    ):
                        self.core.states.pop()
                        self.core.state.current_state = self.core.states[-1]
                        self.core.states.append(self.core.state.current_state)
                        self.cleanup()
                        self.manager.clear_and_reset()
                        self.core.switch_scenes()

    def update(self, time_delta):
        pass

    def draw(self, surface: pygame.Surface):

        if len(self.core.states) > 1:
            self.back_button = UIButton(
                relative_rect=pygame.Rect((10, 10), (50, 50)),
                text="",
                manager=self.manager,
                object_id=ObjectID(class_id="@back_button"),
                container=self.container,
            )
            self.elements.append(self.back_button)

    def _log(self, info: str, status: int = 1) -> None:
        match status:
            case 1:
                logger.info(info)
            case 2:
                logger.warning(info)
            case 3:
                logger.error(info)
            case _:
                logger.info(info)

    def _get_font_rect(self, text: str, **kwargs) -> tuple[pygame.Rect, pygame.Surface]:
        font = pygame.font.Font(
            kwargs["fontname"] if kwargs.get("fontname") else None,
            kwargs["fontsize"] if kwargs.get("fontsize") else 32,
        )
        text_surface = font.render(
            text, True, kwargs["fontcolor"] if kwargs.get("fontcolor") else (1, 255, 1)
        )
        return text_surface.get_rect(), text_surface

    def cleanup(self):
        """Kill all UI elements when leaving the"""
        for i in self.elements:
            i.kill()
        self.elements.clear()
