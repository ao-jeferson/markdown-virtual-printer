cmake_minimum_required(VERSION 3.22.1)
project(mdvp_android_jni C)

add_library(printer_core SHARED IMPORTED GLOBAL)
set_target_properties(printer_core PROPERTIES
    IMPORTED_LOCATION "${CMAKE_CURRENT_LIST_DIR}/../jniLibs/${ANDROID_ABI}/libprinter_core.so")

add_library(mdvp_jni SHARED mdvp_jni.c)
target_compile_features(mdvp_jni PRIVATE c_std_11)
target_link_libraries(mdvp_jni PRIVATE printer_core log)
