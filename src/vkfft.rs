use alloc::vec::Vec;
use ash::vk;

use crate::ffi;

fn vkfft_ok(r: ffi::VkFFTResult) -> bool {
    // Common bindgen output: a constant VKFFT_SUCCESS: u32
    // or a direct enum variant. Use whichever compiles.
    r == ffi::VkFFTResult::VKFFT_SUCCESS
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
    buffers: Vec<vk::Buffer>,
    output_buffers: Vec<vk::Buffer>,
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
            buffers: Vec::new(),
            output_buffers: Vec::new(),
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

    /// Finalize and create the VkFFT application.
    pub fn initialize(&mut self) -> Result<(), ffi::VkFFTResult> {
        let res = unsafe { ffi::vkfft_initialize(&mut self.app as *mut _, self.config) };
        if vkfft_ok(res) {
            Ok(())
        } else {
            Err(res)
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
