//! mac platform module
use crate::base::{Id, NSString, NSURL, NSData, NSError, NIL};

use block::{Block, ConcreteBlock};
use objc::rc::StrongPtr;
use objc::runtime::{BOOL, YES};
use objc::{class, msg_send, sel, sel_impl};

pub trait VZPlatformConfiguration {
    fn id(&self) -> Id;
}
pub struct VZMacAuxiliaryStorage(StrongPtr);
#[derive(Clone)]
pub struct VZMacHardwareModel(StrongPtr);
pub struct VZMacOSRestoreImage(StrongPtr);
pub struct VZMacMachineIdentifier(StrongPtr);
pub struct VZMacPlatformConfiguration(StrongPtr);

impl VZMacOSRestoreImage {
    pub fn fetch_latest_supported_with_completion_handler(
        completion_handler: fn(Result<VZMacOSRestoreImage, NSError>) -> (),
    ) {
        let block = ConcreteBlock::new(move |img: Id, err: Id| {
            let arg = if err != NIL {
                let err = unsafe { NSError(StrongPtr::retain(err)) };
                Err(err)
            } else {
                if img != NIL {
                    Ok(unsafe { Self(StrongPtr::retain(img)) })
                } else {
                    // This is somewhat unexpected!
                    // TODO: return manually crafted NSError instead of panic
                    panic!();
                }
            };

            completion_handler(arg);
        });

        let block = block.copy();

        unsafe {
            let _: () = msg_send![
                class!(VZMacOSRestoreImage), 
                fetchLatestSupportedWithCompletionHandler: block
            ];
        }
    }

    pub fn url(&self) -> Option<NSURL> {
        unsafe {
            let ptr: Id = msg_send![*self.0, URL];
            if ptr != NIL {
                Some(NSURL(StrongPtr::retain(ptr)))
            } else {
                None
            }
        }
    }

    pub fn supported(&self) -> bool {
        // TODO: API macOS 13.0+, will panic if called on a lower API
        unsafe {
            let ret: BOOL = msg_send![*self.0, isSupported];
            ret == YES
        }
    }
}

impl VZMacHardwareModel {
    pub fn new_with_data_representation(data: &[u8]) -> Self {
        unsafe {
            let ns: Id = msg_send![
                class!(NSData), dataWithBytes: data.as_ptr() length: data.len()
            ];

            let p: Id = msg_send![class!(VZMacHardwareModel), alloc];

            // This should transfer ownership of `ns` to this 
            // VZMacHardwareModel instance
            let p: Id = msg_send![p, initWithDataRepresentation: ns];
            Self(StrongPtr::new(p))
        }
    }

    pub fn supported(&self) -> bool {
        let f: BOOL = unsafe { msg_send![*self.0, isSupported] };
        f.into()
    }
}

impl VZMacMachineIdentifier {
    pub fn new_with_data_representation(data: &[u8]) -> Self {
        unsafe {
            let ns: Id = msg_send![
                class!(NSData), dataWithBytes: data.as_ptr() length: data.len()
            ];

            let p: Id = msg_send![class!(VZMacMachineIdentifier), alloc];

            // This should transfer ownership of `ns` to this 
            // VZMacMachineIdentifier instance
            let p: Id = msg_send![p, initWithDataRepresentation: ns];
            Self(StrongPtr::new(p))
        }
    }
}

impl VZMacPlatformConfiguration {
    pub fn new(
        hw_model: VZMacHardwareModel,
        machine_id: VZMacMachineIdentifier,
        aux_storage: VZMacAuxiliaryStorage,
    ) -> Self {
        unsafe {
            let p: Id = msg_send![class!(VZMacPlatformConfiguration), new];
            let _: () = msg_send![p, setHardwareModel: *hw_model.0];
            let _: () = msg_send![p, setMachineIdentifier: *machine_id.0];
            let _: () = msg_send![p, setAuxiliaryStorage: *aux_storage.0];
            Self(StrongPtr::new(p))
        }
    }
}

impl VZPlatformConfiguration for VZMacPlatformConfiguration {
    fn id(&self) -> Id {
        *self.0
    }
}

impl VZMacAuxiliaryStorage {
    pub fn init_with_contents_of_url(url: &str) -> Self {
        // TODO: deprecated since 13.0
        let url = NSURL::file_url_with_path(url, false);
        unsafe {
            let p: Id = msg_send![class!(VZMacAuxiliaryStorage), alloc];
            let p: Id = msg_send![p, initWithContentsOfURL: *url.0];
            Self(StrongPtr::new(p))
        }
    }

    pub fn init_creating_storage_at_url(&self,
        url: &str, 
        hw_model: VZMacHardwareModel,
        allow_overwrite: bool
    ) -> Result<(), NSError> {
        let options: usize = if allow_overwrite { 1 } else { 0 };
        let url = NSURL::url_with_string(url);

        unsafe {
            let error = NSError(StrongPtr::new(0 as Id));
            let _: () = msg_send![*self.0, 
                initCreatingStorageAtUrl: *url.0
                hardwareModel: *hw_model.0
                options: options
                error: *error.0
            ];

            if error.code() != 0 {
                Err(error)
            } else {
                Ok(())
            }
        }
    }
}