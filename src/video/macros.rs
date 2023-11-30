macro_rules! impl_enum_frameintervals {
    () => {
        fn enum_frameintervals(
            &self,
            fourcc: FourCC,
            width: u32,
            height: u32,
        ) -> io::Result<Vec<FrameInterval>> {
            let mut frameintervals = Vec::new();
            let mut v4l2_struct = v4l2_frmivalenum {
                index: 0,
                pixel_format: fourcc.into(),
                width,
                height,
                ..unsafe { mem::zeroed() }
            };

            loop {
                let ret = unsafe {
                    v4l2::ioctl(
                        self.handle().fd(),
                        v4l2::vidioc::VIDIOC_ENUM_FRAMEINTERVALS,
                        &mut v4l2_struct as *mut _ as *mut std::os::raw::c_void,
                    )
                };

                if ret.is_err() {
                    if v4l2_struct.index == 0 {
                        return Err(ret.err().unwrap());
                    } else {
                        return Ok(frameintervals);
                    }
                }

                if let Ok(frame_interval) = FrameInterval::try_from(v4l2_struct) {
                    frameintervals.push(frame_interval);
                }

                v4l2_struct.index += 1;
            }
        }
    };
}

macro_rules! impl_enum_framesizes {
    () => {
        fn enum_framesizes(&self, fourcc: FourCC) -> io::Result<Vec<FrameSize>> {
            let mut framesizes = Vec::new();
            let mut v4l2_struct = v4l2_frmsizeenum {
                index: 0,
                pixel_format: fourcc.into(),
                ..unsafe { mem::zeroed() }
            };

            loop {
                let ret = unsafe {
                    v4l2::ioctl(
                        self.handle().fd(),
                        v4l2::vidioc::VIDIOC_ENUM_FRAMESIZES,
                        &mut v4l2_struct as *mut _ as *mut std::os::raw::c_void,
                    )
                };

                if ret.is_err() {
                    if v4l2_struct.index == 0 {
                        return Err(ret.err().unwrap());
                    } else {
                        return Ok(framesizes);
                    }
                }

                if let Ok(frame_size) = FrameSize::try_from(v4l2_struct) {
                    framesizes.push(frame_size);
                }

                v4l2_struct.index += 1;
            }
        }
    };
}
