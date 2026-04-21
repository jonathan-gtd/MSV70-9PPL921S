# §12 — Limites LTFT / STFT

> **⚠️ Les vraies limites MSV70 sont −8% / +12% — pas ±25%**

**Valeurs réelles lues dans le bin :**

| Paramètre | Adresse | Valeur stock | Description |
|---|---|---|---|
| `c_fac_max_h_rng_lam_ad` | 0x47F4C | **+12.0 %** | Limite haute LTFT — haute charge |
| `c_fac_max_l_rng_lam_ad` | 0x47F4E | **+12.0 %** | Limite haute LTFT — basse charge |
| `c_fac_min_h_rng_lam_ad` | 0x47F50 | **92.0 %** | Limite basse (100%−92% = **−8%**) — haute charge |
| `c_fac_min_l_rng_lam_ad` | 0x47F52 | **92.0 %** | Limite basse — basse charge |
| `c_lam_mv_dyw_dly` | 0x44B3E | **7.7 %** | Fenêtre dynamique STFT |

Les limites réelles de ce bin MSV70 sont asymétriques :

| Trim | Valeur réelle |
|---|---|
| LTFT max positif | **+12 %** |
| LTFT max négatif | **−8 %** |
| STFT fenêtre dynamique | **±7.7 %** |

L'asymétrie (−8%/+12%) est intentionnelle : le MSV70 tolère mieux un mélange légèrement riche qu'un mélange pauvre.

**Implications pour la conversion :**
- `ip_mff_cor_opm` calibré E85 (raw 47 407) + carburant réel E70 → LTFT ≈ **−6 à −8%** → dans les limites, acceptable
- `ip_mff_cor_opm` calibré E85 + E60 hivernal → LTFT cible ≈ **−13%** → dépasse le plafond → ECU ne compense pas complètement → mélange légèrement riche persistant (acceptable mais pas parfait)

**Verdict : ne pas modifier.** Si LTFT plafonne à −8% en permanence : affiner `ip_mff_cor_opm` vers E70 (raw 44 581) selon le titre réel de vos stations.

---

