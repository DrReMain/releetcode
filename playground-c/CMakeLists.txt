cmake_minimum_required(VERSION 3.10)
project(playground-c C)

set(CMAKE_C_STANDARD 11)

include_directories(include)

# DataStructure Library
add_library(datastructure src/datastructure/list_node.c)

# n206 Solution
add_library(n206 src/n206/solution.c)
target_link_libraries(n206 datastructure)

# Unit Tests
add_executable(test_n206 tests/n206/test_solution.c)
target_link_libraries(test_n206 n206 datastructure)

# Benchmarks
add_executable(bench_n206 benchmarks/n206/bench_solution.c)
target_link_libraries(bench_n206 n206 datastructure)

enable_testing()
add_test(NAME n206_test COMMAND test_n206)
