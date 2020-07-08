import ctypes
import numpy as np

class SVM:
    def __init__(self, dll_path):
        self.lib = ctypes.CDLL(dll_path)
        self.initialize_rust_functions()
        self.model = None
        self.model_size = 0

    def initialize_rust_functions(self):
        self.lib.train_svm_model.restype = ctypes.POINTER(ctypes.c_double)
        self.lib.train_svm_model.argtypes = [
            ctypes.POINTER(ctypes.c_double),
            ctypes.POINTER(ctypes.c_double),
            ctypes.c_int,
            ctypes.c_int
        ]

        self.lib.predict_svm_model.restype = ctypes.c_double
        self.lib.predict_svm_model.argtypes = [
            ctypes.POINTER(ctypes.c_double),
            ctypes.POINTER(ctypes.c_double),
            ctypes.c_int
        ]

    def predict_svm_model(self, x):
        return self.lib.predict_svm_model(
            self.model,
            x.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            len(x))

    def train_svm_model(self, x, y):
        self.model = self.lib.train_svm_model(
            x.flatten().ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            y.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            x.shape[1],
            x.shape[0]
        )
        self.model_size = x.shape[1] + 1