use core::ffi::c_void;
use core::ptr::slice_from_raw_parts;
use core::sync::atomic::{AtomicI32, Ordering};

use std::borrow::Cow;
use std::cmp::max;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::sync::Arc;

use chrono::{Date, NaiveDateTime, Utc, Timelike};

use crate::constants::{Direction, Exchange, Offset, OrderType, Status};
use crate::ctp::sys::*;
use crate::interface::Interface;
use crate::structs::{
    AccountData, CancelRequest, ContractData, LoginForm, OrderData, OrderRequest, PositionData,
    TradeData,
};
use crate::types::message::TdApiMessage;
use crate::util::blocker::Blocker;
use crate::util::channel::GroupSender;
use crate::util::hash::HashMap;
use crate::{get_interface_path, get_home_path};
use std::fs::create_dir;
use std::path::PathBuf;
use std::process::id;
use bitflags::_core::ops::Deref;

const POS_LONG: u8 = THOST_FTDC_PD_Long as u8;
const POS_SHORT: u8 = THOST_FTDC_PD_Short as u8;

unsafe fn get_trader_spi<'a>(spi: *mut c_void) -> &'a mut dyn TdCallApi {
    **(spi as *mut *mut &mut dyn TdCallApi)
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnFrontConnected(this: *mut ::std::os::raw::c_void) {
    let x = get_trader_spi(this);
    x.on_front_connected();
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnFrontDisconnected(
    this: *mut ::std::os::raw::c_void,
    nReason: c_int,
) {
    let x = get_trader_spi(this);
    x.on_front_disconnected(nReason);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnHeartBeatWarning(
    this: *mut ::std::os::raw::c_void,
    nTimeLapse: c_int,
) {
    let x = get_trader_spi(this);
    x.on_heart_beat_warning(nTimeLapse);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspAuthenticate(
    this: *mut ::std::os::raw::c_void,
    pRspAuthenticateField: *mut CThostFtdcRspAuthenticateField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_authenticate(pRspAuthenticateField, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspUserLogin(
    this: *mut ::std::os::raw::c_void,
    pRspUserLogin: *mut CThostFtdcRspUserLoginField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_user_login(pRspUserLogin, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspUserLogout(
    this: *mut ::std::os::raw::c_void,
    pUserLogout: *mut CThostFtdcUserLogoutField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_user_logout(pUserLogout, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspUserPasswordUpdate(
    this: *mut ::std::os::raw::c_void,
    pUserPasswordUpdate: *mut CThostFtdcUserPasswordUpdateField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_user_password_update(pUserPasswordUpdate, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspTradingAccountPasswordUpdate(
    this: *mut ::std::os::raw::c_void,
    pTradingAccountPasswordUpdate: *mut CThostFtdcTradingAccountPasswordUpdateField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_trading_account_password_update(
        pTradingAccountPasswordUpdate,
        pRspInfo,
        nRequestID,
        bIsLast,
    );
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspUserAuthMethod(
    this: *mut ::std::os::raw::c_void,
    pRspUserAuthMethod: *mut CThostFtdcRspUserAuthMethodField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_user_auth_method(pRspUserAuthMethod, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspGenUserCaptcha(
    this: *mut ::std::os::raw::c_void,
    pRspGenUserCaptcha: *mut CThostFtdcRspGenUserCaptchaField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_gen_user_captcha(pRspGenUserCaptcha, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspGenUserText(
    this: *mut ::std::os::raw::c_void,
    pRspGenUserText: *mut CThostFtdcRspGenUserTextField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_gen_user_text(pRspGenUserText, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspOrderInsert(
    this: *mut ::std::os::raw::c_void,
    pInputOrder: *mut CThostFtdcInputOrderField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_order_insert(pInputOrder, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspParkedOrderInsert(
    this: *mut ::std::os::raw::c_void,
    pParkedOrder: *mut CThostFtdcParkedOrderField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_parked_order_insert(pParkedOrder, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspParkedOrderAction(
    this: *mut ::std::os::raw::c_void,
    pParkedOrderAction: *mut CThostFtdcParkedOrderActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_parked_order_action(pParkedOrderAction, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspOrderAction(
    this: *mut ::std::os::raw::c_void,
    pInputOrderAction: *mut CThostFtdcInputOrderActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_order_action(pInputOrderAction, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQueryMaxOrderVolume(
    this: *mut ::std::os::raw::c_void,
    pQueryMaxOrderVolume: *mut CThostFtdcQueryMaxOrderVolumeField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_query_max_order_volume(pQueryMaxOrderVolume, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspSettlementInfoConfirm(
    this: *mut ::std::os::raw::c_void,
    pSettlementInfoConfirm: *mut CThostFtdcSettlementInfoConfirmField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_settlement_info_confirm(pSettlementInfoConfirm, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspRemoveParkedOrder(
    this: *mut ::std::os::raw::c_void,
    pRemoveParkedOrder: *mut CThostFtdcRemoveParkedOrderField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_remove_parked_order(pRemoveParkedOrder, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspRemoveParkedOrderAction(
    this: *mut ::std::os::raw::c_void,
    pRemoveParkedOrderAction: *mut CThostFtdcRemoveParkedOrderActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_remove_parked_order_action(pRemoveParkedOrderAction, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspExecOrderInsert(
    this: *mut ::std::os::raw::c_void,
    pInputExecOrder: *mut CThostFtdcInputExecOrderField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_exec_order_insert(pInputExecOrder, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspExecOrderAction(
    this: *mut ::std::os::raw::c_void,
    pInputExecOrderAction: *mut CThostFtdcInputExecOrderActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_exec_order_action(pInputExecOrderAction, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspForQuoteInsert(
    this: *mut ::std::os::raw::c_void,
    pInputForQuote: *mut CThostFtdcInputForQuoteField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_for_quote_insert(pInputForQuote, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQuoteInsert(
    this: *mut ::std::os::raw::c_void,
    pInputQuote: *mut CThostFtdcInputQuoteField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_quote_insert(pInputQuote, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQuoteAction(
    this: *mut ::std::os::raw::c_void,
    pInputQuoteAction: *mut CThostFtdcInputQuoteActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_quote_action(pInputQuoteAction, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspBatchOrderAction(
    this: *mut ::std::os::raw::c_void,
    pInputBatchOrderAction: *mut CThostFtdcInputBatchOrderActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_batch_order_action(pInputBatchOrderAction, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspOptionSelfCloseInsert(
    this: *mut ::std::os::raw::c_void,
    pInputOptionSelfClose: *mut CThostFtdcInputOptionSelfCloseField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_option_self_close_insert(pInputOptionSelfClose, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspOptionSelfCloseAction(
    this: *mut ::std::os::raw::c_void,
    pInputOptionSelfCloseAction: *mut CThostFtdcInputOptionSelfCloseActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_option_self_close_action(pInputOptionSelfCloseAction, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspCombActionInsert(
    this: *mut ::std::os::raw::c_void,
    pInputCombAction: *mut CThostFtdcInputCombActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_comb_action_insert(pInputCombAction, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryOrder(
    this: *mut ::std::os::raw::c_void,
    pOrder: *mut CThostFtdcOrderField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_order(pOrder, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryTrade(
    this: *mut ::std::os::raw::c_void,
    pTrade: *mut CThostFtdcTradeField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_trade(pTrade, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryInvestorPosition(
    this: *mut ::std::os::raw::c_void,
    pInvestorPosition: *mut CThostFtdcInvestorPositionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_investor_position(pInvestorPosition, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryTradingAccount(
    this: *mut ::std::os::raw::c_void,
    pTradingAccount: *mut CThostFtdcTradingAccountField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_trading_account(pTradingAccount, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryInvestor(
    this: *mut ::std::os::raw::c_void,
    pInvestor: *mut CThostFtdcInvestorField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_investor(pInvestor, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryTradingCode(
    this: *mut ::std::os::raw::c_void,
    pTradingCode: *mut CThostFtdcTradingCodeField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_trading_code(pTradingCode, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryInstrumentMarginRate(
    this: *mut ::std::os::raw::c_void,
    pInstrumentMarginRate: *mut CThostFtdcInstrumentMarginRateField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_instrument_margin_rate(pInstrumentMarginRate, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryInstrumentCommissionRate(
    this: *mut ::std::os::raw::c_void,
    pInstrumentCommissionRate: *mut CThostFtdcInstrumentCommissionRateField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_instrument_commission_rate(
        pInstrumentCommissionRate,
        pRspInfo,
        nRequestID,
        bIsLast,
    );
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryExchange(
    this: *mut ::std::os::raw::c_void,
    pExchange: *mut CThostFtdcExchangeField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_exchange(pExchange, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryProduct(
    this: *mut ::std::os::raw::c_void,
    pProduct: *mut CThostFtdcProductField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_product(pProduct, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryInstrument(
    this: *mut ::std::os::raw::c_void,
    pInstrument: *mut CThostFtdcInstrumentField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_instrument(pInstrument, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryDepthMarketData(
    this: *mut ::std::os::raw::c_void,
    pDepthMarketData: *mut CThostFtdcDepthMarketDataField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_depth_market_data(pDepthMarketData, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQrySettlementInfo(
    this: *mut ::std::os::raw::c_void,
    pSettlementInfo: *mut CThostFtdcSettlementInfoField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_settlement_info(pSettlementInfo, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryTransferBank(
    this: *mut ::std::os::raw::c_void,
    pTransferBank: *mut CThostFtdcTransferBankField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_transfer_bank(pTransferBank, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryInvestorPositionDetail(
    this: *mut ::std::os::raw::c_void,
    pInvestorPositionDetail: *mut CThostFtdcInvestorPositionDetailField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_investor_position_detail(pInvestorPositionDetail, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryNotice(
    this: *mut ::std::os::raw::c_void,
    pNotice: *mut CThostFtdcNoticeField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_notice(pNotice, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQrySettlementInfoConfirm(
    this: *mut ::std::os::raw::c_void,
    pSettlementInfoConfirm: *mut CThostFtdcSettlementInfoConfirmField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_settlement_info_confirm(pSettlementInfoConfirm, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryInvestorPositionCombineDetail(
    this: *mut ::std::os::raw::c_void,
    pInvestorPositionCombineDetail: *mut CThostFtdcInvestorPositionCombineDetailField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_investor_position_combine_detail(
        pInvestorPositionCombineDetail,
        pRspInfo,
        nRequestID,
        bIsLast,
    );
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryCFMMCTradingAccountKey(
    this: *mut ::std::os::raw::c_void,
    pCFMMCTradingAccountKey: *mut CThostFtdcCFMMCTradingAccountKeyField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_c_f_m_m_c_trading_account_key(
        pCFMMCTradingAccountKey,
        pRspInfo,
        nRequestID,
        bIsLast,
    );
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryEWarrantOffset(
    this: *mut ::std::os::raw::c_void,
    pEWarrantOffset: *mut CThostFtdcEWarrantOffsetField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_e_warrant_offset(pEWarrantOffset, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryInvestorProductGroupMargin(
    this: *mut ::std::os::raw::c_void,
    pInvestorProductGroupMargin: *mut CThostFtdcInvestorProductGroupMarginField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_investor_product_group_margin(
        pInvestorProductGroupMargin,
        pRspInfo,
        nRequestID,
        bIsLast,
    );
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryExchangeMarginRate(
    this: *mut ::std::os::raw::c_void,
    pExchangeMarginRate: *mut CThostFtdcExchangeMarginRateField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_exchange_margin_rate(pExchangeMarginRate, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryExchangeMarginRateAdjust(
    this: *mut ::std::os::raw::c_void,
    pExchangeMarginRateAdjust: *mut CThostFtdcExchangeMarginRateAdjustField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_exchange_margin_rate_adjust(
        pExchangeMarginRateAdjust,
        pRspInfo,
        nRequestID,
        bIsLast,
    );
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryExchangeRate(
    this: *mut ::std::os::raw::c_void,
    pExchangeRate: *mut CThostFtdcExchangeRateField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_exchange_rate(pExchangeRate, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQrySecAgentACIDMap(
    this: *mut ::std::os::raw::c_void,
    pSecAgentACIDMap: *mut CThostFtdcSecAgentACIDMapField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_sec_agent_a_c_i_d_map(pSecAgentACIDMap, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryProductExchRate(
    this: *mut ::std::os::raw::c_void,
    pProductExchRate: *mut CThostFtdcProductExchRateField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_product_exch_rate(pProductExchRate, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryProductGroup(
    this: *mut ::std::os::raw::c_void,
    pProductGroup: *mut CThostFtdcProductGroupField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_product_group(pProductGroup, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryMMInstrumentCommissionRate(
    this: *mut ::std::os::raw::c_void,
    pMMInstrumentCommissionRate: *mut CThostFtdcMMInstrumentCommissionRateField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_m_m_instrument_commission_rate(
        pMMInstrumentCommissionRate,
        pRspInfo,
        nRequestID,
        bIsLast,
    );
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryMMOptionInstrCommRate(
    this: *mut ::std::os::raw::c_void,
    pMMOptionInstrCommRate: *mut CThostFtdcMMOptionInstrCommRateField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_m_m_option_instr_comm_rate(pMMOptionInstrCommRate, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryInstrumentOrderCommRate(
    this: *mut ::std::os::raw::c_void,
    pInstrumentOrderCommRate: *mut CThostFtdcInstrumentOrderCommRateField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_instrument_order_comm_rate(
        pInstrumentOrderCommRate,
        pRspInfo,
        nRequestID,
        bIsLast,
    );
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQrySecAgentTradingAccount(
    this: *mut ::std::os::raw::c_void,
    pTradingAccount: *mut CThostFtdcTradingAccountField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_sec_agent_trading_account(pTradingAccount, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQrySecAgentCheckMode(
    this: *mut ::std::os::raw::c_void,
    pSecAgentCheckMode: *mut CThostFtdcSecAgentCheckModeField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_sec_agent_check_mode(pSecAgentCheckMode, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQrySecAgentTradeInfo(
    this: *mut ::std::os::raw::c_void,
    pSecAgentTradeInfo: *mut CThostFtdcSecAgentTradeInfoField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_sec_agent_trade_info(pSecAgentTradeInfo, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryOptionInstrTradeCost(
    this: *mut ::std::os::raw::c_void,
    pOptionInstrTradeCost: *mut CThostFtdcOptionInstrTradeCostField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_option_instr_trade_cost(pOptionInstrTradeCost, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryOptionInstrCommRate(
    this: *mut ::std::os::raw::c_void,
    pOptionInstrCommRate: *mut CThostFtdcOptionInstrCommRateField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_option_instr_comm_rate(pOptionInstrCommRate, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryExecOrder(
    this: *mut ::std::os::raw::c_void,
    pExecOrder: *mut CThostFtdcExecOrderField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_exec_order(pExecOrder, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryForQuote(
    this: *mut ::std::os::raw::c_void,
    pForQuote: *mut CThostFtdcForQuoteField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_for_quote(pForQuote, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryQuote(
    this: *mut ::std::os::raw::c_void,
    pQuote: *mut CThostFtdcQuoteField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_quote(pQuote, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryOptionSelfClose(
    this: *mut ::std::os::raw::c_void,
    pOptionSelfClose: *mut CThostFtdcOptionSelfCloseField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_option_self_close(pOptionSelfClose, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryInvestUnit(
    this: *mut ::std::os::raw::c_void,
    pInvestUnit: *mut CThostFtdcInvestUnitField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_invest_unit(pInvestUnit, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryCombInstrumentGuard(
    this: *mut ::std::os::raw::c_void,
    pCombInstrumentGuard: *mut CThostFtdcCombInstrumentGuardField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_comb_instrument_guard(pCombInstrumentGuard, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryCombAction(
    this: *mut ::std::os::raw::c_void,
    pCombAction: *mut CThostFtdcCombActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_comb_action(pCombAction, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryTransferSerial(
    this: *mut ::std::os::raw::c_void,
    pTransferSerial: *mut CThostFtdcTransferSerialField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_transfer_serial(pTransferSerial, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryAccountregister(
    this: *mut ::std::os::raw::c_void,
    pAccountregister: *mut CThostFtdcAccountregisterField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_accountregister(pAccountregister, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspError(
    this: *mut ::std::os::raw::c_void,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_error(pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnOrder(
    this: *mut ::std::os::raw::c_void,
    pOrder: *mut CThostFtdcOrderField,
) {
    let x = get_trader_spi(this);
    x.on_rtn_order(pOrder);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnTrade(
    this: *mut ::std::os::raw::c_void,
    pTrade: *mut CThostFtdcTradeField,
) {
    let x = get_trader_spi(this);
    x.on_rtn_trade(pTrade);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnOrderInsert(
    this: *mut ::std::os::raw::c_void,
    pInputOrder: *mut CThostFtdcInputOrderField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) {
    let x = get_trader_spi(this);
    x.on_err_rtn_order_insert(pInputOrder, pRspInfo);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnOrderAction(
    this: *mut ::std::os::raw::c_void,
    pOrderAction: *mut CThostFtdcOrderActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) {
    let x = get_trader_spi(this);
    x.on_err_rtn_order_action(pOrderAction, pRspInfo);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnInstrumentStatus(
    this: *mut ::std::os::raw::c_void,
    pInstrumentStatus: *mut CThostFtdcInstrumentStatusField,
) {
    let x = get_trader_spi(this);
    x.on_rtn_instrument_status(pInstrumentStatus);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnBulletin(
    this: *mut ::std::os::raw::c_void,
    pBulletin: *mut CThostFtdcBulletinField,
) {
    let x = get_trader_spi(this);
    x.on_rtn_bulletin(pBulletin);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnTradingNotice(
    this: *mut ::std::os::raw::c_void,
    pTradingNoticeInfo: *mut CThostFtdcTradingNoticeInfoField,
) {
    let x = get_trader_spi(this);
    x.on_rtn_trading_notice(pTradingNoticeInfo);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnErrorConditionalOrder(
    this: *mut ::std::os::raw::c_void,
    pErrorConditionalOrder: *mut CThostFtdcErrorConditionalOrderField,
) {
    let x = get_trader_spi(this);
    x.on_rtn_error_conditional_order(pErrorConditionalOrder);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnExecOrder(
    this: *mut ::std::os::raw::c_void,
    pExecOrder: *mut CThostFtdcExecOrderField,
) {
    let x = get_trader_spi(this);
    x.on_rtn_exec_order(pExecOrder);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnExecOrderInsert(
    this: *mut ::std::os::raw::c_void,
    pInputExecOrder: *mut CThostFtdcInputExecOrderField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) {
    let x = get_trader_spi(this);
    x.on_err_rtn_exec_order_insert(pInputExecOrder, pRspInfo);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnExecOrderAction(
    this: *mut ::std::os::raw::c_void,
    pExecOrderAction: *mut CThostFtdcExecOrderActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) {
    let x = get_trader_spi(this);
    x.on_err_rtn_exec_order_action(pExecOrderAction, pRspInfo);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnForQuoteInsert(
    this: *mut ::std::os::raw::c_void,
    pInputForQuote: *mut CThostFtdcInputForQuoteField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) {
    let x = get_trader_spi(this);
    x.on_err_rtn_for_quote_insert(pInputForQuote, pRspInfo);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnQuote(
    this: *mut ::std::os::raw::c_void,
    pQuote: *mut CThostFtdcQuoteField,
) {
    let x = get_trader_spi(this);
    x.on_rtn_quote(pQuote);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnQuoteInsert(
    this: *mut ::std::os::raw::c_void,
    pInputQuote: *mut CThostFtdcInputQuoteField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) {
    let x = get_trader_spi(this);
    x.on_err_rtn_quote_insert(pInputQuote, pRspInfo);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnQuoteAction(
    this: *mut ::std::os::raw::c_void,
    pQuoteAction: *mut CThostFtdcQuoteActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) {
    let x = get_trader_spi(this);
    x.on_err_rtn_quote_action(pQuoteAction, pRspInfo);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnForQuoteRsp(
    this: *mut ::std::os::raw::c_void,
    pForQuoteRsp: *mut CThostFtdcForQuoteRspField,
) {
    let x = get_trader_spi(this);
    x.on_rtn_for_quote_rsp(pForQuoteRsp);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnCFMMCTradingAccountToken(
    this: *mut ::std::os::raw::c_void,
    pCFMMCTradingAccountToken: *mut CThostFtdcCFMMCTradingAccountTokenField,
) {
    let x = get_trader_spi(this);
    x.on_rtn_c_f_m_m_c_trading_account_token(pCFMMCTradingAccountToken);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnBatchOrderAction(
    this: *mut ::std::os::raw::c_void,
    pBatchOrderAction: *mut CThostFtdcBatchOrderActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) {
    let x = get_trader_spi(this);
    x.on_err_rtn_batch_order_action(pBatchOrderAction, pRspInfo);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnOptionSelfClose(
    this: *mut ::std::os::raw::c_void,
    pOptionSelfClose: *mut CThostFtdcOptionSelfCloseField,
) {
    let x = get_trader_spi(this);
    x.on_rtn_option_self_close(pOptionSelfClose);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnOptionSelfCloseInsert(
    this: *mut ::std::os::raw::c_void,
    pInputOptionSelfClose: *mut CThostFtdcInputOptionSelfCloseField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) {
    let x = get_trader_spi(this);
    x.on_err_rtn_option_self_close_insert(pInputOptionSelfClose, pRspInfo);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnOptionSelfCloseAction(
    this: *mut ::std::os::raw::c_void,
    pOptionSelfCloseAction: *mut CThostFtdcOptionSelfCloseActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) {
    let x = get_trader_spi(this);
    x.on_err_rtn_option_self_close_action(pOptionSelfCloseAction, pRspInfo);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnCombAction(
    this: *mut ::std::os::raw::c_void,
    pCombAction: *mut CThostFtdcCombActionField,
) {
    let x = get_trader_spi(this);
    x.on_rtn_comb_action(pCombAction);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnCombActionInsert(
    this: *mut ::std::os::raw::c_void,
    pInputCombAction: *mut CThostFtdcInputCombActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) {
    let x = get_trader_spi(this);
    x.on_err_rtn_comb_action_insert(pInputCombAction, pRspInfo);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryContractBank(
    this: *mut ::std::os::raw::c_void,
    pContractBank: *mut CThostFtdcContractBankField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_contract_bank(pContractBank, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryParkedOrder(
    this: *mut ::std::os::raw::c_void,
    pParkedOrder: *mut CThostFtdcParkedOrderField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_parked_order(pParkedOrder, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryParkedOrderAction(
    this: *mut ::std::os::raw::c_void,
    pParkedOrderAction: *mut CThostFtdcParkedOrderActionField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_parked_order_action(pParkedOrderAction, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryTradingNotice(
    this: *mut ::std::os::raw::c_void,
    pTradingNotice: *mut CThostFtdcTradingNoticeField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_trading_notice(pTradingNotice, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryBrokerTradingParams(
    this: *mut ::std::os::raw::c_void,
    pBrokerTradingParams: *mut CThostFtdcBrokerTradingParamsField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_broker_trading_params(pBrokerTradingParams, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQryBrokerTradingAlgos(
    this: *mut ::std::os::raw::c_void,
    pBrokerTradingAlgos: *mut CThostFtdcBrokerTradingAlgosField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_qry_broker_trading_algos(pBrokerTradingAlgos, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQueryCFMMCTradingAccountToken(
    this: *mut ::std::os::raw::c_void,
    pQueryCFMMCTradingAccountToken: *mut CThostFtdcQueryCFMMCTradingAccountTokenField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_query_c_f_m_m_c_trading_account_token(
        pQueryCFMMCTradingAccountToken,
        pRspInfo,
        nRequestID,
        bIsLast,
    );
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnFromBankToFutureByBank(
    this: *mut ::std::os::raw::c_void,
    pRspTransfer: *mut CThostFtdcRspTransferField,
) {
    let x = get_trader_spi(this);
    x.on_rtn_from_bank_to_future_by_bank(pRspTransfer);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnFromFutureToBankByBank(
    this: *mut ::std::os::raw::c_void,
    pRspTransfer: *mut CThostFtdcRspTransferField,
) {
    let x = get_trader_spi(this);
    x.on_rtn_from_future_to_bank_by_bank(pRspTransfer);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnRepealFromBankToFutureByBank(
    this: *mut ::std::os::raw::c_void,
    pRspRepeal: *mut CThostFtdcRspRepealField,
) {
    let x = get_trader_spi(this);
    x.on_rtn_repeal_from_bank_to_future_by_bank(pRspRepeal);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnRepealFromFutureToBankByBank(
    this: *mut ::std::os::raw::c_void,
    pRspRepeal: *mut CThostFtdcRspRepealField,
) {
    let x = get_trader_spi(this);
    x.on_rtn_repeal_from_future_to_bank_by_bank(pRspRepeal);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnFromBankToFutureByFuture(
    this: *mut ::std::os::raw::c_void,
    pRspTransfer: *mut CThostFtdcRspTransferField,
) {
    let x = get_trader_spi(this);
    x.on_rtn_from_bank_to_future_by_future(pRspTransfer);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnFromFutureToBankByFuture(
    this: *mut ::std::os::raw::c_void,
    pRspTransfer: *mut CThostFtdcRspTransferField,
) {
    let x = get_trader_spi(this);
    x.on_rtn_from_future_to_bank_by_future(pRspTransfer);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnRepealFromBankToFutureByFutureManual(
    this: *mut ::std::os::raw::c_void,
    pRspRepeal: *mut CThostFtdcRspRepealField,
) {
    let x = get_trader_spi(this);
    x.on_rtn_repeal_from_bank_to_future_by_future_manual(pRspRepeal);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnRepealFromFutureToBankByFutureManual(
    this: *mut ::std::os::raw::c_void,
    pRspRepeal: *mut CThostFtdcRspRepealField,
) {
    let x = get_trader_spi(this);
    x.on_rtn_repeal_from_future_to_bank_by_future_manual(pRspRepeal);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnQueryBankBalanceByFuture(
    this: *mut ::std::os::raw::c_void,
    pNotifyQueryAccount: *mut CThostFtdcNotifyQueryAccountField,
) {
    let x = get_trader_spi(this);
    x.on_rtn_query_bank_balance_by_future(pNotifyQueryAccount);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnBankToFutureByFuture(
    this: *mut ::std::os::raw::c_void,
    pReqTransfer: *mut CThostFtdcReqTransferField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) {
    let x = get_trader_spi(this);
    x.on_err_rtn_bank_to_future_by_future(pReqTransfer, pRspInfo);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnFutureToBankByFuture(
    this: *mut ::std::os::raw::c_void,
    pReqTransfer: *mut CThostFtdcReqTransferField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) {
    let x = get_trader_spi(this);
    x.on_err_rtn_future_to_bank_by_future(pReqTransfer, pRspInfo);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnRepealBankToFutureByFutureManual(
    this: *mut ::std::os::raw::c_void,
    pReqRepeal: *mut CThostFtdcReqRepealField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) {
    let x = get_trader_spi(this);
    x.on_err_rtn_repeal_bank_to_future_by_future_manual(pReqRepeal, pRspInfo);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnRepealFutureToBankByFutureManual(
    this: *mut ::std::os::raw::c_void,
    pReqRepeal: *mut CThostFtdcReqRepealField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) {
    let x = get_trader_spi(this);
    x.on_err_rtn_repeal_future_to_bank_by_future_manual(pReqRepeal, pRspInfo);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnErrRtnQueryBankBalanceByFuture(
    this: *mut ::std::os::raw::c_void,
    pReqQueryAccount: *mut CThostFtdcReqQueryAccountField,
    pRspInfo: *mut CThostFtdcRspInfoField,
) {
    let x = get_trader_spi(this);
    x.on_err_rtn_query_bank_balance_by_future(pReqQueryAccount, pRspInfo);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnRepealFromBankToFutureByFuture(
    this: *mut ::std::os::raw::c_void,
    pRspRepeal: *mut CThostFtdcRspRepealField,
) {
    let x = get_trader_spi(this);
    x.on_rtn_repeal_from_bank_to_future_by_future(pRspRepeal);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnRepealFromFutureToBankByFuture(
    this: *mut ::std::os::raw::c_void,
    pRspRepeal: *mut CThostFtdcRspRepealField,
) {
    let x = get_trader_spi(this);
    x.on_rtn_repeal_from_future_to_bank_by_future(pRspRepeal);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspFromBankToFutureByFuture(
    this: *mut ::std::os::raw::c_void,
    pReqTransfer: *mut CThostFtdcReqTransferField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_from_bank_to_future_by_future(pReqTransfer, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspFromFutureToBankByFuture(
    this: *mut ::std::os::raw::c_void,
    pReqTransfer: *mut CThostFtdcReqTransferField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_from_future_to_bank_by_future(pReqTransfer, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRspQueryBankAccountMoneyByFuture(
    this: *mut ::std::os::raw::c_void,
    pReqQueryAccount: *mut CThostFtdcReqQueryAccountField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: bool,
) {
    let x = get_trader_spi(this);
    x.on_rsp_query_bank_account_money_by_future(pReqQueryAccount, pRspInfo, nRequestID, bIsLast);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnOpenAccountByBank(
    this: *mut ::std::os::raw::c_void,
    pOpenAccount: *mut CThostFtdcOpenAccountField,
) {
    let x = get_trader_spi(this);
    x.on_rtn_open_account_by_bank(pOpenAccount);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnCancelAccountByBank(
    this: *mut ::std::os::raw::c_void,
    pCancelAccount: *mut CThostFtdcCancelAccountField,
) {
    let x = get_trader_spi(this);
    x.on_rtn_cancel_account_by_bank(pCancelAccount);
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn RustCtpOnRtnChangeAccountByBank(
    this: *mut ::std::os::raw::c_void,
    pChangeAccount: *mut CThostFtdcChangeAccountField,
) {
    let x = get_trader_spi(this);
    x.on_rtn_change_account_by_bank(pChangeAccount);
}

///  交易回调API
/// 此处应该天有很多地方的回调方法
/// 比如on_front_connected等， 此处的代码应该被python的cpp_generator快速生成
pub trait TdCallApi {
    fn on_front_connected(&mut self) {
        println!("function callback: OnFrontConnected");
    }

    fn on_front_disconnected(&mut self, nReason: c_int) {
        println!("function callback: OnFrontDisconnected");
    }

    fn on_heart_beat_warning(&mut self, nTimeLapse: c_int) {
        println!("function callback: OnHeartBeatWarning");
    }

    fn on_rsp_authenticate(
        &mut self,
        pRspAuthenticateField: *mut CThostFtdcRspAuthenticateField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspAuthenticate");
    }

    /// # Safety
    unsafe fn on_rsp_user_login(
        &mut self,
        pRspUserLogin: *mut CThostFtdcRspUserLoginField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspUserLogin");
    }

    fn on_rsp_user_logout(
        &mut self,
        pUserLogout: *mut CThostFtdcUserLogoutField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspUserLogout");
    }

    fn on_rsp_user_password_update(
        &mut self,
        pUserPasswordUpdate: *mut CThostFtdcUserPasswordUpdateField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspUserPasswordUpdate");
    }

    fn on_rsp_trading_account_password_update(
        &mut self,
        pTradingAccountPasswordUpdate: *mut CThostFtdcTradingAccountPasswordUpdateField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspTradingAccountPasswordUpdate");
    }

    fn on_rsp_user_auth_method(
        &mut self,
        pRspUserAuthMethod: *mut CThostFtdcRspUserAuthMethodField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspUserAuthMethod");
    }

    fn on_rsp_gen_user_captcha(
        &mut self,
        pRspGenUserCaptcha: *mut CThostFtdcRspGenUserCaptchaField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspGenUserCaptcha");
    }

    fn on_rsp_gen_user_text(
        &mut self,
        pRspGenUserText: *mut CThostFtdcRspGenUserTextField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspGenUserText");
    }

    fn on_rsp_order_insert(
        &mut self,
        pInputOrder: *mut CThostFtdcInputOrderField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspOrderInsert");
    }

    fn on_rsp_parked_order_insert(
        &mut self,
        pParkedOrder: *mut CThostFtdcParkedOrderField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspParkedOrderInsert");
    }

    fn on_rsp_parked_order_action(
        &mut self,
        pParkedOrderAction: *mut CThostFtdcParkedOrderActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspParkedOrderAction");
    }

    fn on_rsp_order_action(
        &mut self,
        pInputOrderAction: *mut CThostFtdcInputOrderActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspOrderAction");
    }

    fn on_rsp_query_max_order_volume(
        &mut self,
        pQueryMaxOrderVolume: *mut CThostFtdcQueryMaxOrderVolumeField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQueryMaxOrderVolume");
    }

    fn on_rsp_settlement_info_confirm(
        &mut self,
        pSettlementInfoConfirm: *mut CThostFtdcSettlementInfoConfirmField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspSettlementInfoConfirm");
    }

    fn on_rsp_remove_parked_order(
        &mut self,
        pRemoveParkedOrder: *mut CThostFtdcRemoveParkedOrderField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspRemoveParkedOrder");
    }

    fn on_rsp_remove_parked_order_action(
        &mut self,
        pRemoveParkedOrderAction: *mut CThostFtdcRemoveParkedOrderActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspRemoveParkedOrderAction");
    }

    fn on_rsp_exec_order_insert(
        &mut self,
        pInputExecOrder: *mut CThostFtdcInputExecOrderField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspExecOrderInsert");
    }

    fn on_rsp_exec_order_action(
        &mut self,
        pInputExecOrderAction: *mut CThostFtdcInputExecOrderActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspExecOrderAction");
    }

    fn on_rsp_for_quote_insert(
        &mut self,
        pInputForQuote: *mut CThostFtdcInputForQuoteField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspForQuoteInsert");
    }

    fn on_rsp_quote_insert(
        &mut self,
        pInputQuote: *mut CThostFtdcInputQuoteField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQuoteInsert");
    }

    fn on_rsp_quote_action(
        &mut self,
        pInputQuoteAction: *mut CThostFtdcInputQuoteActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQuoteAction");
    }

    fn on_rsp_batch_order_action(
        &mut self,
        pInputBatchOrderAction: *mut CThostFtdcInputBatchOrderActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspBatchOrderAction");
    }

    fn on_rsp_option_self_close_insert(
        &mut self,
        pInputOptionSelfClose: *mut CThostFtdcInputOptionSelfCloseField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspOptionSelfCloseInsert");
    }

    fn on_rsp_option_self_close_action(
        &mut self,
        pInputOptionSelfCloseAction: *mut CThostFtdcInputOptionSelfCloseActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspOptionSelfCloseAction");
    }

    fn on_rsp_comb_action_insert(
        &mut self,
        pInputCombAction: *mut CThostFtdcInputCombActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspCombActionInsert");
    }

    fn on_rsp_qry_order(
        &mut self,
        pOrder: *mut CThostFtdcOrderField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryOrder");
    }

    fn on_rsp_qry_trade(
        &mut self,
        pTrade: *mut CThostFtdcTradeField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryTrade");
    }

    /// # Safety
    unsafe fn on_rsp_qry_investor_position(
        &mut self,
        pInvestorPosition: *mut CThostFtdcInvestorPositionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryInvestorPosition");
    }

    /// # Safety
    unsafe fn on_rsp_qry_trading_account(
        &mut self,
        pTradingAccount: *mut CThostFtdcTradingAccountField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryTradingAccount");
    }

    fn on_rsp_qry_investor(
        &mut self,
        pInvestor: *mut CThostFtdcInvestorField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryInvestor");
    }

    fn on_rsp_qry_trading_code(
        &mut self,
        pTradingCode: *mut CThostFtdcTradingCodeField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryTradingCode");
    }

    /// # Safety
    unsafe fn on_rsp_qry_instrument_margin_rate(
        &mut self,
        pInstrumentMarginRate: *mut CThostFtdcInstrumentMarginRateField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryInstrumentMarginRate");
    }

    /// # Safety
    unsafe fn on_rsp_qry_instrument_commission_rate(
        &mut self,
        pInstrumentCommissionRate: *mut CThostFtdcInstrumentCommissionRateField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryInstrumentCommissionRate");
    }

    fn on_rsp_qry_exchange(
        &mut self,
        pExchange: *mut CThostFtdcExchangeField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryExchange");
    }

    fn on_rsp_qry_product(
        &mut self,
        pProduct: *mut CThostFtdcProductField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryProduct");
    }

    /// # Safety
    unsafe fn on_rsp_qry_instrument(
        &mut self,
        pInstrument: *mut CThostFtdcInstrumentField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryInstrument");
    }

    fn on_rsp_qry_depth_market_data(
        &mut self,
        pDepthMarketData: *mut CThostFtdcDepthMarketDataField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryDepthMarketData");
    }

    fn on_rsp_qry_settlement_info(
        &mut self,
        pSettlementInfo: *mut CThostFtdcSettlementInfoField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQrySettlementInfo");
    }

    fn on_rsp_qry_transfer_bank(
        &mut self,
        pTransferBank: *mut CThostFtdcTransferBankField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryTransferBank");
    }

    fn on_rsp_qry_investor_position_detail(
        &mut self,
        pInvestorPositionDetail: *mut CThostFtdcInvestorPositionDetailField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryInvestorPositionDetail");
    }

    fn on_rsp_qry_notice(
        &mut self,
        pNotice: *mut CThostFtdcNoticeField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryNotice");
    }

    fn on_rsp_qry_settlement_info_confirm(
        &mut self,
        pSettlementInfoConfirm: *mut CThostFtdcSettlementInfoConfirmField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQrySettlementInfoConfirm");
    }

    fn on_rsp_qry_investor_position_combine_detail(
        &mut self,
        pInvestorPositionCombineDetail: *mut CThostFtdcInvestorPositionCombineDetailField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryInvestorPositionCombineDetail");
    }

    fn on_rsp_qry_c_f_m_m_c_trading_account_key(
        &mut self,
        pCFMMCTradingAccountKey: *mut CThostFtdcCFMMCTradingAccountKeyField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryCFMMCTradingAccountKey");
    }

    fn on_rsp_qry_e_warrant_offset(
        &mut self,
        pEWarrantOffset: *mut CThostFtdcEWarrantOffsetField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryEWarrantOffset");
    }

    fn on_rsp_qry_investor_product_group_margin(
        &mut self,
        pInvestorProductGroupMargin: *mut CThostFtdcInvestorProductGroupMarginField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryInvestorProductGroupMargin");
    }

    fn on_rsp_qry_exchange_margin_rate(
        &mut self,
        pExchangeMarginRate: *mut CThostFtdcExchangeMarginRateField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryExchangeMarginRate");
    }

    fn on_rsp_qry_exchange_margin_rate_adjust(
        &mut self,
        pExchangeMarginRateAdjust: *mut CThostFtdcExchangeMarginRateAdjustField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryExchangeMarginRateAdjust");
    }

    fn on_rsp_qry_exchange_rate(
        &mut self,
        pExchangeRate: *mut CThostFtdcExchangeRateField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryExchangeRate");
    }

    fn on_rsp_qry_sec_agent_a_c_i_d_map(
        &mut self,
        pSecAgentACIDMap: *mut CThostFtdcSecAgentACIDMapField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQrySecAgentACIDMap");
    }

    fn on_rsp_qry_product_exch_rate(
        &mut self,
        pProductExchRate: *mut CThostFtdcProductExchRateField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryProductExchRate");
    }

    fn on_rsp_qry_product_group(
        &mut self,
        pProductGroup: *mut CThostFtdcProductGroupField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryProductGroup");
    }

    fn on_rsp_qry_m_m_instrument_commission_rate(
        &mut self,
        pMMInstrumentCommissionRate: *mut CThostFtdcMMInstrumentCommissionRateField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryMMInstrumentCommissionRate");
    }

    fn on_rsp_qry_m_m_option_instr_comm_rate(
        &mut self,
        pMMOptionInstrCommRate: *mut CThostFtdcMMOptionInstrCommRateField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryMMOptionInstrCommRate");
    }

    fn on_rsp_qry_instrument_order_comm_rate(
        &mut self,
        pInstrumentOrderCommRate: *mut CThostFtdcInstrumentOrderCommRateField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryInstrumentOrderCommRate");
    }

    fn on_rsp_qry_sec_agent_trading_account(
        &mut self,
        pTradingAccount: *mut CThostFtdcTradingAccountField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQrySecAgentTradingAccount");
    }

    fn on_rsp_qry_sec_agent_check_mode(
        &mut self,
        pSecAgentCheckMode: *mut CThostFtdcSecAgentCheckModeField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQrySecAgentCheckMode");
    }

    fn on_rsp_qry_sec_agent_trade_info(
        &mut self,
        pSecAgentTradeInfo: *mut CThostFtdcSecAgentTradeInfoField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQrySecAgentTradeInfo");
    }

    fn on_rsp_qry_option_instr_trade_cost(
        &mut self,
        pOptionInstrTradeCost: *mut CThostFtdcOptionInstrTradeCostField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryOptionInstrTradeCost");
    }

    fn on_rsp_qry_option_instr_comm_rate(
        &mut self,
        pOptionInstrCommRate: *mut CThostFtdcOptionInstrCommRateField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryOptionInstrCommRate");
    }

    fn on_rsp_qry_exec_order(
        &mut self,
        pExecOrder: *mut CThostFtdcExecOrderField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryExecOrder");
    }

    fn on_rsp_qry_for_quote(
        &mut self,
        pForQuote: *mut CThostFtdcForQuoteField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryForQuote");
    }

    fn on_rsp_qry_quote(
        &mut self,
        pQuote: *mut CThostFtdcQuoteField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryQuote");
    }

    fn on_rsp_qry_option_self_close(
        &mut self,
        pOptionSelfClose: *mut CThostFtdcOptionSelfCloseField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryOptionSelfClose");
    }

    fn on_rsp_qry_invest_unit(
        &mut self,
        pInvestUnit: *mut CThostFtdcInvestUnitField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryInvestUnit");
    }

    fn on_rsp_qry_comb_instrument_guard(
        &mut self,
        pCombInstrumentGuard: *mut CThostFtdcCombInstrumentGuardField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryCombInstrumentGuard");
    }

    fn on_rsp_qry_comb_action(
        &mut self,
        pCombAction: *mut CThostFtdcCombActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryCombAction");
    }

    fn on_rsp_qry_transfer_serial(
        &mut self,
        pTransferSerial: *mut CThostFtdcTransferSerialField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryTransferSerial");
    }

    fn on_rsp_qry_accountregister(
        &mut self,
        pAccountregister: *mut CThostFtdcAccountregisterField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryAccountregister");
    }

    fn on_rsp_error(
        &mut self,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspError");
    }

    /// # Safety
    unsafe fn on_rtn_order(&mut self, pOrder: *mut CThostFtdcOrderField) {
        println!("function callback: OnRtnOrder");
    }

    /// # Safety
    unsafe fn on_rtn_trade(&mut self, pTrade: *mut CThostFtdcTradeField) {
        println!("function callback: OnRtnTrade");
    }

    /// # Safety
    unsafe fn on_err_rtn_order_insert(
        &mut self,
        pInputOrder: *mut CThostFtdcInputOrderField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) {
        println!("function callback: OnErrRtnOrderInsert");
    }

    fn on_err_rtn_order_action(
        &mut self,
        pOrderAction: *mut CThostFtdcOrderActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) {
        println!("function callback: OnErrRtnOrderAction");
    }

    fn on_rtn_instrument_status(
        &mut self,
        pInstrumentStatus: *mut CThostFtdcInstrumentStatusField,
    ) {
        println!("function callback: OnRtnInstrumentStatus");
    }

    fn on_rtn_bulletin(&mut self, pBulletin: *mut CThostFtdcBulletinField) {
        println!("function callback: OnRtnBulletin");
    }

    fn on_rtn_trading_notice(&mut self, pTradingNoticeInfo: *mut CThostFtdcTradingNoticeInfoField) {}

    fn on_rtn_error_conditional_order(
        &mut self,
        pErrorConditionalOrder: *mut CThostFtdcErrorConditionalOrderField,
    ) {
        println!("function callback: OnRtnErrorConditionalOrder");
    }

    fn on_rtn_exec_order(&mut self, pExecOrder: *mut CThostFtdcExecOrderField) {
        println!("function callback: OnRtnExecOrder");
    }

    fn on_err_rtn_exec_order_insert(
        &mut self,
        pInputExecOrder: *mut CThostFtdcInputExecOrderField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) {
        println!("function callback: OnErrRtnExecOrderInsert");
    }

    fn on_err_rtn_exec_order_action(
        &mut self,
        pExecOrderAction: *mut CThostFtdcExecOrderActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) {
        println!("function callback: OnErrRtnExecOrderAction");
    }

    fn on_err_rtn_for_quote_insert(
        &mut self,
        pInputForQuote: *mut CThostFtdcInputForQuoteField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) {
        println!("function callback: OnErrRtnForQuoteInsert");
    }

    fn on_rtn_quote(&mut self, pQuote: *mut CThostFtdcQuoteField) {
        println!("function callback: OnRtnQuote");
    }

    fn on_err_rtn_quote_insert(
        &mut self,
        pInputQuote: *mut CThostFtdcInputQuoteField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) {
        println!("function callback: OnErrRtnQuoteInsert");
    }

    fn on_err_rtn_quote_action(
        &mut self,
        pQuoteAction: *mut CThostFtdcQuoteActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) {
        println!("function callback: OnErrRtnQuoteAction");
    }

    fn on_rtn_for_quote_rsp(&mut self, pForQuoteRsp: *mut CThostFtdcForQuoteRspField) {
        println!("function callback: OnRtnForQuoteRsp");
    }

    fn on_rtn_c_f_m_m_c_trading_account_token(
        &mut self,
        pCFMMCTradingAccountToken: *mut CThostFtdcCFMMCTradingAccountTokenField,
    ) {
        println!("function callback: OnRtnCFMMCTradingAccountToken");
    }

    fn on_err_rtn_batch_order_action(
        &mut self,
        pBatchOrderAction: *mut CThostFtdcBatchOrderActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) {
        println!("function callback: OnErrRtnBatchOrderAction");
    }

    fn on_rtn_option_self_close(&mut self, pOptionSelfClose: *mut CThostFtdcOptionSelfCloseField) {
        println!("function callback: OnRtnOptionSelfClose");
    }

    fn on_err_rtn_option_self_close_insert(
        &mut self,
        pInputOptionSelfClose: *mut CThostFtdcInputOptionSelfCloseField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) {
        println!("function callback: OnErrRtnOptionSelfCloseInsert");
    }

    fn on_err_rtn_option_self_close_action(
        &mut self,
        pOptionSelfCloseAction: *mut CThostFtdcOptionSelfCloseActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) {
        println!("function callback: OnErrRtnOptionSelfCloseAction");
    }

    fn on_rtn_comb_action(&mut self, pCombAction: *mut CThostFtdcCombActionField) {
        println!("function callback: OnRtnCombAction");
    }

    fn on_err_rtn_comb_action_insert(
        &mut self,
        pInputCombAction: *mut CThostFtdcInputCombActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) {
        println!("function callback: OnErrRtnCombActionInsert");
    }

    fn on_rsp_qry_contract_bank(
        &mut self,
        pContractBank: *mut CThostFtdcContractBankField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryContractBank");
    }

    fn on_rsp_qry_parked_order(
        &mut self,
        pParkedOrder: *mut CThostFtdcParkedOrderField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryParkedOrder");
    }

    fn on_rsp_qry_parked_order_action(
        &mut self,
        pParkedOrderAction: *mut CThostFtdcParkedOrderActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryParkedOrderAction");
    }

    fn on_rsp_qry_trading_notice(
        &mut self,
        pTradingNotice: *mut CThostFtdcTradingNoticeField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryTradingNotice");
    }

    fn on_rsp_qry_broker_trading_params(
        &mut self,
        pBrokerTradingParams: *mut CThostFtdcBrokerTradingParamsField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryBrokerTradingParams");
    }

    fn on_rsp_qry_broker_trading_algos(
        &mut self,
        pBrokerTradingAlgos: *mut CThostFtdcBrokerTradingAlgosField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQryBrokerTradingAlgos");
    }

    fn on_rsp_query_c_f_m_m_c_trading_account_token(
        &mut self,
        pQueryCFMMCTradingAccountToken: *mut CThostFtdcQueryCFMMCTradingAccountTokenField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQueryCFMMCTradingAccountToken");
    }

    fn on_rtn_from_bank_to_future_by_bank(
        &mut self,
        pRspTransfer: *mut CThostFtdcRspTransferField,
    ) {
        println!("function callback: OnRtnFromBankToFutureByBank");
    }

    fn on_rtn_from_future_to_bank_by_bank(
        &mut self,
        pRspTransfer: *mut CThostFtdcRspTransferField,
    ) {
        println!("function callback: OnRtnFromFutureToBankByBank");
    }

    fn on_rtn_repeal_from_bank_to_future_by_bank(
        &mut self,
        pRspRepeal: *mut CThostFtdcRspRepealField,
    ) {
        println!("function callback: OnRtnRepealFromBankToFutureByBank");
    }

    fn on_rtn_repeal_from_future_to_bank_by_bank(
        &mut self,
        pRspRepeal: *mut CThostFtdcRspRepealField,
    ) {
        println!("function callback: OnRtnRepealFromFutureToBankByBank");
    }

    fn on_rtn_from_bank_to_future_by_future(
        &mut self,
        pRspTransfer: *mut CThostFtdcRspTransferField,
    ) {
        println!("function callback: OnRtnFromBankToFutureByFuture");
    }

    fn on_rtn_from_future_to_bank_by_future(
        &mut self,
        pRspTransfer: *mut CThostFtdcRspTransferField,
    ) {
        println!("function callback: OnRtnFromFutureToBankByFuture");
    }

    fn on_rtn_repeal_from_bank_to_future_by_future_manual(
        &mut self,
        pRspRepeal: *mut CThostFtdcRspRepealField,
    ) {
        println!("function callback: OnRtnRepealFromBankToFutureByFutureManual");
    }

    fn on_rtn_repeal_from_future_to_bank_by_future_manual(
        &mut self,
        pRspRepeal: *mut CThostFtdcRspRepealField,
    ) {
        println!("function callback: OnRtnRepealFromFutureToBankByFutureManual");
    }

    fn on_rtn_query_bank_balance_by_future(
        &mut self,
        pNotifyQueryAccount: *mut CThostFtdcNotifyQueryAccountField,
    ) {
        println!("function callback: OnRtnQueryBankBalanceByFuture");
    }

    fn on_err_rtn_bank_to_future_by_future(
        &mut self,
        pReqTransfer: *mut CThostFtdcReqTransferField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) {
        println!("function callback: OnErrRtnBankToFutureByFuture");
    }

    fn on_err_rtn_future_to_bank_by_future(
        &mut self,
        pReqTransfer: *mut CThostFtdcReqTransferField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) {
        println!("function callback: OnErrRtnFutureToBankByFuture");
    }

    fn on_err_rtn_repeal_bank_to_future_by_future_manual(
        &mut self,
        pReqRepeal: *mut CThostFtdcReqRepealField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) {
        println!("function callback: OnErrRtnRepealBankToFutureByFutureManual");
    }

    fn on_err_rtn_repeal_future_to_bank_by_future_manual(
        &mut self,
        pReqRepeal: *mut CThostFtdcReqRepealField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) {
        println!("function callback: OnErrRtnRepealFutureToBankByFutureManual");
    }

    fn on_err_rtn_query_bank_balance_by_future(
        &mut self,
        pReqQueryAccount: *mut CThostFtdcReqQueryAccountField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) {
        println!("function callback: OnErrRtnQueryBankBalanceByFuture");
    }

    fn on_rtn_repeal_from_bank_to_future_by_future(
        &mut self,
        pRspRepeal: *mut CThostFtdcRspRepealField,
    ) {
        println!("function callback: OnRtnRepealFromBankToFutureByFuture");
    }

    fn on_rtn_repeal_from_future_to_bank_by_future(
        &mut self,
        pRspRepeal: *mut CThostFtdcRspRepealField,
    ) {
        println!("function callback: OnRtnRepealFromFutureToBankByFuture");
    }

    fn on_rsp_from_bank_to_future_by_future(
        &mut self,
        pReqTransfer: *mut CThostFtdcReqTransferField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspFromBankToFutureByFuture");
    }

    fn on_rsp_from_future_to_bank_by_future(
        &mut self,
        pReqTransfer: *mut CThostFtdcReqTransferField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspFromFutureToBankByFuture");
    }

    fn on_rsp_query_bank_account_money_by_future(
        &mut self,
        pReqQueryAccount: *mut CThostFtdcReqQueryAccountField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        println!("function callback: OnRspQueryBankAccountMoneyByFuture");
    }

    fn on_rtn_open_account_by_bank(&mut self, pOpenAccount: *mut CThostFtdcOpenAccountField) {
        println!("function callback: OnRtnOpenAccountByBank");
    }

    fn on_rtn_cancel_account_by_bank(&mut self, pCancelAccount: *mut CThostFtdcCancelAccountField) {
        println!("function callback: OnRtnCancelAccountByBank");
    }

    fn on_rtn_change_account_by_bank(&mut self, pChangeAccount: *mut CThostFtdcChangeAccountField) {
        println!("function callback: OnRtnChangeAccountByBank");
    }
}

/// 交易API
pub struct TdApi {
    data_collector: CallDataCollector,
    request_id: i32,
    front_id: i32,
    session_id: i32,
}

impl TdApi {
    pub(crate) fn session_id(&self) -> i32 {
        self.session_id
    }

    pub(crate) fn front_id(&self) -> i32 {
        self.front_id
    }
}

struct TdApiBlockerInner {
    front_id: AtomicI32,
    session_id: AtomicI32,
    step1: Blocker,
    step2: Blocker,
    step3: Blocker,
    step4: Blocker,
    step5: Blocker,
}

struct TdApiBlocker(Arc<TdApiBlockerInner>);

impl Clone for TdApiBlocker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}


impl TdApiBlocker {
    fn new() -> Self {
        Self(Arc::new(TdApiBlockerInner {
            front_id: AtomicI32::new(0),
            session_id: AtomicI32::new(0),
            step1: Default::default(),
            step2: Default::default(),
            step3: Default::default(),
            step4: Default::default(),
            step5: Default::default(),
        }))
    }
}

pub struct CallDataCollector {
    session_id: i32,

    login_status: bool,
    connect_status: bool,
    sender: GroupSender<TdApiMessage>,
    blocker: Option<TdApiBlocker>,
    pos: HashMap<Cow<'static, str>, PositionData>,
    size_map: HashMap<Cow<'static, str>, f64>,
    exchange_map: HashMap<Cow<'static, str>, Exchange>,
    order_ref: Arc<AtomicI32>,
    trade_pointer: *mut CThostFtdcTraderApi,
    login_form: LoginForm,
    symbols: Vec<&'static str>,
    pub request_id: i32,
    pub front_id: i32,
}


impl TdCallApi for CallDataCollector {
    fn on_front_connected(&mut self) {
        println!(">>> Td Front Connected");
        self.blocker.as_ref().unwrap().0.step1.unblock();
    }

    fn on_front_disconnected(&mut self, nReason: c_int) {
        // todo: 打印连接失败的原因
        self.blocker = Option::from(TdApiBlocker::new());
        println!("function callback: OnFrontDisconnected");
    }

    fn on_rsp_authenticate(
        &mut self,
        pRspAuthenticateField: *mut CThostFtdcRspAuthenticateField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        match get_rsp_info(pRspInfo) {
            Ok(t) => {
                println!(">>> Td Auth successful");
                self.blocker.as_ref().unwrap().0.step2.unblock();
            }
            Err(e) => {
                println!(">>> Td Auth failed, id: {} msg: {}", e.id, e.msg);
            }
        }
    }

    unsafe fn on_rsp_user_login(
        &mut self,
        pRspUserLogin: *mut CThostFtdcRspUserLoginField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        match get_rsp_info(pRspInfo) {
            Ok(t) => {
                let login = *pRspUserLogin;

                println!(">>> Td Login successful");

                println!(
                    "session {} frontid: {}",
                    login.SessionID as f64, login.FrontID as f64
                );

                let blocker = self.blocker.as_ref().unwrap();

                blocker.0.front_id.store(login.FrontID, Ordering::SeqCst);
                blocker
                    .0
                    .session_id
                    .store(login.SessionID, Ordering::SeqCst);

                blocker.0.step3.unblock();
            }
            Err(e) => {
                println!(">>> Td Login failed, id: {} msg: {}", e.id, e.msg);
            }
        }
    }

    fn on_rsp_order_insert(
        &mut self,
        pInputOrder: *mut CThostFtdcInputOrderField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {}

    fn on_rsp_order_action(
        &mut self,
        pInputOrderAction: *mut CThostFtdcInputOrderActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        match get_rsp_info(pRspInfo) {
            Ok(t) => {}
            Err(e) => {
                println!(">>> Order action error, id: {} msg: {}", e.id, e.msg);
            }
        }
    }

    fn on_rsp_settlement_info_confirm(
        &mut self,
        pSettlementInfoConfirm: *mut CThostFtdcSettlementInfoConfirmField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        match get_rsp_info(pRspInfo) {
            Ok(t) => {
                println!(">>> Td Confirmed Successful");
            }
            Err(e) => {
                println!(">>> Td Confirmed failed, id: {} msg: {}", e.id, e.msg);
            }
        }
    }

    unsafe fn on_rsp_qry_investor_position(
        &mut self,
        pInvestorPosition: *mut CThostFtdcInvestorPositionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        if pInvestorPosition.is_null() {} else {
            let position = *pInvestorPosition;
            let symbol = slice_to_string(&position.InstrumentID);
            let open_cost = position.OpenCost;
            let direction = Direction::from(position.PosiDirection);
            let exchange = *self.exchange_map.get(symbol.as_str()).unwrap();
            let td_pos = position.TodayPosition as f64;
            let volume = position.Position as f64;
            let yd_pos = position.YdPosition as f64;
            let profit = position.PositionProfit;
            let frozen = position.ShortFrozen + position.LongFrozen;
            let key = format!("{}_{}", symbol, position.PosiDirection);

            let pos = self
                .pos
                .entry(Cow::from(key))
                .or_insert_with(|| match direction {
                    Direction::SHORT => PositionData::new_with_short(&symbol),
                    Direction::LONG => PositionData::new_with_long(&symbol),
                    _ => panic!("bad direction"),
                });
            // according to the exchange  to setup the yd position
            match exchange {
                Exchange::SHFE => {
                    if yd_pos != 0.0 && td_pos == 0.0 {
                        pos.yd_volume = volume;
                    }
                }
                _ => {
                    pos.yd_volume = volume - td_pos;
                }
            }
            let size = self.size_map.get(symbol.as_str()).unwrap();
            // pos.exchange = Some(*exchange);
            pos.price = (pos.price * pos.volume + open_cost / size) / (pos.volume + volume);
            pos.volume += volume;
            pos.pnl += profit;
            // if is the last data that been pushed,  take them and sent it the core
            if bIsLast {
                self.pos
                    .iter()
                    .for_each(|(k, v)| self.sender.send_all(v.to_owned()));
                self.pos.clear();
            }
        }
    }


    unsafe fn on_rsp_qry_trading_account(
        &mut self,
        pTradingAccount: *mut CThostFtdcTradingAccountField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        if !pTradingAccount.is_null() {
            let acc = *pTradingAccount;
            let account_data = AccountData {
                accountid: slice_to_string(&acc.AccountID),
                balance: acc.Balance,
                frozen: acc.FrozenMargin + acc.FrozenCash + acc.FrozenCommission,
                date: Utc::today(),
            };
            self.sender.send_all(account_data);
        } else {}
        if self.blocker.is_some() {
            self.blocker.take().unwrap().0.step5.unblock();
        }
    }

    unsafe fn on_rsp_qry_instrument(
        &mut self,
        pInstrument: *mut CThostFtdcInstrumentField,
        pRspInfo: *mut CThostFtdcRspInfoField,
        nRequestID: c_int,
        bIsLast: bool,
    ) {
        // todo: add Contract data in here and send it
        let instrument = *pInstrument;

        let contract = ContractData {
            symbol: slice_to_string(&instrument.InstrumentID),
            exchange: Some(Exchange::from(instrument.ExchangeID)),
            name: None,
            product: None,
            size: instrument.VolumeMultiple as f64,
            pricetick: instrument.PriceTick as f64,
            min_volume: 0.0,
            stop_supported: false,
            net_position: false,
            history_data: false,
            option_strike: 0.0,
            option_underlying: None,
            option_type: None,
            option_expiry: None,
            option_portfolio: None,
            option_index: None,
        };

        self.size_map
            .insert(Cow::Owned(contract.symbol.clone()), contract.size);
        self.exchange_map.insert(
            Cow::Owned(contract.symbol.clone()),
            contract.exchange.unwrap(),
        );
        self.sender.send_all(contract);

        if bIsLast {
            self.blocker.as_ref().unwrap().0.step4.unblock();
        }
    }

    unsafe fn on_rtn_order(&mut self, pOrder: *mut CThostFtdcOrderField) {
        let (order, idx) = {
            let order = *pOrder;
            let order_ref = slice_to_string(&order.OrderRef);
            let id = format!("{}_{}_{}", order.SessionID, order.FrontID, order_ref);
            let (idx, refs) = split_into_vec(order_ref.as_str());

            self.order_ref.fetch_max(refs, Ordering::SeqCst);
            let (date, time) = parse_datetime_from_str(order.InsertDate.as_ptr(),
                                                       order.InsertTime.as_ptr());
            let exchange = Exchange::from(order.ExchangeID);
            (
                OrderData {
                    symbol: slice_to_string(&order.InstrumentID),
                    exchange,
                    datetime: NaiveDateTime::new(date, time),
                    orderid: Option::from(id),
                    order_type: OrderType::from(order.OrderPriceType),
                    direction: Some(Direction::from(order.Direction)),
                    offset: Offset::from(order.CombOffsetFlag),
                    price: order.LimitPrice as f64,
                    volume: order.VolumeTotalOriginal as f64,
                    traded: order.VolumeTraded as f64,
                    status: Status::from(order.OrderStatus),
                },
                idx,
            )
        };
        // 这里控制接收order data的策略index
        match idx {
            10000000usize => {
                // Fixme: 如果不帶idx send_all or ignore it ?
                // println!("broadcast message");
                // self.sender.try_send_to(order, 0);
                // self.sender.send_all(order);
            }
            _ => {
                // todo: why order sender error
                self.sender.try_send_to(order, idx).unwrap_or(());
            }
        }
    }

    unsafe fn on_rtn_trade(&mut self, pTrade: *mut CThostFtdcTradeField) {
        let (trade, idx) = {
            let trade = *pTrade;
            let (date, time) = parse_datetime_from_str(trade.TradeDate.as_ptr(),
                                                       trade.TradeTime.as_ptr());
            let order_ref = slice_to_string(&trade.OrderRef);
            let (idx, refs) = split_into_vec(order_ref.as_str());
            (
                TradeData {
                    symbol: Cow::from(slice_to_string(&trade.InstrumentID)),
                    exchange: Some(Exchange::from(trade.ExchangeID)),
                    datetime: NaiveDateTime::new(date, time),
                    orderid: Option::from(order_ref),
                    direction: Some(Direction::from(trade.Direction)),
                    offset: Some(Offset::from(trade.OffsetFlag)),
                    price: trade.Price,
                    volume: trade.Volume,
                    tradeid: Some(slice_to_string(&trade.TradeID)),
                },
                idx,
            )
        };
        // 这里控制接收order data的策略index
        match idx {
            10000000usize => {
                // Fixme: 如果不帶idx send_all or ignore it ?
                // println!("broadcast message");
                // self.api.producer.send_all(trade)
            }
            _ => {
                self.sender.try_send_to(trade, idx).unwrap_or(());
            }
        }
    }

    unsafe fn on_err_rtn_order_insert(
        &mut self,
        pInputOrder: *mut CThostFtdcInputOrderField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) {
        let order = *pInputOrder;

        let order_id = slice_to_string(&order.OrderRef);
        println!("insert order err id: {}", order_id);
        let (idx, refs) = split_into_vec(order_id.as_str());

        // FixMe: 这里是否必须调用api?
        // self.api.order_ref = max(refs, self.api.order_ref);

        match get_rsp_info(pRspInfo) {
            Ok(t) => {}
            Err(e) => println!(">>> Order Insert failed, id: {} msg: {}", e.id, e.msg),
        }
    }

    fn on_err_rtn_order_action(
        &mut self,
        pOrderAction: *mut CThostFtdcOrderActionField,
        pRspInfo: *mut CThostFtdcRspInfoField,
    ) {
        match get_rsp_info(pRspInfo) {
            Ok(t) => {}
            Err(e) => {
                println!(">>> Order Action Err, id: {} msg: {}", e.id, e.msg);
            }
        };
    }

    fn on_rtn_instrument_status(
        &mut self,
        pInstrumentStatus: *mut CThostFtdcInstrumentStatusField,
    ) {}
}

fn get_order_type(order: OrderType) -> c_char {
    match order {
        OrderType::LIMIT => THOST_FTDC_OPT_LimitPrice as i8,
        OrderType::MARKET => THOST_FTDC_OPT_AnyPrice as i8,
        _ => panic!("This Interface do not support this order direction"),
    }
}

fn get_order_offset(offset: Offset) -> u8 {
    match offset {
        Offset::CLOSE => THOST_FTDC_OF_Close,
        Offset::CLOSETODAY => THOST_FTDC_OF_CloseToday,
        Offset::CLOSEYESTERDAY => THOST_FTDC_OF_CloseYesterday,
        Offset::OPEN => THOST_FTDC_OF_Open,
        _ => panic!("This Interface do not support this order direction"),
    }
}

fn get_order_exchange(exchange: Exchange) -> &'static str {
    match exchange {
        Exchange::SHFE => "SHFE",
        Exchange::DCE => "DCE",
        Exchange::CZCE => "CZCE",
        Exchange::INE => "INE",
        Exchange::CFFEX => "CFFEX",
        _ => unreachable!("This Interface do not support this exchange"),
    }
}

fn get_direction(direction: Direction) -> c_char {
    match direction {
        Direction::LONG => THOST_FTDC_D_Buy as i8,
        Direction::SHORT => THOST_FTDC_D_Sell as i8,
        _ => panic!("This Interface do not support this order direction"),
    }
}

impl From<[i8; 9]> for Exchange {
    fn from(i: [i8; 9]) -> Self {
        let exchange = slice_to_string(&i);
        match exchange.as_ref() {
            "SHFE" => Self::SHFE,
            "INE" => Self::INE,
            "CFFEX" => Self::CFFEX,
            "CZCE" => Self::CZCE,
            "DCE" => Self::DCE,
            _ => panic!("ctp do not support this exchange"),
        }
    }
}

impl From<[i8; 5]> for Offset {
    fn from(offset: [i8; 5]) -> Self {
        let x = *offset.first().unwrap() as u8;
        match x {
            THOST_FTDC_OF_Close => Self::CLOSE,
            THOST_FTDC_OF_CloseToday => Self::CLOSETODAY,
            THOST_FTDC_OF_CloseYesterday => Self::CLOSEYESTERDAY,
            THOST_FTDC_OF_Open => Self::OPEN,
            _ => panic!("ctp do not support this"),
        }
    }
}

impl From<i8> for Offset {
    fn from(offset: i8) -> Self {
        let x = offset as u8;
        match x {
            THOST_FTDC_OF_Close => Self::CLOSE,
            THOST_FTDC_OF_CloseToday => Self::CLOSETODAY,
            THOST_FTDC_OF_CloseYesterday => Self::CLOSEYESTERDAY,
            THOST_FTDC_OF_Open => Self::OPEN,
            _ => panic!("ctp do not support this"),
        }
    }
}

impl From<i8> for OrderType {
    fn from(i: i8) -> Self {
        match i as u8 {
            THOST_FTDC_OPT_LimitPrice => Self::LIMIT,
            THOST_FTDC_OPT_AnyPrice => Self::MARKET,
            _ => panic!("ctp do not support this ordertype"),
        }
    }
}

impl From<i8> for Status {
    fn from(i: i8) -> Self {
        match i as u8 {
            THOST_FTDC_OAS_Submitted => Status::SUBMITTING,
            THOST_FTDC_OAS_Accepted => Status::SUBMITTING,
            THOST_FTDC_OAS_Rejected => Status::REJECTED,
            THOST_FTDC_OST_NoTradeQueueing => Status::NOTTRADED,
            THOST_FTDC_OST_PartTradedQueueing => Status::PARTTRADED,
            THOST_FTDC_OST_AllTraded => Status::ALLTRADED,
            THOST_FTDC_OST_Canceled => Status::CANCELLED,
            _ => panic!("ctp do not support this status"),
        }
    }
}

impl From<i8> for Direction {
    fn from(i: i8) -> Self {
        match i as u8 {
            THOST_FTDC_D_Buy => Self::LONG,
            THOST_FTDC_D_Sell => Self::SHORT,
            POS_LONG => Self::LONG,
            POS_SHORT => Self::SHORT,
            _ => panic!("ctp do not support other direction {}", i),
        }
    }
}

impl CallDataCollector {
    pub fn auth(&mut self) {
        self.request_id += 1;
        let form = &self.login_form;
        let req = CThostFtdcReqAuthenticateField {
            UserID: form._user_id().to_c_slice(),
            BrokerID: form._broke_id().to_c_slice(),
            AuthCode: form._auth_code().to_c_slice(),
            AppID: form._app_id().to_c_slice(),
            UserProductInfo: form._production_info().to_c_slice(),
        };
        unsafe {
            RustCtpCallReqAuthenticate(
                self.trade_pointer,
                Box::into_raw(Box::new(req)) as *mut CThostFtdcReqAuthenticateField,
                self.request_id,
            )
        };
    }

    pub fn login(&mut self) {
        self.request_id += 1;
        let form = &(self.login_form);

        let login_req = CThostFtdcReqUserLoginField {
            BrokerID: form._broke_id().to_c_slice(),
            UserID: form._user_id().to_c_slice(),
            Password: form._password().to_c_slice(),
            UserProductInfo: form._production_info().to_c_slice(),
            ..CThostFtdcReqUserLoginField::default()
        };
        unsafe {
            RustCtpCallReqUserLogin(
                self.trade_pointer,
                Box::into_raw(Box::new(login_req)),
                self.request_id,
            )
        };
    }

    pub fn req_instrument(&mut self) {
        self.request_id += 1;
        unsafe {
            RustCtpCallReqQryInstrument(
                self.trade_pointer,
                Box::into_raw(Box::new(CThostFtdcQryInstrumentField::default())),
                self.request_id,
            );
        }
    }

    /// 注册交易前值
    fn register_fronted(&mut self, register_addr: CString) {
        let front_socket_address_ptr = register_addr.into_raw();
        unsafe { RustCtpCallRegisterFront(self.trade_pointer, front_socket_address_ptr) }
    }

    pub fn init(&mut self) -> bool {
        unsafe { RustCtpCallInit(self.trade_pointer) };
        true
    }

    fn register_spi(
        &mut self,
    ) {
        let trait_object_box: Box<Box<&mut TdCallApi>> = Box::new(Box::new(self));
        let trait_object_pointer =
            Box::into_raw(trait_object_box) as *mut Box<&mut dyn TdCallApi> as *mut c_void;
        // 把 rust对象 传给回调SPI
        let trader_spi = unsafe { FCtpTdSpi::new(trait_object_pointer) };
        let ptr = Box::into_raw(Box::new(trader_spi));
        unsafe {
            RustCtpCallSubscribePrivateTopic(self.trade_pointer, 0);
            RustCtpCallSubscribePublicTopic(self.trade_pointer, 0);
            RustCtpCallRegisterSpi(self.trade_pointer, ptr as *mut CThostFtdcTraderSpi)
        };
    }

    fn req_settle(&mut self) {
        self.request_id += 1;
        let form = &(self.login_form);
        let req = CThostFtdcSettlementInfoConfirmField {
            BrokerID: form._broke_id().to_c_slice(),
            InvestorID: form._user_id().to_c_slice(),
            ..CThostFtdcSettlementInfoConfirmField::default()
        };
        unsafe {
            RustCtpCallReqSettlementInfoConfirm(
                self.trade_pointer,
                Box::into_raw(Box::new(req)),
                self.request_id,
            );
        }
    }

    pub fn new(req: &LoginForm, symbols: Vec<&'static str>, sender: GroupSender<TdApiMessage>, trader_pointer: *mut CThostFtdcTraderApi) -> Self {
        let blocker = TdApiBlocker::new();
        CallDataCollector {
            session_id: 0,
            front_id: 0,
            request_id: 0,
            login_status: false,
            connect_status: false,
            sender: sender,
            blocker: Option::from(blocker),
            pos: Default::default(),
            size_map: Default::default(),
            exchange_map: Default::default(),
            order_ref: Arc::new(Default::default()),
            trade_pointer: trader_pointer,
            login_form: req.clone(),
            symbols,
        }
    }

    pub fn send_order_local(&mut self, idx: usize, order: OrderRequest) {
        self.request_id += 1;
        self.order_ref.fetch_add(1, Ordering::SeqCst);

        let form = &(self.login_form);
        let req = CThostFtdcInputOrderField {
            InstrumentID: order.symbol.as_str().to_c_slice(),
            LimitPrice: order.price,
            VolumeTotalOriginal: order.volume as c_int,
            OrderPriceType: get_order_type(order.order_type),
            Direction: get_direction(order.direction),
            UserID: form._user_id().to_c_slice(),
            InvestorID: form._user_id().to_c_slice(),
            BrokerID: form._broke_id().to_c_slice(),
            CombOffsetFlag: String::from_utf8(Vec::from([get_order_offset(order.offset)]))
                .unwrap()
                .to_c_slice(),
            OrderRef: format!("{:0>9}{:0>3}", self.order_ref.load(Ordering::SeqCst), idx).to_c_slice(),
            CombHedgeFlag: String::from_utf8(Vec::from([THOST_FTDC_HF_Speculation]))
                .unwrap()
                .to_c_slice(),
            ContingentCondition: THOST_FTDC_CC_Immediately as i8,
            ForceCloseReason: THOST_FTDC_FCC_NotForceClose as i8,
            IsAutoSuspend: 0 as c_int,
            TimeCondition: THOST_FTDC_TC_GFD as i8,
            VolumeCondition: THOST_FTDC_VC_AV as i8,
            MinVolume: 1 as c_int,
            ExchangeID: get_order_exchange(order.exchange).to_c_slice(),
            ..CThostFtdcInputOrderField::default()
        };
        unsafe {
            RustCtpCallReqOrderInsert(
                self.trade_pointer,
                Box::into_raw(Box::new(req)),
                self.request_id,
            )
        };
    }

    fn connect(&mut self) {

        // 阻塞器交给data collector
        self.register_spi();

        let addr = CString::new(self.login_form._td_address()).unwrap();

        self.register_fronted(addr);

        self.init();

        // 阻塞等待on_front_connected
        self.blocker.as_mut().unwrap().0.step1.block();

        self.auth();

        // 阻塞等待on_rsp_authenticate
        self.blocker.as_mut().unwrap().0.step2.block();
        self.login();

        // 阻塞等待on_rsp_user_login
        self.blocker.as_mut().unwrap().0.step3.block();

        // on_rsp_user_login 完成时会写入atomic i32至blocker，我们读取并赋予TdApi.
        self.session_id = self.blocker.as_mut().unwrap().0.session_id.load(Ordering::SeqCst);
        self.front_id = self.blocker.as_mut().unwrap().0.front_id.load(Ordering::SeqCst);

        self.req_settle();

        self.req_instrument();
        self.req_position();

        //阻塞等待合約查詢完畢
        self.blocker.as_mut().unwrap().0.step4.block();
        // println!("第四步解封");

        self.req_account();
        // 阻塞等待賬戶查詢完畢
        self.blocker.as_mut().unwrap().0.step5.block();
    }

    fn cancel(&mut self, req: CancelRequest) {
        self.request_id += 1;
        let exchange = req.exchange;
        let data = req
            .order_id
            .split('_')
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let form = &(self.login_form);
        let action = CThostFtdcInputOrderActionField {
            InstrumentID: req.symbol.to_c_slice(),
            OrderRef: data[2].to_c_slice(),
            FrontID: data[1].parse::<i32>().unwrap() as c_int,
            SessionID: data[0].parse::<i32>().unwrap() as c_int,
            ActionFlag: THOST_FTDC_AF_Delete as i8,
            BrokerID: form._broke_id().to_c_slice(),
            InvestorID: form._user_id().to_c_slice(),
            ExchangeID: get_order_exchange(exchange).to_c_slice(),
            ..CThostFtdcInputOrderActionField::default()
        };
        unsafe {
            RustCtpCallReqOrderAction(
                self.trade_pointer,
                Box::into_raw(Box::new(action)),
                self.request_id,
            );
        }
    }

    fn req_account(&mut self) {
        self.request_id += 1;
        unsafe {
            RustCtpCallReqQryTradingAccount(
                self.trade_pointer,
                Box::into_raw(Box::new(CThostFtdcQryTradingAccountField::default())),
                self.request_id,
            );
        }
    }

    fn req_position(&mut self) {
        self.request_id += 1;
        let login_info = &(self.login_form);
        unsafe {
            let req = CThostFtdcQryInvestorPositionField {
                BrokerID: login_info._broke_id().to_c_slice(),
                InvestorID: login_info._user_id().to_c_slice(),
                ..CThostFtdcQryInvestorPositionField::default()
            };
            RustCtpCallReqQryInvestorPosition(
                self.trade_pointer,
                Box::into_raw(Box::new(req)),
                self.request_id,
            );
        }
    }
}

impl Drop for TdApi {
    fn drop(&mut self) {}
}

impl Interface for TdApi {
    type Message = TdApiMessage;

    fn new(
        id: impl Into<Vec<u8>>,
        pwd: impl Into<Vec<u8>>,
        symbols: Vec<&'static str>,
        req: &LoginForm,
        sender: GroupSender<Self::Message>,
    ) -> Self {
        let home_path = get_home_path();
        let string = String::from_utf8(id.into()).unwrap();
        let path = home_path.to_string_lossy().to_string() + string.as_str() + "//";
        if !PathBuf::from(path.clone()).exists() {
            create_dir(path.clone()).expect("create dir failed ");
        }
        let p = CString::new(path).unwrap();
        let flow_path_ptr = p.as_ptr();
        let api = unsafe { CThostFtdcTraderApi::CreateFtdcTraderApi(flow_path_ptr) };
        TdApi {
            data_collector: CallDataCollector::new(req, symbols, sender, api),
            request_id: 0,
            front_id: 0,
            session_id: 0,
        }
    }

    fn send_order(&mut self, idx: usize, order: OrderRequest) {
        self.data_collector.send_order_local(idx, order)
    }

    fn cancel_order(&mut self, req: CancelRequest) {
        self.data_collector.cancel(req)
    }

    fn connect(&mut self) {
        // 建立线程阻塞器
        self.data_collector.connect()
    }

    fn req_account(&mut self) {
        self.data_collector.req_account();
    }

    fn req_position(&mut self) {
        self.data_collector.req_position();
    }
}
