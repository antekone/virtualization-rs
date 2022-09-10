//! graphic device module

use crate::base::{Id, NSArray};

use objc::rc::StrongPtr;
use objc::{class, msg_send, sel, sel_impl};

pub trait VZGraphicsDeviceConfiguration {
    fn id(&self) -> Id;
}

pub struct VZMacGraphicsDisplayConfiguration(StrongPtr);

impl VZMacGraphicsDisplayConfiguration {
    pub fn new_with_resolution_and_ppi(width: i16, height: i16, ppi: i16) -> Self {
        unsafe {
            let p: Id = msg_send![class!(VZMacGraphicsDisplayConfiguration), alloc];
            let p = msg_send![p, 
                initWithWidthInPixels: width 
                heightInPixels: height 
                pixelsPerInch: ppi];

            VZMacGraphicsDisplayConfiguration(StrongPtr::new(p))
        }
    }

    pub fn new_with_resolution(width: i16, height: i16) -> Self {
        Self::new_with_resolution_and_ppi(width, height, 80i16)
    }

    pub fn new() -> Self {
        Self::new_with_resolution_and_ppi(1920i16, 1080i16, 80i16)
    }
}

pub struct VZMacGraphicsDeviceConfiguration(StrongPtr);

impl VZMacGraphicsDeviceConfiguration {
    pub fn new(displays: Vec<VZMacGraphicsDisplayConfiguration>) -> Self {
        let id_vec = displays.iter().map(|v| *v.0).collect();
        let ns_array: NSArray<Id> = NSArray::array_with_objects(id_vec);

        unsafe { 
            let g: Id = msg_send![class!(VZMacGraphicsDeviceConfiguration), new];
            let _: () = msg_send![g, setDisplays: *ns_array.p]; 
            Self(StrongPtr::new(g))
        }
    }
}

impl VZGraphicsDeviceConfiguration for VZMacGraphicsDeviceConfiguration {
    fn id(&self) -> Id {
        *self.0
    }
}

#[test]
fn instantiation_test_graphics_display_configuration() {
    let _ = VZMacGraphicsDisplayConfiguration::new_with_resolution_and_ppi(800, 600, 80);
}

#[test]
fn instantiation_test_graphics_device_configuration() {
    let disp1 = VZMacGraphicsDisplayConfiguration::new_with_resolution_and_ppi(800, 600, 80);
    let disp2 = VZMacGraphicsDisplayConfiguration::new_with_resolution_and_ppi(1024, 1080, 80);
    let disp3 = VZMacGraphicsDisplayConfiguration::new_with_resolution_and_ppi(1024, 1200, 80);
    let _ = VZMacGraphicsDeviceConfiguration::new(vec![disp1, disp2, disp3]);
}