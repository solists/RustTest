use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::PhysicalDevice;

use vulkano::device::Device;
use vulkano::device::DeviceExtensions;
use vulkano::device::Features;

use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;

use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::CommandBuffer;

use vulkano::sync::GpuFuture;

use vulkano::pipeline::ComputePipeline;

use vulkano::descriptor::PipelineLayoutAbstract;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use std::sync::Arc;


use cgmath::Vector3;
use cgmath::prelude::*;



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
    mod cs {
        vulkano_shaders::shader!{
            ty: "compute",
            src: "
#version 460
layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;
layout(set = 0, binding = 0) buffer Data {
    uint data[];
} buf;
void main() {
    uint idx = gl_GlobalInvocationID.x;
    buf.data[idx] *= 12;
}"
        }
    }

    let pipeline = Arc::new({
        mod cs {
            vulkano_shaders::shader!{
                ty: "compute",
                // We declare what directories to search for when using the `#include <...>`
                // syntax. Specified directories have descending priorities based on their order.
                include: [ "src/bin/shader-include/standard-shaders" ],
                src: "
                    #version 450
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


}
