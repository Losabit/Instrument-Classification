import tensorflow as tf
from tensorflow import keras
import os
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.image as mpimg
import random
from tensorflow.keras.preprocessing.image import ImageDataGenerator
from random import randrange
from callbacks import *


use_real_dataset = True
class_names = ['saxo', 'guitare']#, 'piano']
max_data_by_label = 1400
IMG_HEIGHT = 32
IMG_WIDTH = 32
ORIGINAL_IMG_HEIGHT = 109
ORIGINAL_IMG_WIDTH = 146
target_size = (IMG_WIDTH, IMG_HEIGHT)

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
                validation_labels.append(i)

train_images = np.array(train_images)
train_labels = np.array(train_labels)
validation_images = np.array(validation_images)
validation_labels = np.array(validation_labels)

train_images, train_labels = shuffle(train_images, train_labels)
validation_images, validation_labels = shuffle(validation_images, validation_labels)

if ORIGINAL_IMG_HEIGHT == IMG_HEIGHT and ORIGINAL_IMG_WIDTH == IMG_WIDTH:
    train_images = np.delete(train_images, 3, 3)
    validation_images = np.delete(validation_images, 3, 3)

print(train_labels)

'''
plt.figure(figsize=(10,10))
for i in range(25):
    plt.subplot(5,5,i+1)
    plt.xticks([])
    plt.yticks([])
    plt.grid(False)
    plt.imshow(train_images[i], cmap=plt.cm.binary)
    plt.xlabel(class_names[train_labels[i]])
plt.show()
'''


model = keras.models.Sequential([
    keras.layers.Conv2D(8, (3, 3), padding='same', activation=keras.activations.tanh),
    keras.layers.MaxPool2D(),
    keras.layers.Conv2D(16, (3, 3), padding='same', activation=keras.activations.tanh),
    keras.layers.MaxPool2D(),
    keras.layers.Conv2D(32, (3, 3), padding='same', activation=keras.activations.tanh),
    keras.layers.MaxPool2D(),
    keras.layers.Conv2D(64, (3, 3), padding='same', activation=keras.activations.tanh),
    keras.layers.MaxPool2D(),
    keras.layers.Flatten(),
    keras.layers.Dense(len(class_names), activation=keras.activations.softmax)
])


model.compile(optimizer='adam',
              loss=tf.keras.losses.SparseCategoricalCrossentropy(from_logits=True),
              metrics=['accuracy'])
model.fit(train_images, train_labels, validation_data=(validation_images, validation_labels), epochs=100,  callbacks=[CustomCallback(), keras.callbacks.TensorBoard(log_dir='logs/mlp_4conv_tanh_adam_2label_32x32')])
