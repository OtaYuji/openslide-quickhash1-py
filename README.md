# openslide-quickhash1-py
simply returns quickhash-1 using openslide-rust

### Build

```sh
rustup run nightly cargo build --release  
ln -s target/release/libopenslide_quickhash1.so openslide_quickhash1.so
```

### Call the function from python

```python
import openslide_quickhash1

wsi_path = "/path/to/whole_slide_image.svs"
quick_hash = openslide_quickhash1.quickhash1(wsi_path)

def print_result(q_hash: String) -> None:
    if q_hash == "":
        print("Failed to load quickhash1")
    else:
        print("quickhash1", q_hash)

print_result(quick_hash)
```
