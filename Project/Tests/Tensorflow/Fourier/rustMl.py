import csv
import os
import numpy as np
import matplotlib.pyplot as plt
import sys
sys.path.append('../../Lib/PythonML')
from linear import Linear

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



# Linear classification
train_data, train_label = importData('Fourier/dataset/train', 2)
validation_data, validation_label = importData('Fourier/dataset/validation', 2)

linear = Linear('../../Lib/SupervisingML/target/debug/libmllib_rust.so')
linear.init_linear_model(frequency_max * 2 * frequency_precision * 2)
linear.train_linear_model_classification(train_data, train_label, 1000, 0.1)
for i in range(len(validation_data)):
    print("predicted result : " + str(linear.predict_linear_model_classification(validation_data[i])) + " / result : " + str(validation_label[i]))