// src/shim.cpp
extern "C" {

// Include VkFFT (and thus inline definitions)
#include "vkFFT.h"
#include "vkFFT/vkFFT_AppManagement/vkFFT_InitializeApp.h"
#include "vkFFT/vkFFT_AppManagement/vkFFT_RunApp.h"
#include "vkFFT/vkFFT_AppManagement/vkFFT_DeleteApp.h"

// Exported wrappers with stable C ABI names
VkFFTResult vkfft_initialize(VkFFTApplication* app, VkFFTConfiguration config) {
    return initializeVkFFT(app, config);
}

VkFFTResult vkfft_append(VkFFTApplication* app, int inverse, VkFFTLaunchParams* launch) {
    return VkFFTAppend(app, inverse, launch);
}

void vkfft_delete(VkFFTApplication* app) {
    deleteVkFFT(app);
}

} // extern "C"

