import math

# imports the module from the given path
# ref https://www.geeksforgeeks.org/how-to-import-a-python-module-given-the-full-path/
from importlib.machinery import SourceFileLoader
time = SourceFileLoader("main","../../e1-10/src/main.py").load_module()

@time.timing
def e11():
    print("hi")
