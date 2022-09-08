//! keyboard device module

use crate::base::{Id, NSArray};

use objc::rc::StrongPtr;
use objc::{class, msg_send, sel, sel_impl};

pub trait VZVirtioSoundDeviceStreamConfiguration {
    fn id(&self) -> Id;
}

pub trait VZAudioInputStreamSource {
    fn id(&self) -> Id;
    fn clone_ptr(&self) -> StrongPtr;
}

pub trait VZAudioOutputStreamSink {
    fn id(&self) -> Id;
    fn clone_ptr(&self) -> StrongPtr;
}

struct VZVirtioSoundDeviceInputStreamConfiguration {
    source: StrongPtr,
    ptr: StrongPtr,
}

impl VZVirtioSoundDeviceInputStreamConfiguration {
    fn new(source: impl VZAudioInputStreamSource) -> Self {
        unsafe {
            let p: Id = msg_send![class!(VZVirtioSoundDeviceInputStreamConfiguration), new];
            let _: () = msg_send![p, setSource: source.id()];
            Self {
                source: source.clone_ptr(), 
                ptr: StrongPtr::new(p)
            }
        }
    }
}

struct VZVirtioSoundDeviceOutputStreamConfiguration {
    sink: StrongPtr,
    ptr: StrongPtr,
}

impl VZVirtioSoundDeviceOutputStreamConfiguration {
    fn new(sink: impl VZAudioOutputStreamSink) -> Self {
        unsafe {
            let p: Id = msg_send![class!(VZVirtioSoundDeviceOutputStreamConfiguration), new];
            let _: () = msg_send![p, setSink: sink.id()];
            Self {
                sink: sink.clone_ptr(), 
                ptr: StrongPtr::new(p)
            }
        }
    }
}

pub struct VZHostAudioInputStreamSource(StrongPtr);

impl VZHostAudioInputStreamSource {
    fn new() -> Self {
        unsafe {
            Self(StrongPtr::new(
                msg_send![class!(VZHostAudioInputStreamSource), new]
            ))
        }
    }
}

impl VZAudioInputStreamSource for VZHostAudioInputStreamSource {
    fn id(&self) -> Id { *self.0 }
    fn clone_ptr(&self) -> StrongPtr {
        self.0.clone()
    }
}

pub struct VZHostAudioOutputStreamSink(StrongPtr);

impl VZHostAudioOutputStreamSink {
    fn new() -> Self {
        unsafe {
            Self(StrongPtr::new(
                msg_send![class!(VZHostAudioOutputStreamSink), new]
            ))
        }
    }
}

impl VZAudioOutputStreamSink for VZHostAudioOutputStreamSink {
    fn id(&self) -> Id { *self.0 }
    fn clone_ptr(&self) -> StrongPtr {
        self.0.clone()
    }
}

#[test]
fn instantiation_test_sound_device_host_input_stream() {
    let _ = VZHostAudioInputStreamSource::new();
}

#[test]
fn instantiation_test_sound_device_host_output_stream() {
    let _ = VZHostAudioOutputStreamSink::new();
}

#[test]
fn instantiation_test_virtio_sound_device_input_stream_config() {
    let host_in = VZHostAudioInputStreamSource::new();
    let _ = VZVirtioSoundDeviceInputStreamConfiguration::new(host_in);
}

#[test]
fn instantiation_test_virtio_sound_device_output_stream_config() {
    let host_out = VZHostAudioOutputStreamSink::new();
    let _ = VZVirtioSoundDeviceOutputStreamConfiguration::new(host_out);
}