//! keyboard device module

use crate::base::{Id, NSArray};

use objc::rc::StrongPtr;
use objc::{class, msg_send, sel, sel_impl};

pub trait VZKeyboardConfiguration {
    fn id(&self) -> Id;
}

pub struct VZUSBKeyboardConfiguration(StrongPtr);

impl VZUSBKeyboardConfiguration {
    pub fn new() -> Self {
        unsafe {
            Self(StrongPtr::new(
                msg_send![class!(VZUSBKeyboardConfiguration), new]
            ))
        }
    }
}

impl VZKeyboardConfiguration for VZUSBKeyboardConfiguration {
    fn id(&self) -> Id {
        *self.0
    }
}

#[test]
fn instantiation_test_usb_keyboard_device() {
    let _ = VZUSBKeyboardConfiguration::new();
}