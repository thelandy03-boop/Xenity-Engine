cmake_minimum_required(VERSION 3.15)
project(Xenity_Engine C CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

# Default to Release if not specified
if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Release)
endif()

# Common compiler flags
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Wall")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wno-unknown-pragmas -Wno-unused-variable -Wno-unused-function -fpermissive")

# Include directories
set(ENGINE_INCLUDE_DIRS
    ${CMAKE_CURRENT_SOURCE_DIR}/Xenity_Engine/Source
    ${CMAKE_CURRENT_SOURCE_DIR}/Xenity_Engine/include
    ${CMAKE_CURRENT_SOURCE_DIR}/Xenity_Engine/include/bullet
    ${CMAKE_CURRENT_SOURCE_DIR}/Xenity_Engine/include/freetype
)

# Find System Packages
find_package(OpenGL REQUIRED)
find_package(Freetype REQUIRED)
find_package(OpenSSL REQUIRED)
find_package(CURL REQUIRED)

# Common sources (Engine core, Bullet physics, Stb, etc.)
file(GLOB_RECURSE ENGINE_CORE_SOURCES
    "${CMAKE_CURRENT_SOURCE_DIR}/Xenity_Engine/Source/engine/*.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/Xenity_Engine/Source/windows/*.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/Xenity_Engine/Source/gl.c"
    "${CMAKE_CURRENT_SOURCE_DIR}/Xenity_Engine/include/bullet/*.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/Xenity_Engine/include/stb_vorbis.c"
)

# Exclude console-specific files in engine core if any matched
list(FILTER ENGINE_CORE_SOURCES EXCLUDE REGEX "Source/engine/file_system/file_ps[0-9].*")
list(FILTER ENGINE_CORE_SOURCES EXCLUDE REGEX "Source/engine/file_system/file_psp.*")

# Editor-specific sources (Editor UI, menus, tools, imgui, implot)
file(GLOB_RECURSE EDITOR_SOURCES
    "${CMAKE_CURRENT_SOURCE_DIR}/Xenity_Engine/Source/editor/*.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/Xenity_Engine/include/imgui/*.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/Xenity_Engine/include/implot/*.cpp"
)

# Libraries
set(SDL3_LIB "${CMAKE_CURRENT_SOURCE_DIR}/Xenity_Engine/libs/libSDL3.so")
set(ASSIMP_LIB "${CMAKE_CURRENT_SOURCE_DIR}/Xenity_Engine/libs/libassimp.so")

set(COMMON_LIBS
    ${SDL3_LIB}
    ${ASSIMP_LIB}
    CURL::libcurl
    OpenGL::GL
    Freetype::Freetype
    OpenSSL::SSL
    OpenSSL::Crypto
    pthread
    dl
    m
)

# ------------------------------------------------------------------------------
# Target 1: Xenity_Editor (Shared Library loaded by xenity-editor GUI)
# ------------------------------------------------------------------------------
add_library(Xenity_Editor SHARED
    ${ENGINE_CORE_SOURCES}
    ${EDITOR_SOURCES}
)

target_include_directories(Xenity_Editor PUBLIC
    ${ENGINE_INCLUDE_DIRS}
    ${FREETYPE_INCLUDE_DIRS}
)

target_compile_definitions(Xenity_Editor PRIVATE
    __LINUX__=1
    EDITOR=1
    EXPORT=1
)

target_link_libraries(Xenity_Editor PRIVATE
    ${COMMON_LIBS}
)

# Set output name so both libXenity_Editor.so and Xenity_Editor.so are accessible
set_target_properties(Xenity_Editor PROPERTIES
    OUTPUT_NAME "Xenity_Editor"
    BUILD_WITH_INSTALL_RPATH TRUE
    INSTALL_RPATH "$ORIGIN;${CMAKE_CURRENT_SOURCE_DIR}/Xenity_Engine/libs"
)

add_custom_command(TARGET Xenity_Editor POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E create_symlink "$<TARGET_FILE:Xenity_Editor>" "${CMAKE_CURRENT_BINARY_DIR}/Xenity_Editor.so"
)

# ------------------------------------------------------------------------------
# Target 2: Xenity_Engine (Standalone Game Executable)
# ------------------------------------------------------------------------------
add_executable(Xenity_Engine
    ${ENGINE_CORE_SOURCES}
    "${CMAKE_CURRENT_SOURCE_DIR}/Xenity_Engine_Executable/main.cpp"
)

target_include_directories(Xenity_Engine PUBLIC
    ${ENGINE_INCLUDE_DIRS}
    ${FREETYPE_INCLUDE_DIRS}
)

target_compile_definitions(Xenity_Engine PRIVATE
    __LINUX__=1
)

target_link_libraries(Xenity_Engine PRIVATE
    ${COMMON_LIBS}
)

set_target_properties(Xenity_Engine PROPERTIES
    BUILD_WITH_INSTALL_RPATH TRUE
    INSTALL_RPATH "$ORIGIN;${CMAKE_CURRENT_SOURCE_DIR}/Xenity_Engine/libs"
)
