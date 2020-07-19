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
from svm import SVM
from scipy.fftpack import fft
from scipy.io import wavfile 
from wavio import readwav

# argv[1] = /home/losabit/Desktop/OriginalData/guitare/Studio/smells-like-teen-spiritnirvana-covered-by-feng-e.wav
# argv[2]= 1
# argv[3] = Models/mlp.txt
#python //home/losabit/Desktop/PA/Instrument-Classification/Project/App/interfacev2/script.py /home/losabit/Desktop/OriginalData/guitare/Studio/smells-like-teen-spiritnirvana-covered-by-feng-e.wav 1 Models/mlp.txt
#python //home/losabit/Desktop/PA/Instrument-Classification/Project/App/interfacev2/script.py /home/losabit/Desktop/OriginalData/guitare/Studio/smells-like-teen-spiritnirvana-covered-by-feng-e.wav 2 Models/linear

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
    linear = Linear(dll_path)
    linear.init_linear_model(IMG_HEIGHT * IMG_WIDTH * 3)
    #linear.

frequency, _, data = readwav(inpath)
if len(data.shape) != 1:
    data = data.sum(axis=1) / 2

response = []
guitare_count = 0
saxo_count = 0
piano_count = 0
for i in range(len(data) // frequency - 1):
    outpath_file = os.path.join(outpath, 'tmp' + str(i) + '.png')
    new_data = data[i * frequency:(i+1) * frequency]
    plt.specgram(new_data, Fs=frequency)
    plt.axis('off')
    plt.savefig(outpath_file, bbox_inches='tight',  pad_inches = 0)
    plt.close()

    image = tf.keras.preprocessing.image.load_img(outpath_file, grayscale=False, color_mode='rgb', 
    target_size=target_size, interpolation='nearest')
    image_predict = np.array(tf.keras.preprocessing.image.img_to_array(image, data_format=None, dtype=None), dtype='float64')
    
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

shutil.rmtree(outpath)
'''
Return :
'''
print("" + str(piano_count / (len(data) // frequency - 1)) + "% a piano")
print("" + str(saxo_count / (len(data) // frequency - 1)) + "% a saxophone")
print("" + str(guitare_count / (len(data) // frequency - 1)) + "% a guitar")
