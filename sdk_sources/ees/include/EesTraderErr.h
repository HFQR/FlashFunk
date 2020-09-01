/*! \file  EesTraderErr.h
 *  \brief EES交易客户端API使用错误类型定义。
*/

#pragma  once

#ifndef _EES_TRADE_API_ERROR_H_
#define _EES_TRADE_API_ERROR_H_

/// 没有错误
#ifndef NO_ERROR
#define NO_ERROR                    0	
#endif

/// 连接服务失败
#define CONN_SERVER_FAILED          1

/// 连接服务失败，一般会在服务器断开时候这个错误
#define REMOTE_DISCONN_SERVER       2	

/// 本地网络断开，会在本地主动断开的时候，产生这个错误
#define LOCAL_DISCONN_SERVER        3

/// 网络出错，网络异常，会产生这个错误
#define NEWWORK_ERROR               4

/// 登录服务失败，会在登录的时候产生
#define LOGON_FAILED                5	

/// 用户进行操作，是需要提前登录的，如果没有登录会产生这个错误
#define NOT_LOGON                   6

/// 操作之前，需要连接服务器
#define NO_CONN_SERVER              7	

/// 错误的交易对象句柄
#define HANDLE_ERRNOR               8	 
/// 设置订单 token 错误
#define ORDER_TOKEN_ERROR			9

/// 非法的密码，目前只支持全空密码检测
#define INVALID_PASSWORD_ERROR		10

/// 调用一次下多个订单式，订单数量错误，必须在1-4之间
#define INVALID_ORDER_COUNT			11

/// 合约类型错误，只支持2-期权，以及3-期货
#define INVALID_SEC_TYPE			12

/// 期权合约代码错误，无法解析的期权合约代码，形式必须为m1703-C-2250
#define INVALID_OPT_SYMBOL			13

/// 黄金合约代码错误，超出了交易的合约代码限制
#define INVALID_SGE_SYMBOL			14

/// 流控，客户端根据服务器配置的流控参数，阻止客户短时间密集发送撤单或者下单
#define CLT_ORDER_SPEED_FLOWCTRL    15

/// 流控参数修改，超出了允许的范围	
#define FLOWCTRL_PARAM_INVALID		16

/// 在UDP模式下，使用了EnterMultiOrders接口，该接口只能在TCP下使用
#define ENTER_MULTI_ORDER_BY_UDP	17

#endif
