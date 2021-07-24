//! An embedded-graphics display driver for UEFI environments
#![no_std]
use core::{convert::TryInto, marker::PhantomData};
use embedded_graphics::{drawable::Pixel, pixelcolor::*, prelude::*, DrawTarget};

#[derive(Debug)]
pub struct Unsupported(());

impl Unsupported {
    fn new<T>(_: T) -> Self {
        Unsupported(())
    }
}

/// Pixel format to use
pub enum PixelFormat {
    /// Red Green Blue
    Rgb,

    /// Blue Green Red
    Bgr,
}

/// UEFI Display driver.
///
/// UEFI supports multiple different pixel formats, but embedded-graphics only
/// supports one.
/// To solve this, this display is generic over `Into<Bgr888>`.
///
/// At the moment this display only supports the BGR888 and RGB888 UEFI pixel
/// formats. BltOnly and Bitmask are unsupported.
pub struct UefiDisplay<'a> {
    /// UEFI Framebuffer
    fb: *mut u8,
    pixel: PixelFormat,
    stride: u32,
    size: (u32, u32),
    spooky: PhantomData<&'a mut [u8]>,
}

impl<'a> UefiDisplay<'a> {
    /// Create a new [`UefiDisplay`].
    ///
    /// `fb` must be the UEFI framebuffer base, `pixel` the pixel format in use,
    /// `stride` the pixel stride,
    /// and `size` the horizontal and vertical resolution, respectively.
    ///
    /// In the UEFI spec this information is found
    /// in the `EFI_GRAPHICS_OUTPUT_MODE_INFORMATION` structure.
    pub fn new(fb: *mut u8, pixel: PixelFormat, stride: u32, size: (u32, u32)) -> Self {
        Self {
            fb,
            pixel,
            stride,
            size,
            spooky: PhantomData,
        }
    }

    /// Return the size of the display
    pub fn size(&self) -> Size {
        // Size::new(self.size.0, self.size.1)
        Size::from(self.size)
    }
}

impl<'a, T: Into<Bgr888> + PixelColor> DrawTarget<T> for UefiDisplay<'a> {
    type Error = Unsupported;

    fn draw_pixel(&mut self, item: Pixel<T>) -> Result<(), Self::Error> {
        let Pixel(point, color) = item;
        let mut bytes = [0u8; 3];
        match self.pixel {
            PixelFormat::Rgb => {
                bytes
                    .copy_from_slice(&Rgb888::from(color.into()).into_storage().to_be_bytes()[1..]);
            }
            PixelFormat::Bgr => {
                bytes.copy_from_slice(&color.into().into_storage().to_be_bytes()[1..]);
            }
        }
        let Size { width, height } = <Self as DrawTarget<T>>::size(self);
        let stride: u64 = self.stride.into();
        let (x, y) = (point.x as u64, point.y as u64);
        if x < width.into() && y < height.into() {
            let index: usize = (((y * stride) + x) * 4)
                .try_into()
                .map_err(Unsupported::new)?;
            unsafe { (self.fb.add(index) as *mut [u8; 3]).write_volatile(bytes) };
        }
        Ok(())
    }

    fn size(&self) -> Size {
        self.size()
    }
}
