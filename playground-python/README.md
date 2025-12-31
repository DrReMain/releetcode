# ReLeetCode Python Playground

```text
playground-python/
├── src/
│   └── releetcode/          # 主包名
│       ├── data_structures/ # 数据结构定义
│       ├── problems/        # 实现
│       │   └── n1_two_sum/
│       └── utils/           # 工具函数
├── tests/                   # 单元测试 (使用 pytest)
├── benchmarks/              # 基准测试 (使用 pytest-benchmark)
├── pyproject.toml           # 项目核心配置与依赖管理 (PEP 621)
└── README.md
```

## 安装项目依赖

在项目根目录下执行，这将以“可编辑模式”安装项目，方便开发和调试：

```bash
pip install -e ".[dev]"
```

## 运行单元测试

```bash
# 运行所有测试
pytest

# 运行特定题目测试
pytest tests/problems/test_n1.py
```

## 运行基准测试 (Performance)

```bash
pytest benchmarks/
```

## 代码风格检查与格式化

项目预配置了高性能的 `ruff` 工具：

```bash
# 静态检查
ruff check .

# 自动格式化代码
ruff format .
```
