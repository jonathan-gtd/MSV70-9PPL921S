"""
Module C — Testo Custom Job Creator.

Workflow:
  1. Load a MSV70_jobs.txt definition file.
  2. Filter / browse the opcode table.
  3. Tick parameters → they appear in the Job Builder panel.
  4. Name the job, add it to the batch.
  5. Repeat for multiple jobs, then export customjobs.xml.

Argument wire format: "3;0xADDR1;0xADDR2;..."
ECU limit validation uses max_job_params from config.
"""
import os
from tkinter import filedialog, messagebox
import customtkinter as ctk


class JobCreatorModule(ctk.CTkFrame):
    def __init__(self, parent, config_manager, testo_factory):
        super().__init__(parent, fg_color="transparent")
        self.cfg     = config_manager
        self.factory = testo_factory

        self._all_params: list  = []    # [{id, hex, desc_en, unit, …}]
        self._check_vars: dict  = {}    # hex -> BooleanVar
        self._batch: list       = []    # completed job dicts

        self._build_ui()

    # ------------------------------------------------------------------ #
    # Layout                                                               #
    # ------------------------------------------------------------------ #

    def _build_ui(self) -> None:
        ctk.CTkLabel(
            self, text="Testo Custom Job Creator",
            font=ctk.CTkFont(size=20, weight="bold"),
        ).pack(anchor="w", padx=20, pady=(16, 6))

        # ── Toolbar ─────────────────────────────────────────────────────
        bar = ctk.CTkFrame(self, fg_color="transparent")
        bar.pack(fill="x", padx=20, pady=(0, 8))

        ctk.CTkButton(
            bar, text="📂  Load Definitions File",
            command=self._load_defs,
            fg_color="#0f4c81", hover_color="#1565c0", height=34,
        ).pack(side="left", padx=(0, 10))

        self._filter_var = ctk.StringVar()
        self._filter_var.trace_add("write", lambda *_: self._apply_filter())
        ctk.CTkEntry(
            bar, textvariable=self._filter_var,
            placeholder_text="🔍  Filter by ID, hex, description…",
            width=280, height=34,
        ).pack(side="left", padx=(0, 10))

        self._count_lbl = ctk.CTkLabel(bar, text="0 / 0", text_color="#64748b")
        self._count_lbl.pack(side="left")

        # ── Split: opcode browser (left) | job builder (right) ──────────
        body = ctk.CTkFrame(self, fg_color="transparent")
        body.pack(fill="both", expand=True, padx=20, pady=(0, 16))
        body.columnconfigure(0, weight=3)
        body.columnconfigure(1, weight=2)
        body.rowconfigure(0, weight=1)

        self._build_opcode_panel(body)
        self._build_builder_panel(body)

    def _build_opcode_panel(self, parent: ctk.CTkFrame) -> None:
        frame = ctk.CTkFrame(parent, fg_color="#0d1117")
        frame.grid(row=0, column=0, sticky="nsew", padx=(0, 8))

        ctk.CTkLabel(frame, text="Available Opcodes",
                     font=ctk.CTkFont(size=13, weight="bold")).pack(
            anchor="w", padx=12, pady=(10, 4))

        # Column headers
        hdr = ctk.CTkFrame(frame, fg_color="#161b22", height=26)
        hdr.pack(fill="x", padx=8, pady=(0, 2))
        hdr.pack_propagate(False)
        for txt, w in [("", 24), ("ID", 58), ("Hex", 72), ("Description", 0), ("Unit", 52)]:
            ctk.CTkLabel(hdr, text=txt, width=w, anchor="w",
                         font=ctk.CTkFont(size=11, weight="bold"),
                         text_color="#64748b").pack(side="left", padx=2)

        self._opcode_scroll = ctk.CTkScrollableFrame(frame, fg_color="transparent")
        self._opcode_scroll.pack(fill="both", expand=True, padx=6, pady=(0, 8))

    def _build_builder_panel(self, parent: ctk.CTkFrame) -> None:
        frame = ctk.CTkFrame(parent, fg_color="#0d1117")
        frame.grid(row=0, column=1, sticky="nsew", padx=(8, 0))

        ctk.CTkLabel(frame, text="Job Builder",
                     font=ctk.CTkFont(size=13, weight="bold")).pack(
            anchor="w", padx=12, pady=(10, 6))

        # Job name
        r = ctk.CTkFrame(frame, fg_color="transparent")
        r.pack(fill="x", padx=12, pady=3)
        ctk.CTkLabel(r, text="Job name:", width=90, anchor="w").pack(side="left")
        self._name_var = ctk.StringVar(value="MY_JOB")
        ctk.CTkEntry(r, textvariable=self._name_var, width=160).pack(side="left")

        # Comment
        r2 = ctk.CTkFrame(frame, fg_color="transparent")
        r2.pack(fill="x", padx=12, pady=3)
        ctk.CTkLabel(r2, text="Comment:", width=90, anchor="w").pack(side="left")
        self._comment_var = ctk.StringVar()
        ctk.CTkEntry(r2, textvariable=self._comment_var, width=200).pack(side="left")

        # Selected params preview
        ctk.CTkLabel(frame, text="Selection:", anchor="w",
                     font=ctk.CTkFont(size=11, weight="bold")).pack(
            anchor="w", padx=12, pady=(8, 2))
        self._sel_scroll = ctk.CTkScrollableFrame(frame, height=170,
                                                   fg_color="#0a0f1a")
        self._sel_scroll.pack(fill="x", padx=10, pady=(0, 4))

        self._sel_count_lbl = ctk.CTkLabel(frame, text="0 parameter(s) selected",
                                            text_color="#64748b",
                                            font=ctk.CTkFont(size=11))
        self._sel_count_lbl.pack(anchor="w", padx=12)

        self._warn_lbl = ctk.CTkLabel(frame, text="", text_color="#ef4444",
                                       wraplength=310, font=ctk.CTkFont(size=11))
        self._warn_lbl.pack(anchor="w", padx=12, pady=2)

        # Action buttons
        br = ctk.CTkFrame(frame, fg_color="transparent")
        br.pack(fill="x", padx=12, pady=(4, 6))
        ctk.CTkButton(br, text="+ Add to Batch", command=self._add_to_batch,
                      fg_color="#166534", hover_color="#14532d",
                      width=130, height=32).pack(side="left", padx=(0, 8))
        ctk.CTkButton(br, text="✕ Clear", command=self._clear_selection,
                      fg_color="#581c87", hover_color="#3b0764",
                      width=80, height=32).pack(side="left")

        # Batch list
        ctk.CTkLabel(frame, text="Batch Queue:",
                     font=ctk.CTkFont(size=11, weight="bold")).pack(
            anchor="w", padx=12, pady=(10, 2))
        self._batch_scroll = ctk.CTkScrollableFrame(frame, height=90,
                                                     fg_color="#0a0f1a")
        self._batch_scroll.pack(fill="x", padx=10, pady=(0, 6))

        # Export buttons
        er = ctk.CTkFrame(frame, fg_color="transparent")
        er.pack(fill="x", padx=12, pady=(2, 12))
        ctk.CTkButton(er, text="💾  Export XML", command=self._export_xml,
                      fg_color="#0f4c81", hover_color="#1565c0",
                      width=130, height=32).pack(side="left", padx=(0, 8))
        ctk.CTkButton(er, text="🗑  Clear Batch", command=self._clear_batch,
                      fg_color="#7f1d1d", hover_color="#450a0a",
                      width=110, height=32).pack(side="left")

    # ------------------------------------------------------------------ #
    # Definitions loading & filtering                                      #
    # ------------------------------------------------------------------ #

    def _load_defs(self) -> None:
        path = filedialog.askopenfilename(
            title="Select MSV70 jobs definition file",
            filetypes=[("Text files", "*.txt"), ("All files", "*.*")],
        )
        if not path:
            return
        self.cfg.invalidate_jobs_cache()
        self._all_params = self.cfg.load_jobs(path)
        self._check_vars.clear()
        self._apply_filter()

    def _apply_filter(self) -> None:
        flt = self._filter_var.get().lower()
        visible = [
            p for p in self._all_params
            if not flt or any(
                flt in p[k].lower()
                for k in ("id", "hex", "desc_en", "stat_name", "desc_de", "unit")
            )
        ]

        for w in self._opcode_scroll.winfo_children():
            w.destroy()

        for param in visible:
            self._render_opcode_row(param)

        total = len(self._all_params)
        shown = len(visible)
        self._count_lbl.configure(text=f"{shown} / {total} params")

    def _render_opcode_row(self, param: dict) -> None:
        hex_addr = param["hex"]
        if hex_addr not in self._check_vars:
            self._check_vars[hex_addr] = ctk.BooleanVar(value=False)
        var = self._check_vars[hex_addr]

        row = ctk.CTkFrame(self._opcode_scroll, fg_color="transparent",
                           cursor="hand2")
        row.pack(fill="x", pady=1)

        ctk.CTkCheckBox(
            row, variable=var, text="", width=20,
            checkbox_width=15, checkbox_height=15,
            command=self._on_check,
        ).pack(side="left")

        id_color = "#38bdf8" if param["id"] not in ("-", "") else "#475569"
        ctk.CTkLabel(row, text=param["id"][:8], width=56, anchor="w",
                     font=ctk.CTkFont(size=11), text_color=id_color).pack(side="left")
        ctk.CTkLabel(row, text=param["hex"], width=70, anchor="w",
                     font=ctk.CTkFont(family="Courier New", size=11),
                     text_color="#4ade80").pack(side="left")

        desc = (param["desc_en"] or param["desc_de"] or param["stat_name"])
        ctk.CTkLabel(row, text=desc[:52], anchor="w",
                     font=ctk.CTkFont(size=11)).pack(side="left", padx=(2, 4), fill="x",
                                                      expand=True)
        ctk.CTkLabel(row, text=param["unit"][:8], width=50, anchor="w",
                     font=ctk.CTkFont(size=11),
                     text_color="#fb923c").pack(side="left")

    # ------------------------------------------------------------------ #
    # Selection state                                                      #
    # ------------------------------------------------------------------ #

    def _on_check(self) -> None:
        selected = self._get_selected_params()

        for w in self._sel_scroll.winfo_children():
            w.destroy()
        for p in selected:
            r = ctk.CTkFrame(self._sel_scroll, fg_color="transparent")
            r.pack(fill="x", pady=1)
            ctk.CTkLabel(r, text=p["hex"], width=74,
                         font=ctk.CTkFont(family="Courier New", size=11),
                         text_color="#4ade80").pack(side="left")
            desc = p["desc_en"] or p["stat_name"]
            ctk.CTkLabel(r, text=desc[:44],
                         font=ctk.CTkFont(size=11)).pack(side="left")

        n = len(selected)
        self._sel_count_lbl.configure(text=f"{n} parameter(s) selected")

        issues = self.factory.validate_job([p["hex"] for p in selected])
        self._warn_lbl.configure(text="\n".join(issues))

    def _get_selected_params(self) -> list:
        return [p for p in self._all_params
                if self._check_vars.get(p["hex"], ctk.BooleanVar()).get()]

    def _clear_selection(self) -> None:
        for var in self._check_vars.values():
            var.set(False)
        self._on_check()

    # ------------------------------------------------------------------ #
    # Batch management                                                     #
    # ------------------------------------------------------------------ #

    def _add_to_batch(self) -> None:
        selected = self._get_selected_params()
        if not selected:
            messagebox.showwarning("Empty selection",
                                   "Tick at least one parameter first.")
            return

        issues = self.factory.validate_job([p["hex"] for p in selected])
        if any("CRITICAL" in i or "Too many" in i for i in issues):
            if not messagebox.askyesno(
                "Validation warning",
                "\n".join(issues) + "\n\nAdd anyway?",
            ):
                return

        job = {
            "virtualname": self._name_var.get().strip().upper().replace(" ", "_"),
            "comment":     self._comment_var.get().strip(),
            "addresses":   [p["hex"] for p in selected],
            "_labels":     [p["desc_en"] or p["stat_name"] for p in selected],
        }
        self._batch.append(job)
        self._refresh_batch_view()
        self._clear_selection()
        self._name_var.set("MY_JOB")
        self._comment_var.set("")

    def _refresh_batch_view(self) -> None:
        for w in self._batch_scroll.winfo_children():
            w.destroy()
        for job in self._batch:
            r = ctk.CTkFrame(self._batch_scroll, fg_color="transparent")
            r.pack(fill="x", pady=2)
            ctk.CTkLabel(
                r,
                text=f"  {job['virtualname']}  ({len(job['addresses'])} params)",
                font=ctk.CTkFont(size=11), text_color="#38bdf8",
            ).pack(side="left")

    def _clear_batch(self) -> None:
        self._batch.clear()
        self._refresh_batch_view()

    # ------------------------------------------------------------------ #
    # Export                                                               #
    # ------------------------------------------------------------------ #

    def _export_xml(self) -> None:
        if not self._batch:
            messagebox.showwarning("Empty batch",
                                   "Add at least one job to the batch first.")
            return

        path = filedialog.asksaveasfilename(
            title="Save customjobs.xml",
            defaultextension=".xml",
            initialfile="customjobs.xml",
            filetypes=[("XML files", "*.xml"), ("All files", "*.*")],
        )
        if not path:
            return

        # Offer to merge with an existing file if one already exists
        if os.path.exists(path):
            if messagebox.askyesno(
                "Merge?",
                f"{os.path.basename(path)} already exists.\n"
                "Merge new jobs into it (skip duplicates)?",
            ):
                content = self.factory.merge_with_existing(self._batch, path)
            else:
                content = self.factory.generate_xml(self._batch)
        else:
            content = self.factory.generate_xml(self._batch)

        with open(path, "w", encoding="utf-8") as fh:
            fh.write(content)

        messagebox.showinfo(
            "Export complete",
            f"Exported {len(self._batch)} job(s) to:\n{path}",
        )
