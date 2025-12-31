# Playground Kotlin

本项目是一个用于 LeetCode 练习的 Kotlin 单语言项目，支持现代化的项目管理、单元测试和性能测试。

## 项目结构

- `src/main/kotlin`: 源代码目录
  - `com.releetcode.datastructure`: 通用数据结构（如 ListNode）
  - `com.releetcode.n206`: 题目 #206 反转链表的实现
- `src/test/kotlin`: 单元测试目录
- `src/jmh/kotlin`: 性能测试 (Benchmark) 目录

## 管理工具

项目使用 Gradle (Kotlin DSL) 进行管理。

## 常用命令

### 运行单元测试
```bash
./gradlew test
```

### 运行 Benchmark
```bash
./gradlew jmh
```

### 构建项目
```bash
./gradlew build
```

## 题目说明

### #206 反转链表
给你单链表的头节点 `head` ，请你反转链表，并返回反转后的链表。
