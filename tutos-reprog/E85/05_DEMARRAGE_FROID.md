# Démarrage à froid (Cranking)

L'éthanol s'évapore difficilement sous 25°C (ébullition à 78°C vs −40°C pour l'essence). Le moteur a besoin de **×1.35 à ×2.00 de masse carburant** au cranking selon la température. Trois paramètres à modifier : les deux tables de cranking + le seuil TCO. Un quatrième paramètre (after-start) gère la phase immédiatement post-démarrage.

**Règles absolues :**
- Ne jamais appuyer sur la pédale avant démarrage — le MSV70 désactive l'enrichissement cranking si la pédale est enfoncée (Full Load Cutoff)
- Batterie en bon état obligatoire — l'E85 froid exige plus de tours, une batterie faible rend le démarrage impossible
- Bougies neuves, gap 0.65–0.70 mm (vs 0.75–0.80 mm stock)

---

<a id="p1"></a>
## ① `ip_mff_cst_opm_1` — Masse carburant cranking, mode Valvetronic

| Champ | Valeur |
|---|---|
| Adresse | 0x437DC |
| Type | Map 3×8 |
| Unité | mg/stk |
| Axes | X = TCO (°C), Y = RPM démarreur |

**Rôle :** Masse de carburant injectée pendant la phase de cranking (moteur en rotation avant premier allumage), en mode Valvetronic. C'est le paramètre principal qui détermine si le moteur démarre ou non sur E85 froid. Les valeurs doivent être multipliées colonne par colonne selon la température — le facteur varie de ×2.00 à −30°C à ×1.05 à 90°C.

**◀ Avant — Stock (mg/stk)**

| RPM (tr/min) \ TCO (°C) | −30.0 | −20.2 | −9.8 | 0.0 | 17.2 | 30.0 | 60.0 | 90.0 |
|---|---|---|---|---|---|---|---|---|
| 80 | 447.7 | 350.6 | 261.3 | 189.3 | 102.2 | 71.4 | 56.2 | 49.6 |
| 320 | 320.3 | 260.7 | 202.1 | 152.1 | 87.9 | 61.3 | 46.5 | 39.1 |
| 920 | 194.4 | 175.1 | 146.0 | 112.9 | 68.4 | 48.6 | 36.5 | 33.0 |

**✏️ Facteurs E85 à appliquer par colonne TCO :**

| TCO (°C) | −30 | −20 | −10 | 0 | +17 | +30 | +60 | +90 |
|---|---|---|---|---|---|---|---|---|
| Facteur (×) | 2.00 | 1.80 | 1.65 | 1.55 | 1.35 | 1.20 | 1.10 | 1.05 |

**✅ Après — E85 (mg/stk)**

| RPM (tr/min) \ TCO (°C) | −30.0 | −20.2 | −9.8 | 0.0 | 17.2 | 30.0 | 60.0 | 90.0 |
|---|---|---|---|---|---|---|---|---|
| 80 | **895.4** | **631.1** | **431.1** | **293.4** | **138.0** | **85.7** | **61.8** | **52.1** |
| 320 | **640.6** | **469.3** | **333.5** | **235.8** | **118.7** | **73.6** | **51.2** | **41.1** |
| 920 | **388.8** | **315.2** | **240.9** | **175.0** | **92.3** | **58.3** | **40.2** | **34.7** |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Démarrage froid (TCO < 10°C), sans pédale | ≤ 3 tours | > 5 tours → cranking +15% sur colonnes froides |
| Démarrage froid (TCO 10–25°C) | ≤ 3 tours | > 3 tours → cranking +10% colonnes concernées |
| Ralenti initial après démarrage | 800–1200 RPM, décroissant | Instable → vérifier warm-up lambda §08 |

---

<a id="p2"></a>
## ② `ip_mff_cst_opm_2` — Masse carburant cranking, mode papillonné (GD)

| Champ | Valeur |
|---|---|
| Adresse | 0x4380C |
| Type | Map 3×8 |
| Unité | mg/stk |
| Axes | X = TCO (°C), Y = RPM démarreur |

**Rôle :** Même rôle qu'opm_1 mais pour le mode papillonné (Gedrosselt, Valvetronic désactivé). Actif au démarrage froid quand le Valvetronic n'est pas encore opérationnel. Les valeurs stock opm_2 sont nettement plus élevées à froid que opm_1. Les mêmes facteurs multiplicatifs s'appliquent aux deux tables. Si seule opm_1 est modifiée, le démarrage reste difficile si le moteur démarre en mode GD.

**◀ Avant — Stock (mg/stk)**

| RPM (tr/min) \ TCO (°C) | −30.0 | −20.2 | −9.8 | 0.0 | 17.2 | 30.0 | 60.0 | 90.0 |
|---|---|---|---|---|---|---|---|---|
| 80 | 731.1 | 527.0 | 362.8 | 245.0 | 138.2 | 102.1 | 67.8 | 49.6 |
| 320 | 546.2 | 415.6 | 297.0 | 201.8 | 106.4 | 82.3 | 57.0 | 39.1 |
| 920 | 363.0 | 281.4 | 215.8 | 159.0 | 84.1 | 65.8 | 47.0 | 34.5 |

**✏️ Mêmes facteurs E85 que opm_1 par colonne TCO :**

| TCO (°C) | −30 | −20 | −10 | 0 | +17 | +30 | +60 | +90 |
|---|---|---|---|---|---|---|---|---|
| Facteur (×) | 2.00 | 1.80 | 1.65 | 1.55 | 1.35 | 1.20 | 1.10 | 1.05 |

**✅ Après — E85 (mg/stk)**

| RPM (tr/min) \ TCO (°C) | −30.0 | −20.2 | −9.8 | 0.0 | 17.2 | 30.0 | 60.0 | 90.0 |
|---|---|---|---|---|---|---|---|---|
| 80 | **1462.2** | **948.6** | **598.6** | **379.8** | **186.6** | **122.5** | **74.6** | **52.1** |
| 320 | **1092.4** | **748.1** | **490.1** | **312.8** | **143.6** | **98.8** | **62.7** | **41.1** |
| 920 | **726.0** | **506.5** | **356.1** | **246.5** | **113.5** | **79.0** | **51.7** | **36.2** |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Démarrage froid (TCO < 10°C), sans pédale | ≤ 3 tours | > 5 tours → vérifier opm_2 modifié (mode GD actif) |
| Démarrage froid (10–25°C) | ≤ 3 tours | Difficile → cranking opm_2 +10% colonnes concernées |

---

<a id="p3"></a>
## ③ `c_tco_n_mff_cst` — Seuil TCO activation cranking enrichi

| Champ | Valeur |
|---|---|
| Adresse | 0x44F2F |
| Type | Constante scalaire |
| Unité | °C |

**Rôle :** Température de liquide de refroidissement (TCO) en dessous de laquelle les tables de cranking enrichies opm_1 et opm_2 s'appliquent. Au-dessus de ce seuil, le calculateur utilise les valeurs stock essence. Sur E85, l'éthanol nécessite un enrichissement cranking jusqu'à ~25°C (contre ~17°C pour l'essence) car sa vaporisation reste déficiente jusqu'à cette température. Sans ce seuil relevé, le moteur démarre en mélange pauvre par temps frais.

**Avant / Après :**

| | ◀ Stock VB67774 | ✅ E85 |
|---|---|---|
| `c_tco_n_mff_cst` | 17.25 °C | **25.00 °C** |

Autres valeurs possibles selon la rigueur hivernale :

| Seuil TCO | Usage |
|---|---|
| 20.25 °C | Minimum E85 (hivers doux) |
| **25.00 °C** | **Recommandé** |
| 29.25 °C | Hivers rigoureux (< −10°C fréquents) |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Démarrage TCO 18–25°C | < 3 tours | > 3 tours → seuil encore trop bas, passer à 29.25 °C |
| Démarrage TCO > 30°C (moteur chaud) | Immédiat | STFT riche → seuil trop haut, réduire |

---

<a id="p4"></a>
## ④ `ip_mff_lgrd_ast` — Enrichissement after-start (phase post-démarrage)

| Champ | Valeur |
|---|---|
| Adresse | (voir XDF) |
| Type | Courbe 1D f(RPM) |
| Unité | facteur multiplicateur (×) |
| Axes | X = RPM (0–800 tr/min post-démarrage) |

**Rôle :** Masse de carburant additionnelle injectée pendant la phase after-start — les premières secondes après que le moteur a pris vie mais avant que le régime soit stabilisé. Distinct du cranking (moteur en rotation sans allumage) et du warm-up (régime stabilisé). Sur E85, si l'enrichissement after-start est insuffisant, le moteur démarre puis cale dans les 2 premières secondes (il "prend" mais s'arrête immédiatement).

**Avant / Après :**

| Condition TCO | ◀ Stock | ✅ E85 | Facteur |
|---|---|---|---|
| TCO ≤ 0°C | Valeur stock | **Stock × 1.55–1.65** | Vaporisation très déficiente |
| TCO 0–17°C | Valeur stock | **Stock × 1.35–1.45** | Vaporisation partielle |
| TCO 17–30°C | Valeur stock | **Stock × 1.20–1.30** | Légèrement insuffisant |
| TCO 30–60°C | Valeur stock | **Stock × 1.10–1.15** | Quasi normal |
| TCO > 60°C | Valeur stock | Inchangé | Vaporisation correcte |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Démarrage froid (< 5°C) | Moteur stable dès la 1ère seconde | Cale dans les 3 s → +15% colonnes froides |
| Montée régime après démarrage | 0 → 800 RPM lisse, pas de chute | Chute transitoire RPM → facteur insuffisant |
| Distinction cranking / after-start | — | Moteur ne part pas = cranking / part puis cale = after-start |
