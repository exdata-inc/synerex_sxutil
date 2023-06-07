# synerex_sxutil

## Rust Ver. Breaking Change:

For memory safety, there are some breaking change compared to Go version.

- After `register_node` call, you must call `tokio::spawn(sxutil::start_keep_alive_with_cmd(cmd_func: Option<fn(nodeapi::KeepAliveCommand, String)>));` to start keep-alive.



sxutil is a Utility Library to support Synerex Server/Client development.


- From Ver 0.5.0 we changed slightly.

The follwing is very important type for Synerex to support re-connection.

'''
// SXSynerexClient is for each server from v0.5.0
type SXSynerexClient struct {
	ServerAddress string
	Client        api.SynerexClient
}

// SXServiceClient Wrappter Structure for synerex client
type SXServiceClient struct {
	ClientID    IDType
	ChannelType uint32
	SXClient    *SXSynerexClient
	ArgJson     string
	MbusIDs     []IDType
	mbusMutex   sync.RWMutex
	NI          *NodeServInfo
}
'''
