use ash::vk;
use core::ffi::c_void;

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

#[cfg(test)]
mod tests {
    use super::VkFft;
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
}
