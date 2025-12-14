use alloc::vec::Vec;
use ash::vk;
use core::ffi::c_void;
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum VkfftError {
    Vkfft(ffi::VkFFTResult),
}

impl core::fmt::Display for VkfftError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Vkfft(res) => write!(f, "VkFFT error: {:?}", res),
        }
    }
}

impl std::error::Error for VkfftError {}

impl From<ffi::VkFFTResult> for VkfftError {
    fn from(value: ffi::VkFFTResult) -> Self {
        Self::Vkfft(value)
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
    buffer_sizes: [u64; 1],

    // Keep Vulkan handles alive because VkFFTConfiguration stores pointers to them.
    phys: vk::PhysicalDevice,
    dev: vk::Device,
    queue: vk::Queue,
    command_pool: vk::CommandPool,
    buffers: Vec<vk::Buffer>,
    output_buffers: Vec<vk::Buffer>,

    buffer_sizes: Vec<u64>,
    temp_buffer_sizes: Vec<u64>,
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
            buffer_sizes: [0],

            phys: vk::PhysicalDevice::null(),
            dev: vk::Device::null(),
            queue: vk::Queue::null(),
            command_pool: vk::CommandPool::null(),
            buffers: Vec::new(),
            output_buffers: Vec::new(),
            buffer_sizes: Vec::new(),
            temp_buffer_sizes: Vec::new(),

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

        // VkFFT size is usually 3- or 4-wide depending on version; pad with 1s.
        let mut tmp = [1u64; 4];
        for (i, v) in sizes.iter().take(4).enumerate() {
            tmp[i] = *v;
        }
        self.config.size[0] = tmp[0];
        self.config.size[1] = tmp[1];
        self.config.size[2] = tmp[2];
        self.config.size[3] = tmp[3];
    }

    /// Configure input (and optional output) device buffers.
    ///
    /// VkFFT accepts one or more buffers via `buffer`/`bufferNum` for in-place transforms or
    /// `inputBuffer`/`outputBuffer` for out-of-place transforms, depending on the version you
    /// generated bindings for. Provide at least one input buffer; optionally pass a matching slice
    /// of output buffers to enable out-of-place execution.
    ///
    /// # Errors
    ///
    /// Returns an error if any provided slice is empty, if input/output slice lengths differ, or if
    /// the slice length conflicts with an explicitly configured `numberBatches`.
    ///
    /// # Examples
    ///
    /// In-place transform using one buffer (common case):
    ///
    /// ```no_run
    /// # use vkfft_bindings::VkFft;
    /// # use ash::vk;
    /// # let buffer: vk::Buffer = vk::Buffer::null();
    /// let mut fft = VkFft::new();
    /// fft.configure_dimensions(&[1024]);
    /// fft.configure_buffers(&[buffer], None)?;
    /// # Ok::<(), &'static str>(())
    /// ```
    ///
    /// Out-of-place transform with separate input/output buffers:
    ///
    /// ```no_run
    /// # use vkfft_bindings::VkFft;
    /// # use ash::vk;
    /// # let input: vk::Buffer = vk::Buffer::null();
    /// # let output: vk::Buffer = vk::Buffer::null();
    /// let mut fft = VkFft::new();
    /// fft.configure_dimensions(&[256, 256]);
    /// fft.configure_buffers(&[input], Some(&[output]))?;
    /// # Ok::<(), &'static str>(())
    /// ```
    pub fn configure_buffers(
        &mut self,
        input_buffers: &[vk::Buffer],
        output_buffers: Option<&[vk::Buffer]>,
    ) -> Result<(), &'static str> {
        if input_buffers.is_empty() {
            return Err("configure_buffers: at least one input buffer is required");
        }

        if let Some(out) = output_buffers.as_ref() {
            if out.is_empty() {
                return Err("configure_buffers: output buffer slice must not be empty");
            }
            if out.len() != input_buffers.len() {
                return Err("configure_buffers: input/output buffer counts must match");
            }
        }

        if self.config.numberBatches > 1
            && input_buffers.len() > 1
            && input_buffers.len() as u64 != self.config.numberBatches
        {
            return Err("configure_buffers: buffer count must match configured numberBatches");
        }

        self.buffers.clear();
        self.buffers.extend_from_slice(input_buffers);
        self.config.bufferNum = self.buffers.len() as u64;
        self.config.buffer = self.buffers.as_mut_ptr().cast();

        if let Some(out) = output_buffers {
            self.output_buffers.clear();
            self.output_buffers.extend_from_slice(out);
            self.config.inputBufferNum = self.buffers.len() as u64;
            self.config.outputBufferNum = self.output_buffers.len() as u64;
            self.config.inputBuffer = self.buffers.as_mut_ptr().cast();
            self.config.outputBuffer = self.output_buffers.as_mut_ptr().cast();
        } else {
            self.output_buffers.clear();
            self.config.inputBufferNum = 0;
            self.config.outputBufferNum = 0;
            self.config.inputBuffer = core::ptr::null_mut();
            self.config.outputBuffer = core::ptr::null_mut();
        }

        Ok(())
    }

    /// Enable or disable double-precision FFT kernels.
    ///
    /// Default VkFFT behavior leaves `doublePrecision` at 0, selecting
    /// single-precision kernels. Setting this flag to `true` requests
    /// double-precision execution when the backend supports it.
    ///
    /// # Safety
    /// The caller must ensure the chosen device and buffer formats are
    /// compatible with double-precision computations.
    pub fn set_double_precision(&mut self, enabled: bool) {
        self.config.doublePrecision = enabled as u64;
    }

    /// Enable or disable half-precision FFT kernels.
    ///
    /// Default VkFFT behavior leaves `halfPrecision` at 0, meaning
    /// half-precision kernels are not used. Setting this flag to `true`
    /// requests half-precision execution when supported.
    ///
    /// # Safety
    /// The caller must ensure the data layout, device, and shader
    /// capabilities can safely handle half-precision computation.
    pub fn set_half_precision(&mut self, enabled: bool) {
        self.config.halfPrecision = enabled as u64;
    }

    /// Configure the number of batched FFTs.
    ///
    /// By default VkFFT uses a single batch when `numberBatches` is 0.
    /// Provide an explicit batch count to match the buffers you plan to
    /// process.
    ///
    /// # Safety
    /// The caller must ensure buffer sizes and strides are sized to fit
    /// the requested batch count.
    pub fn set_batch_count(&mut self, batches: u64) {
        self.config.numberBatches = batches;
    }

    /// Configure element strides for interleaved/strided data.
    ///
    /// The default VkFFT layout uses contiguous buffers with zeroed
    /// stride values. Provide explicit strides to match custom memory
    /// layouts.
    ///
    /// # Safety
    /// Strides must match the actual buffer layout; incorrect values can
    /// lead to out-of-bounds access in device memory.
    pub fn set_strides(
        &mut self,
        buffer_stride: [u64; 4],
        input_stride: Option<[u64; 4]>,
        output_stride: Option<[u64; 4]>,
    ) {
        self.config.bufferStride = buffer_stride;

        if let Some(stride) = input_stride {
            self.config.inputBufferStride = stride;
        }

        if let Some(stride) = output_stride {
            self.config.outputBufferStride = stride;
        }
    }

    /// Declare whether input and output buffers are already formatted for VkFFT.
    ///
    /// VkFFT defaults to `isInputFormatted = 0` and `isOutputFormatted = 0`,
    /// meaning it will apply its own formatting. Setting either flag to `true`
    /// tells VkFFT to treat the corresponding buffer as already formatted.
    ///
    /// # Safety
    /// The caller must ensure the buffers match VkFFT's expected formatted
    /// layout when these flags are enabled.
    pub fn set_layout_flags(&mut self, input_formatted: bool, output_formatted: bool) {
        self.config.isInputFormatted = input_formatted as u64;
        self.config.isOutputFormatted = output_formatted as u64;
    }

    /// Provide explicit sizes for the primary buffers in bytes.
    ///
    /// Defaults leave `bufferSize` as a null pointer, allowing VkFFT to
    /// infer sizes from dimensions. Supplying values is useful when VkFFT
    /// should validate or adjust for exact buffer sizes.
    ///
    /// # Safety
    /// Sizes must match the actual allocations referenced by `buffer` and
    /// related handles. The backing slice is stored in the wrapper to keep
    /// the pointers valid until reconfigured or dropped.
    pub fn set_buffer_sizes(&mut self, sizes: &[u64]) {
        self.buffer_sizes.clear();
        self.buffer_sizes.extend_from_slice(sizes);
        self.config.bufferSize = if self.buffer_sizes.is_empty() {
            core::ptr::null_mut()
        } else {
            self.buffer_sizes.as_mut_ptr()
        };
    }

    /// Provide explicit sizes for temporary buffers in bytes.
    ///
    /// Defaults leave `tempBufferSize` null, letting VkFFT pick temporary
    /// allocations. Passing concrete values allows tighter control over
    /// scratch-space planning.
    ///
    /// # Safety
    /// The caller is responsible for ensuring the provided sizes match the
    /// allocated temporary buffers and that the pointers remain valid. The
    /// wrapper retains the backing storage to maintain pointer validity.
    pub fn set_temp_buffer_sizes(&mut self, sizes: &[u64]) {
        self.temp_buffer_sizes.clear();
        self.temp_buffer_sizes.extend_from_slice(sizes);
        self.config.tempBufferSize = if self.temp_buffer_sizes.is_empty() {
            core::ptr::null_mut()
        } else {
            self.temp_buffer_sizes.as_mut_ptr()
        };
    }

    /// Set the size of the buffer in bytes.
    pub fn configure_buffer_size(&mut self, buffer_size: vk::DeviceSize) {
        self.buffer_sizes[0] = buffer_size;
        self.config.bufferSize = self.buffer_sizes.as_mut_ptr();
        self.config.bufferNum = 1;
    }

    /// Configure plan cache download to write the compiled plan into `blob_out`.
    ///
    /// # Safety
    /// `blob_out` must point to writable memory that is large enough for the
    /// plan string produced by VkFFT. Consult VkFFT documentation for expected
    /// sizes. The caller is responsible for ensuring the pointer remains valid
    /// until initialization completes.
    pub unsafe fn configure_plan_download(&mut self, blob_out: *mut c_void) {
        self.config.saveApplicationToString = 1;
        self.app.saveApplicationString = blob_out;
    }

    /// Configure plan cache upload by pointing VkFFT at a precompiled plan.
    ///
    /// # Safety
    /// `blob` must point to a valid plan string previously produced by
    /// VkFFT, and it must stay alive until initialization reads it.
    pub unsafe fn configure_plan_upload(&mut self, blob: *const u8) {
        self.config.loadApplicationFromString = 1;
        self.config.loadApplicationString = blob.cast_mut().cast();
    }

    /// Set the user callback pointer if the generated bindings expose
    /// callback hooks.
    ///
    /// The callback must use `extern "C"` calling conventions and adhere to
    /// the signature expected by the particular VkFFT build. This helper
    /// returns an error on bindings that do not expose callback pointers.
    pub unsafe fn set_callback_pointer(
        &mut self,
        _callback: unsafe extern "C" fn(*mut c_void),
    ) -> Result<(), &'static str> {
        // Current bindings do not expose callback pointer slots. Keep the
        // signature and documentation available so downstream users can adapt
        // when regenerating bindings from a VkFFT build that supports them.
        Err("Callback pointers are not exposed by the generated bindings")
    }

    /// Finalize and create the VkFFT application.
    pub fn initialize(&mut self) -> Result<(), VkfftError> {
        let res = unsafe { ffi::vkfft_initialize(&mut self.app as *mut _, self.config) };
        if vkfft_ok(res) {
            self.initialized = true;
            Ok(())
        } else {
            self.initialized = false;
            Err(res.into())
        }
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


#[cfg(all(feature = "wrapper", test))]
mod tests {
  
    use ash::vk::Handle;
    use super::*;
    use core::ffi::c_void;

    unsafe extern "C" fn dummy_callback(_userdata: *mut c_void) {}

    #[test]
    fn callback_pointer_api_is_exposed() {
        let mut fft = VkFft::new();

        let res = unsafe { fft.set_callback_pointer(dummy_callback) };

        // Current bindings do not expose callback pointer slots, but the API
        // shape should exist for forwards compatibility.
        assert!(res.is_err());
   }

    #[test]
    fn configures_dimensions_and_buffers() {
        let mut fft = VkFft::new();
        assert!(!fft.initialized);

        fft.configure_dimensions(&[16, 8]);
        assert_eq!(fft.config.FFTdim, 2);
        assert_eq!(fft.config.size[0], 16);
        assert_eq!(fft.config.size[1], 8);
        assert_eq!(fft.config.size[2], 1);
        assert_eq!(fft.config.size[3], 1);

        let dummy_buffer = vk::Buffer::from_raw(0xDEADBEEF_u64);
        fft.configure_buffers(dummy_buffer);

        let stored_ptr = fft.config.buffer as *mut vk::Buffer;
        assert_eq!(unsafe { *stored_ptr }, dummy_buffer);
    }

    #[test]
    fn configures_buffer_size() {
        let mut fft = VkFft::new();

        fft.configure_buffer_size(1024);
        assert_eq!(unsafe { *fft.config.bufferSize }, 1024);
        assert_eq!(fft.config.bufferNum, 1);
    }

    #[test]
    fn append_rejects_when_not_initialized() {
        let mut fft = VkFft::new();
        assert_eq!(fft.initialized, false);

        let err = fft
            .append(vk::CommandBuffer::null(), 1)
            .expect_err("append should fail before initialize");
        assert_eq!(err, -1);

    }
}
