import json
import subprocess
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))

from tools import db_migration


class DryRunMigrationPlanTests(unittest.TestCase):
    def sample_status(self):
        return [
            {
                "version": "20210101000000",
                "description": "Initial schema",
                "type": "sql",
                "applied": True,
            },
            {
                "version": "20210102000000",
                "description": "Add user profiles",
                "type": "sql",
                "applied": True,
            },
            {
                "version": "20210103000000",
                "description": "Create audit logs",
                "type": "sql",
                "applied": False,
            },
        ]

    def test_no_pending_migrations_produces_empty_up_plan(self):
        status = [dict(item, applied=True) for item in self.sample_status()]

        plan = db_migration.build_dry_run_plan(status, direction="up")

        self.assertEqual(plan, [])

    def test_pending_migrations_include_required_machine_readable_fields(self):
        plan = db_migration.build_dry_run_plan(self.sample_status(), direction="up")

        self.assertEqual(
            plan,
            [
                {
                    "version": "20210103000000",
                    "description": "Create audit logs",
                    "direction": "up",
                    "execution_would_be_attempted": True,
                }
            ],
        )

    def test_specific_rollback_target_plans_applied_migrations_in_reverse_order(self):
        plan = db_migration.build_dry_run_plan(
            self.sample_status(),
            direction="down",
            target_version="20210101000000",
        )

        self.assertEqual(
            [item["version"] for item in plan],
            ["20210102000000", "20210101000000"],
        )
        self.assertTrue(all(item["direction"] == "down" for item in plan))
        self.assertTrue(all(item["execution_would_be_attempted"] for item in plan))

    def test_rollback_to_unapplied_migration_raises_error(self):
        with self.assertRaises(ValueError) as ctx:
            db_migration.build_dry_run_plan(
                self.sample_status(),
                direction="down",
                target_version="20210103000000",
            )

        self.assertIn("not yet applied", str(ctx.exception))

    def test_cli_up_dry_run_emits_json_plan_without_psql(self):
        result = subprocess.run(
            [sys.executable, str(ROOT / "tools" / "db_migration.py"), "--up", "--dry-run"],
            check=True,
            capture_output=True,
            text=True,
        )

        payload = json.loads(result.stdout)
        self.assertTrue(payload["dry_run"])
        self.assertGreater(len(payload["plan"]), 0)
        self.assertEqual(payload["plan"][0]["direction"], "up")
        self.assertIn("execution_would_be_attempted", payload["plan"][0])


if __name__ == "__main__":
    unittest.main()
