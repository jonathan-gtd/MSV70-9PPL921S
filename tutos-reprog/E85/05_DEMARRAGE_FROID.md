# Démarrage à froid (Cranking)

L'éthanol s'évapore très difficilement à froid (ébullition à 78°C, chaleur latente ×2.7 vs essence). Il faut **×1.47 à +17°C** (stoech seul) mais **×3.00 à −30°C** (stoech + atomisation + film mural cranking).

`c_ti_as_global = 1.47` couvre les colonnes chaudes (+17°C et plus). Les colonnes froides doivent être multipliées **en plus** dans ip_mff_cst (facteur résiduel = facteur total ÷ 1.47).

**Facteurs par colonne TCO :**

| TCO (°C) | −30 | −20 | −10 | 0 | +17 | +30 | +60 | +90 |
|---|---|---|---|---|---|---|---|---|
| Facteur total E85 | ×3.00 | ×2.50 | ×2.00 | ×1.75 | ×1.47 | ×1.30 | ×1.15 | ×1.05 |
| c_ti_as_global | ×1.47 | ×1.47 | ×1.47 | ×1.47 | ×1.47 | ×1.47 | ×1.47 | ×1.47 |
| **Résiduel ip_mff_cst** | **×2.04** | **×1.70** | **×1.36** | **×1.19** | **×1.00** | **×1.00** | **×1.00** | **×1.00** |

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

**◀ Avant — Stock S7581293 (mg/stk)**

| RPM \ TCO (°C) | −30.0 | −20.2 | −9.8 | 0.0 | 17.2 | 30.0 | 60.0 | 90.0 |
|---|---|---|---|---|---|---|---|---|
| 80 | 373.2 | 301.2 | 231.7 | 173.4 | 96.8 | 70.0 | 56.2 | 49.6 |
| 320 | 267.0 | 224.0 | 179.2 | 139.3 | 83.2 | 60.2 | 46.5 | 39.1 |
| 920 | 162.1 | 150.5 | 129.4 | 103.4 | 64.8 | 47.6 | 36.5 | 33.0 |

**✅ Après — E85 (mg/stk) — facteurs résiduels appliqués colonne par colonne**

| RPM \ TCO (°C) | −30.0 | −20.2 | −9.8 | 0.0 | 17.2 | 30.0 | 60.0 | 90.0 |
|---|---|---|---|---|---|---|---|---|
| 80 | **761.3** | **512.0** | **315.1** | **206.3** | 96.8 | 70.0 | 56.2 | 49.6 |
| 320 | **544.7** | **380.8** | **243.7** | **165.8** | 83.2 | 60.2 | 46.5 | 39.1 |
| 920 | **330.7** | **255.9** | **175.9** | **123.0** | 64.8 | 47.6 | 36.5 | 33.0 |

> Les colonnes +17°C à +90°C sont inchangées — c_ti_as_global × 1.47 couvre le besoin.

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Démarrage froid (TCO < 10°C), sans pédale | ≤ 3 tours | > 5 tours → cranking +15% sur colonnes froides |
| Démarrage froid (TCO 10–25°C) | ≤ 3 tours | > 3 tours → vérifier c_tco_n_mff_cst |

---

<a id="p2"></a>
## ② `ip_mff_cst_opm_2` — Masse carburant cranking, mode papillonné (GD)

| Champ | Valeur |
|---|---|
| Adresse | 0x4380C |
| Type | Map 3×8 |
| Unité | mg/stk |
| Axes | X = TCO (°C), Y = RPM démarreur |

**Rôle :** Même rôle qu'opm_1 pour le mode papillonné (Valvetronic désactivé). Actif au démarrage froid quand le Valvetronic n'est pas encore opérationnel. Mêmes facteurs résiduels.

**◀ Avant — Stock S7581293 (mg/stk)**

| RPM \ TCO (°C) | −30.0 | −20.2 | −9.8 | 0.0 | 17.2 | 30.0 | 60.0 | 90.0 |
|---|---|---|---|---|---|---|---|---|
| 80 | 609.3 | 452.7 | 321.6 | 224.4 | 130.9 | 100.2 | 67.8 | 49.6 |
| 320 | 455.2 | 357.1 | 263.3 | 184.8 | 100.8 | 80.7 | 57.0 | 39.1 |
| 920 | 302.6 | 241.8 | 191.3 | 145.6 | 79.7 | 64.6 | 47.0 | 34.5 |

**✅ Après — E85 (mg/stk)**

| RPM \ TCO (°C) | −30.0 | −20.2 | −9.8 | 0.0 | 17.2 | 30.0 | 60.0 | 90.0 |
|---|---|---|---|---|---|---|---|---|
| 80 | **1243.0** | **769.6** | **437.4** | **267.0** | 130.9 | 100.2 | 67.8 | 49.6 |
| 320 | **928.6** | **607.1** | **358.1** | **219.9** | 100.8 | 80.7 | 57.0 | 39.1 |
| 920 | **617.3** | **411.1** | **260.2** | **173.3** | 79.7 | 64.6 | 47.0 | 34.5 |

---

<a id="p3"></a>
## ③ `c_tco_n_mff_cst` — Seuil TCO activation cranking enrichi

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

**Non modifié.** `c_ti_as_global × stock ip_ti_cast` : exemple à −30°C → stock 1.375 × 1.47 = **2.02** (cible E85 ≈ ×2.00 ✓).

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
| Démarrage froid (TCO < 10°C), sans pédale | ≤ 3 tours | > 5 tours → cranking +15% colonnes froides ip_mff_cst |
| Démarrage froid (TCO 10–25°C) | ≤ 3 tours | > 3 tours → vérifier c_tco_n_mff_cst |
| Part puis cale dans les 3 s | — | after-start → si persistant, augmenter ip_ti_cast froid |
| Ralenti initial après démarrage | 800–1200 RPM, décroissant | Instable → vérifier warm-up lambda §08 |
