# %% IMPORT
import numpy as np
import matplotlib.pyplot as plt
import sys
sys.path.append('../Lib/PythonML')
from linear import Linear

model_one = Linear('../Lib/SupervisingML/target/debug/libmllib_rust.so')
model_two = Linear('../Lib/SupervisingML/target/debug/libmllib_rust.so')
model_three = Linear('../Lib/SupervisingML/target/debug/libmllib_rust.so')

X = np.random.random((500, 2)) * 2.0 - 1.0
Y = np.array([[1, 0, 0] if -p[0] - p[1] - 0.5 > 0 and p[1] < 0 and p[0] - p[1] - 0.5 < 0 else 
              [0, 1, 0] if -p[0] - p[1] - 0.5 < 0 and p[1] > 0 and p[0] - p[1] - 0.5 < 0 else 
              [0, 0, 1] if -p[0] - p[1] - 0.5 < 0 and p[1] < 0 and p[0] - p[1] - 0.5 > 0 else 
              [0, 0, 0]for p in X])


flattened_X = X.flatten()

model_one.init_linear_model(2)
model_one.train_linear_model_classification(flattened_X, np.array([1 if y == [1, 0, 0] else -1 for y in Y.tolist()]), X.shape[0], 1000, 0.1)

model_two.init_linear_model(2)
model_two.train_linear_model_classification(flattened_X, np.array([1 if y == [0, 1, 0]  else -1 for y in Y.tolist()]), X.shape[0], 1000, 0.1)

model_three.init_linear_model(2)
model_three.train_linear_model_classification(flattened_X, np.array([1 if y == [0, 0, 1] else -1 for y in Y.tolist()]), X.shape[0], 1000, 0.1)

models = [model_one, model_two, model_three]

for model in models:
    test_points = np.array([[i, j] for i in range(50) for j in range(50)], dtype='float64') / 50.0 * 2.0 - 1.0
    test_points_predicted = np.zeros(len(test_points))
    red_points = []
    blue_points = []
    for k, test_input_k in enumerate(test_points):
        predicted_value = model.predict_linear_model_classification(test_input_k)
        
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

    plt.scatter(np.array(list(map(lambda elt : elt[1], filter(lambda c: Y[c[0]][0] == 1, enumerate(X)))))[:,0], np.array(list(map(lambda elt : elt[1], filter(lambda c: Y[c[0]][0] == 1, enumerate(X)))))[:,1], color='blue')
    plt.scatter(np.array(list(map(lambda elt : elt[1], filter(lambda c: Y[c[0]][1] == 1, enumerate(X)))))[:,0], np.array(list(map(lambda elt : elt[1], filter(lambda c: Y[c[0]][1] == 1, enumerate(X)))))[:,1], color='red')
    plt.scatter(np.array(list(map(lambda elt : elt[1], filter(lambda c: Y[c[0]][2] == 1, enumerate(X)))))[:,0], np.array(list(map(lambda elt : elt[1], filter(lambda c: Y[c[0]][2] == 1, enumerate(X)))))[:,1], color='green')
    plt.show()
    plt.clf()