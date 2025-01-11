#!/usr/bin/env python

"""
	Script containing the Pawn chess piece for the chess game
"""

from piece import Piece

class Pawn(Piece):

	name = "pawn"
	def __init__(self, image_path):
		super().__init__(image_path)
		self.name = "pawn"