/*! 
* \file  EESQuoteDefine.h
* \brief EES行情类型定义头文件
*  
* 此文件描述了使用EES行情的类型的定义
*  
* \author  SHENGLI
* \version 1.0
* \date    2014-04-18
*/  

#pragma  once 

/// \brief EES行情服务器EQS标识长度
#define EES_EQS_ID_LEN         15

/// \brief EES行情服务器EQS登录名长度
#define EES_EQS_USER_ID_LEN    15

/// \brief EES行情服务器EQS登录密码长度
#define EES_EQS_PASSWORD_LEN   32

/// \brief EES行情类型
enum EesEqsIntrumentType
{
  EQS_INVALID_TYPE = '0', ///< 无效类型
  EQS_SH_STOCK,           ///< 上海股票
  EQS_SZ_STOCK,           ///< 深圳股票
  EQS_STOCK_OPTION,       ///< 股票期权
  EQS_FUTURE_OPTION,      ///< 期货期权
  EQS_INDEX_OPTION,       ///< 股指期权
  EQS_FUTURE,             ///< 期货
  EQS_SGE ,				  ///< 黄金
};

/// \brief 日志级别
enum EesEqsLogLevel
{
  QUOTE_LOG_LV_DEBUG = 0,  ///< 调试信息
  QUOTE_LOG_LV_INFO  = 1,  ///< 消处信息
  QUOTE_LOG_LV_WARN  = 2,  ///< 警告
  QUOTE_LOG_LV_ERROR = 3,  ///< 错误
  QUOTE_LOG_LV_FATAL = 4,  ///< 致命错误
  QUOTE_LOG_LV_USER  = 5,  ///< 用于给用户显示的消息
  QUOTE_LOG_LV_END         ///< 结尾标志
};

#pragma  pack(push, 1)

/////////////////////////////////////////////////////////////////////////
///EESQuoteDateType是一个日期类型
/////////////////////////////////////////////////////////////////////////
typedef char EESQuoteDateType[9];
/////////////////////////////////////////////////////////////////////////
///EESQuoteInstrumentIDType是一个合约代码类型
/////////////////////////////////////////////////////////////////////////
typedef char EESQuoteInstrumentIDType[31];
/////////////////////////////////////////////////////////////////////////
///EESQuoteExchangeIDType是一个交易所代码类型
/// 取值范围
/// 上交所 SHH
/// 深交所 SHZ
/// 上期所 SHFE
/// 中金所 CFFEX
/// 大商所 DCE
/// 郑商所 CZCE
/// 黄金  SGE
/////////////////////////////////////////////////////////////////////////
typedef char EESQuoteExchangeIDType[9];
/////////////////////////////////////////////////////////////////////////
///EESQuotePriceType是一个价格类型
/////////////////////////////////////////////////////////////////////////
typedef double EESQuotePriceType;
/////////////////////////////////////////////////////////////////////////
///EESQuoteLargeVolumeType是一个大额数量类型
/////////////////////////////////////////////////////////////////////////
typedef double EESQuoteLargeVolumeType;
/////////////////////////////////////////////////////////////////////////
///EESQuoteVolumeType是一个数量类型
/////////////////////////////////////////////////////////////////////////
typedef int EESQuoteVolumeType;
/////////////////////////////////////////////////////////////////////////
///EESQuoteMoneyType是一个资金类型
/////////////////////////////////////////////////////////////////////////
typedef double EESQuoteMoneyType;
/////////////////////////////////////////////////////////////////////////
///EESQuoteRatioType是一个比率类型
/////////////////////////////////////////////////////////////////////////
typedef double EESQuoteRatioType;
/////////////////////////////////////////////////////////////////////////
///EESQuoteExchangeInstIDType是一个合约在交易所的代码类型
/////////////////////////////////////////////////////////////////////////
typedef char EESQuoteExchangeInstIDType[31];
/////////////////////////////////////////////////////////////////////////
///EESQuoteMillisecType是一个时间（毫秒）类型
/////////////////////////////////////////////////////////////////////////
typedef int EESQuoteMillisecType;
/////////////////////////////////////////////////////////////////////////
///TFtdcTimeType是一个时间类型
/////////////////////////////////////////////////////////////////////////
typedef char EESQuoteTimeType[9];

/// \brief EES行情服务组播配置信息，每个交易所的行情映射一套组播地址
struct EqsMulticastInfo
{
	char m_mcIp[EES_EQS_ID_LEN + 1];			///< 组播地址
	unsigned short m_mcPort;					///< 组播端口1-65535
	char m_mcLoacalIp[EES_EQS_ID_LEN + 1];		///< 本机地址
	unsigned short m_mcLocalPort;				///< 本机端口1-65535, 本机未被使用的端口号
	EESQuoteExchangeIDType	m_exchangeId;		///< 交易所代码，参考EESQuoteExchangeIDType取值范围
	EqsMulticastInfo()
	{
		memset(this, 0, sizeof(*this));
	}
};


/// \brief EES行情服务器配置信息
struct EqsTcpInfo
{
  EqsTcpInfo()
  {
      m_eqsId[0] = 0x00;
      m_eqsIp[0] = 0x00;
      m_eqsPort  = 0; 
  }

  char            m_eqsId[EES_EQS_ID_LEN + 1]; ///< 填空字符串即可
  char            m_eqsIp[EES_EQS_ID_LEN + 1]; ///< TCP服务器IP地址
  unsigned short  m_eqsPort;                   ///< TCP服务器端口号 
};

/// \brief EES行情服务器登陆信息
struct EqsLoginParam
{
  EqsLoginParam()
  {
      m_loginId[0]  = 0x00;
      m_password[0] = 0x00;
  }
  char  m_loginId[EES_EQS_USER_ID_LEN + 1];   ///< 登录名
  char  m_password[EES_EQS_PASSWORD_LEN + 1]; ///< 登录密码
};

/// \brief EES行情结构
struct EESMarketDepthQuoteData
{
  EESQuoteDateType            TradingDay;     ///<交易日
  EESQuoteInstrumentIDType    InstrumentID;   ///<合约代码
  EESQuoteExchangeIDType      ExchangeID;     ///<交易所代码
  EESQuoteExchangeInstIDType  ExchangeInstID; ///<合约在交易所的代码
  EESQuotePriceType           LastPrice;      ///<最新价
  EESQuotePriceType           PreSettlementPrice; ///<上次结算价
  EESQuotePriceType           PreClosePrice;    ///<昨收盘
  EESQuoteLargeVolumeType     PreOpenInterest; ///<昨持仓量
  EESQuotePriceType           OpenPrice;       ///<今开盘
  EESQuotePriceType           HighestPrice;    ///<最高价
  EESQuotePriceType           LowestPrice;     ///<最低价
  EESQuoteVolumeType          Volume;          ///<数量
  EESQuoteMoneyType           Turnover;        ///<成交金额
  EESQuoteLargeVolumeType     OpenInterest;    ///<持仓量
  EESQuotePriceType           ClosePrice;      ///<今收盘
  EESQuotePriceType           SettlementPrice; ///<本次结算价
  EESQuotePriceType           UpperLimitPrice; ///<涨停板价
  EESQuotePriceType           LowerLimitPrice; ///<跌停板价
  EESQuoteRatioType           PreDelta;        ///<昨虚实度
  EESQuoteRatioType           CurrDelta;       ///<今虚实度
  EESQuoteTimeType            UpdateTime;      ///<最后修改时间
  EESQuoteMillisecType        UpdateMillisec;  ///<最后修改毫秒
  EESQuotePriceType           BidPrice1;       ///<申买价一
  EESQuoteVolumeType          BidVolume1;      ///<申买量一
  EESQuotePriceType           AskPrice1;       ///<申卖价一
  EESQuoteVolumeType          AskVolume1;      ///<申卖量一
  EESQuotePriceType           BidPrice2;       ///<申买价二
  EESQuoteVolumeType          BidVolume2;      ///<申买量二
  EESQuotePriceType           AskPrice2;       ///<申卖价二
  EESQuoteVolumeType          AskVolume2;      ///<申卖量二
  EESQuotePriceType           BidPrice3;       ///<申买价三
  EESQuoteVolumeType          BidVolume3;      ///<申买量三
  EESQuotePriceType           AskPrice3;       ///<申卖价三
  EESQuoteVolumeType          AskVolume3;      ///<申卖量三
  EESQuotePriceType           BidPrice4;       ///<申买价四
  EESQuoteVolumeType          BidVolume4;      ///<申买量四
  EESQuotePriceType           AskPrice4;       ///<申卖价四
  EESQuoteVolumeType          AskVolume4;      ///<申卖量四
  EESQuotePriceType           BidPrice5;       ///<申买价五
  EESQuoteVolumeType          BidVolume5;      ///<申买量五
  EESQuotePriceType           AskPrice5;       ///<申卖价五
  EESQuoteVolumeType          AskVolume5;      ///<申卖量五
  EESQuotePriceType           AveragePrice;    ///<当日均价
};

#pragma  pack(pop)