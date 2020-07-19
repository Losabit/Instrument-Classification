import ctypes
import enum
import numpy as np

class RBF:      
    def __init__(self, dll_path):
        self.lib = ctypes.CDLL(dll_path)
        self.initialize_rust_functions()
        self.model = None
        self.model_size = 0
        self.x = None
        self.gamma = 0

    def initialize_rust_functions(self):
        self.lib.train_rbf.restype = ctypes.POINTER(ctypes.c_double)
        self.lib.train_rbf.argtypes = [
            ctypes.POINTER(ctypes.c_double),
            ctypes.POINTER(ctypes.c_double),
            ctypes.c_double,
            ctypes.c_double,
            ctypes.c_double
        ]

        self.lib.predict_rbf.restype = ctypes.c_double
        self.lib.predict_rbf.argtypes = [
            ctypes.POINTER(ctypes.c_double),
            ctypes.POINTER(ctypes.c_double),
            ctypes.POINTER(ctypes.c_double),
            ctypes.c_double,
            ctypes.c_double,
            ctypes.c_double
        ]

    def predict_rbf_model(self, x):
        return self.lib.predict_rbf(
            self.model,
            self.x.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            x.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            self.gamma,
            self.model_size,
            len(x)
        )
        
    def train_rbf_model(self, x, y, gamma):
        self.model = self.lib.train_rbf(
            x.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            y.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            x.shape[0], 
            x.shape[1],
            gamma
        )
        self.model_size = x.shape[0]
        self.x = x
        self.gamma = gamma

    def save_model(self, path):
        model_list = [str(self.model[i]) for i in range(self.model_size)]
        model_string = ';'.join(model_list)
        x_list = [str(value) for value in self.x.flatten()]
        x_string = ';'.join(x_list)
        with open(path, 'w') as file:
            file.write(model_string + '\n')
            file.write(x_string + '\n')
            file.write(str(self.gamma))

    def load_model(self, path):
        with open(path, 'r') as file:
            lines = file.readlines()
            model = lines[0].split(';')
            self.model = np.array([float(model[i]) for i in range(len(model))],  dtype='float64')
            self.model = self.model.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
            self.model_size = len(model)  

            x_start = lines[1].split(';')
            self.x = np.array([int(x_start[i]) for i in range(len(x_start))],  dtype='float64')
            self.gamma = float(lines[2])