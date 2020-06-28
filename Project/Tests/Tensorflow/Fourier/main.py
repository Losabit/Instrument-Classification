import csv
import os
import tensorflow as tf
import tensorflow.keras as keras
import numpy as np
import matplotlib.pyplot as plt

from tensorflow.keras.models import Sequential
from tensorflow.keras.layers import Dense, Activation

# Import Data
frequency_max = 20000
frequency_precision = 1
labels = ['guitare','piano', 'saxo']

np_array = np.array([])
print(np_array)
np.concatenate(np_array, [4,2])
np.concatenate(np_array, [4,2])
print(np_array)

def getCsvData(path):
    with open(path, newline='\n') as f:
        reader = csv.reader(f, delimiter=';')
        next(reader, None)
        data = [] 
        for row in reader:
            if float(row[0]) > frequency_max:
                break
            data.append([float(row[0]), float(row[1])])
        return data

def importData(path):
    data = []
    label = []
    for i in range(len(labels)):
        directory_path = os.path.join(path, labels[i])
        for r, _, f in os.walk(directory_path):
            for file in f:
                data.append(getCsvData(os.path.join(directory_path, file)))
                label.append(i)
    return (data, label)

# Ajouter le nombre de classe que l'on veut prendre
train_data, train_label = importData('/home/losabit/Desktop/PA/Instrument-Classification/Project/Tests/Tensorflow/Fourier/dataset/train')
validation_data, validation_label = importData('/home/losabit/Desktop/PA/Instrument-Classification/Project/Tests/Tensorflow/Fourier/dataset/validation')


# Linear classification
model = Sequential()
model.add(Dense(1, activation='linear', input_dim = 1))
model.compile(optimizer = 'rmsprop', loss = 'mean_squared_error', metrics = ['accuracy'])
model.fit(data,y, epochs= 1000, batch_size=0.1)
plt.plot(data, regr.predict(data), 'b', data,y, 'k.')