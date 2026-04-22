# §5 — Avance à l'allumage

> L'éthanol a un indice d'octane de 104–108 RON (vs 95 SP95) — il résiste mieux à la détonation et permet d'augmenter l'avance. Calibration **E60 (101 RON)** = pire carburant légal hivernal : zéro risque quelle que soit la saison. Gain : **+5 à +12% de puissance**. Règle absolue : progressivité. Un seul degré de trop = cliquetis = casse moteur.

| Carburant | RON | Avance max exploitable |
|---|:---:|:---:|
| Essence 95 | 95 | Référence |
| E60 (hiver) | ~101 | +2.5° max (stratégie retenue) |
| E70 | ~104 | +3–4° |
| E85 | ~108 | +4–5° |

---

<a id="p1"></a>
## ① `ip_iga_bas_max_knk__n__maf` — Plafond avance anti-cliquetis f(MAF × RPM)

| Champ | Valeur |
|---|---|
| Adresse | 0x4323A |
| Structure | Map 8×8 |
| Axes | X = MAF (0.55–2.25 mg/stk), Y = RPM (608–7008) |

**Rôle :** Table principale d'avance à modifier pour bénéficier de l'octane supérieur de l'E85. C'est le plafond knock-limited — l'avance effective ne peut pas dépasser ces valeurs. En augmentant les cellules de haute charge (colonnes MAF élevé), on autorise le modèle de couple à demander davantage d'avance. Le knock control reste actif et reculera si cliquetis détecté. Procédure obligatoire : un palier à la fois, 50 km entre chaque.

**Procédure d'augmentation par zones :**

| Zone | MAF | Incrément Phase 1 | Incrément Phase 2 | Incrément Phase 3 |
|---|---|---|---|---|
| Ralenti / très faible charge | < 0.65 mg/stk | +0° | +0° | +0° |
| Charge moyenne | 1.0–1.5 mg/stk | +0.5° | +1.0° | — |
| Haute charge / WOT | > 1.5 mg/stk | +1.0° | +2.0° | +2.5° |

**Avant / Après :**

**◀ Stock (°CRK avant PMH)**

| RPM \ MAF | 0.55 | 0.64 | 1.02 | 1.31 | 1.55 | 1.78 | 2.01 | 2.25 |
|---|---|---|---|---|---|---|---|---|
| 608 rpm | +10.50 | +9.38 | +1.88 | −7.12 | −12.00 | −13.88 | −15.00 | −15.00 |
| 1504 rpm | +16.88 | +15.38 | +9.75 | +5.25 | −1.12 | −6.00 | −8.62 | −9.38 |
| 2016 rpm | +21.75 | +20.62 | +15.38 | +10.12 | +6.00 | +3.00 | 0.00 | −2.25 |
| 3008 rpm | +25.88 | +26.25 | +21.00 | +15.75 | +10.50 | +7.12 | +4.88 | +4.12 |
| 4000 rpm | +35.62 | +34.88 | +28.88 | +20.62 | +15.75 | +12.75 | +11.62 | +7.12 |
| 4992 rpm | +38.62 | +38.25 | +33.00 | +23.25 | +20.25 | +16.50 | +13.12 | +12.00 |
| 6016 rpm | +43.12 | +39.75 | +33.38 | +23.25 | +19.88 | +18.00 | +15.38 | +13.50 |
| 7008 rpm | +43.50 | +39.38 | +33.38 | +24.00 | +20.62 | +19.12 | +17.62 | +16.88 |

**✏️ E85 — objectif E60-safe (stock + incréments)**

| RPM \ MAF | 0.55 | 0.64 | 1.02 | 1.31 | 1.55 | 1.78 | 2.01 | 2.25 |
|---|---|---|---|---|---|---|---|---|
| 608 rpm | +10.50 | +9.38 | +1.88 | −7.12 | −12.00 | −13.88 | −15.00 | −15.00 |
| 1504 rpm | +16.88 | +15.38 | +9.75 | +5.25 | **−0.62** | **−5.50** | **−8.12** | **−8.88** |
| 2016 rpm | +21.75 | +20.62 | +15.38 | **+10.62** | **+7.00** | **+4.00** | **+1.00** | **−1.25** |
| 3008 rpm | +25.88 | +26.25 | **+21.50** | **+16.75** | **+12.00** | **+8.62** | **+6.88** | **+6.12** |
| 4000 rpm | +35.62 | +34.88 | **+29.38** | **+21.62** | **+17.25** | **+14.75** | **+13.62** | **+9.12** |
| 4992 rpm | +38.62 | +38.25 | **+33.50** | **+24.25** | **+21.75** | **+18.50** | **+15.62** | **+14.50** |
| 6016 rpm | +43.12 | +39.75 | **+33.88** | **+24.25** | **+21.88** | **+20.00** | **+17.88** | **+16.00** |
| 7008 rpm | +43.50 | +39.38 | **+33.88** | **+25.00** | **+22.62** | **+21.12** | **+20.12** | **+19.38** |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Après chaque palier (+1°) | 50 km sans cliquetis | Son métallique bref → **−1° immédiat** |
| Knock control OBD | LTFT avance stable, aucun retrait répété | Retrait fréquent → trop d'avance |
| Pleine charge | Seulement après validation charge partielle | — |

> Maximum recommandé stratégie E60-safe : **+2.5°** sur les colonnes droites (haute charge). Jamais plus de +5° même sur E85 pur sur un N52 atmosphérique.

---

<a id="p2"></a>
## ② `ip_iga_st_bas_opm_1` — Avance cranking, mode Valvetronic

| Champ | Valeur |
|---|---|
| Adresse | 0x43586 |
| Structure | Map 6×8 |
| Axes | X = TCO (°C), Y = RPM cranking (80–920 tr/min) |

**Rôle :** Avance à l'allumage pendant la phase de cranking uniquement (RPM 80–920), en mode Valvetronic. Active uniquement lors du démarrage. Les valeurs stock sont conservées dans le bin E85 de référence — modifier seulement si le démarrage froid reste difficile (> 5 tours) malgré ip_mff_cst_opm_* correctement calibré.

**Avant / Après :**

**◀ Stock = E85 bin de référence (inchangé)**

| RPM \ TCO | −30.0 | −20.3 | −9.8 | 0.0 | +9.8 | +30.0 | +60.0 | +90.0 |
|---|---|---|---|---|---|---|---|---|
| 80 rpm | 7.88 | 6.75 | 5.25 | 4.13 | 2.63 | 0.75 | −1.88 | −5.63 |
| 160 rpm | 8.63 | 7.50 | 6.00 | 4.88 | 3.38 | 1.50 | −1.13 | −4.88 |
| 320 rpm | 9.75 | 8.63 | 6.75 | 5.63 | 3.75 | 1.88 | −0.38 | −4.13 |
| 460 rpm | 10.50 | 9.38 | 7.50 | 5.63 | 3.75 | 1.50 | −0.38 | −3.38 |
| 640 rpm | 12.00 | 10.50 | 8.25 | 5.25 | 3.38 | 0.75 | −0.38 | −2.63 |
| 920 rpm | 14.63 | 12.75 | 9.38 | 4.88 | 2.25 | −0.38 | −0.75 | −2.63 |

**✏️ E85 (si démarrage difficile — +1° colonnes ≤ 0°C uniquement)**

| RPM \ TCO | −30.0 | −20.3 | −9.8 | 0.0 | +9.8 | +30.0 | +60.0 | +90.0 |
|---|---|---|---|---|---|---|---|---|
| 80 rpm | **8.88** | **7.75** | **6.25** | 4.13 | 2.63 | 0.75 | −1.88 | −5.63 |
| 160 rpm | **9.63** | **8.50** | **7.00** | 4.88 | 3.38 | 1.50 | −1.13 | −4.88 |
| 320 rpm | **10.75** | **9.63** | **7.75** | 5.63 | 3.75 | 1.88 | −0.38 | −4.13 |
| 460 rpm | 10.50 | 9.38 | 7.50 | 5.63 | 3.75 | 1.50 | −0.38 | −3.38 |
| 640 rpm | 12.00 | 10.50 | 8.25 | 5.25 | 3.38 | 0.75 | −0.38 | −2.63 |
| 920 rpm | 14.63 | 12.75 | 9.38 | 4.88 | 2.25 | −0.38 | −0.75 | −2.63 |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Démarrage froid (TCO < 0°C) | < 3 tours | > 5 tours → +1° sur colonnes ≤ 0°C |
| Son au cranking | Aucun cliquetis | Cliquetis → −1° immédiat |

---

<a id="p3"></a>
## ③ `ip_iga_st_bas_opm_2` — Avance cranking, mode papillonné (GD)

| Champ | Valeur |
|---|---|
| Adresse | 0x435B6 |
| Structure | Map 6×8 |
| Axes | X = TCO (°C), Y = RPM cranking (80–920 tr/min) |

**Rôle :** Même rôle qu'opm_1 mais pour le mode papillonné (Valvetronic désactivé au démarrage). Si le N52 démarre en mode GD, c'est cette table qui s'applique. Modifier en même temps qu'opm_1 si la correction est nécessaire — appliquer les mêmes incréments sur les colonnes TCO ≤ 0°C.

**Avant / Après :**

**◀ Stock = E85 bin de référence (inchangé)**

| RPM \ TCO | −30.0 | −20.3 | −9.8 | 0.0 | +9.8 | +30.0 | +60.0 | +90.0 |
|---|---|---|---|---|---|---|---|---|
| 80 rpm | 6.75 | 6.38 | 6.00 | 5.63 | 3.00 | 1.50 | 0.00 | −2.63 |
| 160 rpm | 7.50 | 7.13 | 6.75 | 6.38 | 3.38 | 1.88 | 0.75 | −1.88 |
| 320 rpm | 9.00 | 8.25 | 7.88 | 7.13 | 3.38 | 1.88 | 0.75 | −0.38 |
| 460 rpm | 10.13 | 9.00 | 8.25 | 7.13 | 3.38 | 1.88 | 0.75 | −0.75 |
| 640 rpm | 11.25 | 9.75 | 8.63 | 7.13 | 2.63 | 1.50 | 0.75 | −0.75 |
| 920 rpm | 12.00 | 10.13 | 8.63 | 6.38 | 1.50 | 0.38 | 0.00 | −0.38 |

**✏️ E85 (si démarrage difficile — appliquer même incrément que opm_1 sur colonnes ≤ 0°C)**

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Démarrage froid mode GD | < 3 tours | > 5 tours → +1° colonnes froides, même valeur qu'opm_1 |
