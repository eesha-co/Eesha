plugins {
    id("com.android.application")
}

android {
    namespace = "org.eesha.browser"
    compileSdk = 33

    defaultConfig {
        applicationId = "org.eesha.browser"
        minSdk = 24
        targetSdk = 33
        versionCode = 1
        versionName = "1.0"
        // NOTE: Do NOT set ndk.abiFilters here - it conflicts with splits.abi.
        // The jniLibs directory will only contain the ABI built by CI.
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
        debug {
            isDebuggable = true
            isJniDebuggable = true
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }

    // Splits per ABI to reduce APK size
    splits {
        abi {
            isEnable = true
            reset()
            include("arm64-v8a", "armeabi-v7a", "x86_64")
            isUniversalApk = false
        }
    }

    sourceSets {
        getByName("main") {
            // Copy native .so files from the Rust build output into jniLibs.
            // The Rust cross-compilation targets map to Android ABI directories:
            //   aarch64-linux-android  -> arm64-v8a
            //   armv7-linux-androideabi -> armeabi-v7a
            //   x86_64-linux-android   -> x86_64
            jniLibs.srcDirs(listOf("src/main/jniLibs"))
        }
    }
}

// Task to copy native Rust libraries from the target directory into jniLibs
// for each supported Android ABI.
val copyNativeLibs by tasks.registering(Copy::class) {
    description = "Copy native Rust .so libraries into the Android jniLibs directory"

    val targetDir = file("../../../target")

    // arm64-v8a
    from("$targetDir/aarch64-linux-android/release/libeesha.so") {
        into("arm64-v8a")
    }

    // armeabi-v7a
    from("$targetDir/armv7-linux-androideabi/release/libeesha.so") {
        into("armeabi-v7a")
    }

    // x86_64
    from("$targetDir/x86_64-linux-android/release/libeesha.so") {
        into("x86_64")
    }

    into(file("src/main/jniLibs"))

    // Only copy if the source files exist
    eachFile {
        if (!file.absolutePath.contains("libeesha.so")) {
            exclude()
        }
    }
}

tasks.named("preBuild") {
    dependsOn(copyNativeLibs)
}

dependencies {
    implementation("androidx.core:core:1.10.1")
}
