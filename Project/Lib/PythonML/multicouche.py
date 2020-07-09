import ctypes
import numpy as np

# avacariu.me/writing/2014/calling-rust-from-python
class MLP:
    def __init__(self, dll_path):
        self.lib = ctypes.CDLL(dll_path)
        self.initialize_rust_functions()
        self.model = []
        self.neurones_by_couche = []
        self.nbCouche = 0

    def initialize_rust_functions(self):
        self.lib.init_multicouche_model.argtypes = [ctypes.POINTER(ctypes.c_double), ctypes.c_int]
        self.lib.init_multicouche_model.restype = ctypes.POINTER(ctypes.c_double)

        self.lib.predict_multicouche_model_classification.argtypes = [
            ctypes.POINTER(ctypes.c_double),
            ctypes.POINTER(ctypes.c_double),
            ctypes.POINTER(ctypes.c_double),
            ctypes.c_int]
        self.lib.predict_multicouche_model_classification.restype = ctypes.POINTER(ctypes.c_double)

        self.lib.predict_multicouche_model_regression.argtypes = [
            ctypes.POINTER(ctypes.c_double),
            ctypes.POINTER(ctypes.c_double),
            ctypes.POINTER(ctypes.c_double),
            ctypes.c_int]
        self.lib.predict_multicouche_model_regression.restype = ctypes.POINTER(ctypes.c_double)

        self.lib.train_multicouche_model_classification.argtypes = [
            ctypes.POINTER(ctypes.c_double),
            ctypes.POINTER(ctypes.c_double),
            ctypes.POINTER(ctypes.c_double),
            ctypes.POINTER(ctypes.c_double),
            ctypes.c_int,
            ctypes.c_int,
            ctypes.c_double,
            ctypes.c_double]
        self.lib.train_multicouche_model_classification.restype = ctypes.POINTER(ctypes.c_double)

        self.lib.train_multicouche_model_regression.argtypes = [
            ctypes.POINTER(ctypes.c_double),
            ctypes.POINTER(ctypes.c_double),
            ctypes.POINTER(ctypes.c_double),
            ctypes.POINTER(ctypes.c_double),
            ctypes.c_int,
            ctypes.c_int,
            ctypes.c_double,
            ctypes.c_double]
        self.lib.train_multicouche_model_regression.restype = ctypes.POINTER(ctypes.c_double)

    def init_multicouche_model(self, neurones_by_couche):
        self.neurones_by_couche = neurones_by_couche
        self.nbCouche = len(neurones_by_couche)
        self.model = self.lib.init_multicouche_model(
            neurones_by_couche.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            self.nbCouche
        )
    

    def predict_multicouche_model_classification(self, x):
        return self.lib.predict_multicouche_model_classification(
            self.model, 
            x.ctypes.data_as(ctypes.POINTER(ctypes.c_double)), 
            self.neurones_by_couche.ctypes.data_as(ctypes.POINTER(ctypes.c_double)), 
            len(self.neurones_by_couche))

    def predict_multicouche_model_regression(self, x):
        return self.lib.predict_multicouche_model_regression(
            self.model, 
            x.ctypes.data_as(ctypes.POINTER(ctypes.c_double)), 
            self.neurones_by_couche.ctypes.data_as(ctypes.POINTER(ctypes.c_double)), 
            self.nbCouche)

    def train_multicouche_model_classification(self, x, y, nbExemple, nbIter, alpha):
        self.model = self.lib.train_multicouche_model_classification(
            self.model,
            x.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            y.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            self.neurones_by_couche.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            self.nbCouche,
            nbExemple,
            nbIter,
            alpha)

    def train_multicouche_model_regression(self, x, y, nbExemple, nbIter, alpha):
        self.model = self.lib.train_multicouche_model_regression(
            self.model,
            x.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            y.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            self.neurones_by_couche.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            self.nbCouche,
            nbExemple,
            nbIter,
            alpha)

    def save_model(self, path):
        model_size = 0
        for i in range(len(self.neurones_by_couche) - 1):
            model_size += (int(self.neurones_by_couche[i]) + 1) * int(self.neurones_by_couche[i + 1])
        model_list = [str(self.model[i]) for i in range(model_size)]
        model_string = ';'.join(model_list)
        layer_string = ';'.join([str(int(neurones)) for neurones in self.neurones_by_couche])
        with open(path, 'w') as file:
            file.write(model_string + '\n')
            file.write(layer_string) 

    def load_model(self, path):
        with open(path, 'r') as file:
            lines = file.readlines()
            model = lines[0].split(';')
            self.model = np.array([float(model[i]) for i in range(len(model))],  dtype='float64')
            self.model = self.model.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
            self.model_size = len(model)  

            neurones = lines[1].split(';')
            self.neurones_by_couche = np.array([int(neurones[i]) for i in range(len(neurones))],  dtype='float64')
            self.nbCouche = len(neurones)       