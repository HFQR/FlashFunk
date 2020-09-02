/*! 
* \file  EESQuoteApi.h
* \brief EES行情客户端头文件
*  
* 此文件描述了使用EES行情客户端所需的类型和函数
*  
* \author  SHENGLI
* \version 1.0
* \date    2014-04-18
*/  

#pragma once
#include "EESQuoteDefine.h"
#include <vector>
using std::vector;

#ifdef WIN32
    #ifdef SL_EES_QUOTE_EXPORTS
        #define SL_EES_QUOTE_CLASS   __declspec(dllexport)
        #define SL_EES_QUOTE_FUN     extern "C" __declspec(dllexport)
    #else
        #define SL_EES_QUOTE_CLASS   __declspec(dllimport)
        #define SL_EES_QUOTE_FUN     extern "C" __declspec(dllimport)
    #endif
    
     /// \brief EES行情客户端动态库名
    #define EES_QUOTE_DLL_NAME    "EESQuoteApi.dll"
    /// \brief EES行情客户端静态库名
    #define EES_QUOTE_LIB_NAME    "EESQuoteApi.lib"
    
#else // SHENGLI_LINUX
    #define SL_EES_QUOTE_CLASS 
    #define SL_EES_QUOTE_FUN extern "C"
    
    /// \brief EES行情客户端动态库名
    #define EES_QUOTE_DLL_NAME    "libEESQuoteApi.so"
#endif

/// \brief EES Quote需要注册的回调类 
class EESQuoteEvent
{
public:
    virtual ~EESQuoteEvent() {}

	/// \brief 当服务器连接成功，登录前调用, 如果是组播模式不会发生, 只需判断InitMulticast返回值即可
    virtual void OnEqsConnected() {}

	/// \brief 当服务器曾经连接成功，被断开时调用，组播模式不会发生该事件
    virtual void OnEqsDisconnected() {}

	/// \brief 当登录成功或者失败时调用，组播模式不会发生
	/// \param bSuccess 登陆是否成功标志  
	/// \param pReason  登陆失败原因  
    virtual void OnLoginResponse(bool bSuccess, const char* pReason) {}

	/// \brief 收到行情时调用,具体格式根据instrument_type不同而不同
	/// \param chInstrumentType  EES行情类型
	/// \param pDepthQuoteData   EES统一行情指针  
    virtual void OnQuoteUpdated(EesEqsIntrumentType chInstrumentType, EESMarketDepthQuoteData* pDepthQuoteData) {}

	/// \brief 日志接口，让使用者帮助写日志。
	/// \param nlevel    日志级别
	/// \param pLogText  日志内容
	/// \param nLogLen   日志长度
    virtual void OnWriteTextLog(EesEqsLogLevel nlevel, const char* pLogText, int nLogLen) {}

	/// \brief 注册symbol响应消息来时调用，组播模式不支持行情注册
	/// \param chInstrumentType  EES行情类型
	/// \param pSymbol           合约名称
	/// \param bSuccess          注册是否成功标志
    virtual void OnSymbolRegisterResponse(EesEqsIntrumentType chInstrumentType, const char* pSymbol, bool bSuccess)  {}

	/// \brief  注销symbol响应消息来时调用，组播模式不支持行情注册
	/// \param chInstrumentType  EES行情类型
	/// \param pSymbol           合约名称
	/// \param bSuccess          注册是否成功标志
    virtual void OnSymbolUnregisterResponse(EesEqsIntrumentType chInstrumentType, const char* pSymbol, bool bSuccess)  {}
	
	/// \brief 查询symbol列表响应消息来时调用，组播模式不支持合约列表查询
	/// \param chInstrumentType  EES行情类型
	/// \param pSymbol           合约名称
	/// \param bLast             最后一条查询合约列表消息的标识
	/// \remark 查询合约列表响应, last = true时，本条数据是无效数据。
    virtual void OnSymbolListResponse(EesEqsIntrumentType chInstrumentType, const char* pSymbol, bool bLast)  {}

};

/// \brief EES Quote的纯虚基类
class SL_EES_QUOTE_CLASS EESQuoteApi
{
public:
    virtual ~EESQuoteApi() {}

	/// \brief  EES行情客户端连接行情服务器, 不可以和InitMulticast同时使用
	/// \param  vecEti             一组EES行情服务器的配置    
	/// \param  pQuoteEventHandler 相关事件通知的回调函数
	/// \return true: 连接EES行情服务器成功 false:连接EES行情服务器失败
    virtual bool ConnServer(vector<EqsTcpInfo>& vecEti, EESQuoteEvent* pQuoteEventHandler) = 0;

	/// \brief  EES行情客户端连接行情服务器, 不可以和InitMulticast及另一个ConnServer同时使用
	/// \param  svrInfo             单个EES行情服务器的配置    
	/// \param  pQuoteEventHandler 相关事件通知的回调函数
	/// \return true: 连接EES行情服务器成功 false:连接EES行情服务器失败
	virtual bool ConnServer(EqsTcpInfo& svrInfo, EESQuoteEvent* pQuoteEventHandler) = 0;

	/// \brief  EES行情客户端加入组播地址, 不可以和ConneServer同时使用
	/// \param  vecEti             一组EES行情服务器的配置    
	/// \param  pQuoteEventHandler 相关事件通知的回调函数
	/// \return true: 连接EES行情服务器成功 false:连接EES行情服务器失败
	virtual bool InitMulticast(vector<EqsMulticastInfo>& vecEmi, EESQuoteEvent* pQuoteEventHandler) = 0; 

	/// \brief  EES行情客户端登陆服务器请求, 使用组播模式不需要调用
	/// \param  loginParam   登陆时所用的用户和密码   
	/// \remark 此函数无返回码, 登陆成功与否会在登陆响应的回调中通知
    virtual void LoginToEqs(EqsLoginParam& loginParam) = 0;

	/// \brief  向EES行情服务器请求合约列表, 组播模式不支持
    virtual void QuerySymbolList() = 0;

	/// \brief  EES行情客户端注册合约, 组播模式不支持
	/// \param  chInstrumentType  EES行情类型  
	/// \param  pSymbol           合约名称
	/// \remark 此函数无返回码, 注册成功与否会在注册响应的回调中通知
    virtual void RegisterSymbol(EesEqsIntrumentType chInstrumentType, const char* pSymbol) = 0;

	/// \brief  EES行情客户端注册合约, 组播模式不支持
	/// \param  chInstrumentType  EES行情类型  
	/// \param  pSymbol           合约名称
	/// \return 此函数无返回码, 注销成功与否会在注销响应的回调中通知
    virtual void UnregisterSymbol(EesEqsIntrumentType chInstrumentType, const char* pSymbol) = 0;

	/// \brief 关闭EES行情客户端
	/// \remark 执行此函数后未实例内部信息销毁,如果需要继续使用,需要将实例销毁后,重新创建实例
    virtual void DisConnServer() = 0;
};


/// \brief 创建EES行情客户端实例的函数名
#define CREATE_EES_QUOTE_API_NAME   ("CreateEESQuoteApi")

/// \brief 销毁EES行情客户端实例的函数名
#define DESTROY_EES_QUOTE_API_NAME  ("DestroyEESQuoteApi")

/// \brief 创建EES行情客户端实例的函数声明
SL_EES_QUOTE_FUN EESQuoteApi* CreateEESQuoteApi(void);

/// \brief 销毁EES行情客户端实例的函数声明
SL_EES_QUOTE_FUN void DestroyEESQuoteApi(EESQuoteApi* pEESQuoteApi);

/// \brief 创建EES行情客户端实例的函数指针类型
typedef EESQuoteApi* (*funcCreateEESQuoteApi)(void);

/// \brief 销毁EES行情客户端实例的函数指针类型
typedef void (*funcDestroyEESQuoteApi)(EESQuoteApi* pEESQuoteApi);


