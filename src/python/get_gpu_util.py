try:
    import GPUtil; print(GPUtil.getGPUs().pop().load)
except IndexError:
    print(0.0)
