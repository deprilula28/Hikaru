use num_enum::TryFromPrimitive;

// https://discordapp.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-close-event-codes
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u16)]
pub enum close_code {
    TimeOut,
    UnknownCloseCode,
    Reconnect,
    UnknownError = 4000,
    UnknownOpcode = 4001,
    DecodeError = 4002,
    Unauthorized = 4003,
    AuthenticationError = 4004,
    RepeatedAuth = 4005,
    InvalidSeq = 4007,
    RateLimit = 4008,
    SessionTimeOut = 4009,
    InvalidShard = 4010,
    ShardRequired = 4011,
    DeprecatedAPI = 4012,
    InvalidIntent = 4013,
    DisallowedIntent = 4014
}

#[derive(Debug)]
pub enum GatewayErrorResolve {
    Reconnect,
    NewSession,
    Panic
}

impl close_code {
    pub fn resolve(&self) -> GatewayErrorResolve {
        match self {
            close_code::TimeOut => GatewayErrorResolve::Reconnect,
            close_code::UnknownCloseCode => GatewayErrorResolve::Reconnect,
            close_code::Reconnect => GatewayErrorResolve::Reconnect,
            close_code::UnknownError => GatewayErrorResolve::Reconnect,
            close_code::RateLimit => GatewayErrorResolve::Reconnect,
            close_code::InvalidSeq => GatewayErrorResolve::NewSession,
            close_code::SessionTimeOut => GatewayErrorResolve::NewSession,
            _ => GatewayErrorResolve::Panic
        }
    }
}