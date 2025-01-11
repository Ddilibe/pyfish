#!/usr/bin/env python

"""
	Script that has an instance of every chess piece
"""

import pygame
from uuid import uuid4

class Piece(object):
	"""
		Parent class for the chess pieces
	"""

	def __init__(self, image_path):
		"""
			Method for declaraing and initiating the chess piece

			Parameters:
				self.position: Containing the position of the chess piece at all times
				self.bitwise: Containing the position of the chess piece in bitwise at all times
				self.__name: Containing the name of the chess piece
				self.image_path: Containing the path to the image
		"""
		value = uuid4()
		self.id = value.hex
		self.__position = 0
		self.bitwise = 0
		self.name = None
		self.image_path = image_path
		self.decimal = 0

	def __dict__(self):
		""" Method for the ditionary representation of the class """
		formate = {
			"id" : self.id,
			"Name": self.name,
			"Position": self.position,
			"Bitwise" : self.bitwise,
			"Decimal" : self.decimal,
			"Image Path": self.image_path
		}
		return formate

	def move(self):
		pass

	def capture(self):
		pass

	@property	
	def position(self):
		""" Method for getting the private position attribute """
		return self.__position

	@position.setter
	def position(self, position):
		""" Method for setting the private position attribute """
		self.__position = position

	def __str__(self):
		""" Method for changing the string representation of the class """
		return f"< --\n\tName: {self.name}\n\tPosition: {self.position}\n\tBitwise: {self.bitwise}\n>"

	def display(self, app):
		""" Method for displaying the chess piece on the board """
		img = pygame.image.load(self.image_path)
		# print(self.position)
		app.blit(img, self.position)

	def generate_position(self):
		""" Method for generating the position using the bitwise """
		self.position = (self.position[0] * 80, self.position[1] * 80)
		# print(self, self.position)



"""
import path
import sys
 
# directory reach
directory = path.path(__file__).abspath()
 
# setting path
sys.path.append(directory.parent.parent)
 
# importing
from parentdirectory.geeks import geek_method
"""