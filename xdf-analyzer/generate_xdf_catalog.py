#!/usr/bin/env python3
"""
generate_xdf_catalog.py
Génère knowledge/xdf_catalog.json — stubs NON pour tous les params XDF
dont la catégorie n'est pas couverte par e85_knowledge.json.
"""
import json
import os
import sys
from collections import defaultdict

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, SCRIPT_DIR)

from xdf_parser import XDFParser

XDF_FILE       = os.path.join(SCRIPT_DIR, "data", "Full", "BMW_Siemens_MSV70_9PPL921S_2560K.xdf")
E85_KNOWLEDGE  = os.path.join(SCRIPT_DIR, "knowledge", "e85_knowledge.json")
OUTPUT_FILE    = os.path.join(SCRIPT_DIR, "knowledge", "xdf_catalog.json")

# 1. Catégories déjà couvertes par e85_knowledge.json
with open(E85_KNOWLEDGE, encoding="utf-8") as f:
    e85 = json.load(f)

covered = {sub["xdf_category"] for sub in e85.get("subcategories", []) if sub.get("xdf_category")}
print(f"  {len(covered)} catégories couvertes par e85_knowledge.json — ignorées")

# 2. Parse XDF
parser = XDFParser(XDF_FILE)
all_params = parser.get_all_params()
print(f"  {len(all_params)} paramètres dans le XDF full")

# 3. Grouper les params non couverts par catégorie
catalog_cats = defaultdict(list)
for p in all_params:
    cat = p.get("category", "")
    if cat and cat not in covered:
        catalog_cats[cat].append(p)

print(f"  {len(catalog_cats)} categories non couvertes -> xdf_catalog.json")

# 4. Construire les subcategories
subcategories = []
for cat in sorted(catalog_cats.keys()):
    param_entries = {}
    for p in catalog_cats[cat]:
        title = p.get("title", "")
        desc  = p.get("desc", "") or ""
        ptype = p.get("type", "")
        kind  = p.get("kind", "")
        param_entries[title] = desc if desc else f"{kind} — {ptype}"
    subcategories.append({
        "xdf_category": cat,
        "label": cat,
        "role": "",
        "parameters": param_entries,
    })

catalog = {
    "id": "XDF_CATALOG",
    "label": "XDF Catalog — Paramètres non E85 (auto-généré depuis BMW_Siemens_MSV70_9PPL921S_2560K.xdf)",
    "description": (
        "Auto-généré par generate_xdf_catalog.py. "
        "Contient les stubs de toutes les catégories XDF non couvertes par e85_knowledge.json. "
        "Tous les paramètres sont marqués modification_e85=NON."
    ),
    "subcategories": subcategories
}

os.makedirs(os.path.dirname(OUTPUT_FILE), exist_ok=True)
with open(OUTPUT_FILE, "w", encoding="utf-8") as f:
    json.dump(catalog, f, ensure_ascii=False, indent=2)

n_params = sum(len(s["parameters"]) for s in subcategories)
print(f"Generated {OUTPUT_FILE}")
print(f"  {len(subcategories)} catégories, {n_params} paramètres")
