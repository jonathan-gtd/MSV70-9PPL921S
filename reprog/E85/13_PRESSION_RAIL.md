# §13 — Pression de rail : c_fup_nom / ip_fup_cor

| Paramètre | Adresse | Valeur stock |
|---|---|---|
| `c_fup_nom` | 0x44B0C | **5000 hPa (5.0 bar)** |
| `ip_fup_cor` | 0x4AF44 | Table 6×6, f(débit L/h) |

**Valeurs stock `ip_fup_cor` :**
```
        débit (L/h) →    50     80    100    110    120    140
toutes tensions :      +0.06  -17.0  -34.0  -46.0  -63.0  -101.0 hPa
```

En WOT E85 (~130 L/h requis), chute de pression interpolée ≈ −80 hPa → pression effective ~4920 hPa. Pompe stock dans les limites si en bon état (test ≥ 2.0 L/30 sec).

| Symptôme | Cause | Solution |
|---|---|---|
| Perte de couple progressive WOT > 10 sec | Chute pression rail | Remplacer pompe (N54 swap ou Walbro 255) |
| DTC P0087 (fuel pressure low) | Pompe insuffisante | Remplacement pompe obligatoire |

**Verdict : ne pas modifier.** Si problème de pression : solution mécanique (pompe), pas logicielle.

---

