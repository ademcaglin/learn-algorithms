import unittest
from src.array.is_anagram import is_anagram2, is_anagram


class TestDiff(unittest.TestCase):
    def test_is_anagram2(self):
        self.assertEqual(is_anagram2("hot dog", "ghot do"), True)
    def test_is_anagram(self):
        self.assertEqual(is_anagram("hot dog", "g hot do"), True)