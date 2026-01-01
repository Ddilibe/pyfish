#!/usr/bin/env python3
from pygame import Color
from pygame.font import Font
from pygame_gui.elements import UILabel


class Text:
    """Create a text Object"""

    def __init__(self, text, pos, manager, **options):
        self.text = text
        self.pos = pos

        self.fontname = None
        self.fontsize = 32
        self.fontcolor = Color(1, 255, 1)
        self.set_font(**options)
        self.render()
        self.label = UILabel(
            relative_rect=self.rect,
            text=self.text,
            manager=manager,
        )

    def set_font(self, **kwargs):
        self.font = Font(self.fontname, self.fontsize)

    def render(self):
        self.image = self.font.render(self.text, True, self.fontcolor)
        self.rect = self.image.get_rect()
        self.rect.topleft = self.pos
        return self.image, self.rect

    def draw(self, screen):
        screen.blit(self.image, self.rect)
