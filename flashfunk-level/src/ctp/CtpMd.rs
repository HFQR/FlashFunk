use std::os::raw::c_int;

pub(crate) trait CtpMdCApi {
    fn on_front_connected(&mut self) {
        println!("on_front_connected callback");
    }

    fn on_front_disconnected(&mut self, nReason: c_int) {
        println!("on_front_disconnected callback");
    }

    fn on_heart_beat_warning(&mut self, nTimeLapse: c_int) {
        println!("on_heart_beat_warning callback");
    }

    fn on_rsp_user_login(&mut self, pRspUserLogin: *mut CThostFtdcRspUserLoginField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool) {
        println!("on_rsp_user_login callback");
    }

    fn on_rsp_user_logout(&mut self, pUserLogout: *mut CThostFtdcUserLogoutField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool) {
        println!("on_rsp_user_logout callback");
    }

    fn on_rsp_qry_multicast_instrument(&mut self, pMulticastInstrument: *mut CThostFtdcMulticastInstrumentField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool) {
        println!("on_rsp_qry_multicast_instrument callback");
    }

    fn on_rsp_error(&mut self, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool) {
        println!("on_rsp_error callback");
    }

    fn on_rsp_sub_market_data(&mut self, pSpecificInstrument: *mut CThostFtdcSpecificInstrumentField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool) {
        println!("on_rsp_sub_market_data callback");
    }

    fn on_rsp_un_sub_market_data(&mut self, pSpecificInstrument: *mut CThostFtdcSpecificInstrumentField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool) {
        println!("on_rsp_un_sub_market_data callback");
    }

    fn on_rsp_sub_for_quote_rsp(&mut self, pSpecificInstrument: *mut CThostFtdcSpecificInstrumentField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool) {
        println!("on_rsp_sub_for_quote_rsp callback");
    }

    fn on_rsp_un_sub_for_quote_rsp(&mut self, pSpecificInstrument: *mut CThostFtdcSpecificInstrumentField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool) {
        println!("on_rsp_un_sub_for_quote_rsp callback");
    }

    fn on_rtn_depth_market_data(&mut self, pDepthMarketData: *mut CThostFtdcDepthMarketDataField) {
        println!("on_rtn_depth_market_data callback");
    }

    fn on_rtn_for_quote_rsp(&mut self, pForQuoteRsp: *mut CThostFtdcForQuoteRspField) {
        println!("on_rtn_for_quote_rsp callback");
    }
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionOnFrontDisconnected(this: *mut ::std::os::raw::c_void, nReason: c_int) {
    let instance = **(this as *mut *mut &mut dyn CtpMdCApi);
    instance.on_front_disconnected(nReason);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionOnHeartBeatWarning(this: *mut ::std::os::raw::c_void, nTimeLapse: c_int) {
    let instance = **(this as *mut *mut &mut dyn CtpMdCApi);
    instance.on_heart_beat_warning(nTimeLapse);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionOnRspUserLogin(this: *mut ::std::os::raw::c_void, pRspUserLogin: *mut CThostFtdcRspUserLoginField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool) {
    let instance = **(this as *mut *mut &mut dyn CtpMdCApi);
    instance.on_rsp_user_login(pRspUserLogin, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionOnRspUserLogout(this: *mut ::std::os::raw::c_void, pUserLogout: *mut CThostFtdcUserLogoutField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool) {
    let instance = **(this as *mut *mut &mut dyn CtpMdCApi);
    instance.on_rsp_user_logout(pUserLogout, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionOnRspQryMulticastInstrument(this: *mut ::std::os::raw::c_void, pMulticastInstrument: *mut CThostFtdcMulticastInstrumentField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool) {
    let instance = **(this as *mut *mut &mut dyn CtpMdCApi);
    instance.on_rsp_qry_multicast_instrument(pMulticastInstrument, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionOnRspError(this: *mut ::std::os::raw::c_void, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool) {
    let instance = **(this as *mut *mut &mut dyn CtpMdCApi);
    instance.on_rsp_error(pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionOnRspSubMarketData(this: *mut ::std::os::raw::c_void, pSpecificInstrument: *mut CThostFtdcSpecificInstrumentField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool) {
    let instance = **(this as *mut *mut &mut dyn CtpMdCApi);
    instance.on_rsp_sub_market_data(pSpecificInstrument, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionOnRspUnSubMarketData(this: *mut ::std::os::raw::c_void, pSpecificInstrument: *mut CThostFtdcSpecificInstrumentField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool) {
    let instance = **(this as *mut *mut &mut dyn CtpMdCApi);
    instance.on_rsp_un_sub_market_data(pSpecificInstrument, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionOnRspSubForQuoteRsp(this: *mut ::std::os::raw::c_void, pSpecificInstrument: *mut CThostFtdcSpecificInstrumentField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool) {
    let instance = **(this as *mut *mut &mut dyn CtpMdCApi);
    instance.on_rsp_sub_for_quote_rsp(pSpecificInstrument, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionOnRspUnSubForQuoteRsp(this: *mut ::std::os::raw::c_void, pSpecificInstrument: *mut CThostFtdcSpecificInstrumentField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: c_int, bIsLast: bool) {
    let instance = **(this as *mut *mut &mut dyn CtpMdCApi);
    instance.on_rsp_un_sub_for_quote_rsp(pSpecificInstrument, pRspInfo, nRequestID, bIsLast);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionOnRtnDepthMarketData(this: *mut ::std::os::raw::c_void, pDepthMarketData: *mut CThostFtdcDepthMarketDataField) {
    let instance = **(this as *mut *mut &mut dyn CtpMdCApi);
    instance.on_rtn_depth_market_data(pDepthMarketData);
}

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionOnRtnForQuoteRsp(this: *mut ::std::os::raw::c_void, pForQuoteRsp: *mut CThostFtdcForQuoteRspField) {
    let instance = **(this as *mut *mut &mut dyn CtpMdCApi);
    instance.on_rtn_for_quote_rsp(pForQuoteRsp);
}       
