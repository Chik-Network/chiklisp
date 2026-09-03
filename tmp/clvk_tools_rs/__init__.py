import sys
import chiklisp

this_module = sys.modules[__name__]
for key in dir(chiklisp):
    setattr(this_module, key, getattr(chiklisp, key))
