//
// Created by somewheve' on 2020/8/22.
//
#include "re.hpp"

extern "C" void CThostFtdcMdApi_Init(CThostFtdcMdApi *self) {
    auto s = static_cast<CThostFtdcMdApi *>(self);
    return s->Init();
}

extern "C" const char *CThostFtdcMdApi_GetTradingDay(CThostFtdcMdApi *self) {
    auto s = static_cast<CThostFtdcMdApi *>(self);
    return s->GetTradingDay();
}

extern "C" void CThostFtdcMdApi_RegisterFront(CThostFtdcMdApi *self, char *pszFrontAddress) {
    auto s = static_cast<CThostFtdcMdApi *>(self);
    s->RegisterFront(pszFrontAddress);
}

extern "C" void CThostFtdcMdApi_RegisterSpi(CThostFtdcMdApi *self, CThostFtdcMdSpi * spi) {
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