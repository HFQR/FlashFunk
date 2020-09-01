
//#ifndef XTP_RS_BRIDGE_H_
//#define XTP_RS_BRIDGE_H_
//
#include "../../../sdk_sources/ctp/include/ThostFtdcMdApi.h"
#include "../../../sdk_sources/ctp/include/ThostFtdcUserApiStruct.h"
#include "../../../sdk_sources/ctp/include/ThostFtdcUserApiDataType.h"
#include "../../../sdk_sources/ctp/include/ThostFtdcTraderApi.h"
//
//
//extern "C" const char *CThostFtdcMdApi_GetTradingDay(CThostFtdcMdApi *self) {
//    return self->GetApiVersion();
//};
//
//extern "C" void CThostFtdcMdApi_Init(CThostFtdcMdApi *self);


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