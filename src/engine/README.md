# Chess Engine

This is a chess engine developed my me

### Bitboard

A bitboard is a way of representing the borad on a computer. Bitboards can also be applied to games that work in a square by square board. The bitboard is represented in ones and zeros. The bitboards are read from left to right and from top to bottom. Bitboard is very efficient and takes a lot less space. 

### How to find the Complements
1. One's Complement

To find one's complement of a number, first of all, convert the number into bits then any place that there is an occurance of 2 replace it with 0. After that convert it to denary.


|	Decimal | Binary | One's complement |
| :----:    | :-----:| :-------: |
|		5  |   0101 |     1010 |
|		-5  |   0101 |     1010 |
|		-6  |   0110 |     1001 |
|		-7  |   0111 |     1000 |

2. Two's complement

To find the second complement, find the one's complement and add one to it. This only occurs on negative numbers

## Bitwise Operations

Bitwise operations form my own point of view are those operations that involves the manipulations of bits in the computer. Bits are 1s and 0s. In python programming, bit operation can only be done on integers.

Before any integer is manipulated via bitwise operators, they are first converted into binary form (1s and 0s).

```python

	# Example:
	a, b = 1, 2

	# First, it converts a to binary form which is 1
	# Then it converts b to its binary form which is 10
	# a & b will be	1 & 10 
	ans = a&b
	# Then it goes on with the bitwise operation. 

	# When It is done with the bitwise operation, it then converts it to denary (base 10)
	print(ans)
	# This will print 0
```

There are different types of bit operators

| Operators | Description | Syntax | Explanation |
| --------- | ----------- | ------ | ----------- |
| & | Bitwise AND | x & y | This bitwise operator compares both bit values and return 1 if both bits are 1 else it will return 0 <br> `10001 & 11000 = 10000` |
| \| | Bitwise OR | x\|y | This bitwise operator compares both bit values and return 1 if either bits are 1 else it will return 0 <br> `10001 & 11000 = 11001` |
| ~ | Bitwise NOT | ~x | This bi twise operation is used to obtain one's complement of the value. <br> |
| ^ | Bitwise XOR | x^y | This bitwise operator compares both bit values and returns 1 if one of the bit is 0 and the other one is 1. If both bits are 0 or 1, then it returns 0 <br> `10001 ^ 11000 = 01001` |
| \>\> | Bitwise Right Shift | x\>\>2 | This bitwise operator shifts the left operand bits towards the right side for the given number of times in the right operand <br> `10001 >> 2 = 100` |
| << | Bitwise Left Shift | x<<2 | This bitwise operator is the opposite of the right shift operator. It shifts the left operand bits towards the left side for the given number of times in the right operand. <br> `10001 << 2 = 1000100` |

** Note ** : `10001 = 17` and `11000 = 24`
### Resources

1. [Wikipedia: Chess960 Starting Position](https://en.wikipedia.org/wiki/Fischer_random_chess)

2. [Chess and Bitboards](https://pages.cs.wisc.edu/~psilord/blog/data/chess-pages/index.html)

3. [Chess Programming](https://www.chessprogramming.org/General_Setwise_Operations)

4. [Bitwise Operations on DigitalOcean](https://www.digitalocean.com/community/tutorials/python-bitwise-operators)