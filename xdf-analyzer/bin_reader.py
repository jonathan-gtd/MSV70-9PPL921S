import struct
import re


def _safe_eval(equation: str, x: float) -> float:
    """Evaluate a TunerPro equation string with X replaced by x."""
    if not equation or equation.strip().upper() == "X":
        return x
    expr = re.sub(r'\bX\b', str(x), equation)
    try:
        return float(eval(expr, {"__builtins__": {}}, {}))
    except Exception:
        return x


def _dtype_fmt(bin_dtype: str, lsb_first: bool = True) -> str:
    endian = "<" if lsb_first else ">"
    return {
        "uint8":  endian + "B",
        "uint16": endian + "H",
        "uint32": endian + "I",
        "int8":   endian + "b",
        "int16":  endian + "h",
        "int32":  endian + "i",
    }.get(bin_dtype, endian + "B")


def read_axis_values(bin_path: str, addr_str: str, count: int, bits: int, lsb_first: bool, equation: str) -> list[float] | None:
    """Read axis tick values from a BIN file."""
    if not addr_str:
        return None
    try:
        addr = int(addr_str, 16)
    except ValueError:
        return None
    dtype = {8: "uint8", 16: "uint16", 32: "uint32"}.get(bits, "uint8")
    fmt   = _dtype_fmt(dtype, lsb_first)
    size  = struct.calcsize(fmt)
    try:
        with open(bin_path, "rb") as f:
            f.seek(addr)
            raw_bytes = f.read(size * count)
    except (IOError, OSError):
        return None
    if len(raw_bytes) < size * count:
        return None
    raws = [struct.unpack_from(fmt, raw_bytes, i * size)[0] for i in range(count)]
    return [round(_safe_eval(equation, v), 6) for v in raws]


def read_param_values(bin_path: str, param: dict) -> dict:
    """
    Read raw + converted values for a parameter from a BIN file.

    Returns a dict:
      raw      : list[int]
      physical : list[float]
      rows     : int
      cols     : int
      z_unit   : str
      z_eq     : str
    """
    bin_addr_str = param.get("bin_addr", "?")
    if bin_addr_str == "?":
        return {"error": "No BIN address in XDF definition"}

    try:
        bin_addr = int(bin_addr_str, 16)
    except ValueError:
        return {"error": f"Invalid BIN address: {bin_addr_str}"}

    rows      = int(param.get("rows", 1))
    cols      = int(param.get("cols", 1))
    dtype     = param.get("bin_dtype", "uint8")
    lsb_first = param.get("lsb_first", False)
    z_eq      = param.get("z_eq", "X")
    z_unit    = param.get("z_unit", "")
    count     = rows * cols

    fmt = _dtype_fmt(dtype, lsb_first)
    elem_size = struct.calcsize(fmt)

    try:
        with open(bin_path, "rb") as f:
            f.seek(bin_addr)
            raw_bytes = f.read(elem_size * count)
    except (IOError, OSError) as e:
        return {"error": str(e)}

    if len(raw_bytes) < elem_size * count:
        return {"error": f"BIN too short: needed {elem_size*count} bytes at 0x{bin_addr:X}"}

    raw_values = [
        struct.unpack_from(fmt, raw_bytes, i * elem_size)[0]
        for i in range(count)
    ]
    physical_values = [round(_safe_eval(z_eq, v), 6) for v in raw_values]

    x_values = read_axis_values(bin_path,
        param.get("x_addr"), param.get("x_count", 1),
        param.get("x_bits", 8), param.get("x_lsb", False), param.get("x_eq", "X"))
    y_values = read_axis_values(bin_path,
        param.get("y_addr"), param.get("y_count", 1),
        param.get("y_bits", 8), param.get("y_lsb", False), param.get("y_eq", "X"))

    result = {
        "raw":      raw_values,
        "physical": physical_values,
        "rows":     rows,
        "cols":     cols,
        "z_unit":   z_unit,
        "z_eq":     z_eq,
    }
    if x_values and len(x_values) > 1:
        result["x_values"] = x_values
        result["x_unit"]   = param.get("x_unit", "")
    if y_values and len(y_values) > 1:
        result["y_values"] = y_values
        result["y_unit"]   = param.get("y_unit", "")
    return result


def compare_params(bin_path1: str, bin_path2: str, param: dict) -> dict:
    """Compare a parameter's values between two BIN files."""
    r1 = read_param_values(bin_path1, param)
    r2 = read_param_values(bin_path2, param)

    if "error" in r1:
        return {"error": f"BIN1: {r1['error']}"}
    if "error" in r2:
        return {"error": f"BIN2: {r2['error']}"}

    diffs = []
    for i, (v1, v2) in enumerate(zip(r1["raw"], r2["raw"])):
        if v1 != v2:
            row = i // r1["cols"] if r1["cols"] > 0 else 0
            col = i %  r1["cols"] if r1["cols"] > 0 else 0
            diffs.append({
                "index": i,
                "row":   row,
                "col":   col,
                "raw1":  v1,
                "raw2":  v2,
                "phys1": r1["physical"][i],
                "phys2": r2["physical"][i],
            })

    return {
        "changed":  len(diffs) > 0,
        "diff_count": len(diffs),
        "total_cells": len(r1["raw"]),
        "diffs":    diffs,
        "z_unit":   r1["z_unit"],
        "z_eq":     r1["z_eq"],
    }
