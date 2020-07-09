import ctypes
import enum
import numpy as np

class SVM:

    class Kernel(enum.Enum):
        BASIC = 1
        RBF = 2
        
    def __init__(self, dll_path):
        self.lib = ctypes.CDLL(dll_path)
        self.initialize_rust_functions()
        self.model = None
        self.model_size = 0

    def initialize_rust_functions(self):
        self.lib.train_svm_model_basic_kernel.restype = ctypes.POINTER(ctypes.c_double)
        self.lib.train_svm_model_basic_kernel.argtypes = [
            ctypes.POINTER(ctypes.c_double),
            ctypes.POINTER(ctypes.c_double),
            ctypes.c_int,
            ctypes.c_int
        ]

        self.lib.train_svm_model_rbf_kernel.restype = ctypes.POINTER(ctypes.c_double)
        self.lib.train_svm_model_rbf_kernel.argtypes = [
            ctypes.POINTER(ctypes.c_double),
            ctypes.POINTER(ctypes.c_double),
            ctypes.c_int,
            ctypes.c_int,
            ctypes.c_double,
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

    def train_svm_model_classification(self, x, y, kernel, *args):
        if kernel == self.Kernel.BASIC:
            self.train_svm_model_basic_kernel(x,y)
        elif kernel == self.Kernel.RBF:
            self.train_svm_model_rbf_kernel_classification(x,y,args[0])

    def train_svm_model_regression(self, x, y, kernel, *args):
        if kernel == self.Kernel.BASIC:
            self.train_svm_model_basic_kernel(x,y)
        elif kernel == self.Kernel.RBF:
            self.train_svm_model_rbf_kernel_regression(x,y,args[0])
        
    def train_svm_model_basic_kernel(self, x, y):
        self.model = self.lib.train_svm_model_basic_kernel(
            x.flatten().ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            y.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            x.shape[1],
            x.shape[0]
        )
        self.model_size = x.shape[1] + 1

    def train_svm_model_rbf_kernel_classification(self, x, y, gamma):
        self.model = self.lib.train_svm_model_rbf_kernel(
            x.flatten().ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            y.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            x.shape[1],
            x.shape[0],
            gamma,
            1
        )
        self.model_size = x.shape[1] + 1
    
    def train_svm_model_rbf_kernel_regression(self, x, y, gamma):
        self.model = self.lib.train_svm_model_rbf_kernel(
            x.flatten().ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            y.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            x.shape[1],
            x.shape[0],
            gamma,
            0
        )
        self.model_size = x.shape[1] + 1

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