# cmake_minimum_required(VERSION 3.24)

# project(cxx_qt_test_project)

# # Rust always links against non-debug Windows runtime on *-msvc targets
# # Note it is best to set this on the command line to ensure all targets are consistent
# # https://github.com/corrosion-rs/corrosion/blob/master/doc/src/common_issues.md#linking-debug-cc-libraries-into-rust-fails-on-windows-msvc-targets
# # https://github.com/rust-lang/rust/issues/39016
# if (CMAKE_CXX_COMPILER_ID STREQUAL "MSVC")
#   set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreadedDLL")
# endif()

# set(CMAKE_AUTOMOC ON)
# set(CMAKE_CXX_STANDARD 17)
# set(CMAKE_CXX_STANDARD_REQUIRED ON)

# if(NOT USE_QT5)
#   find_package(Qt6 COMPONENTS Core Gui Qml QuickControls2 QmlImportScanner)
# endif()

# if(NOT Qt6_FOUND)
#   find_package(Qt5 5.15 COMPONENTS Core Gui Qml QuickControls2 QmlImportScanner REQUIRED)
# endif()

# find_package(CxxQt QUIET)
# if(NOT CxxQt_FOUND)
#     include(FetchContent)
#     FetchContent_Declare(
#         CxxQt
#         GIT_REPOSITORY https://github.com/kdab/cxx-qt-cmake.git
#         GIT_TAG main
#     )

#     FetchContent_MakeAvailable(CxxQt)
# endif()

# cxx_qt_import_crate(MANIFEST_PATH rust/Cargo.toml CRATES cxx_qt_test)
# target_link_libraries(cxx_qt_test INTERFACE Qt::Core Qt::Gui Qt::Qml Qt::QuickControls2)

# cxx_qt_import_qml_module(cxx_qt_test_qml_module
#     URI "com.nb.cxx_qt_test"
#     SOURCE_CRATE cxx_qt_test)

# # Define the executable with the C++ source
# add_executable(${PROJECT_NAME} cpp/main.cpp)

# # Link to the Rust library
# target_link_libraries(${PROJECT_NAME} PRIVATE cxx_qt_test_qml_module)

# # If we are using a statically linked Qt then we need to import any qml plugins
# qt_import_qml_plugins(${PROJECT_NAME})

cmake_minimum_required(VERSION 3.24)

project(cxx_qt_test_project)

# Rust always links against non-debug Windows runtime on *-msvc targets
# Note it is best to set this on the command line to ensure all targets are consistent
# https://github.com/corrosion-rs/corrosion/blob/master/doc/src/common_issues.md#linking-debug-cc-libraries-into-rust-fails-on-windows-msvc-targets
# https://github.com/rust-lang/rust/issues/39016
if (CMAKE_CXX_COMPILER_ID STREQUAL "MSVC")
  set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreadedDLL")
endif()

set(CMAKE_AUTOMOC ON)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(NOT USE_QT5)
    find_package(Qt6 COMPONENTS Core Gui Qml QuickControls2 QmlImportScanner)
endif()
if(NOT Qt6_FOUND)
    find_package(Qt5 5.15 COMPONENTS Core Gui Qml QuickControls2 QmlImportScanner REQUIRED)
endif()

find_package(CxxQt QUIET)
if(NOT CxxQt_FOUND)
    include(FetchContent)
    FetchContent_Declare(
        CxxQt
        GIT_REPOSITORY https://github.com/kdab/cxx-qt-cmake.git
        GIT_TAG main
    )

    FetchContent_MakeAvailable(CxxQt)
endif()

# CXX-Qt (using Corrosion) creates a CMake target with the same name as the crate.
cxx_qt_import_crate(MANIFEST_PATH rust/Cargo.toml CRATES cxx_qt_test)
target_link_libraries(cxx_qt_test INTERFACE Qt::Core Qt::Gui Qt::Qml Qt::QuickControls2)

cxx_qt_import_qml_module(cxx_qt_test_qml_module
    URI "com.nb.cxx_qt.test"
    SOURCE_CRATE cxx_qt_test)

# Define the executable with the C++ source
add_executable(${PROJECT_NAME} cpp/main.cpp)

# Link to the qml module, which in turn links to the Rust qml_minimal library
target_link_libraries(${PROJECT_NAME} PRIVATE cxx_qt_test_qml_module)

# If we are using a statically linked Qt then we need to import any qml plugins
qt_import_qml_plugins(${PROJECT_NAME})