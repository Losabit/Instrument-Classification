import csv
import os
import random


#12.8Go
path = '/home/losabit/Desktop/PA/Instrument-Classification/Project/Dataset/Train/Fourier/guitare/'    
max_frequency = 18000

def getCsvData(path):
    with open(path, newline='\n') as f:
        reader = csv.reader(f, delimiter=';')
        next(reader, None)
        data = [] 
        for row in reader:
            if float(row[0]) > max_frequency:
                break
            data.append([float(row[0]), float(row[1])])
        return data

def createCsvFile(path, data):
    if os.path.isfile(path):
        os.remove(path)
    csvFile = open(path, 'w')
    headers = ['Frequence', 'Amplitude']
    writer = csv.DictWriter(csvFile, fieldnames=headers, delimiter=';',lineterminator='\n')
    writer.writeheader()
    for i in range(len(data)):
        writer.writerow({headers[0] : data[i][0], headers[1] : data[i][1] })
    csvFile.close()

for r, _, f in os.walk(path):
    for file in f:
        if '.csv' in file:
            file_path = os.path.join(path, file)
            data = getCsvData(file_path)
            for i in range(len(data)):
                data[i][0] = float('{:.1f}'.format(data[i][0]))
                data[i][1] = float('{:.3f}'.format(data[i][1]))
            createCsvFile(file_path, data)

   