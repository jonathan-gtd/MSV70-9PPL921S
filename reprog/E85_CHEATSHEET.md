# E85 Cheatsheet — MSV70 N52B30 · VB67774_921S · Injecteurs 13537531634

> Adresses = adresses XDF (TunerPro). Bin offset = adresse − 0x40000.

---

## Tableau Principal

| Priorité | Paramètre | Adresse XDF | ◀ Raw stock | ◀ Valeur stock | ▶ Raw E85 | ▶ Valeur E85 | Action TunerPro |
|---|---|---|---|---|---|---|---|
| **1** | `ip_mff_cor_opm_1_1` | 0x4E3D4 | 32 770 | 1.016 | **47 407** | **1.473** | Ctrl+A → saisir 47407 |
| **1** | `ip_mff_cor_opm_1_2` | 0x4E554 | 32 770 | 1.016 | **47 407** | **1.473** | Ctrl+A → saisir 47407 |
| **1** | `ip_mff_cor_opm_2_1` | 0x4E6D4 | 32 770 | 1.016 | **47 407** | **1.473** | Ctrl+A → saisir 47407 |
| **1** | `ip_mff_cor_opm_2_2` | 0x4E7C4 | 32 770 | 1.016 | **47 407** | **1.473** | Ctrl+A → saisir 47407 |
| **1** | `c_tco_n_mff_cst` | 0x44F2F | 87 | 17.25 °C | **97** | **25.00 °C** | Saisir 97 |
| **2** | `ip_mff_cst_opm_1` | 0x437DC | table 3×8 | voir ci-dessous | table 3×8 | **×1.35–2.00** selon TCO | Multiplier col par col (E70) |
| **2** | `ip_mff_cst_opm_2` | 0x4380C | table 3×8 | voir ci-dessous | table 3×8 | **×1.35–2.00** selon TCO | Idem — mode papillonné |
| **2** | `ip_fac_lamb_wup` | 0x42764 | 128 partout | 1.000 partout | — | **1.03–1.08** (basses charges) | Éditer basses charges |
| **3** | `ip_ti_tco_pos_slow_wf_opm_1` | 0x4CBFC | table 8×8 | voir ci-dessous | table 8×8 | **stock × 1.25** | Ctrl+A → Scale ×1.25 |
| **3** | `ip_ti_tco_pos_slow_wf_opm_2` | 0x4CC7C | table 8×8 | voir ci-dessous | table 8×8 | **stock × 1.25** | Ctrl+A → Scale ×1.25 |
| **3** | `ip_ti_tco_pos_fast_wf_opm_1` | 0x443FC | table 8×8 | voir ci-dessous | table 8×8 | **stock × 1.25** | Ctrl+A → Scale ×1.25 |
| **3** | `ip_ti_tco_pos_fast_wf_opm_2` | 0x4443C | table 8×8 | voir ci-dessous | table 8×8 | **stock × 1.25** | Ctrl+A → Scale ×1.25 |
| **4** | `ip_iga_bas_max_knk__n__maf` | 0x4323A | table 8×8 | voir ci-dessous | table 8×8 | **+0 à +2.5°** par zone | Ajouter incréments — 50 km/palier |
| **5** | `c_t_ti_dly_fl_1` | — | — | — | — | **0 ms** | Saisir 0 |
| **5** | `c_t_ti_dly_fl_2` | — | — | — | — | **0 ms** | Saisir 0 |
| **OPT** | `ip_lamb_fl__n` | 0x436A2 | courbe 1×12 | λ 0.920 | — | **Laisser stock** (ou 0.940–0.950) | Rien — stock suffisant |
| **OPT** | `c_iga_ini` | — | — | — | — | **stock +1° à +2°** | Seulement si démarrage > 5 tours |

> `c_fac_mff_ti_stnd` : **NE PAS MODIFIER** (overflow XDF — max physique encodable = 0.393 < cible 0.491)

---

## ip_mff_cst_opm_1 et _2 — Facteurs E70 à appliquer colonne par colonne

| TCO (°C) | −30 | −20 | −10 | 0 | +17 | +30 | +60 | +90 |
|---|---|---|---|---|---|---|---|---|
| **Facteur ×** | **×2.00** | **×1.80** | **×1.65** | **×1.55** | **×1.35** | **×1.20** | **×1.10** | **×1.05** |

**ip_mff_cst_opm_1** (Valvetronic) — AVANT (stock) / APRÈS (E70) :

```
TCO (°C) :  -30.0   -20.2    -9.8    0.0   17.2   30.0   60.0   90.0
  AVANT (stock) :
RPM  80  :  447.7   350.6   261.3  189.3  102.2   71.4   56.2   49.6
RPM 320  :  320.3   260.7   202.1  152.1   87.9   61.3   46.5   39.1
RPM 920  :  194.4   175.1   146.0  112.9   68.4   48.6   36.5   33.0
  APRÈS (E70) :
RPM  80  :  895.4   631.1   431.1  293.4  138.0   85.7   61.8   52.1
RPM 320  :  640.6   469.3   333.5  235.8  118.7   73.6   51.2   41.1
RPM 920  :  388.8   315.2   240.9  175.0   92.3   58.3   40.2   34.7
```

**ip_mff_cst_opm_2** (papillonné) — AVANT / APRÈS (E70) :

```
TCO (°C) :  -30.0   -20.2    -9.8    0.0   17.2   30.0   60.0   90.0
  AVANT (stock) :
RPM  80  :  731.1   527.0   362.8  245.0  138.2  102.1   67.8   49.6
RPM 320  :  546.2   415.6   297.0  201.8  106.4   82.3   57.0   39.1
RPM 920  :  363.0   281.4   215.8  159.0   84.1   65.8   47.0   34.5
  APRÈS (E70) :
RPM  80  : 1462.2   948.6   598.6  379.8  186.6  122.5   74.6   52.1
RPM 320  : 1092.4   748.1   490.1  312.8  143.6   98.8   62.7   41.1
RPM 920  :  726.0   506.5   356.1  246.5  113.5   79.0   51.7   36.2
```

---

## ip_fac_lamb_wup — AVANT : 1.000 partout / APRÈS :

```
MAF →           65    100    200    300    400    500 mg/stk
 704 rpm :    1.08   1.08   1.05   1.03   1.00   1.00
1216 rpm :    1.08   1.07   1.05   1.03   1.00   1.00
1760 rpm :    1.07   1.06   1.04   1.03   1.00   1.00
2016 rpm :    1.06   1.05   1.04   1.03   1.00   1.00
2496 rpm :    1.05   1.05   1.03   1.03   1.00   1.00
3008 rpm :    1.03   1.03   1.03   1.00   1.00   1.00
```

---

## ip_iga_bas_max_knk__n__maf — Incréments à AJOUTER au stock (calibration E60)

```
RPM \ MAF    0.55   0.64   1.02   1.31   1.55   1.78   2.01   2.25
 608          +0     +0     +0     +0     +0     +0     +0     +0
1504          +0     +0     +0     +0    +0.5   +0.5   +0.5   +0.5
2016          +0     +0     +0    +0.5   +1.0   +1.0   +1.0   +1.0
3008          +0     +0    +0.5   +1.0   +1.5   +1.5   +2.0   +2.0
4000          +0     +0    +0.5   +1.0   +1.5   +2.0   +2.0   +2.0
4992          +0     +0    +0.5   +1.0   +1.5   +2.0   +2.5   +2.5
6016          +0     +0    +0.5   +1.0   +2.0   +2.0   +2.5   +2.5
7008          +0     +0    +0.5   +1.0   +2.0   +2.0   +2.5   +2.5
```

---

## Vérification post-flash — Valeurs OBD2 attendues

| Signal | ✅ Normal | ⚠️ Action |
|---|---|---|
| STFT ralenti chaud | **−5% à +5%** | > +15% → `ip_mff_cor_opm_*` +3% / < −15% → −3% |
| LTFT stabilisé | **−8% à +12%** | Hors plage → ajuster `ip_mff_cor_opm_*` |
| Démarrage froid (< 15°C) | **≤ 3 tours** | > 5 tours → cranking +15% |
| STFT après 2 min warm-up | **−5% à +5%** | Oscillant → `ip_fac_lamb_wup` +0.02 |
| Tip-in tiède (~50°C) | **Lisse, aucun trou** | Trou → film fast +10% ligne TCO |
| Knock OBD2 post-avance | **0 counts** | Cliquetis → −1° immédiat |

---

## Référence rapide — Titres éthanol vs facteur injecteur

| Titre pompe | AFR stœchio | Facteur vs stock | Raw `ip_mff_cor_opm_*` |
|---|---|---|---|
| E65 | 10.41:1 | ×1.33 | 43 613 |
| E70 | 10.18:1 | ×1.36 | 44 581 |
| E75 | 9.97:1 | ×1.39 | 45 548 |
| **E85 ← cible** | **9.55:1** | **×1.45** | **47 407** |

---

## Transition E85 ↔ SP95

| Sens | Ordre |
|---|---|
| SP95 → E85 | Flash E85 → Reset LTFT → Trajet station (douce) → Plein E85 |
| E85 → SP95 | Quasi-vide → Flash stock → Reset LTFT → Trajet station (très douce) → Plein SP95 |

> **Jamais : plein d'abord, flash ensuite.**
