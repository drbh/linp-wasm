

### Build
```
wasm-pack build --target no-modules
wasm-opt -Os ./pkg/linp_wasm_bg.wasm -o ./pkg/linp_wasm_opt.wasm
```

### Dev
check the bingen README

### Optimizations
https://rustwasm.github.io/wasm-bindgen/examples/add.html