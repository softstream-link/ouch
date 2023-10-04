use ouch_model::prelude::{CltOuchPayload, SvcOuchPayload};
use soupbintcp_connect_nonblocking::prelude::*;

pub type OuchFramer = SoupBinTcpFramer;
pub type CltOuchMessenger = CltSoupBinTcpMessenger<SvcOuchPayload, CltOuchPayload>;
pub type SvcOuchMessenger = SvcSoupBinTcpMessenger<CltOuchPayload, SvcOuchPayload>;
