## Java SDK Reference

### transactions

- Withdraw

```json
{
  "nonce": "1",
  "public_key": "0x42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9",
  "expiration_timestamp": "1684832800",
  "position_id": "2",
  "amount": "3",
  "eth_address": "0x42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9",
  "asset_id": "0x1a"
}
```

- Transfer

```json
{
  "nonce": "1",
  "public_key": "0x42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9",
  "expiration_timestamp": "11111111",
  "sender_position_id": "1",
  "receiver_public_key": "0x42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9",
  "receiver_position_id": "1",
  "amount": "1",
  "asset_id": "0xa8"
}
```

- limitOrder

```json
{
  "nonce": "1",
  "public_key": "42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9",
  "expiration_timestamp": "2",
  "amount_synthetic": "3",
  "amount_collateral": "4",
  "amount_fee": "5",
  "asset_id_synthetic": "6",
  "asset_id_collateral": "0xa",
  "position_id": "8",
  "is_buying_synthetic": false
}
```

- Liquidate

```json
{
  "liquidator_order": {
    "nonce": "0",
    "public_key": "42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9",
    "expiration_timestamp": "0",
    "amount_synthetic": "1",
    "amount_collateral": "2",
    "amount_fee": "3",
    "asset_id_synthetic": "4",
    "asset_id_collateral": "0x5",
    "position_id": "6",
    "is_buying_synthetic": false
  },
  "liquidated_position_id": "7",
  "actual_collateral": "8",
  "actual_synthetic": "9",
  "actual_liquidator_fee": "10"
}
```

- SignedOraclePrice

```json
{
  "signer_key": "0x42cbd3cbd97f9ac9c5c4b15f0b5ca78d57ff1e5948008799b9c0d330b1e217a9",
  "external_price": "0xa",
  "timestamp": "2",
  "signed_asset_id": "0xa"
}
```

### api

`ZKDEX.verifySignature` verify a signature.

| params | type   | remark              |
|--------|--------|---------------------|
| sig_r  | String | signature' r        |
| sig_s  | String | signature' s        |
| pubKey | String | public key          |
| msg    | String | hash of origin data |

`ZKDEX.signWithdraw` sign a withdraw transcation

| params  | type   | remark           |
|---------|--------|------------------|
| json    | String | json of withdraw |
| assetId | String | assetId          |
| priKey  | String | private key      |

`ZKDEX.signTransfer` sign a transfer transaction

| params | type   | remark                       |
|--------|--------|------------------------------|
| json   | String | json of transfer transaction |
| priKey | String | private key                  |

`ZKDEX.signLimitOrder` sign a LimitOrder transaction

| params | type   | remark              |
|--------|--------|---------------------|
| json   | String | json of limit order |
| priKey | String | private key         |

`ZKDEX.signLiquidate` sign a Liquidate transaction

| params | type   | remark            |
|--------|--------|-------------------|
| json   | String | json of liquidate |
| priKey | String | private key       |

`ZKDEX.signSignedOraclePrice` sign a SignedOraclePrice transaction

| params | type   | remark                    |
|--------|--------|---------------------------|
| json   | String | json of SignedOraclePrice |
| priKey | String | private key               |

`ZKDEX.hashWithdraw` hash a withdraw transcation

| params  | type   | remark           |
|---------|--------|------------------|
| json    | String | json of withdraw |
| assetId | String | assetId          |

`ZKDEX.hashTransfer` hash a transfer transaction

| params | type   | remark                       |
|--------|--------|------------------------------|
| json   | String | json of transfer transaction |

`ZKDEX.hashLimitOrder` hash a LimitOrder transaction

| params | type   | remark              |
|--------|--------|---------------------|
| json   | String | json of limit order |

`ZKDEX.hashLiquidate` hash a Liquidate transaction

| params | type   | remark            |
|--------|--------|-------------------|
| json   | String | json of liquidate |

`ZKDEX.hashSignedOraclePrice` hash a SignedOraclePrice transaction

| params | type   | remark                    |
|--------|--------|---------------------------|
| json   | String | json of SignedOraclePrice |

`ZKDEX.sign` sign a msg

| params     | type   | remark      |
|------------|--------|-------------|
| privateKey | String | private key |
| msg        | String | hash of msg |

`ZKDEX.privateKeyFromSeed` generate a private key from a seed

| params | type   | remark    |
|--------|--------|-----------|
| seed   | String | rand seed |

`ZKDEX.isOnCurve` check if publick key xy is on the curve

| params | type   | remark       |
|--------|--------|--------------|
| x      | String | public key x |
| y      | String | public key y |

`ZKDEX.privateKeyToPublicKeyXY` derive public key xy from private key

| params | type   | remark    |
|--------|--------|-----------|
| seed   | String | rand seed |

