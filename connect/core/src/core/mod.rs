use ouch_model::prelude::{CltOuchPayload, SvcOuchPayload};
use soupbintcp_connect_nonblocking::prelude::*;

// Framer
pub type OuchFramer = SoupBinTcpFramer;

// Messenger
pub type CltOuchMessenger = CltSoupBinTcpMessenger<SvcOuchPayload, CltOuchPayload>;
pub type SvcOuchMessenger = SvcSoupBinTcpMessenger<CltOuchPayload, SvcOuchPayload>;

// Protocol
pub type CltOuchProtocolManual = CltSoupBinTcpProtocolManual<SvcOuchPayload, CltOuchPayload>;
pub type CltOuchProtocolAuto = CltSoupBinTcpProtocolAuto<SvcOuchPayload, CltOuchPayload>;

pub type SvcOuchProtocolManual = SvcSoupBinTcpProtocolManual<CltOuchPayload, SvcOuchPayload>;
pub type SvcOuchProtocolAuto = SvcSoupBinTcpProtocolAuto<CltOuchPayload, SvcOuchPayload>;

// Callback
pub type CltOuchDevNullCallback = DevNullCallback<CltOuchMessenger>;
pub type CltOuchLoggerCallback = LoggerCallback<CltOuchMessenger>;

pub type SvcOuchDevNullCallback = DevNullCallback<SvcOuchMessenger>;
pub type SvcOuchLoggerCallback = LoggerCallback<SvcOuchMessenger>;
