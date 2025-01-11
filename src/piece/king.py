 #!/usr/bin/env python

"""
	Script containing the King chess piece for the chess game
"""

from piece import Piece

class King(Piece):

	name = "king"
	def __init__(self, image_path):
		Piece.__init__(self, image_path)