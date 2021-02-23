# readme


C:\Users\chengwu2\.cargo
config

[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
# 指定镜像
replace-with = 'ustc'

# 清华大学
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"

```
C:\Users\chengwu2\CLionProjects\OpenZKP>C:\Users\chengwu2\.cargo\bin\cargo.exe test
    Updating `git://mirrors.ustc.edu.cn/crates.io-index` index
       Fetch [======================>                                ]  43.30%

```

https://github.com/rust-lang/rust/issues/12859

