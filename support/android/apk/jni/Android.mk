# Android.mk for Eesha browser native library
# This makefile is used by ndk-build to package the Rust-compiled
# native library (libeesha.so) into the APK.
#
# Note: The primary build integration is via Gradle (see servoapp/build.gradle.kts),
# which copies .so files from the Rust target directory into jniLibs.
# This file is provided for compatibility with ndk-build based workflows.

LOCAL_PATH := $(call my-dir)

include $(CLEAR_VARS)

LOCAL_MODULE        := eesha
LOCAL_SRC_FILES     := ../libs/$(TARGET_ARCH_ABI)/libeesha.so
LOCAL_EXPORT_CFLAGS :=

# Prebuilt shared library
include $(PREBUILT_SHARED_LIBRARY)
