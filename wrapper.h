#pragma once
#include <stdint.h>

#include "vkFFT.h"
#include "vkFFT/vkFFT_Structs/vkFFT_Structs.h"

// Declarations for our C ABI shim
#ifdef __cplusplus
extern "C" {
#endif

VkFFTResult vkfft_initialize(VkFFTApplication* app, VkFFTConfiguration config);
VkFFTResult vkfft_append(VkFFTApplication* app, int inverse, VkFFTLaunchParams* launch);
void vkfft_delete(VkFFTApplication* app);

#ifdef __cplusplus
}
#endif

