#![cfg(all(feature = "wrapper", feature = "ash"))]

use std::ffi::CString;
use std::ptr;

use ash::{vk, Entry};
use vkfft_bindings::VkFft;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Minimal Vulkan setup to create a VkFFT plan and record one append call.
    let entry = unsafe { Entry::load()? };

    let app_name = CString::new("vkfft-ash-example")?;
    let app_info = vk::ApplicationInfo {
        s_type: vk::StructureType::APPLICATION_INFO,
        p_application_name: app_name.as_ptr(),
        application_version: vk::make_api_version(0, 0, 1, 0),
        p_engine_name: app_name.as_ptr(),
        engine_version: vk::make_api_version(0, 0, 1, 0),
        api_version: vk::API_VERSION_1_1,
        ..Default::default()
    };

    let instance_info = vk::InstanceCreateInfo {
        s_type: vk::StructureType::INSTANCE_CREATE_INFO,
        p_application_info: &app_info,
        ..Default::default()
    };
    let instance = unsafe { entry.create_instance(&instance_info, None)? };

    let physical_device = unsafe {
        instance
            .enumerate_physical_devices()?
            .into_iter()
            .next()
            .expect("No Vulkan device found")
    };

    let queue_family_index = find_queue_family_index(&instance, physical_device)
        .expect("No queue that supports graphics/compute");

    let queue_priorities = [1.0_f32];
    let queue_info = vk::DeviceQueueCreateInfo {
        s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
        queue_family_index,
        p_queue_priorities: queue_priorities.as_ptr(),
        queue_count: 1,
        ..Default::default()
    };

    let device_info = vk::DeviceCreateInfo {
        s_type: vk::StructureType::DEVICE_CREATE_INFO,
        queue_create_info_count: 1,
        p_queue_create_infos: &queue_info,
        ..Default::default()
    };
    let device = unsafe { instance.create_device(physical_device, &device_info, None)? };
    let queue = unsafe { device.get_device_queue(queue_family_index, 0) };

    let command_pool_info = vk::CommandPoolCreateInfo {
        s_type: vk::StructureType::COMMAND_POOL_CREATE_INFO,
        queue_family_index,
        flags: vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER,
        ..Default::default()
    };
    let command_pool = unsafe { device.create_command_pool(&command_pool_info, None)? };

    let allocate_info = vk::CommandBufferAllocateInfo {
        s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
        command_pool,
        level: vk::CommandBufferLevel::PRIMARY,
        command_buffer_count: 1,
        ..Default::default()
    };
    let command_buffer = unsafe { device.allocate_command_buffers(&allocate_info)? }[0];

    // Allocate a simple storage buffer for the FFT.
    let fft_len = 16_u64;
    let buffer_size: vk::DeviceSize = (std::mem::size_of::<f32>() as u64) * fft_len;
    let buffer_info = vk::BufferCreateInfo {
        s_type: vk::StructureType::BUFFER_CREATE_INFO,
        size: buffer_size,
        usage: vk::BufferUsageFlags::STORAGE_BUFFER,
        sharing_mode: vk::SharingMode::EXCLUSIVE,
        ..Default::default()
    };
    let buffer = unsafe { device.create_buffer(&buffer_info, None)? };

    let memory_requirements = unsafe { device.get_buffer_memory_requirements(buffer) };
    let memory_properties =
        unsafe { instance.get_physical_device_memory_properties(physical_device) };
    let memory_type_index = find_memory_type_index(
        &memory_properties,
        memory_requirements.memory_type_bits,
        vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
    )
    .expect("No compatible memory type for buffer");

    let allocation_info = vk::MemoryAllocateInfo {
        s_type: vk::StructureType::MEMORY_ALLOCATE_INFO,
        allocation_size: memory_requirements.size,
        memory_type_index,
        ..Default::default()
    };
    let buffer_memory = unsafe { device.allocate_memory(&allocation_info, None)? };
    unsafe {
        device.bind_buffer_memory(buffer, buffer_memory, 0)?;
    }

    let mut fft = VkFft::new();
    fft.configure_vulkan(physical_device, device.handle(), queue, command_pool);
    fft.configure_dimensions(&[fft_len]);
    fft.configure_buffers(buffer);
    fft.configure_buffer_size(buffer_size);

    fft.initialize().expect("Failed to initialize VkFFT");

    let begin_info = vk::CommandBufferBeginInfo {
        s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
        p_inheritance_info: ptr::null(),
        ..Default::default()
    };
    unsafe {
        device.begin_command_buffer(command_buffer, &begin_info)?;
    }

    fft.append(command_buffer, 1)
        .expect("Failed to append VkFFT");

    unsafe {
        device.end_command_buffer(command_buffer)?;
    }

    println!("Recorded VkFFT append into command buffer.");

    // Clean up Vulkan resources.
    unsafe {
        device.free_command_buffers(command_pool, &[command_buffer]);
        device.destroy_command_pool(command_pool, None);
        device.destroy_buffer(buffer, None);
        device.free_memory(buffer_memory, None);
        device.destroy_device(None);
        instance.destroy_instance(None);
    }

    Ok(())
}

fn find_queue_family_index(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
) -> Option<u32> {
    let families = unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
    families
        .iter()
        .enumerate()
        .find(|(_i, family)| {
            family.queue_count > 0
                && (family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                    || family.queue_flags.contains(vk::QueueFlags::COMPUTE))
        })
        .map(|(i, _)| i as u32)
}

fn find_memory_type_index(
    properties: &vk::PhysicalDeviceMemoryProperties,
    type_bits: u32,
    required: vk::MemoryPropertyFlags,
) -> Option<u32> {
    properties.memory_types[..properties.memory_type_count as usize]
        .iter()
        .enumerate()
        .find(|(i, mem_type)| {
            (type_bits & (1 << i)) != 0 && mem_type.property_flags.contains(required)
        })
        .map(|(i, _)| i as u32)
}
