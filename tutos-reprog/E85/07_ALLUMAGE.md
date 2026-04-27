# Allumage — Avance et délai WOT

L'éthanol a un indice d'octane de 104–108 RON (vs 95 SP95) — il résiste mieux à la détonation et permet d'augmenter l'avance. Calibration **E60 (101 RON)** = pire carburant légal hivernal : zéro risque quelle que soit la saison. Gain : **+5 à +12% de puissance**.

**Règle absolue : progressivité.** Un seul degré de trop = cliquetis = casse moteur. Valider 50 km par palier, jamais plus de +2.5° sur un N52 atmosphérique.

| Carburant | RON | Avance max exploitable |
|---|:---:|:---:|
| Essence 95 | 95 | Référence |
| E60 (hiver) | ~101 | +2.5° max (stratégie retenue) |
| E70 | ~104 | +3–4° |
| E85 | ~108 | +4–5° |

---

## OBLIGATOIRE

<a id="p1"></a>
## ① `ip_iga_bas_max_knk__n__maf` — Plafond avance anti-cliquetis f(MAF × RPM)

| Champ | Valeur |
|---|---|
| Adresse | 0x4323A |
| Type | Map 8×8 |
| Unité | °CRK avant PMH |
| Axes | X = MAF (0.55–2.25 mg/stk), Y = RPM (608–7008 tr/min) |

**Rôle :** Table principale d'avance à modifier pour bénéficier de l'octane supérieur de l'E85. C'est le plafond knock-limited — l'avance effective ne peut pas dépasser ces valeurs. En augmentant les cellules de haute charge (colonnes MAF élevé), on autorise le modèle de couple à demander davantage d'avance. Le knock control reste actif et reculera si cliquetis détecté.

**Procédure d'augmentation par zones :**

| Zone | MAF | Phase 1 | Phase 2 | Phase 3 |
|---|---|---|---|---|
| Ralenti / très faible charge | < 0.65 mg/stk | +0° | +0° | +0° |
| Charge moyenne | 1.0–1.5 mg/stk | +0.5° | +1.0° | — |
| Haute charge / WOT | > 1.5 mg/stk | +1.0° | +2.0° | +2.5° |

**◀ Avant — Stock (°CRK avant PMH)**

| RPM (tr/min) \ MAF (mg/stk) | 0.55 | 0.64 | 1.02 | 1.31 | 1.55 | 1.78 | 2.01 | 2.25 |
|---|---|---|---|---|---|---|---|---|
| 608 | +10.50 | +9.38 | +1.88 | −7.12 | −12.00 | −13.88 | −15.00 | −15.00 |
| 1504 | +16.88 | +15.38 | +9.75 | +5.25 | −1.12 | −6.00 | −8.62 | −9.38 |
| 2016 | +21.75 | +20.62 | +15.38 | +10.12 | +6.00 | +3.00 | 0.00 | −2.25 |
| 3008 | +25.88 | +26.25 | +21.00 | +15.75 | +10.50 | +7.12 | +4.88 | +4.12 |
| 4000 | +35.62 | +34.88 | +28.88 | +20.62 | +15.75 | +12.75 | +11.62 | +7.12 |
| 4992 | +38.62 | +38.25 | +33.00 | +23.25 | +20.25 | +16.50 | +13.12 | +12.00 |
| 6016 | +43.12 | +39.75 | +33.38 | +23.25 | +19.88 | +18.00 | +15.38 | +13.50 |
| 7008 | +43.50 | +39.38 | +33.38 | +24.00 | +20.62 | +19.12 | +17.62 | +16.88 |

**✏️ Delta à ajouter par cellule (°CRK) — objectif E60-safe**

| RPM (tr/min) \ MAF (mg/stk) | 0.55 | 0.64 | 1.02 | 1.31 | 1.55 | 1.78 | 2.01 | 2.25 |
|---|---|---|---|---|---|---|---|---|
| 608  | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 1504 | 0 | 0 | 0 | 0 | +0.50 | +0.50 | +0.50 | +0.50 |
| 2016 | 0 | 0 | 0 | +0.50 | +1.00 | +1.00 | +1.00 | +1.00 |
| 3008 | 0 | 0 | +0.50 | +1.00 | +1.50 | +1.50 | +2.00 | +2.00 |
| 4000 | 0 | 0 | +0.50 | +1.00 | +1.50 | +2.00 | +2.00 | +2.00 |
| 4992 | 0 | 0 | +0.50 | +1.00 | +1.50 | +2.00 | +2.50 | +2.50 |
| 6016 | 0 | 0 | +0.50 | +1.00 | +2.00 | +2.00 | +2.50 | +2.50 |
| 7008 | 0 | 0 | +0.50 | +1.00 | +2.00 | +2.00 | +2.50 | +2.50 |

**✅ Après — E85 objectif E60-safe (°CRK avant PMH)**

| RPM (tr/min) \ MAF (mg/stk) | 0.55 | 0.64 | 1.02 | 1.31 | 1.55 | 1.78 | 2.01 | 2.25 |
|---|---|---|---|---|---|---|---|---|
| 608 | +10.50 | +9.38 | +1.88 | −7.12 | −12.00 | −13.88 | −15.00 | −15.00 |
| 1504 | +16.88 | +15.38 | +9.75 | +5.25 | **−0.62** | **−5.50** | **−8.12** | **−8.88** |
| 2016 | +21.75 | +20.62 | +15.38 | **+10.62** | **+7.00** | **+4.00** | **+1.00** | **−1.25** |
| 3008 | +25.88 | +26.25 | **+21.50** | **+16.75** | **+12.00** | **+8.62** | **+6.88** | **+6.12** |
| 4000 | +35.62 | +34.88 | **+29.38** | **+21.62** | **+17.25** | **+14.75** | **+13.62** | **+9.12** |
| 4992 | +38.62 | +38.25 | **+33.50** | **+24.25** | **+21.75** | **+18.50** | **+15.62** | **+14.50** |
| 6016 | +43.12 | +39.75 | **+33.88** | **+24.25** | **+21.88** | **+20.00** | **+17.88** | **+16.00** |
| 7008 | +43.50 | +39.38 | **+33.88** | **+25.00** | **+22.62** | **+21.12** | **+20.12** | **+19.38** |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Après chaque palier (+1°) | 50 km sans cliquetis | Son métallique bref → **−1° immédiat** |
| Knock control OBD | LTFT avance stable, aucun retrait répété | Retrait fréquent → trop d'avance, reculer |
| Pleine charge | Seulement après validation charge partielle | — |

---

<a id="p2"></a>
## ② `c_t_ti_dly_fl_1` — Délai enrichissement WOT, boîte manuelle, copie 1

| Champ | Valeur |
|---|---|
| Adresse | 0x44EC4 |
| Type | Constante scalaire |
| Unité | secondes |

**Rôle :** Délai entre la détection du full load flag et l'application de l'enrichissement WOT, pour les boîtes manuelles. Stock 200 ms — conçu sur essence pour éviter des enrichissements intempestifs lors de brèves sollicitations pédale. Sur E85, ce délai crée un lean transitoire non compensé (boucle ouverte WOT) à chaque appui franc sur l'accélérateur.

**Avant / Après :**

| | ◀ Stock VB67774 | ✅ E85 (MT) |
|---|---|---|
| `c_t_ti_dly_fl_1` | 0.200 s | **0.000 s** |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Accélération franche WOT | Enrichissement immédiat | Trou bref → vérifier les 2 copies MT à zéro |

---

<a id="p3"></a>
## ③ `c_t_ti_dly_fl_2` — Délai enrichissement WOT, boîte manuelle, copie 2

| Champ | Valeur |
|---|---|
| Adresse | 0x44EC6 |
| Type | Constante scalaire |
| Unité | secondes |

**Rôle :** Deuxième copie du délai WOT MT. L'ECU alterne entre ces deux copies selon le contexte d'exécution. Si _1 est mis à zéro mais _2 reste à 200 ms, l'enrichissement peut encore être retardé de façon intermittente. Les deux doivent être à zéro.

**Avant / Après :**

| | ◀ Stock VB67774 | ✅ E85 (MT) |
|---|---|---|
| `c_t_ti_dly_fl_2` | 0.200 s | **0.000 s** |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| WOT répété plusieurs fois | Richesse WOT immédiate à chaque sollicitation | Délai intermittent → _2 non modifié |

---

<a id="p4"></a>
## ④ `c_t_ti_dly_fl_at_1` / `c_t_ti_dly_fl_at_2` — Délai enrichissement WOT, boîte automatique

| Champ | Valeur |
|---|---|
| Adresse | 0x44EC8 / 0x44ECA |
| Type | Constante scalaire (×2) |
| Unité | secondes |

**Rôle :** Même logique que les copies MT mais pour les boîtes automatiques ZF 6HP. Modifier uniquement si le véhicule est AT — inutile sur MT.

**Avant / Après :**

| | ◀ Stock VB67774 | ✅ E85 (AT uniquement) |
|---|---|---|
| `c_t_ti_dly_fl_at_1` | 0.200 s | **0.000 s** |
| `c_t_ti_dly_fl_at_2` | 0.200 s | **0.000 s** |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Kickdown AT | Enrichissement immédiat lors du passage de rapport | Délai perceptible → vérifier les 2 copies AT |

---

## OPTIONNEL

<a id="p5"></a>
## ⑤ `ip_iga_st_bas_opm_1` — Avance cranking, mode Valvetronic

| Champ | Valeur |
|---|---|
| Adresse | 0x43586 |
| Type | Map 6×8 |
| Unité | °CRK avant PMH |
| Axes | X = TCO (°C), Y = RPM cranking (80–920 tr/min) |

**Rôle :** Avance à l'allumage pendant la phase de cranking uniquement, en mode Valvetronic. **Modifier seulement si le démarrage froid reste difficile (> 5 tours) malgré `ip_mff_cst_opm_*` correctement calibré.** Le réglage cranking de §05 doit être validé en premier.

**◀ Avant — Stock (°CRK avant PMH)**

| RPM (tr/min) \ TCO (°C) | −30.0 | −20.3 | −9.8 | 0.0 | +9.8 | +30.0 | +60.0 | +90.0 |
|---|---|---|---|---|---|---|---|---|
| 80 | 7.88 | 6.75 | 5.25 | 4.13 | 2.63 | 0.75 | −1.88 | −5.63 |
| 160 | 8.63 | 7.50 | 6.00 | 4.88 | 3.38 | 1.50 | −1.13 | −4.88 |
| 320 | 9.75 | 8.63 | 6.75 | 5.63 | 3.75 | 1.88 | −0.38 | −4.13 |
| 460 | 10.50 | 9.38 | 7.50 | 5.63 | 3.75 | 1.50 | −0.38 | −3.38 |
| 640 | 12.00 | 10.50 | 8.25 | 5.25 | 3.38 | 0.75 | −0.38 | −2.63 |
| 920 | 14.63 | 12.75 | 9.38 | 4.88 | 2.25 | −0.38 | −0.75 | −2.63 |

**✏️ Opération — +1° sur les 3 colonnes TCO ≤ 0°C (−30.0, −20.3, −9.8) uniquement**

**✅ Après — E85 (°CRK avant PMH)**

| RPM (tr/min) \ TCO (°C) | −30.0 | −20.3 | −9.8 | 0.0 | +9.8 | +30.0 | +60.0 | +90.0 |
|---|---|---|---|---|---|---|---|---|
| 80 | **8.88** | **7.75** | **6.25** | 4.13 | 2.63 | 0.75 | −1.88 | −5.63 |
| 160 | **9.63** | **8.50** | **7.00** | 4.88 | 3.38 | 1.50 | −1.13 | −4.88 |
| 320 | **10.75** | **9.63** | **7.75** | 5.63 | 3.75 | 1.88 | −0.38 | −4.13 |
| 460 | 10.50 | 9.38 | 7.50 | 5.63 | 3.75 | 1.50 | −0.38 | −3.38 |
| 640 | 12.00 | 10.50 | 8.25 | 5.25 | 3.38 | 0.75 | −0.38 | −2.63 |
| 920 | 14.63 | 12.75 | 9.38 | 4.88 | 2.25 | −0.38 | −0.75 | −2.63 |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Démarrage froid (TCO < 0°C) | < 3 tours | > 5 tours → +1° sur colonnes ≤ 0°C |
| Son au cranking | Aucun cliquetis | Cliquetis → −1° immédiat |

---

<a id="p6"></a>
## ⑥ `ip_iga_st_bas_opm_2` — Avance cranking, mode papillonné (GD)

| Champ | Valeur |
|---|---|
| Adresse | 0x435B6 |
| Type | Map 6×8 |
| Unité | °CRK avant PMH |
| Axes | X = TCO (°C), Y = RPM cranking (80–920 tr/min) |

**Rôle :** Même rôle qu'opm_1 mais pour le mode papillonné. Modifier en même temps qu'opm_1 si la correction est nécessaire — appliquer les mêmes incréments sur les colonnes TCO ≤ 0°C.

**◀ Avant — Stock (°CRK avant PMH)**

| RPM (tr/min) \ TCO (°C) | −30.0 | −20.3 | −9.8 | 0.0 | +9.8 | +30.0 | +60.0 | +90.0 |
|---|---|---|---|---|---|---|---|---|
| 80 | 6.75 | 6.38 | 6.00 | 5.63 | 3.00 | 1.50 | 0.00 | −2.63 |
| 160 | 7.50 | 7.13 | 6.75 | 6.38 | 3.38 | 1.88 | 0.75 | −1.88 |
| 320 | 9.00 | 8.25 | 7.88 | 7.13 | 3.38 | 1.88 | 0.75 | −0.38 |
| 460 | 10.13 | 9.00 | 8.25 | 7.13 | 3.38 | 1.88 | 0.75 | −0.75 |
| 640 | 11.25 | 9.75 | 8.63 | 7.13 | 2.63 | 1.50 | 0.75 | −0.75 |
| 920 | 12.00 | 10.13 | 8.63 | 6.38 | 1.50 | 0.38 | 0.00 | −0.38 |

**✏️ Opération — +1° sur les 3 colonnes TCO ≤ 0°C (−30.0, −20.3, −9.8), lignes RPM 80–320 uniquement (plage de cranking réaliste)**

**✅ Après — E85 (°CRK avant PMH)**

| RPM (tr/min) \ TCO (°C) | −30.0 | −20.3 | −9.8 | 0.0 | +9.8 | +30.0 | +60.0 | +90.0 |
|---|---|---|---|---|---|---|---|---|
| 80 | **7.75** | **7.38** | **7.00** | 5.63 | 3.00 | 1.50 | 0.00 | −2.63 |
| 160 | **8.50** | **8.13** | **7.75** | 6.38 | 3.38 | 1.88 | 0.75 | −1.88 |
| 320 | **10.00** | **9.25** | **8.88** | 7.13 | 3.38 | 1.88 | 0.75 | −0.38 |
| 460 | 10.13 | 9.00 | 8.25 | 7.13 | 3.38 | 1.88 | 0.75 | −0.75 |
| 640 | 11.25 | 9.75 | 8.63 | 7.13 | 2.63 | 1.50 | 0.75 | −0.75 |
| 920 | 12.00 | 10.13 | 8.63 | 6.38 | 1.50 | 0.38 | 0.00 | −0.38 |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Démarrage froid mode GD | < 3 tours | > 5 tours → +1° colonnes froides, même valeur qu'opm_1 |

---

<a id="p7"></a>
## ⑦ `c_iga_ini` — Avance initiale cranking — DERNIER RECOURS

| Champ | Valeur |
|---|---|
| Adresse | 0x44B2A |
| Type | Constante scalaire |
| Unité | °CRK avant PMH |

**Rôle :** Avance à l'allumage appliquée lors du premier cycle d'allumage pendant la phase de cranking. Stock = 6.0°CRK. Un léger supplément peut aider le premier allumage à se propager dans une chambre froide chargée en éthanol peu vaporisé. **Modifier uniquement si démarrage > 5 tours malgré §05 et ⑤⑥ ci-dessus correctement calibrés.**

**Avant / Après :**

| Scénario | ◀ Stock VB67774 | ✅ E85 |
|---|---|---|
| Recommandé — inchangé | 6.0 °CRK | 6.0 °CRK |
| Option +1° (démarrage > 5 tours) | 6.0 °CRK | **7.13 °CRK** |
| Option +2° (démarrage > 8 tours) | 6.0 °CRK | **7.88 °CRK** |

> Maximum recommandé : +2° (7.88 °CRK). Au-delà, risque de détonation au cranking sur moteur froid.

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Démarrage froid (TCO < 10°C) | < 3 tours | > 5 tours → vérifier §05 en priorité avant de toucher `c_iga_ini` |
| Son de cliquetis au cranking | Absent | Son métallique bref → réduire de −1° immédiatement |
