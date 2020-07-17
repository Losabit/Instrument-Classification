import ctypes
import enum
import numpy as np
import cvxopt
from sklearn.svm import LinearSVC
from sklearn.metrics import confusion_matrix

class SVM:

    class Kernel(enum.Enum):
        BASIC = 1
        RBF = 2
        POLYNOMIAL = 3
        
    def __init__(self, dll_path):
        self.lib = ctypes.CDLL(dll_path)
        self.initialize_rust_functions()
        self.model = None
        self.model_size = 0

    def initialize_rust_functions(self):
        self.lib.predict_svm_model.restype = ctypes.c_double
        self.lib.predict_svm_model.argtypes = [
            ctypes.POINTER(ctypes.c_double),
            ctypes.POINTER(ctypes.c_double),
            ctypes.c_int
        ]

        self.lib.basic_kernel_build.restype = ctypes.POINTER(ctypes.c_double)
        self.lib.basic_kernel_build.argtypes = [
            ctypes.POINTER(ctypes.c_double),
            ctypes.POINTER(ctypes.c_double),
            ctypes.c_int,
            ctypes.c_int
        ]

        self.lib.rbf_kernel_build.restype = ctypes.POINTER(ctypes.c_double)
        self.lib.rbf_kernel_build.argtypes = [
            ctypes.POINTER(ctypes.c_double),
            ctypes.POINTER(ctypes.c_double),
            ctypes.c_int,
            ctypes.c_int,
            ctypes.c_double
        ]

        self.lib.polynomial_kernel_build.restype = ctypes.POINTER(ctypes.c_double)
        self.lib.polynomial_kernel_build.argtypes = [
            ctypes.POINTER(ctypes.c_double),
            ctypes.POINTER(ctypes.c_double),
            ctypes.c_int,
            ctypes.c_int,
            ctypes.c_int32
        ]

        self.lib.train_svm_model.restype = ctypes.POINTER(ctypes.c_double)
        self.lib.train_svm_model.argtypes = [
            ctypes.POINTER(ctypes.c_double),
            ctypes.POINTER(ctypes.c_double),
            ctypes.POINTER(ctypes.c_double),
            ctypes.c_int,
            ctypes.c_int
        ]

    def predict_svm_model(self, x):
        return self.lib.predict_svm_model(
            self.model,
            x.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            len(x))
    
    def qp_solver(self, P, x, y):
        n_samples, n_features = x.shape
        P = cvxopt.matrix(P, (n_samples, n_samples))
        q = cvxopt.matrix(np.ones(n_samples) * -1)
        A = cvxopt.matrix(y, (1, n_samples))
        b = cvxopt.matrix(0.0)
        G = cvxopt.matrix(np.diag(np.ones(n_samples) * -1))
        h = cvxopt.matrix(np.zeros(n_samples))
        cvxopt.solvers.options['show_progress'] = False
        solution = cvxopt.solvers.qp(P, q, G, h, A, b)
        return solution

    def kernel_build(self,kernel, x, y, args):
        if kernel == self.Kernel.BASIC:
            return self.basic_kernel_build(x,y)
        elif kernel == self.Kernel.RBF:
            return self.rbf_kernel_build(x,y,args[0])
        elif kernel == self.Kernel.POLYNOMIAL:
            return self.polynomial_kernel_build(x, y, int(args[0]))

        return None

    def basic_kernel_build(self, x, y):
        P = self.lib.basic_kernel_build(
            x.flatten().ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            y.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            x.shape[1],
            x.shape[0]
        )
        return [P[i] for i in range(x.shape[0] * x.shape[0])]

    def rbf_kernel_build(self, x, y, gamma):
        P = self.lib.rbf_kernel_build(
            x.flatten().ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            y.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            x.shape[1],
            x.shape[0],
            gamma
        )
        return [P[i] for i in range(x.shape[0] * x.shape[0])]

    def polynomial_kernel_build(self, x, y, degree):
        P = self.lib.polynomial_kernel_build(
            x.flatten().ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            y.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            x.shape[1],
            x.shape[0],
            degree
        )
        return [P[i] for i in range(x.shape[0] * x.shape[0])]

    def train_svm_model(self, P, x, y):        
        solution = self.qp_solver(P, x, y)
        x_qp = np.array(solution['x'], dtype='float64')
        self.model = self.lib.train_svm_model(
            x_qp.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            x.flatten().ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            y.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            x.shape[1],
            x.shape[0]
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