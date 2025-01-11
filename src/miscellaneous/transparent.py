#!/usr/bin/env python3

"""	Script containing function for modifying images """

from PIL import Image
import os


def convertImage():
    path = "media/image/image_1"
    files = os.listdir(path)
    for file in files:
            img = Image.open(f"media/image/image_1/{file}")
            img = img.convert("RGBA")
            datas = img.getdata()
            newData = []
            for item in datas:
                if item[0] == 255 and item[1] == 255 and item[2] == 255:
                    newData.append((255, 255, 255, 0))
                else:
                    newData.append(item)
            img.putdata(newData)
            img.save(f"media/image/image_1/{file}", "PNG")
            print("Successful")

if __name__ == '__main__':
	convertImage()