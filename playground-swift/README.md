# Playground Swift

本项目是一个用于 LeetCode 练习的 Swift 单语言项目，支持现代化的项目管理、单元测试和性能测试。

## 项目结构

- `Sources/DataStructure`: 通用数据结构（如 ListNode）
- `Sources/n206`: 题目 #206 反转链表的实现
- `Sources/Benchmark`: 性能测试入口
- `Tests`: 单元测试目录

## 管理工具

项目使用 Swift Package Manager (SPM) 进行管理。

## 常用命令

### 编译项目
```bash
swift build
```

### 运行单元测试
```bash
swift test
```

### 运行 Benchmark
```bash
swift run Benchmark
```

## 题目说明

### #206 反转链表
给你单链表的头节点 `head` ，请你反转链表，并返回反转后的链表。
