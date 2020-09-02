/*! \file  EesTraderDefine.h
 *  \brief EES交易客户端API使用的消息体定义。
 *
 *  本文件详细描述了EES交易客户端API使用的数据结构以及消息体。 
*/
#pragma  once 




#ifndef _EES_TRADE_API_STRUCT_DEFINE_H_
#define _EES_TRADE_API_STRUCT_DEFINE_H_

#include <string.h>


#define SL_EES_API_VERSION    "3.1.3.60"				///<  api版本号

typedef int RESULT;										///< 定义返回值 
typedef int ERR_NO;										///< 定义错误值 

typedef unsigned int			EES_ClientToken;					///< API端订单的客户端ID
typedef int						EES_UserID;						///< 帐户ID
typedef long long int			EES_MarketToken;					///< 订单的市场 ID
typedef int						EES_TradingDate;					///< 交易日
typedef unsigned long long int	EES_Nanosecond;					///< 从1970年1月1日0时0分0秒开始的纳秒时间，请使用ConvertFromTimestamp接口转换为可读的时间

typedef char    EES_Account[17];						///< 交易帐户
typedef char    EES_ProductID[5];						///< 期货的产品类型

typedef char    EES_ReasonText[88];						///< 错误描述
typedef char	EES_ReasonText2[81];					///< 用于留出交易所代码和交易所错误代码的市场拒绝结构体
typedef char	EES_GrammerResultText[1024];			///< 语法检查错误描述
typedef	char	EES_RiskResultText[1024];				///< 风控检查错误描述

typedef char    EES_GrammerResult[32];					///< 下单语法检查
typedef char    EES_RiskResult[96];						///< 下单风控检查
                              
typedef char    EES_Symbol[20];							///< 交易合约编码
typedef char    EES_SymbolName[21];						///< 交易合约名称

typedef char    EES_MarketOrderId[25];                  ///< 交易所订单号
typedef char	EES_MarketExecId[25];					///< 交易所成交号
typedef unsigned char EES_MarketSessionId;				///< 交易所席位代码

typedef unsigned char EES_SideType;						///< 买卖方向
#define EES_SideType_open_long                  1		///< =买单（开今）
#define EES_SideType_close_today_long           2		///< =卖单（平今）
#define EES_SideType_close_today_short          3		///< =买单（平今）
#define EES_SideType_open_short                 4		///< =卖单（开今）
#define EES_SideType_close_ovn_short            5		///< =买单（平昨）
#define EES_SideType_close_ovn_long             6		///< =卖单（平昨）
#define EES_SideType_opt_exec					11		///< =期权行权
#define EES_SideType_close_short				21		///< =买单（平仓）
#define EES_SideType_close_long					22		///< =卖单（平仓）


typedef unsigned char EES_ExchangeID;					///< 交易所ID
#define EES_ExchangeID_sh_cs                    100		///< =上交所
#define EES_ExchangeID_sz_cs                    101		///< =深交所
#define EES_ExchangeID_cffex                    102		///< =中金所
#define EES_ExchangeID_shfe                     103		///< =上期所
#define EES_ExchangeID_dce                      104		///< =大商所
#define EES_ExchangeID_zcze                     105		///< =郑商所
#define EES_ExchangeID_ine						106		///< =能源中心
#define EES_ExchangeID_sge						107		///< =上海金交所
#define EES_ExchangeID_done_away                255		///< =Done-away 


typedef unsigned char EES_SecType;						///< 交易品种类型 
#define EES_SecType_cs                          1		///< =股票
#define EES_SecType_options                     2		///< =期权
#define EES_SecType_fut                         3		///< =期货


typedef unsigned char EES_ForceCloseType;				///< 强平原因 
#define EES_ForceCloseType_not_force_close      0		///< =非强平  
#define EES_ForceCloseType_not_enough_bp        1		///< =资金不足  
#define EES_ForceCloseType_not_enough_position  2		///< =客户超仓  
#define EES_ForceCloseType_not_enough_position2 3		///< =会员超仓  
#define EES_ForceCloseType_not_round_lot        4		///< =持仓非整数倍  
#define EES_ForceCloseType_invalid              5		///< =违规
#define EES_ForceCloseType_other                6		///< =其他

typedef unsigned char EES_OptExecFlag;
#define EES_OptExecFlag_normal					0		///< =常规行权
#define EES_OptExecFlag_dont_auto_exec			1		///< =申请不自动执行
#define EES_OptExecFlag_fut_hedge				2		///< =申请期货仓位自动对冲

typedef unsigned char EES_OrderState;					///< 订单状态
#define EES_OrderState_order_live               1		///< =单子活着
#define EES_OrderState_order_dead               2		///< =单子死了

typedef int           EES_Previlege;					///< 目前硬件暂不支持，也就是说都是完全操作权限 99：完全操作  1：只读 2：只平仓
#define EES_Previlege_open_and_close            99		///< =所有权限
#define EES_Previlege_readonly                  1		///< =只读
#define EES_Previlege_close_only                2		///< =只能平仓


typedef int     EES_PosiDirection;						///< 多空方向 1：多头 5：空头
#define EES_PosiDirection_long					1		///< =多头
#define EES_PosiDirection_short					5		///< =空头


typedef unsigned char EES_RejectedMan;					///< 被谁拒绝，盛立系统还是下面连的交易所 1=盛立
#define EES_RejectedMan_by_shengli				1		///< =被盛立拒绝

typedef unsigned char EES_ReasonCode;					///< 单子被拒绝的理由。这张表将来会增加。请见下表。

typedef unsigned char EES_CxlReasonCode;				///< 撤单成功的原因
#define EES_CxlReasonCode_by_account			1		///< =用户撤单
#define EES_CxlReasonCode_timeout				2		///< =系统timeout, 单子到期被交易所系统取消
#define EES_CxlReasonCode_supervisory			3		///< =Supervisory, 被盛立系统管理者取消
#define EES_CxlReasonCode_by_market				4		///< =被市场拒绝
#define EES_CxlReasonCode_another				255		///< =其他

typedef unsigned char EES_OrderStatus;					///< 按照二进制与存放多个订单状态
#define EES_OrderStatus_shengli_accept			0x80	///< bit7=1：EES系统已接受
#define EES_OrderStatus_mkt_accept				0x40	///< bit6=1：市场已接受或者手工干预订单
#define EES_OrderStatus_executed				0x20	///< bit5=1：已成交或部分成交
#define EES_OrderStatus_cancelled				0x10 	///< bit4=1：已撤销, 可以是部分成交后撤销
#define EES_OrderStatus_cxl_requested			0x08	///< bit3=1：发过客户撤单请求
#define EES_OrderStatus_reserved1				0x04	///< bit2：保留, 目前无用
#define EES_OrderStatus_reserved2				0x02	///< bit1：保留, 目前无用
#define EES_OrderStatus_closed					0x01	///< bit0=1：已关闭, (拒绝/全部成交/已撤销)

typedef unsigned int EES_OrderTif;						///< 成交条件
#define EES_OrderTif_IOC						0		///< 当需要下FAK/FOK订单时，需要将TIF设置为0
#define EES_OrderTif_Day						99998	///< 日内报单

typedef unsigned long long int EES_CustomFieldType;		///< 用户可存放自定义8位数字值	

typedef unsigned char EES_HedgeFlag;					///< 投机套利标志
#define EES_HedgeFlag_Arbitrage				1			///< 套利
#define EES_HedgeFlag_Speculation			2			///< 投机
#define EES_HedgeFlag_Hedge					3			///< 套保

typedef int EES_LogonResult;
#define EES_LOGON_OK							0		///< 成功
#define EES_LOGON_AUTHENTICATION_FAILED			1		///< 用户名/密码不对
#define EES_LOGON_ACCOUNT_NOT_BOUND				2		///< 该用户未绑定任何资金账户
#define EES_LOGON_ALREADY_LOGON					3		///< 如果系统配置为重复登录时，不允许后来的用户登录，那么就会返回这个值，目前一般不会这么配置，所以应该不会收到这个错误码
#define EES_LOGON_ANOTHER_LOGON					4		///< 已经登录成功后，再次登录
#define EES_LOGON_MISSING_EXTRA_INFO			5		///< 缺少客户端标识、mac地址
#define EES_LOGON_INTERNAL_ERROR				6		///< 系统内部错误
#define EES_LOGON_NOT_USING_QUERY_PORT			7		///< 使用新版本API不再会发生这个错误
#define EES_LOGON_CONNECT_QUERY_PORT_FAILED		8		///< 连接查询通道失败
#define EES_LOGON_RPTR_INVALID_INFO				11		///< 无有效采集信息
#define EES_LOGON_RPTR_IP_ERROR					12		///< 采集上报和交易端的远端IP不同
#define EES_LOGON_RPTR_APPID_AUTH_CODE_ERROR	13		///< 校验APPID或授权码错误
#define EES_LOGON_SYSTEM_ENV_CRITICAL			99		///< 服务器目前环境恶劣，内存、硬盘等空间不足，所有登录都被禁止

typedef int EES_ChangePasswordResult;
#define EES_ChangePasswordResult_Success		0		///< 成功
#define EES_ChangePasswordResult_OldPwdNotMatch	1		///< 老密码不对
#define EES_ChangePasswordResult_NewPwdInvalid	2		///< 新密码非法，如空白等
#define	EES_ChangePasswordResult_NotLogIn		3		///< 尚未登录
#define	EES_ChangePasswordResult_InternalError	99		///< 系统后台其他问题

typedef unsigned char EES_OrderType;
#define EES_Order_Type_Limt						1		///< 限价
#define EES_Order_Type_Market					2		///< 市价，尚未支持
#define EES_Order_Type_Stop_Profit_Limit		3		///< 限价止盈
#define EES_Order_Type_Stop_Loss_Limit			4		///< 限价止损
#define EES_Order_Type_Stop_Profit_Market		5		///< 市价止盈，尚未支持
#define EES_Order_Type_Stop_Loss_Market			6		///< 市价止损，尚未支持

typedef unsigned char EES_OrderExchangeStatus;
#define EES_Order_Exchange_Status_Accept			0		///< 常规市场接受
#define EES_Order_Exchange_Status_Triggered			1		///< 止盈止损单触发
#define EES_Order_Exchange_Status_Suspended			2		///< 订单挂起
#define EES_Order_Exchange_Status_Activated			3		///< 订单激活

#pragma pack(push, 1)


/// 登录返回的消息
struct EES_LogonResponse
{
	EES_LogonResult		m_Result;							///< 参见本文档EES_LogonResult定义
	EES_UserID			m_UserId;							///< 登录名对应的用户ID
	unsigned int		m_TradingDate;						///< 交易日，格式为yyyyMMdd的int型值
	EES_ClientToken		m_MaxToken;							///< 以前的最大 token 
	unsigned int		m_OrderFCCount;						///< 下单流控参数，单位时间内下单次数限制的次数
	unsigned int		m_OrderFCInterval;					///< 下单流控参数，单位时间内下单次数限制单位时间，微秒值
	unsigned int		m_CancelFCCount;					///< 撤单流控参数，单位时间内撤单次数限制的次数
	unsigned int		m_CancelFCInterval;					///< 撤单流控参数，单位时间内撤单次数限制单位时间，微秒值
};


/// 下单消息
struct EES_EnterOrderField
{ 
	EES_Account         m_Account;						///< 用户代码
	EES_SideType        m_Side;							///< 买卖方向
	EES_ExchangeID      m_Exchange;						///< 交易所
	EES_Symbol          m_Symbol;						///< 合约代码
	EES_SecType         m_SecType;						///< 交易品种
	double              m_Price;						///< 价格
	unsigned int        m_Qty;							///< 数量
	EES_OptExecFlag		m_OptExecFlag;					///< 期权行权标志位
	EES_ClientToken		m_ClientOrderToken;				///< 整型，必须保证，这次比上次的值大，并不一定需要保证连续
	EES_OrderTif		m_Tif;							///< 当需要下FAK/FOK报单时，需要设置为EES_OrderTif_IOC
	unsigned int		m_MinQty;						///< 当需要下FAK/FOK报单时，该值=0：映射交易所的FAK-任意数量；
														///< 当需要下FAK/FOK报单时，该值>0且<m_Qty：映射交易所的FAK-最小数量，且最小数量即该值
														///< 当需要下FAK/FOK报单时，该值=m_Qty：映射交易所的FOK；
														///< 常规日内报单，请设为0.无论哪种情况，该值如果>m_Qty将被REM系统拒绝
	
	EES_CustomFieldType m_CustomField;					///< 用户自定义字段，8个字节。用户在下单时指定的值，将会在OnOrderAccept，OnQueryTradeOrder事件中返回
	EES_MarketSessionId m_MarketSessionId;				///< 交易所席位代码，从OnResponseQueryMarketSessionId获取合法值，如果填入0或者其他非法值，REM系统将自行决定送单的席位	，除非m_ForceMarketSessionId为true
	EES_HedgeFlag		m_HedgeFlag;					///< 投机套利标志
	unsigned char		m_ForceMarketSessionId;			///< 如果为true，当客户指定席位代码，但是该席位不可用或者非法时，指示服务器不要自行决定送单席位，而是拒绝下单
	unsigned char		m_DoNotAdjustCoverSide;			///< 默认情况下，如果是中金/大连交易所的“平今/平昨”订单，API会自动将之转换为“平仓”订单，该值如果为true，则不进行此转换，一般用于测试场景
	EES_OrderType		m_OrderType;					///< 目前支持：1=限价; 3=限价止盈; 4=限价止损; 这3种，且仅对大连订单真正有效。
	double				m_StopPrice;					///< 当m_OrderType为 3/4/5/6 时必填，其它情况填0
	EES_EnterOrderField()
	{
		memset(this, 0, sizeof(*this));
		m_Tif = EES_OrderTif_Day;
		m_MinQty = 0;
		m_MarketSessionId = 0;
		m_HedgeFlag = EES_HedgeFlag_Speculation;
		m_SecType = EES_SecType_fut;
		m_ForceMarketSessionId = 0;
		m_DoNotAdjustCoverSide = 0;
		m_OrderType = EES_Order_Type_Limt;
		m_StopPrice = 0.0;
	}

};

/// 下单被柜台系统接受消息
struct EES_OrderAcceptField
{ 
	EES_ClientToken     m_ClientOrderToken;				///< 下单的时候，返回给你的token
	EES_MarketToken     m_MarketOrderToken;				///< 市场里面挂单的token
	EES_OrderState      m_OrderState;					///< 订单状态
	EES_UserID          m_UserID;						///< 订单的 user id 
	EES_Nanosecond      m_AcceptTime;					///< 从1970年1月1日0时0分0秒开始的纳秒时间，请使用ConvertFromTimestamp接口转换为可读的时间
	EES_Account         m_Account;						///< 用户代码
	EES_SideType        m_Side;							///< 买卖方向
	EES_ExchangeID      m_Exchange;						///< 交易所
	EES_Symbol          m_Symbol;						///< 合约代码
	EES_SecType         m_SecType;						///< 交易品种
	double              m_Price;						///< 价格
	unsigned int        m_Qty;							///< 数量
	EES_OptExecFlag		m_OptExecFlag;					///< 期权行权标志位
	EES_OrderTif		m_Tif;							///< 用户下单时指定的值
	unsigned int		m_MinQty;						///< 用户下单时指定的值
	EES_CustomFieldType m_CustomField;					///< 用户下单时指定的值
	EES_MarketSessionId m_MarketSessionId;				///< 报单送往交易所的席位代码，有可能和下单时指定的不同。不同的原因有：当前该席位尚未连接好；指定的席位代号非法等；指定0：由REM自行决定
	EES_HedgeFlag		m_HedgeFlag;					///< 投机套利标志
};

/// 下单被市场接受消息
struct EES_OrderMarketAcceptField
{
	EES_Account				m_Account;          ///< 用户代码
	EES_MarketToken			m_MarketOrderToken; ///< 盛立系统产生的单子号，和盛立交流时可用该号。
	EES_MarketOrderId		m_MarketOrderId;    ///< 市场订单号
	EES_Nanosecond			m_MarketTime;       ///< 市场时间信息
	EES_UserID				m_UserID;			///< 订单的 user id 
	EES_ClientToken			m_ClientOrderToken; ///< 订单的ClientToken
	EES_OrderExchangeStatus	m_ExchangeStatus;	///< 参考EES_OrderExchangeStatus定义，0=普通接受; 1=止盈止损单触发; 2=订单挂起; 3=订单激活 
};

/// 下单被柜台系统拒绝
struct EES_OrderRejectField
{
	EES_UserID				m_Userid;			///< 原来单子的用户，对应着LoginID。
	EES_Nanosecond			m_Timestamp;		///< 从1970年1月1日0时0分0秒开始的纳秒时间，请使用ConvertFromTimestamp接口转换为可读的时间
	EES_ClientToken			m_ClientOrderToken;	///< 原来单子的token
	EES_RejectedMan			m_RejectedMan;		///< 被谁拒绝，盛立系统还是下面连的交易所 1=盛立
	EES_ReasonCode			m_ReasonCode;		///< 单子被拒绝的理由。这张表将来会增加。请见下表。
	EES_GrammerResult		m_GrammerResult;	///< 语法检查的结果数组，每个字符映射一种检查错误原因，见文件末尾的附录
	EES_RiskResult			m_RiskResult;		///< 风控检查的结果数组，每个字符映射一种检查错误原因，见文件末尾的附录
	EES_GrammerResultText	m_GrammerText;		///< 语法检查的结果文字描述
	EES_RiskResultText		m_RiskText;			///< 风控检查的结果文字描述			
};

/// 下单被市场拒绝
struct EES_OrderMarketRejectField
{
	EES_Account     m_Account;           ///< 用户代码
	EES_MarketToken m_MarketOrderToken;	 ///< 盛立系统产生的单子号，和盛立交流时可用该号。
	EES_Nanosecond  m_MarketTimestamp;   ///< 市场时间信息, 从1970年1月1日0时0分0秒开始的纳秒时间，请使用ConvertFromTimestamp接口转换为可读的时间
	EES_ReasonText2	m_ReasonText;		 ///< 交易所返回的错误字符串，GB2312编码
	EES_ExchangeID  m_ExchangeId;		 ///< 产生错误码的交易所
	int				m_ExchangeErrorId;	 ///< 交易所api给出的错误码
	EES_UserID      m_UserID;			 ///< 订单的 user id 
	EES_ClientToken m_ClientOrderToken;  ///< 订单的ClientToken
};

/// 订单成交消息体
struct EES_OrderExecutionField
{
	EES_UserID        m_Userid;							///< 原来单子的用户，对应着LoginID。
	EES_Nanosecond    m_Timestamp;						///< 成交时间，从1970年1月1日0时0分0秒开始的纳秒时间
	EES_ClientToken   m_ClientOrderToken;				///< 原来单子的你的token
	EES_MarketToken   m_MarketOrderToken;				///< 盛立系统产生的单子号，和盛立交流时可用该号。
	unsigned int      m_Quantity;						///< 单子成交量
	double            m_Price;							///< 单子成交价
	EES_MarketToken   m_ExecutionID;					///< 单子成交号(TAG 1017)
	EES_MarketExecId  m_MarketExecID;					///< 交易所成交号
};

/// 下单撤单指令
struct EES_CancelOrder
{
	EES_MarketToken m_MarketOrderToken;					///< 盛立系统产生的单子号，和盛立交流时可用该号。
	unsigned int    m_Quantity;							///< 这是该单子被取消后所希望剩下的数量，如为0，改单子为全部取消。在中国目前必须填0，其他值当0处理。
	EES_Account     m_Account;							///< 帐户ID号
	EES_MarketSessionId m_MarketSessionId;				///< 交易所席位代码，从OnResponseQueryMarketSessionId获取合法值，如果填入0或者其他非法值，REM系统将自行决定送单的席位	，除非m_ForceMarketSessionId为true	
	unsigned char	m_ForceMarketSessionId;				///< 如果为1，当客户指定席位代码，但是该席位不可用或者非法时，指示服务器不要自行决定送单席位，而是拒绝下单
	unsigned char   m_ActionFlag;                       ///< 0=撤单, 1=挂起, 2=激活，注意仅上期和能源中心支持激活挂起。
	EES_CancelOrder()
	{
		memset(this, 0, sizeof(*this));
	}
};

/// 订单撤销完成
struct EES_OrderCxled
{ 
	EES_UserID        m_Userid;							///< 原来单子的用户，对应着LoginID。
	EES_Nanosecond    m_Timestamp;						///< 撤单时间，从1970年1月1日0时0分0秒开始的纳秒时间，请使用ConvertFromTimestamp接口转换为可读的时间
	EES_ClientToken   m_ClientOrderToken;				///< 原来单子的token
	EES_MarketToken   m_MarketOrderToken;				///< 盛立系统产生的单子号，和盛立交流时可用该号。
	unsigned int      m_Decrement;						///< 这次信息所取消的单子量
	EES_CxlReasonCode m_Reason;							///< 原因，见下表
};

/// 查询订单的结构
struct EES_QueryAccountOrder
{
	EES_UserID			m_Userid;						///< 原来单子的用户，对应着LoginID。
	EES_Nanosecond		m_Timestamp;					///< 订单创建时间，从1970年1月1日0时0分0秒开始的纳秒时间，请使用ConvertFromTimestamp接口转换为可读的时间
	EES_ClientToken		m_ClientOrderToken;				///< 原来单子的token 
	EES_SideType		m_SideType;						///< 1 = 买单（开今） 2 = 卖单（平今）  3= 买单（平今） 4 = 卖单（开今）  5= 买单（平昨） 6= 卖单（平昨） 7=买单（强平昨）  8=卖单（强平昨）  9=买单（强平今）  10=买单（强平今）
	unsigned int		m_Quantity;						///< 数量（股票为股数，期货为手数）
	EES_SecType			m_InstrumentType;				///< 1＝Equity 股票 2＝Options 期权 3＝Futures 期货
	EES_Symbol			m_symbol;						///< 股票代码，期货代码或者期权代码，以中国交易所标准 (目前6位就可以)
	double				m_Price;						///< 价格
	EES_Account			m_account;						///< 61 16  Alpha 客户帐号.  这个是传到交易所的客户帐号。验证后，必须是容许的值，也可能是这个连接的缺省值。
	EES_ExchangeID		m_ExchengeID;					///< 100＝上交所  101=深交所  102=中金所  103=上期所  104=大商所  105=郑商所  255= done-away  See appendix 
	EES_OptExecFlag		m_OptExecFlag;					///< 期权行权标志位
	EES_MarketToken		m_MarketOrderToken;				///< 盛立系统产生的单子号，和盛立交流时可用该号。 
	EES_OrderStatus		m_OrderStatus;					///< 请参考EES_OrderStatus的定义
	EES_Nanosecond		m_CloseTime;					///< 订单关闭事件，从1970年1月1日0时0分0秒开始的纳秒时间，请使用ConvertFromTimestamp接口转换为可读的时间
	int					m_FilledQty;					///< 0  4 Int4  成交数量
	EES_OrderTif		m_Tif;							///< 用户下单时指定的值
	unsigned int		m_MinQty;						///< 用户下单时指定的值
	EES_CustomFieldType m_CustomField;					///< 用户下单时指定的值
	EES_MarketOrderId	m_MarketOrderId;				///< 交易所单号
	EES_HedgeFlag		m_HedgeFlag;					///< 投机套利标志
};
/// 查询订单成交的结构
struct EES_QueryOrderExecution
{
	EES_UserID			m_Userid;						///< 原来单子的用户，对应着LoginID。
	EES_Nanosecond		m_Timestamp;					///< 成交时间，从1970年1月1日0时0分0秒开始的纳秒时间，请使用ConvertFromTimestamp接口转换为可读的时间
	EES_ClientToken		m_ClientOrderToken;				///< 原来单子的你的token
	EES_MarketToken		m_MarketOrderToken;				///< 盛立系统产生的单子号，和盛立交流时可用该号。
	unsigned int		m_ExecutedQuantity;				///< 单子成交量
	double				m_ExecutionPrice;				///< 单子成交价
	EES_MarketToken		m_ExecutionID;					///< 单子成交号(TAG 1017)
	EES_MarketExecId	m_MarketExecID;					///< 交易所成交号
};

/// 当一个账户的所有订单和成交都回滚完成后的消息
struct EES_QueryAccountTradeFinish
{
	EES_Account			m_account;						///< 账户ID
};

/// 帐户信息基本信息
struct EES_AccountInfo
{
	EES_Account			m_Account;						///< 帐户ID 
	EES_Previlege		m_Previlege;					///< 操作权限，目前硬件暂不支持，也就是说都是完全操作权限 99：完全操作  1：只读 2：只平仓
	double				m_InitialBp;					///< 初始权益
	double				m_AvailableBp;					///< 总可用资金
	double				m_Margin;						///< 所有仓位占用的保证金
	double				m_FrozenMargin;					///< 所有挂单冻结的保证金
	double				m_CommissionFee;				///< 已扣除的手续费总金额
	double				m_FrozenCommission;				///< 挂单冻结的总手续费金额
};

/// 帐户的仓位信息
struct EES_AccountPosition
{
	EES_Account			m_actId;						///< Value  Notes
	EES_Symbol			m_Symbol;						///< 合约名称/股票代码
	EES_PosiDirection	m_PosiDirection;				///< 多空方向 1：多头 5：空头
	unsigned int		m_InitOvnQty;					///< 隔夜仓初始数量，这个值不会变化，除非通过HelpDesk手工修改
	unsigned int		m_OvnQty;						///< 当前隔夜仓数量，可以为0
	unsigned int		m_FrozenOvnQty;					///< 冻结的昨仓数量
	unsigned int		m_TodayQty;						///< 当前今仓数量，可能为0
	unsigned int		m_FrozenTodayQty;				///< 冻结的今仓数量
	double				m_OvnMargin;					///< 隔夜仓占用保证金
	double				m_TodayMargin;					///< 今仓占用的保证金
	double				m_PositionCost;
	EES_HedgeFlag		m_HedgeFlag;					///< 仓位对应的投机套利标志
};

struct EES_AccountOptionPosition
{
	EES_Account			m_actId;						///< Value  Notes
	EES_Symbol			m_Symbol;						///< 合约名称/股票代码
	EES_PosiDirection	m_PosiDirection;				///< 多空方向 1：多头 5：空头
	EES_Symbol			m_UnderlyingSymbol;				
	char				m_CallPut;
	double				m_StrikePrice;
	unsigned int		m_ExpireDate;
	unsigned int		m_InitOvnQty;					///< 隔夜仓初始数量，这个值不会变化，除非通过HelpDesk手工修改		
	unsigned int		m_CurTotalQty;
	unsigned int		m_CoverLockedQty;
	unsigned int		m_ExecPendingQty;
	unsigned int		m_ExecAppliedQty;
	unsigned int		m_CxlExecPendingQty;
	double				m_LiquidPl;
	double				m_AvgPrice;	
	double				m_TotalCommissionFee;
	EES_HedgeFlag		m_HedgeFlag;					///< 仓位对应的投机套利标志
};

/// 帐户的仓位信息
struct EES_AccountBP
{
	EES_Account			m_account;						///< Value  Notes
	double				m_InitialBp;					///< 初始权益
	double				m_AvailableBp;					///< 总可用资金
	double				m_Margin;						///< 所有仓位占用的保证金
	double				m_FrozenMargin;					///< 所有挂单冻结的保证金
	double				m_CommissionFee;				///< 已扣除的手续费总金额
	double				m_FrozenCommission;				///< 挂单冻结的总手续费金额
	double				m_OvnInitMargin;				///< 初始昨仓保证金
	double				m_TotalLiquidPL;				///< 总平仓盈亏
	double				m_TotalMarketPL;				///< 总持仓盈亏
};

/// 合约列表
struct EES_SymbolField
{
	EES_SecType			m_SecType;						///< 3=Future，目前仅支持期货
	EES_Symbol			m_symbol;						///< 合约名称/股票代码
	EES_SymbolName		m_symbolName;					///< 合约名称
	EES_ExchangeID		m_ExchangeID;					///< 102=中金所   103=上期所    104=大商所    105=郑商所
	EES_ProductID		m_ProdID;						///< 产品代码
	unsigned int		m_DeliveryYear;					///< 交割年
	unsigned int		m_DeliveryMonth;				///< 交割月
	unsigned int		m_MaxMarketOrderVolume;			///< 市价单最大下单量
	unsigned int		m_MinMarketOrderVolume;			///< 市价单最小下单量
	unsigned int		m_MaxLimitOrderVolume;			///< 限价单最大下单量
	unsigned int		m_MinLimitOrderVolume;			///< 限价单最小下单量
	unsigned int		m_VolumeMultiple;				///< 合约乘数
	double				m_PriceTick;					///< 最小变动价位 
	unsigned int		m_CreateDate;					///< 创建日期
	unsigned int		m_OpenDate;						///< 上市日期
	unsigned int		m_ExpireDate;					///< 到期日, 期权到期日也用该值
	unsigned int		m_StartDelivDate;				///< 开始交割日
	unsigned int		m_EndDelivDate;					///< 结束交割日
	unsigned int		m_InstLifePhase;				///< 合约生命周期状态   0=未上市    1=上市    2=停牌    3=到齐
	unsigned int		m_IsTrading;					///< 当前是否交易   0=未交易    1=交易
	double				m_StrikePrice;					///< 期权合约的执行价, 期货该值为0
	char				m_CallPut;						///< 期权是认沽还是认购
	EES_Symbol			m_UnderlyingSymbol;				///< 期权标的物期货合约
};

/// 查询帐户的保证金率
struct EES_AccountMargin
{
	EES_SecType			m_SecType;						///< 3=Future，目前仅支持期货
	EES_Symbol			m_symbol;						///< 合约名称/股票代码
	EES_ExchangeID		m_ExchangeID;					///< 102=中金所   103=上期所    104=大商所    105=郑商所
	EES_ProductID		m_ProdID;						///< 4  Alpha 产品代码
	double				m_LongMarginRatio;				///< 多仓保证金率
	double				m_ShortMarginRatio;				///< 空仓保证金率，目前用不上
};

/// 帐户合约费率查询
struct EES_AccountFee
{
	EES_SecType			m_SecType;						///<  3=Future，目前仅支持期货
	EES_Symbol			m_symbol;						///<  合约名称/股票代码
	EES_ExchangeID		m_ExchangeID;					///<  102=中金所    103=上期所    104=大商所    105=郑商所
	EES_ProductID		m_ProdID;						///<  产品代码
	double				m_OpenRatioByMoney;				///<  开仓手续费率，按照金额
	double				m_OpenRatioByVolume;			///<  开仓手续费率，按照手数
	double				m_CloseYesterdayRatioByMoney;	///<  平昨手续费率，按照金额
	double				m_CloseYesterdayRatioByVolume;	///<  平昨手续费率，按照手数
	double				m_CloseTodayRatioByMoney;		///<  平今手续费率，按照金额
	double				m_CloseTodayRatioByVolume;		///<  平今手续费率，按照手数
	EES_PosiDirection	m_PositionDir;					///<  1: 多头；2: 空头
};

/// 撤单被拒绝的消息体
struct EES_CxlOrderRej
{
	EES_Account			m_account;						///< 客户帐号. 
	EES_MarketToken		m_MarketOrderToken;				///< 盛立内部用的orderID
	unsigned int		m_ReasonCode;					///< 错误码，需要配合m_ExchangeID，获取真实原因。参见本文件最后一段
	EES_ReasonText		m_ReasonText;					///< 错误字符串，由API填写
	EES_UserID			m_UserID;						///< 要撤订单的 user id，如果是因为找不到原订单，则为0
	EES_ClientToken		m_ClientOrderToken;				///< 要撤订单的ClientToken，如果是因为找不到原订单，则为0
	EES_ExchangeID		m_ExchangeID;					///< 撤单拒绝的源，0=柜台直接拒绝。102/103/104等值，表示交易所的撤单拒绝错误码
	unsigned char		m_ActionFlag;                   ///< 0=撤单, 1=挂起, 2=激活，注意仅上期和能源中心支持激活挂起。
};

/// 被动订单
struct EES_PostOrder
{
	EES_UserID			m_Userid;						///< 原来单子的用户，对应着LoginID。
	EES_Nanosecond		m_Timestamp;					///< 订单创建时间，从1970年1月1日0时0分0秒开始的纳秒时间，请使用ConvertFromTimestamp接口转换为可读的时间
	EES_MarketToken		m_MarketOrderToken;				///< 盛立系统产生的单子号，和盛立交流时可用该号。
	EES_ClientToken		m_ClientOrderToken;				///< 不起作用，设为０
	EES_SideType		m_SideType;						///< Buy/Sell Indicator 27  1 Int1  1 = 买单（开今）    2 = 卖单（平今）    3= 买单（平今）   4= 卖单（开今）   5= 买单（平昨）   6= 卖单（平昨）   7= 买单 （强平昨）    8= 卖单 （强平昨）    9= 买单 （强平今）    10=买单 （强平今）
	unsigned int		m_Quantity;						///< 数量（股票为股数，期货为手数）
	EES_SecType			m_SecType;						///< 1＝Equity 股票   2＝Options 期权   3＝Futures 期货
	EES_Symbol			m_Symbol;						///< 股票代码，期货代码或者期权代码，以中国交易所标准 (目前6位就可以)
	double				m_price;						///< 价格 
	EES_Account			m_account;						///< 客户帐号.  这个是传到交易所的客户帐号。验证后，必须是容许的值，也可能是这个连接的缺省值。
	EES_ExchangeID		m_ExchangeID;					///< 255=Done-away
	EES_OptExecFlag		m_OptExecFlag;					///< 期权行权标志位
	EES_OrderState		m_OrderState;					///< 单子状态，绝大多时候是1，但是也有可能是2.    1=order live（单子活着）    2=order dead（单子死了）
	EES_MarketOrderId	m_ExchangeOrderID;				///< 交易所单号，如果是人工被动单，该值为空白
	EES_HedgeFlag		m_HedgeFlag;					///< 投机套利标志
};

/// 被动成交
struct EES_PostOrderExecution
{
	EES_UserID			m_Userid;						///< 原来单子的用户，对应着LoginID。
	EES_Nanosecond		m_Timestamp;					///< 被动成交时间，从1970年1月1日0时0分0秒开始的纳秒时间，请使用ConvertFromTimestamp接口转换为可读的时间
	EES_MarketToken		m_MarketOrderToken;				///< 盛立系统产生的单子号，和盛立交流时可用该号。
	unsigned int		m_ExecutedQuantity;				///< 单子成交量
	double				m_ExecutionPrice;				///< 单子成交价
	EES_MarketToken		m_ExecutionNumber;				///< 单子成交号
	EES_MarketExecId	m_MarketExecID;					///< 交易所成交号
};


struct EES_ExchangeMarketSession
{	
	EES_ExchangeID			m_ExchangeID;					///< 102=中金所    103=上期所    104=大商所    105=郑商所	
	unsigned char			m_SessionCount;					///该交易所可用连接数量，也即为m_SessionId的前多少位有效。最大255
	EES_MarketSessionId		m_SessionId[255];				///合法的交易所连接代号
};

struct EES_SymbolStatus
{	
	EES_ExchangeID	m_ExchangeID;		///< 102=中金所    103=上期所    104=大商所    105=郑商所	
	EES_Symbol		m_Symbol;			///< 合约代码
	unsigned char	m_InstrumentStatus;	///< 交易状态： '0':开盘前; '1':非交易; '2':连续交易; '3':集合竞价报单; '4'集合竞价价格平衡; '5':集合竞价撮合; '6': 收盘;
	unsigned int	m_TradingSegmentSN;	///< 交易阶段编号
	char			m_EnterTime[9];		///< 进入本状态时间
	unsigned char	m_EnterReason;		///< 进入本状态原因: '1': 自动切换; '2': 手动切换; '3': 熔断; '4': 熔断手动;
};

struct EES_MarketMBLData
{
	unsigned int		m_RequestId;					///< 请求时所填的id
	unsigned int		m_Result;						///< 0为正常返回，非0为出现错误
	EES_Symbol			m_symbol;						///< 合约名称/股票代码	
	EES_ExchangeID		m_ExchangeID;					///< 102=中金所   103=上期所    104=大商所    105=郑商所
	double				m_Price;						///< 价格
	int					m_Volume;						///< 数量
	unsigned char		m_IsBid;						///< 1: 买方行情，0:卖方行情
};

struct EES_TradeSvrInfo
{
	char            m_remoteTradeIp[16];  /// 服务器交易IP
	unsigned short  m_remoteTradeTCPPort; /// 服务器交易TCP端口
	unsigned short  m_remoteTradeUDPPort; /// 服务器交易UDP端口

	char            m_remoteQueryIp[16];  /// 服务器查询IP
	unsigned short  m_remoteQueryTCPPort; /// 服务器查询TCP端口

	char            m_LocalTradeIp[16];   /// 本地交易IP
	unsigned short  m_LocalTradeUDPPort;  /// 本地交易UDP端口

	EES_TradeSvrInfo()
	{
		memset((void*)this, 0, sizeof(EES_TradeSvrInfo));
	}
};

#pragma pack(pop)

// 以下为 EES_OrderRejectField::m_ReasonCode的取值说明，委托被拒绝是用于说明拒绝原因
//	0 - 22号为语法检查错误
//	
//	0	向交易所发送报单时出现错误。
//	1	强平原因非法，目前只支持“0-非强平”
//	2	交易所代码非法
//	3	不使用
//	4	TIF不在合法值范围，目前只支持：EES_OrderTif_IOC(0) 和 EES_OrderTif_Day(99998)
//	5	不使用
//	6	委托价格必须>0
//	7	不使用
//	8	不使用
//	9	交易品种不合法，目前只支持“3-期货”
//	10	委托数量不合法，必须>0
//	11	不使用
//	12	买卖方向不合法，目前只支持1-6
//	13	不使用
//	14	对没有权限的账户进行操作
//	15	委托编号重复
//	16	不存在的账户
//	17	不合法的合约代码
//	18	委托总数超限，目前系统容量是每日最多850万个委托
//	19	保留，不用关注
//	20	资金账号未正确配置交易所编码
//	21	m_MinQty的值超过了m_Qty
//	22	当所有交易所连接都处于断开状态时，拒绝报单
//  23	当前账户没有期权交易权限
//	24	登录用户与连接session不符
	
// 50- 116由风控拒绝造成，
// 
//	50	订单手数限制
//	51	订单占经纪商保证金额限制
//	52	订单报价增额超限：与盘口价相比
//	53	订单报价增额超限: 与最近成交价相比
//	54	订单报价百分比超限:与盘口价相比
//	55	订单报价百分比超限:与最近成交价相比
//	56	订单报价增额超限：与昨结算价相比
//	57	订单报价百分比超限：与昨结算价相比
//	58	限价委托订单手数限制
//	59	市价委托订单手数限制
//	60	累计下订单发出次数限制
//	61	累计下订单发出手数限制
//	62	累计下订单发出金额限制
//	63	在指定时间1内收到订单次数上限
//	64	在指定时间2内收到订单次数上限
//	65	禁止交易
//	66	累计开仓订单发出次数限制
//	67	累计平仓订单发出次数限制
//	68	风控校验不通过次数限制
//	69	客户权益核查
//	70	总挂单金额校验
//	71	最大撤单次数限制
//	72	某合约最大撤单发出次数限制
//	73	在指定时间1内撤单次数上限
//	74	在指定时间2内撤单次数上限
//	75	大额订单撤单次数限制
//	76	累计持仓手数限制
//	77	累计持仓占用保证金额总和限制
//	78	累计成交手数限制
//	79	成交金额总和限制
//	80	下订单被市场拒绝次数的限制
//	81	下单被柜台系统拒绝次数限制
//	82	撤单被市场拒绝次数限制
//	83	在指定时间1内下订单被市场拒绝次数上限
//	84	在指定时间2内下订单被市场拒绝次数上限
//	85	在指定时间1内撤单被市场拒绝次数上限
//	86	在指定时间2内撤单被市场拒绝次数上限
//	87	净盈亏限制
//	88	浮动盈亏限制
//	89	总盈亏限制
//	90	持多仓手数限制
//	91	持空仓手数限制
//	92	持多仓占用保证金额限制
//	93	持空仓占用保证金额限制
//	94	某合约持多仓手数限制
//	95	某合约持空仓手数限制
//	96	某合约持多仓占用保证金额限制
//	97	某合约持空仓占用保证金额限制
//	98	某合约持仓总手数限制
//	99	某合约持仓占保证金额总额限制
//	100	某合约净收益限制
//	101	某合约浮动盈亏限制
//	102	某合约总收益限制
//	103	累计开仓成交手数限制
//	104	累计开仓成交金额总和限制
//	105	累计开多仓成交手数限制
//	106	累计开空仓成交手数限制
//	107	累计开多仓成交金额总和限制
//	108	累计开空仓成交金额总和限制
//	109	经纪商风险度限制
//	110	交易所风险度限制
//	111	在指定时间1内下单被柜台系统拒绝次数上限
//	112	在指定时间2内下单被柜台系统拒绝次数上限
//	113	不使用
//	114	可用资金不足
//	115	可平仓位不足
//	116	委托价格超过涨跌停范围



//	撤销拒绝原因对照表，对于交易所的撤单拒绝，不同的交易所给出的ReasonCode有各自的含义，需要独立处理
//  以下下整理的是目前已知的错误码
//  REM直接拒绝的错误码： m_ExchangeID = 0
//	5			-  找不到报单
//	33			-  报单已撤单或已成交
//	3			-  报单尚未被市场接受
//	129			-  外部报单不能从本系统撤单
//	513			-  无可用于撤单的席位
//	257			-  客户号或者登录号错误
//	17			-  客户号或者登录号错误
//	1025		-  席位流控
//	2049		-  席位已断开
//	4097		-  内部错误1
//	8193		-  内部错误2
//	16385		-  内部错误3
//	32769		-  内部错误4
//	65537		-  内部错误5
//	131073		-  内部错误6
//	262145		-  撤单触发了流控

//上期所已知的错误码： m_ExchangeID = 103
//	26 - 相关合约非交易时间
//	28 - 报单已经全部成交
//	29 - 报单已经撤销
//	49 - 价格超出涨停板

//大商所已知的错误码： m_ExchangeID = 104
//	40039 - 已经撤单
//	40040 - 完全成交,不能撤单!
//	40054 - 不能撤销其它用户的订单·
//	72003 - 无此撤销定单
#endif
