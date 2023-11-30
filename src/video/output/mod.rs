pub mod parameters;
pub use parameters::Parameters;

use std::convert::TryFrom;
use std::{io, mem};

use crate::buffer::Type;
use crate::device::Device;
use crate::format::FourCC;
use crate::format::{description::Description as FormatDescription, Format};
use crate::frameinterval::FrameInterval;
use crate::framesize::FrameSize;
use crate::v4l2;
use crate::v4l_sys::*;
use crate::video::traits::Output;

#[inline(always)]
const fn get_type(mplane: bool) -> u32 {
    let t = if mplane {
        Type::VideoOutputMplane
    } else {
        Type::VideoOutput
    };

    t as u32
}

impl Output for Device {
    impl_enum_frameintervals!();
    impl_enum_framesizes!();

    fn enum_formats(&self, mplane: bool) -> io::Result<Vec<FormatDescription>> {
        let mut formats: Vec<FormatDescription> = Vec::new();

        let mut v4l2_fmt = v4l2_fmtdesc {
            index: 0,
            type_: get_type(mplane),
            ..unsafe { mem::zeroed() }
        };

        let mut ret: io::Result<()>;

        unsafe {
            ret = v4l2::ioctl(
                self.handle().fd(),
                v4l2::vidioc::VIDIOC_ENUM_FMT,
                &mut v4l2_fmt as *mut _ as *mut std::os::raw::c_void,
            );
        }

        if ret.is_err() {
            // Enumerating the first format (at index 0) failed, so there are no formats available
            // for this device. Just return an empty vec in this case.
            return Ok(Vec::new());
        }

        while ret.is_ok() {
            formats.push(FormatDescription::from(v4l2_fmt));
            v4l2_fmt.index += 1;

            unsafe {
                v4l2_fmt.description = mem::zeroed();
            }

            unsafe {
                ret = v4l2::ioctl(
                    self.handle().fd(),
                    v4l2::vidioc::VIDIOC_ENUM_FMT,
                    &mut v4l2_fmt as *mut _ as *mut std::os::raw::c_void,
                );
            }
        }

        Ok(formats)
    }

    fn format(&self, mplane: bool) -> io::Result<Format> {
        unsafe {
            let mut v4l2_fmt = v4l2_format {
                type_: get_type(mplane),
                ..mem::zeroed()
            };
            v4l2::ioctl(
                self.handle().fd(),
                v4l2::vidioc::VIDIOC_G_FMT,
                &mut v4l2_fmt as *mut _ as *mut std::os::raw::c_void,
            )?;

            if mplane {
                Ok(Format::from(v4l2_fmt.fmt.pix_mp))
            } else {
                Ok(Format::from(v4l2_fmt.fmt.pix))
            }
        }
    }

    fn set_format(&self, fmt: &Format, mplane: bool) -> io::Result<Format> {
        unsafe {
            let mut v4l2_fmt = v4l2_format {
                type_: get_type(mplane),
                fmt: v4l2_format__bindgen_ty_1 { pix: (*fmt).into() },
            };
            v4l2::ioctl(
                self.handle().fd(),
                v4l2::vidioc::VIDIOC_S_FMT,
                &mut v4l2_fmt as *mut _ as *mut std::os::raw::c_void,
            )?;
        }

        self.format(mplane)
    }

    fn params(&self, mplane: bool) -> io::Result<Parameters> {
        unsafe {
            let mut v4l2_params = v4l2_streamparm {
                type_: get_type(mplane),
                ..mem::zeroed()
            };
            v4l2::ioctl(
                self.handle().fd(),
                v4l2::vidioc::VIDIOC_G_PARM,
                &mut v4l2_params as *mut _ as *mut std::os::raw::c_void,
            )?;

            Ok(Parameters::from(v4l2_params.parm.output))
        }
    }

    fn set_params(&self, params: &Parameters, mplane: bool) -> io::Result<Parameters> {
        unsafe {
            let mut v4l2_params = v4l2_streamparm {
                type_: get_type(mplane),
                parm: v4l2_streamparm__bindgen_ty_1 {
                    output: (*params).into(),
                },
            };
            v4l2::ioctl(
                self.handle().fd(),
                v4l2::vidioc::VIDIOC_S_PARM,
                &mut v4l2_params as *mut _ as *mut std::os::raw::c_void,
            )?;
        }

        self.params(mplane)
    }
}
