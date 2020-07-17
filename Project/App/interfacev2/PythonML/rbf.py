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