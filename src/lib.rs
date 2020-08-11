//! An embedded-graphics display driver for UEFI systems
#![no_std]
use core::convert::TryInto;
use embedded_graphics::{drawable::Pixel, pixelcolor::*, prelude::*, DrawTarget};
use uefi::proto::console::gop::*;

#[derive(Debug)]
pub struct Unsupported(());

/// UEFI Display driver.
///
/// UEFI supports multiple different pixel formats, but embedded-graphics only
/// supports one.
/// To solve this, this display is generic over `Into<Bgr888>`.
///
/// At the moment this display only supports the BGR888 and RGB888 UEFI pixel
/// formats. BltOnly and Bitmask are unsupported.
pub struct UefiDisplay<'a> {
    info: ModeInfo,
    fb: FrameBuffer<'a>,
}

impl<'a> UefiDisplay<'a> {
    /// Create a new [`UefiDisplay`].
    pub fn new(info: ModeInfo, fb: FrameBuffer<'a>) -> Self {
        Self { info, fb }
    }
}

impl<'a, T: Into<Bgr888> + PixelColor> DrawTarget<T> for UefiDisplay<'a> {
    type Error = Unsupported;

    fn draw_pixel(&mut self, item: Pixel<T>) -> Result<(), Self::Error> {
        let Pixel(point, color) = item;
        let color: Bgr888 = color.into();
        let Size { width, height } = <Self as DrawTarget<T>>::size(self);
        let stride: u64 = self
            .info
            .stride()
            .try_into()
            .expect("Stride didn't fit in u64. Buggy UEFI firmware?");
        let (x, y) = (point.x as u64, point.y as u64);
        if x < width.into() && y < height.into() {
            let index: usize = (((y * stride) + x) * 4)
                .try_into()
                .expect("Framebuffer index overflowed usize");
            unsafe {
                match self.info.pixel_format() {
                    PixelFormat::RGB => {
                        self.fb
                            .write_value(index, Rgb888::from(color).into_storage().to_ne_bytes());
                    }
                    PixelFormat::BGR => {
                        self.fb
                            .write_value(index, color.into_storage().to_ne_bytes());
                    }
                    _ => return Err(Unsupported(())),
                }
            }
        }
        Ok(())
    }

    fn size(&self) -> Size {
        let (width, height) = self.info.resolution();
        Size::new(
            width
                .try_into()
                .expect("Display width didn't fit in u32. Buggy UEFI firmware?"),
            height
                .try_into()
                .expect("Display height didn't fit in u32. Buggy UEFI firmware?"),
        )
    }
}

/// Same as `UefiDisplay` but without a generic DrawTarget.
///
/// Testing purposes only.
pub struct UefiDisplayNotGeneric<'a> {
    info: ModeInfo,
    fb: FrameBuffer<'a>,
}

impl<'a> UefiDisplayNotGeneric<'a> {
    /// Create a new [`UefiDisplay`].
    pub fn new(info: ModeInfo, fb: FrameBuffer<'a>) -> Self {
        Self { info, fb }
    }
}

impl<'a> DrawTarget<Bgr888> for UefiDisplayNotGeneric<'a> {
    type Error = Unsupported;

    fn draw_pixel(&mut self, item: Pixel<Bgr888>) -> Result<(), Self::Error> {
        let Pixel(point, color) = item;
        let Size { width, height } = self.size();
        let stride: u64 = self
            .info
            .stride()
            .try_into()
            .expect("Stride didn't fit in u64. Buggy UEFI firmware?");
        let (x, y) = (point.x as u64, point.y as u64);
        if x < width.into() && y < height.into() {
            let index: usize = (((y * stride) + x) * 4)
                .try_into()
                .expect("Framebuffer index overflowed usize");
            unsafe {
                match self.info.pixel_format() {
                    PixelFormat::RGB => {
                        self.fb
                            .write_value(index, Rgb888::from(color).into_storage().to_ne_bytes());
                    }
                    PixelFormat::BGR => {
                        self.fb
                            .write_value(index, color.into_storage().to_ne_bytes());
                    }
                    _ => return Err(Unsupported(())),
                }
            }
        }
        Ok(())
    }

    fn size(&self) -> Size {
        let (width, height) = self.info.resolution();
        Size::new(
            width
                .try_into()
                .expect("Display width didn't fit in u32. Buggy UEFI firmware?"),
            height
                .try_into()
                .expect("Display height didn't fit in u32. Buggy UEFI firmware?"),
        )
    }
}
