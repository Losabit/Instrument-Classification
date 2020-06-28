import os
import random
import ntpath
from shutil import copyfile

inpath = 'C:\\Users\\quent\\Desktop\\dataset\\spectrogramm\\saxo\\'
train_outpath = 'C:\\Users\\quent\\Desktop\\Projet Annuel\\Applis\\TestDataset\\ImageClassification\\dataset\\train\\saxo\\'
validation_outpath = 'C:\\Users\\quent\\Desktop\\Projet Annuel\\Applis\\TestDataset\\ImageClassification\\dataset\\validation\\saxo\\'
train_part = 0.8
extension = '.png'

files = []
for r, _, f in os.walk(inpath):
    for file in f:
        if extension in file:
            files.append(os.path.join(r, file))

random.shuffle(files)
train_size = (int)(len(files) * train_part)

for i in range(0, train_size):
    copyfile(files[i], train_outpath + ntpath.basename(files[i]))


for i in range(train_size, len(files)):
    copyfile(files[i], validation_outpath + ntpath.basename(files[i]))
    