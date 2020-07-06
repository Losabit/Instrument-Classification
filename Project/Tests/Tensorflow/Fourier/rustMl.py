import csv
import os
import numpy as np
import matplotlib.pyplot as plt
import sys
sys.path.append('../../Lib/PythonML')
from linear import Linear
from multicouche import MLP

# Import Data
frequency_max = 12000
frequency_precision = 1
labels = ['guitare','saxo', 'piano']
dll_path = '../../Lib/SupervisingML/target/debug/libmllib_rust.so'

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
    for i in range(label_number):
        directory_path = os.path.join(path, labels[i])
        for r, _, f in os.walk(directory_path):
            for file in f:
                data.append(getCsvData(os.path.join(directory_path, file)))
                if i == 0:
                    label.append(-1)
                else:
                    label.append(i)
    return (np.array(data, dtype='float64'), np.array(label, dtype='float64'))


train_data, train_label = importData('Fourier/dataset/train', 2)
validation_data, validation_label = importData('Fourier/dataset/validation', 2)
print("Import finish")
print(train_data.shape)
# Linear classification
'''
linear = Linear(dll_path)
linear.init_linear_model(frequency_max * 2 * frequency_precision * 2)
linear.train_linear_model_classification(train_data.flatten(), train_label.flatten(), len(train_label), 10000, 0.03)

for i in range(len(validation_data)):
    predicted_value = linear.predict_linear_model_classification(validation_data[i])
    print("predicted result : " + str(predicted_value) + " / result : " + str(validation_label[i]))
'''

#MLP
mlp = MLP(dll_path)
mlp.init_multicouche_model(np.array([train_data.shape[1] * train_data.shape[2],1], dtype='float64'))
mlp.train_multicouche_model_classification(train_data.flatten(), train_label.flatten(), train_data.shape[0], 10000.0, 0.03)

for i in range(len(validation_data)):
    predicted_value = mlp.predict_multicouche_model_classification(validation_data)[0]
    print("predicted result : " + str(predicted_value) + " / result : " + str(validation_label[i]))
