"""MCP server exposing XDF/BIN tools for the MSV70 ECU."""
import os
import sys

# Ensure imports resolve correctly regardless of working directory
_HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, _HERE)

from mcp.server.fastmcp import FastMCP
from xdf_parser import XDFParser
from doc_manager import DocManager
from bin_reader import read_param_values, compare_params

# ── Paths ──────────────────────────────────────────────────────────────────────

DATA_FULL    = os.path.join(_HERE, "data", "Full")
DATA_PARTIAL = os.path.join(_HERE, "data", "Partial")
DOCS_PATH    = os.path.join(_HERE, "knowledge")

XDF_FULL    = os.path.join(DATA_FULL,    "BMW_Siemens_MSV70_9PPL921S_2560K.xdf")
XDF_PARTIAL = os.path.join(DATA_PARTIAL, "BMW_Siemens_MSV70_9PPL921S_128K.xdf")

BIN_FILES = {
    "stock":         os.path.join(DATA_FULL,    "VB67774_921S_Full.bin"),
    "stock_e85":     os.path.join(DATA_FULL,    "VB67774_921S_Full_E85.bin"),
    "partial":       os.path.join(DATA_PARTIAL, "VB67774_921S_Partial.bin"),
    "partial_e85":   os.path.join(DATA_PARTIAL, "VB67774_921S_Partial_E85.bin"),
    "s7_partial":    os.path.join(DATA_PARTIAL, "S7581293_Partial.bin"),
}

# ── Lazy-loaded cache ──────────────────────────────────────────────────────────

_cache: dict = {}

def _get_params(full: bool = True) -> list[dict]:
    key = "full" if full else "partial"
    if key not in _cache:
        xdf_path = XDF_FULL if full else XDF_PARTIAL
        parser = XDFParser(xdf_path)
        params = parser.get_all_params()
        doc_mgr = DocManager(DOCS_PATH)
        for p in params:
            doc_mgr.enrich_param(p)
        _cache[key] = params
    return _cache[key]

def _find_param(title: str, full: bool = True) -> dict | None:
    title_lower = title.lower()
    for p in _get_params(full):
        if p["title"].lower() == title_lower:
            return p
    return None

def _resolve_bin(bin_key: str) -> str | None:
    if bin_key in BIN_FILES:
        return BIN_FILES[bin_key]
    if os.path.isfile(bin_key):
        return bin_key
    return None

# ── MCP Server ────────────────────────────────────────────────────────────────

mcp = FastMCP("MSV70 XDF/BIN Analyzer")


@mcp.tool()
def list_params(
    search: str = "",
    category: str = "",
    kind: str = "",
    flex_fuel_only: bool = False,
    full_rom: bool = True,
    limit: int = 100,
) -> dict:
    """
    List ECU parameters from the XDF definition.

    Args:
        search:        Keyword filter on title or description (case-insensitive).
        category:      Filter by XDF category name (partial match).
        kind:          "TABLE" or "CONST" to filter by type.
        flex_fuel_only: Return only E85-relevant parameters.
        full_rom:      True = Full 2560K ROM, False = Partial 128K ROM.
        limit:         Max number of results (default 100).
    """
    params = _get_params(full_rom)
    results = []

    for p in params:
        if search and search.lower() not in p["title"].lower() and search.lower() not in p.get("desc", "").lower():
            continue
        if category and category.lower() not in p.get("category", "").lower():
            continue
        if kind and p.get("kind", "") != kind.upper():
            continue
        if flex_fuel_only and p.get("flex_fuel", "") != "oui":
            continue
        results.append({
            "title":       p["title"],
            "category":    p["category"],
            "kind":        p["kind"],
            "type":        p["type"],
            "bin_addr":    p["bin_addr"],
            "z_unit":      p["z_unit"],
            "flex_fuel":   p.get("flex_fuel", ""),
            "modification_e85": p.get("modification_e85", ""),
        })
        if len(results) >= limit:
            break

    return {"count": len(results), "params": results}


@mcp.tool()
def get_param_info(title: str, full_rom: bool = True) -> dict:
    """
    Get full definition of a parameter (XDF metadata + documentation).

    Args:
        title:    Exact parameter title as defined in the XDF.
        full_rom: True = Full 2560K ROM, False = Partial 128K ROM.
    """
    p = _find_param(title, full_rom)
    if p is None:
        return {"error": f"Parameter '{title}' not found"}
    return p


@mcp.tool()
def read_bin_value(
    title: str,
    bin_key: str = "stock",
    full_rom: bool = True,
) -> dict:
    """
    Read the decoded value(s) of a parameter from a BIN file.

    Args:
        title:    Exact parameter title as defined in the XDF.
        bin_key:  One of: stock, stock_e85, partial, partial_e85, s7_partial.
                  Or an absolute path to any .bin file.
        full_rom: True = Full 2560K ROM XDF, False = Partial 128K XDF.
    """
    p = _find_param(title, full_rom)
    if p is None:
        return {"error": f"Parameter '{title}' not found"}

    bin_path = _resolve_bin(bin_key)
    if bin_path is None:
        return {"error": f"BIN file not found: '{bin_key}'. Valid keys: {list(BIN_FILES.keys())}"}

    result = read_param_values(bin_path, p)
    result["title"]    = title
    result["bin_file"] = os.path.basename(bin_path)
    return result


@mcp.tool()
def compare_bins(
    title: str,
    bin_key1: str = "stock",
    bin_key2: str = "stock_e85",
    full_rom: bool = True,
) -> dict:
    """
    Compare a parameter's values between two BIN files and report differences.

    Args:
        title:    Exact parameter title as defined in the XDF.
        bin_key1: First BIN key (or absolute path).
        bin_key2: Second BIN key (or absolute path).
        full_rom: True = Full 2560K ROM XDF, False = Partial 128K XDF.
    """
    p = _find_param(title, full_rom)
    if p is None:
        return {"error": f"Parameter '{title}' not found"}

    path1 = _resolve_bin(bin_key1)
    path2 = _resolve_bin(bin_key2)
    if path1 is None:
        return {"error": f"BIN not found: '{bin_key1}'"}
    if path2 is None:
        return {"error": f"BIN not found: '{bin_key2}'"}

    result = compare_params(path1, path2, p)
    result["title"] = title
    result["bin1"]  = os.path.basename(path1)
    result["bin2"]  = os.path.basename(path2)
    return result


@mcp.tool()
def list_bin_files() -> dict:
    """List all available BIN files with their size and path."""
    files = {}
    for key, path in BIN_FILES.items():
        if os.path.isfile(path):
            size = os.path.getsize(path)
            files[key] = {"path": path, "size_kb": round(size / 1024, 1), "exists": True}
        else:
            files[key] = {"path": path, "exists": False}
    return files


@mcp.tool()
def diff_all_changed_params(
    bin_key1: str = "stock",
    bin_key2: str = "stock_e85",
    full_rom: bool = True,
    flex_fuel_only: bool = True,
) -> dict:
    """
    Scan all parameters and return those that differ between two BIN files.

    Args:
        bin_key1:      First BIN key.
        bin_key2:      Second BIN key.
        full_rom:      True = Full ROM XDF, False = Partial ROM XDF.
        flex_fuel_only: Restrict to E85-relevant params only (faster).
    """
    path1 = _resolve_bin(bin_key1)
    path2 = _resolve_bin(bin_key2)
    if path1 is None:
        return {"error": f"BIN not found: '{bin_key1}'"}
    if path2 is None:
        return {"error": f"BIN not found: '{bin_key2}'"}

    params = _get_params(full_rom)
    changed = []
    errors  = []

    for p in params:
        if flex_fuel_only and p.get("flex_fuel", "") != "oui":
            continue
        result = compare_params(path1, path2, p)
        if "error" in result:
            errors.append({"title": p["title"], "error": result["error"]})
        elif result["changed"]:
            changed.append({
                "title":       p["title"],
                "category":    p.get("category", ""),
                "type":        p.get("type", ""),
                "diff_count":  result["diff_count"],
                "total_cells": result["total_cells"],
                "z_unit":      result["z_unit"],
            })

    return {
        "bin1":         os.path.basename(path1),
        "bin2":         os.path.basename(path2),
        "changed_count": len(changed),
        "error_count":  len(errors),
        "changed":      changed,
        "errors":       errors,
    }


if __name__ == "__main__":
    mcp.run()
