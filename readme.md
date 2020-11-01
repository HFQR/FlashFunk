## flashfunk 

> High Frequency Trading Framework
 

## 前置声明

This project are contributed by  `somewheve` and `fakeshadow` and `qzm`

## Help
Here are some works unfinished

- 回測計算
I provide a  small core Account. we need to  build it on MockTdApi

- 測試完成
we need to test most operation

- log get 
將底層的錯誤日誌分發到上層處理. 或者單獨使用高性能的log庫來寫入

### Interface compile 
本项目最终采用`rust-indgen` 作为`c++`接口封装



### 一些必要的工作


### Environment setup
In order to run `call` example, for example in linux 

you should export absolute path of `/home/somewheve/Documents/ctpbee-rs/sdk_sources/ctp/linux/`
likes, do not work on windows =_=

for example In Linux 
```
export LD_LIBRARY_PATH=/home/somewheve/Documents/ctpbee-rs/sdk_sources/ctp/linux/
```
`rusct-link-search-dylib` do not work , I don't know why.

also works on windows.

### Necessary tools

- `LLVM` are need in windows. please search how to install and compile it on windows. 

- `gcc`

### At last 
我和我的合作伙伴都認爲架構已經沒有更好的改進方式了  現在針對的主要是字符串解析耗時間
運行example下面的call即可 