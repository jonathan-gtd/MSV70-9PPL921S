"""
TestoFactory — generates and validates customjobs.xml for Testo datalogger.

Argument wire format for STATUS_MESSWERTBLOCK_LESEN:
  "3;0xADDR1;0xADDR2;..."
The leading "3" is the fixed service sub-function used by all MSV70 read jobs.
"""
import xml.etree.ElementTree as ET
from xml.dom import minidom
from typing import List, Dict, Optional


class TestoFactory:
    JOB_FUNCTION = "STATUS_MESSWERTBLOCK_LESEN"
    SERVICE_BYTE = "3"

    def __init__(self, config_manager):
        self.cfg = config_manager

    # ------------------------------------------------------------------ #
    # XML generation                                                       #
    # ------------------------------------------------------------------ #

    def generate_xml(self, jobs: List[Dict]) -> str:
        """
        Build customjobs.xml from a list of job descriptors.
        Each dict: {virtualname, comment, addresses: [str]}
        """
        root = ET.Element("customjobs")
        ecu  = self.cfg.config.get("ecu", "MSV70")

        for job in jobs:
            self._append_job(root, ecu, job)

        return self._prettify(root)

    def merge_with_existing(self, new_jobs: List[Dict], existing_path: str) -> str:
        """
        Append new jobs to an existing XML file, skipping duplicates by virtualname.
        Falls back to generate_xml() if the file is missing or malformed.
        """
        try:
            tree = ET.parse(existing_path)
            root = tree.getroot()
        except (FileNotFoundError, ET.ParseError):
            return self.generate_xml(new_jobs)

        ecu = self.cfg.config.get("ecu", "MSV70")
        existing_names = {
            j.findtext("virtualname", "") for j in root.findall("customjob")
        }
        for job in new_jobs:
            if job.get("virtualname") not in existing_names:
                self._append_job(root, ecu, job)

        return self._prettify(root)

    # ------------------------------------------------------------------ #
    # Validation                                                           #
    # ------------------------------------------------------------------ #

    def validate_job(self, addresses: List[str]) -> List[str]:
        """Return human-readable warning / error strings."""
        issues: List[str] = []
        max_p = int(self.cfg.get_threshold("max_job_params", 20))

        if len(addresses) == 0:
            issues.append("Job has no parameters.")
        elif len(addresses) > max_p:
            issues.append(
                f"Too many parameters: {len(addresses)} (ECU limit: {max_p})."
            )

        for addr in addresses:
            if not self._valid_hex(addr):
                issues.append(f"Invalid hex address: {addr!r}")

        return issues

    # ------------------------------------------------------------------ #
    # Helpers                                                              #
    # ------------------------------------------------------------------ #

    def _append_job(self, root: ET.Element, ecu: str, job: Dict) -> None:
        elem = ET.SubElement(root, "customjob", ecu=ecu)
        ET.SubElement(elem, "jobname").text    = self.JOB_FUNCTION
        ET.SubElement(elem, "virtualname").text = job.get("virtualname", "CUSTOM_JOB")
        ET.SubElement(elem, "comment").text    = job.get("comment", "")
        addrs = job.get("addresses", [])
        arg = self.SERVICE_BYTE
        if addrs:
            arg += ";" + ";".join(addrs)
        ET.SubElement(elem, "argument").text = arg

    @staticmethod
    def _valid_hex(addr: str) -> bool:
        try:
            int(addr, 16)
            return True
        except (ValueError, TypeError):
            return False

    @staticmethod
    def _prettify(root: ET.Element) -> str:
        raw    = ET.tostring(root, encoding="unicode")
        parsed = minidom.parseString(raw)
        return parsed.toprettyxml(indent="  ")
