# Model documentation
* http://nasdaqtrader.com/content/technicalsupport/specifications/TradingProducts/Ouch5.0.pdf

# Json representation of Ouch 5.0
## Clt Messages
```json
{"UPayload":{"EnterOrder":{"user_ref_number":1,"side":"BUY","quantity":100,"symbol":"DUMMY","price":1.2345,"time_in_force":"MARKET_HOURS","display":"VISIBLE","capacity":"AGENCY","int_mkt_sweep_eligibility":"ELIGIBLE","cross_type":"CONTINUOUS_MARKET","clt_order_id":"1","appendages":{"firm":"????","min_qty":0,"customer_type":"PORT_DEFAULT","max_floor":0,"price_type":"LIMIT","peg_offset":-1.1234,"discretion_price":0.0,"discretion_price_type":"LIMIT","discretion_peg_offset":-1.1234,"post_only":"NO","random_reserves":0,"route":"????","expire_time":0,"trade_now":"PORT_DEFAULT","handle_inst":"NO_INSTRUCTIONS","group_id":0,"shares_located":"NO"}}}}

{"UPayload":{"ReplaceOrder":{"orig_user_ref_number":1,"user_ref_number":0,"quantity":100,"price":1.2345,"time_in_force":"MARKET_HOURS","display":"VISIBLE","int_mkt_sweep_eligibility":"ELIGIBLE","clt_order_id":"REPLACE_ME____","appendages":{"min_qty":0,"customer_type":"PORT_DEFAULT","max_floor":0,"price_type":"LIMIT","peg_offset":-1.1234,"discretion_price":0.0,"discretion_price_type":"LIMIT","discretion_peg_offset":-1.1234,"post_only":"NO","random_reserves":0,"expire_time":0,"trade_now":"PORT_DEFAULT","handle_inst":"NO_INSTRUCTIONS","group_id":0,"shares_located":"NO"}}}}

{"UPayload":{"CancelOrder":{"user_ref_number":1,"quantity":10}}}

{"UPayload":{"ModifyOrder":{"user_ref_number":1,"side":"BUY","quantity":10}}}

{"UPayload":{"AccountQueryRequest":{}}}

{"HBeat":{}}

{"Dbg":{"text":"This is a default debug message text"}}

{"Login":{"username":"dummy","password":"dummy","session_id":"session #1","sequence_number":"1","hbeat_timeout_ms":"1000"}}

{"Logout":{}}
```

## Svc Messages
```json
{"SPayload":{"SystemEvent":{"timestamp":53094058193000,"event_code":"START_OF_DAY"}}}

{"SPayload":{"OrderAccepted":{"timestamp":53094058375000,"user_ref_number":1,"side":"BUY","quantity":100,"symbol":"DUMMY","price":1.2345,"time_in_force":"MARKET_HOURS","display":"VISIBLE","order_reference_number":1,"capacity":"AGENCY","int_mkt_sweep_eligibility":"ELIGIBLE","cross_type":"CONTINUOUS_MARKET","order_state":"LIVE","clt_order_id":"1","appendages":{"firm":"????","min_qty":0,"customer_type":"PORT_DEFAULT","max_floor":0,"price_type":"LIMIT","peg_offset":-1.1234,"discretion_price":0.0,"discretion_price_type":"LIMIT","discretion_peg_offset":-1.1234,"post_only":"NO","random_reserves":0,"route":"????","expire_time":0,"trade_now":"PORT_DEFAULT","handle_inst":"NO_INSTRUCTIONS","group_id":0,"shares_located":"NO"}}}}

{"SPayload":{"OrderReplaced":{"timestamp":53094058387000,"orig_user_ref_number":1,"user_ref_number":1,"side":"BUY","quantity":100,"symbol":"DUMMY","price":1.2345,"time_in_force":"MARKET_HOURS","display":"VISIBLE","order_reference_number":0,"capacity":"AGENCY","int_mkt_sweep_eligibility":"ELIGIBLE","cross_type":"CONTINUOUS_MARKET","order_state":"LIVE","clt_order_id":"REPLACE_ME____","appendages":{"firm":"????","min_qty":0,"max_floor":0,"price_type":"LIMIT","post_only":"NO","expire_time":0,"trade_now":"PORT_DEFAULT","handle_inst":"NO_INSTRUCTIONS"}}}}

{"SPayload":{"OrderCanceled":{"timestamp":53094058416000,"orig_user_ref_number":1,"user_ref_number":1,"quantity":10,"cancel_reason":"USER_REQUESTED"}}}

{"SPayload":{"OrderAiqCanceled":{"timestamp":53094058427000,"user_ref_number":1,"decrement_shares":0,"prevented_from_trading":0,"execution_price":0.0,"liquidity_flag":"ADDED","aiq_strategy":"?"}}}

{"SPayload":{"OrderExecuted":{"timestamp":53094058429000,"user_ref_number":1,"quantity":100,"price":1.2345,"liquidity_flag":"ADDED","match_number":0,"appendages":{"firm":"????","min_qty":0,"customer_type":"PORT_DEFAULT","max_floor":0,"price_type":"LIMIT","peg_offset":-1.1234,"discretion_price":0.0,"discretion_price_type":"LIMIT","discretion_peg_offset":-1.1234,"post_only":"NO","random_reserves":0,"route":"????","expire_time":0,"trade_now":"PORT_DEFAULT","handle_inst":"NO_INSTRUCTIONS","group_id":0,"shares_located":"NO"}}}}

{"SPayload":{"BrokenTrade":{"timestamp":53094058439000,"user_ref_number":1,"match_number":0,"broken_trade_reason":"ERRONEOUS","clt_order_id":"1"}}}

{"SPayload":{"OrderRejected":{"timestamp":53094058441000,"user_ref_number":1,"reject_reason":1,"clt_order_id":"1"}}}

{"SPayload":{"CancelPending":{"timestamp":53094058442000,"user_ref_number":1}}}

{"SPayload":{"CancelReject":{"timestamp":53094058444000,"user_ref_number":1}}}

{"SPayload":{"PriorityUpdate":{"timestamp":53094058445000,"user_ref_number":1,"price":1.2345,"display":"VISIBLE","order_reference_number":0}}}

{"SPayload":{"OrderModified":{"timestamp":53094058447000,"user_ref_number":1,"side":"BUY","quantity":10}}}

{"SPayload":{"OrderRestated":{"timestamp":53094058448000,"user_ref_number":1,"restate_reason":"REFRESH_OF_DISPLAY","appendages":{"display_qty":1,"display_price":0.0,"secondary_order_ref_num":1}}}}

{"SPayload":{"AccountQueryResponse":{"timestamp":53094058449000,"next_user_ref_number":1}}}

{"HBeat":{}}

{"Dbg":{"text":"This is a default debug message text"}}

{"LoginAccepted":{"session_id":"session #1","sequence_number":"1"}}

{"LoginRejected":{"reason":"NOT_AUTHORIZED"}}

{"EndOfSession":{}}
```