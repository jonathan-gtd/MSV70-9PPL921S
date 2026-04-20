import xml.etree.ElementTree as ET
import os

class XDFParser:
    def __init__(self, xdf_filename):
        # Accept absolute paths as-is; otherwise resolve relative to data/
        if os.path.isabs(xdf_filename):
            self.xdf_path = xdf_filename
        else:
            self.xdf_path = os.path.join("data", xdf_filename)
        if not os.path.exists(self.xdf_path):
            raise FileNotFoundError(f"Fichier XDF introuvable : {self.xdf_path}")
            
        self.tree = ET.parse(self.xdf_path)
        self.root = self.tree.getroot()
        self.base_offset = self._get_base_offset()
        self.categories = self._parse_categories()

        header = self.root.find("XDFHEADER")
        self.deftitle = header.find("deftitle").text if header is not None and header.find("deftitle") is not None else "Inconnu"
        self.author = header.find("author").text if header is not None and header.find("author") is not None else "Inconnu"

    def _get_base_offset(self):
        header = self.root.find("XDFHEADER")
        bo_el = header.find("BASEOFFSET")
        return int(bo_el.attrib.get("offset", "0"), 0) if bo_el is not None else 0

    def _parse_categories(self):
        cats = {}
        for cat in self.root.findall(".//CATEGORY"):
            idx = cat.attrib.get("index", "")
            name = cat.attrib.get("name", "").split(":")[0].strip()
            cats[f"0x{int(idx, 0):X}".upper()] = name
        return cats

    def get_all_params(self):
        params = []
        # Tables
        for t in self.root.findall(".//XDFTABLE"):
            params.append(self._parse_item(t, "TABLE"))
        # Constantes
        for c in self.root.findall(".//XDFCONSTANT"):
            params.append(self._parse_item(c, "CONST"))
        return params

    def _parse_item(self, el, kind):
        title = el.find("title").text if el.find("title") is not None else "N/A"
        desc = el.find("description").text if el.find("description") is not None else ""
        
        # Gestion Catégorie
        cat_mem = el.find("CATEGORYMEM")
        cat_idx = f"0x{int(cat_mem.attrib.get('category','0'),0):X}".upper() if cat_mem is not None else ""
        category_name = self.categories.get(cat_idx, f"[{cat_idx}]" if cat_idx else "")

        # Default properties
        xdf_addr = "?"
        z_bits = "?"
        z_min = ""
        z_max = ""
        rows, cols = 1, 1
        z_eq = "X"
        z_unit = ""

        # Données Z (Valeurs)
        z_axis = el.find("XDFAXIS[@id='z']") if kind == "TABLE" else el
        
        if z_axis is not None:
            emb = z_axis.find("EMBEDDEDDATA")
            if emb is not None:
                xdf_addr = emb.attrib.get("mmedaddress", "?")
                if kind == "TABLE":
                    rows = int(emb.attrib.get("mmedrowcount", "1"))
                    cols = int(emb.attrib.get("mmedcolcount", "1"))
                z_bits = emb.attrib.get("mmedelementsizebits", "?")
            z_unit = z_axis.find("units").text if z_axis.find("units") is not None else ""
            if kind == "TABLE":
                z_n = z_axis.find("min")
                z_min = z_n.text if z_n is not None and z_n.text else ""
                z_x = z_axis.find("max")
                z_max = z_x.text if z_x is not None and z_x.text else ""
            math = z_axis.find("MATH")
            z_eq = math.attrib.get("equation", "X") if math is not None else "X"

        def _get_axis_info(axis_id):
            if kind == "CONST": return 1, "", "X"
            ax = el.find(f"XDFAXIS[@id='{axis_id}']")
            if ax is None: return 1, "", "X"
            ic_el = ax.find("indexcount")
            ic = int(ic_el.text) if ic_el is not None and ic_el.text else 1
            unit = ax.find("units").text if ax.find("units") is not None else ""
            math = ax.find("MATH")
            eq = math.attrib.get("equation", "X") if math is not None else "X"
            return ic, unit, eq

        x_count, x_unit, x_eq = _get_axis_info("x")
        y_count, y_unit, y_eq = _get_axis_info("y")

        def _bin_addr(addr_str):
            if addr_str == "?": return "?"
            try:
                return f"0x{int(addr_str, 0) + self.base_offset:05X}" 
            except ValueError:
                return "?"

        def _bits_to_type(b):
            return {"8": "uint8", "16": "uint16", "32": "uint32"}.get(b, f"{b}b")

        # Type de structure
        if kind == "CONST":
            p_type = "Scalar"
        else:
            if rows > 1 and cols > 1: p_type = f"3D Map ({rows}x{cols})"
            elif cols > 1 or rows > 1: p_type = f"2D Curve ({max(rows, cols)} pts)"
            else: p_type = "Scalar"

        return {
            "kind": kind,
            "title": title,
            "desc": desc,
            "category": category_name,
            "type": p_type,
            "xdf_addr": xdf_addr,
            "bin_addr": _bin_addr(xdf_addr),
            "bin_dtype": _bits_to_type(z_bits),
            "z_eq": z_eq,
            "z_unit": z_unit,
            "z_min": z_min,
            "z_max": z_max,
            "rows": rows,
            "cols": cols,
            "x_count": x_count,
            "x_unit": x_unit,
            "x_eq": x_eq,
            "y_count": y_count,
            "y_unit": y_unit,
            "y_eq": y_eq,
            # Placeholder for doc infos
            "group_label": "",
            "subcat_label": "",
            "role": "",
            "tuning_impact": "",
            "flex_fuel": "",
            "param_doc": ""
        }