# Démarrage à froid (Cranking)

L'éthanol s'évapore difficilement sous 25°C (ébullition à 78°C vs −40°C pour l'essence). Le moteur a besoin de **×1.35 à ×2.00 de masse carburant** au cranking selon la température.

`c_ti_as_global = 1.47` s'applique pendant le cranking — les tables stock S7581293 × 1.47 donnent l'enrichissement E85 nécessaire. **ip_mff_cst_opm_1 et ip_mff_cst_opm_2 ne sont donc pas modifiées.**

Un seul paramètre cold start à modifier : le **seuil TCO** d'activation des tables cranking.

**Règles absolues :**
- Ne jamais appuyer sur la pédale avant démarrage — le MSV70 désactive l'enrichissement cranking si la pédale est enfoncée (Full Load Cutoff)
- Batterie en bon état obligatoire — l'E85 froid exige plus de tours, une batterie faible rend le démarrage impossible
- Bougies neuves, gap 0.65–0.70 mm (vs 0.75–0.80 mm stock)

---

<a id="p1"></a>
## ① `ip_mff_cst_opm_1` — Masse carburant cranking, mode Valvetronic

**Non modifié.** `c_ti_as_global × stock` = enrichissement E85 au cranking.

**◀ Stock S7581293 (référence, mg/stk)**

| RPM \ TCO (°C) | −30.0 | −20.2 | −9.8 | 0.0 | 17.2 | 30.0 | 60.0 | 90.0 |
|---|---|---|---|---|---|---|---|---|
| 80 | 373.2 | 301.2 | 231.7 | 173.4 | 96.8 | 70.0 | 56.2 | 49.6 |
| 320 | 267.0 | 224.0 | 179.2 | 139.3 | 83.2 | 60.2 | 46.5 | 39.1 |
| 920 | 162.1 | 150.5 | 129.4 | 103.4 | 64.8 | 47.6 | 36.5 | 33.0 |

**Effective après c_ti_as_global × 1.47 (mg/stk)**

| RPM \ TCO (°C) | −30.0 | −20.2 | −9.8 | 0.0 | 17.2 | 30.0 | 60.0 | 90.0 |
|---|---|---|---|---|---|---|---|---|
| 80 | **548.6** | **442.8** | **340.6** | **254.9** | **142.3** | **102.9** | **82.6** | **72.9** |
| 320 | **392.5** | **329.3** | **263.4** | **204.8** | **122.3** | **88.5** | **68.4** | **57.5** |
| 920 | **238.3** | **221.2** | **190.2** | **152.0** | **95.3** | **70.0** | **53.6** | **48.5** |

---

<a id="p2"></a>
## ② `ip_mff_cst_opm_2` — Masse carburant cranking, mode papillonné (GD)

**Non modifié.** Même principe qu'opm_1.

**◀ Stock S7581293 (référence, mg/stk)**

| RPM \ TCO (°C) | −30.0 | −20.2 | −9.8 | 0.0 | 17.2 | 30.0 | 60.0 | 90.0 |
|---|---|---|---|---|---|---|---|---|
| 80 | 609.3 | 452.7 | 321.6 | 224.4 | 130.9 | 100.2 | 67.8 | 49.6 |
| 320 | 455.2 | 357.1 | 263.3 | 184.8 | 100.8 | 80.7 | 57.0 | 39.1 |
| 920 | 302.6 | 241.8 | 191.3 | 145.6 | 79.7 | 64.6 | 47.0 | 34.5 |

**Effective après c_ti_as_global × 1.47 (mg/stk)**

| RPM \ TCO (°C) | −30.0 | −20.2 | −9.8 | 0.0 | 17.2 | 30.0 | 60.0 | 90.0 |
|---|---|---|---|---|---|---|---|---|
| 80 | **895.7** | **665.5** | **472.8** | **329.9** | **192.4** | **147.3** | **99.7** | **72.9** |
| 320 | **669.1** | **524.9** | **387.0** | **271.7** | **148.2** | **118.6** | **83.8** | **57.5** |
| 920 | **444.8** | **355.4** | **281.2** | **214.0** | **117.2** | **94.9** | **69.1** | **50.7** |

---

<a id="p3"></a>
## ③ `c_tco_n_mff_cst` — Seuil TCO activation cranking enrichi

**Modifier.** Ce seuil contrôle quand les tables cranking s'activent — il n'est pas affecté par `c_ti_as_global`. Sur E85, l'éthanol nécessite l'enrichissement cranking jusqu'à ~25°C (contre ~17°C pour l'essence).

**Avant / Après :**

| | ◀ Stock S7581293 | ✅ E85 |
|---|---|---|
| `c_tco_n_mff_cst` | **17.25 °C** | **25.00 °C** |

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
## ④ `ip_ti_cast_opm_1` — Enrichissement after-start, mode Valvetronic

**Non modifié.** `c_ti_as_global × stock ip_ti_cast` fournit l'enrichissement post-démarrage E85. Exemple à −30°C : stock 1.375 × 1.47 = **2.02** (cible E85 ≈ ×2.00 ✓).

Si le moteur démarre puis cale dans les 2 premières secondes malgré le cranking OK → augmenter uniquement à ce moment-là.

**◀ Stock S7581293 (facteur, pour référence)**

| TCO\_adm \ TCO\_liq | −30.0 | −20.2 | −9.8 | 0.0 | 9.8 | 17.2 | 30.0 | 45.0 | 69.8 | 84.0 |
|---|---|---|---|---|---|---|---|---|---|---|
| **−9.8°C** | 1.375 | 1.188 | 1.078 | 0.875 | 0.859 | 0.750 | 0.625 | 0.500 | 0.359 | 0.234 |
| **60.0°C** | 1.375 | 1.172 | 1.031 | 0.813 | 0.813 | 0.672 | 0.547 | 0.438 | 0.313 | 0.219 |

---

<a id="p5"></a>
## ⑤ `ip_ti_cast_opm_2` — Enrichissement after-start, mode papillonné (GD)

**Non modifié.** Même principe qu'opm_1.

**Vérification cold start globale :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Démarrage froid (TCO < 10°C), sans pédale | ≤ 3 tours | > 5 tours → cranking +15% sur colonnes froides dans ip_mff_cst |
| Démarrage froid (TCO 10–25°C) | ≤ 3 tours | > 3 tours → vérifier c_tco_n_mff_cst |
| Part puis cale dans les 3 s | — | after-start → si persistant, augmenter ip_ti_cast froid |
| Ralenti initial après démarrage | 800–1200 RPM, décroissant | Instable → vérifier warm-up lambda §08 |
