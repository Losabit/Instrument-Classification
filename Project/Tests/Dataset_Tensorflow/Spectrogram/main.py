import tensorflow as tf
from tensorflow import keras
import os
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.image as mpimg
import random
from random import randrange

train_path = 'C:\\Users\\quent\\Desktop\\Projet Annuel\\Applis\\TestDataset\\ImageClassification\\dataset\\train'
validation_path = 'C:\\Users\\quent\\Desktop\\Projet Annuel\\Applis\\TestDataset\\ImageClassification\\dataset\\validation'
class_names = ['saxo', 'piano', 'guitare']
IMG_HEIGHT = 109
IMG_WIDTH = 146
extension = '.png'
train_images = []
validation_images = []
train_labels = []
validation_labels = []

for i in range(len(class_names)):
    for r, _, f in os.walk(os.path.join(train_path, class_names[i])):
        for file in f:
            if extension in file:
                train_images.append(mpimg.imread(os.path.join(r, file)))
                train_labels.append(i)
    for r, _, f in os.walk(os.path.join(validation_path, class_names[i])):
        for file in f:
            if extension in file:
                validation_images.append(mpimg.imread(os.path.join(r, file)))
                validation_labels.append(i)

for i in range(len(train_labels)):
    rand = random.randrange(i,len(train_labels))
    train_images[i], train_images[rand] = train_images[rand], train_images[rand]
    train_labels[i], train_labels[rand] = train_labels[rand], train_labels[rand]

for i in range(len(validation_labels)):
    rand = random.randrange(i,len(validation_labels))
    validation_images[i], validation_images[rand] = validation_images[rand], validation_images[rand]
    validation_labels[i], validation_labels[rand] = validation_labels[rand], validation_labels[rand]

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

model = keras.Sequential([
    keras.layers.Flatten(input_shape=(IMG_HEIGHT, IMG_WIDTH)),
    keras.layers.Dense(128, activation='relu'),
    keras.layers.Dense(3)
])

model.compile(optimizer='adam',
              loss=tf.keras.losses.SparseCategoricalCrossentropy(from_logits=True),
              metrics=['accuracy'])

model.fit(np.array(train_images), np.array(train_labels), epochs=10)
test_loss, test_acc = model.evaluate(validation_images,  validation_labels, verbose=2)

print('\nTest accuracy:', test_acc)