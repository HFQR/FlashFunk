use crate::ctpmini::sys::*;
use std::os::raw::c_int;


#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnFrontDisconnected(this: *mut ::std::os::raw::c_void, nReason: c_int ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_front_disconnected(nReason);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnHeartBeatWarning(this: *mut ::std::os::raw::c_void, nTimeLapse: c_int ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_heart_beat_warning(nTimeLapse);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspAuthenticate(this: *mut ::std::os::raw::c_void, pRspAuthenticateField: *mut CThostFtdcRspAuthenticateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_authenticate(pRspAuthenticateField, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspUserLogin(this: *mut ::std::os::raw::c_void, pRspUserLogin: *mut CThostFtdcRspUserLoginField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_user_login(pRspUserLogin, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspUserLogout(this: *mut ::std::os::raw::c_void, pUserLogout: *mut CThostFtdcUserLogoutField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_user_logout(pUserLogout, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspUserPasswordUpdate(this: *mut ::std::os::raw::c_void, pUserPasswordUpdate: *mut CThostFtdcUserPasswordUpdateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_user_password_update(pUserPasswordUpdate, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspTradingAccountPasswordUpdate(this: *mut ::std::os::raw::c_void, pTradingAccountPasswordUpdate: *mut CThostFtdcTradingAccountPasswordUpdateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_trading_account_password_update(pTradingAccountPasswordUpdate, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspOrderInsert(this: *mut ::std::os::raw::c_void, pInputOrder: *mut CThostFtdcInputOrderField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_order_insert(pInputOrder, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspParkedOrderInsert(this: *mut ::std::os::raw::c_void, pParkedOrder: *mut CThostFtdcParkedOrderField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_parked_order_insert(pParkedOrder, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspParkedOrderAction(this: *mut ::std::os::raw::c_void, pParkedOrderAction: *mut CThostFtdcParkedOrderActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_parked_order_action(pParkedOrderAction, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspOrderAction(this: *mut ::std::os::raw::c_void, pInputOrderAction: *mut CThostFtdcInputOrderActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_order_action(pInputOrderAction, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQueryMaxOrderVolume(this: *mut ::std::os::raw::c_void, pQueryMaxOrderVolume: *mut CThostFtdcQueryMaxOrderVolumeField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_query_max_order_volume(pQueryMaxOrderVolume, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspSettlementInfoConfirm(this: *mut ::std::os::raw::c_void, pSettlementInfoConfirm: *mut CThostFtdcSettlementInfoConfirmField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_settlement_info_confirm(pSettlementInfoConfirm, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspRemoveParkedOrder(this: *mut ::std::os::raw::c_void, pRemoveParkedOrder: *mut CThostFtdcRemoveParkedOrderField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_remove_parked_order(pRemoveParkedOrder, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspRemoveParkedOrderAction(this: *mut ::std::os::raw::c_void, pRemoveParkedOrderAction: *mut CThostFtdcRemoveParkedOrderActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_remove_parked_order_action(pRemoveParkedOrderAction, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspExecOrderInsert(this: *mut ::std::os::raw::c_void, pInputExecOrder: *mut CThostFtdcInputExecOrderField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_exec_order_insert(pInputExecOrder, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspExecOrderAction(this: *mut ::std::os::raw::c_void, pInputExecOrderAction: *mut CThostFtdcInputExecOrderActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_exec_order_action(pInputExecOrderAction, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspForQuoteInsert(this: *mut ::std::os::raw::c_void, pInputForQuote: *mut CThostFtdcInputForQuoteField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_for_quote_insert(pInputForQuote, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQuoteInsert(this: *mut ::std::os::raw::c_void, pInputQuote: *mut CThostFtdcInputQuoteField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_quote_insert(pInputQuote, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQuoteAction(this: *mut ::std::os::raw::c_void, pInputQuoteAction: *mut CThostFtdcInputQuoteActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_quote_action(pInputQuoteAction, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspBatchOrderAction(this: *mut ::std::os::raw::c_void, pInputBatchOrderAction: *mut CThostFtdcInputBatchOrderActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_batch_order_action(pInputBatchOrderAction, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspOptionSelfCloseInsert(this: *mut ::std::os::raw::c_void, pInputOptionSelfClose: *mut CThostFtdcInputOptionSelfCloseField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_option_self_close_insert(pInputOptionSelfClose, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspOptionSelfCloseAction(this: *mut ::std::os::raw::c_void, pInputOptionSelfCloseAction: *mut CThostFtdcInputOptionSelfCloseActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_option_self_close_action(pInputOptionSelfCloseAction, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspTransFund(this: *mut ::std::os::raw::c_void, pTransFund: *mut CThostFtdcTransFundField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_trans_fund(pTransFund, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryOrder(this: *mut ::std::os::raw::c_void, pOrder: *mut CThostFtdcOrderField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_order(pOrder, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryTrade(this: *mut ::std::os::raw::c_void, pTrade: *mut CThostFtdcTradeField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_trade(pTrade, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryInvestorPosition(this: *mut ::std::os::raw::c_void, pInvestorPosition: *mut CThostFtdcInvestorPositionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_investor_position(pInvestorPosition, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryTradingAccount(this: *mut ::std::os::raw::c_void, pTradingAccount: *mut CThostFtdcTradingAccountField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_trading_account(pTradingAccount, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryInvestor(this: *mut ::std::os::raw::c_void, pInvestor: *mut CThostFtdcInvestorField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_investor(pInvestor, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryTradingCode(this: *mut ::std::os::raw::c_void, pTradingCode: *mut CThostFtdcTradingCodeField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_trading_code(pTradingCode, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryInstrumentMarginRate(this: *mut ::std::os::raw::c_void, pInstrumentMarginRate: *mut CThostFtdcInstrumentMarginRateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_instrument_margin_rate(pInstrumentMarginRate, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryInstrumentCommissionRate(this: *mut ::std::os::raw::c_void, pInstrumentCommissionRate: *mut CThostFtdcInstrumentCommissionRateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_instrument_commission_rate(pInstrumentCommissionRate, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryExchange(this: *mut ::std::os::raw::c_void, pExchange: *mut CThostFtdcExchangeField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_exchange(pExchange, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryProduct(this: *mut ::std::os::raw::c_void, pProduct: *mut CThostFtdcProductField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_product(pProduct, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryInstrument(this: *mut ::std::os::raw::c_void, pInstrument: *mut CThostFtdcInstrumentField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_instrument(pInstrument, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryDepthMarketData(this: *mut ::std::os::raw::c_void, pDepthMarketData: *mut CThostFtdcDepthMarketDataField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_depth_market_data(pDepthMarketData, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQrySettlementInfo(this: *mut ::std::os::raw::c_void, pSettlementInfo: *mut CThostFtdcSettlementInfoField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_settlement_info(pSettlementInfo, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryInstrumentStatus(this: *mut ::std::os::raw::c_void, pInstrumentStatus: *mut CThostFtdcInstrumentStatusField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_instrument_status(pInstrumentStatus, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryTransferBank(this: *mut ::std::os::raw::c_void, pTransferBank: *mut CThostFtdcTransferBankField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_transfer_bank(pTransferBank, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryInvestorPositionDetail(this: *mut ::std::os::raw::c_void, pInvestorPositionDetail: *mut CThostFtdcInvestorPositionDetailField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_investor_position_detail(pInvestorPositionDetail, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryNotice(this: *mut ::std::os::raw::c_void, pNotice: *mut CThostFtdcNoticeField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_notice(pNotice, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQrySettlementInfoConfirm(this: *mut ::std::os::raw::c_void, pSettlementInfoConfirm: *mut CThostFtdcSettlementInfoConfirmField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_settlement_info_confirm(pSettlementInfoConfirm, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryInvestorPositionCombineDetail(this: *mut ::std::os::raw::c_void, pInvestorPositionCombineDetail: *mut CThostFtdcInvestorPositionCombineDetailField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_investor_position_combine_detail(pInvestorPositionCombineDetail, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryCFMMCTradingAccountKey(this: *mut ::std::os::raw::c_void, pCFMMCTradingAccountKey: *mut CThostFtdcCFMMCTradingAccountKeyField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_c_f_m_m_c_trading_account_key(pCFMMCTradingAccountKey, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryEWarrantOffset(this: *mut ::std::os::raw::c_void, pEWarrantOffset: *mut CThostFtdcEWarrantOffsetField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_e_warrant_offset(pEWarrantOffset, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryInvestorProductGroupMargin(this: *mut ::std::os::raw::c_void, pInvestorProductGroupMargin: *mut CThostFtdcInvestorProductGroupMarginField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_investor_product_group_margin(pInvestorProductGroupMargin, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryExchangeMarginRate(this: *mut ::std::os::raw::c_void, pExchangeMarginRate: *mut CThostFtdcExchangeMarginRateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_exchange_margin_rate(pExchangeMarginRate, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryExchangeMarginRateAdjust(this: *mut ::std::os::raw::c_void, pExchangeMarginRateAdjust: *mut CThostFtdcExchangeMarginRateAdjustField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_exchange_margin_rate_adjust(pExchangeMarginRateAdjust, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryExchangeRate(this: *mut ::std::os::raw::c_void, pExchangeRate: *mut CThostFtdcExchangeRateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_exchange_rate(pExchangeRate, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQrySecAgentACIDMap(this: *mut ::std::os::raw::c_void, pSecAgentACIDMap: *mut CThostFtdcSecAgentACIDMapField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_sec_agent_a_c_i_d_map(pSecAgentACIDMap, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryOptionInstrTradeCost(this: *mut ::std::os::raw::c_void, pOptionInstrTradeCost: *mut CThostFtdcOptionInstrTradeCostField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_option_instr_trade_cost(pOptionInstrTradeCost, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryOptionInstrCommRate(this: *mut ::std::os::raw::c_void, pOptionInstrCommRate: *mut CThostFtdcOptionInstrCommRateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_option_instr_comm_rate(pOptionInstrCommRate, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryExecOrder(this: *mut ::std::os::raw::c_void, pExecOrder: *mut CThostFtdcExecOrderField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_exec_order(pExecOrder, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryForQuote(this: *mut ::std::os::raw::c_void, pForQuote: *mut CThostFtdcForQuoteField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_for_quote(pForQuote, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryQuote(this: *mut ::std::os::raw::c_void, pQuote: *mut CThostFtdcQuoteField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_quote(pQuote, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryOptionSelfClose(this: *mut ::std::os::raw::c_void, pOptionSelfClose: *mut CThostFtdcOptionSelfCloseField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_option_self_close(pOptionSelfClose, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryTransferSerial(this: *mut ::std::os::raw::c_void, pTransferSerial: *mut CThostFtdcTransferSerialField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_transfer_serial(pTransferSerial, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryAccountregister(this: *mut ::std::os::raw::c_void, pAccountregister: *mut CThostFtdcAccountregisterField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_accountregister(pAccountregister, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspError(this: *mut ::std::os::raw::c_void, pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_error(pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRtnOrder(this: *mut ::std::os::raw::c_void, pOrder: *mut CThostFtdcOrderField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rtn_order(pOrder);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRtnTrade(this: *mut ::std::os::raw::c_void, pTrade: *mut CThostFtdcTradeField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rtn_trade(pTrade);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnErrRtnOrderInsert(this: *mut ::std::os::raw::c_void, pInputOrder: *mut CThostFtdcInputOrderField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_err_rtn_order_insert(pInputOrder, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnErrRtnOrderAction(this: *mut ::std::os::raw::c_void, pOrderAction: *mut CThostFtdcOrderActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_err_rtn_order_action(pOrderAction, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRtnInstrumentStatus(this: *mut ::std::os::raw::c_void, pInstrumentStatus: *mut CThostFtdcInstrumentStatusField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rtn_instrument_status(pInstrumentStatus);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRtnTradingNotice(this: *mut ::std::os::raw::c_void, pTradingNoticeInfo: *mut CThostFtdcTradingNoticeInfoField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rtn_trading_notice(pTradingNoticeInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRtnErrorConditionalOrder(this: *mut ::std::os::raw::c_void, pErrorConditionalOrder: *mut CThostFtdcErrorConditionalOrderField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rtn_error_conditional_order(pErrorConditionalOrder);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRtnExecOrder(this: *mut ::std::os::raw::c_void, pExecOrder: *mut CThostFtdcExecOrderField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rtn_exec_order(pExecOrder);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnErrRtnExecOrderInsert(this: *mut ::std::os::raw::c_void, pInputExecOrder: *mut CThostFtdcInputExecOrderField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_err_rtn_exec_order_insert(pInputExecOrder, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnErrRtnExecOrderAction(this: *mut ::std::os::raw::c_void, pExecOrderAction: *mut CThostFtdcExecOrderActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_err_rtn_exec_order_action(pExecOrderAction, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnErrRtnForQuoteInsert(this: *mut ::std::os::raw::c_void, pInputForQuote: *mut CThostFtdcInputForQuoteField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_err_rtn_for_quote_insert(pInputForQuote, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRtnQuote(this: *mut ::std::os::raw::c_void, pQuote: *mut CThostFtdcQuoteField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rtn_quote(pQuote);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnErrRtnQuoteInsert(this: *mut ::std::os::raw::c_void, pInputQuote: *mut CThostFtdcInputQuoteField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_err_rtn_quote_insert(pInputQuote, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnErrRtnQuoteAction(this: *mut ::std::os::raw::c_void, pQuoteAction: *mut CThostFtdcQuoteActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_err_rtn_quote_action(pQuoteAction, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRtnForQuoteRsp(this: *mut ::std::os::raw::c_void, pForQuoteRsp: *mut CThostFtdcForQuoteRspField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rtn_for_quote_rsp(pForQuoteRsp);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnErrRtnBatchOrderAction(this: *mut ::std::os::raw::c_void, pBatchOrderAction: *mut CThostFtdcBatchOrderActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_err_rtn_batch_order_action(pBatchOrderAction, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRtnOptionSelfClose(this: *mut ::std::os::raw::c_void, pOptionSelfClose: *mut CThostFtdcOptionSelfCloseField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rtn_option_self_close(pOptionSelfClose);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnErrRtnOptionSelfCloseInsert(this: *mut ::std::os::raw::c_void, pInputOptionSelfClose: *mut CThostFtdcInputOptionSelfCloseField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_err_rtn_option_self_close_insert(pInputOptionSelfClose, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnErrRtnOptionSelfCloseAction(this: *mut ::std::os::raw::c_void, pOptionSelfCloseAction: *mut CThostFtdcOptionSelfCloseActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_err_rtn_option_self_close_action(pOptionSelfCloseAction, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryContractBank(this: *mut ::std::os::raw::c_void, pContractBank: *mut CThostFtdcContractBankField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_contract_bank(pContractBank, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryParkedOrder(this: *mut ::std::os::raw::c_void, pParkedOrder: *mut CThostFtdcParkedOrderField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_parked_order(pParkedOrder, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryParkedOrderAction(this: *mut ::std::os::raw::c_void, pParkedOrderAction: *mut CThostFtdcParkedOrderActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_parked_order_action(pParkedOrderAction, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryTradingNotice(this: *mut ::std::os::raw::c_void, pTradingNotice: *mut CThostFtdcTradingNoticeField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_trading_notice(pTradingNotice, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryBrokerTradingParams(this: *mut ::std::os::raw::c_void, pBrokerTradingParams: *mut CThostFtdcBrokerTradingParamsField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_broker_trading_params(pBrokerTradingParams, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryBrokerTradingAlgos(this: *mut ::std::os::raw::c_void, pBrokerTradingAlgos: *mut CThostFtdcBrokerTradingAlgosField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_broker_trading_algos(pBrokerTradingAlgos, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQryInstrumentOrderCommRate(this: *mut ::std::os::raw::c_void, pInstrumentOrderCommRate: *mut CThostFtdcInstrumentOrderCommRateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_qry_instrument_order_comm_rate(pInstrumentOrderCommRate, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRtnFromBankToFutureByBank(this: *mut ::std::os::raw::c_void, pRspTransfer: *mut CThostFtdcRspTransferField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rtn_from_bank_to_future_by_bank(pRspTransfer);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRtnFromFutureToBankByBank(this: *mut ::std::os::raw::c_void, pRspTransfer: *mut CThostFtdcRspTransferField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rtn_from_future_to_bank_by_bank(pRspTransfer);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRtnRepealFromBankToFutureByBank(this: *mut ::std::os::raw::c_void, pRspRepeal: *mut CThostFtdcRspRepealField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rtn_repeal_from_bank_to_future_by_bank(pRspRepeal);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRtnRepealFromFutureToBankByBank(this: *mut ::std::os::raw::c_void, pRspRepeal: *mut CThostFtdcRspRepealField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rtn_repeal_from_future_to_bank_by_bank(pRspRepeal);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRtnFromBankToFutureByFuture(this: *mut ::std::os::raw::c_void, pRspTransfer: *mut CThostFtdcRspTransferField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rtn_from_bank_to_future_by_future(pRspTransfer);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRtnFromFutureToBankByFuture(this: *mut ::std::os::raw::c_void, pRspTransfer: *mut CThostFtdcRspTransferField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rtn_from_future_to_bank_by_future(pRspTransfer);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRtnRepealFromBankToFutureByFutureManual(this: *mut ::std::os::raw::c_void, pRspRepeal: *mut CThostFtdcRspRepealField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rtn_repeal_from_bank_to_future_by_future_manual(pRspRepeal);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRtnRepealFromFutureToBankByFutureManual(this: *mut ::std::os::raw::c_void, pRspRepeal: *mut CThostFtdcRspRepealField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rtn_repeal_from_future_to_bank_by_future_manual(pRspRepeal);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRtnQueryBankBalanceByFuture(this: *mut ::std::os::raw::c_void, pNotifyQueryAccount: *mut CThostFtdcNotifyQueryAccountField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rtn_query_bank_balance_by_future(pNotifyQueryAccount);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnErrRtnBankToFutureByFuture(this: *mut ::std::os::raw::c_void, pReqTransfer: *mut CThostFtdcReqTransferField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_err_rtn_bank_to_future_by_future(pReqTransfer, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnErrRtnFutureToBankByFuture(this: *mut ::std::os::raw::c_void, pReqTransfer: *mut CThostFtdcReqTransferField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_err_rtn_future_to_bank_by_future(pReqTransfer, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnErrRtnRepealBankToFutureByFutureManual(this: *mut ::std::os::raw::c_void, pReqRepeal: *mut CThostFtdcReqRepealField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_err_rtn_repeal_bank_to_future_by_future_manual(pReqRepeal, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnErrRtnRepealFutureToBankByFutureManual(this: *mut ::std::os::raw::c_void, pReqRepeal: *mut CThostFtdcReqRepealField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_err_rtn_repeal_future_to_bank_by_future_manual(pReqRepeal, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnErrRtnQueryBankBalanceByFuture(this: *mut ::std::os::raw::c_void, pReqQueryAccount: *mut CThostFtdcReqQueryAccountField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_err_rtn_query_bank_balance_by_future(pReqQueryAccount, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRtnRepealFromBankToFutureByFuture(this: *mut ::std::os::raw::c_void, pRspRepeal: *mut CThostFtdcRspRepealField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rtn_repeal_from_bank_to_future_by_future(pRspRepeal);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRtnRepealFromFutureToBankByFuture(this: *mut ::std::os::raw::c_void, pRspRepeal: *mut CThostFtdcRspRepealField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rtn_repeal_from_future_to_bank_by_future(pRspRepeal);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspFromBankToFutureByFuture(this: *mut ::std::os::raw::c_void, pReqTransfer: *mut CThostFtdcReqTransferField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_from_bank_to_future_by_future(pReqTransfer, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspFromFutureToBankByFuture(this: *mut ::std::os::raw::c_void, pReqTransfer: *mut CThostFtdcReqTransferField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_from_future_to_bank_by_future(pReqTransfer, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspQueryBankAccountMoneyByFuture(this: *mut ::std::os::raw::c_void, pReqQueryAccount: *mut CThostFtdcReqQueryAccountField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: *mut bool ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rsp_query_bank_account_money_by_future(pReqQueryAccount, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRtnOpenAccountByBank(this: *mut ::std::os::raw::c_void, pOpenAccount: *mut CThostFtdcOpenAccountField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rtn_open_account_by_bank(pOpenAccount);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRtnCancelAccountByBank(this: *mut ::std::os::raw::c_void, pCancelAccount: *mut CThostFtdcCancelAccountField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rtn_cancel_account_by_bank(pCancelAccount);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRtnChangeAccountByBank(this: *mut ::std::os::raw::c_void, pChangeAccount: *mut CThostFtdcChangeAccountField  ) {
    let instance =  **(this as *mut *mut &mut dyn CtpMiniTdApi);
    instance.on_rtn_change_account_by_bank(pChangeAccount);
}       
