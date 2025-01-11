#!/usr/bin/env python

"""
	Script containing the Configurations for the scene
"""

from scene import dir_path
import pygame


class Scene:
	""" Class for chossing of the Scene """

	def __init__(self, app):
		""" Initializing the scene class """
		self._select_scene = f"{dir_path}/media/image/board/chessgrille.png"
		self.app = app

	def select_image(self):
		""" Method for selecting images """
		pass

	def display_image(self): # chess_pieces):
		img = pygame.image.load(self._select_scene)
		image, size = self.transformScaleKeepRatio(img, self.app.get_size())
		# for i in chess_pieces:
		# 	i.generate_position(image.get_width(), image.get_height())
		# 	i.display(self.app)
		self.app.blit(image, image.get_rect())

	def transformScaleKeepRatio(self, image, size):
	    iwidth, iheight = image.get_size()
	    scale = min(size[0] / iwidth, size[1] / iheight)
	    new_size = (round(iwidth * scale), round(iheight * scale))
	    scaled_image = pygame.transform.smoothscale(image, new_size) 
	    image_rect = scaled_image.get_rect(center = (size[0] // 2, size[1] // 2))
	    return scaled_image, image_rect