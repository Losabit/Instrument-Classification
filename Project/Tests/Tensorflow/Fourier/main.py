import csv
import os
import tensorflow as tf
import tensorflow.keras as keras
import numpy as np
import matplotlib.pyplot as plt

from tensorflow.keras.utils import to_categorical
from tensorflow.keras.models import Sequential
from tensorflow.keras.layers import Dense, Activation

# Import Data
frequency_max = 20000
frequency_precision = 1
labels = ['guitare','piano', 'saxo']

def getCsvData(path):
    with open(path, newline='\n') as f:
        reader = csv.reader(f, delimiter=';')
        next(reader, None)
        data = [] 
        count = 0
        #0,1 -> 0 2,3 -> 1 4,5 -> 2
        for row in reader:
            frequency = float(row[0])
            if frequency > frequency_max:
                break
            if int(frequency) * 2 + 1 < count:
                continue
            count += 1
            data.append([float(row[0]), float(row[1])])
        if len(data) != (frequency_max * 2 * frequency_precision):
            raise ValueError('Bad data size expected ' + str(frequency_max * 2 * frequency_precision) + ' and found ' + str(len(data)) + ' for ' + path)
        return data

def importData(path, label_number = len(labels)):
    data = []
    label = []
    if label_number > len(labels):
        label_number = len(labels)
    for i in range(label_number):
        directory_path = os.path.join(path, labels[i])
        for r, _, f in os.walk(directory_path):
            for file in f:
                data.append(getCsvData(os.path.join(directory_path, file)))
                label.append(i)
    return (np.array(data), np.array(label))


'''
train_data, train_label = importData('/home/losabit/Desktop/PA/Instrument-Classification/Project/Tests/Tensorflow/Fourier/dataset/train')
validation_data, validation_label = importData('/home/losabit/Desktop/PA/Instrument-Classification/Project/Tests/Tensorflow/Fourier/dataset/validation')
'''
train_data, train_label = importData("C:\\Users\\quent\\Desktop\\Projet Annuel\\Instrument-Classification\\Project\\Tests\\Tensorflow\\Fourier\\dataset\\train", 2)
#validation_data, validation_label = importData('/home/losabit/Desktop/PA/Instrument-Classification/Project/Tests/Tensorflow/Fourier/dataset/validation')

# Linear classification
train_data = to_categorical(train_data)
train_label = to_categorical(train_label)

model = Sequential()
model.add(Dense(1, activation='linear', input_shape=train_data.shape[1:]))
model.compile(optimizer = 'adam', loss = 'categorical_crossentropy')
model.fit(train_data, train_label, epochs= 1000, batch_size=0.1)
#plt.plot(data, regr.predict(data), 'b', data,y, 'k.')'''