import csv
import os
import tensorflow as tf
import tensorflow.keras as keras
import numpy as np
import matplotlib.pyplot as plt
import random

from random import randrange
from tensorflow.keras.utils import to_categorical
from tensorflow.keras.models import Sequential
from tensorflow.keras.layers import Dense, Activation, Conv2D, MaxPooling2D

# Import Data
frequency_max = 12000
frequency_precision = 1
labels = ['guitare','saxo', 'piano']

def getCsvData(path):
    with open(path, newline='\n') as f:
        reader = csv.reader(f, delimiter=';')
        next(reader, None)
        data = [] 
        count = 0
        #0,1 -> 0 2,3 -> 1 4,5 -> 2
        for row in reader:
            if len(data) == (frequency_max * 2 * frequency_precision):
                #raise ValueError('Bad data size expected ' + str(frequency_max * 2 * frequency_precision) + ' and found ' + str(len(data)) + ' for ' + path)
                break

            frequency = float(row[0])
            if frequency > frequency_max:
                break
            if int(frequency) * 2 + 1 < count:
                continue
            count += 1
            data.append([float(row[0]), float(row[1])])
        return data

def importData(path, label_number = len(labels)):
    data = []
    label = []
    if label_number > len(labels):
        label_number = len(labels)
      
    for r, d, f in os.walk(path):
        for i in range(label_number):
            for folder in d:
                directory_path = os.path.join(path, folder, labels[i])
                for r2, d2, f2 in os.walk(directory_path):
                    for file in f2:
                        data.append(getCsvData(os.path.join(directory_path, file)))
                        label.append(i)
                    
    return (np.array(data), np.array(label))



data, label = importData('/home/losabit/Desktop/PA/Instrument-Classification/Project/Tests/Tensorflow/Fourier/dataset')
print("Import Finish")

for i in range(len(label)):
    rand = random.randrange(i,len(label))
    label[i], label[rand] = label[rand], label[i]

# Linear classification
'''
train_label = to_categorical(train_label)

model = Sequential()
model.add(Dense(1, activation='linear', input_shape=train_data.shape[1:]))
model.summary()
model.compile(optimizer = 'adam', loss = 'categorical_crossentropy')
model.fit(train_data, train_label, epochs= 1000)
#plt.plot(data, regr.predict(data), 'b', data,y, 'k.')'''

#MLP
model = keras.Sequential([
    keras.layers.Flatten(input_shape=data.shape[1:]),
    keras.layers.Dense(12, activation='relu'),
    keras.layers.Dense(3)
])

model.compile(optimizer='adam',
              loss=tf.keras.losses.SparseCategoricalCrossentropy(from_logits=True),
              metrics=['accuracy'])
model.summary()
history = model.fit(data, label, validation_split = 0.1, epochs=100)

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