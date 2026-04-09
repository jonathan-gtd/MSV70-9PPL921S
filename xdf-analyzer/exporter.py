import os
import re
import csv
from datetime import datetime

class CSVExporter:
    def __init__(self, export_dir="exports"):
        self.export_dir = export_dir
        if not os.path.exists(self.export_dir):
            os.makedirs(self.export_dir)

    def get_timestamped_filename(self, base_name, extension="csv"):
        """Génère un nom de fichier horodaté."""
        now = datetime.now().strftime("%Y_%m_%d_%H_%M_%S")
        return f"{base_name}_{now}.{extension}"

    @staticmethod
    def _parse_param_doc(doc_text):
        """
        Extrait les champs structurés depuis le texte libre de param_doc.
        Retourne un dict avec les colonnes enrichies.
        """
        result = {
            "type_doc": "",          # TYPE extrait du préfixe (TABLE 3D, CONSTANTE, COURBE 2D…)
            "modification_e85": "",  # OUI / NON / À_VÉRIFIER
            "e85_action": "",        # Description de l'action E85 à réaliser
            "stock_raw": "",         # Valeur raw stock
            "stock_physical": "",    # Valeur physique stock (avec unité)
            "e60_raw": "",           # Raw cible E60
            "e80_raw": "",           # Raw cible E80
            "e100_raw": "",          # Raw cible E100
            "e85_factor": "",        # Multiplicateur (ex. ×1.35 à ×2.20) pour tables/courbes
            "dependencies": "",      # Paramètres à modifier ensemble
            "warning": "",           # OUI si ⚠️ présent
        }

        if not doc_text:
            return result

        # 1. Type de paramètre extrait du préfixe textuel
        type_match = re.match(
            r"^(TABLE\s+\w+|CONSTANTE|COURBE\s*\w*|CARTOGRAPHIE\s+\w+)\s*[—–\-]\s*",
            doc_text, re.IGNORECASE
        )
        if type_match:
            result["type_doc"] = type_match.group(1).strip().upper()

        # 2. Stock raw — patterns : "Stock raw=87", "raw=56567", "Stock : raw=28284"
        stock_raw_m = re.search(
            r"[Ss]tock\s*(?:[:\s])*\s*raw[=\s]+(\d+)", doc_text
        )
        if not stock_raw_m:
            stock_raw_m = re.search(r"\braw[=\s]+(\d+)", doc_text)
        if stock_raw_m:
            result["stock_raw"] = stock_raw_m.group(1)

        # 3. Valeur physique stock — "(0.3394 ms/mg)", "(17.25°C)" — après le raw stock
        stock_phys_m = re.search(r"raw[=\s]+\d+\s*\(([^)]+)\)", doc_text)
        if stock_phys_m:
            result["stock_physical"] = stock_phys_m.group(1).strip()

        # 4. Valeurs raw par titre éthanol — "E60→raw 74116" ou "E60→74116"
        for level in ("60", "80", "100"):
            e_m = re.search(rf"E{level}\s*[→>]\s*(?:raw\s+)?(\d+)", doc_text)
            if e_m:
                result[f"e{level}_raw"] = e_m.group(1)

        # 5. Facteurs multiplicateurs E85 — "×1.35", "×2.20", "x1.35"
        factors = re.findall(r"[×x](\d+\.\d+)", doc_text)
        if factors:
            result["e85_factor"] = " / ".join(dict.fromkeys(factors))  # dédoublonnage ordonné

        # 6. Modification requise
        if re.search(r"[Nn]e\s+pas\s+modifier", doc_text):
            result["modification_e85"] = "NON"
        elif (result["e100_raw"] or result["e85_factor"]
              or re.search(r"E\d{2,3}\s*[\s:→]", doc_text)):
            result["modification_e85"] = "OUI"
        elif doc_text.strip():
            result["modification_e85"] = "À_VÉRIFIER"

        # 7. Dépendances — "TOUJOURS mettre à jour conjointement avec …"
        dep_m = re.search(
            r"TOUJOURS\s+mettre\s+à\s+jour\s+conjointement\s+avec\s+([^.]+)", doc_text
        )
        if dep_m:
            result["dependencies"] = dep_m.group(1).strip()

        # 8. Warning
        result["warning"] = "OUI" if "⚠️" in doc_text else ""

        return result

    def export(self, data, base_name, meta=None):
        """Exporte les données enrichies en CSV avec un colonnage ordonné."""
        if not data:
            print("⚠️ Aucune donnée à exporter.")
            return

        filename = self.get_timestamped_filename(base_name)
        filepath = os.path.join(self.export_dir, filename)

        # Définition de l'ordre des colonnes et mapping pour une meilleure lisibilité
        HEADER_MAPPING = {
            "group_label": "Group",
            "subcat_label": "Subcategory",
            "category": "Category",
            "title": "Title",
            "role": "Role",
            "kind": "Kind",
            "tuning_impact": "Tuning Impact",
            "flex_fuel": "Flex Fuel",
            # --- Colonnes analyse éthanol ---
            "modification_e85": "Modification E85",
            "type_doc": "Type (doc)",
            "e85_action": "Action E85",
            "stock_raw": "Stock Raw",
            "stock_physical": "Stock Physical",
            "e60_raw": "E60 Raw",
            "e80_raw": "E80 Raw",
            "e100_raw": "E100 Raw",
            "e85_factor": "E85 Factor",
            "dependencies": "Dependencies",
            "warning": "Warning",
            "tags": "Tags",
            # --- Description textuelle ---
            "param_doc": "Documentation",
            "desc": "Description (Siemens)",
            # --- Technique XDF ---
            "type": "Structure Type",
            "bin_dtype": "Data Type",
            "bin_addr": "BIN Address",
            "xdf_addr": "XDF Address",
            "z_eq": "Z Equation",
            "z_unit": "Z Unit",
            "z_min": "Z Min",
            "z_max": "Z Max",
            "rows": "Rows",
            "cols": "Columns",
            "x_count": "X Count",
            "x_unit": "X Unit",
            "x_eq": "X Equation",
            "y_count": "Y Count",
            "y_unit": "Y Unit",
            "y_eq": "Y Equation"
        }

        CSV_FIELDS = list(HEADER_MAPPING.keys())

        # Enrichissement : parse param_doc pour extraire les colonnes structurées éthanol
        enriched_data = []
        for row in data:
            r = dict(row)
            parsed = self._parse_param_doc(r.get("param_doc", ""))
            # On injecte les champs parsés uniquement s'ils ne sont pas déjà présents
            for k, v in parsed.items():
                if not r.get(k):
                    r[k] = v
            enriched_data.append(r)

        # S'assurer que toutes les clés possibles de data sont présentes
        # au cas où une info dynamique a été ajoutée ailleurs
        all_keys = set()
        for row in enriched_data:
            all_keys.update(row.keys())

        final_fields = CSV_FIELDS + [k for k in all_keys if k not in CSV_FIELDS]

        with open(filepath, "w", newline="", encoding="utf-8-sig") as f:
            writer = csv.DictWriter(f, fieldnames=final_fields, extrasaction="ignore")

            # Écriture d'un header lisible personnalisé
            readable_header = {key: HEADER_MAPPING.get(key, key) for key in final_fields}
            writer.writerow(readable_header)

            # Tri par Groupe JSON, puis sous-catégorie, puis type
            sorted_data = sorted(enriched_data, key=lambda x: (
                x.get("group_label") or "ZZ",
                x.get("subcat_label") or "ZZ",
                x.get("kind", ""),
                x.get("title", "")
            ))
            writer.writerows(sorted_data)

        self._print_summary(enriched_data, meta or {}, filepath)

    @staticmethod
    def _print_summary(data, meta, filepath):
        from collections import Counter

        total = len(data)
        consts  = [r for r in data if r.get("kind") == "CONST"]
        tables  = [r for r in data if r.get("kind") == "TABLE"]
        maps3d  = [t for t in tables if "3D" in str(t.get("type", ""))]
        curves2d = [t for t in tables if "2D" in str(t.get("type", ""))]

        def pct(n, d): return f"{n/d*100:5.1f}%" if d else "   —  "

        # ── Équations ───────────────────────────────────────────────────────────
        def eq_x(lst):
            return sum(1 for r in lst if str(r.get("z_eq","")).strip() in ("X","x",""))
        def eq_ok(lst):
            return len(lst) - eq_x(lst)

        # ── Flex-fuel ────────────────────────────────────────────────────────────
        flex      = [r for r in data if r.get("flex_fuel") == "oui"]
        mod_count = Counter(r.get("modification_e85","") for r in flex)
        oui_list  = [r for r in flex if r.get("modification_e85") == "OUI"]
        type_oui  = Counter(r.get("type_doc","—") for r in oui_list)

        with_factor  = sum(1 for r in flex if str(r.get("e85_factor","")).strip())
        with_e100    = sum(1 for r in flex if str(r.get("e100_raw","")).strip())
        with_warning = sum(1 for r in flex if str(r.get("warning","")).strip() == "OUI")

        # ── Tags ─────────────────────────────────────────────────────────────
        tag_count = Counter()
        for r in data:
            for t in str(r.get("tags","")).split(","):
                t = t.strip()
                if t:
                    tag_count[t] += 1

        # ── Documentation ────────────────────────────────────────────────────────
        doc_total = sum(1 for r in data if str(r.get("param_doc","")).strip())
        doc_flex  = sum(1 for r in flex if str(r.get("param_doc","")).strip())

        W = 54  # largeur bloc

        def sep(char="─"): return char * W

        lines = [
            "",
            "=" * W,
            f"  Calculateur : {meta.get('deftitle','')}",
            f"  Auteur      : {meta.get('author','')}",
            f"  XDF Source  : {meta.get('xdf_name','')}",
            "=" * W,
            "",
            f"{'📊 TOTAL PARAMÈTRES':.<{W-8}} {total}",
            sep(),
            f"  Scalaires  (CONST)       : {len(consts):>5}  ({pct(len(consts),total)})",
            f"  Courbes    2D            : {len(curves2d):>5}  ({pct(len(curves2d),total)})",
            f"  Carto      3D            : {len(maps3d):>5}  ({pct(len(maps3d),total)})",
            "",
            f"{'🔢 ÉQUATIONS DE CONVERSION  (eq ≠ X  /  eq = X)':.<{W}}",
            sep(),
            f"  Courbes 2D   eq ≠ X : {eq_ok(curves2d):>5}  /  eq = X : {eq_x(curves2d):>5}  ({pct(eq_ok(curves2d),len(curves2d))} résolues)",
            f"  Cartos  3D   eq ≠ X : {eq_ok(maps3d):>5}  /  eq = X : {eq_x(maps3d):>5}  ({pct(eq_ok(maps3d),len(maps3d))} résolues)",
            f"  Scalaires    eq ≠ X : {eq_ok(consts):>5}  /  eq = X : {eq_x(consts):>5}  ({pct(eq_ok(consts),len(consts))} résolues)",
            "",
            f"{'⛽ FLEX-FUEL E85':.<{W-14}} {len(flex)} / {total} ({pct(len(flex),total)})",
            sep(),
            f"  Modification OUI          : {mod_count.get('OUI',0):>5}  ({pct(mod_count.get('OUI',0),len(flex))})",
            f"  Modification NON          : {mod_count.get('NON',0):>5}  ({pct(mod_count.get('NON',0),len(flex))})",
            f"  Modification À_VÉRIFIER   : {mod_count.get('À_VÉRIFIER',0):>5}  ({pct(mod_count.get('À_VÉRIFIER',0),len(flex))})",
            "",
            f"  ⚡ Paramètres OUI — par type doc :",
        ]
        for ttype, cnt in sorted(type_oui.items(), key=lambda x: -x[1]):
            lines.append(f"     {ttype:<26} : {cnt:>3}")

        lines += [
            "",
            f"  📐 Calibration E85 :",
            f"     Avec facteur multiplicateur : {with_factor:>3} params",
            f"     Avec raw E100 défini        : {with_e100:>3} params",
            f"     Avec warning ⚠️             : {with_warning:>3} params",
            "",
            f"{'📝 DOCUMENTATION':.<{W}}",
            sep(),
            f"  Documentés (global)       : {doc_total:>5} / {total:<5} ({pct(doc_total,total)})",
            f"  Flex-fuel documentés      : {doc_flex:>5} / {len(flex):<5} ({pct(doc_flex,len(flex))})",
            f"  Non documentés flex-fuel  : {len(flex)-doc_flex:>5} / {len(flex):<5} ({pct(len(flex)-doc_flex,len(flex))})",
            "",
            f"{'🏷️  TAGS':.<{W}}",
            sep(),
        ] + [
            f"  {tag:<28} : {cnt:>5} params"
            for tag, cnt in sorted(tag_count.items(), key=lambda x: -x[1])
        ] + [
            "",
            f"✅  {filepath}",
            "",
        ]

        print("\n".join(lines))
