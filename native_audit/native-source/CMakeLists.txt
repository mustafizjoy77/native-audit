cmake_minimum_required(VERSION 3.22.1)
project("tun2proxy" C CXX)

set(CMAKE_CXX_STANDARD 17)

# Pre-built or Cargo corrosion integration for libtun2proxy.so
add_library(tun2proxy SHARED IMPORTED)
set_target_properties(tun2proxy PROPERTIES
    IMPORTED_LOCATION "${CMAKE_CURRENT_SOURCE_DIR}/../jniLibs/${ANDROID_ABI}/libtun2proxy.so"
)

include_directories(cpp)

find_library(log-lib log)
find_library(android-lib android)
