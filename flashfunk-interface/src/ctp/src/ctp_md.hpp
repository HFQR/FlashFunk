#include "../../../sdk_sources/ctp/include/ThostFtdcUserApiStruct.h"
#include "../../../sdk_sources/ctp/include/ThostFtdcUserApiDataType.h"
#include "../../../sdk_sources/ctp/include/ThostFtdcMdApi.h"
#include "../../../sdk_sources/ctp/include/ThostFtdcTraderApi.h"


//MdApi {
//    MdSpi
//}

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

// 我们自己写的rust的对象给他载入进去
class QuoteSpi : CThostFtdcMdSpi {
public:
    void *rust_object;

    QuoteSpi(void *rust_object);

    void OnFrontConnected();

    void OnFrontDisconnected(int nReason);

    void OnHeartBeatWarning(int nTimeLapse);

    void OnRspUserLogin(CThostFtdcRspUserLoginField *pRspUserLogin, CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                        bool bIsLast);

    void OnRspUserLogout(CThostFtdcUserLogoutField *pUserLogout, CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                         bool bIsLast);
//
//    void OnRspQryMulticastInstrument(CThostFtdcMulticastInstrumentField *pMulticastInstrument,
//                                     CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    void OnRspError(CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);

    void OnRspSubMarketData(CThostFtdcSpecificInstrumentField *pSpecificInstrument, CThostFtdcRspInfoField *pRspInfo,
                            int nRequestID, bool bIsLast);

    void OnRspUnSubMarketData(CThostFtdcSpecificInstrumentField *pSpecificInstrument, CThostFtdcRspInfoField *pRspInfo,
                              int nRequestID, bool bIsLast);

    void OnRspSubForQuoteRsp(CThostFtdcSpecificInstrumentField *pSpecificInstrument, CThostFtdcRspInfoField *pRspInfo,
                             int nRequestID, bool bIsLast);

    void OnRspUnSubForQuoteRsp(CThostFtdcSpecificInstrumentField *pSpecificInstrument, CThostFtdcRspInfoField *pRspInfo,
                               int nRequestID, bool bIsLast);

    void OnRtnDepthMarketData(CThostFtdcDepthMarketDataField *pDepthMarketData);

    void OnRtnForQuoteRsp(CThostFtdcForQuoteRspField *pForQuoteRsp);

    ~QuoteSpi();
};

extern "C" void QuoteSpi_Destructor(QuoteSpi *stub) { delete stub; };
extern "C" void QuoteSpi_OnFrontConnected(void *rust_object);
extern "C" void QuoteSpi_OnFrontDisconnected(void *rust_object, int reason);
extern "C" void QuoteSpi_OnHeartBeatWarning(void *rust_object, int nTimeLapse);
extern "C" void QuoteSpi_OnRspUserLogin(void *rust_object, CThostFtdcRspUserLoginField *pRspUserLogin,
                                        CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                                        bool bIsLast);
extern "C" void QuoteSpi_OnRspUserLogout(void *rust_object, CThostFtdcUserLogoutField *pUserLogout,
                                         CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                                         bool bIsLast);
//extern "C" void QuoteSpi_OnRspQryMulticastInstrument(void *rust_object,
//                                                     CThostFtdcMulticastInstrumentField *pMulticastInstrument,
//                                                     CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
extern "C" void QuoteSpi_OnRspError(void *rust_object, CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
extern "C" void QuoteSpi_OnRspSubMarketData(void *rust_object, CThostFtdcSpecificInstrumentField *pSpecificInstrument,
                                            CThostFtdcRspInfoField *pRspInfo,
                                            int nRequestID, bool bIsLast);
extern "C" void QuoteSpi_OnRspUnSubMarketData(void *rust_object, CThostFtdcSpecificInstrumentField *pSpecificInstrument,
                                              CThostFtdcRspInfoField *pRspInfo,
                                              int nRequestID, bool bIsLast);
extern "C" void QuoteSpi_OnRspSubForQuoteRsp(void *rust_object, CThostFtdcSpecificInstrumentField *pSpecificInstrument,
                                             CThostFtdcRspInfoField *pRspInfo,
                                             int nRequestID, bool bIsLast);
extern "C" void QuoteSpi_OnRspUnSubForQuoteRsp(void *rust_object,
                                               CThostFtdcSpecificInstrumentField *pSpecificInstrument,
                                               CThostFtdcRspInfoField *pRspInfo,
                                               int nRequestID, bool bIsLast);
extern "C" void QuoteSpi_OnRtnDepthMarketData(void *rust_object, CThostFtdcDepthMarketDataField *pDepthMarketData);
extern "C" void QuoteSpi_OnRtnForQuoteRsp(void *rust_object, CThostFtdcForQuoteRspField *pForQuoteRsp);

extern "C" void QuoteSpi_Rust_Destructor(void *rust_object);

