# Uefi-graphics

An embedded-graphics display driver for UEFI environments.

Simple, slight work in progress. Should work fine, doesn't do too much.

## Example

Using the [`uefi`](https://crates.io/crates/uefi), currently at `0.11.0` crate

```rust
// The GraphicsOutput from the uefi crate
let graphics: &mut GraphicsOutput;

// Get the framebuffer.
let mut fb = graphics.frame_buffer();

let display = &mut UefiDisplay::new(
    // The framebuffer pointer.
    // The framebuffer is stored in a variable separately to ensure the
    // `UefiDisplay` cannot become invalid.
    fb.as_mut_ptr(),

    // These casts are needed because, while the uefi spec has these as u32,
    // the uefi crate casts them to usize for some reason.
    mode.stride() as u32,
    (mode.resolution().0 as u32, mode.resolution().1 as u32),

    // This ensures that the lifetimes are correct.
    &fb,
);
```
