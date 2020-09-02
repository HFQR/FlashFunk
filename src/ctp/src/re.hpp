
//#ifndef XTP_RS_BRIDGE_H_
//#define XTP_RS_BRIDGE_H_
//
#include "../../../sdk_sources/ctp/include/ThostFtdcMdApi.h"
#include "../../../sdk_sources/ctp/include/ThostFtdcUserApiStruct.h"
#include "../../../sdk_sources/ctp/include/ThostFtdcUserApiDataType.h"
#include "../../../sdk_sources/ctp/include/ThostFtdcTraderApi.h"


extern "C" void CThostFtdcMdApi_Init(CThostFtdcMdApi *self);
extern "C" const char *CThostFtdcMdApi_GetTradingDay(CThostFtdcMdApi *self);
extern "C" void CThostFtdcMdApi_RegisterFront(CThostFtdcMdApi *self, char *pszFrontAddress);
extern "C" void CThostFtdcMdApi_RegisterSpi(CThostFtdcMdApi *self, CThostFtdcMdSpi *spi);
extern "C" int CThostFtdcMdApi_SubscribeMarketData(CThostFtdcMdApi *self, char *ppInstrumentID[], int nCount);
extern "C" int CThostFtdcMdApi_UnSubscribeMarketData(CThostFtdcMdApi *self, char *ppInstrumentID[], int nCount);

extern "C" int
CThostFtdcMdApi_ReqUserLogin(CThostFtdcMdApi *self, CThostFtdcReqUserLoginField *pReqUserLoginField, int nRequestID);
extern "C" int
CThostFtdcMdApi_ReqUserLogout(CThostFtdcMdApi *self, CThostFtdcUserLogoutField *pUserLogout, int nRequestID);
extern "C" void CThostFtdcMdApi_RegisterNameServer(CThostFtdcMdApi *self, char *pszNsAddress);
extern "C" void CThostFtdcMdApi_RegisterFensUserInfo(CThostFtdcMdApi *self, CThostFtdcFensUserInfoField *pFensUserInfo);

//// 行情API回调

//class QuoteSpi : CThostFtdcMdSpi {
//public:
//    void *rust_object;
//
//    void OnFrontConnected() {};
//
//    ///当客户端与交易后台通信连接断开时，该方法被调用。当发生这个情况后，API会自动重新连接，客户端可不做处理。
//    ///@param nReason 错误原因
//    ///        0x1001 网络读失败
//    ///        0x1002 网络写失败
//    ///        0x2001 接收心跳超时
//    ///        0x2002 发送心跳失败
//    ///        0x2003 收到错误报文
//    void OnFrontDisconnected(int nReason) {
//        this->rust_object.
//    };
//
//    ///心跳超时警告。当长时间未收到报文时，该方法被调用。
//    ///@param nTimeLapse 距离上次接收报文的时间
//    void OnHeartBeatWarning(int nTimeLapse) {};
//
//
//    ///登录请求响应
//    void OnRspUserLogin(CThostFtdcRspUserLoginField *pRspUserLogin, CThostFtdcRspInfoField *pRspInfo, int nRequestID,
//                        bool bIsLast) {};
//
//    ///登出请求响应
//    void OnRspUserLogout(CThostFtdcUserLogoutField *pUserLogout, CThostFtdcRspInfoField *pRspInfo, int nRequestID,
//                         bool bIsLast);
//
//    ///请求查询组播合约响应
//    void OnRspQryMulticastInstrument(CThostFtdcMulticastInstrumentField *pMulticastInstrument,
//                                     CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast) {};
//
//    ///错误应答
//    void OnRspError(CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast) {};
//
//    ///订阅行情应答
//    void OnRspSubMarketData(CThostFtdcSpecificInstrumentField *pSpecificInstrument, CThostFtdcRspInfoField *pRspInfo,
//                            int nRequestID, bool bIsLast) {};
//
//    ///取消订阅行情应答
//    void OnRspUnSubMarketData(CThostFtdcSpecificInstrumentField *pSpecificInstrument, CThostFtdcRspInfoField *pRspInfo,
//                              int nRequestID, bool bIsLast) {};
//
//    ///订阅询价应答
//    void OnRspSubForQuoteRsp(CThostFtdcSpecificInstrumentField *pSpecificInstrument, CThostFtdcRspInfoField *pRspInfo,
//                             int nRequestID, bool bIsLast) {};
//
//    ///取消订阅询价应答
//    void OnRspUnSubForQuoteRsp(CThostFtdcSpecificInstrumentField *pSpecificInstrument, CThostFtdcRspInfoField *pRspInfo,
//                               int nRequestID, bool bIsLast) {};
//
//    ///深度行情通知
//    void OnRtnDepthMarketData(CThostFtdcDepthMarketDataField *pDepthMarketData) {};
//
//    ///询价通知
//    void OnRtnForQuoteRsp(CThostFtdcForQuoteRspField *pForQuoteRsp) {};
//
//};


// 交易API