use winit::event_loop::EventLoop;

use vulkano::swapchain::PresentMode;

use vulkano_util::context::{VulkanoConfig, VulkanoContext};
use vulkano_util::window::{VulkanoWindows, WindowDescriptor};

fn main() {
    let mut event_loop = EventLoop::new();
    let context = VulkanoContext::new(VulkanoConfig::default());
    let mut windows = VulkanoWindows::default();

    let _id = windows.create_window(
        &event_loop,
        &context,
        &WindowDescriptor {
            title: "Optics Simulator".into(),
            present_mode: PresentMode::Fifo,
            ..Default::default()
        },
        |_| {});

    let gfx_queue = context.graphics_queue();
    
    // More initializing?

    // Loop baby
    loop {

    }
}
