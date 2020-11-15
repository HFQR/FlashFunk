//
// Created by somewheve' on 2020/8/22.
//
#include "ctp_md.hpp"

#include "ctp_td.hpp"

extern "C" void CThostFtdcMdApi_Init(CThostFtdcMdApi *self) {
    return self->Init();
}

extern "C" const char *CThostFtdcMdApi_GetTradingDay(CThostFtdcMdApi *self) {
    auto s = static_cast<CThostFtdcMdApi *>(self);
    return s->GetTradingDay();
}

extern "C" void CThostFtdcMdApi_RegisterFront(CThostFtdcMdApi *self, char *pszFrontAddress) {
    auto s = static_cast<CThostFtdcMdApi *>(self);
    s->RegisterFront(pszFrontAddress);
}

extern "C" void CThostFtdcMdApi_RegisterSpi(CThostFtdcMdApi *self, CThostFtdcMdSpi *spi) {
    auto s = static_cast<CThostFtdcMdApi *>(self);
    s->RegisterSpi(spi);
}

extern "C" int CThostFtdcMdApi_SubscribeMarketData(CThostFtdcMdApi *self, char *ppInstrumentID[], int nCount) {
    auto s = static_cast<CThostFtdcMdApi *>(self);
    return s->SubscribeMarketData(ppInstrumentID, nCount);
}
extern "C" int CThostFtdcMdApi_UnSubscribeMarketData(CThostFtdcMdApi *self, char *ppInstrumentID[], int nCount) {
    auto s = static_cast<CThostFtdcMdApi *>(self);
    return s->UnSubscribeMarketData(ppInstrumentID, nCount);
}

extern "C" int
CThostFtdcMdApi_ReqUserLogin(CThostFtdcMdApi *self, CThostFtdcReqUserLoginField *pReqUserLoginField, int nRequestID) {
    auto s = static_cast<CThostFtdcMdApi *>(self);
    return s->ReqUserLogin(pReqUserLoginField, nRequestID);

}
extern "C" int CThostFtdcMdApi_ReqUserLogout(CThostFtdcMdApi *self,
                                             CThostFtdcUserLogoutField *pUserLogout, int nRequestID) {
    auto s = static_cast<CThostFtdcMdApi *>(self);
    return s->ReqUserLogout(pUserLogout, nRequestID);
}

extern "C" void CThostFtdcMdApi_RegisterNameServer(CThostFtdcMdApi *self, char *pszNsAddress) {
    auto s = static_cast<CThostFtdcMdApi *>(self);
    s->RegisterNameServer(pszNsAddress);
}
extern "C" void
CThostFtdcMdApi_RegisterFensUserInfo(CThostFtdcMdApi *self, CThostFtdcFensUserInfoField *pFensUserInfo) {
    auto s = static_cast<CThostFtdcMdApi *>(self);
    s->RegisterFensUserInfo(pFensUserInfo);
}


/// 行情API回调处理函数
QuoteSpi::QuoteSpi(void *rust_object) : rust_object(rust_object) {}

void QuoteSpi::OnFrontConnected() {
    QuoteSpi_OnFrontConnected(this->rust_object);
}

void QuoteSpi::OnFrontDisconnected(int nReason) {
    return QuoteSpi_OnFrontDisconnected(this->rust_object, nReason);
}

void QuoteSpi::OnHeartBeatWarning(int nTimeLapse) {
    QuoteSpi_OnHeartBeatWarning(this->rust_object, nTimeLapse);
}

void
QuoteSpi::OnRspUserLogin(CThostFtdcRspUserLoginField *pRspUserLogin, CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                         bool bIsLast) {
    QuoteSpi_OnRspUserLogin(this->rust_object, pRspUserLogin, pRspInfo, nRequestID, bIsLast);
}

void QuoteSpi::OnRspUserLogout(CThostFtdcUserLogoutField *pUserLogout, CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                               bool bIsLast) {
    QuoteSpi_OnRspUserLogout(this->rust_object, pUserLogout, pRspInfo, nRequestID, bIsLast);
}

//void QuoteSpi::OnRspQryMulticastInstrument(CThostFtdcMulticastInstrumentField *pMulticastInstrument,
//                                           CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast) {
//    QuoteSpi_OnRspQryMulticastInstrument(this->rust_object, pMulticastInstrument, pRspInfo, nRequestID, bIsLast);
//}

void QuoteSpi::OnRspError(CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast) {
    QuoteSpi_OnRspError(this->rust_object, pRspInfo, nRequestID, bIsLast);
}

void QuoteSpi::OnRspSubMarketData(CThostFtdcSpecificInstrumentField *pSpecificInstrument,
                                  CThostFtdcRspInfoField *pRspInfo,
                                  int nRequestID, bool bIsLast) {
    QuoteSpi_OnRspSubMarketData(this->rust_object, pSpecificInstrument, pRspInfo, nRequestID, bIsLast);
}

void QuoteSpi::OnRspUnSubMarketData(CThostFtdcSpecificInstrumentField *pSpecificInstrument,
                                    CThostFtdcRspInfoField *pRspInfo,
                                    int nRequestID, bool bIsLast) {
    QuoteSpi_OnRspUnSubMarketData(this->rust_object, pSpecificInstrument, pRspInfo, nRequestID, bIsLast);
}

void QuoteSpi::OnRspSubForQuoteRsp(CThostFtdcSpecificInstrumentField *pSpecificInstrument,
                                   CThostFtdcRspInfoField *pRspInfo,
                                   int nRequestID, bool bIsLast) {

}

void QuoteSpi::OnRspUnSubForQuoteRsp(CThostFtdcSpecificInstrumentField *pSpecificInstrument,
                                     CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast) {
    QuoteSpi_OnRspUnSubForQuoteRsp(this->rust_object, pSpecificInstrument, pRspInfo, nRequestID, bIsLast);
}

void QuoteSpi::OnRtnDepthMarketData(CThostFtdcDepthMarketDataField *pDepthMarketData) {
    QuoteSpi_OnRtnDepthMarketData(this->rust_object, pDepthMarketData);
}

void QuoteSpi::OnRtnForQuoteRsp(CThostFtdcForQuoteRspField *pForQuoteRsp) {
    QuoteSpi_OnRtnForQuoteRsp(this->rust_object, pForQuoteRsp);
}

QuoteSpi::~QuoteSpi() {
    QuoteSpi_Rust_Destructor(this->rust_object);
}

