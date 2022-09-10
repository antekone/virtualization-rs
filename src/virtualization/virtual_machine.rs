//! virtual machine module

use crate::{
    base::{Id, NSArray, NSError},
    virtualization::boot_loader::VZBootLoader,
    virtualization::entropy_device::VZEntropyDeviceConfiguration,
    virtualization::memory_device::VZMemoryBalloonDeviceConfiguration,
    virtualization::network_device::VZNetworkDeviceConfiguration,
    virtualization::serial_port::VZSerialPortConfiguration,
    virtualization::socket_device::VZSocketDeviceConfiguration,
    virtualization::storage_device::VZStorageDeviceConfiguration,
};

use block::Block;
use objc::runtime::BOOL;
use objc::{class, msg_send, sel, sel_impl};
use objc::{rc::StrongPtr, runtime::YES};

use super::{platform::VZPlatformConfiguration, keyboard_device::VZKeyboardConfiguration, graphic_device::VZGraphicsDeviceConfiguration, pointing_device::VZPointingDeviceConfiguration};

/// builder for VZVirtualMachineConfiguration
/// # Examples
/// ```rust
/// let conf = VZVirtualMachineConfigurationBuilder::new()
///     .boot_loader(boot_loader)
///     .cpu_count(cpu_count)
///     .memory_size(memory_size)
///     .entropy_devices(vec![entropy])
///     .memory_balloon_devices(vec![memory_balloon])
///     .network_devices(vec![network_device])
///     .serial_ports(vec![serial])
///     .storage_devices(vec![block_device])
///     .build();
/// ```
pub struct VZVirtualMachineConfigurationBuilder {
    conf: VZVirtualMachineConfiguration,
}

impl VZVirtualMachineConfigurationBuilder {
    pub fn new() -> Self {
        VZVirtualMachineConfigurationBuilder {
            conf: VZVirtualMachineConfiguration::new(),
        }
    }

    pub fn boot_loader<T: VZBootLoader>(mut self, boot_loader: T) -> Self {
        self.conf.set_boot_loader(boot_loader);
        self
    }

    pub fn platform(mut self, platform: impl VZPlatformConfiguration) -> Self {
        self.conf.set_platform(platform);
        self
    }

    pub fn cpu_count(mut self, cpu_count: usize) -> Self {
        self.conf.set_cpu_count(cpu_count);
        self
    }

    pub fn memory_size(mut self, memory_size: usize) -> Self {
        self.conf.set_memory_size(memory_size);
        self
    }

    pub fn entropy_devices<T: VZEntropyDeviceConfiguration>(
        mut self,
        entropy_devices: Vec<T>,
    ) -> Self {
        self.conf.set_entropy_devices(entropy_devices);
        self
    }

    pub fn graphics_devices<T: VZGraphicsDeviceConfiguration>(
        mut self,
        devices: Vec<T>,
    ) -> Self {
        self.conf.set_graphics_devices(devices);
        self
    }

    pub fn pointing_devices<T: VZPointingDeviceConfiguration>(
        mut self,
        devices: Vec<T>,
    ) -> Self {
        self.conf.set_pointing_devices(devices);
        self
    }

    pub fn keyboard_devices<T: VZKeyboardConfiguration>(
        mut self,
        devices: Vec<T>,
    ) -> Self {
        self.conf.set_keyboards(devices);
        self
    }

    pub fn memory_balloon_devices<T: VZMemoryBalloonDeviceConfiguration>(
        mut self,
        memory_balloon_devices: Vec<T>,
    ) -> Self {
        self.conf.set_memory_balloon_devices(memory_balloon_devices);
        self
    }

    pub fn network_devices<T: VZNetworkDeviceConfiguration>(
        mut self,
        network_devices: Vec<T>,
    ) -> Self {
        self.conf.set_network_devices(network_devices);
        self
    }

    pub fn serial_ports<T: VZSerialPortConfiguration>(mut self, serial_ports: Vec<T>) -> Self {
        self.conf.set_serial_ports(serial_ports);
        self
    }

    pub fn socket_devices<T: VZSocketDeviceConfiguration>(
        mut self,
        socket_devices: Vec<T>,
    ) -> Self {
        self.conf.set_socket_devices(socket_devices);
        self
    }

    pub fn storage_devices<T: VZStorageDeviceConfiguration>(
        mut self,
        storage_devices: Vec<T>,
    ) -> Self {
        self.conf.set_storage_devices(storage_devices);
        self
    }

    pub fn build(self) -> VZVirtualMachineConfiguration {
        self.conf
    }
}

/// this structure is undocumented, but useful
pub struct VZVirtualMachineStartOptions(StrongPtr);

impl VZVirtualMachineStartOptions {
    pub fn new() -> Self {
        unsafe {
            let obj = msg_send![class!(_VZVirtualMachineStartOptions), new];
            Self(StrongPtr::new(obj))
        }
    }

    pub fn set_boot_recovery(&mut self, flag: bool) {
        unsafe { let _: () = msg_send![*self.0, setBootMacOSRecovery: flag]; }
    }

    pub fn set_panic_action(&mut self, flag: bool) {
        unsafe { let _: () = msg_send![*self.0, setPanicAction: flag]; }
    }

    pub fn set_stop_in_iboot_stage_1(&mut self, flag: bool) {
        unsafe { let _: () = msg_send![*self.0, setStopInIBootStage1: flag]; }
    }

    pub fn set_stop_in_iboot_stage_2(&mut self, flag: bool) {
        unsafe { let _: () = msg_send![*self.0, setStopInIBootStage2: flag]; }
    }

    pub fn set_force_dfu(&mut self, flag: bool) {
        unsafe { let _: () = msg_send![*self.0, setForceDFU: flag]; }
    }
}

/// configure of virtual machine
pub struct VZVirtualMachineConfiguration(StrongPtr);

impl VZVirtualMachineConfiguration {
    fn new() -> VZVirtualMachineConfiguration {
        unsafe {
            let obj = StrongPtr::new(msg_send![class!(VZVirtualMachineConfiguration), new]);
            VZVirtualMachineConfiguration(obj)
        }
    }

    fn set_boot_loader<T: VZBootLoader>(&mut self, boot_loader: T) {
        unsafe {
            let _: () = msg_send![*self.0, setBootLoader: boot_loader.id()];
        }
    }

    fn set_cpu_count(&mut self, cnt: usize) {
        unsafe {
            let _: () = msg_send![*self.0, setCPUCount: cnt];
        }
    }

    fn set_memory_size(&mut self, size: usize) {
        unsafe {
            let _: () = msg_send![*self.0, setMemorySize: size];
        }
    }

    fn set_entropy_devices<T: VZEntropyDeviceConfiguration>(&mut self, devices: Vec<T>) {
        let device_ids = devices.iter().map(|x| x.id()).collect();
        let arr: NSArray<T> = NSArray::array_with_objects(device_ids);
        unsafe {
            let _: () = msg_send![*self.0, setEntropyDevices:*arr.p];
        }
    }

    fn set_memory_balloon_devices<T: VZMemoryBalloonDeviceConfiguration>(
        &mut self,
        devices: Vec<T>,
    ) {
        let device_ids = devices.iter().map(|x| x.id()).collect();
        let arr: NSArray<T> = NSArray::array_with_objects(device_ids);
        unsafe {
            let _: () = msg_send![*self.0, setMemoryBalloonDevices:*arr.p];
        }
    }

    fn set_network_devices<T: VZNetworkDeviceConfiguration>(&mut self, devices: Vec<T>) {
        let device_ids = devices.iter().map(|x| x.id()).collect();
        let arr: NSArray<T> = NSArray::array_with_objects(device_ids);
        unsafe {
            let _: () = msg_send![*self.0, setNetworkDevices:*arr.p];
        }
    }

    fn set_serial_ports<T: VZSerialPortConfiguration>(&mut self, devices: Vec<T>) {
        let device_ids = devices.iter().map(|x| x.id()).collect();
        let arr: NSArray<T> = NSArray::array_with_objects(device_ids);
        unsafe {
            let _: () = msg_send![*self.0, setSerialPorts:*arr.p];
        }
    }

    fn set_socket_devices<T: VZSocketDeviceConfiguration>(&mut self, devices: Vec<T>) {
        let device_ids = devices.iter().map(|x| x.id()).collect();
        let arr: NSArray<T> = NSArray::array_with_objects(device_ids);
        unsafe {
            let _: () = msg_send![*self.0, setSocketDevices:*arr.p];
        }
    }

    fn set_storage_devices<T: VZStorageDeviceConfiguration>(&mut self, devices: Vec<T>) {
        let device_ids = devices.iter().map(|x| x.id()).collect();
        let arr: NSArray<T> = NSArray::array_with_objects(device_ids);
        unsafe {
            let _: () = msg_send![*self.0, setStorageDevices:*arr.p];
        }
    }

    fn set_keyboards<T: VZKeyboardConfiguration>(&mut self, devices: Vec<T>) {
        let device_ids = devices.iter().map(|x| x.id()).collect();
        let arr: NSArray<T> = NSArray::array_with_objects(device_ids);
        unsafe {
            let _: () = msg_send![*self.0, setKeyboards:*arr.p];
        }
    }

    fn set_graphics_devices<T: VZGraphicsDeviceConfiguration>(&mut self, devices: Vec<T>) {
        let device_ids = devices.iter().map(|x| x.id()).collect();
        let arr: NSArray<T> = NSArray::array_with_objects(device_ids);
        unsafe {
            let _: () = msg_send![*self.0, setGraphicsDevices:*arr.p];
        }
    }

    fn set_pointing_devices<T: VZPointingDeviceConfiguration>(&mut self, devices: Vec<T>) {
        let device_ids = devices.iter().map(|x| x.id()).collect();
        let arr: NSArray<T> = NSArray::array_with_objects(device_ids);
        unsafe {
            let _: () = msg_send![*self.0, setPointingDevices:*arr.p];
        }
    }

    fn set_platform(&mut self, platform: impl VZPlatformConfiguration) {
        unsafe {
            let _: () = msg_send![*self.0, setPlatform: platform.id()];
        }
    }

    pub fn validate_with_error(&self) -> Result<BOOL, NSError> {
        unsafe {
            let error = NSError(StrongPtr::new(0 as Id));
            let obj: BOOL = msg_send![*self.0, validateWithError: &(*error.0)];
            if error.code() != 0 {
                Err(error)
            } else {
                Ok(obj)
            }
        }
    }
}

/// virtual machine
#[derive(Clone)]
pub struct VZVirtualMachine(StrongPtr);

/// state of virtual machine
#[derive(Debug)]
pub enum VZVirtualMachineState {
    /// Initial state before the virtual machine is started.
    VZVirtualMachineStateStopped,

    /// Running virtual machine.
    VZVirtualMachineStateRunning,

    /// A started virtual machine is paused. This state can only be transitioned from VZVirtualMachineStatePausing.
    VZVirtualMachineStatePaused,

    /// The virtual machine has encountered an internal error.
    VZVirtualMachineStateError,

    /// The virtual machine is configuring the hardware and starting.
    VZVirtualMachineStateStarting,

    /// The virtual machine is being paused. This is the intermediate state between VZVirtualMachineStateRunning and VZVirtualMachineStatePaused.
    VZVirtualMachineStatePausing,

    /// The virtual machine is being resumed. This is the intermediate state between VZVirtualMachineStatePaused and VZVirtualMachineStateRunning. */
    VZVirtualMachineStateResuming,

    /// Other
    Other,
}

impl VZVirtualMachine {
    pub fn new(conf: VZVirtualMachineConfiguration, queue: Id) -> VZVirtualMachine {
        unsafe {
            let i: Id = msg_send![class!(VZVirtualMachine), alloc];
            let p = StrongPtr::new(msg_send![i, initWithConfiguration:*conf.0 queue:queue]);
            println!("VZVirtualMachine::new() -- p={:?}", *p);
            VZVirtualMachine(p)
        }
    }

    pub fn start_with_completion_handler(&mut self, completion_handler: &Block<(Id,), ()>) {
        unsafe {
            let _: Id = msg_send![*self.0, startWithCompletionHandler: completion_handler];
        }
    }

    pub fn start_with_completion_handler_and_options(&mut self,
        completion_handler: &Block<(Id,), ()>,
        options: &VZVirtualMachineStartOptions
    ) {
        unsafe {
            let _: Id = msg_send![*self.0, 
                startWithOptions: options
                completionHandler: completion_handler
            ];
        }
    }

    pub unsafe fn request_stop_with_error(&mut self) -> Result<bool, NSError> {
        let error = NSError(StrongPtr::new(0 as Id));
        let ret: BOOL = msg_send![*self.0, requestStopWithError:*error.0];
        if error.code() != 0 {
            Err(error)
        } else {
            Ok(ret == YES)
        }
    }

    pub fn supported() -> bool {
        unsafe {
            let b: BOOL = msg_send![class!(VZVirtualMachine), isSupported];
            b == YES
        }
    }

    pub unsafe fn state(&self) -> VZVirtualMachineState {
        let n: isize = msg_send![*self.0, state];
        match n {
            0 => VZVirtualMachineState::VZVirtualMachineStateStopped,
            1 => VZVirtualMachineState::VZVirtualMachineStateRunning,
            2 => VZVirtualMachineState::VZVirtualMachineStatePaused,
            3 => VZVirtualMachineState::VZVirtualMachineStateError,
            4 => VZVirtualMachineState::VZVirtualMachineStateStarting,
            5 => VZVirtualMachineState::VZVirtualMachineStatePausing,
            6 => VZVirtualMachineState::VZVirtualMachineStateResuming,
            _ => VZVirtualMachineState::Other,
        }
    }

    pub unsafe fn id(&self) -> Id {
        *self.0
    }
}

#[test]
fn should_be_supported() {
    assert_eq!(true, VZVirtualMachine::supported());
}

#[test]
fn start_options_should_work() {
    let mut opts = VZVirtualMachineStartOptions::new();
    opts.set_boot_recovery(true);
    opts.set_force_dfu(true);
    opts.set_panic_action(true);
    opts.set_stop_in_iboot_stage_1(true);
    opts.set_stop_in_iboot_stage_2(true);
}