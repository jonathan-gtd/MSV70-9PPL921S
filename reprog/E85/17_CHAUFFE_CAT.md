# §17 — Stratégie de chauffe catalyseur

| Paramètre | Adresse | Structure |
|---|---|---|
| `ip_fac_eff_iga_ch_cold_opm_1` | 0x4A444 | 10×10, uint8, ×0.044, f(TPS × RPM) |
| `ip_fac_eff_iga_ch_cold_opm_2` | 0x4A4A8 | 10×10, uint8, ×0.044 |

Facteurs de retard d'allumage (~4–7.5°) appliqués pendant la chauffe catalyseur pour générer des EGT élevées et accélérer le light-off. L'éthanol brûle à une température légèrement plus basse → EGT démarrage E85 ~30–50°C inférieures → catalyseur met un peu plus longtemps à monter en température.

**Conséquences pratiques :** DTC P0420/P0430 peuvent apparaître les 200–500 premiers km → normaux, disparaissent après stabilisation des LTFT.

**Verdict : ne pas modifier si les catalyseurs sont présents.** Si P0420/P0430 persistent > 500 km : effacer adaptations ISTA + 200 km supplémentaires.

---

> ⚠️ **Sans catalyseurs (dépollution supprimée) :** ces deux tables doivent être **mises à zéro** (toutes cellules → raw 0). Le retard de chauffe n'a plus de raison d'être et pénalise inutilement l'allumage à froid. DTC P0420/P0430 devront être masqués séparément dans le bin.
