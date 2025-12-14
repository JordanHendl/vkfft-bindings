use alloc::vec::Vec;
use ash::vk;

use crate::ffi;

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

    // Keep Vulkan handles alive because VkFFTConfiguration stores pointers to them.
    phys: vk::PhysicalDevice,
    dev: vk::Device,
    queue: vk::Queue,
    command_pool: vk::CommandPool,
    buffer: vk::Buffer,

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

            phys: vk::PhysicalDevice::null(),
            dev: vk::Device::null(),
            queue: vk::Queue::null(),
            command_pool: vk::CommandPool::null(),
            buffer: vk::Buffer::null(),

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
    pub fn append(&mut self, command_buffer: vk::CommandBuffer, direction: i32) -> Result<(), i32> {
        if !self.initialized {
            return Err(-1);
        }

        let mut launch = unsafe { core::mem::zeroed::<ffi::VkFFTLaunchParams>() };

        // Many VkFFT versions want the command buffer passed as a raw handle inside launch params.
        // Field names may differ.
        let mut cmd = command_buffer;
        launch.commandBuffer = (&mut cmd as *mut vk::CommandBuffer).cast();

        let res =
            unsafe { ffi::vkfft_append(&mut self.app as *mut _, direction, &mut launch as *mut _) };

        if vkfft_ok(res) {
            Ok(())
        } else {
            Err(res as i32)
        }
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
