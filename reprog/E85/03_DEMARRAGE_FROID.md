# §3 — Démarrage à Froid (Cranking & After-Start)

> 💡 L'éthanol s'évapore difficilement sous 20°C (ébullition 78°C vs −40°C essence). Le moteur a besoin de **1.5× à 2× plus** de masse carburant au cranking. Trois paramètres à régler : cranking, seuil TCO, warm-up.

---

### 2.1 — Tables de cranking : `ip_mff_cst_opm_1` et `ip_mff_cst_opm_2`

### 📋 Tables à modifier

| Paramètre | Adresse | Structure | Équation | Axes |
|---|---|---|---|---|
| `ip_mff_cst_opm_1` | 0x437DC | 3×8 | 0.021195 × X | X = TCO (°C), Y = RPM |
| `ip_mff_cst_opm_2` | 0x4380C | 3×8 | 0.021195 × X | X = TCO (°C), Y = RPM |

### 🔨 Procédure — facteurs E70 à appliquer colonne par colonne

| TCO (°C) | −30 | −20 | −10 | 0 | +17 | +30 | +60 | +90 |
|---|---|---|---|---|---|---|---|---|
| **Facteur** | ×2.00 | ×1.80 | ×1.65 | ×1.55 | ×1.35 | ×1.20 | ×1.10 | ×1.05 |

> Sur E70 (30% essence), la volatilité à froid est améliorée — facteurs légèrement inférieurs à un E85 pur.

<a id="pencil-cranking"></a>

### ✏️ Avant / Après

**`ip_mff_cst_opm_1` @ 0x437DC** (mode normal / Valvetronic actif) :

STOCK :
```
TCO (°C) :  -30.0   -20.2    -9.8    0.0   17.2   30.0   60.0   90.0
RPM  80  :  447.7   350.6   261.3  189.3  102.2   71.4   56.2   49.6
RPM 320  :  320.3   260.7   202.1  152.1   87.9   61.3   46.5   39.1
RPM 920  :  194.4   175.1   146.0  112.9   68.4   48.6   36.5   33.0
```

OBJECTIF E70 :
```
TCO (°C) :  -30.0   -20.2    -9.8    0.0   17.2   30.0   60.0   90.0
RPM  80  :  895.4   631.1   431.1  293.4  138.0   85.7   61.8   52.1
RPM 320  :  640.6   469.3   333.5  235.8  118.7   73.6   51.2   41.1
RPM 920  :  388.8   315.2   240.9  175.0   92.3   58.3   40.2   34.7
```

**`ip_mff_cst_opm_2` @ 0x4380C** (mode papillonné / Valvetronic désactivé) :

STOCK :
```
TCO (°C) :  -30.0   -20.2    -9.8    0.0   17.2   30.0   60.0   90.0
RPM  80  :  731.1   527.0   362.8  245.0  138.2  102.1   67.8   49.6
RPM 320  :  546.2   415.6   297.0  201.8  106.4   82.3   57.0   39.1
RPM 920  :  363.0   281.4   215.8  159.0   84.1   65.8   47.0   34.5
```

OBJECTIF E70 :
```
TCO (°C) :  -30.0   -20.2    -9.8    0.0   17.2   30.0   60.0   90.0
RPM  80  : 1462.2   948.6   598.6  379.8  186.6  122.5   74.6   52.1
RPM 320  : 1092.4   748.1   490.1  312.8  143.6   98.8   62.7   41.1
RPM 920  :  726.0   506.5   356.1  246.5  113.5   79.0   51.7   36.2
```

> opm_2 a des valeurs nettement plus élevées à froid (jusqu'à ×2.0 vs opm_1 à −30°C) — les mêmes facteurs multiplicatifs s'appliquent aux deux tables.

### ✅ Vérification

| Critère | ✅ OK | ⚠️ Si raté |
|---|---|---|
| Démarrage froid, sans pédale | **≤ 3 tours** | > 5 tours → cranking +15%, reflasher |
| Ralenti initial | 800–1200 tr/min, décroissant | Instable → voir §2.3 warm-up |

---

### 2.2 — Seuil de cranking : `c_tco_n_mff_cst`

### 📋 Tables à modifier

| Paramètre | Adresse | Équation | Note |
|---|---|---|---|
| `c_tco_n_mff_cst` | 0x44F2F | 0.75 × X − 48 | Seuil TCO activation enrichissements cranking |

### 🔨 Procédure

```
TunerPro → c_tco_n_mff_cst → raw 87 → 97
```

<a id="pencil-tco"></a>

### ✏️ Avant / Après

| | ◀ Stock | ▶ E85 |
|---|---|---|
| Raw | 87 | **97** |
| °C | 17.25 °C | **25.00 °C** |

### ✅ Vérification

Les enrichissements cranking restent actifs jusqu'à 25°C TCO (observable via ISTA pendant le warm-up).

---

### 2.3 — Warm-up lambda : `ip_fac_lamb_wup`

### 📋 Tables à modifier

| Paramètre | Adresse | Structure | Axes | Équation |
|---|---|---|---|---|
| `ip_fac_lamb_wup` | 0x42764 | 6×6 | X = MAF (65–500 mg/stk), Y = RPM (704–3008 tr/min) | direct (facteur −) |

> Attention : axes MAF × RPM, **pas** TCO. Enrichit les basses charges pendant que la sonde lambda monte en température.

### 🔨 Procédure

Augmenter les cellules basse charge / bas régime. Cellules hautes charges (MAF > 400) restent à 1.000.

<a id="pencil-wup"></a>

### ✏️ Avant / Après

AVANT (stock) :
```
MAF →           65    100    200    300    400    500 mg/stk
 704 rpm :    1.000  1.000  1.000  1.000  1.000  1.000
1216 rpm :    1.000  1.000  1.000  1.000  1.000  1.000
1760 rpm :    1.000  1.000  1.000  1.000  1.000  1.000
2016 rpm :    1.000  1.000  1.000  1.000  1.000  1.000
2496 rpm :    1.000  1.000  1.000  1.000  1.000  1.000
3008 rpm :    1.000  1.000  1.000  1.000  1.000  1.000
```

APRÈS (E85) :
```
MAF →           65    100    200    300    400    500 mg/stk
 704 rpm :    1.08   1.08   1.05   1.03   1.00   1.00
1216 rpm :    1.08   1.07   1.05   1.03   1.00   1.00
1760 rpm :    1.07   1.06   1.04   1.03   1.00   1.00
2016 rpm :    1.06   1.05   1.04   1.03   1.00   1.00
2496 rpm :    1.05   1.05   1.03   1.03   1.00   1.00
3008 rpm :    1.03   1.03   1.03   1.00   1.00   1.00
```

### ✅ Vérification

| Moment | ✅ Cible | ⚠️ Si hors cible |
|---|---|---|
| 30 premières secondes | STFT −10% à +15% (normal à froid) | STFT oscillent → +0.02 sur cellules basse charge |
| Après 2 min warm-up | STFT **−5% à +5%** | Itérer |

### 2.4 — Procédure de test démarrage à froid

1. **Préparez votre scanner OBD2** : Enregistrez TCO, STFT, RPM en continu
2. **Moteur froid (< 20°C)** : Clé, ne pas appuyer sur la pédale d'accélérateur
3. **Critères de réussite :**
   - Démarrage en ≤ 3 tours de vilebrequin
   - Ralenti initial : 800–1200 tr/min acceptable, diminue progressivement
   - Après 30 sec : STFT entre −10% et +10%
4. **Si démarrage difficile (> 5 tours) :** +15% sur toutes les valeurs de cranking
5. **Si le moteur cale après démarrage :** Augmentez ip_fac_lamb_wup de +0.10 à 20–40°C

### ⚠️ Conseils Pratiques Démarrage Froid

- **Ne touchez JAMAIS la pédale avant démarrage** : le MSV70 désactive l'enrichissement cranking si la pédale est enfoncée (Full Load Cutoff)
- **Batterie impeccable obligatoire** : l'E85 froid exige plus de tours pour démarrer → batterie faible = démarrage impossible
- **Bougies neuves** : gap 0.65–0.70 mm (vs 0.75–0.80 mm stock)
- **Hiver < 0°C :** Envisagez un kit dual-fuel ou l'ajout d'essence 95 dans le réservoir (20% essence suffit pour améliorer le démarrage)

---


---

## Récapitulatif — Valeurs Avant / Après

### ② ip_fac_lamb_wup — Warm-up post-démarrage @ 0x42764 (6×6, MAF × RPM)

> Stock = 1.000 partout. Enrichit les basses charges pendant que la sonde monte en température.

```
AVANT (stock) :
MAF →           65    100    200    300    400    500 mg/stk
 704 rpm :    1.000  1.000  1.000  1.000  1.000  1.000
1216 rpm :    1.000  1.000  1.000  1.000  1.000  1.000
1760 rpm :    1.000  1.000  1.000  1.000  1.000  1.000
2016 rpm :    1.000  1.000  1.000  1.000  1.000  1.000
2496 rpm :    1.000  1.000  1.000  1.000  1.000  1.000
3008 rpm :    1.000  1.000  1.000  1.000  1.000  1.000

APRÈS (E85) :
MAF →           65    100    200    300    400    500 mg/stk
 704 rpm :    1.08   1.08   1.05   1.03   1.00   1.00
1216 rpm :    1.08   1.07   1.05   1.03   1.00   1.00
1760 rpm :    1.07   1.06   1.04   1.03   1.00   1.00
2016 rpm :    1.06   1.05   1.04   1.03   1.00   1.00
2496 rpm :    1.05   1.05   1.03   1.03   1.00   1.00
3008 rpm :    1.03   1.03   1.03   1.00   1.00   1.00
```

---

### ③ ip_mff_cst_opm_1 — Cranking Valvetronic @ 0x437DC (3×8, mg/stk, cible E70)

```
AVANT (stock) :
TCO (°C) :  -30.0   -20.2    -9.8    0.0   17.2   30.0   60.0   90.0
RPM  80  :  447.7   350.6   261.3  189.3  102.2   71.4   56.2   49.6
RPM 320  :  320.3   260.7   202.1  152.1   87.9   61.3   46.5   39.1
RPM 920  :  194.4   175.1   146.0  112.9   68.4   48.6   36.5   33.0

APRÈS (E70 — facteurs ×1.35 à ×2.00 selon TCO) :
TCO (°C) :  -30.0   -20.2    -9.8    0.0   17.2   30.0   60.0   90.0
RPM  80  :  895.4   631.1   431.1  293.4  138.0   85.7   61.8   52.1
RPM 320  :  640.6   469.3   333.5  235.8  118.7   73.6   51.2   41.1
RPM 920  :  388.8   315.2   240.9  175.0   92.3   58.3   40.2   34.7
```

---

### ④ ip_mff_cst_opm_2 — Cranking papillonné @ 0x4380C (3×8, mg/stk, cible E70)

```
AVANT (stock) :
TCO (°C) :  -30.0   -20.2    -9.8    0.0   17.2   30.0   60.0   90.0
RPM  80  :  731.1   527.0   362.8  245.0  138.2  102.1   67.8   49.6
RPM 320  :  546.2   415.6   297.0  201.8  106.4   82.3   57.0   39.1
RPM 920  :  363.0   281.4   215.8  159.0   84.1   65.8   47.0   34.5

APRÈS (E70 — mêmes facteurs) :
TCO (°C) :  -30.0   -20.2    -9.8    0.0   17.2   30.0   60.0   90.0
RPM  80  : 1462.2   948.6   598.6  379.8  186.6  122.5   74.6   52.1
RPM 320  : 1092.4   748.1   490.1  312.8  143.6   98.8   62.7   41.1
RPM 920  :  726.0   506.5   356.1  246.5  113.5   79.0   51.7   36.2
```

---
