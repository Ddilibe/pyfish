#!/usr/bin/env python

""" Script that tests bitwise operation """

import math

def testing_bitwise(value):
	try:
		value = eval(value)
		# if value < 0:
			# value = 
		value = bin(value)
		value = value.split("0b")[1]
		while (len(value) < 65):
			value = "0" + str(value)
		return value
	except Exception:
		return "Pls, Use Numbers"

def onescomplement(n):
	number_of_bits = int(math.floor(math.log(n)/ math.log(2))) + 1
	return ((1 << number_of_bits) - 1) ^ n

if __name__ == '__main__':
 	print("Please enter bitwise operation: ") 
 	while True:
 		value = input()
 		if value == "q":
 			break
 		print(f"\t {testing_bitwise(value)}")