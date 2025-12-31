# Playground C++

本项目是一个用于 LeetCode 练习的 C++ 单语言项目，支持现代化的项目管理、单元测试和性能测试。

## 项目结构

- `include/datastructure`: 头文件目录
- `src/datastructure`: 通用数据结构实现（如 ListNode）
- `src/n206`: 题目 #206 反转链表的实现
- `tests`: 单元测试目录 (使用 Google Test)
- `benchmarks`: 性能测试目录 (使用 Google Benchmark)

## 管理工具

项目使用 CMake 进行管理。

## 常用命令

### 编译项目
```bash
mkdir build && cd build
cmake ..
make
```

### 运行单元测试
```bash
cd build
ctest
# 或者直接运行
./test_n206_cpp
```

### 运行 Benchmark
```bash
cd build
./bench_n206_cpp
```

## 题目说明

### #206 反转链表
给你单链表的头节点 `head` ，请你反转链表，并返回反转后的链表。
