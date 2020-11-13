推荐在Linux下使用本框架 

## 1. 安装`rust`和`cargo` 
最简单方法是使用`rustup`脚本，获取当前稳定版本的 [Rust](https://www.rust-lang.org/):
在 Linux 系统上，这可以通过以下方式完成:

> 在国内可以先通过下面的环境变量后再执行安装 
```
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
export PATH="$HOME/.cargo/bin:$PATH"
```

```console
$ curl -sSf https://static.rust-lang.org/rustup.sh | sh
```


它将下载一个脚本，然后开始安装。如果一切顺利，您会看到:

```console
Rust is installed now. Great!
```

## 2.安装llvm 
+ ubuntu  
    1.安装基础环境  (if you are root, just remove `sudo`)
    ```
    sudo apt-get install clang-format clang-tidy clang-tools clang clangd libc++-dev libc++1 libc++abi-dev libc++abi1 libclang-dev libclang1 liblldb-dev libllvm-ocaml-dev libomp-dev libomp5 lld lldb llvm-dev llvm-runtime llvm python-clang
    ```
    
    2.通过官方脚本安装llvm
    
    ```
    sudo bash -c "$(wget -O - https://apt.llvm.org/llvm.sh)"
    ```
    
    3.创建符号连接 
    ```
    ln -s llvm-config-11 /usr/bin/llvm-config
    ```
  
+ centos   
    1. 安装基础环境 (if you are root, just remove `sudo`)
    ```
    sudo yum install centos-release-scl
    sudo yum-config-manager --enable rhel-server-rhscl-7-rpms
    sudo yum install llvm-toolset-7.0
    sudo yum install llvm-toolset-7-llvm-devel
    scl enable llvm-toolset-7.0 bash
    ```
    


## 3.基础环境设置 
根据你当前目录的位置
```
# 将下面的{flashfunk}替换为你flashfunk源代码的位置 
export LD_LIBRARY_PATH="{flashfunk}/sdk_sources/ctp/linux/:$LD_LIBRARY_PATH"
```
或者一劳永逸
```
同样替换目录位置
echo export LD_LIBRARY_PATH="{flashfunk}/sdk_sources/ctp/linux/:$LD_LIBRARY_PATH" > ~/.bashrc
```

## 4.编译 
进入源代码目录下
```
cargo build 
```
> 如果更新crate.io速度太慢 建议使用清华源 
> 请看 [这里](https://mirrors.tuna.tsinghua.edu.cn/help/crates.io-index.git/)


这将生成对应的链接，注意仅仅是第一次需要生成

运行示例
```
cargo run --example call --release 
```

### for developer

如果你正在调试接口编译，请认真阅读`build.rs`


> 感谢`duckduckquant`对本文档的贡献 Thanks 
