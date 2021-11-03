extern crate vulkano;
extern crate vulkano_win;
extern crate winit;

//use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
//use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState, SubpassContents};
//use vulkano::device::{Device, DeviceExtensions};
//use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, RenderPassAbstract, Subpass};
//use vulkano::image::{ImageUsage, SwapchainImage};
//use vulkano::instance::{Instance, PhysicalDevice};

use vulkano::Version;

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
    
    let physical = vulkano::device::physical::PhysicalDevice::enumerate(&instance)
        .next()
        .unwrap();
    
    let eventloop = winit::event_loop::EventLoop::new();
    
    let surface = WindowBuilder::new()
        .build_vk_surface(&eventloop, instance.clone())
        .unwrap();
    
    // Get a command queue that can draw graphics
    
    
    // Initialize device


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