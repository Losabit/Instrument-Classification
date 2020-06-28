import matplotlib.pyplot as plt
from scipy import signal
from scipy.io import wavfile

#path =  r"C:\Users\quent\Desktop\Projet Annuel\Ressources\ressources\Test\300_801.wav"
path = r"C:\Users\quent\Desktop\Projet Annuel\Ressources\ressources\musique\Lauryn-Hill-Doo-Wop-_That-Thing_-_Official-Video_ - Copie.wav"
frequency, data = wavfile.read(path)
if len(data.shape) != 1:
    data = data.sum(axis=1) / 2

#frequencies, times, spectrogram = signal.spectrogram(data, frequency)
#plt.pcolormesh(times, frequencies, spectrogram)
plt.specgram(data, Fs=frequency)
#plt.ylabel('Frequency [Hz]')
#plt.xlabel('Time [sec]')
plt.axis('off')
#plt.show()
plt.savefig("test2.png", bbox_inches='tight',  pad_inches = 0)
