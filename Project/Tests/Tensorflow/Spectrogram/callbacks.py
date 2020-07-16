import tensorflow as tf
from tensorflow import keras
import numpy as np
import matplotlib.pyplot as plt


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