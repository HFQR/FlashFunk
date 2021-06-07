#include "../../../sdk_sources/ctp/include/ThostFtdcMdApi.h
#include "../../../sdk_sources/ctp/include/ThostFtdcTraderApi.h
#include "../../../sdk_sources/ctp/include/ThostFtdcUserApiDataType.h
#include "../../../sdk_sources/ctp/include/ThostFtdcUserApiStruct.h

extern "C" void CThostFtdcMdApi_CreateFtdcMdApi(CThostFtdcMdApi * self , const char * pszFlowPath,const bool bIsUsingUdp,const bool bIsMulticast) {
    auto instance = static_cast<CThostFtdcMdApi*>(self);
    instance->CreateFtdcMdApi(pszFlowPath, bIsUsingUdp, bIsMulticast);
}
        
extern "C" void CThostFtdcMdApi_GetApiVersion(CThostFtdcMdApi * self ) {
    auto instance = static_cast<CThostFtdcMdApi*>(self);
    instance->GetApiVersion();
}
        
extern "C" void CThostFtdcMdApi_Release(CThostFtdcMdApi * self ) {
    auto instance = static_cast<CThostFtdcMdApi*>(self);
    instance->Release();
}
        
extern "C" void CThostFtdcMdApi_Init(CThostFtdcMdApi * self ) {
    auto instance = static_cast<CThostFtdcMdApi*>(self);
    instance->Init();
}
        
extern "C" void CThostFtdcMdApi_Join(CThostFtdcMdApi * self ) {
    auto instance = static_cast<CThostFtdcMdApi*>(self);
    instance->Join();
}
        
extern "C" void CThostFtdcMdApi_GetTradingDay(CThostFtdcMdApi * self ) {
    auto instance = static_cast<CThostFtdcMdApi*>(self);
    instance->GetTradingDay();
}
        
extern "C" void CThostFtdcMdApi_RegisterFront(CThostFtdcMdApi * self , char * pszFrontAddress) {
    auto instance = static_cast<CThostFtdcMdApi*>(self);
    instance->RegisterFront(pszFrontAddress);
}
        
extern "C" void CThostFtdcMdApi_RegisterNameServer(CThostFtdcMdApi * self , char * pszNsAddress) {
    auto instance = static_cast<CThostFtdcMdApi*>(self);
    instance->RegisterNameServer(pszNsAddress);
}
        
extern "C" void CThostFtdcMdApi_RegisterFensUserInfo(CThostFtdcMdApi * self , CThostFtdcFensUserInfoField * pFensUserInfo) {
    auto instance = static_cast<CThostFtdcMdApi*>(self);
    instance->RegisterFensUserInfo(pFensUserInfo);
}
        
extern "C" void CThostFtdcMdApi_RegisterSpi(CThostFtdcMdApi * self , CThostFtdcMdSpi * pSpi) {
    auto instance = static_cast<CThostFtdcMdApi*>(self);
    instance->RegisterSpi(pSpi);
}
        
extern "C" void CThostFtdcMdApi_SubscribeMarketData(CThostFtdcMdApi * self , char * ppInstrumentID,int nCount) {
    auto instance = static_cast<CThostFtdcMdApi*>(self);
    instance->SubscribeMarketData(ppInstrumentID, nCount);
}
        
extern "C" void CThostFtdcMdApi_UnSubscribeMarketData(CThostFtdcMdApi * self , char * ppInstrumentID,int nCount) {
    auto instance = static_cast<CThostFtdcMdApi*>(self);
    instance->UnSubscribeMarketData(ppInstrumentID, nCount);
}
        
extern "C" void CThostFtdcMdApi_SubscribeForQuoteRsp(CThostFtdcMdApi * self , char * ppInstrumentID,int nCount) {
    auto instance = static_cast<CThostFtdcMdApi*>(self);
    instance->SubscribeForQuoteRsp(ppInstrumentID, nCount);
}
        
extern "C" void CThostFtdcMdApi_UnSubscribeForQuoteRsp(CThostFtdcMdApi * self , char * ppInstrumentID,int nCount) {
    auto instance = static_cast<CThostFtdcMdApi*>(self);
    instance->UnSubscribeForQuoteRsp(ppInstrumentID, nCount);
}
        
extern "C" void CThostFtdcMdApi_ReqUserLogin(CThostFtdcMdApi * self , CThostFtdcReqUserLoginField * pReqUserLoginField,int nRequestID) {
    auto instance = static_cast<CThostFtdcMdApi*>(self);
    instance->ReqUserLogin(pReqUserLoginField, nRequestID);
}
        
extern "C" void CThostFtdcMdApi_ReqUserLogout(CThostFtdcMdApi * self , CThostFtdcUserLogoutField * pUserLogout,int nRequestID) {
    auto instance = static_cast<CThostFtdcMdApi*>(self);
    instance->ReqUserLogout(pUserLogout, nRequestID);
}
        
extern "C" void CThostFtdcMdApi_CThostFtdcMdApi(CThostFtdcMdApi * self ) {
    auto instance = static_cast<CThostFtdcMdApi*>(self);
    instance->CThostFtdcMdApi();
}
        extern "C" void RustCtpActionOnFrontDisconnected(void *rust_call_object int nReason)extern "C" void RustCtpActionOnHeartBeatWarning(void *rust_call_object int nTimeLapse)extern "C" void RustCtpActionOnRspUserLogin(void *rust_call_object CThostFtdcRspUserLoginField * pRspUserLogin,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspUserLogout(void *rust_call_object CThostFtdcUserLogoutField * pUserLogout,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspError(void *rust_call_object CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspSubMarketData(void *rust_call_object CThostFtdcSpecificInstrumentField * pSpecificInstrument,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspUnSubMarketData(void *rust_call_object CThostFtdcSpecificInstrumentField * pSpecificInstrument,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspSubForQuoteRsp(void *rust_call_object CThostFtdcSpecificInstrumentField * pSpecificInstrument,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspUnSubForQuoteRsp(void *rust_call_object CThostFtdcSpecificInstrumentField * pSpecificInstrument,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRtnDepthMarketData(void *rust_call_object CThostFtdcDepthMarketDataField * pDepthMarketData)extern "C" void RustCtpActionOnRtnForQuoteRsp(void *rust_call_object CThostFtdcForQuoteRspField * pForQuoteRsp)
class CtpMdSpi: CThostFtdcMdSpi {
public:
    void *rust_call_object;
    
    CtpMdSpi(void *rust_call_object) : rust_call_object(rust_call_object) {};
                
            
     void OnFrontDisconnected(int nReason){
          RustCtpActionOnFrontDisconnected(this->rust_call_object, nReason);            
     }
            
     void OnHeartBeatWarning(int nTimeLapse){
          RustCtpActionOnHeartBeatWarning(this->rust_call_object, nTimeLapse);            
     }
            
     void OnRspUserLogin(CThostFtdcRspUserLoginField * pRspUserLogin,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspUserLogin(this->rust_call_object, pRspUserLogin, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspUserLogout(CThostFtdcUserLogoutField * pUserLogout,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspUserLogout(this->rust_call_object, pUserLogout, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspError(CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspError(this->rust_call_object, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspSubMarketData(CThostFtdcSpecificInstrumentField * pSpecificInstrument,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspSubMarketData(this->rust_call_object, pSpecificInstrument, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspUnSubMarketData(CThostFtdcSpecificInstrumentField * pSpecificInstrument,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspUnSubMarketData(this->rust_call_object, pSpecificInstrument, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspSubForQuoteRsp(CThostFtdcSpecificInstrumentField * pSpecificInstrument,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspSubForQuoteRsp(this->rust_call_object, pSpecificInstrument, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspUnSubForQuoteRsp(CThostFtdcSpecificInstrumentField * pSpecificInstrument,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspUnSubForQuoteRsp(this->rust_call_object, pSpecificInstrument, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRtnDepthMarketData(CThostFtdcDepthMarketDataField * pDepthMarketData){
          RustCtpActionOnRtnDepthMarketData(this->rust_call_object, pDepthMarketData);            
     }
            
     void OnRtnForQuoteRsp(CThostFtdcForQuoteRspField * pForQuoteRsp){
          RustCtpActionOnRtnForQuoteRsp(this->rust_call_object, pForQuoteRsp);            
     }
            
protected:
    ~CtpMdSpi(){}
}
