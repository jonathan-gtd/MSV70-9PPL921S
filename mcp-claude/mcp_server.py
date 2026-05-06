"""MCP server exposing XDF/BIN/0da tools for the MSV70 ECU.

All tools accept explicit file paths — no hardcoded project paths.
The only optional shorthand is XDF auto-detection based on file size when
full_rom is omitted, which the caller can override at any time.
"""
import os
import sys
import tempfile

_HERE     = os.path.dirname(os.path.abspath(__file__))
_ANALYZER = os.path.normpath(os.path.join(_HERE, "..", "xdf-analyzer"))
sys.path.insert(0, _ANALYZER)

from mcp.server.fastmcp import FastMCP
from xdf_parser import XDFParser
from bin_reader import read_param_values, compare_params
from da_converter import extract_calibration

mcp = FastMCP("MSV70 XDF/BIN Analyzer")

# ── Parameter cache (keyed by xdf_path) ──────────────────────────────────────

_param_cache: dict[str, list[dict]] = {}

def _load_params(xdf_path: str) -> list[dict]:
    if xdf_path not in _param_cache:
        parser = XDFParser(xdf_path)
        _param_cache[xdf_path] = parser.get_all_params()
    return _param_cache[xdf_path]

def _find_param(title: str, xdf_path: str) -> dict | None:
    tl = title.lower()
    for p in _load_params(xdf_path):
        if p["title"].lower() == tl:
            return p
    return None


# ── Tools ─────────────────────────────────────────────────────────────────────

@mcp.tool()
def list_params(
    xdf_path: str,
    search: str = "",
    category: str = "",
    kind: str = "",
    limit: int = 100,
) -> dict:
    """
    List ECU parameters from an XDF file.

    Args:
        xdf_path:  Absolute path to the XDF file.
        search:    Keyword filter on title (case-insensitive).
        category:  Filter by category name (partial match).
        kind:      "TABLE" or "CONST".
        limit:     Max results (default 100).
    """
    params = _load_params(xdf_path)
    results = []
    for p in params:
        if search and search.lower() not in p["title"].lower():
            continue
        if category and category.lower() not in p.get("category", "").lower():
            continue
        if kind and p.get("kind", "") != kind.upper():
            continue
        results.append({
            "title":    p["title"],
            "category": p.get("category", ""),
            "kind":     p.get("kind", ""),
            "bin_addr": p.get("bin_addr", ""),
            "z_unit":   p.get("z_unit", ""),
            "rows":     p.get("rows", 1),
            "cols":     p.get("cols", 1),
        })
        if len(results) >= limit:
            break
    return {"xdf": os.path.basename(xdf_path), "count": len(results), "params": results}


@mcp.tool()
def get_param_info(xdf_path: str, title: str) -> dict:
    """
    Get full XDF definition of a parameter.

    Args:
        xdf_path: Absolute path to the XDF file.
        title:    Exact parameter title.
    """
    p = _find_param(title, xdf_path)
    if p is None:
        return {"error": f"Parameter '{title}' not found in {os.path.basename(xdf_path)}"}
    return p


@mcp.tool()
def read_param(xdf_path: str, bin_path: str, title: str) -> dict:
    """
    Read the decoded value(s) of a parameter from a BIN file.

    Args:
        xdf_path: Absolute path to the XDF file.
        bin_path: Absolute path to the BIN file.
        title:    Exact parameter title.
    """
    p = _find_param(title, xdf_path)
    if p is None:
        return {"error": f"Parameter '{title}' not found"}
    if not os.path.isfile(bin_path):
        return {"error": f"BIN not found: {bin_path}"}
    result = read_param_values(bin_path, p)
    result["title"]    = title
    result["bin_file"] = os.path.basename(bin_path)
    return result


@mcp.tool()
def compare_param(xdf_path: str, bin_path1: str, bin_path2: str, title: str) -> dict:
    """
    Compare a single parameter between two BIN files.

    Args:
        xdf_path:  Absolute path to the XDF file.
        bin_path1: Absolute path to the first BIN (reference).
        bin_path2: Absolute path to the second BIN.
        title:     Exact parameter title.
    """
    p = _find_param(title, xdf_path)
    if p is None:
        return {"error": f"Parameter '{title}' not found"}
    for path in (bin_path1, bin_path2):
        if not os.path.isfile(path):
            return {"error": f"BIN not found: {path}"}
    result = compare_params(bin_path1, bin_path2, p)
    result["title"] = title
    result["bin1"]  = os.path.basename(bin_path1)
    result["bin2"]  = os.path.basename(bin_path2)
    return result


@mcp.tool()
def diff_bins(
    xdf_path: str,
    bin_path1: str,
    bin_path2: str,
    search: str = "",
    category: str = "",
    changed_only: bool = True,
) -> dict:
    """
    Compare ALL parameters between two BIN files and report differences.

    Args:
        xdf_path:     Absolute path to the XDF file.
        bin_path1:    Reference BIN (absolute path).
        bin_path2:    Comparison BIN (absolute path).
        search:       Optional keyword filter on param title.
        category:     Optional category filter.
        changed_only: Return only changed params (default True).
    """
    for path in (bin_path1, bin_path2):
        if not os.path.isfile(path):
            return {"error": f"BIN not found: {path}"}

    params = _load_params(xdf_path)
    changed, same, errors = [], [], []

    for p in params:
        if search and search.lower() not in p["title"].lower():
            continue
        if category and category.lower() not in p.get("category", "").lower():
            continue
        result = compare_params(bin_path1, bin_path2, p)
        if "error" in result:
            errors.append({"title": p["title"], "error": result["error"]})
        elif result["changed"]:
            changed.append({
                "title":       p["title"],
                "category":    p.get("category", ""),
                "kind":        p.get("kind", ""),
                "rows":        p.get("rows", 1),
                "cols":        p.get("cols", 1),
                "z_unit":      result["z_unit"],
                "diff_count":  result["diff_count"],
                "total_cells": result["total_cells"],
                "diffs":       result["diffs"],
            })
        else:
            if not changed_only:
                same.append(p["title"])

    out = {
        "xdf":           os.path.basename(xdf_path),
        "bin1":          os.path.basename(bin_path1),
        "bin2":          os.path.basename(bin_path2),
        "changed_count": len(changed),
        "error_count":   len(errors),
        "changed":       changed,
        "errors":        errors,
    }
    if not changed_only:
        out["same_count"] = len(same)
    return out


@mcp.tool()
def convert_0da(
    da_path: str,
    output_path: str = "",
    base_addr: int = 0x840000,
    size_kb: int = 128,
) -> dict:
    """
    Convert a WinKFP .0da Intel HEX calibration file to a flat binary.

    For MSV70: calibration at ECU address 0x840000, 128 KB.
    Output binary can be flashed directly as a partial calibration.

    Args:
        da_path:     Absolute path to the .0da file.
        output_path: Where to write the .bin (default: same name, .bin extension).
        base_addr:   ECU calibration base address (default 0x840000).
        size_kb:     Output size in KB (default 128).
    """
    if not os.path.isfile(da_path):
        return {"error": f"0da not found: {da_path}"}

    size = size_kb * 1024
    cal_bytes, bytes_written = extract_calibration(da_path, base_addr, size)

    if not output_path:
        output_path = os.path.splitext(da_path)[0] + ".bin"

    with open(output_path, "wb") as f:
        f.write(cal_bytes)

    return {
        "da_file":       os.path.basename(da_path),
        "output_bin":    output_path,
        "size_kb":       size_kb,
        "base_addr":     hex(base_addr),
        "bytes_written": bytes_written,
        "ff_bytes":      cal_bytes.count(0xFF),
        "programmed_kb": round((size - cal_bytes.count(0xFF)) / 1024, 1),
    }


@mcp.tool()
def diff_0da_vs_bin(
    xdf_path: str,
    da_path: str,
    bin_path: str,
    base_addr: int = 0x840000,
    size_kb: int = 128,
    search: str = "",
    category: str = "",
) -> dict:
    """
    Convert a .0da on-the-fly and compare all parameters against a reference BIN.

    Args:
        xdf_path:  Absolute path to the XDF file.
        da_path:   Absolute path to the .0da file.
        bin_path:  Absolute path to the reference BIN.
        base_addr: ECU calibration base address (default 0x840000).
        size_kb:   Calibration region size in KB (default 128).
        search:    Optional keyword filter on param title.
        category:  Optional category filter.
    """
    for label, path in [("0da", da_path), ("BIN", bin_path)]:
        if not os.path.isfile(path):
            return {"error": f"{label} not found: {path}"}

    size = size_kb * 1024
    cal_bytes, bytes_written = extract_calibration(da_path, base_addr, size)

    tmp = tempfile.NamedTemporaryFile(delete=False, suffix=".bin")
    try:
        tmp.write(cal_bytes)
        tmp.close()

        params = _load_params(xdf_path)
        changed, errors = [], []

        for p in params:
            if search and search.lower() not in p["title"].lower():
                continue
            if category and category.lower() not in p.get("category", "").lower():
                continue
            result = compare_params(bin_path, tmp.name, p)
            if "error" in result:
                errors.append({"title": p["title"], "error": result["error"]})
            elif result["changed"]:
                changed.append({
                    "title":       p["title"],
                    "category":    p.get("category", ""),
                    "kind":        p.get("kind", ""),
                    "rows":        p.get("rows", 1),
                    "cols":        p.get("cols", 1),
                    "z_unit":      result["z_unit"],
                    "diff_count":  result["diff_count"],
                    "total_cells": result["total_cells"],
                    "diffs":       result["diffs"],
                })
    finally:
        os.unlink(tmp.name)

    return {
        "xdf":           os.path.basename(xdf_path),
        "da_file":       os.path.basename(da_path),
        "bin_ref":       os.path.basename(bin_path),
        "bytes_written": bytes_written,
        "changed_count": len(changed),
        "error_count":   len(errors),
        "changed":       changed,
        "errors":        errors,
    }


@mcp.tool()
def patch_full_rom(
    full_bin_path: str,
    cal_bin_path: str,
    output_path: str = "",
    cal_offset: int = 0x40000,
) -> dict:
    """
    Inject a calibration (partial) BIN into a full 2MB ROM BIN.

    Replaces the calibration region in the full ROM with the contents of
    cal_bin_path, producing a new full ROM ready to send to the flasher.

    For MSV70: calibration lives at offset 0x40000 in the full ROM
    (ECU address 0x840000, ROM base 0x800000).

    Args:
        full_bin_path: Absolute path to the 2MB full ROM BIN.
        cal_bin_path:  Absolute path to the partial calibration BIN.
        output_path:   Where to write the patched full ROM (default: cal_bin stem + '_full.bin').
        cal_offset:    Offset of the calibration region in the full ROM (default 0x40000).
    """
    for label, path in [("full ROM", full_bin_path), ("calibration", cal_bin_path)]:
        if not os.path.isfile(path):
            return {"error": f"{label} not found: {path}"}

    full = bytearray(open(full_bin_path, "rb").read())
    cal  = open(cal_bin_path, "rb").read()

    if cal_offset + len(cal) > len(full):
        return {"error": f"Calibration ({len(cal)} B) at offset 0x{cal_offset:X} exceeds full ROM size ({len(full)} B)"}

    full[cal_offset:cal_offset + len(cal)] = cal

    if not output_path:
        stem = os.path.splitext(cal_bin_path)[0]
        output_path = stem + "_full.bin"

    with open(output_path, "wb") as f:
        f.write(full)

    return {
        "full_rom":    os.path.basename(full_bin_path),
        "calibration": os.path.basename(cal_bin_path),
        "output":      output_path,
        "cal_offset":  hex(cal_offset),
        "cal_size_kb": round(len(cal) / 1024, 1),
        "output_size_kb": round(len(full) / 1024, 1),
    }


if __name__ == "__main__":
    mcp.run()
