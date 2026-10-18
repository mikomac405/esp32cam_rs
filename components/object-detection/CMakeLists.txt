# ESP-IDF component for the Edge Impulse C++ library export (FOMO can detector).
#
# edge-impulse-sdk/ is a git submodule (edgeimpulse/inferencing-sdk-cpp).
# When you re-export a model, copy only model-parameters/ and tflite-model/
# from the export - keep this file (the export's CMakeLists.txt is for Zephyr)
# and check the submodule out at the SDK version the export was built with.

set(EI_DIR ${CMAKE_CURRENT_LIST_DIR})
set(EI_SDK ${EI_DIR}/edge-impulse-sdk)

if(NOT EXISTS ${EI_DIR}/model-parameters/model_metadata.h)
    message(FATAL_ERROR
        "Edge Impulse model missing in ${EI_DIR}.\n"
        "The model is not in git (Edge Impulse ToS) - export it from Edge Impulse "
        "Studio as a C++ library and copy model-parameters/ and tflite-model/ here. "
        "See README.md, section 'Object detection model'.")
endif()
if(NOT EXISTS ${EI_SDK}/cmake/utils.cmake)
    message(FATAL_ERROR
        "Edge Impulse SDK submodule missing - run: git submodule update --init")
endif()

include(${EI_SDK}/cmake/utils.cmake)

# SDK sources. CMSIS is ARM-only code; the per-platform porting layers are
# guarded by EI_PORTING_* macros, so only the Espressif one is compiled in.
RECURSIVE_FIND_FILE_EXCLUDE_DIR(EI_CPP_SRCS "${EI_SDK}" "CMSIS" "*.cpp")
RECURSIVE_FIND_FILE_EXCLUDE_DIR(EI_CC_SRCS  "${EI_SDK}" "CMSIS" "*.cc")
RECURSIVE_FIND_FILE_EXCLUDE_DIR(EI_C_SRCS   "${EI_SDK}" "CMSIS" "*.c")

# The one CMSIS-DSP function the SDK's DSP code uses on non-ARM targets.
list(APPEND EI_C_SRCS "${EI_SDK}/CMSIS/DSP/Source/StatisticsFunctions/arm_rms_f32.c")

# ESP-NN (Espressif-optimized NN kernels) and esp-dsp FFT assembly.
# The ESP32-S3/P4-only files are guarded internally and compile to nothing here.
RECURSIVE_FIND_FILE_APPEND(EI_ASM_SRCS "${EI_SDK}/porting/espressif/ESP-NN" "*.S")
RECURSIVE_FIND_FILE_APPEND(EI_ASM_SRCS "${EI_SDK}/porting/espressif/ESP-NN" "*.s")
RECURSIVE_FIND_FILE_APPEND(EI_ASM_SRCS "${EI_SDK}/porting/espressif/esp-dsp/modules/fft" "*.S")
RECURSIVE_FIND_FILE_APPEND(EI_ASM_SRCS "${EI_SDK}/porting/espressif/esp-dsp/modules/fft" "*.s")

# The trained model (EON-compiled).
RECURSIVE_FIND_FILE(MODEL_SRCS "${EI_DIR}/tflite-model" "*.cpp")

idf_component_register(
    SRCS
        ${EI_CPP_SRCS} ${EI_CC_SRCS} ${EI_C_SRCS} ${EI_ASM_SRCS}
        ${MODEL_SRCS}
    INCLUDE_DIRS
        "."     # SDK headers include each other as "edge-impulse-sdk/...", "model-parameters/..."
    PRIV_INCLUDE_DIRS
        "edge-impulse-sdk"
        "edge-impulse-sdk/porting"
        "edge-impulse-sdk/porting/espressif/ESP-NN/include"
        "edge-impulse-sdk/porting/espressif/ESP-NN/src/common"
        "edge-impulse-sdk/porting/espressif/esp-dsp/modules/common/include"
        "edge-impulse-sdk/porting/espressif/esp-dsp/modules/fft/include"
    PRIV_REQUIRES
        esp_timer   # ei_read_timer_us() in the Espressif porting layer
)

target_compile_options(${COMPONENT_LIB} PRIVATE
    # The SDK detects ESP-IDF via CONFIG_IDF_TARGET_ESP32, but only includes
    # sdkconfig.h *after* that check. Force it in, otherwise the SDK silently
    # builds without the Espressif porting layer and without ESP-NN.
    -include sdkconfig.h
    # Inference speed: build this component optimized regardless of the
    # project-wide (debug) optimization level.
    -O2
    # Vendored third-party code: don't let its warnings fail the build.
    -Wno-error
)
