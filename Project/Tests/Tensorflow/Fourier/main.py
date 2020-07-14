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
frequency_max = 8000
frequency_precision = 1
labels = ['guitare','saxo', 'piano']
dll_path = '../../../Lib/SupervisingML/target/debug/libmllib_rust.so'
train_max_by_label = 300
use_real_dataset = True
if use_real_dataset:
    train_path = '../../../Dataset/Train/Fourier'
    validation_path = '../../../Dataset/Test/Fourier'
else:
    train_path = '../Fourier/dataset/train'
    validation_path = '../Fourier/dataset/validation'

def getCsvData(path):
    with open(path, newline='\n') as f:
        reader = csv.reader(f, delimiter=';')
        next(reader, None)
        data = [] 
        count = 0
        #0,1 -> 0 2,3 -> 1 4,5 -> 2
        for row in reader:
            if len(data) == (frequency_max * 2 * frequency_precision):
                break
                
            frequency = float(row[0])
            if frequency > frequency_max:
                break
            if int(frequency) * 2 + 1 < count:
                continue
            count += 1
            data.append([float(row[0]), float(row[1])])
        return data

def importData(path, label_number = len(labels), max_by_label = -1):
    data = []
    label = []
    if label_number > len(labels):
        label_number = len(labels)
    for i in range(label_number):
        data_count_by_label = 0
        directory_path = os.path.join(path, labels[i])
        for r, _, f in os.walk(directory_path):
            for file in f:
                if data_count_by_label == max_by_label and max_by_label != -1:
                    break
                
                data_buff = getCsvData(os.path.join(directory_path, file))
                if len(data_buff) == (frequency_max * 2 * frequency_precision):
                    data.append(data_buff)
                    if i == 0 and label_number == 2:
                        label.append(-1)
                    else:
                        label.append(i)
                    data_count_by_label += 1

    return (np.array(data, dtype='float64'), np.array(label, dtype='float64'))

def shuffle(data,label):
    for i in range(len(label)):
        rand = random.randrange(i,len(label))
        label[i], label[rand] = label[rand], label[i]
        data[i], data[rand] = data[rand], data[i]
    return (data,label)

def test(data, label):
    good_predict = [0,0]
    number_of_predict = [0,0]
    for i in range(len(data)):
        predicted_value = linear.predict_linear_model_classification(data[i])
        if label[i] == -1.0:
            number_of_predict[0] += 1
            if predicted_value == -1.0:
                good_predict[0] += 1
        else:
            number_of_predict[1] += 1
            if predicted_value == 1.0:
                good_predict[1] += 1
    return (good_predict, number_of_predict)




train_data, train_label = importData(train_path, 2, train_max_by_label)
validation_data, validation_label = importData(validation_path, 2, train_max_by_label)
print("Import Finish")

train_data, train_label = shuffle(train_data, train_label)
validation_data, validation_label = shuffle(validation_data, validation_label)  
data = np.concatenate([train_data, validation_data])
label = np.concatenate([train_label, validation_label])
print(train_data.shape)
print(train_label.shape)


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
'''
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
'''