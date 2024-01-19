pub mod protocol;

use links_nonblocking::prelude::*;
use ouch_model::prelude::{CltOuchPayload, SvcOuchPayload};
use soupbintcp_connect_nonblocking::prelude::*;

// Framer
pub type OuchFramer = SoupBinTcpFramer;

// Messenger
pub type CltOuchMessenger = CltSoupBinTcpMessenger<SvcOuchPayload, CltOuchPayload>;
pub type SvcOuchMessenger = SvcSoupBinTcpMessenger<CltOuchPayload, SvcOuchPayload>;

// Callback
pub type CltOuchDevNullCallback = DevNullCallback<CltOuchMessenger>;
pub type CltOuchLoggerCallback = LoggerCallback<CltOuchMessenger>;

pub type SvcOuchDevNullCallback = DevNullCallback<SvcOuchMessenger>;
pub type SvcOuchLoggerCallback = LoggerCallback<SvcOuchMessenger>;
