#coding=utf8
import os
import sys
import csv
import numpy as np
import matplotlib.pyplot as plt
import random
import shutil
import tensorflow as tf

sys.path.append('PythonML')

from linear import Linear
from multicouche import MLP
from rbf import RBF
from scipy.fftpack import fft
from scipy.io import wavfile
from wavio import readwav
from PythonML.rbf import RBF


# argv[1] = /home/losabit/Desktop/OriginalData/guitare/Studio/smells-like-teen-spiritnirvana-covered-by-feng-e.wav
# argv[2]= 1
# argv[3] = Models/mlp.txt
# python //home/losabit/Desktop/PA/Instrument-Classification/Project/App/interfacev2/script.py /home/losabit/Desktop/OriginalData/guitare/Studio/smells-like-teen-spiritnirvana-covered-by-feng-e.wav 1 Models/mlp.txt
# python //home/losabit/Desktop/PA/Instrument-Classification/Project/App/interfacev2/script.py /home/losabit/Desktop/OriginalData/Notes/piano.wav 2 Models/rbf_one_vs_one

def max_indice(values):
    max_value = values[0]
    max_ind = 0
    for i in range(1, len(values)):
        if values[i] > max_value:
            max_ind = i
            max_value = values[i]
    return max_ind


'''
ARGUMENT 1 => File path
ARGUMENT 2 => TYPE MODEL
ARGUMENT 3 => MODEL PATH
'''
inpath = sys.argv[1]

model_type = int(sys.argv[2])
model_path = sys.argv[3]

dll_path = 'PythonML/libmllib_rust.so'
IMG_HEIGHT = 32
IMG_WIDTH = 32
target_size = (IMG_WIDTH, IMG_HEIGHT)
out_layer_size = 3
outpath = os.path.join("tmp/", str(random.randint(0, 10000)))

os.mkdir(outpath)

if model_type == 1:
    mlp = MLP(dll_path)
    mlp.load_model(model_path)
elif model_type == 2:
    rbf_guitare_saxo = RBF(dll_path)
    rbf_guitare_saxo.load_model(os.path.join(model_path, "guitare_saxo.txt"))
    rbf_piano_saxo = RBF(dll_path)
    rbf_piano_saxo.load_model(os.path.join(model_path, "piano_saxo.txt"))
    rbf_guitare_piano = RBF(dll_path)
    rbf_guitare_piano.load_model(os.path.join(model_path, "guitare_piano.txt"))
    

frequency, _, data = readwav(inpath)
if len(data.shape) != 1:
    data = data.sum(axis=1) / 2

response = []
guitare_count = 0
saxo_count = 0
piano_count = 0
for i in range(len(data) // frequency - 1):
    outpath_file = os.path.join(outpath, 'tmp' + str(i) + '.png')
    new_data = data[i * frequency:(i + 1) * frequency]
    plt.specgram(new_data, Fs=frequency)
    plt.axis('off')
    plt.savefig(outpath_file, bbox_inches='tight', pad_inches=0)
    plt.close()

    image = tf.keras.preprocessing.image.load_img(outpath_file, grayscale=False, color_mode='rgb',
                                                  target_size=target_size, interpolation='nearest')
    image_predict = np.array(tf.keras.preprocessing.image.img_to_array(image, data_format=None, dtype=None) / 255,
                             dtype='float64')

    if model_type == 1:
        predicted_value = mlp.predict_multicouche_model_classification(image_predict)
        predicted_list = [predicted_value[i] for i in range(out_layer_size)]
        max_ind = max_indice(predicted_list)
        if max_ind == 0:
            piano_count += 1
        elif max_ind == 1:
            saxo_count += 1
        elif max_ind == 2:
            guitare_count += 1
        else:
            ValueError("not implemented")
    elif model_type == 2:
        predicted_value = rbf_guitare_saxo.predict_rbf_model(image_predict.flatten())
    
        if predicted_value < 0:
            guitare_count += 1
        else:
            saxo_count += 1

        predicted_value = rbf_piano_saxo.predict_rbf_model(image_predict.flatten())
    
        if predicted_value < 0:
            piano_count += 1
        else:
            saxo_count += 1

        predicted_value = rbf_guitare_piano.predict_rbf_model(image_predict.flatten())
    
        if predicted_value < 0:
            guitare_count += 1
        else:
            piano_count += 1

shutil.rmtree(outpath)

if model_type == 1:
    print("" + str((piano_count * 100) / (len(data) // frequency - 1)) + "")
    print("" + str((saxo_count * 100) / (len(data) // frequency - 1)) + "")
    print("" + str((guitare_count * 100) / (len(data) // frequency - 1)) + "")
elif model_type == 2:
    print("" + str((piano_count * 100) / (len(data) // frequency - 1) / 3) + "")
    print("" + str((saxo_count * 100) / (len(data) // frequency - 1) / 3) + "")
    print("" + str((guitare_count * 100) / (len(data) // frequency - 1) / 3) + "")
