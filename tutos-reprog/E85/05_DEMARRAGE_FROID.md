# §5 — Démarrage à Froid (Cranking)

> L'éthanol s'évapore difficilement sous 20°C (ébullition 78°C vs −40°C essence). Le moteur a besoin de **1.5× à 2× plus** de masse carburant au cranking. Deux paramètres à régler : tables de cranking + seuil TCO.

---

## ip_mff_cst_opm_1 et ip_mff_cst_opm_2

| Paramètre | Adresse | Structure | Équation | Axes |
|---|---|---|---|---|
| `ip_mff_cst_opm_1` | 0x437DC | 3×8 | 0.021195 × X | X = TCO (°C), Y = RPM démarreur |
| `ip_mff_cst_opm_2` | 0x4380C | 3×8 | 0.021195 × X | X = TCO (°C), Y = RPM démarreur |

`opm_1` = Valvetronic actif. `opm_2` = mode papillonné (Valvetronic désactivé). Les deux tables doivent être modifiées — elles sont sélectionnées selon le mode de fonctionnement du moteur.

**Facteurs E85 à appliquer colonne par colonne :**

| TCO (°C) | −30 | −20 | −10 | 0 | +17 | +30 | +60 | +90 |
|---|---|---|---|---|---|---|---|---|
| Facteur | ×2.00 | ×1.80 | ×1.65 | ×1.55 | ×1.35 | ×1.20 | ×1.10 | ×1.05 |

**ip_mff_cst_opm_1 @ 0x437DC :**

```
STOCK :
TCO (°C) :  -30.0   -20.2    -9.8    0.0   17.2   30.0   60.0   90.0
RPM  80  :  447.7   350.6   261.3  189.3  102.2   71.4   56.2   49.6
RPM 320  :  320.3   260.7   202.1  152.1   87.9   61.3   46.5   39.1
RPM 920  :  194.4   175.1   146.0  112.9   68.4   48.6   36.5   33.0

OBJECTIF E85 :
TCO (°C) :  -30.0   -20.2    -9.8    0.0   17.2   30.0   60.0   90.0
RPM  80  :  895.4   631.1   431.1  293.4  138.0   85.7   61.8   52.1
RPM 320  :  640.6   469.3   333.5  235.8  118.7   73.6   51.2   41.1
RPM 920  :  388.8   315.2   240.9  175.0   92.3   58.3   40.2   34.7
```

**ip_mff_cst_opm_2 @ 0x4380C :**

```
STOCK :
TCO (°C) :  -30.0   -20.2    -9.8    0.0   17.2   30.0   60.0   90.0
RPM  80  :  731.1   527.0   362.8  245.0  138.2  102.1   67.8   49.6
RPM 320  :  546.2   415.6   297.0  201.8  106.4   82.3   57.0   39.1
RPM 920  :  363.0   281.4   215.8  159.0   84.1   65.8   47.0   34.5

OBJECTIF E85 :
TCO (°C) :  -30.0   -20.2    -9.8    0.0   17.2   30.0   60.0   90.0
RPM  80  : 1462.2   948.6   598.6  379.8  186.6  122.5   74.6   52.1
RPM 320  : 1092.4   748.1   490.1  312.8  143.6   98.8   62.7   41.1
RPM 920  :  726.0   506.5   356.1  246.5  113.5   79.0   51.7   36.2
```

> `opm_2` a des valeurs nettement plus élevées à froid — les mêmes facteurs multiplicatifs s'appliquent aux deux tables.

**Vérification :**

| Critère | OK | Si raté |
|---|---|---|
| Démarrage froid, sans pédale | ≤ 3 tours | > 5 tours → cranking +15%, reflasher |
| Ralenti initial | 800–1200 RPM, décroissant | Instable → vérifier warm-up lambda ([tuto warmup §03](../../warmup/03_LAMBDA_CHAUFFE.md)) |

---

## c_tco_n_mff_cst

| Paramètre | Adresse | Équation |
|---|---|---|
| `c_tco_n_mff_cst` | 0x44F2F | 0.75 × raw − 48 |

Seuil TCO en dessous duquel les tables de cranking enrichies s'appliquent.

| | Stock | E85 |
|---|---|---|
| Raw | 87 | **97** |
| °C | 17.25°C | **25.00°C** |

L'éthanol nécessite un enrichissement cranking jusqu'à ~25°C contre ~17°C pour l'essence.

**Conseils pratiques :**
- Ne jamais appuyer sur la pédale avant démarrage — le MSV70 désactive l'enrichissement cranking si la pédale est enfoncée (Full Load Cutoff)
- Batterie en bon état obligatoire — l'E85 froid exige plus de tours, une batterie faible rend le démarrage impossible
- Bougies neuves, gap 0.65–0.70 mm (vs 0.75–0.80 mm stock)

---

## ip_mff_lgrd_ast — Enrichissement after-start (phase post-démarrage)

| Paramètre | Adresse | Structure | Équation | Axe |
|---|---|---|---|---|
| `ip_mff_lgrd_ast` | (voir XDF) | Courbe 1D | 0.021195 × raw | f(RPM démarreur, 0–800 RPM) |

**Rôle :** Masse de carburant additionnelle injectée pendant la phase "after-start" — les premières secondes après que le moteur a pris vie mais avant que le régime soit stabilisé (typiquement 0–800 RPM en montée). C'est différent du cranking (moteur non démarré) et du warm-up (moteur stabilisé). Sur E85, cette zone est critique : si l'enrichissement after-start est insuffisant, le moteur cale après avoir démarré (il "prend" mais s'arrête immédiatement).

**Facteur E85 :** Appliquer le même facteur que les tables cranking selon la TCO de démarrage — typiquement ×1.35 à ×1.65 selon la température. La logique est identique : l'éthanol s'évapore moins bien, il faut plus de masse pour maintenir la combustion pendant la montée en régime.

### ✏️ Avant / Après — `ip_mff_lgrd_ast`

| Condition | ◀ Valeur stock | ▶ Valeur E85 | Facteur |
|---|---|---|---|
| TCO ≤ 0°C | Valeur stock | **Stock × 1.55–1.65** | Forte vaporisation déficiente |
| TCO 0–17°C | Valeur stock | **Stock × 1.35–1.45** | Vaporisation partielle |
| TCO 17–30°C | Valeur stock | **Stock × 1.20–1.30** | Légèrement insuffisant |
| TCO > 30°C | Valeur stock | **Stock × 1.10–1.15** | Quasi normal |
| TCO > 60°C | Valeur stock | **Stock (inchangé)** | Vaporisation correcte |

> Symptôme d'after-start insuffisant : démarrage réussi (moteur "part") puis calage dans les 2 premières secondes. Distinct d'un cranking insuffisant (moteur ne démarre pas du tout).

### ✅ Vérification

| Critère | ✅ OK | ⚠️ Si raté |
|---|---|---|
| Démarrage froid < 5°C | Moteur stable dès 1 s après démarrage | Cale dans les 3 s → ip_mff_lgrd_ast +15% sur colonnes froides |
| Montée régime après démarrage | 0 → 800 RPM lisse | Chute transitoire → facteur insuffisant |
