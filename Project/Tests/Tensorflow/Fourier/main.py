import csv
import os
import tensorflow as tf
import tensorflow.keras as keras
import numpy as np
import matplotlib.pyplot as plt
import random

from functions import *
from random import randrange
from tensorflow.keras.utils import to_categorical
from tensorflow.keras.models import Sequential
from tensorflow.keras.layers import Dense, Activation, Conv2D, MaxPooling2D

# Import Data
train_max_by_label = 400
use_real_dataset = True
if use_real_dataset:
    train_path = '../../../Dataset/Train/Fourier'
    validation_path = '../../../Dataset/Test/Fourier'
else:
    train_path = '../Fourier/dataset/train'
    validation_path = '../Fourier/dataset/validation'


train_data, train_label = importData(train_path, 3, train_max_by_label)
validation_data, validation_label = importData(validation_path, 2, train_max_by_label // 2)
print("Import Finish")

train_data, train_label = shuffle(train_data, train_label)
validation_data, validation_label = shuffle(validation_data, validation_label)

# Linear classification
'''
label_categorical = to_categorical(label)

model = Sequential()
model.add(keras.layers.Flatten(input_shape=train_data.shape[1:]))
model.add(Dense(1, activation='tanh'))
#model.summary()
model.compile(optimizer = 'adam',  loss="binary_crossentropy")
history = model.fit(data, label, validation_split = 0.1, epochs= 100)

plt.plot(history.history['loss'])
plt.plot(history.history['val_loss'])
plt.title('model loss')
plt.ylabel('loss')
plt.xlabel('epoch')
plt.legend(['train', 'test'], loc='upper left')
plt.show()
'''
#MLP
model = keras.Sequential([
    keras.layers.Flatten(input_shape=train_data.shape[1:]),
    keras.layers.Dense(128, activation='tanh'),
    keras.layers.Dense(128, activation='tanh'),
    keras.layers.Dense(128,activation='tanh'),
    keras.layers.Dense(5)
])

model.compile(optimizer='adam',
              loss=tf.keras.losses.SparseCategoricalCrossentropy(from_logits=True),
              metrics=['accuracy'])
model.summary()
history = model.fit(train_data, train_label, validation_data=(validation_data, validation_label), epochs=1000)

plt.plot(history.history['accuracy'])
plt.plot(history.history['val_accuracy'])
plt.title('model accuracy')
plt.ylabel('accuracy')
plt.xlabel('epoch')
plt.legend(['train', 'test'], loc='upper left')
plt.show()

plt.plot(history.history['loss'])
plt.plot(history.history['val_loss'])
plt.title('model loss')
plt.ylabel('loss')
plt.xlabel('epoch')
plt.legend(['train', 'test'], loc='upper left')
plt.show()