use std::{fs::File, sync::{RwLock, Arc}};

use block::{ConcreteBlock, Block};
use libc::sleep;
use objc::rc::StrongPtr;
use virtualization_rs::{virtualization::{virtual_machine::{VZVirtualMachineConfigurationBuilder, VZVirtualMachine}, platform::{VZMacOSRestoreImage, VZMacHardwareModel, VZMacMachineIdentifier, VZMacAuxiliaryStorage, VZMacPlatformConfiguration}, boot_loader::VZMacOSBootLoader, graphic_device::{VZMacGraphicsDeviceConfiguration, VZMacGraphicsDisplayConfiguration}, pointing_device::VZUSBScreenCoordinatePointingDeviceConfiguration}, base::{dispatch_sync, NSError, dispatch_queue_create, NIL, Id, dispatch_async}};
use std::io::Read;

fn main() {
    let mut data = File::open("/Volumes/MacExtension/VM.bundle/HardwareModel").unwrap();
    let mut v = Vec::<u8>::new();
    data.read_to_end(&mut v);

    let mut data = File::open("/Volumes/MacExtension/VM.bundle/MachineIdentifier").unwrap();
    let mut id = Vec::<u8>::new();
    data.read_to_end(&mut id);

    let model = VZMacHardwareModel::new_with_data_representation(v.as_slice());
    let machine_id = VZMacMachineIdentifier::new_with_data_representation(id.as_slice());
    let aux_storage = VZMacAuxiliaryStorage::init_with_contents_of_url("/Volumes/MacExtension/VM.bundle/AuxiliaryStorage");
    let platform = VZMacPlatformConfiguration::new(
        model, machine_id, aux_storage
    );

    let display_config = VZMacGraphicsDisplayConfiguration::new_with_resolution(800, 600);

    let config = VZVirtualMachineConfigurationBuilder::new()
        .cpu_count(1)
        .memory_size(1 * 1024 * 1024 * 1024)
        .boot_loader(VZMacOSBootLoader::new())
        .platform(platform)
        .graphics_devices(vec![VZMacGraphicsDeviceConfiguration::new(vec![display_config])])
        .pointing_devices(vec![VZUSBScreenCoordinatePointingDeviceConfiguration::new()])
        .build();

    let r = config.validate_with_error();
    if let Err(err) = r {
        println!("Config error: {}", err.localized_description().as_str());
        std::process::exit(1);
    }

    println!("configuration is ok, starting VM...");

    let label = std::ffi::CString::new("second").unwrap();
    let queue = unsafe { dispatch_queue_create(label.as_ptr(), NIL) };
    let vm = Arc::new(RwLock::new(VZVirtualMachine::new(config, queue)));
    let dispatch_block = ConcreteBlock::new(move || {
        let completion_handler = ConcreteBlock::new(|err: Id| {
            if err != NIL {
                let error = unsafe { NSError(StrongPtr::new(err)) };
                error.dump();
            }
        });
        let completion_handler = completion_handler.copy();
        let completion_handler: &Block<(Id,), ()> = &completion_handler;
        vm.write()
            .unwrap()
            .start_with_completion_handler(completion_handler);
    });
    let dispatch_block = dispatch_block.copy();
    let dispatch_block: &Block<(), ()> = &dispatch_block;
    unsafe {
        dispatch_async(queue, dispatch_block);
    }
    loop {
        unsafe {
            sleep(100);
        }
    }

    // println!("dispatch");
    // unsafe { dispatch::ffi::dispatch_main(); }
}