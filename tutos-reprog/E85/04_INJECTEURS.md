# §2 — Mise à l'échelle des injecteurs — Paramètre CRITIQUE

> 💡 AFR E85 = 9.55:1 vs 14.7:1 essence → **+45% de masse carburant** à injecter. Enrichissement via les 4 maps `ip_mff_cor_opm_*` (multiplicateurs d'injection, max encodable = 2.031).

### 📋 Tables à modifier

| Paramètre | Adresse | Structure |
|---|---|---|
| `ip_mff_cor_opm_1_1` | 0x4E3D4 | 12×16 flat |
| `ip_mff_cor_opm_1_2` | 0x4E554 | 12×16 flat |
| `ip_mff_cor_opm_2_1` | 0x4E6D4 | 10×12 flat |
| `ip_mff_cor_opm_2_2` | 0x4E7C4 | 10×12 flat |

### 🔨 Procédure

```
TunerPro → chaque map → Ctrl+A → saisir raw 47 407 → répéter ×4
```

<a id="pencil-injecteurs"></a>

### ✏️ Avant / Après

| Paramètre | ◀ Stock | ▶ E85 |
|---|---|---|
| `ip_mff_cor_opm_*` (×4 — toutes cellules flat) | raw 32 770 — phys **1.016** | raw **47 407** — phys **1.473** |

> `0.3394 × 1.473 = 0.500 ms/mg` — ratio ×1.450 (cible ×1.447, écart **0.2%**)

### ✅ Vérification

| Signal OBD2 | ✅ Cible | ⚠️ Si hors cible |
|---|---|---|
| STFT — ralenti chaud (TCO > 80°C) | **−5% à +5%** | > +15% → `ip_mff_cor_opm_*` **+2–3%** |
| | | < −15% → `ip_mff_cor_opm_*` **−2–3%** |

---

### Quel facteur selon le titre éthanol réel ?

La formule correcte dérive de l'AFR stœchiométrique du mélange :

```
AFR_blend = 1 / ( E_fraction/9.0 + (1−E_fraction)/14.7 )

Facteur_injection = (14.7 / AFR_blend) × 0.94
                    ↑ ratio AFR       ↑ correction densité éthanol/essence
```

| Teneur éthanol | AFR stœchio | Facteur vs stock | `ip_mff_cor_opm_*` phys cible | `ip_mff_cor_opm_*` raw cible |
|---|---|---|---|---|
| E65 (65%) | 10.41:1 | **×1.33** | **1.352** | **43 613** |
| E70 (70%) | 10.18:1 | **×1.36** | **1.382** | **44 581** |
| E75 (75%) | 9.97:1 | **×1.39** | **1.412** | **45 548** |
| **E85 (85%) ← CIBLE INJECTEUR** | **9.55:1** | **×1.45** | **1.473** | **47 407** |

> **Pourquoi E85 comme cible du facteur injecteur alors que le carburant réel est E70 ?**
> En boucle fermée, le LTFT corrige automatiquement l'excès de richesse : avec du E70 réel, le LTFT se stabilise à environ −6% — dans la plage −8%/+12% du MSV70, aucun problème.
> En boucle ouverte (WOT, transitions), aucune correction n'intervient. En calibrant sur E85, on garantit qu'on est toujours du côté riche en open loop, quelle que soit la teneur réelle de la pompe (E60 à E85). C'est le choix sécuritaire pour une carto de rue.

> Si vous ne savez pas exactement quel titre vous avez, commencez avec E70 (raw 44 581) — les STFT vous diront si vous montez ou descendez.

### Formule pour injecteurs remplacés

```
ip_mff_cor_nouveau = ip_mff_cor_STOCK × Facteur_Ethanol × (Débit_stock / Débit_nouveaux)
                   = 1.016 × Facteur_Ethanol × (Débit_stock / Débit_nouveaux)

Exemple — injecteurs N54 (débit ~30% supérieur aux N52 stock) sur E70 :
  ip_mff_cor = 1.016 × 1.36 × (1 / 1.30) = 1.016 × 1.046 ≈ 1.063  → raw ip_mff_cor ≈ 34 290
```

### Durée de test au ralenti et risques

> **Question fréquente :** peut-on laisser tourner le moteur au ralenti avec une richesse très pauvre le temps de vérifier les STFT, sans risquer de l'endommager ?

**Réponse courte : oui, mais avec limites.**

| Condition STFT | Durée max recommandée | Risque principal | Action |
|---|---|---|---|
| STFT +5% à +12% | Plusieurs minutes sans problème | Dans les limites LTFT — ECU compense | Observer, ajuster |
| STFT > +12% (dépasse plafond LTFT) | 2–3 min maximum | Légère surchauffe soupapes échappement | Couper, ajuster facteur |
| STFT > +12% ET LTFT bloqué à +12% | **< 30 secondes** | ECU ne compense plus — mélange pauvre permanent → calamine possible | **Couper immédiatement** |

**Pourquoi l'idle est moins dangereux que la charge :**
- Au ralenti (650–750 rpm), la quantité d'air et de carburant est faible → même 20% de pauvreté représente peu de masse
- La sonde lambda fonctionne et la correction STFT est active — le moteur n'est **jamais** totalement en boucle ouverte au ralenti chaud
- Les températures d'échappement restent modérées (~350–450°C) vs charge pleine (~650–800°C)

**Ce qui est dangereux en revanche :**
- Couper le moteur quand STFT > +20% et **redémarrer immédiatement** sans ajuster → cycles répétés de chauffe sur mélange pauvre
- Faire tourner au ralenti avec **TCO < 80°C** (boucle fermée inactive ou instable) → lambda non régulé
- Faire des accélérations même courtes si STFT > +15% — sous charge le risque d'inflammation anormale (detonation) augmente fortement sur mélange pauvre

**Procédure de test sécurisée :**
```
1. Démarrer à chaud (TCO > 80°C, STFT actif)
2. Laisser tourner 30 secondes → lire STFT moyen
3. Si STFT dans [−10%, +15%] → rouler prudemment, laisser LTFT converger
4. Si STFT > +15% → couper, ajuster facteur +3%, reflasher, recommencer
5. Ne jamais dépasser 3 min avec STFT > +20%
6. Surveiller aussi la température d'eau (TCO) : si monte anormalement → pauvre
```

---


---

## Récapitulatif — Valeurs Avant / Après

### ① Scalaires / paramètres à valeur unique

| # | Paramètre | Adresse | ◀ Raw stock | ◀ Valeur stock | ▶ Raw E85 | ▶ Valeur E85 |
|---|---|---|---|---|---|---|
| 1 | `ip_mff_cor_opm_1_1` (12×16 flat) | 0x4E3D4 | 32 770 | 1.016 | **47 407** | **1.473** |
| 2 | `ip_mff_cor_opm_1_2` (12×16 flat) | 0x4E554 | 32 770 | 1.016 | **47 407** | **1.473** |
| 3 | `ip_mff_cor_opm_2_1` (10×12 flat) | 0x4E6D4 | 32 770 | 1.016 | **47 407** | **1.473** |
| 4 | `ip_mff_cor_opm_2_2` (10×12 flat) | 0x4E7C4 | 32 770 | 1.016 | **47 407** | **1.473** |
| 5 | `c_tco_n_mff_cst` | 0x44F2F | 87 | 17.25 °C | **97** | **25.00 °C** |
| 6 | `c_t_ti_dly_fl_1` | 0x44EC4 | 20 | 0.200 s | **0** | **0.000 s** |
| 7 | `c_t_ti_dly_fl_2` | 0x44EC6 | 20 | 0.200 s | **0** | **0.000 s** |
| 8 | `c_iga_ini` *(optionnel)* | 0x44B2A | 111 | 6.0 °CRK | **111** | **6.0 °CRK (inchangé)** — modifier +1° à +2° si démarrage > 5 tours |

> Pour les 4 maps `ip_mff_cor_opm_*` : TunerPro → Ctrl+A → saisir raw **47 407** → répéter ×4. `c_fac_mff_ti_stnd` reste inchangé (overflow XDF empêche d'y encoder 0.491).

