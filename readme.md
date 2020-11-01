## flashfunk 

> High Frequency Trading Framework


## Why Rust
有幸和`yutiansut`一起工作， 接觸到了rust，後來發現Rust很適合金融， 因爲強大的編譯器能夠幫你解決大部分開發中遇到的問題，
只要編譯能通過，邏輯閉環， 基本都能夠穩定運行， 而不是像c++那樣，在高性能的服務領域需要非常優秀的c++技巧和細緻入微的耐心才能夠寫出高可用的系統。

在藉助了`rust-bindgen`寫了這個項目 ，同時也感謝`xtp-rs`的作者提供的包，藉助了他的package我才入門rust的c++封裝技巧的。
期望後面大家一起貢獻這個項目， 從回測到實盤，都期望你的參與 ~  一起構建rust裏面的高性能低延時領域。

還有這個項目是部分開源的，僅僅想給做高頻的你們，你們不要拿去換資源哦。 否則發現我會把這個項目徹底關閉~~  

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
export LD_LIBRARY_PATH="/home/somewheve/Documents/ctpbee-rs/sdk_sources/ctp/linux/:$LD_LIBRARY_PATH"
```
`rusct-link-search-dylib` do not work , I don't know why.

also works on windows.

### Necessary tools

- `LLVM` are need in windows. please search how to install and compile it on windows. 

- `gcc`

### At last 
我和我的合作伙伴都認爲架構已經沒有更好的改進方式了  現在針對的主要是字符串解析耗時間
運行example下面的call即可 