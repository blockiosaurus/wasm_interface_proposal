import pywasm

runtime = pywasm.load("./pkg/zero_copy_test_bg.wasm")
print(runtime)