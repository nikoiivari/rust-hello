extern crate vulkano;
extern crate vulkano_win;
extern crate winit;

//use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
//use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState, SubpassContents};

//use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, RenderPassAbstract, Subpass};
//use vulkano::image::{ImageUsage, SwapchainImage};
//use vulkano::instance::{Instance, PhysicalDevice};

use vulkano::Version;
use vulkano::device::{Device, DeviceExtensions, Features};
use vulkano::device::physical::PhysicalDevice;

use vulkano_win::VkSurfaceBuild;
//use winit::event::{Event, WindowEvent};
//use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder};

use std::time::Duration;

fn main() {

    // Initialize Vulkan ======================================================== //
    
    let extensions = vulkano_win::required_extensions();
    let instance = vulkano::instance::Instance::new(None, Version::V1_1, &extensions, None)
        .unwrap();
    
    let physical = PhysicalDevice::enumerate(&instance)
        .next()
        .unwrap();
    
    let eventloop = winit::event_loop::EventLoop::new();
    
    let surface = WindowBuilder::new()
        .build_vk_surface(&eventloop, instance.clone())
        .unwrap();
    
    // Initialize device
    // https://docs.rs/vulkano/0.26.0/vulkano/device/index.html
    
    let queuefam = physical.queue_families().next().unwrap();
    let features = Features::none();
    let extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::none()
    };

        //match Device::new(physical, &features, &extensions, Some((queuefam, 1.0))) {
        //    Ok(d) => d,
        //    Err(err) => panic!("Couldn't build device: {:?}", err)
        //}
    
    
    // Get a command queue that can draw graphics
    


    // Create swapchain


    // Create vertex buffer
    
    
    // Create shaders


    // Create renderpass


    // Create pipeline


    // Create dynamic viewport


    // Create framebuffers


    // 
    
    // Mainloop ================================================================= //
    
    'running: loop {
        
        break 'running
        
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 480));
    }
    
    // Cleanup ================================================================== //
    // ...
}