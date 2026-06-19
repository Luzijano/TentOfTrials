#!/usr/bin/env python3
"""Audit lowercase legacy references for an uppercase LEGACY comment marker.

The audit walks tracked text files and fails when a file contains a legacy
reference but does not also contain an uppercase LEGACY marker in a comment.
"""
from __future__ import annotations

import subprocess
import sys
from pathlib import Path

SKIP_PARTS = {'.git', 'diagnostic', 'target', 'node_modules', '__pycache__'}
TEXT_SUFFIXES = {'.py', '.rs', '.md', '.toml', '.yaml', '.yml', '.json', '.txt', '.sh'}
COMMENT_PREFIXES = ('#', '//', '/*', '*', '<!--')


def tracked_files() -> list[Path]:
    result = subprocess.run(['git', 'ls-files'], text=True, stdout=subprocess.PIPE, check=True)
    return [Path(line) for line in result.stdout.splitlines() if line]


def is_text_candidate(path: Path) -> bool:
    return not any(part in SKIP_PARTS for part in path.parts) and path.suffix.lower() in TEXT_SUFFIXES


def read_text(path: Path) -> str:
    return path.read_text(encoding='utf-8', errors='ignore')


def contains_legacy(text: str) -> bool:
    return 'legacy' in text.lower()


def has_legacy_comment(text: str) -> bool:
    for line in text.splitlines():
        stripped = line.strip()
        if 'LEGACY' in stripped and stripped.startswith(COMMENT_PREFIXES):
            return True
    return False


def audit() -> list[str]:
    failures: list[str] = []
    for path in tracked_files():
        if not is_text_candidate(path) or not path.exists():
            continue
        text = read_text(path)
        if contains_legacy(text) and not has_legacy_comment(text):
            failures.append(str(path))
    return failures


def main() -> int:
    failures = audit()
    if failures:
        print('Files with legacy references but no uppercase LEGACY comment marker:')
        for path in failures:
            print(f'- {path}')
        return 1
    print('LEGACY comment audit passed')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
