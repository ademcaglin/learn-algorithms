import unittest
from src.diff import diff


class TestDiff(unittest.TestCase):
    def test_diff(self):
        self.assertEqual(diff("aabaac", "acda"), "baa")