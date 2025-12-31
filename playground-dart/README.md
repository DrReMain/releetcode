# Playground Dart

本项目是一个用于 LeetCode 练习的 Dart 单语言项目，支持现代化的项目管理、单元测试和性能测试。

## 项目结构

- `lib/datastructure`: 通用数据结构（如 ListNode）
- `lib/n206`: 题目 #206 反转链表的实现
- `test`: 单元测试目录
- `benchmark`: 性能测试 (Benchmark) 目录

## 管理工具

项目使用 Dart SDK 进行管理。

## 常用命令

### 安装依赖
```bash
dart pub get
```

### 运行单元测试
```bash
dart test
```

### 运行 Benchmark
```bash
dart run benchmark/n206/reverse_list_benchmark.dart
```

## 题目说明

### #206 反转链表
给你单链表的头节点 `head` ，请你反转链表，并返回反转后的链表。
