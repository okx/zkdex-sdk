import unittest
import zkdex_python_sdk
import json

pri_key = "0x028dd913a169cf3732c306959e9c2a66a0075663e54e086977ed71c61fd7c273"
pk_x = "0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa"
pk_y = "0x09e3c9c66770d2f49401e83b0d07e20f74a311d354505aea32f900b9d533d5f7"

class TestZKDEX(unittest.TestCase):



    def test_verify_signature(self):
        sigr = "0x2e39e39381ac5e962650072a8936b99716fc0b3fda124f59ef62066301fd0749"
        sigs = "0x37fd915bf958893ed35132a91b98fc4fcd7821c9fe784057bbc85d8fc5e7d4f"
        msg = "0x08a09b19adaa35815065dffcc4b5e0ee75f54660eb474c5932929b96c0ff15c9"
        r = zkdex_python_sdk.verify_signature(sigr, sigs, pk_x, pk_y, msg)
        self.assertTrue(r)

    def test_sign_withdraw(self):
        json_str = "{\"nonce\":\"1\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"1684832800\",\"position_id\":\"2\",\"amount\":\"3\",\"eth_address\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"asset_id\":\"0x1\"}";
        r = zkdex_python_sdk.sign_withdraw(json_str, pri_key)
        sig = json.loads(r)
        self.assertEqual('0xa5d62dbb0566a1b69162df475097fbfca6a317535ea59ea3275580dce2d7043e', sig['r'])
        self.assertEqual('0x03c61d342a339d329341494ee136ccadf10675b9f8f90894e6a9e86ac6a19dec', sig['s'])

    def test_sign_transfer(self):
        json_str = "{\"nonce\":\"0\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"0\",\"sender_position_id\":\"0\",\"receiver_public_key\":\"0x0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":\"0\",\"amount\":\"0\",\"asset_id\":\"0xa\"}";
        r = zkdex_python_sdk.sign_transfer(json_str, pri_key)
        sig = json.loads(r)
        self.assertEqual('0xa5920612d2b265813f31ee169b9e96e89548bdd53e9f4541e53fcdb1205c9c9a', sig['r'])
        self.assertEqual('0x0028bdb4cc8f9f70c6ad081c03d662599fe732c118f268e537da019e3b473a09', sig['s'])

    def test_sign_limit_order(self):
        json_str = "{\"nonce\":\"1\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"2\",\"amount_synthetic\":\"3\",\"amount_collateral\":\"4\",\"amount_fee\":\"5\",\"asset_id_synthetic\":\"0x6\",\"asset_id_collateral\":\"0x7\",\"position_id\":\"8\",\"is_buying_synthetic\":false}";
        r = zkdex_python_sdk.sign_limit_order(json_str, pri_key)
        sig = json.loads(r)
        self.assertEqual('0xb009ccc02daa847671c14bbe2ae576076d0ed8e4ed9af3b8553b1090a122f2b7', sig['r'])
        self.assertEqual('0x0319dcc4dde119be949f194aeaa727d4ac0a1666f4e260436b1a9fd5b9d4e739', sig['s'])

    def test_sign_liquidate(self):
        json_str = "{\"liquidator_order\":{\"nonce\":\"0\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"0\",\"amount_synthetic\":\"1\",\"amount_collateral\":\"2\",\"amount_fee\":\"3\",\"asset_id_synthetic\":\"4\",\"asset_id_collateral\":\"0x5\",\"position_id\":\"6\",\"is_buying_synthetic\":false},\"liquidated_position_id\":\"7\",\"actual_collateral\":\"8\",\"actual_synthetic\":\"9\",\"actual_liquidator_fee\":\"10\"}";
        r = zkdex_python_sdk.sign_liquidate(json_str, pri_key)
        sig = json.loads(r)
        self.assertEqual('0xa2b928904a4015f324244432ac4cc28286446f93cc6e0e8fcd0f6a9278a152f5', sig['r'])
        self.assertEqual('0x01b612dd6801d8044f3ad6e345cabc3c7f41a02ecfdfe3c48fd81eb4ac01fd36', sig['s'])

    def test_sign_signed_oracle_price(self):
        json_str = "{\"signer_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"external_price\":\"1\",\"timestamp\":\"2\",\"signed_asset_id\":\"0x3\"}";
        r = zkdex_python_sdk.sign_signed_oracle_price(json_str, pri_key)
        sig = json.loads(r)
        self.assertEqual('0x8510a3eab6ac786e2c97c59db9fc5ea60eb39057b61e746fe2120e02c163fd4b', sig['r'])
        self.assertEqual('0x035ac9dd0980f0625b5d540ce43b62171cb80ed07cc63df88a8990ce2f4ea293', sig['s'])

    def test_hash_withdraw(self):
        json_str = "{\"nonce\":\"1\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"1684832800\",\"position_id\":\"2\",\"amount\":\"3\",\"eth_address\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"asset_id\":\"0x1\"}";
        r = zkdex_python_sdk.hash_withdraw(json_str)
        self.assertEqual('0x22e58e85163d975aba853ef13742320fc8f7b5e1fed5667e37a275916e96a561', r)

    def test_hash_transfer(self):
        json_str = "{\"nonce\":\"0\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"0\",\"sender_position_id\":\"0\",\"receiver_public_key\":\"0x0000000000000000000000000000000000000000000000000000000000000000\",\"receiver_position_id\":\"0\",\"amount\":\"0\",\"asset_id\":\"0xa\"}"
        r = zkdex_python_sdk.hash_transfer(json_str)
        self.assertEqual('0x023408af1feaf9432599c6562003b4f105a83aa7fa5bf9dbfb17e37d2f876601', r)

    def test_hash_limit_order(self):
        json_str = "{\"nonce\":\"1\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"2\",\"amount_synthetic\":\"3\",\"amount_collateral\":\"4\",\"amount_fee\":\"5\",\"asset_id_synthetic\":\"6\",\"asset_id_collateral\":\"0x7\",\"position_id\":\"8\",\"is_buying_synthetic\":false}";
        r = zkdex_python_sdk.hash_limit_order(json_str)
        self.assertEqual('0x151301a401fab9fdf8d88f5d28261740a9fb7ecbfc1110312e67480a40deb51c', r)

    def test_hash_liquidate(self):
        json_str = "{\"liquidator_order\":{\"nonce\":\"0\",\"public_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"expiration_timestamp\":\"0\",\"amount_synthetic\":\"1\",\"amount_collateral\":\"2\",\"amount_fee\":\"3\",\"asset_id_synthetic\":\"4\",\"asset_id_collateral\":\"0x5\",\"position_id\":\"6\",\"is_buying_synthetic\":false},\"liquidated_position_id\":\"7\",\"actual_collateral\":\"8\",\"actual_synthetic\":\"9\",\"actual_liquidator_fee\":\"10\"}";
        r = zkdex_python_sdk.hash_liquidate(json_str)
        self.assertEqual('0x11fbfdb033ed2a370a6213e172d48aa152254597ec9d16b7d851ffacfa9ae29e', r)

    def test_hash_signed_oracle_price(self):
        json_str = "{\"signer_key\":\"0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa\",\"external_price\":\"1\",\"timestamp\":\"2\",\"signed_asset_id\":\"0x3\"}";
        r = zkdex_python_sdk.hash_signed_oracle_price(json_str)
        self.assertEqual('0x2aff026c07ab995e4874dd866d81ed332ed5580d938a4a4ef0b7f54ed500c9e2', r)

    def test_sign(self):
        hash = '0x4068df25a7d520d7b11133a1c6ef27d009400e55bba6bf9b59c6cef63cb37d12'
        r = zkdex_python_sdk.sign(pri_key, hash)
        sig = json.loads(r)
        self.assertEqual('0x1fb6b3bd2d5cf21862d38a189a16056926d00dc37cdc36586b60b2c9115c762c', sig['r'])
        self.assertEqual('0x002b7e082743fd090f407430dc28e7031af4191f7e876b1abb01c901583170ca', sig['s'])

    def test_is_on_curve(self):
        x = "0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa"
        y = "0x09e3c9c66770d2f49401e83b0d07e20f74a311d354505aea32f900b9d533d5f7"
        r = zkdex_python_sdk.is_on_curve(x, y)
        self.assertTrue(r)

    def test_private_key_from_seed(self):
        seed = "hello world good life 996 very nice"
        r = zkdex_python_sdk.private_key_from_seed(seed)
        self.assertEqual('0x02aca28609503a6474ec0a115b8662dbf760b6da6109e17c757dbbd3835c93f9',r)

    def test_private_key_to_public_key_xy(self):
        pri_key = '0x028dd913a169cf3732c306959e9c2a66a0075663e54e086977ed71c61fd7c273'
        r = zkdex_python_sdk.private_key_to_public_key_xy(pri_key)
        pk = json.loads(r)
        self.assertEqual(pk['x'], "0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa")
        self.assertEqual(pk['y'], "0x09e3c9c66770d2f49401e83b0d07e20f74a311d354505aea32f900b9d533d5f7")

    def test_publickey_to_xy(self):
        r = zkdex_python_sdk.public_key_to_xy("0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa")
        pk = json.loads(r)
        self.assertEqual(pk['x'], "0x8f792ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa")
        self.assertEqual(pk['y'], "0x09e3c9c66770d2f49401e83b0d07e20f74a311d354505aea32f900b9d533d5f7")

    def test_eth_sign(self):
        hash = "0x196cdf49e6d3f3614fdba8e3459fef498685b88627b80035c62beaa7ca056eea"
        pri_key = "0x03f2d0a8ec58aac5ad28ac9bbc76a43c2f40c167885c9117b5863545dd2471f3"
        r = zkdex_python_sdk.eth_sign(pri_key, hash)
        sig = json.loads(r)
        self.assertEqual(sig['x'], "0x062b74e4bde7c5655093bcfd717b2be2757fc7c85f2b5fdc0f43820df2ce510a")
        self.assertEqual(sig['y'], "0x124c1159c6164b8f80348f23a39ff79af229ecb2f00e806e60798601607c4595")
        self.assertEqual(sig['s'], "0x04f89ebc83800e89f19e3501562793e2d9097b921ee0759b5f37017b993238c4")
        self.assertEqual(sig['pk_x'], "0x96c4d93a49c8159e27542601ba19fdfce52b3e9b43dafaefe9aa9cd32efded86")
        self.assertEqual(sig['pk_y'], "0x0cc8a68b8dba85bd5418e308b34439ddffca3a0f6589a32f02adf60da6e73f55")


if __name__ == '__main__':
    unittest.main()