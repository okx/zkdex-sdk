## Functions

<dl>
<dt><a href="#sign_transfer">sign_transfer(json, private)</a> ⇒ <code>string</code></dt>
<dd><p>sign a transfer transaction</p>
</dd>
<dt><a href="#hash_transfer">hash_transfer(json)</a> ⇒ <code>string</code></dt>
<dd><p>hash a transfer transaction</p>
</dd>
<dt><a href="#sign_withdraw">sign_withdraw(json, private_key)</a> ⇒ <code>string</code></dt>
<dd><p>sign a withdraw transaction</p>
</dd>
<dt><a href="#hash_withdraw">hash_withdraw(json)</a> ⇒ <code>string</code></dt>
<dd><p>hash a withdraw transaction</p>
</dd>
<dt><a href="#sign_limit_order">sign_limit_order(json, private_key)</a> ⇒ <code>string</code></dt>
<dd><p>sign a limit order transaction</p>
</dd>
<dt><a href="#hash_limit_order">hash_limit_order(json)</a> ⇒ <code>string</code></dt>
<dd><p>sign a limit order transaction</p>
</dd>
<dt><a href="#sign_liquidate">sign_liquidate(json, private_key)</a> ⇒ <code>string</code></dt>
<dd><p>sign a liquidate transaction</p>
</dd>
<dt><a href="#hash_liquidate">hash_liquidate(json)</a> ⇒ <code>string</code></dt>
<dd><p>hash a liquidate transaction</p>
</dd>
<dt><a href="#sign_signed_oracle_price">sign_signed_oracle_price(json, private_key)</a> ⇒ <code>string</code></dt>
<dd><p>sign a signed oracle price transaction</p>
</dd>
<dt><a href="#hash_signed_oracle_price">hash_signed_oracle_price(json)</a> ⇒ <code>string</code></dt>
<dd><p>hash a signed oracle price transaction</p>
</dd>
<dt><a href="#verify_signature">verify_signature(sig_r, sig_s, pub_key_x, pub_key_y, msg)</a> ⇒ <code>boolean</code></dt>
<dd><p>verify a signature</p>
</dd>
<dt><a href="#l1_sign">l1_sign(msg, private_key)</a> ⇒ <code>string</code></dt>
<dd><p>l1 sign
sign a msg on l1 when signing a eth address</p>
</dd>
<dt><a href="#is_on_curve">is_on_curve(pub_key_x, pub_key_y)</a> ⇒ <code>boolean</code></dt>
<dd><p>check the (x,y) is on curve</p>
</dd>
<dt><a href="#sign">sign(pri_key, msg)</a> ⇒ <code>string</code></dt>
<dd><p>sign a msg on l2</p>
</dd>
<dt><a href="#private_key_from_seed">private_key_from_seed(seed)</a> ⇒ <code>string</code></dt>
<dd><p>derive a private key from a random seed, the seed could be anything</p>
</dd>
<dt><a href="#private_key_to_pubkey_xy">private_key_to_pubkey_xy(pri_key)</a> ⇒ <code>string</code></dt>
<dd><p>derive a public with xy from private key</p>
</dd>
<dt><a href="#public_key_to_xy">public_key_to_xy(pub_key)</a> ⇒ <code>string</code></dt>
<dd><p>convert public key to xy</p>
</dd>
<dt><a href="#zkdex_init">zkdex_init()</a></dt>
<dd><p>This method initializes params for current thread, otherwise they will be initialized when signing
first message.</p>
</dd>
</dl>

<a name="sign_transfer"></a>

## sign\_transfer(json, private) ⇒ <code>string</code>
sign a transfer transaction

**Kind**: global function  
**Returns**: <code>string</code> - signature of transfer transaction  

| Param | Type | Description |
| --- | --- | --- |
| json | <code>string</code> | of transfer transaction |
| private | <code>string</code> | key hex with 0x prefix |

<a name="hash_transfer"></a>

## hash\_transfer(json) ⇒ <code>string</code>
hash a transfer transaction

**Kind**: global function  

| Param | Type |
| --- | --- |
| json | <code>string</code> | 

<a name="sign_withdraw"></a>

## sign\_withdraw(json, private_key) ⇒ <code>string</code>
sign a withdraw transaction

**Kind**: global function  

| Param | Type |
| --- | --- |
| json | <code>string</code> | 
| private_key | <code>string</code> | 

<a name="hash_withdraw"></a>

## hash\_withdraw(json) ⇒ <code>string</code>
hash a withdraw transaction

**Kind**: global function  

| Param | Type |
| --- | --- |
| json | <code>string</code> | 

<a name="sign_limit_order"></a>

## sign\_limit\_order(json, private_key) ⇒ <code>string</code>
sign a limit order transaction

**Kind**: global function  

| Param | Type |
| --- | --- |
| json | <code>string</code> | 
| private_key | <code>string</code> | 

<a name="hash_limit_order"></a>

## hash\_limit\_order(json) ⇒ <code>string</code>
sign a limit order transaction

**Kind**: global function  

| Param | Type |
| --- | --- |
| json | <code>string</code> | 

<a name="sign_liquidate"></a>

## sign\_liquidate(json, private_key) ⇒ <code>string</code>
sign a liquidate transaction

**Kind**: global function  

| Param | Type |
| --- | --- |
| json | <code>string</code> | 
| private_key | <code>string</code> | 

<a name="hash_liquidate"></a>

## hash\_liquidate(json) ⇒ <code>string</code>
hash a liquidate transaction

**Kind**: global function  

| Param | Type |
| --- | --- |
| json | <code>string</code> | 

<a name="sign_signed_oracle_price"></a>

## sign\_signed\_oracle\_price(json, private_key) ⇒ <code>string</code>
sign a signed oracle price transaction

**Kind**: global function  

| Param | Type |
| --- | --- |
| json | <code>string</code> | 
| private_key | <code>string</code> | 

<a name="hash_signed_oracle_price"></a>

## hash\_signed\_oracle\_price(json) ⇒ <code>string</code>
hash a signed oracle price transaction

**Kind**: global function  

| Param | Type |
| --- | --- |
| json | <code>string</code> | 

<a name="verify_signature"></a>

## verify\_signature(sig_r, sig_s, pub_key_x, pub_key_y, msg) ⇒ <code>boolean</code>
verify a signature

**Kind**: global function  

| Param | Type |
| --- | --- |
| sig_r | <code>string</code> | 
| sig_s | <code>string</code> | 
| pub_key_x | <code>string</code> | 
| pub_key_y | <code>string</code> | 
| msg | <code>string</code> | 

<a name="l1_sign"></a>

## l1\_sign(msg, private_key) ⇒ <code>string</code>
l1 sign
sign a msg on l1 when signing a eth address

**Kind**: global function  

| Param | Type |
| --- | --- |
| msg | <code>string</code> | 
| private_key | <code>string</code> | 

<a name="is_on_curve"></a>

## is\_on\_curve(pub_key_x, pub_key_y) ⇒ <code>boolean</code>
check the (x,y) is on curve

**Kind**: global function  

| Param | Type |
| --- | --- |
| pub_key_x | <code>string</code> | 
| pub_key_y | <code>string</code> | 

<a name="sign"></a>

## sign(pri_key, msg) ⇒ <code>string</code>
sign a msg on l2

**Kind**: global function  

| Param | Type |
| --- | --- |
| pri_key | <code>string</code> | 
| msg | <code>string</code> | 

<a name="private_key_from_seed"></a>

## private\_key\_from\_seed(seed) ⇒ <code>string</code>
derive a private key from a random seed, the seed could be anything

**Kind**: global function  

| Param | Type |
| --- | --- |
| seed | <code>string</code> | 

<a name="private_key_to_pubkey_xy"></a>

## private\_key\_to\_pubkey\_xy(pri_key) ⇒ <code>string</code>
derive a public with xy from private key

**Kind**: global function  

| Param | Type |
| --- | --- |
| pri_key | <code>string</code> | 

<a name="public_key_to_xy"></a>

## public\_key\_to\_xy(pub_key) ⇒ <code>string</code>
convert public key to xy

**Kind**: global function  

| Param | Type |
| --- | --- |
| pub_key | <code>string</code> | 

<a name="zkdex_init"></a>

## zkdex\_init()
This method initializes params for current thread, otherwise they will be initialized when signing
first message.

**Kind**: global function  
