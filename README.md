# Rust WASM Starter

Build your WebAssembly app in Rust.

> [!IMPORTANT]
> Use cargo to install `wasm-pack` firstly: `cargo install wasm-pack`

# Usage

Choose one of the following method to use the template.

1. Generate repository directly using `Use this template` button in GitHub.
2. Clone to local:
    ```shell
      npx degit https://github.com/Vincent-the-gamer/rust-wasm-starter.git

      # pnpm
      pnpx degit https://github.com/Vincent-the-gamer/rust-wasm-starter.git
    ```

# Dev
```shell
cd wasm
wasm-pack build --target web --out-dir output
```

wasm package will be generated at `wasm/<out_dir>`, `<out_dir>` is `pkg` by default.

## Debug or test

> [!CAUTION]
> WASM package is used for browser, if you need a Rust-based Node.js module developing method, please use [neon-starter](https://github.com/Vincent-the-gamer/neon-starter).

Debug in a `HTML` file:

```html
<script type="module">
import init, { add } from "xxx/output/wasm.js"

function addTwo() {
    const content = document.getElementById("content")
    const left = document.getElementById("left").value
    const right = document.getElementById("right").value
    const result = add(left, right)
    content.innerText = result
}

// You must call the function after wasm module initialization.
init().then(() => {
    window.addTwo = addTwo
})
</script>
```

### Test
Run [dev commands](#dev), and then test in `test/test.html`.

# Build
```shell
cd wasm
wasm-pack build --target web --release --out-dir output
```

wasm package will be generated at `wasm/<out_dir>`, `<out_dir>` is `pkg` by default.

# License 
[MIT](https://github.com/Vincent-the-gamer/rust-wasm-starter/blob/main/LICENSE)