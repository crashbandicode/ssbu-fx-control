"""Actor, stage-particle and model traces keep their identities and high bits."""
import io
import json
import sys
import tempfile
import unittest
from contextlib import redirect_stdout
from pathlib import Path
import resolve_trace


class TraceFormatTests(unittest.TestCase):
    def test_fighter_and_stage_records_do_not_confuse_packed_fields(self):
        fighter = resolve_trace.hash40("edge_fire3_burst")
        stage = resolve_trace.hash40("stg_trail_castle_bubble")
        model_path = 0x2fa44f1dfb
        packed_actor = 1 | (0x123 << 8) | (0xffff << 24) | (512 << 40)
        packed_stage = (361 << 32) | 511
        lines = ["1 10 110 0x0 0x1", "2 11 111 0x1 0x4",
                 f"3 12 110 {fighter:#x} {packed_actor:#x}",
                 f"4 13 114 {stage:#x} {packed_stage:#x}",
                 f"5 14 112 {stage:#x} {packed_stage:#x}",
                  f"6 15 116 {model_path:#x} {(347 << 32) | 512:#x}",
                  f"7 16 115 {model_path:#x} {packed_stage:#x}"]
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "events.log"
            path.write_text("\n".join(lines))
            rows = resolve_trace.read_events(path)
        self.assertEqual(rows, [
            {"latched": True, "tick": 10},
            {"tick": 12, "hash": fighter, "category": 1, "kind": 0x123,
             "status": -1, "ordinal": 512},
            {"tick": 13, "hash": stage, "category": 254, "kind": 361,
             "status": -1, "ordinal": 511},
            {"tick": 15, "hash": model_path, "category": 253, "kind": 347,
             "status": -1, "ordinal": 512},
        ])

    def test_stage_diagnostics_keep_negative_stage_sentinel_and_flag_reason_split(self):
        name = resolve_trace.hash40("stg_end_swarm")
        # stage -1, flags name-hit|row-off (9), caller sentinel.
        particle_b = (0xFFFF << 48) | (0x9 << 32) | 0xFFFFFFFF
        # Same hash and caller with flags cleared: On must not collapse into Off.
        particle_on = (0xFFFF << 48) | 0xFFFFFFFF
        # stage 351, reason contended (4). Not a particle flag.
        count_a = (351 << 32) | 4
        # stage -1, reason requested-native-hidden (6). Low 16 bits are not flag 1.
        negative_count = (0xFFFFFFFF << 32) | 6
        lines = [
            "1 10 110 0x0 0x0",
            f"2 11 117 0x1 0x0",
            f"3 12 118 {count_a:#x} 0x2a",
            f"4 13 119 {name:#x} {particle_b:#x}",
            f"5 14 119 {name:#x} {particle_on:#x}",
            f"6 15 118 {negative_count:#x} 0x4",
            "7 16 114 0xabc 0x15f00000200",
            "8 17 117 0x0 0x0",
        ]
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "events.log"
            path.write_text("\n".join(lines))
            rows = resolve_trace.read_events(path)
            diag = resolve_trace.read_stage_diagnostics(path)
        self.assertEqual(rows[0], {"latched": False, "tick": 10})
        self.assertEqual(rows[1]["category"], 254)
        self.assertEqual(rows[1]["kind"], 0x15F)
        self.assertEqual(len(rows), 2)
        self.assertEqual(diag["enabled"], [
            {"tick": 11, "enabled": True},
            {"tick": 17, "enabled": False},
        ])
        contended = diag["model_counts"][0]
        self.assertEqual(contended["stage_id"], 351)
        self.assertEqual(contended["reason"], 4)
        self.assertEqual(contended["reason_name"], "contended")
        self.assertEqual(contended["count"], 0x2A)
        hidden = diag["model_counts"][1]
        self.assertEqual(hidden["stage_id"], -1)
        self.assertEqual(hidden["reason"], 6)
        self.assertEqual(hidden["reason_name"], "requested-native-hidden")
        self.assertNotEqual(hidden["reason"], resolve_trace.PARTICLE_FLAGS[0][0])
        off = diag["particle_observations"][0]
        on = diag["particle_observations"][1]
        self.assertEqual(off["hash"], name & 0xFFFFFFFFFF)
        self.assertEqual(off["stage_id"], -1)
        self.assertEqual(off["flags"], 9)
        self.assertEqual(off["flag_names"], ["name-hit", "row-off"])
        self.assertTrue(off["caller_unrepresentable"])
        self.assertEqual(off["caller"], 0xFFFFFFFF)
        self.assertEqual(on["flags"], 0)
        self.assertEqual(on["flag_names"], [])
        self.assertNotEqual(off["flags"], on["flags"])
        self.assertNotEqual(contended["reason_name"], "name-hit")

    def test_json_keeps_110_groups_separate_and_omits_empty_diagnostics(self):
        name = resolve_trace.hash40("stg_end_swarm")
        particle_b = (0xFFFF << 48) | (0x9 << 32) | 0xFFFFFFFF
        with tempfile.TemporaryDirectory() as directory:
            directory = Path(directory)
            names = directory / "names.txt"
            names.write_text("stg_end_swarm\n")
            mixed = directory / "mixed.log"
            mixed.write_text(
                "1 10 110 0x0 0x1\n"
                f"2 11 117 0x1 0x0\n"
                f"3 12 119 {name:#x} {particle_b:#x}\n"
                "4 13 112 0x1 0x2\n"
            )
            only = directory / "only.log"
            only.write_text("1 10 110 0x0 0x1\n9 20 116 0x2edb810e01 0x5f00000003\n")
            mixed_payload = self.run_json(mixed, names)
            only_payload = self.run_json(only, names)
        self.assertEqual(mixed_payload["latched"], [True])
        self.assertEqual(mixed_payload["requests"], [])
        particle = mixed_payload["stage_diagnostics"]["particle_observations"][0]
        self.assertEqual(particle["stage_id"], -1)
        self.assertEqual(particle["name"], "stg_end_swarm")
        self.assertTrue(particle["caller_unrepresentable"])
        self.assertNotIn("stage_diagnostics", only_payload)
        self.assertEqual(only_payload["latched"], [True])
        self.assertEqual(only_payload["requests"][0]["category"], 0xFD)
        self.assertEqual(only_payload["requests"][0]["kind"], 0x5F)
        self.assertEqual(set(only_payload), {"latched", "requests"})

    def run_json(self, events, names):
        argv = sys.argv
        sys.argv = ["resolve_trace.py", str(events), "--json", "--names", str(names)]
        buffer = io.StringIO()
        try:
            with redirect_stdout(buffer):
                resolve_trace.main()
        finally:
            sys.argv = argv
        return json.loads(buffer.getvalue())


if __name__ == "__main__":
    unittest.main()
