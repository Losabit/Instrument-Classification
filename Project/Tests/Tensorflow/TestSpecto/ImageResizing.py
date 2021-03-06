import os
import PIL
from PIL import Image

def getFiles(path, extension):
    files = []
    for r, _, f in os.walk(path):
        for file in f:
            if extension in file:
                files.append(os.path.join(r, file))
    return files


start_width = 496
start_height = 369
factor_width_height = start_width / start_height
height = 64
width = (int)(height * factor_width_height)

path = 'D:\Dev\Instrument-Classification\Project\Tests\Tensorflow\TestSpecto\dataset/train'
subDirectories = ['saxo','piano','guitare']

for subDirectory in subDirectories:
    files = getFiles(os.path.join(path,subDirectory), '.png')
    for i in range(len(files)):
        image = Image.open(files[i])
        image.resize((width,height)).save(files[i])
