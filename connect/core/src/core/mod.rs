use ouch_model::prelude::{CltOuchPayload, SvcOuchPayload};
use soupbintcp_connect_nonblocking::prelude::*;

pub type OuchFramer = SoupBinTcpFramer;
pub type CltOuchMessenger = CltSoupBinTcpMessenger<SvcOuchPayload, CltOuchPayload>;
pub type SvcOuchMessenger = SvcSoupBinTcpMessenger<CltOuchPayload, SvcOuchPayload>;

pub type CltOuchDevNullCallback = DevNullCallback<CltOuchMessenger>;
pub type CltOuchLoggerCallback = LoggerCallback<CltOuchMessenger>;

pub type SvcOuchDevNullCallback = DevNullCallback<SvcOuchMessenger>;
pub type SvcOuchLoggerCallback = LoggerCallback<SvcOuchMessenger>;