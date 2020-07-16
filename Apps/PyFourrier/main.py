import os
import sys
import csv
import numpy as np
import matplotlib.pyplot as plt
from scipy.fftpack import fft
from scipy.io import wavfile 
from enum import Enum
from wavio import readwav

'''
https://stackoverflow.com/questions/23377665/python-scipy-fft-wav-files
https://docs.scipy.org/doc/scipy/reference/tutorial/fft.html
'''

class ExtractMode(Enum):
    CSV = 1
    PNG = 2

class OperationMode(Enum):
    Fourier = 1
    Spectrogram = 2



inpath = '/home/losabit/Desktop/toimport/guitare/'
outpath = '/home/losabit/Desktop/toimport/spectro/guitare/'
filename = 'Guitare'
mode = ExtractMode.PNG
operation = OperationMode.Spectrogram

files = []
for r, _, f in os.walk(inpath):
    for file in f:
        if '.wav' in file:
            files.append(os.path.join(r, file))

for i in range(len(files)):
    frequency, _, data = readwav(files[i])
    if len(data.shape) != 1:
        data = data.sum(axis=1) / 2

    if operation == OperationMode.Spectrogram:
        plt.specgram(data, Fs=frequency)
        plt.axis('off')
        plt.savefig(outpath + filename + str(i) + ".png", bbox_inches='tight',  pad_inches = 0)
        plt.close()

    elif operation == OperationMode.Fourier:
        seconds  = len(data) / frequency
        t = 1.0 / frequency
        fftData = fft(data) 
        xf = np.linspace(0.0, 1.0/(2.0*t), len(data)//2)
        yf = 2.0/len(data) * np.abs(fftData[0:len(data)//2])

        if(mode == ExtractMode.CSV and len(xf) == len(yf)):
            csvFile = open(outpath + filename + str(i) + ".csv", 'w')
            headers = ['Frequence', 'Amplitude']
            writer = csv.DictWriter(csvFile, fieldnames=headers, delimiter=';',lineterminator='\n')
            writer.writeheader()
            for i in range(len(xf)):
                writer.writerow({headers[0] : xf[i], headers[1] : yf[i] })
            csvFile.close()
        
        if(mode == ExtractMode.PNG):
            plt.plot(xf, yf)
            plt.axis('off')
            plt.savefig(outpath + filename + str(i) + ".png", bbox_inches='tight',  pad_inches = 0)
            plt.close()