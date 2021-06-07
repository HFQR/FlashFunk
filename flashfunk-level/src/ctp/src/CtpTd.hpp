# include "../../../sdk_sources/ctp/include/ThostFtdcMdApi.h"
# include "../../../sdk_sources/ctp/include/ThostFtdcTraderApi.h"
# include "../../../sdk_sources/ctp/include/ThostFtdcUserApiDataType.h"
# include "../../../sdk_sources/ctp/include/ThostFtdcUserApiStruct.h"

static CThostFtdcTraderApi * CThostFtdcTraderApiCreateFtdcTraderApi(CThostFtdcTraderApi * self , const char * pszFlowPath) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->CreateFtdcTraderApi(pszFlowPath);
}
        
static const char * CThostFtdcTraderApiGetApiVersion(CThostFtdcTraderApi * self ) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->GetApiVersion();
}
        
void CThostFtdcTraderApiRelease(CThostFtdcTraderApi * self ) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->Release();
}
        
void CThostFtdcTraderApiInit(CThostFtdcTraderApi * self ) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->Init();
}
        
int CThostFtdcTraderApiJoin(CThostFtdcTraderApi * self ) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->Join();
}
        
const char * CThostFtdcTraderApiGetTradingDay(CThostFtdcTraderApi * self ) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->GetTradingDay();
}
        
void CThostFtdcTraderApiRegisterFront(CThostFtdcTraderApi * self , char * pszFrontAddress) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->RegisterFront(pszFrontAddress);
}
        
void CThostFtdcTraderApiRegisterNameServer(CThostFtdcTraderApi * self , char * pszNsAddress) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->RegisterNameServer(pszNsAddress);
}
        
void CThostFtdcTraderApiRegisterFensUserInfo(CThostFtdcTraderApi * self , CThostFtdcFensUserInfoField * pFensUserInfo) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->RegisterFensUserInfo(pFensUserInfo);
}
        
void CThostFtdcTraderApiRegisterSpi(CThostFtdcTraderApi * self , CThostFtdcTraderSpi * pSpi) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->RegisterSpi(pSpi);
}
        
void CThostFtdcTraderApiSubscribePrivateTopic(CThostFtdcTraderApi * self , THOST_TE_RESUME_TYPE nResumeType) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->SubscribePrivateTopic(nResumeType);
}
        
void CThostFtdcTraderApiSubscribePublicTopic(CThostFtdcTraderApi * self , THOST_TE_RESUME_TYPE nResumeType) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->SubscribePublicTopic(nResumeType);
}
        
int CThostFtdcTraderApiReqAuthenticate(CThostFtdcTraderApi * self , CThostFtdcReqAuthenticateField * pReqAuthenticateField,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqAuthenticate(pReqAuthenticateField, nRequestID);
}
        
int CThostFtdcTraderApiRegisterUserSystemInfo(CThostFtdcTraderApi * self , CThostFtdcUserSystemInfoField * pUserSystemInfo) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->RegisterUserSystemInfo(pUserSystemInfo);
}
        
int CThostFtdcTraderApiSubmitUserSystemInfo(CThostFtdcTraderApi * self , CThostFtdcUserSystemInfoField * pUserSystemInfo) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->SubmitUserSystemInfo(pUserSystemInfo);
}
        
int CThostFtdcTraderApiReqUserLogin(CThostFtdcTraderApi * self , CThostFtdcReqUserLoginField * pReqUserLoginField,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqUserLogin(pReqUserLoginField, nRequestID);
}
        
int CThostFtdcTraderApiReqUserLogout(CThostFtdcTraderApi * self , CThostFtdcUserLogoutField * pUserLogout,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqUserLogout(pUserLogout, nRequestID);
}
        
int CThostFtdcTraderApiReqUserPasswordUpdate(CThostFtdcTraderApi * self , CThostFtdcUserPasswordUpdateField * pUserPasswordUpdate,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqUserPasswordUpdate(pUserPasswordUpdate, nRequestID);
}
        
int CThostFtdcTraderApiReqTradingAccountPasswordUpdate(CThostFtdcTraderApi * self , CThostFtdcTradingAccountPasswordUpdateField * pTradingAccountPasswordUpdate,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqTradingAccountPasswordUpdate(pTradingAccountPasswordUpdate, nRequestID);
}
        
int CThostFtdcTraderApiReqUserAuthMethod(CThostFtdcTraderApi * self , CThostFtdcReqUserAuthMethodField * pReqUserAuthMethod,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqUserAuthMethod(pReqUserAuthMethod, nRequestID);
}
        
int CThostFtdcTraderApiReqGenUserCaptcha(CThostFtdcTraderApi * self , CThostFtdcReqGenUserCaptchaField * pReqGenUserCaptcha,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqGenUserCaptcha(pReqGenUserCaptcha, nRequestID);
}
        
int CThostFtdcTraderApiReqGenUserText(CThostFtdcTraderApi * self , CThostFtdcReqGenUserTextField * pReqGenUserText,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqGenUserText(pReqGenUserText, nRequestID);
}
        
int CThostFtdcTraderApiReqUserLoginWithCaptcha(CThostFtdcTraderApi * self , CThostFtdcReqUserLoginWithCaptchaField * pReqUserLoginWithCaptcha,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqUserLoginWithCaptcha(pReqUserLoginWithCaptcha, nRequestID);
}
        
int CThostFtdcTraderApiReqUserLoginWithText(CThostFtdcTraderApi * self , CThostFtdcReqUserLoginWithTextField * pReqUserLoginWithText,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqUserLoginWithText(pReqUserLoginWithText, nRequestID);
}
        
int CThostFtdcTraderApiReqUserLoginWithOTP(CThostFtdcTraderApi * self , CThostFtdcReqUserLoginWithOTPField * pReqUserLoginWithOTP,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqUserLoginWithOTP(pReqUserLoginWithOTP, nRequestID);
}
        
int CThostFtdcTraderApiReqOrderInsert(CThostFtdcTraderApi * self , CThostFtdcInputOrderField * pInputOrder,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqOrderInsert(pInputOrder, nRequestID);
}
        
int CThostFtdcTraderApiReqParkedOrderInsert(CThostFtdcTraderApi * self , CThostFtdcParkedOrderField * pParkedOrder,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqParkedOrderInsert(pParkedOrder, nRequestID);
}
        
int CThostFtdcTraderApiReqParkedOrderAction(CThostFtdcTraderApi * self , CThostFtdcParkedOrderActionField * pParkedOrderAction,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqParkedOrderAction(pParkedOrderAction, nRequestID);
}
        
int CThostFtdcTraderApiReqOrderAction(CThostFtdcTraderApi * self , CThostFtdcInputOrderActionField * pInputOrderAction,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqOrderAction(pInputOrderAction, nRequestID);
}
        
int CThostFtdcTraderApiReqQueryMaxOrderVolume(CThostFtdcTraderApi * self , CThostFtdcQueryMaxOrderVolumeField * pQueryMaxOrderVolume,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQueryMaxOrderVolume(pQueryMaxOrderVolume, nRequestID);
}
        
int CThostFtdcTraderApiReqSettlementInfoConfirm(CThostFtdcTraderApi * self , CThostFtdcSettlementInfoConfirmField * pSettlementInfoConfirm,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqSettlementInfoConfirm(pSettlementInfoConfirm, nRequestID);
}
        
int CThostFtdcTraderApiReqRemoveParkedOrder(CThostFtdcTraderApi * self , CThostFtdcRemoveParkedOrderField * pRemoveParkedOrder,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqRemoveParkedOrder(pRemoveParkedOrder, nRequestID);
}
        
int CThostFtdcTraderApiReqRemoveParkedOrderAction(CThostFtdcTraderApi * self , CThostFtdcRemoveParkedOrderActionField * pRemoveParkedOrderAction,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqRemoveParkedOrderAction(pRemoveParkedOrderAction, nRequestID);
}
        
int CThostFtdcTraderApiReqExecOrderInsert(CThostFtdcTraderApi * self , CThostFtdcInputExecOrderField * pInputExecOrder,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqExecOrderInsert(pInputExecOrder, nRequestID);
}
        
int CThostFtdcTraderApiReqExecOrderAction(CThostFtdcTraderApi * self , CThostFtdcInputExecOrderActionField * pInputExecOrderAction,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqExecOrderAction(pInputExecOrderAction, nRequestID);
}
        
int CThostFtdcTraderApiReqForQuoteInsert(CThostFtdcTraderApi * self , CThostFtdcInputForQuoteField * pInputForQuote,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqForQuoteInsert(pInputForQuote, nRequestID);
}
        
int CThostFtdcTraderApiReqQuoteInsert(CThostFtdcTraderApi * self , CThostFtdcInputQuoteField * pInputQuote,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQuoteInsert(pInputQuote, nRequestID);
}
        
int CThostFtdcTraderApiReqQuoteAction(CThostFtdcTraderApi * self , CThostFtdcInputQuoteActionField * pInputQuoteAction,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQuoteAction(pInputQuoteAction, nRequestID);
}
        
int CThostFtdcTraderApiReqBatchOrderAction(CThostFtdcTraderApi * self , CThostFtdcInputBatchOrderActionField * pInputBatchOrderAction,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqBatchOrderAction(pInputBatchOrderAction, nRequestID);
}
        
int CThostFtdcTraderApiReqOptionSelfCloseInsert(CThostFtdcTraderApi * self , CThostFtdcInputOptionSelfCloseField * pInputOptionSelfClose,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqOptionSelfCloseInsert(pInputOptionSelfClose, nRequestID);
}
        
int CThostFtdcTraderApiReqOptionSelfCloseAction(CThostFtdcTraderApi * self , CThostFtdcInputOptionSelfCloseActionField * pInputOptionSelfCloseAction,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqOptionSelfCloseAction(pInputOptionSelfCloseAction, nRequestID);
}
        
int CThostFtdcTraderApiReqCombActionInsert(CThostFtdcTraderApi * self , CThostFtdcInputCombActionField * pInputCombAction,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqCombActionInsert(pInputCombAction, nRequestID);
}
        
int CThostFtdcTraderApiReqQryOrder(CThostFtdcTraderApi * self , CThostFtdcQryOrderField * pQryOrder,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryOrder(pQryOrder, nRequestID);
}
        
int CThostFtdcTraderApiReqQryTrade(CThostFtdcTraderApi * self , CThostFtdcQryTradeField * pQryTrade,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryTrade(pQryTrade, nRequestID);
}
        
int CThostFtdcTraderApiReqQryInvestorPosition(CThostFtdcTraderApi * self , CThostFtdcQryInvestorPositionField * pQryInvestorPosition,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryInvestorPosition(pQryInvestorPosition, nRequestID);
}
        
int CThostFtdcTraderApiReqQryTradingAccount(CThostFtdcTraderApi * self , CThostFtdcQryTradingAccountField * pQryTradingAccount,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryTradingAccount(pQryTradingAccount, nRequestID);
}
        
int CThostFtdcTraderApiReqQryInvestor(CThostFtdcTraderApi * self , CThostFtdcQryInvestorField * pQryInvestor,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryInvestor(pQryInvestor, nRequestID);
}
        
int CThostFtdcTraderApiReqQryTradingCode(CThostFtdcTraderApi * self , CThostFtdcQryTradingCodeField * pQryTradingCode,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryTradingCode(pQryTradingCode, nRequestID);
}
        
int CThostFtdcTraderApiReqQryInstrumentMarginRate(CThostFtdcTraderApi * self , CThostFtdcQryInstrumentMarginRateField * pQryInstrumentMarginRate,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryInstrumentMarginRate(pQryInstrumentMarginRate, nRequestID);
}
        
int CThostFtdcTraderApiReqQryInstrumentCommissionRate(CThostFtdcTraderApi * self , CThostFtdcQryInstrumentCommissionRateField * pQryInstrumentCommissionRate,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryInstrumentCommissionRate(pQryInstrumentCommissionRate, nRequestID);
}
        
int CThostFtdcTraderApiReqQryExchange(CThostFtdcTraderApi * self , CThostFtdcQryExchangeField * pQryExchange,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryExchange(pQryExchange, nRequestID);
}
        
int CThostFtdcTraderApiReqQryProduct(CThostFtdcTraderApi * self , CThostFtdcQryProductField * pQryProduct,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryProduct(pQryProduct, nRequestID);
}
        
int CThostFtdcTraderApiReqQryInstrument(CThostFtdcTraderApi * self , CThostFtdcQryInstrumentField * pQryInstrument,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryInstrument(pQryInstrument, nRequestID);
}
        
int CThostFtdcTraderApiReqQryDepthMarketData(CThostFtdcTraderApi * self , CThostFtdcQryDepthMarketDataField * pQryDepthMarketData,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryDepthMarketData(pQryDepthMarketData, nRequestID);
}
        
int CThostFtdcTraderApiReqQrySettlementInfo(CThostFtdcTraderApi * self , CThostFtdcQrySettlementInfoField * pQrySettlementInfo,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQrySettlementInfo(pQrySettlementInfo, nRequestID);
}
        
int CThostFtdcTraderApiReqQryTransferBank(CThostFtdcTraderApi * self , CThostFtdcQryTransferBankField * pQryTransferBank,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryTransferBank(pQryTransferBank, nRequestID);
}
        
int CThostFtdcTraderApiReqQryInvestorPositionDetail(CThostFtdcTraderApi * self , CThostFtdcQryInvestorPositionDetailField * pQryInvestorPositionDetail,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryInvestorPositionDetail(pQryInvestorPositionDetail, nRequestID);
}
        
int CThostFtdcTraderApiReqQryNotice(CThostFtdcTraderApi * self , CThostFtdcQryNoticeField * pQryNotice,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryNotice(pQryNotice, nRequestID);
}
        
int CThostFtdcTraderApiReqQrySettlementInfoConfirm(CThostFtdcTraderApi * self , CThostFtdcQrySettlementInfoConfirmField * pQrySettlementInfoConfirm,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQrySettlementInfoConfirm(pQrySettlementInfoConfirm, nRequestID);
}
        
int CThostFtdcTraderApiReqQryInvestorPositionCombineDetail(CThostFtdcTraderApi * self , CThostFtdcQryInvestorPositionCombineDetailField * pQryInvestorPositionCombineDetail,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryInvestorPositionCombineDetail(pQryInvestorPositionCombineDetail, nRequestID);
}
        
int CThostFtdcTraderApiReqQryCFMMCTradingAccountKey(CThostFtdcTraderApi * self , CThostFtdcQryCFMMCTradingAccountKeyField * pQryCFMMCTradingAccountKey,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryCFMMCTradingAccountKey(pQryCFMMCTradingAccountKey, nRequestID);
}
        
int CThostFtdcTraderApiReqQryEWarrantOffset(CThostFtdcTraderApi * self , CThostFtdcQryEWarrantOffsetField * pQryEWarrantOffset,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryEWarrantOffset(pQryEWarrantOffset, nRequestID);
}
        
int CThostFtdcTraderApiReqQryInvestorProductGroupMargin(CThostFtdcTraderApi * self , CThostFtdcQryInvestorProductGroupMarginField * pQryInvestorProductGroupMargin,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryInvestorProductGroupMargin(pQryInvestorProductGroupMargin, nRequestID);
}
        
int CThostFtdcTraderApiReqQryExchangeMarginRate(CThostFtdcTraderApi * self , CThostFtdcQryExchangeMarginRateField * pQryExchangeMarginRate,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryExchangeMarginRate(pQryExchangeMarginRate, nRequestID);
}
        
int CThostFtdcTraderApiReqQryExchangeMarginRateAdjust(CThostFtdcTraderApi * self , CThostFtdcQryExchangeMarginRateAdjustField * pQryExchangeMarginRateAdjust,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryExchangeMarginRateAdjust(pQryExchangeMarginRateAdjust, nRequestID);
}
        
int CThostFtdcTraderApiReqQryExchangeRate(CThostFtdcTraderApi * self , CThostFtdcQryExchangeRateField * pQryExchangeRate,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryExchangeRate(pQryExchangeRate, nRequestID);
}
        
int CThostFtdcTraderApiReqQrySecAgentACIDMap(CThostFtdcTraderApi * self , CThostFtdcQrySecAgentACIDMapField * pQrySecAgentACIDMap,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQrySecAgentACIDMap(pQrySecAgentACIDMap, nRequestID);
}
        
int CThostFtdcTraderApiReqQryProductExchRate(CThostFtdcTraderApi * self , CThostFtdcQryProductExchRateField * pQryProductExchRate,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryProductExchRate(pQryProductExchRate, nRequestID);
}
        
int CThostFtdcTraderApiReqQryProductGroup(CThostFtdcTraderApi * self , CThostFtdcQryProductGroupField * pQryProductGroup,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryProductGroup(pQryProductGroup, nRequestID);
}
        
int CThostFtdcTraderApiReqQryMMInstrumentCommissionRate(CThostFtdcTraderApi * self , CThostFtdcQryMMInstrumentCommissionRateField * pQryMMInstrumentCommissionRate,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryMMInstrumentCommissionRate(pQryMMInstrumentCommissionRate, nRequestID);
}
        
int CThostFtdcTraderApiReqQryMMOptionInstrCommRate(CThostFtdcTraderApi * self , CThostFtdcQryMMOptionInstrCommRateField * pQryMMOptionInstrCommRate,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryMMOptionInstrCommRate(pQryMMOptionInstrCommRate, nRequestID);
}
        
int CThostFtdcTraderApiReqQryInstrumentOrderCommRate(CThostFtdcTraderApi * self , CThostFtdcQryInstrumentOrderCommRateField * pQryInstrumentOrderCommRate,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryInstrumentOrderCommRate(pQryInstrumentOrderCommRate, nRequestID);
}
        
int CThostFtdcTraderApiReqQrySecAgentTradingAccount(CThostFtdcTraderApi * self , CThostFtdcQryTradingAccountField * pQryTradingAccount,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQrySecAgentTradingAccount(pQryTradingAccount, nRequestID);
}
        
int CThostFtdcTraderApiReqQrySecAgentCheckMode(CThostFtdcTraderApi * self , CThostFtdcQrySecAgentCheckModeField * pQrySecAgentCheckMode,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQrySecAgentCheckMode(pQrySecAgentCheckMode, nRequestID);
}
        
int CThostFtdcTraderApiReqQrySecAgentTradeInfo(CThostFtdcTraderApi * self , CThostFtdcQrySecAgentTradeInfoField * pQrySecAgentTradeInfo,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQrySecAgentTradeInfo(pQrySecAgentTradeInfo, nRequestID);
}
        
int CThostFtdcTraderApiReqQryOptionInstrTradeCost(CThostFtdcTraderApi * self , CThostFtdcQryOptionInstrTradeCostField * pQryOptionInstrTradeCost,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryOptionInstrTradeCost(pQryOptionInstrTradeCost, nRequestID);
}
        
int CThostFtdcTraderApiReqQryOptionInstrCommRate(CThostFtdcTraderApi * self , CThostFtdcQryOptionInstrCommRateField * pQryOptionInstrCommRate,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryOptionInstrCommRate(pQryOptionInstrCommRate, nRequestID);
}
        
int CThostFtdcTraderApiReqQryExecOrder(CThostFtdcTraderApi * self , CThostFtdcQryExecOrderField * pQryExecOrder,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryExecOrder(pQryExecOrder, nRequestID);
}
        
int CThostFtdcTraderApiReqQryForQuote(CThostFtdcTraderApi * self , CThostFtdcQryForQuoteField * pQryForQuote,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryForQuote(pQryForQuote, nRequestID);
}
        
int CThostFtdcTraderApiReqQryQuote(CThostFtdcTraderApi * self , CThostFtdcQryQuoteField * pQryQuote,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryQuote(pQryQuote, nRequestID);
}
        
int CThostFtdcTraderApiReqQryOptionSelfClose(CThostFtdcTraderApi * self , CThostFtdcQryOptionSelfCloseField * pQryOptionSelfClose,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryOptionSelfClose(pQryOptionSelfClose, nRequestID);
}
        
int CThostFtdcTraderApiReqQryInvestUnit(CThostFtdcTraderApi * self , CThostFtdcQryInvestUnitField * pQryInvestUnit,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryInvestUnit(pQryInvestUnit, nRequestID);
}
        
int CThostFtdcTraderApiReqQryCombInstrumentGuard(CThostFtdcTraderApi * self , CThostFtdcQryCombInstrumentGuardField * pQryCombInstrumentGuard,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryCombInstrumentGuard(pQryCombInstrumentGuard, nRequestID);
}
        
int CThostFtdcTraderApiReqQryCombAction(CThostFtdcTraderApi * self , CThostFtdcQryCombActionField * pQryCombAction,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryCombAction(pQryCombAction, nRequestID);
}
        
int CThostFtdcTraderApiReqQryTransferSerial(CThostFtdcTraderApi * self , CThostFtdcQryTransferSerialField * pQryTransferSerial,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryTransferSerial(pQryTransferSerial, nRequestID);
}
        
int CThostFtdcTraderApiReqQryAccountregister(CThostFtdcTraderApi * self , CThostFtdcQryAccountregisterField * pQryAccountregister,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryAccountregister(pQryAccountregister, nRequestID);
}
        
int CThostFtdcTraderApiReqQryContractBank(CThostFtdcTraderApi * self , CThostFtdcQryContractBankField * pQryContractBank,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryContractBank(pQryContractBank, nRequestID);
}
        
int CThostFtdcTraderApiReqQryParkedOrder(CThostFtdcTraderApi * self , CThostFtdcQryParkedOrderField * pQryParkedOrder,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryParkedOrder(pQryParkedOrder, nRequestID);
}
        
int CThostFtdcTraderApiReqQryParkedOrderAction(CThostFtdcTraderApi * self , CThostFtdcQryParkedOrderActionField * pQryParkedOrderAction,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryParkedOrderAction(pQryParkedOrderAction, nRequestID);
}
        
int CThostFtdcTraderApiReqQryTradingNotice(CThostFtdcTraderApi * self , CThostFtdcQryTradingNoticeField * pQryTradingNotice,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryTradingNotice(pQryTradingNotice, nRequestID);
}
        
int CThostFtdcTraderApiReqQryBrokerTradingParams(CThostFtdcTraderApi * self , CThostFtdcQryBrokerTradingParamsField * pQryBrokerTradingParams,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryBrokerTradingParams(pQryBrokerTradingParams, nRequestID);
}
        
int CThostFtdcTraderApiReqQryBrokerTradingAlgos(CThostFtdcTraderApi * self , CThostFtdcQryBrokerTradingAlgosField * pQryBrokerTradingAlgos,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryBrokerTradingAlgos(pQryBrokerTradingAlgos, nRequestID);
}
        
int CThostFtdcTraderApiReqQueryCFMMCTradingAccountToken(CThostFtdcTraderApi * self , CThostFtdcQueryCFMMCTradingAccountTokenField * pQueryCFMMCTradingAccountToken,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQueryCFMMCTradingAccountToken(pQueryCFMMCTradingAccountToken, nRequestID);
}
        
int CThostFtdcTraderApiReqFromBankToFutureByFuture(CThostFtdcTraderApi * self , CThostFtdcReqTransferField * pReqTransfer,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqFromBankToFutureByFuture(pReqTransfer, nRequestID);
}
        
int CThostFtdcTraderApiReqFromFutureToBankByFuture(CThostFtdcTraderApi * self , CThostFtdcReqTransferField * pReqTransfer,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqFromFutureToBankByFuture(pReqTransfer, nRequestID);
}
        
int CThostFtdcTraderApiReqQueryBankAccountMoneyByFuture(CThostFtdcTraderApi * self , CThostFtdcReqQueryAccountField * pReqQueryAccount,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQueryBankAccountMoneyByFuture(pReqQueryAccount, nRequestID);
}
        extern "C" void RustCtpActionTdOnFrontDisconnected(void *rust_call_object, int nReason); 
extern "C" void RustCtpActionTdOnHeartBeatWarning(void *rust_call_object, int nTimeLapse); 
extern "C" void RustCtpActionTdOnRspAuthenticate(void *rust_call_object, CThostFtdcRspAuthenticateField * pRspAuthenticateField,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspUserLogin(void *rust_call_object, CThostFtdcRspUserLoginField * pRspUserLogin,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspUserLogout(void *rust_call_object, CThostFtdcUserLogoutField * pUserLogout,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspUserPasswordUpdate(void *rust_call_object, CThostFtdcUserPasswordUpdateField * pUserPasswordUpdate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspTradingAccountPasswordUpdate(void *rust_call_object, CThostFtdcTradingAccountPasswordUpdateField * pTradingAccountPasswordUpdate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspUserAuthMethod(void *rust_call_object, CThostFtdcRspUserAuthMethodField * pRspUserAuthMethod,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspGenUserCaptcha(void *rust_call_object, CThostFtdcRspGenUserCaptchaField * pRspGenUserCaptcha,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspGenUserText(void *rust_call_object, CThostFtdcRspGenUserTextField * pRspGenUserText,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspOrderInsert(void *rust_call_object, CThostFtdcInputOrderField * pInputOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspParkedOrderInsert(void *rust_call_object, CThostFtdcParkedOrderField * pParkedOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspParkedOrderAction(void *rust_call_object, CThostFtdcParkedOrderActionField * pParkedOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspOrderAction(void *rust_call_object, CThostFtdcInputOrderActionField * pInputOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQueryMaxOrderVolume(void *rust_call_object, CThostFtdcQueryMaxOrderVolumeField * pQueryMaxOrderVolume,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspSettlementInfoConfirm(void *rust_call_object, CThostFtdcSettlementInfoConfirmField * pSettlementInfoConfirm,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspRemoveParkedOrder(void *rust_call_object, CThostFtdcRemoveParkedOrderField * pRemoveParkedOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspRemoveParkedOrderAction(void *rust_call_object, CThostFtdcRemoveParkedOrderActionField * pRemoveParkedOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspExecOrderInsert(void *rust_call_object, CThostFtdcInputExecOrderField * pInputExecOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspExecOrderAction(void *rust_call_object, CThostFtdcInputExecOrderActionField * pInputExecOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspForQuoteInsert(void *rust_call_object, CThostFtdcInputForQuoteField * pInputForQuote,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQuoteInsert(void *rust_call_object, CThostFtdcInputQuoteField * pInputQuote,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQuoteAction(void *rust_call_object, CThostFtdcInputQuoteActionField * pInputQuoteAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspBatchOrderAction(void *rust_call_object, CThostFtdcInputBatchOrderActionField * pInputBatchOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspOptionSelfCloseInsert(void *rust_call_object, CThostFtdcInputOptionSelfCloseField * pInputOptionSelfClose,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspOptionSelfCloseAction(void *rust_call_object, CThostFtdcInputOptionSelfCloseActionField * pInputOptionSelfCloseAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspCombActionInsert(void *rust_call_object, CThostFtdcInputCombActionField * pInputCombAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryOrder(void *rust_call_object, CThostFtdcOrderField * pOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryTrade(void *rust_call_object, CThostFtdcTradeField * pTrade,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryInvestorPosition(void *rust_call_object, CThostFtdcInvestorPositionField * pInvestorPosition,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryTradingAccount(void *rust_call_object, CThostFtdcTradingAccountField * pTradingAccount,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryInvestor(void *rust_call_object, CThostFtdcInvestorField * pInvestor,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryTradingCode(void *rust_call_object, CThostFtdcTradingCodeField * pTradingCode,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryInstrumentMarginRate(void *rust_call_object, CThostFtdcInstrumentMarginRateField * pInstrumentMarginRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryInstrumentCommissionRate(void *rust_call_object, CThostFtdcInstrumentCommissionRateField * pInstrumentCommissionRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryExchange(void *rust_call_object, CThostFtdcExchangeField * pExchange,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryProduct(void *rust_call_object, CThostFtdcProductField * pProduct,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryInstrument(void *rust_call_object, CThostFtdcInstrumentField * pInstrument,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryDepthMarketData(void *rust_call_object, CThostFtdcDepthMarketDataField * pDepthMarketData,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQrySettlementInfo(void *rust_call_object, CThostFtdcSettlementInfoField * pSettlementInfo,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryTransferBank(void *rust_call_object, CThostFtdcTransferBankField * pTransferBank,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryInvestorPositionDetail(void *rust_call_object, CThostFtdcInvestorPositionDetailField * pInvestorPositionDetail,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryNotice(void *rust_call_object, CThostFtdcNoticeField * pNotice,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQrySettlementInfoConfirm(void *rust_call_object, CThostFtdcSettlementInfoConfirmField * pSettlementInfoConfirm,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryInvestorPositionCombineDetail(void *rust_call_object, CThostFtdcInvestorPositionCombineDetailField * pInvestorPositionCombineDetail,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryCFMMCTradingAccountKey(void *rust_call_object, CThostFtdcCFMMCTradingAccountKeyField * pCFMMCTradingAccountKey,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryEWarrantOffset(void *rust_call_object, CThostFtdcEWarrantOffsetField * pEWarrantOffset,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryInvestorProductGroupMargin(void *rust_call_object, CThostFtdcInvestorProductGroupMarginField * pInvestorProductGroupMargin,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryExchangeMarginRate(void *rust_call_object, CThostFtdcExchangeMarginRateField * pExchangeMarginRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryExchangeMarginRateAdjust(void *rust_call_object, CThostFtdcExchangeMarginRateAdjustField * pExchangeMarginRateAdjust,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryExchangeRate(void *rust_call_object, CThostFtdcExchangeRateField * pExchangeRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQrySecAgentACIDMap(void *rust_call_object, CThostFtdcSecAgentACIDMapField * pSecAgentACIDMap,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryProductExchRate(void *rust_call_object, CThostFtdcProductExchRateField * pProductExchRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryProductGroup(void *rust_call_object, CThostFtdcProductGroupField * pProductGroup,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryMMInstrumentCommissionRate(void *rust_call_object, CThostFtdcMMInstrumentCommissionRateField * pMMInstrumentCommissionRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryMMOptionInstrCommRate(void *rust_call_object, CThostFtdcMMOptionInstrCommRateField * pMMOptionInstrCommRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryInstrumentOrderCommRate(void *rust_call_object, CThostFtdcInstrumentOrderCommRateField * pInstrumentOrderCommRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQrySecAgentTradingAccount(void *rust_call_object, CThostFtdcTradingAccountField * pTradingAccount,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQrySecAgentCheckMode(void *rust_call_object, CThostFtdcSecAgentCheckModeField * pSecAgentCheckMode,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQrySecAgentTradeInfo(void *rust_call_object, CThostFtdcSecAgentTradeInfoField * pSecAgentTradeInfo,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryOptionInstrTradeCost(void *rust_call_object, CThostFtdcOptionInstrTradeCostField * pOptionInstrTradeCost,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryOptionInstrCommRate(void *rust_call_object, CThostFtdcOptionInstrCommRateField * pOptionInstrCommRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryExecOrder(void *rust_call_object, CThostFtdcExecOrderField * pExecOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryForQuote(void *rust_call_object, CThostFtdcForQuoteField * pForQuote,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryQuote(void *rust_call_object, CThostFtdcQuoteField * pQuote,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryOptionSelfClose(void *rust_call_object, CThostFtdcOptionSelfCloseField * pOptionSelfClose,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryInvestUnit(void *rust_call_object, CThostFtdcInvestUnitField * pInvestUnit,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryCombInstrumentGuard(void *rust_call_object, CThostFtdcCombInstrumentGuardField * pCombInstrumentGuard,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryCombAction(void *rust_call_object, CThostFtdcCombActionField * pCombAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryTransferSerial(void *rust_call_object, CThostFtdcTransferSerialField * pTransferSerial,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryAccountregister(void *rust_call_object, CThostFtdcAccountregisterField * pAccountregister,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspError(void *rust_call_object, CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRtnOrder(void *rust_call_object, CThostFtdcOrderField * pOrder); 
extern "C" void RustCtpActionTdOnRtnTrade(void *rust_call_object, CThostFtdcTradeField * pTrade); 
extern "C" void RustCtpActionTdOnErrRtnOrderInsert(void *rust_call_object, CThostFtdcInputOrderField * pInputOrder,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpActionTdOnErrRtnOrderAction(void *rust_call_object, CThostFtdcOrderActionField * pOrderAction,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpActionTdOnRtnInstrumentStatus(void *rust_call_object, CThostFtdcInstrumentStatusField * pInstrumentStatus); 
extern "C" void RustCtpActionTdOnRtnBulletin(void *rust_call_object, CThostFtdcBulletinField * pBulletin); 
extern "C" void RustCtpActionTdOnRtnTradingNotice(void *rust_call_object, CThostFtdcTradingNoticeInfoField * pTradingNoticeInfo); 
extern "C" void RustCtpActionTdOnRtnErrorConditionalOrder(void *rust_call_object, CThostFtdcErrorConditionalOrderField * pErrorConditionalOrder); 
extern "C" void RustCtpActionTdOnRtnExecOrder(void *rust_call_object, CThostFtdcExecOrderField * pExecOrder); 
extern "C" void RustCtpActionTdOnErrRtnExecOrderInsert(void *rust_call_object, CThostFtdcInputExecOrderField * pInputExecOrder,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpActionTdOnErrRtnExecOrderAction(void *rust_call_object, CThostFtdcExecOrderActionField * pExecOrderAction,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpActionTdOnErrRtnForQuoteInsert(void *rust_call_object, CThostFtdcInputForQuoteField * pInputForQuote,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpActionTdOnRtnQuote(void *rust_call_object, CThostFtdcQuoteField * pQuote); 
extern "C" void RustCtpActionTdOnErrRtnQuoteInsert(void *rust_call_object, CThostFtdcInputQuoteField * pInputQuote,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpActionTdOnErrRtnQuoteAction(void *rust_call_object, CThostFtdcQuoteActionField * pQuoteAction,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpActionTdOnRtnForQuoteRsp(void *rust_call_object, CThostFtdcForQuoteRspField * pForQuoteRsp); 
extern "C" void RustCtpActionTdOnRtnCFMMCTradingAccountToken(void *rust_call_object, CThostFtdcCFMMCTradingAccountTokenField * pCFMMCTradingAccountToken); 
extern "C" void RustCtpActionTdOnErrRtnBatchOrderAction(void *rust_call_object, CThostFtdcBatchOrderActionField * pBatchOrderAction,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpActionTdOnRtnOptionSelfClose(void *rust_call_object, CThostFtdcOptionSelfCloseField * pOptionSelfClose); 
extern "C" void RustCtpActionTdOnErrRtnOptionSelfCloseInsert(void *rust_call_object, CThostFtdcInputOptionSelfCloseField * pInputOptionSelfClose,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpActionTdOnErrRtnOptionSelfCloseAction(void *rust_call_object, CThostFtdcOptionSelfCloseActionField * pOptionSelfCloseAction,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpActionTdOnRtnCombAction(void *rust_call_object, CThostFtdcCombActionField * pCombAction); 
extern "C" void RustCtpActionTdOnErrRtnCombActionInsert(void *rust_call_object, CThostFtdcInputCombActionField * pInputCombAction,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpActionTdOnRspQryContractBank(void *rust_call_object, CThostFtdcContractBankField * pContractBank,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryParkedOrder(void *rust_call_object, CThostFtdcParkedOrderField * pParkedOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryParkedOrderAction(void *rust_call_object, CThostFtdcParkedOrderActionField * pParkedOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryTradingNotice(void *rust_call_object, CThostFtdcTradingNoticeField * pTradingNotice,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryBrokerTradingParams(void *rust_call_object, CThostFtdcBrokerTradingParamsField * pBrokerTradingParams,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQryBrokerTradingAlgos(void *rust_call_object, CThostFtdcBrokerTradingAlgosField * pBrokerTradingAlgos,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQueryCFMMCTradingAccountToken(void *rust_call_object, CThostFtdcQueryCFMMCTradingAccountTokenField * pQueryCFMMCTradingAccountToken,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRtnFromBankToFutureByBank(void *rust_call_object, CThostFtdcRspTransferField * pRspTransfer); 
extern "C" void RustCtpActionTdOnRtnFromFutureToBankByBank(void *rust_call_object, CThostFtdcRspTransferField * pRspTransfer); 
extern "C" void RustCtpActionTdOnRtnRepealFromBankToFutureByBank(void *rust_call_object, CThostFtdcRspRepealField * pRspRepeal); 
extern "C" void RustCtpActionTdOnRtnRepealFromFutureToBankByBank(void *rust_call_object, CThostFtdcRspRepealField * pRspRepeal); 
extern "C" void RustCtpActionTdOnRtnFromBankToFutureByFuture(void *rust_call_object, CThostFtdcRspTransferField * pRspTransfer); 
extern "C" void RustCtpActionTdOnRtnFromFutureToBankByFuture(void *rust_call_object, CThostFtdcRspTransferField * pRspTransfer); 
extern "C" void RustCtpActionTdOnRtnRepealFromBankToFutureByFutureManual(void *rust_call_object, CThostFtdcRspRepealField * pRspRepeal); 
extern "C" void RustCtpActionTdOnRtnRepealFromFutureToBankByFutureManual(void *rust_call_object, CThostFtdcRspRepealField * pRspRepeal); 
extern "C" void RustCtpActionTdOnRtnQueryBankBalanceByFuture(void *rust_call_object, CThostFtdcNotifyQueryAccountField * pNotifyQueryAccount); 
extern "C" void RustCtpActionTdOnErrRtnBankToFutureByFuture(void *rust_call_object, CThostFtdcReqTransferField * pReqTransfer,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpActionTdOnErrRtnFutureToBankByFuture(void *rust_call_object, CThostFtdcReqTransferField * pReqTransfer,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpActionTdOnErrRtnRepealBankToFutureByFutureManual(void *rust_call_object, CThostFtdcReqRepealField * pReqRepeal,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpActionTdOnErrRtnRepealFutureToBankByFutureManual(void *rust_call_object, CThostFtdcReqRepealField * pReqRepeal,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpActionTdOnErrRtnQueryBankBalanceByFuture(void *rust_call_object, CThostFtdcReqQueryAccountField * pReqQueryAccount,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpActionTdOnRtnRepealFromBankToFutureByFuture(void *rust_call_object, CThostFtdcRspRepealField * pRspRepeal); 
extern "C" void RustCtpActionTdOnRtnRepealFromFutureToBankByFuture(void *rust_call_object, CThostFtdcRspRepealField * pRspRepeal); 
extern "C" void RustCtpActionTdOnRspFromBankToFutureByFuture(void *rust_call_object, CThostFtdcReqTransferField * pReqTransfer,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspFromFutureToBankByFuture(void *rust_call_object, CThostFtdcReqTransferField * pReqTransfer,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRspQueryBankAccountMoneyByFuture(void *rust_call_object, CThostFtdcReqQueryAccountField * pReqQueryAccount,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpActionTdOnRtnOpenAccountByBank(void *rust_call_object, CThostFtdcOpenAccountField * pOpenAccount); 
extern "C" void RustCtpActionTdOnRtnCancelAccountByBank(void *rust_call_object, CThostFtdcCancelAccountField * pCancelAccount); 
extern "C" void RustCtpActionTdOnRtnChangeAccountByBank(void *rust_call_object, CThostFtdcChangeAccountField * pChangeAccount); 

class CtpTdSpi: CThostFtdcTraderSpi {
public:
    void *rust_call_object;

//    CtpTdSpi(void *rust_call_object) rust_call_object(rust_call_object) {};
    CtpTdSpi(void *rust_call_object);
            
     void OnFrontDisconnected(int nReason){
          RustCtpActionTdOnFrontDisconnected(this->rust_call_object, nReason);            
     }
            
     void OnHeartBeatWarning(int nTimeLapse){
          RustCtpActionTdOnHeartBeatWarning(this->rust_call_object, nTimeLapse);            
     }
            
     void OnRspAuthenticate(CThostFtdcRspAuthenticateField * pRspAuthenticateField,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspAuthenticate(this->rust_call_object, pRspAuthenticateField, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspUserLogin(CThostFtdcRspUserLoginField * pRspUserLogin,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspUserLogin(this->rust_call_object, pRspUserLogin, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspUserLogout(CThostFtdcUserLogoutField * pUserLogout,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspUserLogout(this->rust_call_object, pUserLogout, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspUserPasswordUpdate(CThostFtdcUserPasswordUpdateField * pUserPasswordUpdate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspUserPasswordUpdate(this->rust_call_object, pUserPasswordUpdate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspTradingAccountPasswordUpdate(CThostFtdcTradingAccountPasswordUpdateField * pTradingAccountPasswordUpdate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspTradingAccountPasswordUpdate(this->rust_call_object, pTradingAccountPasswordUpdate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspUserAuthMethod(CThostFtdcRspUserAuthMethodField * pRspUserAuthMethod,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspUserAuthMethod(this->rust_call_object, pRspUserAuthMethod, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspGenUserCaptcha(CThostFtdcRspGenUserCaptchaField * pRspGenUserCaptcha,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspGenUserCaptcha(this->rust_call_object, pRspGenUserCaptcha, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspGenUserText(CThostFtdcRspGenUserTextField * pRspGenUserText,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspGenUserText(this->rust_call_object, pRspGenUserText, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspOrderInsert(CThostFtdcInputOrderField * pInputOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspOrderInsert(this->rust_call_object, pInputOrder, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspParkedOrderInsert(CThostFtdcParkedOrderField * pParkedOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspParkedOrderInsert(this->rust_call_object, pParkedOrder, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspParkedOrderAction(CThostFtdcParkedOrderActionField * pParkedOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspParkedOrderAction(this->rust_call_object, pParkedOrderAction, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspOrderAction(CThostFtdcInputOrderActionField * pInputOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspOrderAction(this->rust_call_object, pInputOrderAction, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQueryMaxOrderVolume(CThostFtdcQueryMaxOrderVolumeField * pQueryMaxOrderVolume,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQueryMaxOrderVolume(this->rust_call_object, pQueryMaxOrderVolume, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspSettlementInfoConfirm(CThostFtdcSettlementInfoConfirmField * pSettlementInfoConfirm,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspSettlementInfoConfirm(this->rust_call_object, pSettlementInfoConfirm, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspRemoveParkedOrder(CThostFtdcRemoveParkedOrderField * pRemoveParkedOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspRemoveParkedOrder(this->rust_call_object, pRemoveParkedOrder, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspRemoveParkedOrderAction(CThostFtdcRemoveParkedOrderActionField * pRemoveParkedOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspRemoveParkedOrderAction(this->rust_call_object, pRemoveParkedOrderAction, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspExecOrderInsert(CThostFtdcInputExecOrderField * pInputExecOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspExecOrderInsert(this->rust_call_object, pInputExecOrder, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspExecOrderAction(CThostFtdcInputExecOrderActionField * pInputExecOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspExecOrderAction(this->rust_call_object, pInputExecOrderAction, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspForQuoteInsert(CThostFtdcInputForQuoteField * pInputForQuote,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspForQuoteInsert(this->rust_call_object, pInputForQuote, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQuoteInsert(CThostFtdcInputQuoteField * pInputQuote,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQuoteInsert(this->rust_call_object, pInputQuote, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQuoteAction(CThostFtdcInputQuoteActionField * pInputQuoteAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQuoteAction(this->rust_call_object, pInputQuoteAction, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspBatchOrderAction(CThostFtdcInputBatchOrderActionField * pInputBatchOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspBatchOrderAction(this->rust_call_object, pInputBatchOrderAction, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspOptionSelfCloseInsert(CThostFtdcInputOptionSelfCloseField * pInputOptionSelfClose,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspOptionSelfCloseInsert(this->rust_call_object, pInputOptionSelfClose, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspOptionSelfCloseAction(CThostFtdcInputOptionSelfCloseActionField * pInputOptionSelfCloseAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspOptionSelfCloseAction(this->rust_call_object, pInputOptionSelfCloseAction, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspCombActionInsert(CThostFtdcInputCombActionField * pInputCombAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspCombActionInsert(this->rust_call_object, pInputCombAction, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryOrder(CThostFtdcOrderField * pOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryOrder(this->rust_call_object, pOrder, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryTrade(CThostFtdcTradeField * pTrade,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryTrade(this->rust_call_object, pTrade, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInvestorPosition(CThostFtdcInvestorPositionField * pInvestorPosition,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryInvestorPosition(this->rust_call_object, pInvestorPosition, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryTradingAccount(CThostFtdcTradingAccountField * pTradingAccount,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryTradingAccount(this->rust_call_object, pTradingAccount, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInvestor(CThostFtdcInvestorField * pInvestor,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryInvestor(this->rust_call_object, pInvestor, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryTradingCode(CThostFtdcTradingCodeField * pTradingCode,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryTradingCode(this->rust_call_object, pTradingCode, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInstrumentMarginRate(CThostFtdcInstrumentMarginRateField * pInstrumentMarginRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryInstrumentMarginRate(this->rust_call_object, pInstrumentMarginRate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInstrumentCommissionRate(CThostFtdcInstrumentCommissionRateField * pInstrumentCommissionRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryInstrumentCommissionRate(this->rust_call_object, pInstrumentCommissionRate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryExchange(CThostFtdcExchangeField * pExchange,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryExchange(this->rust_call_object, pExchange, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryProduct(CThostFtdcProductField * pProduct,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryProduct(this->rust_call_object, pProduct, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInstrument(CThostFtdcInstrumentField * pInstrument,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryInstrument(this->rust_call_object, pInstrument, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryDepthMarketData(CThostFtdcDepthMarketDataField * pDepthMarketData,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryDepthMarketData(this->rust_call_object, pDepthMarketData, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQrySettlementInfo(CThostFtdcSettlementInfoField * pSettlementInfo,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQrySettlementInfo(this->rust_call_object, pSettlementInfo, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryTransferBank(CThostFtdcTransferBankField * pTransferBank,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryTransferBank(this->rust_call_object, pTransferBank, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInvestorPositionDetail(CThostFtdcInvestorPositionDetailField * pInvestorPositionDetail,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryInvestorPositionDetail(this->rust_call_object, pInvestorPositionDetail, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryNotice(CThostFtdcNoticeField * pNotice,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryNotice(this->rust_call_object, pNotice, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQrySettlementInfoConfirm(CThostFtdcSettlementInfoConfirmField * pSettlementInfoConfirm,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQrySettlementInfoConfirm(this->rust_call_object, pSettlementInfoConfirm, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInvestorPositionCombineDetail(CThostFtdcInvestorPositionCombineDetailField * pInvestorPositionCombineDetail,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryInvestorPositionCombineDetail(this->rust_call_object, pInvestorPositionCombineDetail, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryCFMMCTradingAccountKey(CThostFtdcCFMMCTradingAccountKeyField * pCFMMCTradingAccountKey,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryCFMMCTradingAccountKey(this->rust_call_object, pCFMMCTradingAccountKey, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryEWarrantOffset(CThostFtdcEWarrantOffsetField * pEWarrantOffset,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryEWarrantOffset(this->rust_call_object, pEWarrantOffset, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInvestorProductGroupMargin(CThostFtdcInvestorProductGroupMarginField * pInvestorProductGroupMargin,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryInvestorProductGroupMargin(this->rust_call_object, pInvestorProductGroupMargin, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryExchangeMarginRate(CThostFtdcExchangeMarginRateField * pExchangeMarginRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryExchangeMarginRate(this->rust_call_object, pExchangeMarginRate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryExchangeMarginRateAdjust(CThostFtdcExchangeMarginRateAdjustField * pExchangeMarginRateAdjust,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryExchangeMarginRateAdjust(this->rust_call_object, pExchangeMarginRateAdjust, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryExchangeRate(CThostFtdcExchangeRateField * pExchangeRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryExchangeRate(this->rust_call_object, pExchangeRate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQrySecAgentACIDMap(CThostFtdcSecAgentACIDMapField * pSecAgentACIDMap,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQrySecAgentACIDMap(this->rust_call_object, pSecAgentACIDMap, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryProductExchRate(CThostFtdcProductExchRateField * pProductExchRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryProductExchRate(this->rust_call_object, pProductExchRate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryProductGroup(CThostFtdcProductGroupField * pProductGroup,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryProductGroup(this->rust_call_object, pProductGroup, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryMMInstrumentCommissionRate(CThostFtdcMMInstrumentCommissionRateField * pMMInstrumentCommissionRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryMMInstrumentCommissionRate(this->rust_call_object, pMMInstrumentCommissionRate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryMMOptionInstrCommRate(CThostFtdcMMOptionInstrCommRateField * pMMOptionInstrCommRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryMMOptionInstrCommRate(this->rust_call_object, pMMOptionInstrCommRate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInstrumentOrderCommRate(CThostFtdcInstrumentOrderCommRateField * pInstrumentOrderCommRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryInstrumentOrderCommRate(this->rust_call_object, pInstrumentOrderCommRate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQrySecAgentTradingAccount(CThostFtdcTradingAccountField * pTradingAccount,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQrySecAgentTradingAccount(this->rust_call_object, pTradingAccount, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQrySecAgentCheckMode(CThostFtdcSecAgentCheckModeField * pSecAgentCheckMode,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQrySecAgentCheckMode(this->rust_call_object, pSecAgentCheckMode, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQrySecAgentTradeInfo(CThostFtdcSecAgentTradeInfoField * pSecAgentTradeInfo,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQrySecAgentTradeInfo(this->rust_call_object, pSecAgentTradeInfo, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryOptionInstrTradeCost(CThostFtdcOptionInstrTradeCostField * pOptionInstrTradeCost,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryOptionInstrTradeCost(this->rust_call_object, pOptionInstrTradeCost, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryOptionInstrCommRate(CThostFtdcOptionInstrCommRateField * pOptionInstrCommRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryOptionInstrCommRate(this->rust_call_object, pOptionInstrCommRate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryExecOrder(CThostFtdcExecOrderField * pExecOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryExecOrder(this->rust_call_object, pExecOrder, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryForQuote(CThostFtdcForQuoteField * pForQuote,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryForQuote(this->rust_call_object, pForQuote, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryQuote(CThostFtdcQuoteField * pQuote,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryQuote(this->rust_call_object, pQuote, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryOptionSelfClose(CThostFtdcOptionSelfCloseField * pOptionSelfClose,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryOptionSelfClose(this->rust_call_object, pOptionSelfClose, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInvestUnit(CThostFtdcInvestUnitField * pInvestUnit,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryInvestUnit(this->rust_call_object, pInvestUnit, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryCombInstrumentGuard(CThostFtdcCombInstrumentGuardField * pCombInstrumentGuard,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryCombInstrumentGuard(this->rust_call_object, pCombInstrumentGuard, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryCombAction(CThostFtdcCombActionField * pCombAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryCombAction(this->rust_call_object, pCombAction, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryTransferSerial(CThostFtdcTransferSerialField * pTransferSerial,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryTransferSerial(this->rust_call_object, pTransferSerial, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryAccountregister(CThostFtdcAccountregisterField * pAccountregister,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryAccountregister(this->rust_call_object, pAccountregister, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspError(CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspError(this->rust_call_object, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRtnOrder(CThostFtdcOrderField * pOrder){
          RustCtpActionTdOnRtnOrder(this->rust_call_object, pOrder);            
     }
            
     void OnRtnTrade(CThostFtdcTradeField * pTrade){
          RustCtpActionTdOnRtnTrade(this->rust_call_object, pTrade);            
     }
            
     void OnErrRtnOrderInsert(CThostFtdcInputOrderField * pInputOrder,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionTdOnErrRtnOrderInsert(this->rust_call_object, pInputOrder, pRspInfo);            
     }
            
     void OnErrRtnOrderAction(CThostFtdcOrderActionField * pOrderAction,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionTdOnErrRtnOrderAction(this->rust_call_object, pOrderAction, pRspInfo);            
     }
            
     void OnRtnInstrumentStatus(CThostFtdcInstrumentStatusField * pInstrumentStatus){
          RustCtpActionTdOnRtnInstrumentStatus(this->rust_call_object, pInstrumentStatus);            
     }
            
     void OnRtnBulletin(CThostFtdcBulletinField * pBulletin){
          RustCtpActionTdOnRtnBulletin(this->rust_call_object, pBulletin);            
     }
            
     void OnRtnTradingNotice(CThostFtdcTradingNoticeInfoField * pTradingNoticeInfo){
          RustCtpActionTdOnRtnTradingNotice(this->rust_call_object, pTradingNoticeInfo);            
     }
            
     void OnRtnErrorConditionalOrder(CThostFtdcErrorConditionalOrderField * pErrorConditionalOrder){
          RustCtpActionTdOnRtnErrorConditionalOrder(this->rust_call_object, pErrorConditionalOrder);            
     }
            
     void OnRtnExecOrder(CThostFtdcExecOrderField * pExecOrder){
          RustCtpActionTdOnRtnExecOrder(this->rust_call_object, pExecOrder);            
     }
            
     void OnErrRtnExecOrderInsert(CThostFtdcInputExecOrderField * pInputExecOrder,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionTdOnErrRtnExecOrderInsert(this->rust_call_object, pInputExecOrder, pRspInfo);            
     }
            
     void OnErrRtnExecOrderAction(CThostFtdcExecOrderActionField * pExecOrderAction,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionTdOnErrRtnExecOrderAction(this->rust_call_object, pExecOrderAction, pRspInfo);            
     }
            
     void OnErrRtnForQuoteInsert(CThostFtdcInputForQuoteField * pInputForQuote,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionTdOnErrRtnForQuoteInsert(this->rust_call_object, pInputForQuote, pRspInfo);            
     }
            
     void OnRtnQuote(CThostFtdcQuoteField * pQuote){
          RustCtpActionTdOnRtnQuote(this->rust_call_object, pQuote);            
     }
            
     void OnErrRtnQuoteInsert(CThostFtdcInputQuoteField * pInputQuote,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionTdOnErrRtnQuoteInsert(this->rust_call_object, pInputQuote, pRspInfo);            
     }
            
     void OnErrRtnQuoteAction(CThostFtdcQuoteActionField * pQuoteAction,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionTdOnErrRtnQuoteAction(this->rust_call_object, pQuoteAction, pRspInfo);            
     }
            
     void OnRtnForQuoteRsp(CThostFtdcForQuoteRspField * pForQuoteRsp){
          RustCtpActionTdOnRtnForQuoteRsp(this->rust_call_object, pForQuoteRsp);            
     }
            
     void OnRtnCFMMCTradingAccountToken(CThostFtdcCFMMCTradingAccountTokenField * pCFMMCTradingAccountToken){
          RustCtpActionTdOnRtnCFMMCTradingAccountToken(this->rust_call_object, pCFMMCTradingAccountToken);            
     }
            
     void OnErrRtnBatchOrderAction(CThostFtdcBatchOrderActionField * pBatchOrderAction,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionTdOnErrRtnBatchOrderAction(this->rust_call_object, pBatchOrderAction, pRspInfo);            
     }
            
     void OnRtnOptionSelfClose(CThostFtdcOptionSelfCloseField * pOptionSelfClose){
          RustCtpActionTdOnRtnOptionSelfClose(this->rust_call_object, pOptionSelfClose);            
     }
            
     void OnErrRtnOptionSelfCloseInsert(CThostFtdcInputOptionSelfCloseField * pInputOptionSelfClose,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionTdOnErrRtnOptionSelfCloseInsert(this->rust_call_object, pInputOptionSelfClose, pRspInfo);            
     }
            
     void OnErrRtnOptionSelfCloseAction(CThostFtdcOptionSelfCloseActionField * pOptionSelfCloseAction,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionTdOnErrRtnOptionSelfCloseAction(this->rust_call_object, pOptionSelfCloseAction, pRspInfo);            
     }
            
     void OnRtnCombAction(CThostFtdcCombActionField * pCombAction){
          RustCtpActionTdOnRtnCombAction(this->rust_call_object, pCombAction);            
     }
            
     void OnErrRtnCombActionInsert(CThostFtdcInputCombActionField * pInputCombAction,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionTdOnErrRtnCombActionInsert(this->rust_call_object, pInputCombAction, pRspInfo);            
     }
            
     void OnRspQryContractBank(CThostFtdcContractBankField * pContractBank,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryContractBank(this->rust_call_object, pContractBank, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryParkedOrder(CThostFtdcParkedOrderField * pParkedOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryParkedOrder(this->rust_call_object, pParkedOrder, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryParkedOrderAction(CThostFtdcParkedOrderActionField * pParkedOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryParkedOrderAction(this->rust_call_object, pParkedOrderAction, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryTradingNotice(CThostFtdcTradingNoticeField * pTradingNotice,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryTradingNotice(this->rust_call_object, pTradingNotice, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryBrokerTradingParams(CThostFtdcBrokerTradingParamsField * pBrokerTradingParams,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryBrokerTradingParams(this->rust_call_object, pBrokerTradingParams, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryBrokerTradingAlgos(CThostFtdcBrokerTradingAlgosField * pBrokerTradingAlgos,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQryBrokerTradingAlgos(this->rust_call_object, pBrokerTradingAlgos, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQueryCFMMCTradingAccountToken(CThostFtdcQueryCFMMCTradingAccountTokenField * pQueryCFMMCTradingAccountToken,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQueryCFMMCTradingAccountToken(this->rust_call_object, pQueryCFMMCTradingAccountToken, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRtnFromBankToFutureByBank(CThostFtdcRspTransferField * pRspTransfer){
          RustCtpActionTdOnRtnFromBankToFutureByBank(this->rust_call_object, pRspTransfer);            
     }
            
     void OnRtnFromFutureToBankByBank(CThostFtdcRspTransferField * pRspTransfer){
          RustCtpActionTdOnRtnFromFutureToBankByBank(this->rust_call_object, pRspTransfer);            
     }
            
     void OnRtnRepealFromBankToFutureByBank(CThostFtdcRspRepealField * pRspRepeal){
          RustCtpActionTdOnRtnRepealFromBankToFutureByBank(this->rust_call_object, pRspRepeal);            
     }
            
     void OnRtnRepealFromFutureToBankByBank(CThostFtdcRspRepealField * pRspRepeal){
          RustCtpActionTdOnRtnRepealFromFutureToBankByBank(this->rust_call_object, pRspRepeal);            
     }
            
     void OnRtnFromBankToFutureByFuture(CThostFtdcRspTransferField * pRspTransfer){
          RustCtpActionTdOnRtnFromBankToFutureByFuture(this->rust_call_object, pRspTransfer);            
     }
            
     void OnRtnFromFutureToBankByFuture(CThostFtdcRspTransferField * pRspTransfer){
          RustCtpActionTdOnRtnFromFutureToBankByFuture(this->rust_call_object, pRspTransfer);            
     }
            
     void OnRtnRepealFromBankToFutureByFutureManual(CThostFtdcRspRepealField * pRspRepeal){
          RustCtpActionTdOnRtnRepealFromBankToFutureByFutureManual(this->rust_call_object, pRspRepeal);            
     }
            
     void OnRtnRepealFromFutureToBankByFutureManual(CThostFtdcRspRepealField * pRspRepeal){
          RustCtpActionTdOnRtnRepealFromFutureToBankByFutureManual(this->rust_call_object, pRspRepeal);            
     }
            
     void OnRtnQueryBankBalanceByFuture(CThostFtdcNotifyQueryAccountField * pNotifyQueryAccount){
          RustCtpActionTdOnRtnQueryBankBalanceByFuture(this->rust_call_object, pNotifyQueryAccount);            
     }
            
     void OnErrRtnBankToFutureByFuture(CThostFtdcReqTransferField * pReqTransfer,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionTdOnErrRtnBankToFutureByFuture(this->rust_call_object, pReqTransfer, pRspInfo);            
     }
            
     void OnErrRtnFutureToBankByFuture(CThostFtdcReqTransferField * pReqTransfer,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionTdOnErrRtnFutureToBankByFuture(this->rust_call_object, pReqTransfer, pRspInfo);            
     }
            
     void OnErrRtnRepealBankToFutureByFutureManual(CThostFtdcReqRepealField * pReqRepeal,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionTdOnErrRtnRepealBankToFutureByFutureManual(this->rust_call_object, pReqRepeal, pRspInfo);            
     }
            
     void OnErrRtnRepealFutureToBankByFutureManual(CThostFtdcReqRepealField * pReqRepeal,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionTdOnErrRtnRepealFutureToBankByFutureManual(this->rust_call_object, pReqRepeal, pRspInfo);            
     }
            
     void OnErrRtnQueryBankBalanceByFuture(CThostFtdcReqQueryAccountField * pReqQueryAccount,CThostFtdcRspInfoField * pRspInfo){
          RustCtpActionTdOnErrRtnQueryBankBalanceByFuture(this->rust_call_object, pReqQueryAccount, pRspInfo);            
     }
            
     void OnRtnRepealFromBankToFutureByFuture(CThostFtdcRspRepealField * pRspRepeal){
          RustCtpActionTdOnRtnRepealFromBankToFutureByFuture(this->rust_call_object, pRspRepeal);            
     }
            
     void OnRtnRepealFromFutureToBankByFuture(CThostFtdcRspRepealField * pRspRepeal){
          RustCtpActionTdOnRtnRepealFromFutureToBankByFuture(this->rust_call_object, pRspRepeal);            
     }
            
     void OnRspFromBankToFutureByFuture(CThostFtdcReqTransferField * pReqTransfer,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspFromBankToFutureByFuture(this->rust_call_object, pReqTransfer, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspFromFutureToBankByFuture(CThostFtdcReqTransferField * pReqTransfer,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspFromFutureToBankByFuture(this->rust_call_object, pReqTransfer, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQueryBankAccountMoneyByFuture(CThostFtdcReqQueryAccountField * pReqQueryAccount,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpActionTdOnRspQueryBankAccountMoneyByFuture(this->rust_call_object, pReqQueryAccount, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRtnOpenAccountByBank(CThostFtdcOpenAccountField * pOpenAccount){
          RustCtpActionTdOnRtnOpenAccountByBank(this->rust_call_object, pOpenAccount);            
     }
            
     void OnRtnCancelAccountByBank(CThostFtdcCancelAccountField * pCancelAccount){
          RustCtpActionTdOnRtnCancelAccountByBank(this->rust_call_object, pCancelAccount);            
     }
            
     void OnRtnChangeAccountByBank(CThostFtdcChangeAccountField * pChangeAccount){
          RustCtpActionTdOnRtnChangeAccountByBank(this->rust_call_object, pChangeAccount);            
     }
            
protected:
    ~CtpTdSpi(){}
};
CtpTdSpi::CtpTdSpi(void *rust_call_object) : rust_call_object(rust_call_object) {};