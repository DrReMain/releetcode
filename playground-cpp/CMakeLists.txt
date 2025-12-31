cmake_minimum_required(VERSION 3.14)
project(playground-cpp CXX)

set(CMAKE_CXX_STANDARD 17)

include(FetchContent)

# GTest
FetchContent_Declare(
  googletest
  URL https://github.com/google/googletest/archive/refs/heads/main.zip
)
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)

# Google Benchmark
FetchContent_Declare(
  benchmark
  URL https://github.com/google/benchmark/archive/refs/heads/main.zip
)
FetchContent_MakeAvailable(benchmark)

include_directories(include src)

# DataStructure Library
add_library(datastructure_cpp src/datastructure/list_node.cpp)

# Unit Tests
add_executable(test_n206_cpp tests/n206/test_solution.cpp)
target_link_libraries(test_n206_cpp GTest::gtest_main datastructure_cpp)

# Benchmarks
add_executable(bench_n206_cpp benchmarks/n206/bench_solution.cpp)
target_link_libraries(bench_n206_cpp benchmark::benchmark datastructure_cpp)

enable_testing()
add_test(NAME n206_test_cpp COMMAND test_n206_cpp)
