import unittest
import zkdex_python_sdk
import json
class TestStringMethods(unittest.TestCase):

    def test_upper(self):
        r = zkdex_python_sdk.sum_as_string(1,2)
        print(r)
        self.assertEqual('3', r)

    def test_publickey_to_xy(self):
        r = zkdex_python_sdk.public_key_to_xy("0x8f79ad4f9b161ad77e37423d3709e0fc3d694259f4ec84c354f532e58643faa")
        print(r)

    def test_isupper(self):
        self.assertTrue('FOO'.isupper())
        self.assertFalse('Foo'.isupper())

    def test_split(self):
        s = 'hello world'
        self.assertEqual(s.split(), ['hello', 'world'])
        # check that s.split fails when the separator is not a string
        with self.assertRaises(TypeError):
            s.split(2)

if __name__ == '__main__':
    unittest.main()