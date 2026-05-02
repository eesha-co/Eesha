//! Android JNI entry point for Eesha browser.
//!
//! This module provides the `android_main` function that is called
//! when the native activity starts on Android.

use crate::app::{Eesha, EventLoopProxyMessage};
use crate::Result;
use winit::application::ApplicationHandler;
use winit::event_loop::{DeviceEvents, EventLoop, EventLoopProxy};
use winit::platform::android::EventLoopBuilderExtAndroid;

struct App {
    eesha: Option<Eesha>,
    proxy: EventLoopProxy<EventLoopProxyMessage>,
}

impl ApplicationHandler<EventLoopProxyMessage> for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.eesha = Some(Eesha::new(event_loop, self.proxy.clone()));
        self.eesha.as_mut().unwrap().init();
    }

    fn exiting(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(v) = self.eesha.as_mut() {
            v.before_shutdown();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let Some(v) = self.eesha.as_mut() {
            v.handle_window_event(event_loop, window_id, event);
        }
    }

    fn user_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        event: EventLoopProxyMessage,
    ) {
        if let Some(v) = self.eesha.as_mut() {
            match event {
                EventLoopProxyMessage::Wake => {
                    v.request_redraw(event_loop);
                }
                EventLoopProxyMessage::IpcMessage(message) => {
                    v.handle_incoming_webview_message(*message);
                }
                EventLoopProxyMessage::EeshaInternalMessage(message) => {
                    v.handle_eesha_internal_message(message);
                }
            }
        }
    }
}

/// Android main entry point.
/// This is called by the NativeActivity when the app starts.
#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    init_crypto();

    let event_loop = EventLoop::<EventLoopProxyMessage>::with_user_event()
        .with_android_app(app)
        .build()
        .expect("Failed to create event loop");
    event_loop.listen_device_events(DeviceEvents::Never);
    let proxy = event_loop.create_proxy();
    let mut eesha_app = App {
        eesha: None,
        proxy,
    };
    event_loop.run_app(&mut eesha_app).expect("Eesha event loop error");
}

fn init_crypto() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Error initializing crypto provider");
}
