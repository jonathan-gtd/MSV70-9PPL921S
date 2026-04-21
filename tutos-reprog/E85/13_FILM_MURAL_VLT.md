# §11 — Film mural Valvetronic (N52) : ip_fac_ti_maf_sp_wf_pos_opm_1

| Paramètre | Adresse | Structure |
|---|---|---|
| `ip_fac_ti_maf_sp_wf_pos_opm_1` | 0x42C5A | 1×8, uint8, ×0.007813, f(TCO) |
| `ip_fac_ti_maf_sp_wf_neg_opm_1` | 0x42C4A | 1×8, uint8, ×0.007813 |

**Valeurs stock :**
```
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.3   65.3   84.8  114.8
pos :        0.414  0.399  0.383  0.359  0.313  0.234  0.211  0.195
neg :        0.406  0.399  0.375  0.344  0.274  0.125  0.047  0.008
```

Sur N52, c'est la levée Valvetronic (pas le papillon) qui crée les variations de MAF setpoint. Le MSV70 a une compensation spécifique pour le film mural induit par ces changements de levée — distincte du film mural TCO/RPM de §5.

**Symptôme d'inadéquation :** couple instable lors de modulations douces de la pédale (pas de tip-in brusque), spécifiquement les 5 premières minutes à froid.

**Verdict : ne pas modifier en première intention.** Valider d'abord §5. Si symptôme persiste après validation §5 : +15 à +25% sur TCO < 15°C de `ip_fac_ti_maf_sp_wf_pos_opm_1`.

---

