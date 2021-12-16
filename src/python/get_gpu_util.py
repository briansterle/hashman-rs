import GPUtil
try:
    print(GPUtil.getGPUs().pop().load)
except IndexError:
    print(0.0)