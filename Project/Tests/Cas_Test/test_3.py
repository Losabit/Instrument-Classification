# %% IMPORT
import numpy as np
import matplotlib.pyplot as plt
import sys
sys.path.append('../Lib/PythonML')
from linear import Linear

linearMod = Linear('../Lib/SupervisingML/target/debug/libmllib_rust.so')


X = np.array([[1, 0], [0, 1], [0, 0], [1, 1]])
Y = np.array([1, 1, -1, -1])

linearMod.init_linear_model(2)
flattened_X = X.flatten()
linearMod.train_linear_model_classification(flattened_X, Y, X.shape[0], 1000, 0.1)
test_points = np.array([[i, j] for i in range(50) for j in range(50)], dtype='float64') / 50.0

test_points_predicted = np.zeros(len(test_points))
red_points = []
blue_points = []
for k, test_input_k in enumerate(test_points):
    predicted_value = linearMod.predict_linear_model_classification(test_input_k)
    
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

plt.scatter(X[0:2, 0], X[0:2, 1], color='blue')
plt.scatter(X[2:4,0], X[2:4,1], color='red')
plt.show()
plt.clf()