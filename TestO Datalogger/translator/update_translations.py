#!/usr/bin/env python3
"""
Sync jobs_all_en_EN.txt from translations_table.json.
Preserves the original file's line endings (CRLF on Windows).
"""

import json
import os

TRANSLATOR_JSON = os.path.join(os.path.dirname(__file__), "translations_table.json")
TRANSLATIONS_TXT = os.path.join(os.path.dirname(__file__), "..", "translations", "jobs_all_en_EN.txt")


def load_json(path):
    with open(path, encoding="utf-8") as f:
        return json.load(f)


def main():
    table = load_json(TRANSLATOR_JSON)

    # Read raw bytes to detect line ending and preserve encoding exactly
    with open(TRANSLATIONS_TXT, encoding="utf-8", newline="") as f:
        lines = f.readlines()

    # Detect original line ending (CRLF or LF)
    eol = "\r\n" if lines and "\r\n" in lines[0] else "\n"

    updated_l = 0
    updated_s = 0

    for i, line in enumerate(lines):
        parts = line.rstrip("\r\n").split("\t")
        if len(parts) < 2:
            continue
        key, typ = parts[0], parts[1]
        current_val = parts[2] if len(parts) > 2 else ""

        if key not in table:
            continue

        entry = table[key]

        if typ == "RES_L" and entry.get("L"):
            new_val = entry["L"]
            if new_val != current_val:
                lines[i] = f"{key}\t{typ}\t{new_val}{eol}"
                updated_l += 1

        elif typ == "RES_S" and entry.get("S"):
            new_val = entry["S"]
            if new_val != current_val:
                lines[i] = f"{key}\t{typ}\t{new_val}{eol}"
                updated_s += 1

    with open(TRANSLATIONS_TXT, "w", encoding="utf-8", newline="") as f:
        f.writelines(lines)

    print(f"Updated {updated_l} RES_L and {updated_s} RES_S entries.")
    print(f"Line endings: {'CRLF' if eol == chr(13)+chr(10) else 'LF'}")


if __name__ == "__main__":
    main()
