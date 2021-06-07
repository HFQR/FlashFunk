use crate::ctp::sys::*;
use std::os::raw::{c_int, c_void};

pub(crate) trait CtpTdCApi { 

        fn on_front_connected(&mut self, ) {
            println!("on_front_connected callback");
        }
        
        fn on_front_disconnected(&mut self, nReason: c_int ) {
            println!("on_front_disconnected callback");
        }
        
        fn on_heart_beat_warning(&mut self, nTimeLapse: c_int ) {
            println!("on_heart_beat_warning callback");
        }
        
        fn on_rsp_authenticate(&mut self, pRspAuthenticateField: *mut CThostFtdcRspAuthenticateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_authenticate callback");
        }
        
        fn on_rsp_user_login(&mut self, pRspUserLogin: *mut CThostFtdcRspUserLoginField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_user_login callback");
        }
        
        fn on_rsp_user_logout(&mut self, pUserLogout: *mut CThostFtdcUserLogoutField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_user_logout callback");
        }
        
        fn on_rsp_user_password_update(&mut self, pUserPasswordUpdate: *mut CThostFtdcUserPasswordUpdateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_user_password_update callback");
        }
        
        fn on_rsp_trading_account_password_update(&mut self, pTradingAccountPasswordUpdate: *mut CThostFtdcTradingAccountPasswordUpdateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_trading_account_password_update callback");
        }
        
        fn on_rsp_user_auth_method(&mut self, pRspUserAuthMethod: *mut CThostFtdcRspUserAuthMethodField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_user_auth_method callback");
        }
        
        fn on_rsp_gen_user_captcha(&mut self, pRspGenUserCaptcha: *mut CThostFtdcRspGenUserCaptchaField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_gen_user_captcha callback");
        }
        
        fn on_rsp_gen_user_text(&mut self, pRspGenUserText: *mut CThostFtdcRspGenUserTextField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_gen_user_text callback");
        }
        
        fn on_rsp_order_insert(&mut self, pInputOrder: *mut CThostFtdcInputOrderField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_order_insert callback");
        }
        
        fn on_rsp_parked_order_insert(&mut self, pParkedOrder: *mut CThostFtdcParkedOrderField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_parked_order_insert callback");
        }
        
        fn on_rsp_parked_order_action(&mut self, pParkedOrderAction: *mut CThostFtdcParkedOrderActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_parked_order_action callback");
        }
        
        fn on_rsp_order_action(&mut self, pInputOrderAction: *mut CThostFtdcInputOrderActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_order_action callback");
        }
        
        fn on_rsp_query_max_order_volume(&mut self, pQueryMaxOrderVolume: *mut CThostFtdcQueryMaxOrderVolumeField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_query_max_order_volume callback");
        }
        
        fn on_rsp_settlement_info_confirm(&mut self, pSettlementInfoConfirm: *mut CThostFtdcSettlementInfoConfirmField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_settlement_info_confirm callback");
        }
        
        fn on_rsp_remove_parked_order(&mut self, pRemoveParkedOrder: *mut CThostFtdcRemoveParkedOrderField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_remove_parked_order callback");
        }
        
        fn on_rsp_remove_parked_order_action(&mut self, pRemoveParkedOrderAction: *mut CThostFtdcRemoveParkedOrderActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_remove_parked_order_action callback");
        }
        
        fn on_rsp_exec_order_insert(&mut self, pInputExecOrder: *mut CThostFtdcInputExecOrderField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_exec_order_insert callback");
        }
        
        fn on_rsp_exec_order_action(&mut self, pInputExecOrderAction: *mut CThostFtdcInputExecOrderActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_exec_order_action callback");
        }
        
        fn on_rsp_for_quote_insert(&mut self, pInputForQuote: *mut CThostFtdcInputForQuoteField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_for_quote_insert callback");
        }
        
        fn on_rsp_quote_insert(&mut self, pInputQuote: *mut CThostFtdcInputQuoteField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_quote_insert callback");
        }
        
        fn on_rsp_quote_action(&mut self, pInputQuoteAction: *mut CThostFtdcInputQuoteActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_quote_action callback");
        }
        
        fn on_rsp_batch_order_action(&mut self, pInputBatchOrderAction: *mut CThostFtdcInputBatchOrderActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_batch_order_action callback");
        }
        
        fn on_rsp_option_self_close_insert(&mut self, pInputOptionSelfClose: *mut CThostFtdcInputOptionSelfCloseField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_option_self_close_insert callback");
        }
        
        fn on_rsp_option_self_close_action(&mut self, pInputOptionSelfCloseAction: *mut CThostFtdcInputOptionSelfCloseActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_option_self_close_action callback");
        }
        
        fn on_rsp_comb_action_insert(&mut self, pInputCombAction: *mut CThostFtdcInputCombActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_comb_action_insert callback");
        }
        
        fn on_rsp_qry_order(&mut self, pOrder: *mut CThostFtdcOrderField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_order callback");
        }
        
        fn on_rsp_qry_trade(&mut self, pTrade: *mut CThostFtdcTradeField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_trade callback");
        }
        
        fn on_rsp_qry_investor_position(&mut self, pInvestorPosition: *mut CThostFtdcInvestorPositionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_investor_position callback");
        }
        
        fn on_rsp_qry_trading_account(&mut self, pTradingAccount: *mut CThostFtdcTradingAccountField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_trading_account callback");
        }
        
        fn on_rsp_qry_investor(&mut self, pInvestor: *mut CThostFtdcInvestorField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_investor callback");
        }
        
        fn on_rsp_qry_trading_code(&mut self, pTradingCode: *mut CThostFtdcTradingCodeField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_trading_code callback");
        }
        
        fn on_rsp_qry_instrument_margin_rate(&mut self, pInstrumentMarginRate: *mut CThostFtdcInstrumentMarginRateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_instrument_margin_rate callback");
        }
        
        fn on_rsp_qry_instrument_commission_rate(&mut self, pInstrumentCommissionRate: *mut CThostFtdcInstrumentCommissionRateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_instrument_commission_rate callback");
        }
        
        fn on_rsp_qry_exchange(&mut self, pExchange: *mut CThostFtdcExchangeField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_exchange callback");
        }
        
        fn on_rsp_qry_product(&mut self, pProduct: *mut CThostFtdcProductField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_product callback");
        }
        
        fn on_rsp_qry_instrument(&mut self, pInstrument: *mut CThostFtdcInstrumentField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_instrument callback");
        }
        
        fn on_rsp_qry_depth_market_data(&mut self, pDepthMarketData: *mut CThostFtdcDepthMarketDataField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_depth_market_data callback");
        }
        
        fn on_rsp_qry_settlement_info(&mut self, pSettlementInfo: *mut CThostFtdcSettlementInfoField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_settlement_info callback");
        }
        
        fn on_rsp_qry_transfer_bank(&mut self, pTransferBank: *mut CThostFtdcTransferBankField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_transfer_bank callback");
        }
        
        fn on_rsp_qry_investor_position_detail(&mut self, pInvestorPositionDetail: *mut CThostFtdcInvestorPositionDetailField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_investor_position_detail callback");
        }
        
        fn on_rsp_qry_notice(&mut self, pNotice: *mut CThostFtdcNoticeField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_notice callback");
        }
        
        fn on_rsp_qry_settlement_info_confirm(&mut self, pSettlementInfoConfirm: *mut CThostFtdcSettlementInfoConfirmField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_settlement_info_confirm callback");
        }
        
        fn on_rsp_qry_investor_position_combine_detail(&mut self, pInvestorPositionCombineDetail: *mut CThostFtdcInvestorPositionCombineDetailField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_investor_position_combine_detail callback");
        }
        
        fn on_rsp_qry_c_f_m_m_c_trading_account_key(&mut self, pCFMMCTradingAccountKey: *mut CThostFtdcCFMMCTradingAccountKeyField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_c_f_m_m_c_trading_account_key callback");
        }
        
        fn on_rsp_qry_e_warrant_offset(&mut self, pEWarrantOffset: *mut CThostFtdcEWarrantOffsetField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_e_warrant_offset callback");
        }
        
        fn on_rsp_qry_investor_product_group_margin(&mut self, pInvestorProductGroupMargin: *mut CThostFtdcInvestorProductGroupMarginField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_investor_product_group_margin callback");
        }
        
        fn on_rsp_qry_exchange_margin_rate(&mut self, pExchangeMarginRate: *mut CThostFtdcExchangeMarginRateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_exchange_margin_rate callback");
        }
        
        fn on_rsp_qry_exchange_margin_rate_adjust(&mut self, pExchangeMarginRateAdjust: *mut CThostFtdcExchangeMarginRateAdjustField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_exchange_margin_rate_adjust callback");
        }
        
        fn on_rsp_qry_exchange_rate(&mut self, pExchangeRate: *mut CThostFtdcExchangeRateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_exchange_rate callback");
        }
        
        fn on_rsp_qry_sec_agent_a_c_i_d_map(&mut self, pSecAgentACIDMap: *mut CThostFtdcSecAgentACIDMapField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_sec_agent_a_c_i_d_map callback");
        }
        
        fn on_rsp_qry_product_exch_rate(&mut self, pProductExchRate: *mut CThostFtdcProductExchRateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_product_exch_rate callback");
        }
        
        fn on_rsp_qry_product_group(&mut self, pProductGroup: *mut CThostFtdcProductGroupField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_product_group callback");
        }
        
        fn on_rsp_qry_m_m_instrument_commission_rate(&mut self, pMMInstrumentCommissionRate: *mut CThostFtdcMMInstrumentCommissionRateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_m_m_instrument_commission_rate callback");
        }
        
        fn on_rsp_qry_m_m_option_instr_comm_rate(&mut self, pMMOptionInstrCommRate: *mut CThostFtdcMMOptionInstrCommRateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_m_m_option_instr_comm_rate callback");
        }
        
        fn on_rsp_qry_instrument_order_comm_rate(&mut self, pInstrumentOrderCommRate: *mut CThostFtdcInstrumentOrderCommRateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_instrument_order_comm_rate callback");
        }
        
        fn on_rsp_qry_sec_agent_trading_account(&mut self, pTradingAccount: *mut CThostFtdcTradingAccountField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_sec_agent_trading_account callback");
        }
        
        fn on_rsp_qry_sec_agent_check_mode(&mut self, pSecAgentCheckMode: *mut CThostFtdcSecAgentCheckModeField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_sec_agent_check_mode callback");
        }
        
        fn on_rsp_qry_sec_agent_trade_info(&mut self, pSecAgentTradeInfo: *mut CThostFtdcSecAgentTradeInfoField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_sec_agent_trade_info callback");
        }
        
        fn on_rsp_qry_option_instr_trade_cost(&mut self, pOptionInstrTradeCost: *mut CThostFtdcOptionInstrTradeCostField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_option_instr_trade_cost callback");
        }
        
        fn on_rsp_qry_option_instr_comm_rate(&mut self, pOptionInstrCommRate: *mut CThostFtdcOptionInstrCommRateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_option_instr_comm_rate callback");
        }
        
        fn on_rsp_qry_exec_order(&mut self, pExecOrder: *mut CThostFtdcExecOrderField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_exec_order callback");
        }
        
        fn on_rsp_qry_for_quote(&mut self, pForQuote: *mut CThostFtdcForQuoteField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_for_quote callback");
        }
        
        fn on_rsp_qry_quote(&mut self, pQuote: *mut CThostFtdcQuoteField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_quote callback");
        }
        
        fn on_rsp_qry_option_self_close(&mut self, pOptionSelfClose: *mut CThostFtdcOptionSelfCloseField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_option_self_close callback");
        }
        
        fn on_rsp_qry_invest_unit(&mut self, pInvestUnit: *mut CThostFtdcInvestUnitField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_invest_unit callback");
        }
        
        fn on_rsp_qry_comb_instrument_guard(&mut self, pCombInstrumentGuard: *mut CThostFtdcCombInstrumentGuardField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_comb_instrument_guard callback");
        }
        
        fn on_rsp_qry_comb_action(&mut self, pCombAction: *mut CThostFtdcCombActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_comb_action callback");
        }
        
        fn on_rsp_qry_transfer_serial(&mut self, pTransferSerial: *mut CThostFtdcTransferSerialField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_transfer_serial callback");
        }
        
        fn on_rsp_qry_accountregister(&mut self, pAccountregister: *mut CThostFtdcAccountregisterField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_accountregister callback");
        }
        
        fn on_rsp_error(&mut self, pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_error callback");
        }
        
        fn on_rtn_order(&mut self, pOrder: *mut CThostFtdcOrderField  ) {
            println!("on_rtn_order callback");
        }
        
        fn on_rtn_trade(&mut self, pTrade: *mut CThostFtdcTradeField  ) {
            println!("on_rtn_trade callback");
        }
        
        fn on_err_rtn_order_insert(&mut self, pInputOrder: *mut CThostFtdcInputOrderField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
            println!("on_err_rtn_order_insert callback");
        }
        
        fn on_err_rtn_order_action(&mut self, pOrderAction: *mut CThostFtdcOrderActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
            println!("on_err_rtn_order_action callback");
        }
        
        fn on_rtn_instrument_status(&mut self, pInstrumentStatus: *mut CThostFtdcInstrumentStatusField  ) {
            println!("on_rtn_instrument_status callback");
        }
        
        fn on_rtn_bulletin(&mut self, pBulletin: *mut CThostFtdcBulletinField  ) {
            println!("on_rtn_bulletin callback");
        }
        
        fn on_rtn_trading_notice(&mut self, pTradingNoticeInfo: *mut CThostFtdcTradingNoticeInfoField  ) {
            println!("on_rtn_trading_notice callback");
        }
        
        fn on_rtn_error_conditional_order(&mut self, pErrorConditionalOrder: *mut CThostFtdcErrorConditionalOrderField  ) {
            println!("on_rtn_error_conditional_order callback");
        }
        
        fn on_rtn_exec_order(&mut self, pExecOrder: *mut CThostFtdcExecOrderField  ) {
            println!("on_rtn_exec_order callback");
        }
        
        fn on_err_rtn_exec_order_insert(&mut self, pInputExecOrder: *mut CThostFtdcInputExecOrderField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
            println!("on_err_rtn_exec_order_insert callback");
        }
        
        fn on_err_rtn_exec_order_action(&mut self, pExecOrderAction: *mut CThostFtdcExecOrderActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
            println!("on_err_rtn_exec_order_action callback");
        }
        
        fn on_err_rtn_for_quote_insert(&mut self, pInputForQuote: *mut CThostFtdcInputForQuoteField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
            println!("on_err_rtn_for_quote_insert callback");
        }
        
        fn on_rtn_quote(&mut self, pQuote: *mut CThostFtdcQuoteField  ) {
            println!("on_rtn_quote callback");
        }
        
        fn on_err_rtn_quote_insert(&mut self, pInputQuote: *mut CThostFtdcInputQuoteField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
            println!("on_err_rtn_quote_insert callback");
        }
        
        fn on_err_rtn_quote_action(&mut self, pQuoteAction: *mut CThostFtdcQuoteActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
            println!("on_err_rtn_quote_action callback");
        }
        
        fn on_rtn_for_quote_rsp(&mut self, pForQuoteRsp: *mut CThostFtdcForQuoteRspField  ) {
            println!("on_rtn_for_quote_rsp callback");
        }
        
        fn on_rtn_c_f_m_m_c_trading_account_token(&mut self, pCFMMCTradingAccountToken: *mut CThostFtdcCFMMCTradingAccountTokenField  ) {
            println!("on_rtn_c_f_m_m_c_trading_account_token callback");
        }
        
        fn on_err_rtn_batch_order_action(&mut self, pBatchOrderAction: *mut CThostFtdcBatchOrderActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
            println!("on_err_rtn_batch_order_action callback");
        }
        
        fn on_rtn_option_self_close(&mut self, pOptionSelfClose: *mut CThostFtdcOptionSelfCloseField  ) {
            println!("on_rtn_option_self_close callback");
        }
        
        fn on_err_rtn_option_self_close_insert(&mut self, pInputOptionSelfClose: *mut CThostFtdcInputOptionSelfCloseField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
            println!("on_err_rtn_option_self_close_insert callback");
        }
        
        fn on_err_rtn_option_self_close_action(&mut self, pOptionSelfCloseAction: *mut CThostFtdcOptionSelfCloseActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
            println!("on_err_rtn_option_self_close_action callback");
        }
        
        fn on_rtn_comb_action(&mut self, pCombAction: *mut CThostFtdcCombActionField  ) {
            println!("on_rtn_comb_action callback");
        }
        
        fn on_err_rtn_comb_action_insert(&mut self, pInputCombAction: *mut CThostFtdcInputCombActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
            println!("on_err_rtn_comb_action_insert callback");
        }
        
        fn on_rsp_qry_contract_bank(&mut self, pContractBank: *mut CThostFtdcContractBankField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_contract_bank callback");
        }
        
        fn on_rsp_qry_parked_order(&mut self, pParkedOrder: *mut CThostFtdcParkedOrderField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_parked_order callback");
        }
        
        fn on_rsp_qry_parked_order_action(&mut self, pParkedOrderAction: *mut CThostFtdcParkedOrderActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_parked_order_action callback");
        }
        
        fn on_rsp_qry_trading_notice(&mut self, pTradingNotice: *mut CThostFtdcTradingNoticeField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_trading_notice callback");
        }
        
        fn on_rsp_qry_broker_trading_params(&mut self, pBrokerTradingParams: *mut CThostFtdcBrokerTradingParamsField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_broker_trading_params callback");
        }
        
        fn on_rsp_qry_broker_trading_algos(&mut self, pBrokerTradingAlgos: *mut CThostFtdcBrokerTradingAlgosField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_qry_broker_trading_algos callback");
        }
        
        fn on_rsp_query_c_f_m_m_c_trading_account_token(&mut self, pQueryCFMMCTradingAccountToken: *mut CThostFtdcQueryCFMMCTradingAccountTokenField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_query_c_f_m_m_c_trading_account_token callback");
        }
        
        fn on_rtn_from_bank_to_future_by_bank(&mut self, pRspTransfer: *mut CThostFtdcRspTransferField  ) {
            println!("on_rtn_from_bank_to_future_by_bank callback");
        }
        
        fn on_rtn_from_future_to_bank_by_bank(&mut self, pRspTransfer: *mut CThostFtdcRspTransferField  ) {
            println!("on_rtn_from_future_to_bank_by_bank callback");
        }
        
        fn on_rtn_repeal_from_bank_to_future_by_bank(&mut self, pRspRepeal: *mut CThostFtdcRspRepealField  ) {
            println!("on_rtn_repeal_from_bank_to_future_by_bank callback");
        }
        
        fn on_rtn_repeal_from_future_to_bank_by_bank(&mut self, pRspRepeal: *mut CThostFtdcRspRepealField  ) {
            println!("on_rtn_repeal_from_future_to_bank_by_bank callback");
        }
        
        fn on_rtn_from_bank_to_future_by_future(&mut self, pRspTransfer: *mut CThostFtdcRspTransferField  ) {
            println!("on_rtn_from_bank_to_future_by_future callback");
        }
        
        fn on_rtn_from_future_to_bank_by_future(&mut self, pRspTransfer: *mut CThostFtdcRspTransferField  ) {
            println!("on_rtn_from_future_to_bank_by_future callback");
        }
        
        fn on_rtn_repeal_from_bank_to_future_by_future_manual(&mut self, pRspRepeal: *mut CThostFtdcRspRepealField  ) {
            println!("on_rtn_repeal_from_bank_to_future_by_future_manual callback");
        }
        
        fn on_rtn_repeal_from_future_to_bank_by_future_manual(&mut self, pRspRepeal: *mut CThostFtdcRspRepealField  ) {
            println!("on_rtn_repeal_from_future_to_bank_by_future_manual callback");
        }
        
        fn on_rtn_query_bank_balance_by_future(&mut self, pNotifyQueryAccount: *mut CThostFtdcNotifyQueryAccountField  ) {
            println!("on_rtn_query_bank_balance_by_future callback");
        }
        
        fn on_err_rtn_bank_to_future_by_future(&mut self, pReqTransfer: *mut CThostFtdcReqTransferField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
            println!("on_err_rtn_bank_to_future_by_future callback");
        }
        
        fn on_err_rtn_future_to_bank_by_future(&mut self, pReqTransfer: *mut CThostFtdcReqTransferField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
            println!("on_err_rtn_future_to_bank_by_future callback");
        }
        
        fn on_err_rtn_repeal_bank_to_future_by_future_manual(&mut self, pReqRepeal: *mut CThostFtdcReqRepealField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
            println!("on_err_rtn_repeal_bank_to_future_by_future_manual callback");
        }
        
        fn on_err_rtn_repeal_future_to_bank_by_future_manual(&mut self, pReqRepeal: *mut CThostFtdcReqRepealField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
            println!("on_err_rtn_repeal_future_to_bank_by_future_manual callback");
        }
        
        fn on_err_rtn_query_bank_balance_by_future(&mut self, pReqQueryAccount: *mut CThostFtdcReqQueryAccountField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
            println!("on_err_rtn_query_bank_balance_by_future callback");
        }
        
        fn on_rtn_repeal_from_bank_to_future_by_future(&mut self, pRspRepeal: *mut CThostFtdcRspRepealField  ) {
            println!("on_rtn_repeal_from_bank_to_future_by_future callback");
        }
        
        fn on_rtn_repeal_from_future_to_bank_by_future(&mut self, pRspRepeal: *mut CThostFtdcRspRepealField  ) {
            println!("on_rtn_repeal_from_future_to_bank_by_future callback");
        }
        
        fn on_rsp_from_bank_to_future_by_future(&mut self, pReqTransfer: *mut CThostFtdcReqTransferField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_from_bank_to_future_by_future callback");
        }
        
        fn on_rsp_from_future_to_bank_by_future(&mut self, pReqTransfer: *mut CThostFtdcReqTransferField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_from_future_to_bank_by_future callback");
        }
        
        fn on_rsp_query_bank_account_money_by_future(&mut self, pReqQueryAccount: *mut CThostFtdcReqQueryAccountField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
            println!("on_rsp_query_bank_account_money_by_future callback");
        }
        
        fn on_rtn_open_account_by_bank(&mut self, pOpenAccount: *mut CThostFtdcOpenAccountField  ) {
            println!("on_rtn_open_account_by_bank callback");
        }
        
        fn on_rtn_cancel_account_by_bank(&mut self, pCancelAccount: *mut CThostFtdcCancelAccountField  ) {
            println!("on_rtn_cancel_account_by_bank callback");
        }
        
        fn on_rtn_change_account_by_bank(&mut self, pChangeAccount: *mut CThostFtdcChangeAccountField  ) {
            println!("on_rtn_change_account_by_bank callback");
        }
        
}
           unsafe fn get_api<'a>(spi: *mut c_void) -> &'a mut dyn CtpTdCApi {
            &mut **(spi as *mut *mut dyn CtpTdCApi)
        }
        
#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnFrontDisconnected(this: *mut ::std::os::raw::c_void, nReason: c_int ) {
    let instance = get_api(this);
    instance.on_front_disconnected(nReason);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnHeartBeatWarning(this: *mut ::std::os::raw::c_void, nTimeLapse: c_int ) {
    let instance = get_api(this);
    instance.on_heart_beat_warning(nTimeLapse);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspAuthenticate(this: *mut ::std::os::raw::c_void, pRspAuthenticateField: *mut CThostFtdcRspAuthenticateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_authenticate(pRspAuthenticateField, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspUserLogin(this: *mut ::std::os::raw::c_void, pRspUserLogin: *mut CThostFtdcRspUserLoginField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_user_login(pRspUserLogin, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspUserLogout(this: *mut ::std::os::raw::c_void, pUserLogout: *mut CThostFtdcUserLogoutField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_user_logout(pUserLogout, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspUserPasswordUpdate(this: *mut ::std::os::raw::c_void, pUserPasswordUpdate: *mut CThostFtdcUserPasswordUpdateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_user_password_update(pUserPasswordUpdate, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspTradingAccountPasswordUpdate(this: *mut ::std::os::raw::c_void, pTradingAccountPasswordUpdate: *mut CThostFtdcTradingAccountPasswordUpdateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_trading_account_password_update(pTradingAccountPasswordUpdate, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspUserAuthMethod(this: *mut ::std::os::raw::c_void, pRspUserAuthMethod: *mut CThostFtdcRspUserAuthMethodField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_user_auth_method(pRspUserAuthMethod, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspGenUserCaptcha(this: *mut ::std::os::raw::c_void, pRspGenUserCaptcha: *mut CThostFtdcRspGenUserCaptchaField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_gen_user_captcha(pRspGenUserCaptcha, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspGenUserText(this: *mut ::std::os::raw::c_void, pRspGenUserText: *mut CThostFtdcRspGenUserTextField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_gen_user_text(pRspGenUserText, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspOrderInsert(this: *mut ::std::os::raw::c_void, pInputOrder: *mut CThostFtdcInputOrderField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_order_insert(pInputOrder, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspParkedOrderInsert(this: *mut ::std::os::raw::c_void, pParkedOrder: *mut CThostFtdcParkedOrderField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_parked_order_insert(pParkedOrder, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspParkedOrderAction(this: *mut ::std::os::raw::c_void, pParkedOrderAction: *mut CThostFtdcParkedOrderActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_parked_order_action(pParkedOrderAction, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspOrderAction(this: *mut ::std::os::raw::c_void, pInputOrderAction: *mut CThostFtdcInputOrderActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_order_action(pInputOrderAction, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQueryMaxOrderVolume(this: *mut ::std::os::raw::c_void, pQueryMaxOrderVolume: *mut CThostFtdcQueryMaxOrderVolumeField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_query_max_order_volume(pQueryMaxOrderVolume, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspSettlementInfoConfirm(this: *mut ::std::os::raw::c_void, pSettlementInfoConfirm: *mut CThostFtdcSettlementInfoConfirmField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_settlement_info_confirm(pSettlementInfoConfirm, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspRemoveParkedOrder(this: *mut ::std::os::raw::c_void, pRemoveParkedOrder: *mut CThostFtdcRemoveParkedOrderField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_remove_parked_order(pRemoveParkedOrder, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspRemoveParkedOrderAction(this: *mut ::std::os::raw::c_void, pRemoveParkedOrderAction: *mut CThostFtdcRemoveParkedOrderActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_remove_parked_order_action(pRemoveParkedOrderAction, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspExecOrderInsert(this: *mut ::std::os::raw::c_void, pInputExecOrder: *mut CThostFtdcInputExecOrderField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_exec_order_insert(pInputExecOrder, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspExecOrderAction(this: *mut ::std::os::raw::c_void, pInputExecOrderAction: *mut CThostFtdcInputExecOrderActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_exec_order_action(pInputExecOrderAction, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspForQuoteInsert(this: *mut ::std::os::raw::c_void, pInputForQuote: *mut CThostFtdcInputForQuoteField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_for_quote_insert(pInputForQuote, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQuoteInsert(this: *mut ::std::os::raw::c_void, pInputQuote: *mut CThostFtdcInputQuoteField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_quote_insert(pInputQuote, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQuoteAction(this: *mut ::std::os::raw::c_void, pInputQuoteAction: *mut CThostFtdcInputQuoteActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_quote_action(pInputQuoteAction, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspBatchOrderAction(this: *mut ::std::os::raw::c_void, pInputBatchOrderAction: *mut CThostFtdcInputBatchOrderActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_batch_order_action(pInputBatchOrderAction, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspOptionSelfCloseInsert(this: *mut ::std::os::raw::c_void, pInputOptionSelfClose: *mut CThostFtdcInputOptionSelfCloseField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_option_self_close_insert(pInputOptionSelfClose, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspOptionSelfCloseAction(this: *mut ::std::os::raw::c_void, pInputOptionSelfCloseAction: *mut CThostFtdcInputOptionSelfCloseActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_option_self_close_action(pInputOptionSelfCloseAction, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspCombActionInsert(this: *mut ::std::os::raw::c_void, pInputCombAction: *mut CThostFtdcInputCombActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_comb_action_insert(pInputCombAction, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryOrder(this: *mut ::std::os::raw::c_void, pOrder: *mut CThostFtdcOrderField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_order(pOrder, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryTrade(this: *mut ::std::os::raw::c_void, pTrade: *mut CThostFtdcTradeField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_trade(pTrade, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryInvestorPosition(this: *mut ::std::os::raw::c_void, pInvestorPosition: *mut CThostFtdcInvestorPositionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_investor_position(pInvestorPosition, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryTradingAccount(this: *mut ::std::os::raw::c_void, pTradingAccount: *mut CThostFtdcTradingAccountField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_trading_account(pTradingAccount, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryInvestor(this: *mut ::std::os::raw::c_void, pInvestor: *mut CThostFtdcInvestorField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_investor(pInvestor, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryTradingCode(this: *mut ::std::os::raw::c_void, pTradingCode: *mut CThostFtdcTradingCodeField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_trading_code(pTradingCode, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryInstrumentMarginRate(this: *mut ::std::os::raw::c_void, pInstrumentMarginRate: *mut CThostFtdcInstrumentMarginRateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_instrument_margin_rate(pInstrumentMarginRate, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryInstrumentCommissionRate(this: *mut ::std::os::raw::c_void, pInstrumentCommissionRate: *mut CThostFtdcInstrumentCommissionRateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_instrument_commission_rate(pInstrumentCommissionRate, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryExchange(this: *mut ::std::os::raw::c_void, pExchange: *mut CThostFtdcExchangeField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_exchange(pExchange, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryProduct(this: *mut ::std::os::raw::c_void, pProduct: *mut CThostFtdcProductField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_product(pProduct, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryInstrument(this: *mut ::std::os::raw::c_void, pInstrument: *mut CThostFtdcInstrumentField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_instrument(pInstrument, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryDepthMarketData(this: *mut ::std::os::raw::c_void, pDepthMarketData: *mut CThostFtdcDepthMarketDataField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_depth_market_data(pDepthMarketData, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQrySettlementInfo(this: *mut ::std::os::raw::c_void, pSettlementInfo: *mut CThostFtdcSettlementInfoField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_settlement_info(pSettlementInfo, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryTransferBank(this: *mut ::std::os::raw::c_void, pTransferBank: *mut CThostFtdcTransferBankField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_transfer_bank(pTransferBank, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryInvestorPositionDetail(this: *mut ::std::os::raw::c_void, pInvestorPositionDetail: *mut CThostFtdcInvestorPositionDetailField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_investor_position_detail(pInvestorPositionDetail, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryNotice(this: *mut ::std::os::raw::c_void, pNotice: *mut CThostFtdcNoticeField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_notice(pNotice, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQrySettlementInfoConfirm(this: *mut ::std::os::raw::c_void, pSettlementInfoConfirm: *mut CThostFtdcSettlementInfoConfirmField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_settlement_info_confirm(pSettlementInfoConfirm, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryInvestorPositionCombineDetail(this: *mut ::std::os::raw::c_void, pInvestorPositionCombineDetail: *mut CThostFtdcInvestorPositionCombineDetailField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_investor_position_combine_detail(pInvestorPositionCombineDetail, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryCFMMCTradingAccountKey(this: *mut ::std::os::raw::c_void, pCFMMCTradingAccountKey: *mut CThostFtdcCFMMCTradingAccountKeyField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_c_f_m_m_c_trading_account_key(pCFMMCTradingAccountKey, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryEWarrantOffset(this: *mut ::std::os::raw::c_void, pEWarrantOffset: *mut CThostFtdcEWarrantOffsetField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_e_warrant_offset(pEWarrantOffset, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryInvestorProductGroupMargin(this: *mut ::std::os::raw::c_void, pInvestorProductGroupMargin: *mut CThostFtdcInvestorProductGroupMarginField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_investor_product_group_margin(pInvestorProductGroupMargin, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryExchangeMarginRate(this: *mut ::std::os::raw::c_void, pExchangeMarginRate: *mut CThostFtdcExchangeMarginRateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_exchange_margin_rate(pExchangeMarginRate, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryExchangeMarginRateAdjust(this: *mut ::std::os::raw::c_void, pExchangeMarginRateAdjust: *mut CThostFtdcExchangeMarginRateAdjustField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_exchange_margin_rate_adjust(pExchangeMarginRateAdjust, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryExchangeRate(this: *mut ::std::os::raw::c_void, pExchangeRate: *mut CThostFtdcExchangeRateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_exchange_rate(pExchangeRate, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQrySecAgentACIDMap(this: *mut ::std::os::raw::c_void, pSecAgentACIDMap: *mut CThostFtdcSecAgentACIDMapField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_sec_agent_a_c_i_d_map(pSecAgentACIDMap, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryProductExchRate(this: *mut ::std::os::raw::c_void, pProductExchRate: *mut CThostFtdcProductExchRateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_product_exch_rate(pProductExchRate, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryProductGroup(this: *mut ::std::os::raw::c_void, pProductGroup: *mut CThostFtdcProductGroupField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_product_group(pProductGroup, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryMMInstrumentCommissionRate(this: *mut ::std::os::raw::c_void, pMMInstrumentCommissionRate: *mut CThostFtdcMMInstrumentCommissionRateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_m_m_instrument_commission_rate(pMMInstrumentCommissionRate, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryMMOptionInstrCommRate(this: *mut ::std::os::raw::c_void, pMMOptionInstrCommRate: *mut CThostFtdcMMOptionInstrCommRateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_m_m_option_instr_comm_rate(pMMOptionInstrCommRate, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryInstrumentOrderCommRate(this: *mut ::std::os::raw::c_void, pInstrumentOrderCommRate: *mut CThostFtdcInstrumentOrderCommRateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_instrument_order_comm_rate(pInstrumentOrderCommRate, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQrySecAgentTradingAccount(this: *mut ::std::os::raw::c_void, pTradingAccount: *mut CThostFtdcTradingAccountField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_sec_agent_trading_account(pTradingAccount, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQrySecAgentCheckMode(this: *mut ::std::os::raw::c_void, pSecAgentCheckMode: *mut CThostFtdcSecAgentCheckModeField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_sec_agent_check_mode(pSecAgentCheckMode, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQrySecAgentTradeInfo(this: *mut ::std::os::raw::c_void, pSecAgentTradeInfo: *mut CThostFtdcSecAgentTradeInfoField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_sec_agent_trade_info(pSecAgentTradeInfo, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryOptionInstrTradeCost(this: *mut ::std::os::raw::c_void, pOptionInstrTradeCost: *mut CThostFtdcOptionInstrTradeCostField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_option_instr_trade_cost(pOptionInstrTradeCost, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryOptionInstrCommRate(this: *mut ::std::os::raw::c_void, pOptionInstrCommRate: *mut CThostFtdcOptionInstrCommRateField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_option_instr_comm_rate(pOptionInstrCommRate, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryExecOrder(this: *mut ::std::os::raw::c_void, pExecOrder: *mut CThostFtdcExecOrderField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_exec_order(pExecOrder, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryForQuote(this: *mut ::std::os::raw::c_void, pForQuote: *mut CThostFtdcForQuoteField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_for_quote(pForQuote, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryQuote(this: *mut ::std::os::raw::c_void, pQuote: *mut CThostFtdcQuoteField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_quote(pQuote, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryOptionSelfClose(this: *mut ::std::os::raw::c_void, pOptionSelfClose: *mut CThostFtdcOptionSelfCloseField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_option_self_close(pOptionSelfClose, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryInvestUnit(this: *mut ::std::os::raw::c_void, pInvestUnit: *mut CThostFtdcInvestUnitField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_invest_unit(pInvestUnit, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryCombInstrumentGuard(this: *mut ::std::os::raw::c_void, pCombInstrumentGuard: *mut CThostFtdcCombInstrumentGuardField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_comb_instrument_guard(pCombInstrumentGuard, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryCombAction(this: *mut ::std::os::raw::c_void, pCombAction: *mut CThostFtdcCombActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_comb_action(pCombAction, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryTransferSerial(this: *mut ::std::os::raw::c_void, pTransferSerial: *mut CThostFtdcTransferSerialField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_transfer_serial(pTransferSerial, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryAccountregister(this: *mut ::std::os::raw::c_void, pAccountregister: *mut CThostFtdcAccountregisterField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_accountregister(pAccountregister, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspError(this: *mut ::std::os::raw::c_void, pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_error(pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRtnOrder(this: *mut ::std::os::raw::c_void, pOrder: *mut CThostFtdcOrderField  ) {
    let instance = get_api(this);
    instance.on_rtn_order(pOrder);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRtnTrade(this: *mut ::std::os::raw::c_void, pTrade: *mut CThostFtdcTradeField  ) {
    let instance = get_api(this);
    instance.on_rtn_trade(pTrade);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnErrRtnOrderInsert(this: *mut ::std::os::raw::c_void, pInputOrder: *mut CThostFtdcInputOrderField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance = get_api(this);
    instance.on_err_rtn_order_insert(pInputOrder, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnErrRtnOrderAction(this: *mut ::std::os::raw::c_void, pOrderAction: *mut CThostFtdcOrderActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance = get_api(this);
    instance.on_err_rtn_order_action(pOrderAction, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRtnInstrumentStatus(this: *mut ::std::os::raw::c_void, pInstrumentStatus: *mut CThostFtdcInstrumentStatusField  ) {
    let instance = get_api(this);
    instance.on_rtn_instrument_status(pInstrumentStatus);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRtnBulletin(this: *mut ::std::os::raw::c_void, pBulletin: *mut CThostFtdcBulletinField  ) {
    let instance = get_api(this);
    instance.on_rtn_bulletin(pBulletin);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRtnTradingNotice(this: *mut ::std::os::raw::c_void, pTradingNoticeInfo: *mut CThostFtdcTradingNoticeInfoField  ) {
    let instance = get_api(this);
    instance.on_rtn_trading_notice(pTradingNoticeInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRtnErrorConditionalOrder(this: *mut ::std::os::raw::c_void, pErrorConditionalOrder: *mut CThostFtdcErrorConditionalOrderField  ) {
    let instance = get_api(this);
    instance.on_rtn_error_conditional_order(pErrorConditionalOrder);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRtnExecOrder(this: *mut ::std::os::raw::c_void, pExecOrder: *mut CThostFtdcExecOrderField  ) {
    let instance = get_api(this);
    instance.on_rtn_exec_order(pExecOrder);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnErrRtnExecOrderInsert(this: *mut ::std::os::raw::c_void, pInputExecOrder: *mut CThostFtdcInputExecOrderField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance = get_api(this);
    instance.on_err_rtn_exec_order_insert(pInputExecOrder, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnErrRtnExecOrderAction(this: *mut ::std::os::raw::c_void, pExecOrderAction: *mut CThostFtdcExecOrderActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance = get_api(this);
    instance.on_err_rtn_exec_order_action(pExecOrderAction, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnErrRtnForQuoteInsert(this: *mut ::std::os::raw::c_void, pInputForQuote: *mut CThostFtdcInputForQuoteField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance = get_api(this);
    instance.on_err_rtn_for_quote_insert(pInputForQuote, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRtnQuote(this: *mut ::std::os::raw::c_void, pQuote: *mut CThostFtdcQuoteField  ) {
    let instance = get_api(this);
    instance.on_rtn_quote(pQuote);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnErrRtnQuoteInsert(this: *mut ::std::os::raw::c_void, pInputQuote: *mut CThostFtdcInputQuoteField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance = get_api(this);
    instance.on_err_rtn_quote_insert(pInputQuote, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnErrRtnQuoteAction(this: *mut ::std::os::raw::c_void, pQuoteAction: *mut CThostFtdcQuoteActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance = get_api(this);
    instance.on_err_rtn_quote_action(pQuoteAction, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRtnForQuoteRsp(this: *mut ::std::os::raw::c_void, pForQuoteRsp: *mut CThostFtdcForQuoteRspField  ) {
    let instance = get_api(this);
    instance.on_rtn_for_quote_rsp(pForQuoteRsp);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRtnCFMMCTradingAccountToken(this: *mut ::std::os::raw::c_void, pCFMMCTradingAccountToken: *mut CThostFtdcCFMMCTradingAccountTokenField  ) {
    let instance = get_api(this);
    instance.on_rtn_c_f_m_m_c_trading_account_token(pCFMMCTradingAccountToken);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnErrRtnBatchOrderAction(this: *mut ::std::os::raw::c_void, pBatchOrderAction: *mut CThostFtdcBatchOrderActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance = get_api(this);
    instance.on_err_rtn_batch_order_action(pBatchOrderAction, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRtnOptionSelfClose(this: *mut ::std::os::raw::c_void, pOptionSelfClose: *mut CThostFtdcOptionSelfCloseField  ) {
    let instance = get_api(this);
    instance.on_rtn_option_self_close(pOptionSelfClose);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnErrRtnOptionSelfCloseInsert(this: *mut ::std::os::raw::c_void, pInputOptionSelfClose: *mut CThostFtdcInputOptionSelfCloseField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance = get_api(this);
    instance.on_err_rtn_option_self_close_insert(pInputOptionSelfClose, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnErrRtnOptionSelfCloseAction(this: *mut ::std::os::raw::c_void, pOptionSelfCloseAction: *mut CThostFtdcOptionSelfCloseActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance = get_api(this);
    instance.on_err_rtn_option_self_close_action(pOptionSelfCloseAction, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRtnCombAction(this: *mut ::std::os::raw::c_void, pCombAction: *mut CThostFtdcCombActionField  ) {
    let instance = get_api(this);
    instance.on_rtn_comb_action(pCombAction);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnErrRtnCombActionInsert(this: *mut ::std::os::raw::c_void, pInputCombAction: *mut CThostFtdcInputCombActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance = get_api(this);
    instance.on_err_rtn_comb_action_insert(pInputCombAction, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryContractBank(this: *mut ::std::os::raw::c_void, pContractBank: *mut CThostFtdcContractBankField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_contract_bank(pContractBank, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryParkedOrder(this: *mut ::std::os::raw::c_void, pParkedOrder: *mut CThostFtdcParkedOrderField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_parked_order(pParkedOrder, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryParkedOrderAction(this: *mut ::std::os::raw::c_void, pParkedOrderAction: *mut CThostFtdcParkedOrderActionField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_parked_order_action(pParkedOrderAction, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryTradingNotice(this: *mut ::std::os::raw::c_void, pTradingNotice: *mut CThostFtdcTradingNoticeField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_trading_notice(pTradingNotice, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryBrokerTradingParams(this: *mut ::std::os::raw::c_void, pBrokerTradingParams: *mut CThostFtdcBrokerTradingParamsField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_broker_trading_params(pBrokerTradingParams, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQryBrokerTradingAlgos(this: *mut ::std::os::raw::c_void, pBrokerTradingAlgos: *mut CThostFtdcBrokerTradingAlgosField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_qry_broker_trading_algos(pBrokerTradingAlgos, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQueryCFMMCTradingAccountToken(this: *mut ::std::os::raw::c_void, pQueryCFMMCTradingAccountToken: *mut CThostFtdcQueryCFMMCTradingAccountTokenField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_query_c_f_m_m_c_trading_account_token(pQueryCFMMCTradingAccountToken, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRtnFromBankToFutureByBank(this: *mut ::std::os::raw::c_void, pRspTransfer: *mut CThostFtdcRspTransferField  ) {
    let instance = get_api(this);
    instance.on_rtn_from_bank_to_future_by_bank(pRspTransfer);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRtnFromFutureToBankByBank(this: *mut ::std::os::raw::c_void, pRspTransfer: *mut CThostFtdcRspTransferField  ) {
    let instance = get_api(this);
    instance.on_rtn_from_future_to_bank_by_bank(pRspTransfer);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRtnRepealFromBankToFutureByBank(this: *mut ::std::os::raw::c_void, pRspRepeal: *mut CThostFtdcRspRepealField  ) {
    let instance = get_api(this);
    instance.on_rtn_repeal_from_bank_to_future_by_bank(pRspRepeal);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRtnRepealFromFutureToBankByBank(this: *mut ::std::os::raw::c_void, pRspRepeal: *mut CThostFtdcRspRepealField  ) {
    let instance = get_api(this);
    instance.on_rtn_repeal_from_future_to_bank_by_bank(pRspRepeal);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRtnFromBankToFutureByFuture(this: *mut ::std::os::raw::c_void, pRspTransfer: *mut CThostFtdcRspTransferField  ) {
    let instance = get_api(this);
    instance.on_rtn_from_bank_to_future_by_future(pRspTransfer);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRtnFromFutureToBankByFuture(this: *mut ::std::os::raw::c_void, pRspTransfer: *mut CThostFtdcRspTransferField  ) {
    let instance = get_api(this);
    instance.on_rtn_from_future_to_bank_by_future(pRspTransfer);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRtnRepealFromBankToFutureByFutureManual(this: *mut ::std::os::raw::c_void, pRspRepeal: *mut CThostFtdcRspRepealField  ) {
    let instance = get_api(this);
    instance.on_rtn_repeal_from_bank_to_future_by_future_manual(pRspRepeal);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRtnRepealFromFutureToBankByFutureManual(this: *mut ::std::os::raw::c_void, pRspRepeal: *mut CThostFtdcRspRepealField  ) {
    let instance = get_api(this);
    instance.on_rtn_repeal_from_future_to_bank_by_future_manual(pRspRepeal);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRtnQueryBankBalanceByFuture(this: *mut ::std::os::raw::c_void, pNotifyQueryAccount: *mut CThostFtdcNotifyQueryAccountField  ) {
    let instance = get_api(this);
    instance.on_rtn_query_bank_balance_by_future(pNotifyQueryAccount);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnErrRtnBankToFutureByFuture(this: *mut ::std::os::raw::c_void, pReqTransfer: *mut CThostFtdcReqTransferField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance = get_api(this);
    instance.on_err_rtn_bank_to_future_by_future(pReqTransfer, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnErrRtnFutureToBankByFuture(this: *mut ::std::os::raw::c_void, pReqTransfer: *mut CThostFtdcReqTransferField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance = get_api(this);
    instance.on_err_rtn_future_to_bank_by_future(pReqTransfer, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnErrRtnRepealBankToFutureByFutureManual(this: *mut ::std::os::raw::c_void, pReqRepeal: *mut CThostFtdcReqRepealField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance = get_api(this);
    instance.on_err_rtn_repeal_bank_to_future_by_future_manual(pReqRepeal, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnErrRtnRepealFutureToBankByFutureManual(this: *mut ::std::os::raw::c_void, pReqRepeal: *mut CThostFtdcReqRepealField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance = get_api(this);
    instance.on_err_rtn_repeal_future_to_bank_by_future_manual(pReqRepeal, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnErrRtnQueryBankBalanceByFuture(this: *mut ::std::os::raw::c_void, pReqQueryAccount: *mut CThostFtdcReqQueryAccountField  ,pRspInfo: *mut CThostFtdcRspInfoField  ) {
    let instance = get_api(this);
    instance.on_err_rtn_query_bank_balance_by_future(pReqQueryAccount, pRspInfo);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRtnRepealFromBankToFutureByFuture(this: *mut ::std::os::raw::c_void, pRspRepeal: *mut CThostFtdcRspRepealField  ) {
    let instance = get_api(this);
    instance.on_rtn_repeal_from_bank_to_future_by_future(pRspRepeal);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRtnRepealFromFutureToBankByFuture(this: *mut ::std::os::raw::c_void, pRspRepeal: *mut CThostFtdcRspRepealField  ) {
    let instance = get_api(this);
    instance.on_rtn_repeal_from_future_to_bank_by_future(pRspRepeal);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspFromBankToFutureByFuture(this: *mut ::std::os::raw::c_void, pReqTransfer: *mut CThostFtdcReqTransferField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_from_bank_to_future_by_future(pReqTransfer, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspFromFutureToBankByFuture(this: *mut ::std::os::raw::c_void, pReqTransfer: *mut CThostFtdcReqTransferField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_from_future_to_bank_by_future(pReqTransfer, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRspQueryBankAccountMoneyByFuture(this: *mut ::std::os::raw::c_void, pReqQueryAccount: *mut CThostFtdcReqQueryAccountField  ,pRspInfo: *mut CThostFtdcRspInfoField  ,nRequestID: c_int ,bIsLast: bool ) {
    let instance = get_api(this);
    instance.on_rsp_query_bank_account_money_by_future(pReqQueryAccount, pRspInfo, nRequestID, bIsLast);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRtnOpenAccountByBank(this: *mut ::std::os::raw::c_void, pOpenAccount: *mut CThostFtdcOpenAccountField  ) {
    let instance = get_api(this);
    instance.on_rtn_open_account_by_bank(pOpenAccount);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRtnCancelAccountByBank(this: *mut ::std::os::raw::c_void, pCancelAccount: *mut CThostFtdcCancelAccountField  ) {
    let instance = get_api(this);
    instance.on_rtn_cancel_account_by_bank(pCancelAccount);
}       

#[no_mangle]
pub unsafe extern "C" fn RustCtpActionTdOnRtnChangeAccountByBank(this: *mut ::std::os::raw::c_void, pChangeAccount: *mut CThostFtdcChangeAccountField  ) {
    let instance = get_api(this);
    instance.on_rtn_change_account_by_bank(pChangeAccount);
}       
