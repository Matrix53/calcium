# Calcium

## 项目说明

本项目是将 miniSysY(C 语言的一个子集)编译成 LLVM IR 的一个编译器，使用 Rust 实现。采用手写 DFA 进行词法分析，采用递归子程序进行语法分析并同时进行语法制导翻译。

## 运行方法

- 使用`git clone https://github.com/Matrix53/calcium`将代码克隆到本地
- 使用`cargo build`命令构建项目
- 使用`cargo run input output`命令进行 miniSysY 的编译，`input`是输入文件路径，`output`是输出文件路径

**P.S.** 本地必须有 Rust 语言环境，才能进行项目的编译
