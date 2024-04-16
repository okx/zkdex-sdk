## Functions

<dl>
<dt><a href="#zkdex_init">zkdex_init()</a></dt>
<dd><p>This method initializes params for current thread, otherwise they will be initialized when signing
first message.</p>
</dd>
<dt><a href="#sign_transfer">sign_transfer(json, private_key)</a> ⇒ <code>string</code></dt>
<dd><p>sign_transfer, sign a transfer transaction.</p>
</dd>
<dt><a href="#hash_transfer">hash_transfer(json)</a> ⇒ <code>string</code></dt>
<dd><p>hash_transfer, hash a transfer transaction.</p>
</dd>
<dt><a href="#sign_withdraw">sign_withdraw(json, private_key)</a> ⇒ <code>string</code></dt>
<dd><p>sign_withdraw, sign a withdraw transaction.</p>
</dd>
<dt><a href="#hash_withdraw">hash_withdraw(json)</a> ⇒ <code>string</code></dt>
<dd><p>hash_withdraw, hash a withdraw transaction.</p>
</dd>
<dt><a href="#sign_limit_order">sign_limit_order(json, private_key)</a> ⇒ <code>string</code></dt>
<dd><p>sign_limit_order, sign a limit order transaction.</p>
</dd>
<dt><a href="#hash_limit_order">hash_limit_order(json)</a> ⇒ <code>string</code></dt>
<dd><p>hash_limit_order, sign a limit order transaction.</p>
</dd>
<dt><a href="#sign_liquidate">sign_liquidate(json, private_key)</a> ⇒ <code>string</code></dt>
<dd><p>sign_liquidate, sign a liquidate transaction.</p>
</dd>
<dt><a href="#hash_liquidate">hash_liquidate(json)</a> ⇒ <code>string</code></dt>
<dd><p>hash_liquidate, hash a liquidate transaction.</p>
</dd>
<dt><a href="#sign_signed_oracle_price">sign_signed_oracle_price(json, private_key)</a> ⇒ <code>string</code></dt>
<dd><p>sign_signed_oracle_price, sign a signed oracle price transaction.</p>
</dd>
<dt><a href="#hash_signed_oracle_price">hash_signed_oracle_price(json)</a> ⇒ <code>string</code></dt>
<dd><p>hash_signed_oracle_price, hash a signed oracle price transaction.</p>
</dd>
<dt><a href="#verify_signature">verify_signature(sig_r, sig_s, pub_key_x, pub_key_y, msg)</a> ⇒ <code>bool</code></dt>
<dd><p>verify_signature, verify a signature.</p>
</dd>
<dt><a href="#l1_sign">l1_sign(msg, private_key, string)</a></dt>
<dd><p>l1 sign, sign a msg on l1 when signing a eth address.</p>
</dd>
<dt><a href="#sign_eth_address">sign_eth_address(address, pubkey, l2_private_key)</a></dt>
<dd><p>sign eth address</p>
</dd>
<dt><a href="#is_on_curve">is_on_curve(pub_key_x, pub_key_y)</a> ⇒ <code>bool</code></dt>
<dd><p>is_on_curve, check the (x,y) is on curve.</p>
</dd>
<dt><a href="#sign">sign(msg, private_key)</a> ⇒ <code>string</code></dt>
<dd><p>sign, sign a msg on l2, is a generic signature methods.</p>
</dd>
<dt><a href="#private_key_from_seed">private_key_from_seed(seed)</a> ⇒ <code>string</code></dt>
<dd><p>private_key_from_seed, derive a private key from a random seed, the seed could be anything.</p>
</dd>
<dt><a href="#private_key_to_pubkey_xy">private_key_to_pubkey_xy(private_key)</a> ⇒ <code>string</code></dt>
<dd><p>private_key_to_pubkey_xy, derive a public with xy from private key.</p>
</dd>
<dt><a href="#public_key_to_xy">public_key_to_xy(pub_key)</a> ⇒ <code>string</code></dt>
<dd><p>public_key_to_xy, convert public key to xy.</p>
</dd>
<dt><a href="#sign_spot_transfer">sign_spot_transfer(json, private_key)</a> ⇒ <code>string</code></dt>
<dd><p>sign_spot_transfer, sign a spot transfer transaction.</p>
</dd>
<dt><a href="#hash_spot_transfer">hash_spot_transfer(json)</a> ⇒ <code>string</code></dt>
<dd><p>hash_spot_transfer, hash a spot transfer transaction.</p>
</dd>
<dt><a href="#sign_spot_withdrawal">sign_spot_withdrawal(json, private_key)</a> ⇒ <code>string</code></dt>
<dd><p>sign_spot_withdrawal, sign a spot withdrawal transaction.</p>
</dd>
<dt><a href="#hash_spot_withdrawal">hash_spot_withdrawal(json)</a> ⇒ <code>string</code></dt>
<dd><p>hash_spot_withdrawal, hash a spot withdrawal transaction.</p>
</dd>
<dt><a href="#sign_spot_limit_order">sign_spot_limit_order(json, private_key)</a> ⇒ <code>string</code></dt>
<dd><p>sign_spot_limit_order, sign a spot LimitOrder transaction.</p>
</dd>
<dt><a href="#hash_spot_limit_order">hash_spot_limit_order(json)</a> ⇒ <code>string</code></dt>
<dd><p>hash_spot_limit_order, hash a spot LimitOrder transaction.</p>
</dd>
</dl>

<a name="zkdex_init"></a>

## zkdex\_init()
This method initializes params for current thread, otherwise they will be initialized when signing
first message.

**Kind**: global function  
<a name="sign_transfer"></a>

## sign\_transfer(json, private_key) ⇒ <code>string</code>
sign_transfer, sign a transfer transaction.

**Kind**: global function  
**Returns**: <code>string</code> - json signature of transfer transaction.  

| Param | Type | Description |
| --- | --- | --- |
| json | <code>string</code> | json of transfer transaction. |
| private_key | <code>string</code> | private key hex with 0x prefix. |

<a name="hash_transfer"></a>

## hash\_transfer(json) ⇒ <code>string</code>
hash_transfer, hash a transfer transaction.

**Kind**: global function  
**Returns**: <code>string</code> - string hash of transfer transaction with 0x prefix.  

| Param | Type | Description |
| --- | --- | --- |
| json | <code>string</code> | json of transfer transaction. |

<a name="sign_withdraw"></a>

## sign\_withdraw(json, private_key) ⇒ <code>string</code>
sign_withdraw, sign a withdraw transaction.

**Kind**: global function  
**Returns**: <code>string</code> - json signature of withdraw transaction.  

| Param | Type | Description |
| --- | --- | --- |
| json | <code>string</code> | json of withdraw transaction. |
| private_key | <code>string</code> | private key hex with 0x prefix. |

<a name="hash_withdraw"></a>

## hash\_withdraw(json) ⇒ <code>string</code>
hash_withdraw, hash a withdraw transaction.

**Kind**: global function  
**Returns**: <code>string</code> - string hash of withdraw transaction with 0x prefix.  

| Param | Type | Description |
| --- | --- | --- |
| json | <code>string</code> | json of withdraw transaction. |

<a name="sign_limit_order"></a>

## sign\_limit\_order(json, private_key) ⇒ <code>string</code>
sign_limit_order, sign a limit order transaction.

**Kind**: global function  
**Returns**: <code>string</code> - json signature of limit order transaction.  

| Param | Type | Description |
| --- | --- | --- |
| json | <code>string</code> | json of limit order transaction. |
| private_key | <code>string</code> | private key hex with 0x prefix. |

<a name="hash_limit_order"></a>

## hash\_limit\_order(json) ⇒ <code>string</code>
hash_limit_order, sign a limit order transaction.

**Kind**: global function  
**Returns**: <code>string</code> - string hash of limit order transaction with 0x prefix.  

| Param | Type | Description |
| --- | --- | --- |
| json | <code>string</code> | json of limit order transaction. |

<a name="sign_liquidate"></a>

## sign\_liquidate(json, private_key) ⇒ <code>string</code>
sign_liquidate, sign a liquidate transaction.

**Kind**: global function  
**Returns**: <code>string</code> - json signature of liquidate transaction.  

| Param | Type | Description |
| --- | --- | --- |
| json | <code>string</code> | json of liquidate transaction. |
| private_key | <code>string</code> | private key hex with 0x prefix. |

<a name="hash_liquidate"></a>

## hash\_liquidate(json) ⇒ <code>string</code>
hash_liquidate, hash a liquidate transaction.

**Kind**: global function  
**Returns**: <code>string</code> - string hash of liquidate transaction with 0x prefix.  

| Param | Type | Description |
| --- | --- | --- |
| json | <code>string</code> | json of liquidate transaction. |

<a name="sign_signed_oracle_price"></a>

## sign\_signed\_oracle\_price(json, private_key) ⇒ <code>string</code>
sign_signed_oracle_price, sign a signed oracle price transaction.

**Kind**: global function  
**Returns**: <code>string</code> - json signature of liquidate transaction.  

| Param | Type | Description |
| --- | --- | --- |
| json | <code>string</code> | json of liquidate transaction. |
| private_key | <code>string</code> | private key hex with 0x prefix. |

<a name="hash_signed_oracle_price"></a>

## hash\_signed\_oracle\_price(json) ⇒ <code>string</code>
hash_signed_oracle_price, hash a signed oracle price transaction.

**Kind**: global function  
**Returns**: <code>string</code> - string hash of signed oracle transaction with 0x prefix.  

| Param | Type | Description |
| --- | --- | --- |
| json | <code>string</code> | json of signed oracle transaction. |

<a name="verify_signature"></a>

## verify\_signature(sig_r, sig_s, pub_key_x, pub_key_y, msg) ⇒ <code>bool</code>
verify_signature, verify a signature.

**Kind**: global function  
**Returns**: <code>bool</code> - whether the signature is valid.  

| Param | Type | Description |
| --- | --- | --- |
| sig_r | <code>string</code> | r of signature. |
| sig_s | <code>string</code> | s of signature. |
| pub_key_x | <code>string</code> | x of public key. |
| pub_key_y | <code>string</code> | y of public key. |
| msg | <code>string</code> | msg hex with 0x prefix. |

<a name="l1_sign"></a>

## l1\_sign(msg, private_key, string)
l1 sign, sign a msg on l1 when signing a eth address.

**Kind**: global function  

| Param | Type | Description |
| --- | --- | --- |
| msg | <code>string</code> | msg coding in hex with 0x prefix. |
| private_key | <code>string</code> | private key hex with 0x prefix. |
| string | <code>string</code> | of signature. |

<a name="sign_eth_address"></a>

## sign\_eth\_address(address, pubkey, l2_private_key)
sign eth address

**Kind**: global function  

| Param | Type | Description |
| --- | --- | --- |
| address | <code>string</code> | with 0x prefix. |
| pubkey | <code>string</code> | with 0x prefix. |
| l2_private_key | <code>string</code> | with 0x prefix. |

<a name="is_on_curve"></a>

## is\_on\_curve(pub_key_x, pub_key_y) ⇒ <code>bool</code>
is_on_curve, check the (x,y) is on curve.

**Kind**: global function  
**Returns**: <code>bool</code> - whether the (x,y) is on curve.  

| Param | Type | Description |
| --- | --- | --- |
| pub_key_x | <code>string</code> | x of public key with 0x prefix. |
| pub_key_y | <code>string</code> | y of public key with 0x prefix. |

<a name="sign"></a>

## sign(msg, private_key) ⇒ <code>string</code>
sign, sign a msg on l2, is a generic signature methods.

**Kind**: global function  
**Returns**: <code>string</code> - json string of the signature.  

| Param | Type | Description |
| --- | --- | --- |
| msg | <code>string</code> | msg coding in hex with 0x prefix. |
| private_key | <code>string</code> | private key hex with 0x prefix. |

<a name="private_key_from_seed"></a>

## private\_key\_from\_seed(seed) ⇒ <code>string</code>
private_key_from_seed, derive a private key from a random seed, the seed could be anything.

**Kind**: global function  
**Returns**: <code>string</code> - string of private coding in hex with 0x prefix.  

| Param | Type | Description |
| --- | --- | --- |
| seed | <code>string</code> | anything string. |

<a name="private_key_to_pubkey_xy"></a>

## private\_key\_to\_pubkey\_xy(private_key) ⇒ <code>string</code>
private_key_to_pubkey_xy, derive a public with xy from private key.

**Kind**: global function  
**Returns**: <code>string</code> - json string of public key xy.  

| Param | Type | Description |
| --- | --- | --- |
| private_key | <code>string</code> | private key hex with 0x prefix. |

<a name="public_key_to_xy"></a>

## public\_key\_to\_xy(pub_key) ⇒ <code>string</code>
public_key_to_xy, convert public key to xy.

**Kind**: global function  
**Returns**: <code>string</code> - json string of public key xy.  

| Param | Type | Description |
| --- | --- | --- |
| pub_key | <code>string</code> | public key hex with 0x prefix. |

<a name="sign_spot_transfer"></a>

## sign\_spot\_transfer(json, private_key) ⇒ <code>string</code>
sign_spot_transfer, sign a spot transfer transaction.

**Kind**: global function  
**Returns**: <code>string</code> - json signature of spot transfer transaction.  

| Param | Type | Description |
| --- | --- | --- |
| json | <code>string</code> | json of spot transfer transaction. |
| private_key | <code>string</code> | private key hex with 0x prefix. |

<a name="hash_spot_transfer"></a>

## hash\_spot\_transfer(json) ⇒ <code>string</code>
hash_spot_transfer, hash a spot transfer transaction.

**Kind**: global function  
**Returns**: <code>string</code> - string hash of spot transfer transaction with 0x prefix.  

| Param | Type | Description |
| --- | --- | --- |
| json | <code>string</code> | json of spot transfer transaction. |

<a name="sign_spot_withdrawal"></a>

## sign\_spot\_withdrawal(json, private_key) ⇒ <code>string</code>
sign_spot_withdrawal, sign a spot withdrawal transaction.

**Kind**: global function  
**Returns**: <code>string</code> - json signature of spot withdrawal transaction.  

| Param | Type | Description |
| --- | --- | --- |
| json | <code>string</code> | json of spot withdrawal transaction. |
| private_key | <code>string</code> | private key hex with 0x prefix. |

<a name="hash_spot_withdrawal"></a>

## hash\_spot\_withdrawal(json) ⇒ <code>string</code>
hash_spot_withdrawal, hash a spot withdrawal transaction.

**Kind**: global function  
**Returns**: <code>string</code> - string hash of spot withdrawal transaction with 0x prefix.  

| Param | Type | Description |
| --- | --- | --- |
| json | <code>string</code> | json of spot withdrawal transaction. |

<a name="sign_spot_limit_order"></a>

## sign\_spot\_limit\_order(json, private_key) ⇒ <code>string</code>
sign_spot_limit_order, sign a spot LimitOrder transaction.

**Kind**: global function  
**Returns**: <code>string</code> - json signature of spot LimitOrder transaction.  

| Param | Type | Description |
| --- | --- | --- |
| json | <code>string</code> | json of spot LimitOrder transaction. |
| private_key | <code>string</code> | private key hex with 0x prefix. |

<a name="hash_spot_limit_order"></a>

## hash\_spot\_limit\_order(json) ⇒ <code>string</code>
hash_spot_limit_order, hash a spot LimitOrder transaction.

**Kind**: global function  
**Returns**: <code>string</code> - string hash of spot LimitOrder transaction with 0x prefix.  

| Param | Type | Description |
| --- | --- | --- |
| json | <code>string</code> | json of spot LimitOrder transaction. |

