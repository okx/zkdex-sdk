## Functions

<dl>
<dt><a href="#zksync_crypto_init">zksync_crypto_init()</a></dt>
<dd><p>This method initializes params for current thread, otherwise they will be initialized when signing
first message.</p>
</dd>
<dt><a href="#pubKeyHash">pubKeyHash(pubkey)</a> ⇒ <code>Uint8Array</code></dt>
<dd></dd>
<dt><a href="#private_key_to_pubkey_hash">private_key_to_pubkey_hash(private_key)</a> ⇒ <code>Uint8Array</code></dt>
<dd></dd>
<dt><a href="#sign_transfer">sign_transfer(json, private_key)</a> ⇒ <code>string</code></dt>
<dd></dd>
<dt><a href="#hash_transfer">hash_transfer(json)</a> ⇒ <code>string</code></dt>
<dd></dd>
<dt><a href="#sign_withdraw">sign_withdraw(json, asset_id_collateral, private_key)</a> ⇒ <code>string</code></dt>
<dd></dd>
<dt><a href="#hash_withdraw">hash_withdraw(json, asset_id_collateral)</a> ⇒ <code>string</code></dt>
<dd></dd>
<dt><a href="#sign_limit_order">sign_limit_order(json, private_key)</a> ⇒ <code>string</code></dt>
<dd></dd>
<dt><a href="#hash_limit_order">hash_limit_order(json)</a> ⇒ <code>string</code></dt>
<dd></dd>
<dt><a href="#sign_liquidate">sign_liquidate(json, private_key)</a> ⇒ <code>string</code></dt>
<dd></dd>
<dt><a href="#hash_liquidate">hash_liquidate(json)</a> ⇒ <code>string</code></dt>
<dd></dd>
<dt><a href="#sign_signed_oracle_price">sign_signed_oracle_price(json, private_key)</a> ⇒ <code>string</code></dt>
<dd></dd>
<dt><a href="#hash_signed_oracle_price">hash_signed_oracle_price(json)</a> ⇒ <code>string</code></dt>
<dd></dd>
<dt><a href="#verify_signature">verify_signature(sig_r, sig_s, pub_key_x, pub_key_y, msg)</a> ⇒ <code>boolean</code></dt>
<dd></dd>
<dt><a href="#l1_sign">l1_sign(msg, private_key)</a> ⇒ <code>string</code></dt>
<dd></dd>
<dt><a href="#is_on_curve">is_on_curve(pub_key_x, pub_key_y)</a> ⇒ <code>boolean</code></dt>
<dd></dd>
<dt><a href="#sign">sign(pri_key, msg)</a> ⇒ <code>string</code></dt>
<dd></dd>
<dt><a href="#private_key_from_seed">private_key_from_seed(seed)</a> ⇒ <code>string</code></dt>
<dd></dd>
<dt><a href="#private_key_to_pubkey_xy">private_key_to_pubkey_xy(pri_key)</a> ⇒ <code>string</code></dt>
<dd></dd>
</dl>

<a name="zksync_crypto_init"></a>

## zksync\_crypto\_init()
This method initializes params for current thread, otherwise they will be initialized when signing
first message.

**Kind**: global function  
<a name="pubKeyHash"></a>

## pubKeyHash(pubkey) ⇒ <code>Uint8Array</code>
**Kind**: global function  

| Param | Type |
| --- | --- |
| pubkey | <code>Uint8Array</code> | 

<a name="private_key_to_pubkey_hash"></a>

## private\_key\_to\_pubkey\_hash(private_key) ⇒ <code>Uint8Array</code>
**Kind**: global function  

| Param | Type |
| --- | --- |
| private_key | <code>Uint8Array</code> | 

<a name="sign_transfer"></a>

## sign\_transfer(json, private_key) ⇒ <code>string</code>
**Kind**: global function  

| Param | Type |
| --- | --- |
| json | <code>string</code> | 
| private_key | <code>string</code> | 

<a name="hash_transfer"></a>

## hash\_transfer(json) ⇒ <code>string</code>
**Kind**: global function  

| Param | Type |
| --- | --- |
| json | <code>string</code> | 

<a name="sign_withdraw"></a>

## sign\_withdraw(json, asset_id_collateral, private_key) ⇒ <code>string</code>
**Kind**: global function  

| Param | Type |
| --- | --- |
| json | <code>string</code> | 
| asset_id_collateral | <code>string</code> | 
| private_key | <code>string</code> | 

<a name="hash_withdraw"></a>

## hash\_withdraw(json, asset_id_collateral) ⇒ <code>string</code>
**Kind**: global function  

| Param | Type |
| --- | --- |
| json | <code>string</code> | 
| asset_id_collateral | <code>string</code> | 

<a name="sign_limit_order"></a>

## sign\_limit\_order(json, private_key) ⇒ <code>string</code>
**Kind**: global function  

| Param | Type |
| --- | --- |
| json | <code>string</code> | 
| private_key | <code>string</code> | 

<a name="hash_limit_order"></a>

## hash\_limit\_order(json) ⇒ <code>string</code>
**Kind**: global function  

| Param | Type |
| --- | --- |
| json | <code>string</code> | 

<a name="sign_liquidate"></a>

## sign\_liquidate(json, private_key) ⇒ <code>string</code>
**Kind**: global function  

| Param | Type |
| --- | --- |
| json | <code>string</code> | 
| private_key | <code>string</code> | 

<a name="hash_liquidate"></a>

## hash\_liquidate(json) ⇒ <code>string</code>
**Kind**: global function  

| Param | Type |
| --- | --- |
| json | <code>string</code> | 

<a name="sign_signed_oracle_price"></a>

## sign\_signed\_oracle\_price(json, private_key) ⇒ <code>string</code>
**Kind**: global function  

| Param | Type |
| --- | --- |
| json | <code>string</code> | 
| private_key | <code>string</code> | 

<a name="hash_signed_oracle_price"></a>

## hash\_signed\_oracle\_price(json) ⇒ <code>string</code>
**Kind**: global function  

| Param | Type |
| --- | --- |
| json | <code>string</code> | 

<a name="verify_signature"></a>

## verify\_signature(sig_r, sig_s, pub_key_x, pub_key_y, msg) ⇒ <code>boolean</code>
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
**Kind**: global function  

| Param | Type |
| --- | --- |
| msg | <code>string</code> | 
| private_key | <code>string</code> | 

<a name="is_on_curve"></a>

## is\_on\_curve(pub_key_x, pub_key_y) ⇒ <code>boolean</code>
**Kind**: global function  

| Param | Type |
| --- | --- |
| pub_key_x | <code>string</code> | 
| pub_key_y | <code>string</code> | 

<a name="sign"></a>

## sign(pri_key, msg) ⇒ <code>string</code>
**Kind**: global function  

| Param | Type |
| --- | --- |
| pri_key | <code>string</code> | 
| msg | <code>string</code> | 

<a name="private_key_from_seed"></a>

## private\_key\_from\_seed(seed) ⇒ <code>string</code>
**Kind**: global function  

| Param | Type |
| --- | --- |
| seed | <code>string</code> | 

<a name="private_key_to_pubkey_xy"></a>

## private\_key\_to\_pubkey\_xy(pri_key) ⇒ <code>string</code>
**Kind**: global function  

| Param | Type |
| --- | --- |
| pri_key | <code>string</code> | 

