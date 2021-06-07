#include "../../../sdk_sources/ctp/include/ThostFtdcMdApi.h
#include "../../../sdk_sources/ctp/include/ThostFtdcTraderApi.h
#include "../../../sdk_sources/ctp/include/ThostFtdcUserApiDataType.h
#include "../../../sdk_sources/ctp/include/ThostFtdcUserApiStruct.h

extern "C" void CThostFtdcTraderApi_CreateFtdcTraderApi(CThostFtdcTraderApi * self , const char * pszFlowPath) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->CreateFtdcTraderApi(pszFlowPath);
}
        
extern "C" void CThostFtdcTraderApi_GetApiVersion(CThostFtdcTraderApi * self ) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->GetApiVersion();
}
        
extern "C" void CThostFtdcTraderApi_Release(CThostFtdcTraderApi * self ) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->Release();
}
        
extern "C" void CThostFtdcTraderApi_Init(CThostFtdcTraderApi * self ) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->Init();
}
        
extern "C" void CThostFtdcTraderApi_Join(CThostFtdcTraderApi * self ) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->Join();
}
        
extern "C" void CThostFtdcTraderApi_GetTradingDay(CThostFtdcTraderApi * self ) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->GetTradingDay();
}
        
extern "C" void CThostFtdcTraderApi_RegisterFront(CThostFtdcTraderApi * self , char * pszFrontAddress) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->RegisterFront(pszFrontAddress);
}
        
extern "C" void CThostFtdcTraderApi_RegisterNameServer(CThostFtdcTraderApi * self , char * pszNsAddress) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->RegisterNameServer(pszNsAddress);
}
        
extern "C" void CThostFtdcTraderApi_RegisterFensUserInfo(CThostFtdcTraderApi * self , CThostFtdcFensUserInfoField * pFensUserInfo) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->RegisterFensUserInfo(pFensUserInfo);
}
        
extern "C" void CThostFtdcTraderApi_RegisterSpi(CThostFtdcTraderApi * self , CThostFtdcTraderSpi * pSpi) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->RegisterSpi(pSpi);
}
        
extern "C" void CThostFtdcTraderApi_SubscribePrivateTopic(CThostFtdcTraderApi * self , THOST_TE_RESUME_TYPE nResumeType) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->SubscribePrivateTopic(nResumeType);
}
        
extern "C" void CThostFtdcTraderApi_SubscribePublicTopic(CThostFtdcTraderApi * self , THOST_TE_RESUME_TYPE nResumeType) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->SubscribePublicTopic(nResumeType);
}
        
extern "C" void CThostFtdcTraderApi_ReqAuthenticate(CThostFtdcTraderApi * self , CThostFtdcReqAuthenticateField * pReqAuthenticateField,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqAuthenticate(pReqAuthenticateField, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_RegisterUserSystemInfo(CThostFtdcTraderApi * self , CThostFtdcUserSystemInfoField * pUserSystemInfo) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->RegisterUserSystemInfo(pUserSystemInfo);
}
        
extern "C" void CThostFtdcTraderApi_SubmitUserSystemInfo(CThostFtdcTraderApi * self , CThostFtdcUserSystemInfoField * pUserSystemInfo) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->SubmitUserSystemInfo(pUserSystemInfo);
}
        
extern "C" void CThostFtdcTraderApi_ReqUserLogin(CThostFtdcTraderApi * self , CThostFtdcReqUserLoginField * pReqUserLoginField,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqUserLogin(pReqUserLoginField, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqUserLogout(CThostFtdcTraderApi * self , CThostFtdcUserLogoutField * pUserLogout,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqUserLogout(pUserLogout, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqUserPasswordUpdate(CThostFtdcTraderApi * self , CThostFtdcUserPasswordUpdateField * pUserPasswordUpdate,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqUserPasswordUpdate(pUserPasswordUpdate, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqTradingAccountPasswordUpdate(CThostFtdcTraderApi * self , CThostFtdcTradingAccountPasswordUpdateField * pTradingAccountPasswordUpdate,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqTradingAccountPasswordUpdate(pTradingAccountPasswordUpdate, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqUserAuthMethod(CThostFtdcTraderApi * self , CThostFtdcReqUserAuthMethodField * pReqUserAuthMethod,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqUserAuthMethod(pReqUserAuthMethod, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqGenUserCaptcha(CThostFtdcTraderApi * self , CThostFtdcReqGenUserCaptchaField * pReqGenUserCaptcha,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqGenUserCaptcha(pReqGenUserCaptcha, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqGenUserText(CThostFtdcTraderApi * self , CThostFtdcReqGenUserTextField * pReqGenUserText,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqGenUserText(pReqGenUserText, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqUserLoginWithCaptcha(CThostFtdcTraderApi * self , CThostFtdcReqUserLoginWithCaptchaField * pReqUserLoginWithCaptcha,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqUserLoginWithCaptcha(pReqUserLoginWithCaptcha, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqUserLoginWithText(CThostFtdcTraderApi * self , CThostFtdcReqUserLoginWithTextField * pReqUserLoginWithText,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqUserLoginWithText(pReqUserLoginWithText, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqUserLoginWithOTP(CThostFtdcTraderApi * self , CThostFtdcReqUserLoginWithOTPField * pReqUserLoginWithOTP,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqUserLoginWithOTP(pReqUserLoginWithOTP, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqOrderInsert(CThostFtdcTraderApi * self , CThostFtdcInputOrderField * pInputOrder,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqOrderInsert(pInputOrder, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqParkedOrderInsert(CThostFtdcTraderApi * self , CThostFtdcParkedOrderField * pParkedOrder,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqParkedOrderInsert(pParkedOrder, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqParkedOrderAction(CThostFtdcTraderApi * self , CThostFtdcParkedOrderActionField * pParkedOrderAction,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqParkedOrderAction(pParkedOrderAction, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqOrderAction(CThostFtdcTraderApi * self , CThostFtdcInputOrderActionField * pInputOrderAction,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqOrderAction(pInputOrderAction, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQueryMaxOrderVolume(CThostFtdcTraderApi * self , CThostFtdcQueryMaxOrderVolumeField * pQueryMaxOrderVolume,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQueryMaxOrderVolume(pQueryMaxOrderVolume, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqSettlementInfoConfirm(CThostFtdcTraderApi * self , CThostFtdcSettlementInfoConfirmField * pSettlementInfoConfirm,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqSettlementInfoConfirm(pSettlementInfoConfirm, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqRemoveParkedOrder(CThostFtdcTraderApi * self , CThostFtdcRemoveParkedOrderField * pRemoveParkedOrder,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqRemoveParkedOrder(pRemoveParkedOrder, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqRemoveParkedOrderAction(CThostFtdcTraderApi * self , CThostFtdcRemoveParkedOrderActionField * pRemoveParkedOrderAction,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqRemoveParkedOrderAction(pRemoveParkedOrderAction, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqExecOrderInsert(CThostFtdcTraderApi * self , CThostFtdcInputExecOrderField * pInputExecOrder,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqExecOrderInsert(pInputExecOrder, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqExecOrderAction(CThostFtdcTraderApi * self , CThostFtdcInputExecOrderActionField * pInputExecOrderAction,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqExecOrderAction(pInputExecOrderAction, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqForQuoteInsert(CThostFtdcTraderApi * self , CThostFtdcInputForQuoteField * pInputForQuote,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqForQuoteInsert(pInputForQuote, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQuoteInsert(CThostFtdcTraderApi * self , CThostFtdcInputQuoteField * pInputQuote,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQuoteInsert(pInputQuote, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQuoteAction(CThostFtdcTraderApi * self , CThostFtdcInputQuoteActionField * pInputQuoteAction,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQuoteAction(pInputQuoteAction, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqBatchOrderAction(CThostFtdcTraderApi * self , CThostFtdcInputBatchOrderActionField * pInputBatchOrderAction,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqBatchOrderAction(pInputBatchOrderAction, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqOptionSelfCloseInsert(CThostFtdcTraderApi * self , CThostFtdcInputOptionSelfCloseField * pInputOptionSelfClose,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqOptionSelfCloseInsert(pInputOptionSelfClose, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqOptionSelfCloseAction(CThostFtdcTraderApi * self , CThostFtdcInputOptionSelfCloseActionField * pInputOptionSelfCloseAction,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqOptionSelfCloseAction(pInputOptionSelfCloseAction, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqCombActionInsert(CThostFtdcTraderApi * self , CThostFtdcInputCombActionField * pInputCombAction,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqCombActionInsert(pInputCombAction, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryOrder(CThostFtdcTraderApi * self , CThostFtdcQryOrderField * pQryOrder,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryOrder(pQryOrder, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryTrade(CThostFtdcTraderApi * self , CThostFtdcQryTradeField * pQryTrade,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryTrade(pQryTrade, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryInvestorPosition(CThostFtdcTraderApi * self , CThostFtdcQryInvestorPositionField * pQryInvestorPosition,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryInvestorPosition(pQryInvestorPosition, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryTradingAccount(CThostFtdcTraderApi * self , CThostFtdcQryTradingAccountField * pQryTradingAccount,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryTradingAccount(pQryTradingAccount, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryInvestor(CThostFtdcTraderApi * self , CThostFtdcQryInvestorField * pQryInvestor,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryInvestor(pQryInvestor, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryTradingCode(CThostFtdcTraderApi * self , CThostFtdcQryTradingCodeField * pQryTradingCode,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryTradingCode(pQryTradingCode, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryInstrumentMarginRate(CThostFtdcTraderApi * self , CThostFtdcQryInstrumentMarginRateField * pQryInstrumentMarginRate,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryInstrumentMarginRate(pQryInstrumentMarginRate, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryInstrumentCommissionRate(CThostFtdcTraderApi * self , CThostFtdcQryInstrumentCommissionRateField * pQryInstrumentCommissionRate,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryInstrumentCommissionRate(pQryInstrumentCommissionRate, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryExchange(CThostFtdcTraderApi * self , CThostFtdcQryExchangeField * pQryExchange,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryExchange(pQryExchange, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryProduct(CThostFtdcTraderApi * self , CThostFtdcQryProductField * pQryProduct,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryProduct(pQryProduct, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryInstrument(CThostFtdcTraderApi * self , CThostFtdcQryInstrumentField * pQryInstrument,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryInstrument(pQryInstrument, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryDepthMarketData(CThostFtdcTraderApi * self , CThostFtdcQryDepthMarketDataField * pQryDepthMarketData,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryDepthMarketData(pQryDepthMarketData, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQrySettlementInfo(CThostFtdcTraderApi * self , CThostFtdcQrySettlementInfoField * pQrySettlementInfo,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQrySettlementInfo(pQrySettlementInfo, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryTransferBank(CThostFtdcTraderApi * self , CThostFtdcQryTransferBankField * pQryTransferBank,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryTransferBank(pQryTransferBank, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryInvestorPositionDetail(CThostFtdcTraderApi * self , CThostFtdcQryInvestorPositionDetailField * pQryInvestorPositionDetail,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryInvestorPositionDetail(pQryInvestorPositionDetail, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryNotice(CThostFtdcTraderApi * self , CThostFtdcQryNoticeField * pQryNotice,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryNotice(pQryNotice, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQrySettlementInfoConfirm(CThostFtdcTraderApi * self , CThostFtdcQrySettlementInfoConfirmField * pQrySettlementInfoConfirm,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQrySettlementInfoConfirm(pQrySettlementInfoConfirm, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryInvestorPositionCombineDetail(CThostFtdcTraderApi * self , CThostFtdcQryInvestorPositionCombineDetailField * pQryInvestorPositionCombineDetail,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryInvestorPositionCombineDetail(pQryInvestorPositionCombineDetail, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryCFMMCTradingAccountKey(CThostFtdcTraderApi * self , CThostFtdcQryCFMMCTradingAccountKeyField * pQryCFMMCTradingAccountKey,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryCFMMCTradingAccountKey(pQryCFMMCTradingAccountKey, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryEWarrantOffset(CThostFtdcTraderApi * self , CThostFtdcQryEWarrantOffsetField * pQryEWarrantOffset,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryEWarrantOffset(pQryEWarrantOffset, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryInvestorProductGroupMargin(CThostFtdcTraderApi * self , CThostFtdcQryInvestorProductGroupMarginField * pQryInvestorProductGroupMargin,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryInvestorProductGroupMargin(pQryInvestorProductGroupMargin, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryExchangeMarginRate(CThostFtdcTraderApi * self , CThostFtdcQryExchangeMarginRateField * pQryExchangeMarginRate,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryExchangeMarginRate(pQryExchangeMarginRate, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryExchangeMarginRateAdjust(CThostFtdcTraderApi * self , CThostFtdcQryExchangeMarginRateAdjustField * pQryExchangeMarginRateAdjust,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryExchangeMarginRateAdjust(pQryExchangeMarginRateAdjust, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryExchangeRate(CThostFtdcTraderApi * self , CThostFtdcQryExchangeRateField * pQryExchangeRate,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryExchangeRate(pQryExchangeRate, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQrySecAgentACIDMap(CThostFtdcTraderApi * self , CThostFtdcQrySecAgentACIDMapField * pQrySecAgentACIDMap,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQrySecAgentACIDMap(pQrySecAgentACIDMap, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryProductExchRate(CThostFtdcTraderApi * self , CThostFtdcQryProductExchRateField * pQryProductExchRate,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryProductExchRate(pQryProductExchRate, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryProductGroup(CThostFtdcTraderApi * self , CThostFtdcQryProductGroupField * pQryProductGroup,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryProductGroup(pQryProductGroup, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryMMInstrumentCommissionRate(CThostFtdcTraderApi * self , CThostFtdcQryMMInstrumentCommissionRateField * pQryMMInstrumentCommissionRate,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryMMInstrumentCommissionRate(pQryMMInstrumentCommissionRate, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryMMOptionInstrCommRate(CThostFtdcTraderApi * self , CThostFtdcQryMMOptionInstrCommRateField * pQryMMOptionInstrCommRate,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryMMOptionInstrCommRate(pQryMMOptionInstrCommRate, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryInstrumentOrderCommRate(CThostFtdcTraderApi * self , CThostFtdcQryInstrumentOrderCommRateField * pQryInstrumentOrderCommRate,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryInstrumentOrderCommRate(pQryInstrumentOrderCommRate, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQrySecAgentTradingAccount(CThostFtdcTraderApi * self , CThostFtdcQryTradingAccountField * pQryTradingAccount,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQrySecAgentTradingAccount(pQryTradingAccount, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQrySecAgentCheckMode(CThostFtdcTraderApi * self , CThostFtdcQrySecAgentCheckModeField * pQrySecAgentCheckMode,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQrySecAgentCheckMode(pQrySecAgentCheckMode, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQrySecAgentTradeInfo(CThostFtdcTraderApi * self , CThostFtdcQrySecAgentTradeInfoField * pQrySecAgentTradeInfo,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQrySecAgentTradeInfo(pQrySecAgentTradeInfo, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryOptionInstrTradeCost(CThostFtdcTraderApi * self , CThostFtdcQryOptionInstrTradeCostField * pQryOptionInstrTradeCost,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryOptionInstrTradeCost(pQryOptionInstrTradeCost, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryOptionInstrCommRate(CThostFtdcTraderApi * self , CThostFtdcQryOptionInstrCommRateField * pQryOptionInstrCommRate,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryOptionInstrCommRate(pQryOptionInstrCommRate, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryExecOrder(CThostFtdcTraderApi * self , CThostFtdcQryExecOrderField * pQryExecOrder,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryExecOrder(pQryExecOrder, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryForQuote(CThostFtdcTraderApi * self , CThostFtdcQryForQuoteField * pQryForQuote,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryForQuote(pQryForQuote, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryQuote(CThostFtdcTraderApi * self , CThostFtdcQryQuoteField * pQryQuote,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryQuote(pQryQuote, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryOptionSelfClose(CThostFtdcTraderApi * self , CThostFtdcQryOptionSelfCloseField * pQryOptionSelfClose,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryOptionSelfClose(pQryOptionSelfClose, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryInvestUnit(CThostFtdcTraderApi * self , CThostFtdcQryInvestUnitField * pQryInvestUnit,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryInvestUnit(pQryInvestUnit, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryCombInstrumentGuard(CThostFtdcTraderApi * self , CThostFtdcQryCombInstrumentGuardField * pQryCombInstrumentGuard,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryCombInstrumentGuard(pQryCombInstrumentGuard, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryCombAction(CThostFtdcTraderApi * self , CThostFtdcQryCombActionField * pQryCombAction,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryCombAction(pQryCombAction, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryTransferSerial(CThostFtdcTraderApi * self , CThostFtdcQryTransferSerialField * pQryTransferSerial,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryTransferSerial(pQryTransferSerial, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryAccountregister(CThostFtdcTraderApi * self , CThostFtdcQryAccountregisterField * pQryAccountregister,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryAccountregister(pQryAccountregister, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryContractBank(CThostFtdcTraderApi * self , CThostFtdcQryContractBankField * pQryContractBank,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryContractBank(pQryContractBank, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryParkedOrder(CThostFtdcTraderApi * self , CThostFtdcQryParkedOrderField * pQryParkedOrder,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryParkedOrder(pQryParkedOrder, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryParkedOrderAction(CThostFtdcTraderApi * self , CThostFtdcQryParkedOrderActionField * pQryParkedOrderAction,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryParkedOrderAction(pQryParkedOrderAction, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryTradingNotice(CThostFtdcTraderApi * self , CThostFtdcQryTradingNoticeField * pQryTradingNotice,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryTradingNotice(pQryTradingNotice, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryBrokerTradingParams(CThostFtdcTraderApi * self , CThostFtdcQryBrokerTradingParamsField * pQryBrokerTradingParams,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryBrokerTradingParams(pQryBrokerTradingParams, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQryBrokerTradingAlgos(CThostFtdcTraderApi * self , CThostFtdcQryBrokerTradingAlgosField * pQryBrokerTradingAlgos,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQryBrokerTradingAlgos(pQryBrokerTradingAlgos, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQueryCFMMCTradingAccountToken(CThostFtdcTraderApi * self , CThostFtdcQueryCFMMCTradingAccountTokenField * pQueryCFMMCTradingAccountToken,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQueryCFMMCTradingAccountToken(pQueryCFMMCTradingAccountToken, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqFromBankToFutureByFuture(CThostFtdcTraderApi * self , CThostFtdcReqTransferField * pReqTransfer,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqFromBankToFutureByFuture(pReqTransfer, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqFromFutureToBankByFuture(CThostFtdcTraderApi * self , CThostFtdcReqTransferField * pReqTransfer,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqFromFutureToBankByFuture(pReqTransfer, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_ReqQueryBankAccountMoneyByFuture(CThostFtdcTraderApi * self , CThostFtdcReqQueryAccountField * pReqQueryAccount,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->ReqQueryBankAccountMoneyByFuture(pReqQueryAccount, nRequestID);
}
        
extern "C" void CThostFtdcTraderApi_CThostFtdcTraderApi(CThostFtdcTraderApi * self ) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->CThostFtdcTraderApi();
}
        extern "C" void RustCtpActionOnFrontDisconnected(void *rust_call_object int nReason)extern "C" void RustCtpActionOnHeartBeatWarning(void *rust_call_object int nTimeLapse)extern "C" void RustCtpActionOnRspAuthenticate(void *rust_call_object CThostFtdcRspAuthenticateField * pRspAuthenticateField,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspUserLogin(void *rust_call_object CThostFtdcRspUserLoginField * pRspUserLogin,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspUserLogout(void *rust_call_object CThostFtdcUserLogoutField * pUserLogout,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspUserPasswordUpdate(void *rust_call_object CThostFtdcUserPasswordUpdateField * pUserPasswordUpdate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspTradingAccountPasswordUpdate(void *rust_call_object CThostFtdcTradingAccountPasswordUpdateField * pTradingAccountPasswordUpdate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspUserAuthMethod(void *rust_call_object CThostFtdcRspUserAuthMethodField * pRspUserAuthMethod,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspGenUserCaptcha(void *rust_call_object CThostFtdcRspGenUserCaptchaField * pRspGenUserCaptcha,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspGenUserText(void *rust_call_object CThostFtdcRspGenUserTextField * pRspGenUserText,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspOrderInsert(void *rust_call_object CThostFtdcInputOrderField * pInputOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspParkedOrderInsert(void *rust_call_object CThostFtdcParkedOrderField * pParkedOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspParkedOrderAction(void *rust_call_object CThostFtdcParkedOrderActionField * pParkedOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspOrderAction(void *rust_call_object CThostFtdcInputOrderActionField * pInputOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQueryMaxOrderVolume(void *rust_call_object CThostFtdcQueryMaxOrderVolumeField * pQueryMaxOrderVolume,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspSettlementInfoConfirm(void *rust_call_object CThostFtdcSettlementInfoConfirmField * pSettlementInfoConfirm,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspRemoveParkedOrder(void *rust_call_object CThostFtdcRemoveParkedOrderField * pRemoveParkedOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspRemoveParkedOrderAction(void *rust_call_object CThostFtdcRemoveParkedOrderActionField * pRemoveParkedOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspExecOrderInsert(void *rust_call_object CThostFtdcInputExecOrderField * pInputExecOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspExecOrderAction(void *rust_call_object CThostFtdcInputExecOrderActionField * pInputExecOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspForQuoteInsert(void *rust_call_object CThostFtdcInputForQuoteField * pInputForQuote,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQuoteInsert(void *rust_call_object CThostFtdcInputQuoteField * pInputQuote,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQuoteAction(void *rust_call_object CThostFtdcInputQuoteActionField * pInputQuoteAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspBatchOrderAction(void *rust_call_object CThostFtdcInputBatchOrderActionField * pInputBatchOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspOptionSelfCloseInsert(void *rust_call_object CThostFtdcInputOptionSelfCloseField * pInputOptionSelfClose,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspOptionSelfCloseAction(void *rust_call_object CThostFtdcInputOptionSelfCloseActionField * pInputOptionSelfCloseAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspCombActionInsert(void *rust_call_object CThostFtdcInputCombActionField * pInputCombAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryOrder(void *rust_call_object CThostFtdcOrderField * pOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryTrade(void *rust_call_object CThostFtdcTradeField * pTrade,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryInvestorPosition(void *rust_call_object CThostFtdcInvestorPositionField * pInvestorPosition,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryTradingAccount(void *rust_call_object CThostFtdcTradingAccountField * pTradingAccount,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryInvestor(void *rust_call_object CThostFtdcInvestorField * pInvestor,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryTradingCode(void *rust_call_object CThostFtdcTradingCodeField * pTradingCode,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryInstrumentMarginRate(void *rust_call_object CThostFtdcInstrumentMarginRateField * pInstrumentMarginRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryInstrumentCommissionRate(void *rust_call_object CThostFtdcInstrumentCommissionRateField * pInstrumentCommissionRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryExchange(void *rust_call_object CThostFtdcExchangeField * pExchange,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryProduct(void *rust_call_object CThostFtdcProductField * pProduct,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryInstrument(void *rust_call_object CThostFtdcInstrumentField * pInstrument,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryDepthMarketData(void *rust_call_object CThostFtdcDepthMarketDataField * pDepthMarketData,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQrySettlementInfo(void *rust_call_object CThostFtdcSettlementInfoField * pSettlementInfo,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryTransferBank(void *rust_call_object CThostFtdcTransferBankField * pTransferBank,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryInvestorPositionDetail(void *rust_call_object CThostFtdcInvestorPositionDetailField * pInvestorPositionDetail,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryNotice(void *rust_call_object CThostFtdcNoticeField * pNotice,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQrySettlementInfoConfirm(void *rust_call_object CThostFtdcSettlementInfoConfirmField * pSettlementInfoConfirm,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryInvestorPositionCombineDetail(void *rust_call_object CThostFtdcInvestorPositionCombineDetailField * pInvestorPositionCombineDetail,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryCFMMCTradingAccountKey(void *rust_call_object CThostFtdcCFMMCTradingAccountKeyField * pCFMMCTradingAccountKey,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryEWarrantOffset(void *rust_call_object CThostFtdcEWarrantOffsetField * pEWarrantOffset,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryInvestorProductGroupMargin(void *rust_call_object CThostFtdcInvestorProductGroupMarginField * pInvestorProductGroupMargin,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryExchangeMarginRate(void *rust_call_object CThostFtdcExchangeMarginRateField * pExchangeMarginRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryExchangeMarginRateAdjust(void *rust_call_object CThostFtdcExchangeMarginRateAdjustField * pExchangeMarginRateAdjust,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryExchangeRate(void *rust_call_object CThostFtdcExchangeRateField * pExchangeRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQrySecAgentACIDMap(void *rust_call_object CThostFtdcSecAgentACIDMapField * pSecAgentACIDMap,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryProductExchRate(void *rust_call_object CThostFtdcProductExchRateField * pProductExchRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryProductGroup(void *rust_call_object CThostFtdcProductGroupField * pProductGroup,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryMMInstrumentCommissionRate(void *rust_call_object CThostFtdcMMInstrumentCommissionRateField * pMMInstrumentCommissionRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryMMOptionInstrCommRate(void *rust_call_object CThostFtdcMMOptionInstrCommRateField * pMMOptionInstrCommRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryInstrumentOrderCommRate(void *rust_call_object CThostFtdcInstrumentOrderCommRateField * pInstrumentOrderCommRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQrySecAgentTradingAccount(void *rust_call_object CThostFtdcTradingAccountField * pTradingAccount,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQrySecAgentCheckMode(void *rust_call_object CThostFtdcSecAgentCheckModeField * pSecAgentCheckMode,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQrySecAgentTradeInfo(void *rust_call_object CThostFtdcSecAgentTradeInfoField * pSecAgentTradeInfo,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryOptionInstrTradeCost(void *rust_call_object CThostFtdcOptionInstrTradeCostField * pOptionInstrTradeCost,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryOptionInstrCommRate(void *rust_call_object CThostFtdcOptionInstrCommRateField * pOptionInstrCommRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryExecOrder(void *rust_call_object CThostFtdcExecOrderField * pExecOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryForQuote(void *rust_call_object CThostFtdcForQuoteField * pForQuote,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryQuote(void *rust_call_object CThostFtdcQuoteField * pQuote,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryOptionSelfClose(void *rust_call_object CThostFtdcOptionSelfCloseField * pOptionSelfClose,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryInvestUnit(void *rust_call_object CThostFtdcInvestUnitField * pInvestUnit,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryCombInstrumentGuard(void *rust_call_object CThostFtdcCombInstrumentGuardField * pCombInstrumentGuard,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryCombAction(void *rust_call_object CThostFtdcCombActionField * pCombAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryTransferSerial(void *rust_call_object CThostFtdcTransferSerialField * pTransferSerial,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryAccountregister(void *rust_call_object CThostFtdcAccountregisterField * pAccountregister,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspError(void *rust_call_object CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRtnOrder(void *rust_call_object CThostFtdcOrderField * pOrder)extern "C" void RustCtpActionOnRtnTrade(void *rust_call_object CThostFtdcTradeField * pTrade)extern "C" void RustCtpActionOnErrRtnOrderInsert(void *rust_call_object CThostFtdcInputOrderField * pInputOrder,CThostFtdcRspInfoField * pRspInfo)extern "C" void RustCtpActionOnErrRtnOrderAction(void *rust_call_object CThostFtdcOrderActionField * pOrderAction,CThostFtdcRspInfoField * pRspInfo)extern "C" void RustCtpActionOnRtnInstrumentStatus(void *rust_call_object CThostFtdcInstrumentStatusField * pInstrumentStatus)extern "C" void RustCtpActionOnRtnBulletin(void *rust_call_object CThostFtdcBulletinField * pBulletin)extern "C" void RustCtpActionOnRtnTradingNotice(void *rust_call_object CThostFtdcTradingNoticeInfoField * pTradingNoticeInfo)extern "C" void RustCtpActionOnRtnErrorConditionalOrder(void *rust_call_object CThostFtdcErrorConditionalOrderField * pErrorConditionalOrder)extern "C" void RustCtpActionOnRtnExecOrder(void *rust_call_object CThostFtdcExecOrderField * pExecOrder)extern "C" void RustCtpActionOnErrRtnExecOrderInsert(void *rust_call_object CThostFtdcInputExecOrderField * pInputExecOrder,CThostFtdcRspInfoField * pRspInfo)extern "C" void RustCtpActionOnErrRtnExecOrderAction(void *rust_call_object CThostFtdcExecOrderActionField * pExecOrderAction,CThostFtdcRspInfoField * pRspInfo)extern "C" void RustCtpActionOnErrRtnForQuoteInsert(void *rust_call_object CThostFtdcInputForQuoteField * pInputForQuote,CThostFtdcRspInfoField * pRspInfo)extern "C" void RustCtpActionOnRtnQuote(void *rust_call_object CThostFtdcQuoteField * pQuote)extern "C" void RustCtpActionOnErrRtnQuoteInsert(void *rust_call_object CThostFtdcInputQuoteField * pInputQuote,CThostFtdcRspInfoField * pRspInfo)extern "C" void RustCtpActionOnErrRtnQuoteAction(void *rust_call_object CThostFtdcQuoteActionField * pQuoteAction,CThostFtdcRspInfoField * pRspInfo)extern "C" void RustCtpActionOnRtnForQuoteRsp(void *rust_call_object CThostFtdcForQuoteRspField * pForQuoteRsp)extern "C" void RustCtpActionOnRtnCFMMCTradingAccountToken(void *rust_call_object CThostFtdcCFMMCTradingAccountTokenField * pCFMMCTradingAccountToken)extern "C" void RustCtpActionOnErrRtnBatchOrderAction(void *rust_call_object CThostFtdcBatchOrderActionField * pBatchOrderAction,CThostFtdcRspInfoField * pRspInfo)extern "C" void RustCtpActionOnRtnOptionSelfClose(void *rust_call_object CThostFtdcOptionSelfCloseField * pOptionSelfClose)extern "C" void RustCtpActionOnErrRtnOptionSelfCloseInsert(void *rust_call_object CThostFtdcInputOptionSelfCloseField * pInputOptionSelfClose,CThostFtdcRspInfoField * pRspInfo)extern "C" void RustCtpActionOnErrRtnOptionSelfCloseAction(void *rust_call_object CThostFtdcOptionSelfCloseActionField * pOptionSelfCloseAction,CThostFtdcRspInfoField * pRspInfo)extern "C" void RustCtpActionOnRtnCombAction(void *rust_call_object CThostFtdcCombActionField * pCombAction)extern "C" void RustCtpActionOnErrRtnCombActionInsert(void *rust_call_object CThostFtdcInputCombActionField * pInputCombAction,CThostFtdcRspInfoField * pRspInfo)extern "C" void RustCtpActionOnRspQryContractBank(void *rust_call_object CThostFtdcContractBankField * pContractBank,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryParkedOrder(void *rust_call_object CThostFtdcParkedOrderField * pParkedOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryParkedOrderAction(void *rust_call_object CThostFtdcParkedOrderActionField * pParkedOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryTradingNotice(void *rust_call_object CThostFtdcTradingNoticeField * pTradingNotice,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryBrokerTradingParams(void *rust_call_object CThostFtdcBrokerTradingParamsField * pBrokerTradingParams,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQryBrokerTradingAlgos(void *rust_call_object CThostFtdcBrokerTradingAlgosField * pBrokerTradingAlgos,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQueryCFMMCTradingAccountToken(void *rust_call_object CThostFtdcQueryCFMMCTradingAccountTokenField * pQueryCFMMCTradingAccountToken,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRtnFromBankToFutureByBank(void *rust_call_object CThostFtdcRspTransferField * pRspTransfer)extern "C" void RustCtpActionOnRtnFromFutureToBankByBank(void *rust_call_object CThostFtdcRspTransferField * pRspTransfer)extern "C" void RustCtpActionOnRtnRepealFromBankToFutureByBank(void *rust_call_object CThostFtdcRspRepealField * pRspRepeal)extern "C" void RustCtpActionOnRtnRepealFromFutureToBankByBank(void *rust_call_object CThostFtdcRspRepealField * pRspRepeal)extern "C" void RustCtpActionOnRtnFromBankToFutureByFuture(void *rust_call_object CThostFtdcRspTransferField * pRspTransfer)extern "C" void RustCtpActionOnRtnFromFutureToBankByFuture(void *rust_call_object CThostFtdcRspTransferField * pRspTransfer)extern "C" void RustCtpActionOnRtnRepealFromBankToFutureByFutureManual(void *rust_call_object CThostFtdcRspRepealField * pRspRepeal)extern "C" void RustCtpActionOnRtnRepealFromFutureToBankByFutureManual(void *rust_call_object CThostFtdcRspRepealField * pRspRepeal)extern "C" void RustCtpActionOnRtnQueryBankBalanceByFuture(void *rust_call_object CThostFtdcNotifyQueryAccountField * pNotifyQueryAccount)extern "C" void RustCtpActionOnErrRtnBankToFutureByFuture(void *rust_call_object CThostFtdcReqTransferField * pReqTransfer,CThostFtdcRspInfoField * pRspInfo)extern "C" void RustCtpActionOnErrRtnFutureToBankByFuture(void *rust_call_object CThostFtdcReqTransferField * pReqTransfer,CThostFtdcRspInfoField * pRspInfo)extern "C" void RustCtpActionOnErrRtnRepealBankToFutureByFutureManual(void *rust_call_object CThostFtdcReqRepealField * pReqRepeal,CThostFtdcRspInfoField * pRspInfo)extern "C" void RustCtpActionOnErrRtnRepealFutureToBankByFutureManual(void *rust_call_object CThostFtdcReqRepealField * pReqRepeal,CThostFtdcRspInfoField * pRspInfo)extern "C" void RustCtpActionOnErrRtnQueryBankBalanceByFuture(void *rust_call_object CThostFtdcReqQueryAccountField * pReqQueryAccount,CThostFtdcRspInfoField * pRspInfo)extern "C" void RustCtpActionOnRtnRepealFromBankToFutureByFuture(void *rust_call_object CThostFtdcRspRepealField * pRspRepeal)extern "C" void RustCtpActionOnRtnRepealFromFutureToBankByFuture(void *rust_call_object CThostFtdcRspRepealField * pRspRepeal)extern "C" void RustCtpActionOnRspFromBankToFutureByFuture(void *rust_call_object CThostFtdcReqTransferField * pReqTransfer,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspFromFutureToBankByFuture(void *rust_call_object CThostFtdcReqTransferField * pReqTransfer,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRspQueryBankAccountMoneyByFuture(void *rust_call_object CThostFtdcReqQueryAccountField * pReqQueryAccount,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast)extern "C" void RustCtpActionOnRtnOpenAccountByBank(void *rust_call_object CThostFtdcOpenAccountField * pOpenAccount)extern "C" void RustCtpActionOnRtnCancelAccountByBank(void *rust_call_object CThostFtdcCancelAccountField * pCancelAccount)extern "C" void RustCtpActionOnRtnChangeAccountByBank(void *rust_call_object CThostFtdcChangeAccountField * pChangeAccount)
class CtpTdSpi: CThostFtdcTraderSpi {
public:
    void *rust_call_object;
    
    CtpTdSpi(void *rust_call_object) : rust_call_object(rust_call_object) {};
                
            
     void OnFrontDisconnected(int nReason){
          RustCtpActionOnFrontDisconnected(this->rust_call_object, nReason);            
     }
            
     void OnHeartBeatWarning(int nTimeLapse){
          RustCtpActionOnHeartBeatWarning(this->rust_call_object, nTimeLapse);            
     }
            
     void OnRspAuthenticate(CThostFtdcRspAuthenticateField * pRspAuthenticateField,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspAuthenticate(this->rust_call_object, pRspAuthenticateField, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspUserLogin(CThostFtdcRspUserLoginField * pRspUserLogin,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspUserLogin(this->rust_call_object, pRspUserLogin, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspUserLogout(CThostFtdcUserLogoutField * pUserLogout,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspUserLogout(this->rust_call_object, pUserLogout, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspUserPasswordUpdate(CThostFtdcUserPasswordUpdateField * pUserPasswordUpdate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspUserPasswordUpdate(this->rust_call_object, pUserPasswordUpdate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspTradingAccountPasswordUpdate(CThostFtdcTradingAccountPasswordUpdateField * pTradingAccountPasswordUpdate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspTradingAccountPasswordUpdate(this->rust_call_object, pTradingAccountPasswordUpdate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspUserAuthMethod(CThostFtdcRspUserAuthMethodField * pRspUserAuthMethod,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspUserAuthMethod(this->rust_call_object, pRspUserAuthMethod, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspGenUserCaptcha(CThostFtdcRspGenUserCaptchaField * pRspGenUserCaptcha,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspGenUserCaptcha(this->rust_call_object, pRspGenUserCaptcha, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspGenUserText(CThostFtdcRspGenUserTextField * pRspGenUserText,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspGenUserText(this->rust_call_object, pRspGenUserText, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspOrderInsert(CThostFtdcInputOrderField * pInputOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspOrderInsert(this->rust_call_object, pInputOrder, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspParkedOrderInsert(CThostFtdcParkedOrderField * pParkedOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspParkedOrderInsert(this->rust_call_object, pParkedOrder, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspParkedOrderAction(CThostFtdcParkedOrderActionField * pParkedOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspParkedOrderAction(this->rust_call_object, pParkedOrderAction, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspOrderAction(CThostFtdcInputOrderActionField * pInputOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspOrderAction(this->rust_call_object, pInputOrderAction, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQueryMaxOrderVolume(CThostFtdcQueryMaxOrderVolumeField * pQueryMaxOrderVolume,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQueryMaxOrderVolume(this->rust_call_object, pQueryMaxOrderVolume, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspSettlementInfoConfirm(CThostFtdcSettlementInfoConfirmField * pSettlementInfoConfirm,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspSettlementInfoConfirm(this->rust_call_object, pSettlementInfoConfirm, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspRemoveParkedOrder(CThostFtdcRemoveParkedOrderField * pRemoveParkedOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspRemoveParkedOrder(this->rust_call_object, pRemoveParkedOrder, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspRemoveParkedOrderAction(CThostFtdcRemoveParkedOrderActionField * pRemoveParkedOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspRemoveParkedOrderAction(this->rust_call_object, pRemoveParkedOrderAction, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspExecOrderInsert(CThostFtdcInputExecOrderField * pInputExecOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspExecOrderInsert(this->rust_call_object, pInputExecOrder, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspExecOrderAction(CThostFtdcInputExecOrderActionField * pInputExecOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspExecOrderAction(this->rust_call_object, pInputExecOrderAction, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspForQuoteInsert(CThostFtdcInputForQuoteField * pInputForQuote,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspForQuoteInsert(this->rust_call_object, pInputForQuote, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQuoteInsert(CThostFtdcInputQuoteField * pInputQuote,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQuoteInsert(this->rust_call_object, pInputQuote, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQuoteAction(CThostFtdcInputQuoteActionField * pInputQuoteAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQuoteAction(this->rust_call_object, pInputQuoteAction, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspBatchOrderAction(CThostFtdcInputBatchOrderActionField * pInputBatchOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspBatchOrderAction(this->rust_call_object, pInputBatchOrderAction, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspOptionSelfCloseInsert(CThostFtdcInputOptionSelfCloseField * pInputOptionSelfClose,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspOptionSelfCloseInsert(this->rust_call_object, pInputOptionSelfClose, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspOptionSelfCloseAction(CThostFtdcInputOptionSelfCloseActionField * pInputOptionSelfCloseAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspOptionSelfCloseAction(this->rust_call_object, pInputOptionSelfCloseAction, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspCombActionInsert(CThostFtdcInputCombActionField * pInputCombAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspCombActionInsert(this->rust_call_object, pInputCombAction, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryOrder(CThostFtdcOrderField * pOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryOrder(this->rust_call_object, pOrder, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryTrade(CThostFtdcTradeField * pTrade,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryTrade(this->rust_call_object, pTrade, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInvestorPosition(CThostFtdcInvestorPositionField * pInvestorPosition,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryInvestorPosition(this->rust_call_object, pInvestorPosition, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryTradingAccount(CThostFtdcTradingAccountField * pTradingAccount,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryTradingAccount(this->rust_call_object, pTradingAccount, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInvestor(CThostFtdcInvestorField * pInvestor,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryInvestor(this->rust_call_object, pInvestor, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryTradingCode(CThostFtdcTradingCodeField * pTradingCode,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryTradingCode(this->rust_call_object, pTradingCode, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInstrumentMarginRate(CThostFtdcInstrumentMarginRateField * pInstrumentMarginRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryInstrumentMarginRate(this->rust_call_object, pInstrumentMarginRate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInstrumentCommissionRate(CThostFtdcInstrumentCommissionRateField * pInstrumentCommissionRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryInstrumentCommissionRate(this->rust_call_object, pInstrumentCommissionRate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryExchange(CThostFtdcExchangeField * pExchange,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryExchange(this->rust_call_object, pExchange, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryProduct(CThostFtdcProductField * pProduct,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryProduct(this->rust_call_object, pProduct, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInstrument(CThostFtdcInstrumentField * pInstrument,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryInstrument(this->rust_call_object, pInstrument, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryDepthMarketData(CThostFtdcDepthMarketDataField * pDepthMarketData,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryDepthMarketData(this->rust_call_object, pDepthMarketData, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQrySettlementInfo(CThostFtdcSettlementInfoField * pSettlementInfo,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQrySettlementInfo(this->rust_call_object, pSettlementInfo, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryTransferBank(CThostFtdcTransferBankField * pTransferBank,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryTransferBank(this->rust_call_object, pTransferBank, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInvestorPositionDetail(CThostFtdcInvestorPositionDetailField * pInvestorPositionDetail,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryInvestorPositionDetail(this->rust_call_object, pInvestorPositionDetail, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryNotice(CThostFtdcNoticeField * pNotice,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryNotice(this->rust_call_object, pNotice, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQrySettlementInfoConfirm(CThostFtdcSettlementInfoConfirmField * pSettlementInfoConfirm,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQrySettlementInfoConfirm(this->rust_call_object, pSettlementInfoConfirm, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInvestorPositionCombineDetail(CThostFtdcInvestorPositionCombineDetailField * pInvestorPositionCombineDetail,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryInvestorPositionCombineDetail(this->rust_call_object, pInvestorPositionCombineDetail, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryCFMMCTradingAccountKey(CThostFtdcCFMMCTradingAccountKeyField * pCFMMCTradingAccountKey,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryCFMMCTradingAccountKey(this->rust_call_object, pCFMMCTradingAccountKey, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryEWarrantOffset(CThostFtdcEWarrantOffsetField * pEWarrantOffset,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryEWarrantOffset(this->rust_call_object, pEWarrantOffset, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInvestorProductGroupMargin(CThostFtdcInvestorProductGroupMarginField * pInvestorProductGroupMargin,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryInvestorProductGroupMargin(this->rust_call_object, pInvestorProductGroupMargin, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryExchangeMarginRate(CThostFtdcExchangeMarginRateField * pExchangeMarginRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryExchangeMarginRate(this->rust_call_object, pExchangeMarginRate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryExchangeMarginRateAdjust(CThostFtdcExchangeMarginRateAdjustField * pExchangeMarginRateAdjust,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryExchangeMarginRateAdjust(this->rust_call_object, pExchangeMarginRateAdjust, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryExchangeRate(CThostFtdcExchangeRateField * pExchangeRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryExchangeRate(this->rust_call_object, pExchangeRate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQrySecAgentACIDMap(CThostFtdcSecAgentACIDMapField * pSecAgentACIDMap,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQrySecAgentACIDMap(this->rust_call_object, pSecAgentACIDMap, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryProductExchRate(CThostFtdcProductExchRateField * pProductExchRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryProductExchRate(this->rust_call_object, pProductExchRate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryProductGroup(CThostFtdcProductGroupField * pProductGroup,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryProductGroup(this->rust_call_object, pProductGroup, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryMMInstrumentCommissionRate(CThostFtdcMMInstrumentCommissionRateField * pMMInstrumentCommissionRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryMMInstrumentCommissionRate(this->rust_call_object, pMMInstrumentCommissionRate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryMMOptionInstrCommRate(CThostFtdcMMOptionInstrCommRateField * pMMOptionInstrCommRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryMMOptionInstrCommRate(this->rust_call_object, pMMOptionInstrCommRate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInstrumentOrderCommRate(CThostFtdcInstrumentOrderCommRateField * pInstrumentOrderCommRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryInstrumentOrderCommRate(this->rust_call_object, pInstrumentOrderCommRate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQrySecAgentTradingAccount(CThostFtdcTradingAccountField * pTradingAccount,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQrySecAgentTradingAccount(this->rust_call_object, pTradingAccount, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQrySecAgentCheckMode(CThostFtdcSecAgentCheckModeField * pSecAgentCheckMode,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQrySecAgentCheckMode(this->rust_call_object, pSecAgentCheckMode, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQrySecAgentTradeInfo(CThostFtdcSecAgentTradeInfoField * pSecAgentTradeInfo,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQrySecAgentTradeInfo(this->rust_call_object, pSecAgentTradeInfo, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryOptionInstrTradeCost(CThostFtdcOptionInstrTradeCostField * pOptionInstrTradeCost,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryOptionInstrTradeCost(this->rust_call_object, pOptionInstrTradeCost, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryOptionInstrCommRate(CThostFtdcOptionInstrCommRateField * pOptionInstrCommRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryOptionInstrCommRate(this->rust_call_object, pOptionInstrCommRate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryExecOrder(CThostFtdcExecOrderField * pExecOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryExecOrder(this->rust_call_object, pExecOrder, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryForQuote(CThostFtdcForQuoteField * pForQuote,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryForQuote(this->rust_call_object, pForQuote, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryQuote(CThostFtdcQuoteField * pQuote,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryQuote(this->rust_call_object, pQuote, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryOptionSelfClose(CThostFtdcOptionSelfCloseField * pOptionSelfClose,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryOptionSelfClose(this->rust_call_object, pOptionSelfClose, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInvestUnit(CThostFtdcInvestUnitField * pInvestUnit,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryInvestUnit(this->rust_call_object, pInvestUnit, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryCombInstrumentGuard(CThostFtdcCombInstrumentGuardField * pCombInstrumentGuard,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryCombInstrumentGuard(this->rust_call_object, pCombInstrumentGuard, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryCombAction(CThostFtdcCombActionField * pCombAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryCombAction(this->rust_call_object, pCombAction, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryTransferSerial(CThostFtdcTransferSerialField * pTransferSerial,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryTransferSerial(this->rust_call_object, pTransferSerial, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryAccountregister(CThostFtdcAccountregisterField * pAccountregister,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryAccountregister(this->rust_call_object, pAccountregister, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspError(CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspError(this->rust_call_object, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRtnOrder(CThostFtdcOrderField * pOrder){
          RustCtpActionOnRtnOrder(this->rust_call_object, pOrder);            
     }
            
     void OnRtnTrade(CThostFtdcTradeField * pTrade){
          RustCtpActionOnRtnTrade(this->rust_call_object, pTrade);            
     }
            
     void OnErrRtnOrderInsert(CThostFtdcInputOrderField * pInputOrder,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionOnErrRtnOrderInsert(this->rust_call_object, pInputOrder, pRspInfo);            
     }
            
     void OnErrRtnOrderAction(CThostFtdcOrderActionField * pOrderAction,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionOnErrRtnOrderAction(this->rust_call_object, pOrderAction, pRspInfo);            
     }
            
     void OnRtnInstrumentStatus(CThostFtdcInstrumentStatusField * pInstrumentStatus){
          RustCtpActionOnRtnInstrumentStatus(this->rust_call_object, pInstrumentStatus);            
     }
            
     void OnRtnBulletin(CThostFtdcBulletinField * pBulletin){
          RustCtpActionOnRtnBulletin(this->rust_call_object, pBulletin);            
     }
            
     void OnRtnTradingNotice(CThostFtdcTradingNoticeInfoField * pTradingNoticeInfo){
          RustCtpActionOnRtnTradingNotice(this->rust_call_object, pTradingNoticeInfo);            
     }
            
     void OnRtnErrorConditionalOrder(CThostFtdcErrorConditionalOrderField * pErrorConditionalOrder){
          RustCtpActionOnRtnErrorConditionalOrder(this->rust_call_object, pErrorConditionalOrder);            
     }
            
     void OnRtnExecOrder(CThostFtdcExecOrderField * pExecOrder){
          RustCtpActionOnRtnExecOrder(this->rust_call_object, pExecOrder);            
     }
            
     void OnErrRtnExecOrderInsert(CThostFtdcInputExecOrderField * pInputExecOrder,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionOnErrRtnExecOrderInsert(this->rust_call_object, pInputExecOrder, pRspInfo);            
     }
            
     void OnErrRtnExecOrderAction(CThostFtdcExecOrderActionField * pExecOrderAction,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionOnErrRtnExecOrderAction(this->rust_call_object, pExecOrderAction, pRspInfo);            
     }
            
     void OnErrRtnForQuoteInsert(CThostFtdcInputForQuoteField * pInputForQuote,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionOnErrRtnForQuoteInsert(this->rust_call_object, pInputForQuote, pRspInfo);            
     }
            
     void OnRtnQuote(CThostFtdcQuoteField * pQuote){
          RustCtpActionOnRtnQuote(this->rust_call_object, pQuote);            
     }
            
     void OnErrRtnQuoteInsert(CThostFtdcInputQuoteField * pInputQuote,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionOnErrRtnQuoteInsert(this->rust_call_object, pInputQuote, pRspInfo);            
     }
            
     void OnErrRtnQuoteAction(CThostFtdcQuoteActionField * pQuoteAction,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionOnErrRtnQuoteAction(this->rust_call_object, pQuoteAction, pRspInfo);            
     }
            
     void OnRtnForQuoteRsp(CThostFtdcForQuoteRspField * pForQuoteRsp){
          RustCtpActionOnRtnForQuoteRsp(this->rust_call_object, pForQuoteRsp);            
     }
            
     void OnRtnCFMMCTradingAccountToken(CThostFtdcCFMMCTradingAccountTokenField * pCFMMCTradingAccountToken){
          RustCtpActionOnRtnCFMMCTradingAccountToken(this->rust_call_object, pCFMMCTradingAccountToken);            
     }
            
     void OnErrRtnBatchOrderAction(CThostFtdcBatchOrderActionField * pBatchOrderAction,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionOnErrRtnBatchOrderAction(this->rust_call_object, pBatchOrderAction, pRspInfo);            
     }
            
     void OnRtnOptionSelfClose(CThostFtdcOptionSelfCloseField * pOptionSelfClose){
          RustCtpActionOnRtnOptionSelfClose(this->rust_call_object, pOptionSelfClose);            
     }
            
     void OnErrRtnOptionSelfCloseInsert(CThostFtdcInputOptionSelfCloseField * pInputOptionSelfClose,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionOnErrRtnOptionSelfCloseInsert(this->rust_call_object, pInputOptionSelfClose, pRspInfo);            
     }
            
     void OnErrRtnOptionSelfCloseAction(CThostFtdcOptionSelfCloseActionField * pOptionSelfCloseAction,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionOnErrRtnOptionSelfCloseAction(this->rust_call_object, pOptionSelfCloseAction, pRspInfo);            
     }
            
     void OnRtnCombAction(CThostFtdcCombActionField * pCombAction){
          RustCtpActionOnRtnCombAction(this->rust_call_object, pCombAction);            
     }
            
     void OnErrRtnCombActionInsert(CThostFtdcInputCombActionField * pInputCombAction,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionOnErrRtnCombActionInsert(this->rust_call_object, pInputCombAction, pRspInfo);            
     }
            
     void OnRspQryContractBank(CThostFtdcContractBankField * pContractBank,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryContractBank(this->rust_call_object, pContractBank, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryParkedOrder(CThostFtdcParkedOrderField * pParkedOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryParkedOrder(this->rust_call_object, pParkedOrder, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryParkedOrderAction(CThostFtdcParkedOrderActionField * pParkedOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryParkedOrderAction(this->rust_call_object, pParkedOrderAction, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryTradingNotice(CThostFtdcTradingNoticeField * pTradingNotice,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryTradingNotice(this->rust_call_object, pTradingNotice, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryBrokerTradingParams(CThostFtdcBrokerTradingParamsField * pBrokerTradingParams,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryBrokerTradingParams(this->rust_call_object, pBrokerTradingParams, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryBrokerTradingAlgos(CThostFtdcBrokerTradingAlgosField * pBrokerTradingAlgos,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQryBrokerTradingAlgos(this->rust_call_object, pBrokerTradingAlgos, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQueryCFMMCTradingAccountToken(CThostFtdcQueryCFMMCTradingAccountTokenField * pQueryCFMMCTradingAccountToken,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQueryCFMMCTradingAccountToken(this->rust_call_object, pQueryCFMMCTradingAccountToken, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRtnFromBankToFutureByBank(CThostFtdcRspTransferField * pRspTransfer){
          RustCtpActionOnRtnFromBankToFutureByBank(this->rust_call_object, pRspTransfer);            
     }
            
     void OnRtnFromFutureToBankByBank(CThostFtdcRspTransferField * pRspTransfer){
          RustCtpActionOnRtnFromFutureToBankByBank(this->rust_call_object, pRspTransfer);            
     }
            
     void OnRtnRepealFromBankToFutureByBank(CThostFtdcRspRepealField * pRspRepeal){
          RustCtpActionOnRtnRepealFromBankToFutureByBank(this->rust_call_object, pRspRepeal);            
     }
            
     void OnRtnRepealFromFutureToBankByBank(CThostFtdcRspRepealField * pRspRepeal){
          RustCtpActionOnRtnRepealFromFutureToBankByBank(this->rust_call_object, pRspRepeal);            
     }
            
     void OnRtnFromBankToFutureByFuture(CThostFtdcRspTransferField * pRspTransfer){
          RustCtpActionOnRtnFromBankToFutureByFuture(this->rust_call_object, pRspTransfer);            
     }
            
     void OnRtnFromFutureToBankByFuture(CThostFtdcRspTransferField * pRspTransfer){
          RustCtpActionOnRtnFromFutureToBankByFuture(this->rust_call_object, pRspTransfer);            
     }
            
     void OnRtnRepealFromBankToFutureByFutureManual(CThostFtdcRspRepealField * pRspRepeal){
          RustCtpActionOnRtnRepealFromBankToFutureByFutureManual(this->rust_call_object, pRspRepeal);            
     }
            
     void OnRtnRepealFromFutureToBankByFutureManual(CThostFtdcRspRepealField * pRspRepeal){
          RustCtpActionOnRtnRepealFromFutureToBankByFutureManual(this->rust_call_object, pRspRepeal);            
     }
            
     void OnRtnQueryBankBalanceByFuture(CThostFtdcNotifyQueryAccountField * pNotifyQueryAccount){
          RustCtpActionOnRtnQueryBankBalanceByFuture(this->rust_call_object, pNotifyQueryAccount);            
     }
            
     void OnErrRtnBankToFutureByFuture(CThostFtdcReqTransferField * pReqTransfer,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionOnErrRtnBankToFutureByFuture(this->rust_call_object, pReqTransfer, pRspInfo);            
     }
            
     void OnErrRtnFutureToBankByFuture(CThostFtdcReqTransferField * pReqTransfer,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionOnErrRtnFutureToBankByFuture(this->rust_call_object, pReqTransfer, pRspInfo);            
     }
            
     void OnErrRtnRepealBankToFutureByFutureManual(CThostFtdcReqRepealField * pReqRepeal,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionOnErrRtnRepealBankToFutureByFutureManual(this->rust_call_object, pReqRepeal, pRspInfo);            
     }
            
     void OnErrRtnRepealFutureToBankByFutureManual(CThostFtdcReqRepealField * pReqRepeal,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionOnErrRtnRepealFutureToBankByFutureManual(this->rust_call_object, pReqRepeal, pRspInfo);            
     }
            
     void OnErrRtnQueryBankBalanceByFuture(CThostFtdcReqQueryAccountField * pReqQueryAccount,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionOnErrRtnQueryBankBalanceByFuture(this->rust_call_object, pReqQueryAccount, pRspInfo);            
     }
            
     void OnRtnRepealFromBankToFutureByFuture(CThostFtdcRspRepealField * pRspRepeal){
          RustCtpActionOnRtnRepealFromBankToFutureByFuture(this->rust_call_object, pRspRepeal);            
     }
            
     void OnRtnRepealFromFutureToBankByFuture(CThostFtdcRspRepealField * pRspRepeal){
          RustCtpActionOnRtnRepealFromFutureToBankByFuture(this->rust_call_object, pRspRepeal);            
     }
            
     void OnRspFromBankToFutureByFuture(CThostFtdcReqTransferField * pReqTransfer,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspFromBankToFutureByFuture(this->rust_call_object, pReqTransfer, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspFromFutureToBankByFuture(CThostFtdcReqTransferField * pReqTransfer,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspFromFutureToBankByFuture(this->rust_call_object, pReqTransfer, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQueryBankAccountMoneyByFuture(CThostFtdcReqQueryAccountField * pReqQueryAccount,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionOnRspQueryBankAccountMoneyByFuture(this->rust_call_object, pReqQueryAccount, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRtnOpenAccountByBank(CThostFtdcOpenAccountField * pOpenAccount){
          RustCtpActionOnRtnOpenAccountByBank(this->rust_call_object, pOpenAccount);            
     }
            
     void OnRtnCancelAccountByBank(CThostFtdcCancelAccountField * pCancelAccount){
          RustCtpActionOnRtnCancelAccountByBank(this->rust_call_object, pCancelAccount);            
     }
            
     void OnRtnChangeAccountByBank(CThostFtdcChangeAccountField * pChangeAccount){
          RustCtpActionOnRtnChangeAccountByBank(this->rust_call_object, pChangeAccount);            
     }
            
protected:
    ~CtpTdSpi(){}
}
