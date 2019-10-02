### 简介
这是一个中国象棋引擎，由纯Rust写成。
目的是为了学习Rust和实践极大极小值算法。

另外提供了一个UI
https://gitlab.com/shallop/chess_ui

---

### 版本计划

| 版本号 | 实现的基本功能 | 进度计划 |
|:---:|:---|:---:|
| V0.0.1 | 和客户端之间实现单线程Socket通信 | 100% |
| V0.0.5 | 棋盘基本数据结构定义 | 100% |
| V0.1.0 | 极大极小值判断 | 100% |
| V0.2.0 | 棋子移动规则 | 100% |
| V0.3.0 | 评价系统 | 100% |
| V0.4.0 | 选择随机化 | 0% |
| V0.5.0 | 死棋预判 | 0% |
| V0.6.0 | 导入棋谱 | 0% |
| V0.7.0 | 并发计算 | 0% |
| V0.8.0 | 评价系统优化 | 0% |
| V0.9.0 | 实现棋谱自我学习 | 0% |
| V1.0.0 | 部署 | 0% |

---

### 详细计划
- v0.3.0 已完成，可实现最基本的棋子计算和移动
- TODO v0.8.0 增加被将军的位置过滤
