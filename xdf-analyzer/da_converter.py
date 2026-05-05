"""Convert WinKFP .0da Intel HEX calibration files to flat binary."""


def _parse_intel_hex(path: str) -> dict[int, int]:
    """Parse Intel HEX records, return {absolute_address: byte_value}."""
    data: dict[int, int] = {}
    upper_base = 0
    with open(path, "r", encoding="latin-1") as f:
        for line in f:
            line = line.strip()
            if not line.startswith(":"):
                continue
            byte_count = int(line[1:3], 16)
            address    = int(line[3:7], 16)
            rec_type   = int(line[7:9], 16)
            payload    = bytes.fromhex(line[9:9 + byte_count * 2])

            if rec_type == 0x00:  # Data record
                abs_addr = upper_base + address
                for i, b in enumerate(payload):
                    data[abs_addr + i] = b
            elif rec_type == 0x04:  # Extended linear address
                upper_base = int.from_bytes(payload, "big") << 16
            elif rec_type == 0x01:  # EOF
                break
    return data


def extract_calibration(path: str, base_addr: int = 0x840000, size: int = 0x20000) -> bytes:
    """
    Extract the calibration region from a .0da Intel HEX file.

    The MSV70 calibration lives at ECU internal address 0x840000 (128 KB).
    Returns a flat bytes object of `size` bytes (0xFF = erased/unprogrammed).
    """
    raw = _parse_intel_hex(path)
    result = bytearray(b"\xFF" * size)
    in_range = 0
    for addr, val in raw.items():
        offset = addr - base_addr
        if 0 <= offset < size:
            result[offset] = val
            in_range += 1
    return bytes(result), in_range
