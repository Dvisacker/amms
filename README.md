## amms

A fork of amms-rs using alloy and with added support for curve, aerodrome, camelot, etc.

Currently work in progress especially with CL (Uniswap V3, Aerodrome Slipstream etc.) pools

TODO: 
- Populate CL pool tick data with batched requests (populating via logs is too slow)
- Replace dynamic decoding with static decoding for batch requests
- Verify simulated swaps work with method above
- Balancer support
- Remove bindings from git
- Aerodrome Slipstream