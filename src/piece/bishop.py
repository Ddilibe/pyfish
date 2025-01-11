 #!/usr/bin/env python

"""
	Script containing the bishop chess piece for the chess game
"""

from piece import Piece

class Bishop(Piece):

	name = "bishop"
	def __init__(self, image_path):
		Piece.__init__(self, image_path)
		self.name = "bishop"