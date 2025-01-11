#!/usr/bin/env python

"""
	Script for testing already created Functionalities
"""

if __name__ == '__main__':
	# Section for testing the board generation part
	from boardgeneration import BoardGeneration
	board = BoardGeneration(True, False)
	print("This is the original")
	board.initiate_normal_chess()
	board = BoardGeneration(True, False)
	print("This is the chess 960")
	board.initiate_chess_960()


