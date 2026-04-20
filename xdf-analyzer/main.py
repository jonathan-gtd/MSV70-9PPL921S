import os
from xdf_parser import XDFParser
from doc_manager import DocManager
from exporter import CSVExporter

# --- CONFIGURATION ---
XDF_NAME = "BMW_Siemens_MSV70_9PPL921S_2560K.xdf"
EXPORT_DIR = "exports"

def main():
    # 1. Extraction technique via le Parser (dossier data/)
    try:
        parser = XDFParser(XDF_NAME)
        all_params = parser.get_all_params()
    except FileNotFoundError as e:
        print(f"❌ {e}")
        return

    # 2. Enrichissement via le DocManager (dossier knowledge/ — charge les JSON récursivement)
    doc_manager = DocManager("knowledge")
    for p in all_params:
        doc_manager.enrich_param(p)

    # 3. Exportation finale (le summary est imprimé dans l'exporter)
    meta = {
        "deftitle": parser.deftitle,
        "author":   parser.author,
        "xdf_name": XDF_NAME,
    }
    exporter = CSVExporter(EXPORT_DIR)
    exporter.export(all_params, "msv70_full_prefixed", meta=meta)

if __name__ == "__main__":
    main()