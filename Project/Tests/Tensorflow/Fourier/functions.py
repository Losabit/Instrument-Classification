import csv
import os
import tensorflow as tf
import tensorflow.keras as keras
import numpy as np
import matplotlib.pyplot as plt
import random

from random import randrange

frequency_max = 8000
frequency_precision = 2
labels = ['guitare','saxo', 'piano']

def getCsvData(path):
    with open(path, newline='\n') as f:
        reader = csv.reader(f, delimiter=';')
        next(reader, None)
        data = [] 
        count = 0
        #0,1 -> 0 2,3 -> 1 4,5 -> 2
        for row in reader:
            if len(data) == (frequency_max * 2) / frequency_precision:
                break
            if count % frequency_precision != 0:
                count += 1
                continue
    
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
                if len(data_buff) == (frequency_max * 2) / frequency_precision:
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


class CustomCallback(keras.callbacks.Callback):
    def on_epoch_end(self, epoch, logs=None):
        epoch_values.append(epoch)
        loss_values.append(logs['loss'])
        val_loss_values.append(logs['val_loss'])

        acc_values.append(logs['accuracy'])
        val_acc_values.append(logs['val_accuracy'])

        if epoch % 10 == 0:
            plt.plot(epoch_values, loss_values, c="red")
            plt.plot(epoch_values, val_loss_values, c="blue")
            plt.plot(epoch_values, acc_values, c="green")
            plt.plot(epoch_values, val_acc_values, c="yellow")

epoch_values = []
loss_values = []
val_loss_values = []

acc_values = []
val_acc_values = []