# Raztec
#### **Still work in progress**

Aztec barcode reader and writer written in Rust.
The objective is that no third-party library should be used and code generation
should be fast and accurate.

## Quick start

The Aztec code generator comes as a Builder:
```Rust
use raztec::writer::AztecCodeBuilder;
let code = AztecCodeBuilder::new().error_correction(23)
    .append("Hello").append(", ").append("World!").build().unwrap();
```
Please note that the `build` function returns a **Result** as the Aztec Code
generation may fail.

This gives you an AztecCode struct that have the Index IndexMut and Display
traits. You can get the current side size with `code.size()`.
You can convert the AztecCode struct into a pixel array using `to_image`,
`to_rgb8` and `to_mono8`.

This library also supports the generation and scanning of Aztec Runes.
An Aztec Rune is a compact Aztec Code containing only one byte of information.
Here is an example of generating a rune with the byte value 38:
```Rust
use raztec::writer::AztecCodeBuilder;
let rune = AztecCodeBuilder::build_rune(38);
```

There is currently no builtin Aztec code reader (Coming soon).

### Generated using raztec

![Example Aztec Code](https://i.imgur.com/HmgLg70.png)

## Documentation

The library's code is fully documented, to see the documentation use:
```cargo doc --open```

## Issues

If you enconter any issues or bugs while generating or scanning Aztec Codes,
please open an issue so I can look into it.

## TODO

- Implement an Aztec code reader
- Improve speed of big Aztec Code generation

# Build Python3 bindings using PyO3 and maturin with Docker for Armhf (32-bit ARM):

## first step: build the docker image (only needed once):

```bash
docker build --no-cache -f Dockerfile.armhf -t raztec-armhf .
```
## second step: build the Python3 bindings:

```bash
docker run --rm -v $(pwd)/dist:/build/dist raztec-armhf
```

After running the above command, you should find the generated Python wheel file in the `dist` directory. You can then install it using pip:

```bash
pip install dist/raztec-*.whl
```

## Example usage of the Python3 bindings:



```python3
from raztec import AztecCodeBuilder                                                                                                                                                                                              
                                                                                                                                                                                                                               
builder = AztecCodeBuilder()                                                                                                                                                                                                     
builder.append("Hello World")                                                                                                                                                                                                    
code = builder.build()

print(code)          # print the Aztec code in the terminal
print(code.size())   # Module Size (z.B. 15)
print(code.is_compact())  # True für kompakte Codes
```


With custom error correction rate:
```python3
from raztec import AztecCodeBuilder

builder = AztecCodeBuilder()
builder.error_correction(33)  # 33% instead default 23%
builder.append("Hello World")
code = builder.build()

print(code)          # print the Aztec code in the terminal
print(code.size())   # Module Size (z.B. 15)
print(code.is_compact())  # True für kompakte Codes
```

Using Binary data:

```python3
from raztec import AztecCodeBuilder

builder = AztecCodeBuilder()
builder.error_correction(33)  # 33% statt default 23%
builder.append_bytes(b"\x01\x02\x03")
code = builder.build()

print(code)          # print the Aztec code in the terminal
print(code.size())   # Module Size (z.B. 15)
print(code.is_compact())  # True if compact code
```

Aztec Rune (single byte):

```python3
from raztec import rune

code = rune(42)
print(code)
```

Save Aztec code as Image (with Pillow):

```python3
from PIL import Image
from raztec import AztecCodeBuilder

builder = AztecCodeBuilder()
builder.append("Hello World")
code = builder.build()

module_size = 4  # Pixel/Module
pixels = code.to_mono8(module_size)
img_size = code.size() * module_size

img = Image.frombytes("L", (img_size, img_size), bytes(pixels))
img.save("aztec.png")
```

With the `to_mono8()` method, you can get the pixel data as a list of grayscale values (0=black, 255=white). 
The `to_rgb8()` method provides RGB values as a list of u32 integers.

