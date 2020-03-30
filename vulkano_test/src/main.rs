// Common
use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::PhysicalDevice;

// Device
use vulkano::device::Device;
use vulkano::device::DeviceExtensions;
use vulkano::device::Features;

// Buffer
use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;

// Commands
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::CommandBuffer;

// Sync, wait until finish computing
use vulkano::sync::GpuFuture;


// Pipeline
use vulkano::pipeline::ComputePipeline;
use vulkano::descriptor::PipelineLayoutAbstract;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use std::sync::Arc;

// Image processing
use vulkano::format::Format;
use vulkano::image::Dimensions;
use vulkano::image::StorageImage;
use vulkano::format::ClearValue;

// For graphics
use vulkano::pipeline::GraphicsPipeline;
use vulkano::framebuffer::Subpass;
use vulkano::command_buffer::DynamicState;
use vulkano::pipeline::viewport::Viewport;

// Cgmath for testing purposes
use cgmath::Vector3;
use cgmath::prelude::*;

// For profiling
use std::time::{Duration, Instant};

use image::ImageBuffer;
use image::Rgba;
use vulkano::framebuffer::Framebuffer;



fn main() {
    let instance = Instance::new(None, &InstanceExtensions::none(), None)
    .expect("failed to create instance");

    let physical = PhysicalDevice::enumerate(&instance).next().expect("no device available");

    for family in physical.queue_families() {
        println!("Found a queue family with {:?} queue(s)", family.queues_count());
    }
    
    let queue_family = physical.queue_families()
    .find(|&q| q.supports_graphics())
    .expect("couldn't find a graphical queue family");

    let (device, mut queues) = {
        /*Device::new(physical, &Features::none(), &DeviceExtensions::none(),
                    [(queue_family, 0.5)].iter().cloned()).expect("failed to create device")*/
        Device::new(physical, &Features::none(), 
                    &DeviceExtensions{khr_storage_buffer_storage_class:true, ..DeviceExtensions::none()},
                    [(queue_family, 0.5)].iter().cloned()).expect("failed to create device")
    };
    
    let queue = queues.next().unwrap();


    // Test of interfering with buffer ***************
    let d = Vector3::new(1., 2., 3.);
    let v = Vector3::new(1., 2., 4.);
    //let c = data.cross(v);
    //let d = data.dot(v);
    //println!("{:?}", d);
    struct MyStruct {
        a: Vector3<f32>,
    }
    
    let data = MyStruct { a: v };
    let buffer = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(),
                                            true, data).expect("failed to create buffer");

    let mut content = buffer.write().unwrap();
    content.a = content.a.cross(d);
    println!("{:?}", content.a);

    let source_content = 0 .. 64;
    let source = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(),
                                            false, source_content).expect("failed to create buffer");
    // ***********************************************

    // Test of moving buffers ************************
    // Filled with zeros, to check if move actually occurs
    let dest_content = (0 .. 64).map(|_| 0);
    let dest = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(),
                                          false, dest_content).expect("failed to create buffer");


    let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap()
        .copy_buffer(source.clone(), dest.clone()).unwrap()
        .build().unwrap();
    
    let finished = command_buffer.execute(queue.clone()).unwrap();

    finished.then_signal_fence_and_flush().unwrap()
    .wait(None).unwrap();

    let src_content = source.read().unwrap();
    let dest_content = dest.read().unwrap();
    assert_eq!(&*src_content, &*dest_content);
    // ************************************************


    // Test parallel computing features ***************
    let data_iter = 0 .. 65536;
    let data_buffer = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(),
                                                 false, data_iter).expect("failed to create buffer");
    // OpenGL Shading Language

    let pipeline = Arc::new({
        mod cs {
            vulkano_shaders::shader!{
                ty: "compute",
                // We declare what directories to search for when using the `#include <...>`
                // syntax. Specified directories have descending priorities based on their order.
                include: [ "src/bin/shader-include/standard-shaders" ],
                src: "
                    #version 460
                    // Substitutes this line with the contents of the file `common.glsl` found in one of the standard
                    // `include` directories specified above.
                    // Note, that relative inclusion (`#include \"...\"`), although it falls back to standard
                    // inclusion, should not be used for **embedded** shader source, as it may be misleading and/or
                    // confusing.
                    #include <common.glsl>
                    layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;
                    layout(set = 0, binding = 0) buffer Data {
                       uint data[];
                    } data;

                    /*uint multiply_by_3(in uint arg) {
                        return 3 * arg;
                    }
                    uint multiply_by_12(in uint arg) {
                        return 2 * multiply_by_3(multiply_by_2(arg));
                    }*/
                    void main() {
                       uint idx = gl_GlobalInvocationID.x;
                       data.data[idx] = multiply_by_12(data.data[idx]);
                    }
                "
           }
       }
       let shader = cs::Shader::load(device.clone()).unwrap();
       ComputePipeline::new(device.clone(), &shader.main_entry_point(), &()).unwrap()
   });

   // old code, for reference purposes only
    /*let shader = cs::Shader::load(device.clone())
    .expect("failed to create shader module");

    let compute_pipeline = Arc::new(ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
    .expect("failed to create compute pipeline"));*/

    let layout = pipeline.layout().descriptor_set_layout(0).unwrap();

    let set = Arc::new(PersistentDescriptorSet::start(layout.clone())
        .add_buffer(data_buffer.clone()).unwrap()
        .build().unwrap()
    );

    let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap()
        .dispatch([1024, 1, 1], pipeline.clone(), set.clone(), ()).unwrap()
        .build().unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();
    finished.then_signal_fence_and_flush().unwrap()
    .wait(None).unwrap();

    let content = data_buffer.read().unwrap();
    for (n, val) in content.iter().enumerate() {
        assert_eq!(*val, n as u32 * 12);
    }

    println!("Everything succeeded!");


    // ************************************************


    // Test image processing **************************
    let now = Instant::now();
    // Storage image is more about buffer to store multiple values than actual image
    let image = StorageImage::new(device.clone(), Dimensions::Dim2d { width: 1024, height: 1024 },
                              Format::R8G8B8A8Unorm, Some(queue.family())).unwrap();

    // It is not generally available to modify image manually, ask for gpu to do this, fill with color
    // bcz R8G8B8A8Unorm - clear value is float
    let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap()
        .clear_color_image(image.clone(), ClearValue::Float([0.0, 0.0, 1.0, 1.0])).unwrap()
        .build().unwrap();
    
    // Buffer to store image
    let image_buf = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(),
                                         false, (0 .. 1024 * 1024 * 4).map(|_| 0u8))
                                         .expect("failed to create buffer");
    // Copying image to image_buf
    let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap()
        .clear_color_image(image.clone(), ClearValue::Float([0.0, 0.0, 1.0, 1.0])).unwrap()
        .copy_image_to_buffer(image.clone(), image_buf.clone()).unwrap()
        .build().unwrap();

    // Wait until copying is being finished
    let finished = command_buffer.execute(queue.clone()).unwrap();
    finished.then_signal_fence_and_flush().unwrap()
        .wait(None).unwrap();

    use image::{ImageBuffer, Rgba};

    let buffer_content = image_buf.read().unwrap();
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();

    image.save("image.png").unwrap();
    println!("Simple blue image  succeeded");
    println!("Time in ms: {}", now.elapsed().as_millis());
    // ************************************************

    // Test fractals image ****************************
    let now = Instant::now();
    let image = StorageImage::new(device.clone(), Dimensions::Dim2d { width: 1024, height: 1024 },
                                  Format::R8G8B8A8Unorm, Some(queue.family())).unwrap();

    let pipeline = Arc::new({    
        mod cs {
            vulkano_shaders::shader!{
                ty: "compute",
                src: "
                #version 460
                layout(local_size_x = 8, local_size_y = 8, local_size_z = 1) in;
                layout(set = 0, binding = 0, rgba8) uniform writeonly image2D img;
                void main() {
                    vec2 norm_coordinates = (gl_GlobalInvocationID.xy + vec2(0.5)) / vec2(imageSize(img));
                    vec2 c = (norm_coordinates - vec2(0.5)) * 2.0 - vec2(1.0, 0.0);
                    vec2 z = vec2(0.0, 0.0);
                    float i;
                    for (i = 0.0; i < 1.0; i += 0.005) {
                        z = vec2(
                            z.x * z.x - z.y * z.y + c.x,
                            z.y * z.x + z.x * z.y + c.y
                        );
                        if (length(z) > 4.0) {
                            break;
                        }
                    }
                    vec4 to_write = vec4(vec3(i), 1.0);
                    imageStore(img, ivec2(gl_GlobalInvocationID.xy), to_write);
                }
            "
            }
        }
        let shader = cs::Shader::load(device.clone()).unwrap();
        ComputePipeline::new(device.clone(), &shader.main_entry_point(), &()).unwrap()
    });

    let layout = pipeline.layout().descriptor_set_layout(0).unwrap();

    let set = Arc::new(PersistentDescriptorSet::start(layout.clone())
        .add_image(image.clone()).unwrap()
        .build().unwrap()
    );

    let buf = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(),
                                             false, (0 .. 1024 * 1024 * 4).map(|_| 0u8))
                                             .expect("failed to create buffer");

    let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap()
        .dispatch([1024 / 8, 1024 / 8, 1], pipeline.clone(), set.clone(), ()).unwrap()
        .copy_image_to_buffer(image.clone(), buf.clone()).unwrap()
        .build().unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();
    finished.then_signal_fence_and_flush().unwrap()
        .wait(None).unwrap();

    let buffer_content = buf.read().unwrap();
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();
    image.save("image.png").unwrap();
    println!("Fractals succeeded");
    println!("Time in ms: {}", now.elapsed().as_millis());

    // ************************************************

    // Test graphical side of Vulkan API **************
    let now = Instant::now();


    let image = StorageImage::new(device.clone(), Dimensions::Dim2d { width: 1024, height: 1024 },
                                  Format::R8G8B8A8Unorm, Some(queue.family())).unwrap();

    let buf = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(),
                                             false, (0 .. 1024 * 1024 * 4).map(|_| 0u8))
                                             .expect("failed to create buffer");

    let p1 = cgmath::Point2 {x: -0.5, y: -0.5};
    let p2 = cgmath::Point2 {x: 0., y: 0.5};
    let p3 = cgmath::Point2 {x: 0.5, y: -0.25};

    let vertex_buffer = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), 
                                                false, vec![p1, p2, p3].into_iter()).unwrap();

    // Not working with cgmath for now, vertex instead
    #[derive(Default, Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }
    vulkano::impl_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let vertex_buffer = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::vertex_buffer(),
                                                    false, vec![vertex1, vertex2, vertex3].into_iter()).unwrap();


    let render_pass = Arc::new(vulkano::single_pass_renderpass!(device.clone(),
        attachments: {
            color: {
                load: Clear,
                store: Store,
                format: Format::R8G8B8A8Unorm,
                samples: 1,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {}
        }
    ).unwrap());

    let framebuffer = Arc::new(Framebuffer::start(render_pass.clone())
        .add(image.clone()).unwrap()
        .build().unwrap());

    mod vs {
        vulkano_shaders::shader!{
            ty: "vertex",
            src:"
            #version 460
            layout(location = 0) in vec2 position;
            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
            "
        }
    }
    mod fs {
        vulkano_shaders::shader!{
            ty: "fragment",
            src:"
            #version 450        
            layout(location = 0) out vec4 f_color;
            void main() {
                f_color = vec4(1.0, 0.0, 0.0, 1.0);
            }
            "
        }
    }

    let vs = vs::Shader::load(device.clone()).expect("failed to create shader module");
    let fs = fs::Shader::load(device.clone()).expect("failed to create shader module");

    let pipeline = Arc::new(GraphicsPipeline::start()
    // Defines what kind of vertex input is expected.
    .vertex_input_single_buffer::<Vertex>()
    // The vertex shader.
    .vertex_shader(vs.main_entry_point(), ())
    // Defines the viewport (explanations below).
    .viewports_dynamic_scissors_irrelevant(1)
    // The fragment shader.
    .fragment_shader(fs.main_entry_point(), ())
    // This graphics pipeline object concerns the first pass of the render pass.
    .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
    // Now that everything is specified, we call `build`.
    .build(device.clone())
    .unwrap());

    let dynamic_state = DynamicState {
        viewports: Some(vec![Viewport {
            origin: [0.0, 0.0],
            dimensions: [1024.0, 1024.0],
            depth_range: 0.0 .. 1.0,
        }]),
        .. DynamicState::none()
    };
    
    let command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family()).unwrap()
        .begin_render_pass(framebuffer.clone(), false, vec![[0.0, 0.0, 1.0, 1.0].into()])
        .unwrap()
    
        .draw(pipeline.clone(), &dynamic_state, vertex_buffer.clone(), (), ())
        .unwrap()
    
        .end_render_pass()
        .unwrap()

        .copy_image_to_buffer(image.clone(), buf.clone())
        .unwrap()

        .build()
        .unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();
    finished.then_signal_fence_and_flush().unwrap()
        .wait(None).unwrap();

    let buffer_content = buf.read().unwrap();
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();
    image.save("triangle.png").unwrap();

    
    println!("Time in ms: {}", now.elapsed().as_millis());
    // ************************************************

}
