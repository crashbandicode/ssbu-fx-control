import unittest
from export_effect_names import retained_names


class NameBoundaryTests(unittest.TestCase):
    def test_header_mention_is_not_manual_delimiter(self):
        text = "# names below '# manual' are kept\nold_fighter_effect\n# manual\n# local note\nmanual_effect\n"
        self.assertEqual(retained_names(text),
                         ({"old_fighter_effect"}, ["# local note", "manual_effect"]))

    def test_recovers_legacy_duplicate_boundary_without_junk_or_lost_names(self):
        text = "# names below '# manual' are kept\nnew_stage_effect\n# manual\n' are kept\nold_fighter_effect\n# manual\nmanual_effect\n"
        self.assertEqual(retained_names(text),
                         ({"new_stage_effect", "old_fighter_effect"}, ["manual_effect"]))


if __name__ == "__main__":
    unittest.main()
