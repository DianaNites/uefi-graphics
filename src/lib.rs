//! An embedded-graphics display driver for UEFI environments
#![no_std]
use core::{convert::TryInto, marker::PhantomData};
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::{IntoStorage, Rgb888},
    Pixel,
};

#[derive(Debug)]
pub struct Unsupported(());

impl Unsupported {
    fn new<T>(_: T) -> Self {
        Unsupported(())
    }
}

/// UEFI Display driver. This assumes Rgb888 pixel formatting.
pub struct UefiDisplay<'a> {
    /// UEFI Framebuffer
    fb: *mut u8,
    stride: u32,
    size: (u32, u32),
    spooky: PhantomData<&'a mut [u8]>,
}

impl<'a> UefiDisplay<'a> {
    /// Create a new [`UefiDisplay`].
    ///
    /// `fb` must be the UEFI framebuffer base, and `stride` the pixel stride,
    /// and `size` the horizontal and vertical resolution, respectively.
    ///
    /// In the UEFI spec this information is found
    /// in the `EFI_GRAPHICS_OUTPUT_MODE_INFORMATION` structure.
    ///
    /// `T` is something providing a lifetime for `fb`.
    /// If your UEFI API does not provide a lifetime, `&()` should work.
    pub fn new<T>(fb: *mut u8, stride: u32, size: (u32, u32), _lifetime: &'a T) -> Self {
        Self {
            fb,
            stride,
            size,
            spooky: PhantomData,
        }
    }
}

impl<'a> OriginDimensions for UefiDisplay<'a> {
    /// Return the size of the display
    fn size(&self) -> Size {
        Size::from(self.size)
    }
}

impl<'a> DrawTarget for UefiDisplay<'a> {
    type Color = Rgb888;

    type Error = Unsupported;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let pixels = pixels.into_iter();
        for Pixel(point, color) in pixels {
            let bytes = color.into_storage();
            let stride = self.stride as u64;
            let (x, y) = (point.x as u64, point.y as u64);
            // Get the linear index
            let index: usize = (((y * stride) + x) * 4)
                .try_into()
                .map_err(Unsupported::new)?;
            unsafe { (self.fb.add(index) as *mut u32).write_volatile(bytes) };
        }
        Ok(())
    }
}
