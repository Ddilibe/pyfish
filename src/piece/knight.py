#!/usr/bin/env python

"""
	Script containing the Knight chess piece for the chess game
"""

from piece import Piece

class Knight(Piece):

	name = "knight"
	def __init__(self, image_path):
		Piece.__init__(self, image_path)