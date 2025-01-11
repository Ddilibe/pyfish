#!/usr/bin/env python

"""
	Script containing the Rook chess piece for the chess game
"""

from piece import Piece

class Rook(Piece):

	name = "rook"
	def __init__(self, image_path):
		Piece.__init__(self, image_path)
		self.name = "rook"