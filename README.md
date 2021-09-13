# sv4state

sv4state is a Rust library for SystemVerilog 4-state value.
`logic` value which is passed throgh SystemVerilog DPI can be handled by this library.

## Example

`svLogicVecVal` shows a 32bit `logic` value of SystemVerilog.
So `logic [127:0]` corresponds to `[svLogicVecVal; 4]`.

```rust
use sv4state::{svLogicVecVal, Sv4State};

#[no_mangle]
pub extern "C" fn get_data(data: &[svLogicVecVal; 4]) {
    let sv_u32 = Sv4State::<u32>::from_dpi(data);
    println!("{:x}", sv_u32[0]);
    println!("{:x}", sv_u32[1]);
    println!("{:x}", sv_u32[2]);
    println!("{:x}", sv_u32[3]);
}
```

The `get_data()` can be call through SystemVerilog DPI like below:

```SystemVerilog
import "DPI-C" function void get_data(
  input logic [127:0] data
);
```
