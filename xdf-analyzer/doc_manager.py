import os
import json

class DocManager:
    def __init__(self, docs_path="docs"):
        self.docs_path = docs_path
        self.doc_map = {}  # Clé: xdf_category
        self._load_all_docs()

    def _load_all_docs(self):
        if not os.path.exists(self.docs_path):
            os.makedirs(self.docs_path)
            return

        # Walk recursively: charge docs/ethanol/ et docs/base/ (et docs/ directement si besoin)
        for root, dirs, files in os.walk(self.docs_path):
            # Trier pour un chargement déterministe (ethanol/ en priorité sur base/)
            dirs.sort()
            for filename in sorted(files):
                if not filename.endswith(".json"):
                    continue
                filepath = os.path.join(root, filename)
                with open(filepath, 'r', encoding='utf-8') as f:
                    data = json.load(f)
                for sub in data.get("subcategories", []):
                    cat_id = sub.get("xdf_category")
                    self.doc_map[cat_id] = {
                        "group_label": data.get("label"),
                        "sub_label": sub.get("label"),
                        "role": sub.get("role"),
                        "impact": sub.get("tuning_impact", ""),
                        "flex_fuel": "oui" if sub.get("flex_fuel_sensitive") else "non",
                        "params": sub.get("parameters", {}),
                        "undocumented_default": sub.get("undocumented_params_default", "À_VÉRIFIER"),
                        "sub_tags": sub.get("tags", []),
                    }

    # Champs structurés lus directement depuis les entrées JSON de type dict.
    # Ils sont copiés tels quels dans le param dict (sauf bool/list qui sont normalisés).
    _STRUCTURED_FIELDS = (
        "type_doc", "modification_e85", "e85_action",
        "stock_raw", "stock_physical",
        "e60_raw", "e80_raw", "e100_raw",
        "e85_factor", "warning", "dependencies", "tags",
    )

    def enrich_param(self, p):
        meta = self.doc_map.get(p["category"], {})
        p["group_label"] = meta.get("group_label", "")
        p["subcat_label"] = meta.get("sub_label", "")
        p["role"] = meta.get("role", "")
        p["tuning_impact"] = meta.get("impact", "")
        p["flex_fuel"] = meta.get("flex_fuel", "")

        # Résolution de l'entrée de doc : cherche le titre exact,
        # puis sans index [n], puis avec wildcard [0-1] ou [0-5].
        clean_title = p["title"].split("[")[0].strip()
        params_doc = meta.get("params", {})

        param_entry = (
            params_doc.get(p["title"])
            or params_doc.get(clean_title)
            or params_doc.get(f"{clean_title}[0-5]")
            or params_doc.get(f"{clean_title}[0-1]")
            or ""
        )

        if isinstance(param_entry, dict):
            # ── Nouveau format structuré ──────────────────────────────────────
            p["param_doc"] = param_entry.get("doc", "")
            for field in self._STRUCTURED_FIELDS:
                val = param_entry.get(field)
                if val is None:
                    continue
                if field == "tags":
                    # Merge subcategory tags + param-level tags, deduplicated
                    sub_tags = meta.get("sub_tags", [])
                    merged = list(dict.fromkeys(sub_tags + (val if isinstance(val, list) else [val])))
                    p["tags"] = ",".join(merged)
                elif isinstance(val, list):
                    p[field] = " / ".join(str(x) for x in val)
                elif isinstance(val, bool):
                    p[field] = "OUI" if val else ""
                else:
                    p[field] = val  # int, float ou str — laissé tel quel
            # If no param-level tags but subcategory has tags, apply them
            if "tags" not in p or not p.get("tags"):
                sub_tags = meta.get("sub_tags", [])
                if sub_tags:
                    p["tags"] = ",".join(sub_tags)
        else:
            # ── Format legacy (string brut) ───────────────────────────────────
            p["param_doc"] = param_entry or ""
            # Paramètre dans une sous-catégorie flex_fuel sans entrée doc spécifique :
            # utilise le défaut de la sous-catégorie, ou "À_VÉRIFIER" si absent.
            if not param_entry and meta.get("flex_fuel") == "oui":
                default = meta.get("undocumented_default", "À_VÉRIFIER")
                p.setdefault("modification_e85", default)
            # Appliquer les tags de la sous-catégorie
            sub_tags = meta.get("sub_tags", [])
            if sub_tags and not p.get("tags"):
                p["tags"] = ",".join(sub_tags)