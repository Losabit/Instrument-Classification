import os
import random
import ntpath
from shutil import copyfile

def removeDirectoryContent(folder):
    if os.path.isdir(folder) == False:
        os.mkdir(folder)
    for filename in os.listdir(folder):
        file_path = os.path.join(folder, filename)
        if os.path.isfile(file_path) or os.path.islink(file_path):
            os.unlink(file_path)
        elif os.path.isdir(file_path):
            shutil.rmtree(file_path)


inpath = '/home/losabit/Desktop/PA/dataset_fresh/spectrogramm/'
outpath = 'Fourier/dataset'
labels = ['guitare', 'piano', 'saxo']
train_part = 0.85
part_to_take = 1
extension = '.png'

for label in labels:
    files = []
    for r, _, f in os.walk(os.path.join(inpath, label)):
        for i in range(int(len(f) * part_to_take)) :
            file = f[i]
            if extension in file:
                files.append(os.path.join(r, file))

    random.shuffle(files)
    train_size = (int)(len(files) * train_part)
    train_path = os.path.join('/home/losabit/Desktop/PA/Instrument-Classification/Project/Dataset/Train/Spectrogram/', label)
    validation_path = os.path.join('/home/losabit/Desktop/PA/Instrument-Classification/Project/Dataset/Test/Spectrogram/', label)

    removeDirectoryContent(train_path)
    removeDirectoryContent(validation_path)

    for i in range(0, train_size):
        copyfile(files[i], os.path.join(train_path, ntpath.basename(files[i])))


    for i in range(train_size, len(files)):
        copyfile(files[i], os.path.join(validation_path, ntpath.basename(files[i])))
        