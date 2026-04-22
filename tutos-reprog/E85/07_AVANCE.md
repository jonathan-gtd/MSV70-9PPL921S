# §5 — Avance à l'Allumage

> 💡 L'éthanol (104–108 RON vs 95 SP95) résiste mieux au cliquetis → avance augmentable. Cible calibrée **E60 (101 RON)** = pire carburant légal hivernal : zéro risque quelle que soit la station. Gain : **+5 à +12% de puissance**.

| Carburant | RON | Auto-inflammation |
|---|:---:|:---:|
| Essence 95 | 95 | ~280°C |
| E70 | ~104 | ~400°C |
| E85 | ~108 | ~420°C |

### 📋 Tables à modifier

| Paramètre | Adresse | Structure | Priorité |
|---|---|---|---|
| `ip_iga_bas_max_knk__n__maf` | 0x4323A | 8×8, X = MAF, Y = RPM | **CRITIQUE** |
| `ip_iga_st_bas_opm_1` / `_opm_2` | 0x43586 / 0x435B6 | 6×8, X = TCO, Y = RPM cranking | Optionnel |

### 🔨 Procédure

Ajouter les incréments E60 progressivement — **un palier à la fois, 50 km entre chaque**. Tableau de modification : §3.2.

### 3.1 — Tables d'avance à identifier dans le MSV70

Le MSV70 gère l'allumage par **modèle de couple** (torque model). Il n'y a pas une seule « table de base » qui sort directement l'avance appliquée : l'avance effective résulte d'un calcul borné entre un plafond (knock-limited) et un plancher. Les tables réellement utilisées **en roulage** sont :

| Paramètre | Adresse | Dimensions / axes | Rôle | Sensibilité E85 |
|---|---|---|---|---|
| `ip_iga_bas_max_knk__n__maf` | **0x4323A** | 8×8, X=MAF 0.55–2.25 mg/stk, Y=RPM 608–7008 | **Plafond knock — référence MBT.** C'est la table principale à modifier pour gagner de l'avance en E85. | **CRITIQUE** |
| `ip_iga_min_n_maf_opm_1` | 0x4347A | 8×6, X=MAF 0.36–2.14, Y=RPM 320–6528 | Plancher d'avance (mode normal) | Aucune |
| `ip_iga_min_n_maf_opm_2` | 0x434AA | 8×6, X=MAF 0.28–2.14, Y=RPM 320–6528 | Plancher d'avance (mode throttled) | Aucune |
| `ip_fac_eff_iga_opm_1` / `ip_fac_eff_iga_opm_2` | 0x4A5D4 | 16×16, X=couple %, Y=RPM | Facteur d'efficacité avance pour le modèle de couple | Faible/Modérée |
| `ip_iga_st_bas_opm_1` / `ip_iga_st_bas_opm_2` | 0x43586 / 0x435B6 | 6×8, X=TCO, Y=RPM cranking | Avance uniquement pendant la phase de démarrage (cranking) — axes TCO et RPM 80–920 tr/min uniquement | Optionnel (démarrage froid) |

<a id="pencil-avance"></a>

### ✏️ Avant / Après — `ip_iga_bas_max_knk__n__maf` (°CRK avant PMH)

```
            MAF →  0.55   0.64   1.02   1.31   1.55   1.78   2.01   2.25 mg/stk
 608 rpm :        +10.50  +9.38  +1.88  -7.12 -12.00 -13.88 -15.00 -15.00
1504 rpm :        +16.88 +15.38  +9.75  +5.25  -1.12  -6.00  -8.62  -9.38
2016 rpm :        +21.75 +20.62 +15.38 +10.12  +6.00  +3.00   0.00  -2.25
3008 rpm :        +25.88 +26.25 +21.00 +15.75 +10.50  +7.12  +4.88  +4.12
4000 rpm :        +35.62 +34.88 +28.88 +20.62 +15.75 +12.75 +11.62  +7.12
4992 rpm :        +38.62 +38.25 +33.00 +23.25 +20.25 +16.50 +13.12 +12.00
6016 rpm :        +43.12 +39.75 +33.38 +23.25 +19.88 +18.00 +15.38 +13.50
7008 rpm :        +43.50 +39.38 +33.38 +24.00 +20.62 +19.12 +17.62 +16.88
```

C'est cette carto qui fixe le plafond au-delà duquel le knock control est censé reculer l'avance. En augmentant ces valeurs en haute charge (colonnes de droite) de +2° à +5°, vous laissez le modèle de couple MSV70 demander davantage d'avance tant qu'il ne détecte pas de cliquetis — ce qui est précisément ce que permet l'E85.

### 3.2 — Procédure d'augmentation d'avance pour E85

**RÈGLE D'OR : Progressivité absolue. Un seul degré de trop = cliquetis = casse moteur.**

**Table à modifier : `ip_iga_bas_max_knk__n__maf` (adresse 0x4323A)**

**Zones à modifier :**

**Zone 1 : Ralenti / très faible charge (MAF < 0.65, RPM < 1500)**
```
Action : Ne rien changer
Raison : Le ralenti N52 est déjà bien calibré, le modifier déstabilise
```

**Zone 2 : Charge moyenne (MAF 1.0–1.5 mg/stk)**
```
Augmentation : +0.5° à +1°
Procédure : Testez +0.5° d'abord sur 50 km avant d'aller à +1°
```

**Zone 3 : Haute charge / WOT (MAF > 1.5 mg/stk)**
```
Phase 1 : +1° → Roulez 50 km, pas de cliquetis → continuer
Phase 2 : +2° → Roulez 50 km, vérifiez → continuer
Phase 3 : +2.5° → Test final sur montée exigeante, cliquetis = STOP

Maximum recommandé stratégie E60-safe : +2.5° sur les colonnes droites
```

> **Pourquoi se limiter à +2.5° alors que l'éthanol permettrait +4° à +5° ?**
> Le "E85" pompe France peut légalement descendre à 60% d'éthanol en hiver (~101 RON). Si on grille +4.5° d'avance calibrés pour E85 (108 RON) et que la station délivre du E60, le knock control détectera du cliquetis et reculera l'avance automatiquement — mais avec du stress moteur inutile à chaque accélération. En calibrant le plafond knock pour E60, le moteur n'approche jamais la limite quelle que soit la saison. Le gain est légèrement réduit (+2.5° vs +4.5°), mais la sécurité est totale.

**Tableau de modification (en degrés à AJOUTER aux valeurs stock de `ip_iga_bas_max_knk__n__maf`) — calibré E60 :**

```
RPM \ MAF    0.55   0.64   1.02   1.31   1.55   1.78   2.01   2.25
 608        +0     +0     +0     +0     +0     +0     +0     +0
1504        +0     +0     +0     +0     +0.5   +0.5   +0.5   +0.5
2016        +0     +0     +0     +0.5   +1.0   +1.0   +1.0   +1.0
3008        +0     +0     +0.5   +1.0   +1.5   +1.5   +2.0   +2.0
4000        +0     +0     +0.5   +1.0   +1.5   +2.0   +2.0   +2.0
4992        +0     +0     +0.5   +1.0   +1.5   +2.0   +2.5   +2.5
6016        +0     +0     +0.5   +1.0   +2.0   +2.0   +2.5   +2.5
7008        +0     +0     +0.5   +1.0   +2.0   +2.0   +2.5   +2.5
```

**`ip_iga_bas_max_knk__n__maf` — Valeurs OBJECTIF E60 (stock + incréments ci-dessus) :**

```
            MAF →  0.55   0.64   1.02   1.31   1.55   1.78   2.01   2.25 mg/stk
 608 rpm :        +10.50  +9.38  +1.88  -7.12 -12.00 -13.88 -15.00 -15.00
1504 rpm :        +16.88 +15.38  +9.75  +5.25  -0.62  -5.50  -8.12  -8.88
2016 rpm :        +21.75 +20.62 +15.38 +10.62  +7.00  +4.00  +1.00  -1.25
3008 rpm :        +25.88 +26.25 +21.50 +16.75 +12.00  +8.62  +6.88  +6.12
4000 rpm :        +35.62 +34.88 +29.38 +21.62 +17.25 +14.75 +13.62  +9.12
4992 rpm :        +38.62 +38.25 +33.50 +24.25 +21.75 +18.50 +15.62 +14.50
6016 rpm :        +43.12 +39.75 +33.88 +24.25 +21.88 +20.00 +17.88 +16.00
7008 rpm :        +43.50 +39.38 +33.88 +25.00 +22.62 +21.12 +20.12 +19.38
```

> **Démarrage difficile sur E85 à froid ?** Vous pouvez séparément ajouter +1° à +2° sur les cellules basses colonnes (TCO froide) de `ip_iga_st_bas_opm_1` / `ip_iga_st_bas_opm_2` — mais c'est une optimisation de cranking, pas de puissance.

### ✅ Vérification

| Critère | ✅ OK | ⚠️ Si raté |
|---|---|---|
| Après chaque palier | 50 km sans cliquetis | Son métallique bref → **−1° immédiat** |
| Knock control (OBD2) | LTFT avance stable | Recul d'avance répété → trop avancé |
| Pleine charge | Seulement après validation charge partielle | — |

### 3.3 — Contrôle Anti-Cliquetis MSV70

Le MSV70 dispose d'un **contrôle de cliquetis actif** via les capteurs de détonation. Ce système peut reculer l'avance automatiquement si cliquetis détecté.

**Paramètres IGA_KnockControl :**
- `ip_nr_mpl_is` / `ip_nr_mpl_st` : Nombre de cycles pour décision retrait avance
- `ip_td_mpl` : Délai entre détection et retrait
- `c_n_max_ign_mpl` : RPM max pour l'activité knock control

Sur E85 pur, le knock control intervient **beaucoup moins souvent** qu'avec l'essence. Si vous voyez les LTFT d'avance bouger fréquemment après votre calibration, c'est que vous avez encore trop d'avance → reculez de −1°.

### ⚠️ Avertissements Critiques Allumage

- **Jamais plus de +5°** même avec E85 sur un N52 atmosphérique (sur le plafond knock)
- **Testez à chaud (90°C+)** : le N52 est plus sujet aux cliquetis à chaud
- **E85 = protection supérieure, pas immunité** : le piston peut toujours fondre

---


---

## Récapitulatif — Valeurs Avant / Après

### ④ ip_iga_st_bas_opm_1 — Avance cranking Valvetronic @ 0x43586 (6×8, °CRK avant PMH)

> Axes : X = TCO (°C), Y = RPM cranking. Équation : 0.375 × raw − 35.625. **Identique stock et E85 bin de référence** — modifier seulement si démarrage froid difficile (ajouter +1° sur colonnes froides).

```
STOCK (= E85 bin de référence — inchangé) :
TCO (°C) →  -30.0  -20.3   -9.8    0.0   +9.8  +30.0  +60.0  +90.0
  80 rpm :    7.88   6.75   5.25   4.13   2.63   0.75  -1.88  -5.63
 160 rpm :    8.63   7.50   6.00   4.88   3.38   1.50  -1.13  -4.88
 320 rpm :    9.75   8.63   6.75   5.63   3.75   1.88  -0.38  -4.13
 460 rpm :   10.50   9.38   7.50   5.63   3.75   1.50  -0.38  -3.38
 640 rpm :   12.00  10.50   8.25   5.25   3.38   0.75  -0.38  -2.63
 920 rpm :   14.63  12.75   9.38   4.88   2.25  -0.38  -0.75  -2.63
```

---

### ⑤ ip_iga_st_bas_opm_2 — Avance cranking papillonné @ 0x435B6 (6×8, °CRK avant PMH)

> Même axes et équation. **Identique stock et E85 bin de référence.**

```
STOCK (= E85 bin de référence — inchangé) :
TCO (°C) →  -30.0  -20.3   -9.8    0.0   +9.8  +30.0  +60.0  +90.0
  80 rpm :    6.75   6.38   6.00   5.63   3.00   1.50   0.00  -2.63
 160 rpm :    7.50   7.13   6.75   6.38   3.38   1.88   0.75  -1.88
 320 rpm :    9.00   8.25   7.88   7.13   3.38   1.88   0.75  -0.38
 460 rpm :   10.13   9.00   8.25   7.13   3.38   1.88   0.75  -0.75
 640 rpm :   11.25   9.75   8.63   7.13   2.63   1.50   0.75  -0.75
 920 rpm :   12.00  10.13   8.63   6.38   1.50   0.38   0.00  -0.38
```

---

### ⑥ ip_iga_bas_max_knk__n__maf — Avance plafond knock @ 0x4323A (8×8, °CRK avant PMH)

> Cible E60 (101 RON — pire carburant légal hivernal). Progressivité obligatoire : un palier à la fois, 50 km entre chaque.

```
AVANT (stock) :
            MAF →  0.55   0.64   1.02   1.31   1.55   1.78   2.01   2.25 mg/stk
 608 rpm :        +10.50  +9.38  +1.88  -7.12 -12.00 -13.88 -15.00 -15.00
1504 rpm :        +16.88 +15.38  +9.75  +5.25  -1.12  -6.00  -8.62  -9.38
2016 rpm :        +21.75 +20.62 +15.38 +10.12  +6.00  +3.00   0.00  -2.25
3008 rpm :        +25.88 +26.25 +21.00 +15.75 +10.50  +7.12  +4.88  +4.12
4000 rpm :        +35.62 +34.88 +28.88 +20.62 +15.75 +12.75 +11.62  +7.12
4992 rpm :        +38.62 +38.25 +33.00 +23.25 +20.25 +16.50 +13.12 +12.00
6016 rpm :        +43.12 +39.75 +33.38 +23.25 +19.88 +18.00 +15.38 +13.50
7008 rpm :        +43.50 +39.38 +33.38 +24.00 +20.62 +19.12 +17.62 +16.88

APRÈS (objectif E60 = stock + incréments) :
            MAF →  0.55   0.64   1.02   1.31   1.55   1.78   2.01   2.25 mg/stk
 608 rpm :        +10.50  +9.38  +1.88  -7.12 -12.00 -13.88 -15.00 -15.00
1504 rpm :        +16.88 +15.38  +9.75  +5.25  -0.62  -5.50  -8.12  -8.88
2016 rpm :        +21.75 +20.62 +15.38 +10.62  +7.00  +4.00  +1.00  -1.25
3008 rpm :        +25.88 +26.25 +21.50 +16.75 +12.00  +8.62  +6.88  +6.12
4000 rpm :        +35.62 +34.88 +29.38 +21.62 +17.25 +14.75 +13.62  +9.12
4992 rpm :        +38.62 +38.25 +33.50 +24.25 +21.75 +18.50 +15.62 +14.50
6016 rpm :        +43.12 +39.75 +33.88 +24.25 +21.88 +20.00 +17.88 +16.00
7008 rpm :        +43.50 +39.38 +33.88 +25.00 +22.62 +21.12 +20.12 +19.38
```

---
