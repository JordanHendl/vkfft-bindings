use ash::vk;
use core::fmt;

use crate::ffi;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct VkFftError {
    pub code: ffi::VkFFTResult,
    pub message: &'static str,
}

impl VkFftError {
    pub fn from_result(code: ffi::VkFFTResult) -> Self {
        Self {
            code,
            message: vkfft_result_to_str(code),
        }
    }
}

impl fmt::Display for VkFftError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub fn vkfft_result_to_str(result: ffi::VkFFTResult) -> &'static str {
    match result {
        ffi::VkFFTResult::VKFFT_SUCCESS => "VKFFT_SUCCESS",
        ffi::VkFFTResult::VKFFT_ERROR_MALLOC_FAILED => "VKFFT_ERROR_MALLOC_FAILED",
        ffi::VkFFTResult::VKFFT_ERROR_INSUFFICIENT_CODE_BUFFER => {
            "VKFFT_ERROR_INSUFFICIENT_CODE_BUFFER"
        }
        ffi::VkFFTResult::VKFFT_ERROR_INSUFFICIENT_TEMP_BUFFER => {
            "VKFFT_ERROR_INSUFFICIENT_TEMP_BUFFER"
        }
        ffi::VkFFTResult::VKFFT_ERROR_PLAN_NOT_INITIALIZED => "VKFFT_ERROR_PLAN_NOT_INITIALIZED",
        ffi::VkFFTResult::VKFFT_ERROR_NULL_TEMP_PASSED => "VKFFT_ERROR_NULL_TEMP_PASSED",
        ffi::VkFFTResult::VKFFT_ERROR_MATH_FAILED => "VKFFT_ERROR_MATH_FAILED",
        ffi::VkFFTResult::VKFFT_ERROR_FFTdim_GT_MAX_FFT_DIMENSIONS => {
            "VKFFT_ERROR_FFTdim_GT_MAX_FFT_DIMENSIONS"
        }
        ffi::VkFFTResult::VKFFT_ERROR_NONZERO_APP_INITIALIZATION => {
            "VKFFT_ERROR_NONZERO_APP_INITIALIZATION"
        }
        ffi::VkFFTResult::VKFFT_ERROR_INVALID_PHYSICAL_DEVICE => {
            "VKFFT_ERROR_INVALID_PHYSICAL_DEVICE"
        }
        ffi::VkFFTResult::VKFFT_ERROR_INVALID_DEVICE => "VKFFT_ERROR_INVALID_DEVICE",
        ffi::VkFFTResult::VKFFT_ERROR_INVALID_QUEUE => "VKFFT_ERROR_INVALID_QUEUE",
        ffi::VkFFTResult::VKFFT_ERROR_INVALID_COMMAND_POOL => "VKFFT_ERROR_INVALID_COMMAND_POOL",
        ffi::VkFFTResult::VKFFT_ERROR_INVALID_FENCE => "VKFFT_ERROR_INVALID_FENCE",
        ffi::VkFFTResult::VKFFT_ERROR_ONLY_FORWARD_FFT_INITIALIZED => {
            "VKFFT_ERROR_ONLY_FORWARD_FFT_INITIALIZED"
        }
        ffi::VkFFTResult::VKFFT_ERROR_ONLY_INVERSE_FFT_INITIALIZED => {
            "VKFFT_ERROR_ONLY_INVERSE_FFT_INITIALIZED"
        }
        ffi::VkFFTResult::VKFFT_ERROR_INVALID_CONTEXT => "VKFFT_ERROR_INVALID_CONTEXT",
        ffi::VkFFTResult::VKFFT_ERROR_INVALID_PLATFORM => "VKFFT_ERROR_INVALID_PLATFORM",
        ffi::VkFFTResult::VKFFT_ERROR_ENABLED_saveApplicationToString => {
            "VKFFT_ERROR_ENABLED_saveApplicationToString"
        }
        ffi::VkFFTResult::VKFFT_ERROR_EMPTY_FILE => "VKFFT_ERROR_EMPTY_FILE",
        ffi::VkFFTResult::VKFFT_ERROR_EMPTY_FFTdim => "VKFFT_ERROR_EMPTY_FFTdim",
        ffi::VkFFTResult::VKFFT_ERROR_EMPTY_size => "VKFFT_ERROR_EMPTY_size",
        ffi::VkFFTResult::VKFFT_ERROR_EMPTY_bufferSize => "VKFFT_ERROR_EMPTY_bufferSize",
        ffi::VkFFTResult::VKFFT_ERROR_EMPTY_buffer => "VKFFT_ERROR_EMPTY_buffer",
        ffi::VkFFTResult::VKFFT_ERROR_EMPTY_tempBufferSize => "VKFFT_ERROR_EMPTY_tempBufferSize",
        ffi::VkFFTResult::VKFFT_ERROR_EMPTY_tempBuffer => "VKFFT_ERROR_EMPTY_tempBuffer",
        ffi::VkFFTResult::VKFFT_ERROR_EMPTY_inputBufferSize => "VKFFT_ERROR_EMPTY_inputBufferSize",
        ffi::VkFFTResult::VKFFT_ERROR_EMPTY_inputBuffer => "VKFFT_ERROR_EMPTY_inputBuffer",
        ffi::VkFFTResult::VKFFT_ERROR_EMPTY_outputBufferSize => {
            "VKFFT_ERROR_EMPTY_outputBufferSize"
        }
        ffi::VkFFTResult::VKFFT_ERROR_EMPTY_outputBuffer => "VKFFT_ERROR_EMPTY_outputBuffer",
        ffi::VkFFTResult::VKFFT_ERROR_EMPTY_kernelSize => "VKFFT_ERROR_EMPTY_kernelSize",
        ffi::VkFFTResult::VKFFT_ERROR_EMPTY_kernel => "VKFFT_ERROR_EMPTY_kernel",
        ffi::VkFFTResult::VKFFT_ERROR_EMPTY_applicationString => {
            "VKFFT_ERROR_EMPTY_applicationString"
        }
        ffi::VkFFTResult::VKFFT_ERROR_EMPTY_useCustomBluesteinPaddingPattern_arrays => {
            "VKFFT_ERROR_EMPTY_useCustomBluesteinPaddingPattern_arrays"
        }
        ffi::VkFFTResult::VKFFT_ERROR_EMPTY_app => "VKFFT_ERROR_EMPTY_app",
        ffi::VkFFTResult::VKFFT_ERROR_INVALID_user_tempBuffer_too_small => {
            "VKFFT_ERROR_INVALID_user_tempBuffer_too_small"
        }
        ffi::VkFFTResult::VKFFT_ERROR_UNSUPPORTED_RADIX => "VKFFT_ERROR_UNSUPPORTED_RADIX",
        ffi::VkFFTResult::VKFFT_ERROR_UNSUPPORTED_FFT_LENGTH => {
            "VKFFT_ERROR_UNSUPPORTED_FFT_LENGTH"
        }
        ffi::VkFFTResult::VKFFT_ERROR_UNSUPPORTED_FFT_LENGTH_R2C => {
            "VKFFT_ERROR_UNSUPPORTED_FFT_LENGTH_R2C"
        }
        ffi::VkFFTResult::VKFFT_ERROR_UNSUPPORTED_FFT_LENGTH_R2R => {
            "VKFFT_ERROR_UNSUPPORTED_FFT_LENGTH_R2R"
        }
        ffi::VkFFTResult::VKFFT_ERROR_UNSUPPORTED_FFT_OMIT => "VKFFT_ERROR_UNSUPPORTED_FFT_OMIT",
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_ALLOCATE => "VKFFT_ERROR_FAILED_TO_ALLOCATE",
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_MAP_MEMORY => "VKFFT_ERROR_FAILED_TO_MAP_MEMORY",
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_ALLOCATE_COMMAND_BUFFERS => {
            "VKFFT_ERROR_FAILED_TO_ALLOCATE_COMMAND_BUFFERS"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_BEGIN_COMMAND_BUFFER => {
            "VKFFT_ERROR_FAILED_TO_BEGIN_COMMAND_BUFFER"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_END_COMMAND_BUFFER => {
            "VKFFT_ERROR_FAILED_TO_END_COMMAND_BUFFER"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_SUBMIT_QUEUE => {
            "VKFFT_ERROR_FAILED_TO_SUBMIT_QUEUE"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_WAIT_FOR_FENCES => {
            "VKFFT_ERROR_FAILED_TO_WAIT_FOR_FENCES"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_RESET_FENCES => {
            "VKFFT_ERROR_FAILED_TO_RESET_FENCES"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_CREATE_DESCRIPTOR_POOL => {
            "VKFFT_ERROR_FAILED_TO_CREATE_DESCRIPTOR_POOL"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_CREATE_DESCRIPTOR_SET_LAYOUT => {
            "VKFFT_ERROR_FAILED_TO_CREATE_DESCRIPTOR_SET_LAYOUT"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_ALLOCATE_DESCRIPTOR_SETS => {
            "VKFFT_ERROR_FAILED_TO_ALLOCATE_DESCRIPTOR_SETS"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_CREATE_PIPELINE_LAYOUT => {
            "VKFFT_ERROR_FAILED_TO_CREATE_PIPELINE_LAYOUT"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_SHADER_PREPROCESS => {
            "VKFFT_ERROR_FAILED_SHADER_PREPROCESS"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_SHADER_PARSE => "VKFFT_ERROR_FAILED_SHADER_PARSE",
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_SHADER_LINK => "VKFFT_ERROR_FAILED_SHADER_LINK",
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_SPIRV_GENERATE => "VKFFT_ERROR_FAILED_SPIRV_GENERATE",
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_CREATE_SHADER_MODULE => {
            "VKFFT_ERROR_FAILED_TO_CREATE_SHADER_MODULE"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_CREATE_INSTANCE => {
            "VKFFT_ERROR_FAILED_TO_CREATE_INSTANCE"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_SETUP_DEBUG_MESSENGER => {
            "VKFFT_ERROR_FAILED_TO_SETUP_DEBUG_MESSENGER"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_FIND_PHYSICAL_DEVICE => {
            "VKFFT_ERROR_FAILED_TO_FIND_PHYSICAL_DEVICE"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_CREATE_DEVICE => {
            "VKFFT_ERROR_FAILED_TO_CREATE_DEVICE"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_CREATE_FENCE => {
            "VKFFT_ERROR_FAILED_TO_CREATE_FENCE"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_CREATE_COMMAND_POOL => {
            "VKFFT_ERROR_FAILED_TO_CREATE_COMMAND_POOL"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_CREATE_BUFFER => {
            "VKFFT_ERROR_FAILED_TO_CREATE_BUFFER"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_ALLOCATE_MEMORY => {
            "VKFFT_ERROR_FAILED_TO_ALLOCATE_MEMORY"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_BIND_BUFFER_MEMORY => {
            "VKFFT_ERROR_FAILED_TO_BIND_BUFFER_MEMORY"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_FIND_MEMORY => "VKFFT_ERROR_FAILED_TO_FIND_MEMORY",
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_SYNCHRONIZE => "VKFFT_ERROR_FAILED_TO_SYNCHRONIZE",
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_COPY => "VKFFT_ERROR_FAILED_TO_COPY",
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_CREATE_PROGRAM => {
            "VKFFT_ERROR_FAILED_TO_CREATE_PROGRAM"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_COMPILE_PROGRAM => {
            "VKFFT_ERROR_FAILED_TO_COMPILE_PROGRAM"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_GET_CODE_SIZE => {
            "VKFFT_ERROR_FAILED_TO_GET_CODE_SIZE"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_GET_CODE => "VKFFT_ERROR_FAILED_TO_GET_CODE",
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_DESTROY_PROGRAM => {
            "VKFFT_ERROR_FAILED_TO_DESTROY_PROGRAM"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_LOAD_MODULE => "VKFFT_ERROR_FAILED_TO_LOAD_MODULE",
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_GET_FUNCTION => {
            "VKFFT_ERROR_FAILED_TO_GET_FUNCTION"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_SET_DYNAMIC_SHARED_MEMORY => {
            "VKFFT_ERROR_FAILED_TO_SET_DYNAMIC_SHARED_MEMORY"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_MODULE_GET_GLOBAL => {
            "VKFFT_ERROR_FAILED_TO_MODULE_GET_GLOBAL"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_LAUNCH_KERNEL => {
            "VKFFT_ERROR_FAILED_TO_LAUNCH_KERNEL"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_EVENT_RECORD => {
            "VKFFT_ERROR_FAILED_TO_EVENT_RECORD"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_ADD_NAME_EXPRESSION => {
            "VKFFT_ERROR_FAILED_TO_ADD_NAME_EXPRESSION"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_INITIALIZE => "VKFFT_ERROR_FAILED_TO_INITIALIZE",
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_SET_DEVICE_ID => {
            "VKFFT_ERROR_FAILED_TO_SET_DEVICE_ID"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_GET_DEVICE => "VKFFT_ERROR_FAILED_TO_GET_DEVICE",
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_CREATE_CONTEXT => {
            "VKFFT_ERROR_FAILED_TO_CREATE_CONTEXT"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_CREATE_PIPELINE => {
            "VKFFT_ERROR_FAILED_TO_CREATE_PIPELINE"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_SET_KERNEL_ARG => {
            "VKFFT_ERROR_FAILED_TO_SET_KERNEL_ARG"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_CREATE_COMMAND_QUEUE => {
            "VKFFT_ERROR_FAILED_TO_CREATE_COMMAND_QUEUE"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_RELEASE_COMMAND_QUEUE => {
            "VKFFT_ERROR_FAILED_TO_RELEASE_COMMAND_QUEUE"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_ENUMERATE_DEVICES => {
            "VKFFT_ERROR_FAILED_TO_ENUMERATE_DEVICES"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_GET_ATTRIBUTE => {
            "VKFFT_ERROR_FAILED_TO_GET_ATTRIBUTE"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_CREATE_EVENT => {
            "VKFFT_ERROR_FAILED_TO_CREATE_EVENT"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_CREATE_COMMAND_LIST => {
            "VKFFT_ERROR_FAILED_TO_CREATE_COMMAND_LIST"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_DESTROY_COMMAND_LIST => {
            "VKFFT_ERROR_FAILED_TO_DESTROY_COMMAND_LIST"
        }
        ffi::VkFFTResult::VKFFT_ERROR_FAILED_TO_SUBMIT_BARRIER => {
            "VKFFT_ERROR_FAILED_TO_SUBMIT_BARRIER"
        }
        _ => "Unknown VkFFT error",
    }
}

fn vkfft_ok(r: ffi::VkFFTResult) -> bool {
    // Common bindgen output: a constant VKFFT_SUCCESS: u32
    // or a direct enum variant. Use whichever compiles.
    r == ffi::VkFFTResult::VKFFT_SUCCESS
}

pub fn vkfft_check(result: ffi::VkFFTResult) -> Result<(), VkFftError> {
    if vkfft_ok(result) {
        Ok(())
    } else {
        Err(VkFftError::from_result(result))
    }
}

/// Thin wrapper around VkFFTApplication lifetime.
///
/// VkFFT is C-style; most functions return an error code.
/// This wrapper provides:
/// - create/destroy
/// - initialize (create VkFFT plan)
/// - append (record FFT dispatch into an existing command buffer)
pub struct VkFft {
    app: ffi::VkFFTApplication,
    config: ffi::VkFFTConfiguration,
    initialized: bool,

    // Keep Vulkan handles alive because VkFFTConfiguration stores pointers to them.
    phys: vk::PhysicalDevice,
    dev: vk::Device,
    queue: vk::Queue,
    command_pool: vk::CommandPool,
    buffer: vk::Buffer,
}

impl VkFft {
    /// Create a new wrapper with a zeroed VkFFTApplication.
    ///
    /// You must call `initialize` before `append_*`.
    pub fn new() -> Self {
        // VkFFTApplication is a plain C struct; zero-init is typical in VkFFT examples.
        let app = unsafe { core::mem::zeroed::<ffi::VkFFTApplication>() };
        let config = unsafe { core::mem::zeroed::<ffi::VkFFTConfiguration>() };

        Self {
            app,
            config,
            initialized: false,

            phys: vk::PhysicalDevice::null(),
            dev: vk::Device::null(),
            queue: vk::Queue::null(),
            command_pool: vk::CommandPool::null(),
            buffer: vk::Buffer::null(),
        }
    }

    /// Initialize VkFFT with common Vulkan handles from `ash`.
    ///
    /// This method sets up the VkFFTConfiguration with:
    /// - instance / physical_device / device / queue / command_pool
    ///
    /// You must also provide FFT dimensions and buffer pointers separately using `configure_*`
    /// before calling `initialize` (or extend this helper to do it all at once).
    pub fn configure_vulkan(
        &mut self,
        physical_device: vk::PhysicalDevice,
        device: vk::Device,
        queue: vk::Queue,
        command_pool: vk::CommandPool,
    ) {
        // VkFFT expects raw Vulkan handles; ash::vk types are repr(transparent) handles.
        // We pass them through as-is.

        // NOTE: VkFFTConfiguration fields differ slightly by VkFFT version.
        // The generated `ffi::VkFFTConfiguration` will tell you exact field names.
        // The below is representative; you may need to rename fields to match your VkFFT.
        self.phys = physical_device;
        self.dev = device;
        self.queue = queue;
        self.command_pool = command_pool;

        // Cast pointers-to-ash-handles to pointers-to-Vk* handles.
        // ash handle wrappers are repr(transparent) over u64, so the address is stable.
        self.config.physicalDevice = (&mut self.phys as *mut vk::PhysicalDevice).cast();
        self.config.device = (&mut self.dev as *mut vk::Device).cast();
        self.config.queue = (&mut self.queue as *mut vk::Queue).cast();
        self.config.commandPool = (&mut self.command_pool as *mut vk::CommandPool).cast();
    }

    /// Configure FFT sizes.
    ///
    /// Example: 2D FFT => pass [width, height] and set `dim = 2`.
    pub fn configure_dimensions(&mut self, sizes: &[u64]) {
        // VkFFT typically uses:
        //   config.FFTdim (u32) and config.size (u64[3])
        // Adjust names if your VkFFT differs.
        self.config.FFTdim = sizes.len() as u64;

        // VkFFT size is usually 3-wide; pad with 1s.
        let mut tmp = [1u64; 3];
        for (i, v) in sizes.iter().take(3).enumerate() {
            tmp[i] = *v;
        }
        self.config.size[0] = tmp[0];
        self.config.size[1] = tmp[1];
        self.config.size[2] = tmp[2];
    }

    /// Configure input/output buffers (device buffers).
    ///
    /// VkFFT generally wants pointers to VkBuffer handles (or arrays of them) stored as `u64`/void*,
    /// depending on version. Here we assume the common `buffer` / `bufferSize` style.
    ///
    /// You will need to confirm the exact field names in your generated bindings and adjust.
    pub fn configure_buffers(&mut self, buffer: vk::Buffer) {
        // Common pattern: config.buffer = &vk_buffer_handle
        // Many VkFFT examples store VkBuffer as a u64.
        // Bindgen will reveal the type; this is the most frequent shape:
        self.buffer = buffer;
        self.config.buffer = (&mut self.buffer as *mut vk::Buffer).cast();
    }

    /// Finalize and create the VkFFT application.
    pub fn initialize(&mut self) -> Result<(), VkFftError> {
        let res = unsafe { ffi::vkfft_initialize(&mut self.app as *mut _, self.config) };
        vkfft_check(res).map(|_| {
            self.initialized = true;
        })
    }

    /// Record FFT dispatch into an existing command buffer.
    ///
    /// Direction is usually `-1` inverse, `1` forward in VkFFT.
    pub fn append(
        &mut self,
        command_buffer: vk::CommandBuffer,
        direction: i32,
    ) -> Result<(), VkFftError> {
        if !self.initialized {
            return Err(VkFftError::from_result(
                ffi::VkFFTResult::VKFFT_ERROR_PLAN_NOT_INITIALIZED,
            ));
        }

        let mut launch = unsafe { core::mem::zeroed::<ffi::VkFFTLaunchParams>() };

        // Many VkFFT versions want the command buffer passed as a raw handle inside launch params.
        // Field names may differ.
        let mut cmd = command_buffer;
        launch.commandBuffer = (&mut cmd as *mut vk::CommandBuffer).cast();

        let res =
            unsafe { ffi::vkfft_append(&mut self.app as *mut _, direction, &mut launch as *mut _) };

        vkfft_check(res)
    }
}

impl Drop for VkFft {
    fn drop(&mut self) {
        if self.initialized {
            unsafe {
                let _ = ffi::vkfft_delete(&mut self.app as *mut _);
            }
        }
    }
}
