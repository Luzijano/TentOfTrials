import unittest

import build


class ModuleSelectionTests(unittest.TestCase):
    def test_parse_module_selection_trims_comma_separated_values(self):
        self.assertEqual(
            build.parse_module_selection("frontend, market,backend"),
            ["frontend", "market", "backend"],
        )

    def test_validate_module_selection_accepts_all(self):
        selected, invalid = build.validate_module_selection("all")
        self.assertEqual(invalid, [])
        self.assertEqual(selected, build.MODULES)

    def test_validate_module_selection_preserves_requested_order(self):
        selected, invalid = build.validate_module_selection("market, frontend")
        self.assertEqual(invalid, [])
        self.assertEqual([module.name for module in selected], ["market", "frontend"])

    def test_validate_module_selection_reports_invalid_names(self):
        selected, invalid = build.validate_module_selection("frontend, nope, missing")
        self.assertEqual(selected, [])
        self.assertEqual(invalid, ["nope", "missing"])

    def test_valid_module_names_lists_known_modules(self):
        names = build.valid_module_names()
        self.assertIn("frontend", names)
        self.assertIn("backend", names)


if __name__ == "__main__":
    unittest.main()
