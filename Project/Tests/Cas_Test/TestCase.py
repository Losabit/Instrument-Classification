# %% IMPORT
import numpy as np
import matplotlib.pyplot as plt
from linear import *

linearMod = Linear("C:/Users/marvi/PycharmProjects/Cas_Test/mllib_rust.dll")
model = linearMod.init_linear_model(2)
X = np.array([
    [1, 1],
    [2, 3],
    [3, 3]
], dtype='float64')
Y = np.array([
    1,
    -1,
    -1
], dtype='float64')
flattened_X = X.flatten()
linearMod.train_linear_model_classification(flattened_X, Y, X.shape[1], 1000, 0.01)
test_points = np.array([[i, j] for i in range(50) for j in range(50)], dtype='float64') / 50.0 * 2.0 + 1.0
test_points_predicted = np.zeros(len(test_points))
red_points = []
blue_points = []
for k, test_input_k in enumerate(test_points):
    # print(test_points)
    predicted_value = linearMod.predict_linear_model_classification(
        test_input_k)
    # print(predicted_value)
    if predicted_value == 1.0:
        blue_points.append(test_input_k)
    else:
        red_points.append(test_input_k)

red_points = np.array(red_points)
blue_points = np.array(blue_points)

if len(red_points) > 0:
    plt.scatter(red_points[:, 0], red_points[:, 1], color='red', alpha=0.5, s=2)
if len(blue_points) > 0:
    plt.scatter(blue_points[:, 0], blue_points[:, 1], color='blue', alpha=0.5, s=2)
plt.scatter(X[0, 0], X[0, 1], color='blue', s=10)
plt.scatter(X[1:3, 0], X[1:3, 1], color='red', s=10)
plt.show()
plt.clf()