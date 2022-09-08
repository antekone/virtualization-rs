//! pointing device module

use crate::base::{Id, NSArray};

use objc::rc::StrongPtr;
use objc::{class, msg_send, sel, sel_impl};

pub trait VZPointingDeviceConfiguration {
    fn id(&self) -> Id;
}

pub struct VZUSBScreenCoordinatePointingDeviceConfiguration(StrongPtr);

impl VZUSBScreenCoordinatePointingDeviceConfiguration {
    pub fn new() -> Self {
        unsafe {
            Self(StrongPtr::new(
                msg_send![class!(VZUSBScreenCoordinatePointingDeviceConfiguration), new]
            ))
        }
    }
}

impl VZPointingDeviceConfiguration for VZUSBScreenCoordinatePointingDeviceConfiguration {
    fn id(&self) -> Id {
        *self.0
    }
}

#[test]
fn instantiation_test_usb_screen_coord_pointing_device() {
    let _ = VZUSBScreenCoordinatePointingDeviceConfiguration::new();
}