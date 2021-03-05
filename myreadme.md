# readme


C:\Users\chengwu2\.cargo

```shell
    目录: C:\Users\chengwu2\.cargo


Mode                LastWriteTime         Length Name
----                -------------         ------ ----
d-----        2020/12/9     17:10                bin
d-----        2020/12/8     18:08                registry
-a----        2020/12/8     17:32              0 .package-cache
-a----        2020/12/9     17:28            295 config


Mode                LastWriteTime         Length Name
----                -------------         ------ ----
-a----        2020/12/9     17:07        9800273 cargo-clippy.exe
-a----        2020/12/9     17:07        9800273 cargo-fmt.exe
-a----        2020/12/9     17:07        9800273 cargo-miri.exe
-a----        2020/12/9     17:07        9800273 cargo.exe
-a----        2020/12/9     17:07        9800273 clippy-driver.exe
-a----        2020/12/9     17:07        9800273 rls.exe
-a----        2020/12/9     17:07        9800273 rust-gdb.exe
-a----        2020/12/9     17:07        9800273 rust-lldb.exe
-a----        2020/12/9     17:07        9800273 rustc.exe
-a----        2020/12/9     17:07        9800273 rustdoc.exe
-a----        2020/12/9     17:07        9800273 rustfmt.exe
-a----        2020/12/9     17:07        9800273 rustup.exe

```

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

rustc下载的文件，能不放在C盘吗，磁盘空间不够
