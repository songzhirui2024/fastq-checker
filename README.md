# Fastq-Checker

Fastq-Checker 是一个高效的工具，用于快速分析 FASTQ 格式的测序数据文件。它可以确定文件是原始数据（raw）还是经过处理的数据（clean），并提供基本的统计信息。

## 特性

- 快速处理大型 FASTQ.gz 文件
- 多线程支持，提高处理速度
- 提供序列读数、总碱基数等基本统计信息
- 可选的提前退出功能，在检测到 clean 数据时立即停止处理
- 命令行界面，易于集成到分析流程中

## 安装

确保您的系统中安装了 Rust 编程语言。然后，您可以通过以下命令克隆并构建项目：

```bash
git clone https://github.com/songzhirui2024/fastq-checker.git
cd fastq-checker
cargo build --release
```

## 使用方法

```bash
./fastq-checker -f <FASTQ_FILE> [-t <THREADS>] [-e <EARLY_EXIT>]
```

参数说明：
- `-f, --fastqgz <FILE>`: FASTQ.gz 文件的路径（必需）
- `-t, --threads <THREADS>`: 使用的线程数（默认：4）
- `-e, --early-exit <EARLY_EXIT>`: 检测到 clean 数据时是否提前退出（默认：true）

示例：
```bash
./fastq-checker -f sample.fastq.gz -t 8 -e false
```

## 输出

程序将输出以下信息：
- 文件名
- 读取的 reads 数量
- 总数据量（碱基对）
- 数据状态（raw 或 clean）

## 贡献

欢迎提交问题报告和拉取请求。对于重大更改，请先开启一个问题讨论您想要改变的内容。

## 许可证

本项目采用 MIT 许可证 - 详情请见 [LICENSE](LICENSE) 文件
