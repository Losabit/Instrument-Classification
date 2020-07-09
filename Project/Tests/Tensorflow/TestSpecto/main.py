import os
import numpy as np
import matplotlib.image as mpimg
import random
from tensorflow import keras

traning_path = "D:\Dev\Instrument-Classification\Project\Tests\Tensorflow\TestSpecto\dataset/train"
validation_path = "D:\Dev\Instrument-Classification\Project\Tests\Tensorflow\TestSpecto\dataset/validation"
class_names = ['piano', 'saxophone', 'guitare']
extension = '.png'
IMGH = 64
IMGW = 64
train_images = []
validation_images = []
train_labels = []
validation_labels = []
for i in range(len(class_names)):
    for r, _, f in os.walk(os.path.join(traning_path, class_names[i])):
        for file in f:
            if extension in file:
                if len(train_images) == 0:
                    train_images.append(mpimg.imread(os.path.join(r, file)))
                    train_labels.append(i)
    for r, _, f in os.walk(os.path.join(validation_path, class_names[i])):
        for file in f:
            if extension in file:
                continue
                validation_images.append(mpimg.imread(os.path.join(r, file)))
                validation_labels.append(i)
print(len(train_images))
for i in range(len(train_labels)):
    rand = random.randrange(i, len(train_labels))
    train_images[i], train_images[rand] = train_images[rand], train_images[rand]
    train_labels[i], train_labels[rand] = train_labels[rand], train_labels[rand]
for i in range(len(validation_labels)):
    rand = random.randrange(i, len(validation_labels))
    validation_images[i], validation_images[rand] = validation_images[rand], validation_images[rand]
    validation_labels[i], validation_labels[rand] = validation_labels[rand], validation_labels[rand]
arraynp = np.array(train_images)
print(arraynp)
# model = keras.Sequential([keras.layers.Flatten(input_shape=(IMGH, IMGW)),
#                           keras.layers.Dense(140, activation='relu'),
#                           keras.layers.Dense(10)])
# model.compile(loss=keras.losses.mean_squared_error, optimizer=keras.optimizers.Adam(),
#               metrics=[keras.metrics.categorical_accuracy])
# model.fit(np.array(train_images), np.array(train_labels), epochs=100)
