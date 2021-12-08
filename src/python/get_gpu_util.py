import GPUtil
try:
    print(GPUtil.getGPUs().pop().load)
except IndexError as e:
    print(0.0)
