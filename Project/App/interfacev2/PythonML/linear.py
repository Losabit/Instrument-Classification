import ctypes
import numpy as np


class Linear:
    def __init__(self, dll_path):
        self.lib = ctypes.CDLL(dll_path)
        self.initialize_rust_functions()
        self.model = None
        self.model_size = 0

    def initialize_rust_functions(self):
        self.lib.init_linear_model.argtypes = [ctypes.c_int]
        self.lib.init_linear_model.restype = ctypes.POINTER(ctypes.c_double)

        self.lib.predict_linear_model_regression.restype = ctypes.c_double
        self.lib.predict_linear_model_regression.argtypes = [
            ctypes.POINTER(ctypes.c_double),
            ctypes.POINTER(ctypes.c_double),
            ctypes.c_int
        ]

        self.lib.predict_linear_model_classification.restype = ctypes.c_double
        self.lib.predict_linear_model_classification.argtypes = [
            ctypes.POINTER(ctypes.c_double),
            ctypes.POINTER(ctypes.c_double),
            ctypes.c_int
        ]

        self.lib.train_linear_model_classification.restype = ctypes.POINTER(ctypes.c_double)
        self.lib.train_linear_model_classification.argtypes = [
            ctypes.POINTER(ctypes.c_double),
            ctypes.POINTER(ctypes.c_double),
            ctypes.POINTER(ctypes.c_double),
            ctypes.c_int,
            ctypes.c_int,
            ctypes.c_int,
            ctypes.c_double
        ]

        self.lib.train_linear_model_regression.restype = ctypes.POINTER(ctypes.c_double)
        self.lib.train_linear_model_regression.argtypes = [
            ctypes.POINTER(ctypes.c_double),
            ctypes.POINTER(ctypes.c_double),
            ctypes.c_int
        ]

    def init_linear_model(self, size):
        self.model = self.lib.init_linear_model(ctypes.c_int(size))
        self.model_size = size + 1

    def predict_linear_model_classification(self, points):
        return self.lib.predict_linear_model_classification(self.model,
                                                            points.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
                                                            len(points))

    def predict_linear_model_regression(self, points):
        return self.lib.predict_linear_model_regression(
            self.model,
            points.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            len(points)
        )

    def train_linear_model_classification(self, x, y, sample_size, nb_iter, alpha):
        self.model = self.lib.train_linear_model_classification(
            self.model,
            x.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            y.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            sample_size,
            self.model_size - 1,
            nb_iter,
            alpha
        )

    def train_linear_model_regression(self, x, y, x_size):
        self.model = self.lib.train_linear_model_regression(
            x.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            y.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            x_size
        )
        self.model_size = x_size // 2
        return [self.model[i] for i in range(self.model_size)]

    def save_model(self, path):
        model_list = [str(self.model[i]) for i in range(self.model_size)]
        model_string = ';'.join(model_list)
        with open(path, 'w') as file:
            file.write(model_string)

    def load_model(self, path):
        with open(path, 'r') as file:
            model = file.readlines()[0]
            model = model.split(';')
            self.model = np.array([float(model[i]) for i in range(len(model))],  dtype='float64')
            self.model = self.model.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
            self.model_size = len(model)