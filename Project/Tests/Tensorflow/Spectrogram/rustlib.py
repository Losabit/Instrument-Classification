import tensorflow as tf
import os
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.image as mpimg
import random
import sys

sys.path.append('../../../Lib/PythonML')
from linear import Linear
from multicouche import MLP
from tensorflow import keras
from random import randrange
from callbacks import *


use_real_dataset = True
class_names = ['saxo', 'guitare']#, 'piano']
max_data_by_label = 700
IMG_HEIGHT = 32
IMG_WIDTH = 32
ORIGINAL_IMG_HEIGHT = 109
ORIGINAL_IMG_WIDTH = 146
target_size = (IMG_WIDTH, IMG_HEIGHT)
dll_path = '../../../Lib/SupervisingML/target/debug/libmllib_rust.so'
last_couche = 1

train_images = []
validation_images = []
train_labels = []
validation_labels = []

def shuffle(data,label):
    for i in range(len(label)):
        rand = random.randrange(i,len(label))
        label[i], label[rand] = label[rand], label[i]
        data[i], data[rand] = data[rand], data[i]
    return (data,label)

def test(model, data, label, last_couche):
    good_predict = [0 for i in range(last_couche + 1)]
    number_of_predict = [0 for i in range(last_couche + 1)]
    for i in range(len(data)):
        predicted_value = model.predict_multicouche_model_classification(data[i])
        if last_couche == 1:
            predicted_value = predicted_value[0]
            if label[i] < 0.0:
                number_of_predict[0] += 1
                if predicted_value < 0.0:
                    good_predict[0] += 1
            else:
                number_of_predict[1] += 1
                if predicted_value >= 0.0:
                    good_predict[1] += 1
        else:
            predicted_list = [predicted_value[i] for i in range(last_couche)]
            print(predicted_list)
            max_ind = max_indice(predicted_list)  
            number_of_predict[int(label[i]) + 1] += 1
            if int(label[i]) == max_ind:
                good_predict[int(label[i]) +  1] += 1
    return (good_predict, number_of_predict)

def max_indice(values):
    max_value = values[0]
    max_ind = 0
    for i in range(1, len(values)):
        if values[i] > max_value:
            max_ind = i
            max_value = values[i]
    return max_ind

if use_real_dataset:
    train_path = '../../../Dataset/Train/Spectrogram'
    validation_path = '../../../Dataset/Test/Spectrogram'
else:
    train_path = '../Spectrogram/dataset/train'
    validation_path = '../Spectrogram/dataset/validation'


for i in range(len(class_names)):
    for r, _, f in os.walk(os.path.join(train_path, class_names[i])):
        for file in f:
            if len(train_images) == max_data_by_label * (i + 1):
                break
            else:
                if ORIGINAL_IMG_HEIGHT == IMG_HEIGHT and ORIGINAL_IMG_WIDTH == IMG_WIDTH:
                    train_images.append(mpimg.imread(os.path.join(r, file))) 
                else:
                    image = tf.keras.preprocessing.image.load_img(
                        os.path.join(r, file), grayscale=False, color_mode='rgb', target_size=target_size,
                        interpolation='nearest'
                    )
                    train_images.append(tf.keras.preprocessing.image.img_to_array(image, data_format=None, dtype=None) / 255)
                if i == 0 and len(class_names) == 2:
                    train_labels.append(-1)
                else:
                    train_labels.append(i)

    for r, _, f in os.walk(os.path.join(validation_path, class_names[i])):
        for file in f:
            if len(train_images) == max_data_by_label * (i + 1) // 2:
                break
            else:
                if ORIGINAL_IMG_HEIGHT == IMG_HEIGHT and ORIGINAL_IMG_WIDTH == IMG_WIDTH:
                    validation_images.append(mpimg.imread(os.path.join(r, file)))
                else:
                    image = tf.keras.preprocessing.image.load_img(
                        os.path.join(r, file), grayscale=False, color_mode='rgb', target_size=target_size,
                        interpolation='nearest'
                    )
                    validation_images.append(tf.keras.preprocessing.image.img_to_array(image, data_format=None, dtype=None) / 255)
                if i == 0 and len(class_names) == 2:
                    validation_labels.append(-1)
                else:
                    validation_labels.append(i)

train_images = np.array(train_images, dtype='float64')
train_labels = np.array(train_labels, dtype='float64')
validation_images = np.array(validation_images, dtype='float64')
validation_labels = np.array(validation_labels, dtype='float64')

print("End of Import")

train_images, train_labels = shuffle(train_images, train_labels)
validation_images, validation_labels = shuffle(validation_images, validation_labels)

if ORIGINAL_IMG_HEIGHT == IMG_HEIGHT and ORIGINAL_IMG_WIDTH == IMG_WIDTH:
    train_images = np.delete(train_images, 3, 3)
    validation_images = np.delete(validation_images, 3, 3)

mlp = MLP(dll_path)
mlp.init_multicouche_model(np.array([IMG_HEIGHT * IMG_WIDTH * 3, 512, 64, 32, last_couche], dtype='float64'))
print("Start Training")
mlp.train_multicouche_model_classification(train_images.flatten(), train_labels.flatten(), train_images.shape[0], 2000.0, 0.2)


good_predict, number_of_predict = test(mlp, validation_images, validation_labels)
print("Test :")
print(good_predict)
print(number_of_predict)
'''
print("Predict of -1.0 : " + str(good_predict[0]) + " of " + str(number_of_predict[0]))
print("Predict of 1.0 : " + str(good_predict[1]) + " of " + str(number_of_predict[1]))
'''
