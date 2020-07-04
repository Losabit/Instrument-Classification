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
            ctypes.POINTER(ctypes.c_int),
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
            ctypes.POINTER(ctypes.c_int),
            ctypes.POINTER(ctypes.c_int),
            ctypes.c_int,
            ctypes.c_int,
            ctypes.c_int,
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
            self.neurones_by_couche.ctypes.data_as(ctypes.POINTER(ctypes.c_int)), 
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
            y.ctypes.data_as(ctypes.POINTER(ctypes.c_int8)),
            self.neurones_by_couche.ctypes.data_as(ctypes.POINTER(ctypes.c_int)),
            self.nbCouche,
            nbExemple,
            nbIter,
            alpha)
