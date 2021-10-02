# include "../../../sdk_sources/ctpmini/include/ThostFtdcMdApi.h"
# include "../../../sdk_sources/ctpmini/include/ThostFtdcTraderApi.h"
# include "../../../sdk_sources/ctpmini/include/ThostFtdcUserApiDataType.h"
# include "../../../sdk_sources/ctpmini/include/ThostFtdcUserApiStruct.h"

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
        
void CThostFtdcTraderApiInit(CThostFtdcTraderApi * self , bool bContinuous) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    instance->Init(bContinuous);
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
        
int CThostFtdcTraderApiReqUserLogin(CThostFtdcTraderApi * self , CThostFtdcReqUserLoginField * pReqUserLoginField,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqUserLogin(pReqUserLoginField, nRequestID);
}
        
int CThostFtdcTraderApiReqUserLoginEncrypt(CThostFtdcTraderApi * self , CThostFtdcReqUserLoginField * pReqUserLoginField,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqUserLoginEncrypt(pReqUserLoginField, nRequestID);
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
        
int CThostFtdcTraderApiReqTransFund(CThostFtdcTraderApi * self , CThostFtdcTransFundField * pFtdcTransFund,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqTransFund(pFtdcTransFund, nRequestID);
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
        
int CThostFtdcTraderApiReqQryOptionSelfClose(CThostFtdcTraderApi * self , CThostFtdcQryOptionSelfCloseField * pQryOptionSelfClose,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryOptionSelfClose(pQryOptionSelfClose, nRequestID);
}
        
int CThostFtdcTraderApiReqQrySettlementInfo(CThostFtdcTraderApi * self , CThostFtdcQrySettlementInfoField * pQrySettlementInfo,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQrySettlementInfo(pQrySettlementInfo, nRequestID);
}
        
int CThostFtdcTraderApiReqQryInstrumentStatus(CThostFtdcTraderApi * self , CThostFtdcQryInstrumentStatusField * pQryInstrumentStatus,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryInstrumentStatus(pQryInstrumentStatus, nRequestID);
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
        
int CThostFtdcTraderApiReqQryInstrumentOrderCommRate(CThostFtdcTraderApi * self , CThostFtdcQryInstrumentOrderCommRateField * pQryInstrumentOrderCommRate,int nRequestID) {
    auto instance = static_cast<CThostFtdcTraderApi*>(self);
    return instance->ReqQryInstrumentOrderCommRate(pQryInstrumentOrderCommRate, nRequestID);
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
        extern "C" void RustCtpMiniActionOnFrontDisconnected(void *rust_call_object , int nReason); 
extern "C" void RustCtpMiniActionOnHeartBeatWarning(void *rust_call_object , int nTimeLapse); 
extern "C" void RustCtpMiniActionOnRspAuthenticate(void *rust_call_object , CThostFtdcRspAuthenticateField * pRspAuthenticateField,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspUserLogin(void *rust_call_object , CThostFtdcRspUserLoginField * pRspUserLogin,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspUserLogout(void *rust_call_object , CThostFtdcUserLogoutField * pUserLogout,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspUserPasswordUpdate(void *rust_call_object , CThostFtdcUserPasswordUpdateField * pUserPasswordUpdate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspTradingAccountPasswordUpdate(void *rust_call_object , CThostFtdcTradingAccountPasswordUpdateField * pTradingAccountPasswordUpdate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspOrderInsert(void *rust_call_object , CThostFtdcInputOrderField * pInputOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspParkedOrderInsert(void *rust_call_object , CThostFtdcParkedOrderField * pParkedOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspParkedOrderAction(void *rust_call_object , CThostFtdcParkedOrderActionField * pParkedOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspOrderAction(void *rust_call_object , CThostFtdcInputOrderActionField * pInputOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQueryMaxOrderVolume(void *rust_call_object , CThostFtdcQueryMaxOrderVolumeField * pQueryMaxOrderVolume,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspSettlementInfoConfirm(void *rust_call_object , CThostFtdcSettlementInfoConfirmField * pSettlementInfoConfirm,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspRemoveParkedOrder(void *rust_call_object , CThostFtdcRemoveParkedOrderField * pRemoveParkedOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspRemoveParkedOrderAction(void *rust_call_object , CThostFtdcRemoveParkedOrderActionField * pRemoveParkedOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspExecOrderInsert(void *rust_call_object , CThostFtdcInputExecOrderField * pInputExecOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspExecOrderAction(void *rust_call_object , CThostFtdcInputExecOrderActionField * pInputExecOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspForQuoteInsert(void *rust_call_object , CThostFtdcInputForQuoteField * pInputForQuote,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQuoteInsert(void *rust_call_object , CThostFtdcInputQuoteField * pInputQuote,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQuoteAction(void *rust_call_object , CThostFtdcInputQuoteActionField * pInputQuoteAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspBatchOrderAction(void *rust_call_object , CThostFtdcInputBatchOrderActionField * pInputBatchOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspOptionSelfCloseInsert(void *rust_call_object , CThostFtdcInputOptionSelfCloseField * pInputOptionSelfClose,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspOptionSelfCloseAction(void *rust_call_object , CThostFtdcInputOptionSelfCloseActionField * pInputOptionSelfCloseAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspTransFund(void *rust_call_object , CThostFtdcTransFundField * pTransFund,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryOrder(void *rust_call_object , CThostFtdcOrderField * pOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryTrade(void *rust_call_object , CThostFtdcTradeField * pTrade,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryInvestorPosition(void *rust_call_object , CThostFtdcInvestorPositionField * pInvestorPosition,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryTradingAccount(void *rust_call_object , CThostFtdcTradingAccountField * pTradingAccount,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryInvestor(void *rust_call_object , CThostFtdcInvestorField * pInvestor,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryTradingCode(void *rust_call_object , CThostFtdcTradingCodeField * pTradingCode,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryInstrumentMarginRate(void *rust_call_object , CThostFtdcInstrumentMarginRateField * pInstrumentMarginRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryInstrumentCommissionRate(void *rust_call_object , CThostFtdcInstrumentCommissionRateField * pInstrumentCommissionRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryExchange(void *rust_call_object , CThostFtdcExchangeField * pExchange,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryProduct(void *rust_call_object , CThostFtdcProductField * pProduct,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryInstrument(void *rust_call_object , CThostFtdcInstrumentField * pInstrument,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryDepthMarketData(void *rust_call_object , CThostFtdcDepthMarketDataField * pDepthMarketData,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQrySettlementInfo(void *rust_call_object , CThostFtdcSettlementInfoField * pSettlementInfo,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryInstrumentStatus(void *rust_call_object , CThostFtdcInstrumentStatusField * pInstrumentStatus,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryTransferBank(void *rust_call_object , CThostFtdcTransferBankField * pTransferBank,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryInvestorPositionDetail(void *rust_call_object , CThostFtdcInvestorPositionDetailField * pInvestorPositionDetail,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryNotice(void *rust_call_object , CThostFtdcNoticeField * pNotice,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQrySettlementInfoConfirm(void *rust_call_object , CThostFtdcSettlementInfoConfirmField * pSettlementInfoConfirm,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryInvestorPositionCombineDetail(void *rust_call_object , CThostFtdcInvestorPositionCombineDetailField * pInvestorPositionCombineDetail,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryCFMMCTradingAccountKey(void *rust_call_object , CThostFtdcCFMMCTradingAccountKeyField * pCFMMCTradingAccountKey,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryEWarrantOffset(void *rust_call_object , CThostFtdcEWarrantOffsetField * pEWarrantOffset,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryInvestorProductGroupMargin(void *rust_call_object , CThostFtdcInvestorProductGroupMarginField * pInvestorProductGroupMargin,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryExchangeMarginRate(void *rust_call_object , CThostFtdcExchangeMarginRateField * pExchangeMarginRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryExchangeMarginRateAdjust(void *rust_call_object , CThostFtdcExchangeMarginRateAdjustField * pExchangeMarginRateAdjust,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryExchangeRate(void *rust_call_object , CThostFtdcExchangeRateField * pExchangeRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQrySecAgentACIDMap(void *rust_call_object , CThostFtdcSecAgentACIDMapField * pSecAgentACIDMap,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryOptionInstrTradeCost(void *rust_call_object , CThostFtdcOptionInstrTradeCostField * pOptionInstrTradeCost,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryOptionInstrCommRate(void *rust_call_object , CThostFtdcOptionInstrCommRateField * pOptionInstrCommRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryExecOrder(void *rust_call_object , CThostFtdcExecOrderField * pExecOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryForQuote(void *rust_call_object , CThostFtdcForQuoteField * pForQuote,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryQuote(void *rust_call_object , CThostFtdcQuoteField * pQuote,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryOptionSelfClose(void *rust_call_object , CThostFtdcOptionSelfCloseField * pOptionSelfClose,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryTransferSerial(void *rust_call_object , CThostFtdcTransferSerialField * pTransferSerial,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryAccountregister(void *rust_call_object , CThostFtdcAccountregisterField * pAccountregister,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspError(void *rust_call_object , CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRtnOrder(void *rust_call_object , CThostFtdcOrderField * pOrder); 
extern "C" void RustCtpMiniActionOnRtnTrade(void *rust_call_object , CThostFtdcTradeField * pTrade); 
extern "C" void RustCtpMiniActionOnErrRtnOrderInsert(void *rust_call_object , CThostFtdcInputOrderField * pInputOrder,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpMiniActionOnErrRtnOrderAction(void *rust_call_object , CThostFtdcOrderActionField * pOrderAction,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpMiniActionOnRtnInstrumentStatus(void *rust_call_object , CThostFtdcInstrumentStatusField * pInstrumentStatus); 
extern "C" void RustCtpMiniActionOnRtnTradingNotice(void *rust_call_object , CThostFtdcTradingNoticeInfoField * pTradingNoticeInfo); 
extern "C" void RustCtpMiniActionOnRtnErrorConditionalOrder(void *rust_call_object , CThostFtdcErrorConditionalOrderField * pErrorConditionalOrder); 
extern "C" void RustCtpMiniActionOnRtnExecOrder(void *rust_call_object , CThostFtdcExecOrderField * pExecOrder); 
extern "C" void RustCtpMiniActionOnErrRtnExecOrderInsert(void *rust_call_object , CThostFtdcInputExecOrderField * pInputExecOrder,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpMiniActionOnErrRtnExecOrderAction(void *rust_call_object , CThostFtdcExecOrderActionField * pExecOrderAction,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpMiniActionOnErrRtnForQuoteInsert(void *rust_call_object , CThostFtdcInputForQuoteField * pInputForQuote,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpMiniActionOnRtnQuote(void *rust_call_object , CThostFtdcQuoteField * pQuote); 
extern "C" void RustCtpMiniActionOnErrRtnQuoteInsert(void *rust_call_object , CThostFtdcInputQuoteField * pInputQuote,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpMiniActionOnErrRtnQuoteAction(void *rust_call_object , CThostFtdcQuoteActionField * pQuoteAction,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpMiniActionOnRtnForQuoteRsp(void *rust_call_object , CThostFtdcForQuoteRspField * pForQuoteRsp); 
extern "C" void RustCtpMiniActionOnErrRtnBatchOrderAction(void *rust_call_object , CThostFtdcBatchOrderActionField * pBatchOrderAction,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpMiniActionOnRtnOptionSelfClose(void *rust_call_object , CThostFtdcOptionSelfCloseField * pOptionSelfClose); 
extern "C" void RustCtpMiniActionOnErrRtnOptionSelfCloseInsert(void *rust_call_object , CThostFtdcInputOptionSelfCloseField * pInputOptionSelfClose,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpMiniActionOnErrRtnOptionSelfCloseAction(void *rust_call_object , CThostFtdcOptionSelfCloseActionField * pOptionSelfCloseAction,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpMiniActionOnRspQryContractBank(void *rust_call_object , CThostFtdcContractBankField * pContractBank,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryParkedOrder(void *rust_call_object , CThostFtdcParkedOrderField * pParkedOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryParkedOrderAction(void *rust_call_object , CThostFtdcParkedOrderActionField * pParkedOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryTradingNotice(void *rust_call_object , CThostFtdcTradingNoticeField * pTradingNotice,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryBrokerTradingParams(void *rust_call_object , CThostFtdcBrokerTradingParamsField * pBrokerTradingParams,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryBrokerTradingAlgos(void *rust_call_object , CThostFtdcBrokerTradingAlgosField * pBrokerTradingAlgos,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQryInstrumentOrderCommRate(void *rust_call_object , CThostFtdcInstrumentOrderCommRateField * pInstrumentOrderCommRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRtnFromBankToFutureByBank(void *rust_call_object , CThostFtdcRspTransferField * pRspTransfer); 
extern "C" void RustCtpMiniActionOnRtnFromFutureToBankByBank(void *rust_call_object , CThostFtdcRspTransferField * pRspTransfer); 
extern "C" void RustCtpMiniActionOnRtnRepealFromBankToFutureByBank(void *rust_call_object , CThostFtdcRspRepealField * pRspRepeal); 
extern "C" void RustCtpMiniActionOnRtnRepealFromFutureToBankByBank(void *rust_call_object , CThostFtdcRspRepealField * pRspRepeal); 
extern "C" void RustCtpMiniActionOnRtnFromBankToFutureByFuture(void *rust_call_object , CThostFtdcRspTransferField * pRspTransfer); 
extern "C" void RustCtpMiniActionOnRtnFromFutureToBankByFuture(void *rust_call_object , CThostFtdcRspTransferField * pRspTransfer); 
extern "C" void RustCtpMiniActionOnRtnRepealFromBankToFutureByFutureManual(void *rust_call_object , CThostFtdcRspRepealField * pRspRepeal); 
extern "C" void RustCtpMiniActionOnRtnRepealFromFutureToBankByFutureManual(void *rust_call_object , CThostFtdcRspRepealField * pRspRepeal); 
extern "C" void RustCtpMiniActionOnRtnQueryBankBalanceByFuture(void *rust_call_object , CThostFtdcNotifyQueryAccountField * pNotifyQueryAccount); 
extern "C" void RustCtpMiniActionOnErrRtnBankToFutureByFuture(void *rust_call_object , CThostFtdcReqTransferField * pReqTransfer,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpMiniActionOnErrRtnFutureToBankByFuture(void *rust_call_object , CThostFtdcReqTransferField * pReqTransfer,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpMiniActionOnErrRtnRepealBankToFutureByFutureManual(void *rust_call_object , CThostFtdcReqRepealField * pReqRepeal,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpMiniActionOnErrRtnRepealFutureToBankByFutureManual(void *rust_call_object , CThostFtdcReqRepealField * pReqRepeal,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpMiniActionOnErrRtnQueryBankBalanceByFuture(void *rust_call_object , CThostFtdcReqQueryAccountField * pReqQueryAccount,CThostFtdcRspInfoField * pRspInfo); 
extern "C" void RustCtpMiniActionOnRtnRepealFromBankToFutureByFuture(void *rust_call_object , CThostFtdcRspRepealField * pRspRepeal); 
extern "C" void RustCtpMiniActionOnRtnRepealFromFutureToBankByFuture(void *rust_call_object , CThostFtdcRspRepealField * pRspRepeal); 
extern "C" void RustCtpMiniActionOnRspFromBankToFutureByFuture(void *rust_call_object , CThostFtdcReqTransferField * pReqTransfer,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspFromFutureToBankByFuture(void *rust_call_object , CThostFtdcReqTransferField * pReqTransfer,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRspQueryBankAccountMoneyByFuture(void *rust_call_object , CThostFtdcReqQueryAccountField * pReqQueryAccount,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast); 
extern "C" void RustCtpMiniActionOnRtnOpenAccountByBank(void *rust_call_object , CThostFtdcOpenAccountField * pOpenAccount); 
extern "C" void RustCtpMiniActionOnRtnCancelAccountByBank(void *rust_call_object , CThostFtdcCancelAccountField * pCancelAccount); 
extern "C" void RustCtpMiniActionOnRtnChangeAccountByBank(void *rust_call_object , CThostFtdcChangeAccountField * pChangeAccount); 

class CtpMiniTdSpi: CThostFtdcTraderSpi {
public:
    void *rust_call_object;
    
    CtpMiniTdSpi(void *rust_call_object) : rust_call_object(rust_call_object) {};
                
            
     void OnFrontDisconnected(int nReason){
          RustCtpMiniActionOnFrontDisconnected(this->rust_call_object, nReason);            
     }
            
     void OnHeartBeatWarning(int nTimeLapse){
          RustCtpMiniActionOnHeartBeatWarning(this->rust_call_object, nTimeLapse);            
     }
            
     void OnRspAuthenticate(CThostFtdcRspAuthenticateField * pRspAuthenticateField,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspAuthenticate(this->rust_call_object, pRspAuthenticateField, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspUserLogin(CThostFtdcRspUserLoginField * pRspUserLogin,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspUserLogin(this->rust_call_object, pRspUserLogin, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspUserLogout(CThostFtdcUserLogoutField * pUserLogout,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspUserLogout(this->rust_call_object, pUserLogout, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspUserPasswordUpdate(CThostFtdcUserPasswordUpdateField * pUserPasswordUpdate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspUserPasswordUpdate(this->rust_call_object, pUserPasswordUpdate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspTradingAccountPasswordUpdate(CThostFtdcTradingAccountPasswordUpdateField * pTradingAccountPasswordUpdate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspTradingAccountPasswordUpdate(this->rust_call_object, pTradingAccountPasswordUpdate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspOrderInsert(CThostFtdcInputOrderField * pInputOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspOrderInsert(this->rust_call_object, pInputOrder, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspParkedOrderInsert(CThostFtdcParkedOrderField * pParkedOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspParkedOrderInsert(this->rust_call_object, pParkedOrder, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspParkedOrderAction(CThostFtdcParkedOrderActionField * pParkedOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspParkedOrderAction(this->rust_call_object, pParkedOrderAction, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspOrderAction(CThostFtdcInputOrderActionField * pInputOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspOrderAction(this->rust_call_object, pInputOrderAction, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQueryMaxOrderVolume(CThostFtdcQueryMaxOrderVolumeField * pQueryMaxOrderVolume,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQueryMaxOrderVolume(this->rust_call_object, pQueryMaxOrderVolume, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspSettlementInfoConfirm(CThostFtdcSettlementInfoConfirmField * pSettlementInfoConfirm,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspSettlementInfoConfirm(this->rust_call_object, pSettlementInfoConfirm, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspRemoveParkedOrder(CThostFtdcRemoveParkedOrderField * pRemoveParkedOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspRemoveParkedOrder(this->rust_call_object, pRemoveParkedOrder, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspRemoveParkedOrderAction(CThostFtdcRemoveParkedOrderActionField * pRemoveParkedOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspRemoveParkedOrderAction(this->rust_call_object, pRemoveParkedOrderAction, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspExecOrderInsert(CThostFtdcInputExecOrderField * pInputExecOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspExecOrderInsert(this->rust_call_object, pInputExecOrder, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspExecOrderAction(CThostFtdcInputExecOrderActionField * pInputExecOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspExecOrderAction(this->rust_call_object, pInputExecOrderAction, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspForQuoteInsert(CThostFtdcInputForQuoteField * pInputForQuote,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspForQuoteInsert(this->rust_call_object, pInputForQuote, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQuoteInsert(CThostFtdcInputQuoteField * pInputQuote,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQuoteInsert(this->rust_call_object, pInputQuote, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQuoteAction(CThostFtdcInputQuoteActionField * pInputQuoteAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQuoteAction(this->rust_call_object, pInputQuoteAction, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspBatchOrderAction(CThostFtdcInputBatchOrderActionField * pInputBatchOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspBatchOrderAction(this->rust_call_object, pInputBatchOrderAction, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspOptionSelfCloseInsert(CThostFtdcInputOptionSelfCloseField * pInputOptionSelfClose,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspOptionSelfCloseInsert(this->rust_call_object, pInputOptionSelfClose, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspOptionSelfCloseAction(CThostFtdcInputOptionSelfCloseActionField * pInputOptionSelfCloseAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspOptionSelfCloseAction(this->rust_call_object, pInputOptionSelfCloseAction, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspTransFund(CThostFtdcTransFundField * pTransFund,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspTransFund(this->rust_call_object, pTransFund, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryOrder(CThostFtdcOrderField * pOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryOrder(this->rust_call_object, pOrder, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryTrade(CThostFtdcTradeField * pTrade,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryTrade(this->rust_call_object, pTrade, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInvestorPosition(CThostFtdcInvestorPositionField * pInvestorPosition,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryInvestorPosition(this->rust_call_object, pInvestorPosition, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryTradingAccount(CThostFtdcTradingAccountField * pTradingAccount,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryTradingAccount(this->rust_call_object, pTradingAccount, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInvestor(CThostFtdcInvestorField * pInvestor,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryInvestor(this->rust_call_object, pInvestor, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryTradingCode(CThostFtdcTradingCodeField * pTradingCode,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryTradingCode(this->rust_call_object, pTradingCode, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInstrumentMarginRate(CThostFtdcInstrumentMarginRateField * pInstrumentMarginRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryInstrumentMarginRate(this->rust_call_object, pInstrumentMarginRate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInstrumentCommissionRate(CThostFtdcInstrumentCommissionRateField * pInstrumentCommissionRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryInstrumentCommissionRate(this->rust_call_object, pInstrumentCommissionRate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryExchange(CThostFtdcExchangeField * pExchange,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryExchange(this->rust_call_object, pExchange, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryProduct(CThostFtdcProductField * pProduct,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryProduct(this->rust_call_object, pProduct, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInstrument(CThostFtdcInstrumentField * pInstrument,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryInstrument(this->rust_call_object, pInstrument, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryDepthMarketData(CThostFtdcDepthMarketDataField * pDepthMarketData,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryDepthMarketData(this->rust_call_object, pDepthMarketData, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQrySettlementInfo(CThostFtdcSettlementInfoField * pSettlementInfo,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQrySettlementInfo(this->rust_call_object, pSettlementInfo, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInstrumentStatus(CThostFtdcInstrumentStatusField * pInstrumentStatus,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryInstrumentStatus(this->rust_call_object, pInstrumentStatus, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryTransferBank(CThostFtdcTransferBankField * pTransferBank,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryTransferBank(this->rust_call_object, pTransferBank, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInvestorPositionDetail(CThostFtdcInvestorPositionDetailField * pInvestorPositionDetail,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryInvestorPositionDetail(this->rust_call_object, pInvestorPositionDetail, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryNotice(CThostFtdcNoticeField * pNotice,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryNotice(this->rust_call_object, pNotice, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQrySettlementInfoConfirm(CThostFtdcSettlementInfoConfirmField * pSettlementInfoConfirm,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQrySettlementInfoConfirm(this->rust_call_object, pSettlementInfoConfirm, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInvestorPositionCombineDetail(CThostFtdcInvestorPositionCombineDetailField * pInvestorPositionCombineDetail,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryInvestorPositionCombineDetail(this->rust_call_object, pInvestorPositionCombineDetail, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryCFMMCTradingAccountKey(CThostFtdcCFMMCTradingAccountKeyField * pCFMMCTradingAccountKey,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryCFMMCTradingAccountKey(this->rust_call_object, pCFMMCTradingAccountKey, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryEWarrantOffset(CThostFtdcEWarrantOffsetField * pEWarrantOffset,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryEWarrantOffset(this->rust_call_object, pEWarrantOffset, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInvestorProductGroupMargin(CThostFtdcInvestorProductGroupMarginField * pInvestorProductGroupMargin,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryInvestorProductGroupMargin(this->rust_call_object, pInvestorProductGroupMargin, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryExchangeMarginRate(CThostFtdcExchangeMarginRateField * pExchangeMarginRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryExchangeMarginRate(this->rust_call_object, pExchangeMarginRate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryExchangeMarginRateAdjust(CThostFtdcExchangeMarginRateAdjustField * pExchangeMarginRateAdjust,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryExchangeMarginRateAdjust(this->rust_call_object, pExchangeMarginRateAdjust, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryExchangeRate(CThostFtdcExchangeRateField * pExchangeRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryExchangeRate(this->rust_call_object, pExchangeRate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQrySecAgentACIDMap(CThostFtdcSecAgentACIDMapField * pSecAgentACIDMap,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQrySecAgentACIDMap(this->rust_call_object, pSecAgentACIDMap, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryOptionInstrTradeCost(CThostFtdcOptionInstrTradeCostField * pOptionInstrTradeCost,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryOptionInstrTradeCost(this->rust_call_object, pOptionInstrTradeCost, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryOptionInstrCommRate(CThostFtdcOptionInstrCommRateField * pOptionInstrCommRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryOptionInstrCommRate(this->rust_call_object, pOptionInstrCommRate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryExecOrder(CThostFtdcExecOrderField * pExecOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryExecOrder(this->rust_call_object, pExecOrder, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryForQuote(CThostFtdcForQuoteField * pForQuote,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryForQuote(this->rust_call_object, pForQuote, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryQuote(CThostFtdcQuoteField * pQuote,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryQuote(this->rust_call_object, pQuote, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryOptionSelfClose(CThostFtdcOptionSelfCloseField * pOptionSelfClose,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryOptionSelfClose(this->rust_call_object, pOptionSelfClose, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryTransferSerial(CThostFtdcTransferSerialField * pTransferSerial,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryTransferSerial(this->rust_call_object, pTransferSerial, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryAccountregister(CThostFtdcAccountregisterField * pAccountregister,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryAccountregister(this->rust_call_object, pAccountregister, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspError(CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspError(this->rust_call_object, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRtnOrder(CThostFtdcOrderField * pOrder){
          RustCtpMiniActionOnRtnOrder(this->rust_call_object, pOrder);            
     }
            
     void OnRtnTrade(CThostFtdcTradeField * pTrade){
          RustCtpMiniActionOnRtnTrade(this->rust_call_object, pTrade);            
     }
            
     void OnErrRtnOrderInsert(CThostFtdcInputOrderField * pInputOrder,CThostFtdcRspInfoField * pRspInfo){
          RustCtpMiniActionOnErrRtnOrderInsert(this->rust_call_object, pInputOrder, pRspInfo);            
     }
            
     void OnErrRtnOrderAction(CThostFtdcOrderActionField * pOrderAction,CThostFtdcRspInfoField * pRspInfo){
          RustCtpMiniActionOnErrRtnOrderAction(this->rust_call_object, pOrderAction, pRspInfo);            
     }
            
     void OnRtnInstrumentStatus(CThostFtdcInstrumentStatusField * pInstrumentStatus){
          RustCtpMiniActionOnRtnInstrumentStatus(this->rust_call_object, pInstrumentStatus);            
     }
            
     void OnRtnTradingNotice(CThostFtdcTradingNoticeInfoField * pTradingNoticeInfo){
          RustCtpMiniActionOnRtnTradingNotice(this->rust_call_object, pTradingNoticeInfo);            
     }
            
     void OnRtnErrorConditionalOrder(CThostFtdcErrorConditionalOrderField * pErrorConditionalOrder){
          RustCtpMiniActionOnRtnErrorConditionalOrder(this->rust_call_object, pErrorConditionalOrder);            
     }
            
     void OnRtnExecOrder(CThostFtdcExecOrderField * pExecOrder){
          RustCtpMiniActionOnRtnExecOrder(this->rust_call_object, pExecOrder);            
     }
            
     void OnErrRtnExecOrderInsert(CThostFtdcInputExecOrderField * pInputExecOrder,CThostFtdcRspInfoField * pRspInfo){
          RustCtpMiniActionOnErrRtnExecOrderInsert(this->rust_call_object, pInputExecOrder, pRspInfo);            
     }
            
     void OnErrRtnExecOrderAction(CThostFtdcExecOrderActionField * pExecOrderAction,CThostFtdcRspInfoField * pRspInfo){
          RustCtpMiniActionOnErrRtnExecOrderAction(this->rust_call_object, pExecOrderAction, pRspInfo);            
     }
            
     void OnErrRtnForQuoteInsert(CThostFtdcInputForQuoteField * pInputForQuote,CThostFtdcRspInfoField * pRspInfo){
          RustCtpMiniActionOnErrRtnForQuoteInsert(this->rust_call_object, pInputForQuote, pRspInfo);            
     }
            
     void OnRtnQuote(CThostFtdcQuoteField * pQuote){
          RustCtpMiniActionOnRtnQuote(this->rust_call_object, pQuote);            
     }
            
     void OnErrRtnQuoteInsert(CThostFtdcInputQuoteField * pInputQuote,CThostFtdcRspInfoField * pRspInfo){
          RustCtpMiniActionOnErrRtnQuoteInsert(this->rust_call_object, pInputQuote, pRspInfo);            
     }
            
     void OnErrRtnQuoteAction(CThostFtdcQuoteActionField * pQuoteAction,CThostFtdcRspInfoField * pRspInfo){
          RustCtpMiniActionOnErrRtnQuoteAction(this->rust_call_object, pQuoteAction, pRspInfo);            
     }
            
     void OnRtnForQuoteRsp(CThostFtdcForQuoteRspField * pForQuoteRsp){
          RustCtpMiniActionOnRtnForQuoteRsp(this->rust_call_object, pForQuoteRsp);            
     }
            
     void OnErrRtnBatchOrderAction(CThostFtdcBatchOrderActionField * pBatchOrderAction,CThostFtdcRspInfoField * pRspInfo){
          RustCtpMiniActionOnErrRtnBatchOrderAction(this->rust_call_object, pBatchOrderAction, pRspInfo);            
     }
            
     void OnRtnOptionSelfClose(CThostFtdcOptionSelfCloseField * pOptionSelfClose){
          RustCtpMiniActionOnRtnOptionSelfClose(this->rust_call_object, pOptionSelfClose);            
     }
            
     void OnErrRtnOptionSelfCloseInsert(CThostFtdcInputOptionSelfCloseField * pInputOptionSelfClose,CThostFtdcRspInfoField * pRspInfo){
          RustCtpMiniActionOnErrRtnOptionSelfCloseInsert(this->rust_call_object, pInputOptionSelfClose, pRspInfo);            
     }
            
     void OnErrRtnOptionSelfCloseAction(CThostFtdcOptionSelfCloseActionField * pOptionSelfCloseAction,CThostFtdcRspInfoField * pRspInfo){
          RustCtpMiniActionOnErrRtnOptionSelfCloseAction(this->rust_call_object, pOptionSelfCloseAction, pRspInfo);            
     }
            
     void OnRspQryContractBank(CThostFtdcContractBankField * pContractBank,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryContractBank(this->rust_call_object, pContractBank, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryParkedOrder(CThostFtdcParkedOrderField * pParkedOrder,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryParkedOrder(this->rust_call_object, pParkedOrder, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryParkedOrderAction(CThostFtdcParkedOrderActionField * pParkedOrderAction,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryParkedOrderAction(this->rust_call_object, pParkedOrderAction, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryTradingNotice(CThostFtdcTradingNoticeField * pTradingNotice,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryTradingNotice(this->rust_call_object, pTradingNotice, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryBrokerTradingParams(CThostFtdcBrokerTradingParamsField * pBrokerTradingParams,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryBrokerTradingParams(this->rust_call_object, pBrokerTradingParams, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryBrokerTradingAlgos(CThostFtdcBrokerTradingAlgosField * pBrokerTradingAlgos,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryBrokerTradingAlgos(this->rust_call_object, pBrokerTradingAlgos, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQryInstrumentOrderCommRate(CThostFtdcInstrumentOrderCommRateField * pInstrumentOrderCommRate,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQryInstrumentOrderCommRate(this->rust_call_object, pInstrumentOrderCommRate, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRtnFromBankToFutureByBank(CThostFtdcRspTransferField * pRspTransfer){
          RustCtpMiniActionOnRtnFromBankToFutureByBank(this->rust_call_object, pRspTransfer);            
     }
            
     void OnRtnFromFutureToBankByBank(CThostFtdcRspTransferField * pRspTransfer){
          RustCtpMiniActionOnRtnFromFutureToBankByBank(this->rust_call_object, pRspTransfer);            
     }
            
     void OnRtnRepealFromBankToFutureByBank(CThostFtdcRspRepealField * pRspRepeal){
          RustCtpMiniActionOnRtnRepealFromBankToFutureByBank(this->rust_call_object, pRspRepeal);            
     }
            
     void OnRtnRepealFromFutureToBankByBank(CThostFtdcRspRepealField * pRspRepeal){
          RustCtpMiniActionOnRtnRepealFromFutureToBankByBank(this->rust_call_object, pRspRepeal);            
     }
            
     void OnRtnFromBankToFutureByFuture(CThostFtdcRspTransferField * pRspTransfer){
          RustCtpMiniActionOnRtnFromBankToFutureByFuture(this->rust_call_object, pRspTransfer);            
     }
            
     void OnRtnFromFutureToBankByFuture(CThostFtdcRspTransferField * pRspTransfer){
          RustCtpMiniActionOnRtnFromFutureToBankByFuture(this->rust_call_object, pRspTransfer);            
     }
            
     void OnRtnRepealFromBankToFutureByFutureManual(CThostFtdcRspRepealField * pRspRepeal){
          RustCtpMiniActionOnRtnRepealFromBankToFutureByFutureManual(this->rust_call_object, pRspRepeal);            
     }
            
     void OnRtnRepealFromFutureToBankByFutureManual(CThostFtdcRspRepealField * pRspRepeal){
          RustCtpMiniActionOnRtnRepealFromFutureToBankByFutureManual(this->rust_call_object, pRspRepeal);            
     }
            
     void OnRtnQueryBankBalanceByFuture(CThostFtdcNotifyQueryAccountField * pNotifyQueryAccount){
          RustCtpMiniActionOnRtnQueryBankBalanceByFuture(this->rust_call_object, pNotifyQueryAccount);            
     }
            
     void OnErrRtnBankToFutureByFuture(CThostFtdcReqTransferField * pReqTransfer,CThostFtdcRspInfoField * pRspInfo){
          RustCtpMiniActionOnErrRtnBankToFutureByFuture(this->rust_call_object, pReqTransfer, pRspInfo);            
     }
            
     void OnErrRtnFutureToBankByFuture(CThostFtdcReqTransferField * pReqTransfer,CThostFtdcRspInfoField * pRspInfo){
          RustCtpMiniActionOnErrRtnFutureToBankByFuture(this->rust_call_object, pReqTransfer, pRspInfo);            
     }
            
     void OnErrRtnRepealBankToFutureByFutureManual(CThostFtdcReqRepealField * pReqRepeal,CThostFtdcRspInfoField * pRspInfo){
          RustCtpMiniActionOnErrRtnRepealBankToFutureByFutureManual(this->rust_call_object, pReqRepeal, pRspInfo);            
     }
            
     void OnErrRtnRepealFutureToBankByFutureManual(CThostFtdcReqRepealField * pReqRepeal,CThostFtdcRspInfoField * pRspInfo){
          RustCtpMiniActionOnErrRtnRepealFutureToBankByFutureManual(this->rust_call_object, pReqRepeal, pRspInfo);            
     }
            
     void OnErrRtnQueryBankBalanceByFuture(CThostFtdcReqQueryAccountField * pReqQueryAccount,CThostFtdcRspInfoField * pRspInfo){
          RustCtpMiniActionOnErrRtnQueryBankBalanceByFuture(this->rust_call_object, pReqQueryAccount, pRspInfo);            
     }
            
     void OnRtnRepealFromBankToFutureByFuture(CThostFtdcRspRepealField * pRspRepeal){
          RustCtpMiniActionOnRtnRepealFromBankToFutureByFuture(this->rust_call_object, pRspRepeal);            
     }
            
     void OnRtnRepealFromFutureToBankByFuture(CThostFtdcRspRepealField * pRspRepeal){
          RustCtpMiniActionOnRtnRepealFromFutureToBankByFuture(this->rust_call_object, pRspRepeal);            
     }
            
     void OnRspFromBankToFutureByFuture(CThostFtdcReqTransferField * pReqTransfer,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspFromBankToFutureByFuture(this->rust_call_object, pReqTransfer, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspFromFutureToBankByFuture(CThostFtdcReqTransferField * pReqTransfer,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspFromFutureToBankByFuture(this->rust_call_object, pReqTransfer, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRspQueryBankAccountMoneyByFuture(CThostFtdcReqQueryAccountField * pReqQueryAccount,CThostFtdcRspInfoField * pRspInfo,int nRequestID,bool bIsLast){
          RustCtpMiniActionOnRspQueryBankAccountMoneyByFuture(this->rust_call_object, pReqQueryAccount, pRspInfo, nRequestID, bIsLast);            
     }
            
     void OnRtnOpenAccountByBank(CThostFtdcOpenAccountField * pOpenAccount){
          RustCtpMiniActionOnRtnOpenAccountByBank(this->rust_call_object, pOpenAccount);            
     }
            
     void OnRtnCancelAccountByBank(CThostFtdcCancelAccountField * pCancelAccount){
          RustCtpMiniActionOnRtnCancelAccountByBank(this->rust_call_object, pCancelAccount);            
     }
            
     void OnRtnChangeAccountByBank(CThostFtdcChangeAccountField * pChangeAccount){
          RustCtpMiniActionOnRtnChangeAccountByBank(this->rust_call_object, pChangeAccount);            
     }
            
protected:
    ~CtpMiniTdSpi(){}
};
