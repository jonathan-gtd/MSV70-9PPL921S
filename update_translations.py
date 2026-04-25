#!/usr/bin/env python3
"""
Sync jobs_all_en_EN.txt from translations_table.json.
For each RESULTNAME in the JSON, updates matching RES_L and RES_S lines.
"""

import json
import os

TRANSLATOR_JSON = os.path.join(
    os.path.dirname(__file__),
    "TestO Datalogger", "translator", "translations_table.json"
)
TRANSLATIONS_TXT = os.path.join(
    os.path.dirname(__file__),
    "TestO Datalogger", "translations", "jobs_all_en_EN.txt"
)


def load_json(path):
    with open(path, encoding="utf-8") as f:
        return json.load(f)


def load_txt(path):
    with open(path, encoding="utf-8") as f:
        return f.readlines()


def save_txt(path, lines):
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        f.writelines(lines)


def main():
    table = load_json(TRANSLATOR_JSON)
    lines = load_txt(TRANSLATIONS_TXT)

    updated_l = 0
    updated_s = 0

    for i, line in enumerate(lines):
        parts = line.rstrip("\n").split("\t")
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
                lines[i] = f"{key}\t{typ}\t{new_val}\n"
                updated_l += 1

        elif typ == "RES_S" and entry.get("S"):
            new_val = entry["S"]
            if new_val != current_val:
                lines[i] = f"{key}\t{typ}\t{new_val}\n"
                updated_s += 1

    save_txt(TRANSLATIONS_TXT, lines)
    print(f"Updated {updated_l} RES_L and {updated_s} RES_S entries.")


if __name__ == "__main__":
    main()
