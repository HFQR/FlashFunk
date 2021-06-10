use crate::ctpmini::sys::*;
use std::os::raw::c_int;

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnFrontDisconnected(
    this: *mut ::std::os::raw::c_void,
    nReason: c_int,
) {
    let instance = **(this as *mut *mut &mut dyn CtpMiniMdApi);
    instance.on_front_disconnected(nReason);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnHeartBeatWarning(
    this: *mut ::std::os::raw::c_void,
    nTimeLapse: c_int,
) {
    let instance = **(this as *mut *mut &mut dyn CtpMiniMdApi);
    instance.on_heart_beat_warning(nTimeLapse);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspUserLogin(
    this: *mut ::std::os::raw::c_void,
    pRspUserLogin: *mut CThostFtdcRspUserLoginField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: *mut bool,
) {
    let instance = **(this as *mut *mut &mut dyn CtpMiniMdApi);
    instance.on_rsp_user_login(pRspUserLogin, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspUserLogout(
    this: *mut ::std::os::raw::c_void,
    pUserLogout: *mut CThostFtdcUserLogoutField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: *mut bool,
) {
    let instance = **(this as *mut *mut &mut dyn CtpMiniMdApi);
    instance.on_rsp_user_logout(pUserLogout, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspError(
    this: *mut ::std::os::raw::c_void,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: *mut bool,
) {
    let instance = **(this as *mut *mut &mut dyn CtpMiniMdApi);
    instance.on_rsp_error(pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspSubMarketData(
    this: *mut ::std::os::raw::c_void,
    pSpecificInstrument: *mut CThostFtdcSpecificInstrumentField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: *mut bool,
) {
    let instance = **(this as *mut *mut &mut dyn CtpMiniMdApi);
    instance.on_rsp_sub_market_data(pSpecificInstrument, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspUnSubMarketData(
    this: *mut ::std::os::raw::c_void,
    pSpecificInstrument: *mut CThostFtdcSpecificInstrumentField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: *mut bool,
) {
    let instance = **(this as *mut *mut &mut dyn CtpMiniMdApi);
    instance.on_rsp_un_sub_market_data(pSpecificInstrument, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspSubForQuoteRsp(
    this: *mut ::std::os::raw::c_void,
    pSpecificInstrument: *mut CThostFtdcSpecificInstrumentField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: *mut bool,
) {
    let instance = **(this as *mut *mut &mut dyn CtpMiniMdApi);
    instance.on_rsp_sub_for_quote_rsp(pSpecificInstrument, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRspUnSubForQuoteRsp(
    this: *mut ::std::os::raw::c_void,
    pSpecificInstrument: *mut CThostFtdcSpecificInstrumentField,
    pRspInfo: *mut CThostFtdcRspInfoField,
    nRequestID: c_int,
    bIsLast: *mut bool,
) {
    let instance = **(this as *mut *mut &mut dyn CtpMiniMdApi);
    instance.on_rsp_un_sub_for_quote_rsp(pSpecificInstrument, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRtnDepthMarketData(
    this: *mut ::std::os::raw::c_void,
    pDepthMarketData: *mut CThostFtdcDepthMarketDataField,
) {
    let instance = **(this as *mut *mut &mut dyn CtpMiniMdApi);
    instance.on_rtn_depth_market_data(pDepthMarketData);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRtnMBLMarketData(
    this: *mut ::std::os::raw::c_void,
    pMBLMarketData: *mut CThostFtdcMBLMarketDataField,
) {
    let instance = **(this as *mut *mut &mut dyn CtpMiniMdApi);
    instance.on_rtn_m_b_l_market_data(pMBLMarketData);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpMiniActionOnRtnForQuoteRsp(
    this: *mut ::std::os::raw::c_void,
    pForQuoteRsp: *mut CThostFtdcForQuoteRspField,
) {
    let instance = **(this as *mut *mut &mut dyn CtpMiniMdApi);
    instance.on_rtn_for_quote_rsp(pForQuoteRsp);
}
