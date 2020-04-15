use num_enum::TryFromPrimitive;

// https://discordapp.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-close-event-codes
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u16)]
pub enum GatewayCloseCode {
    TimeOut,
    UnknownCloseCode,
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

impl GatewayCloseCode {
    pub fn resolve(&self) -> GatewayErrorResolve {
        match self {
            GatewayCloseCode::TimeOut => GatewayErrorResolve::Reconnect,
            GatewayCloseCode::UnknownCloseCode => GatewayErrorResolve::Reconnect,
            GatewayCloseCode::UnknownError => GatewayErrorResolve::Reconnect,
            GatewayCloseCode::RateLimit => GatewayErrorResolve::Reconnect,
            GatewayCloseCode::InvalidSeq => GatewayErrorResolve::NewSession,
            GatewayCloseCode::SessionTimeOut => GatewayErrorResolve::NewSession,
            _ => GatewayErrorResolve::Panic
        }
    }
}