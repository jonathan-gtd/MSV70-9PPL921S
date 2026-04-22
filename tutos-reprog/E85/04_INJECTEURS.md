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

---

## Dead Time — Compensation Tension Batterie (`ip_ti_min`)

Le dead time est le délai électromécanique entre la commande d'ouverture injecteur et l'ouverture réelle de l'aiguille. Il varie avec la tension batterie : moins de tension = plus de temps pour magnétiser la bobine.

Le MSV70 corrige automatiquement chaque injection : `TI_total = TI_calculé + dead_time(Ubatt)`.

**Sur les injecteurs stock (Bosch EV14 13537531634) : ne rien modifier.** La table est calibrée d'usine.

**Si vous changez les injecteurs : recalibration OBLIGATOIRE.**

### ✏️ Avant / Après — `ip_ti_min` f(Ubatt)

| Tension batterie | ◀ Stock (Bosch EV14 13537531634) | ▶ E85 injecteurs stock | Nouveaux injecteurs |
|---|---|---|---|
| 7V  | ~1.20 ms | **Identique** | Datasheet constructeur |
| 9V  | ~0.85 ms | **Identique** | Datasheet constructeur |
| 11V | ~0.62 ms | **Identique** | Datasheet constructeur |
| 13V | ~0.50 ms | **Identique** | Datasheet constructeur |
| 14.4V | ~0.43 ms | **Identique** | Datasheet constructeur |

> Un dead time trop long = mélange riche permanent au ralenti (STFT instable dès démarrage). Trop court = mélange pauvre permanent. Non-compensable par les fuel trims seuls.

---

## Duty Cycle Injecteur — Saturation Haut Régime

Le duty cycle (DC) est le ratio temps d'injection / temps disponible par cycle moteur. Au-delà de **85%**, l'injecteur entre en saturation : le débit plafonne → mélange pauvre en WOT, non détectable par les sondes (boucle ouverte), directement destructeur.

Sur E85, le TI est **×1.35 à ×1.45** plus long qu'à l'essence → le risque de saturation apparaît naturellement plus tôt à haut régime.

### Calcul du temps disponible par cycle

```
T_cycle (ms) = (60 000 / RPM) × 2

→ 4000 RPM : 30.0 ms  — TI max safe (85%) = 25.5 ms
→ 5500 RPM : 21.8 ms  — TI max safe (85%) = 18.5 ms
→ 6500 RPM : 18.5 ms  — TI max safe (85%) = 15.7 ms
```

### ✏️ Avant / Après — Duty cycle estimé @ 6500 RPM WOT

| Condition | TI estimé | Duty Cycle | Statut |
|---|---|---|---|
| ◀ Stock essence, injecteurs 13537531634 | ~8–10 ms | ~43–54% | Confortable |
| ▶ E85, injecteurs 13537531634, stock power | ~12–14 ms | ~65–76% | Dans les limites, marge faible |
| E85, injecteurs 13537531634, puissance augmentée | >15 ms | >81% | **Zone critique** |
| E85, injecteurs 13537531634, saturé | ≥15.7 ms | **≥85%** | **SATURATION — danger immédiat** |

> La saturation est silencieuse : aucune sonde ne la détecte directement en WOT. Le moteur part pauvre sans signe avant-coureur → détonation, fonte pistons.

### ✅ Vérification duty cycle

| PID OBD | Cible | Action si hors cible |
|---|---|---|
| Injection Time (Ti) | < 15.7 ms @ 6500 RPM | Si > 15 ms → injecteurs remplacés nécessaires |
| Duty Cycle calculé | < 85% à tous régimes | Si proche 85% → ne pas augmenter la puissance sans nouveaux injecteurs |

---

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

## Sections par paramètre — Avant / Après

---

### ① `ip_mff_cor_opm_1_1` — Multiplicateur injection Valvetronic mode 1 @ 0x4E3D4

**Structure :** map 12×16, toutes cellules identiques (flat)  
**Équation :** `0.031250 × raw − 1.000`  
**Rôle :** Facteur multiplicateur de la masse carburant calculée, mode Valvetronic, première séquence d'injection. Sur essence ≈ 1.0. Sur E85 doit être ×1.45 pour compenser l'AFR plus bas (9.55 vs 14.7).

| | ◀ Raw stock | ◀ Valeur stock | ▶ Raw E85 | ▶ Valeur E85 |
|---|---|---|---|---|
| `ip_mff_cor_opm_1_1` | 32 770 | 1.016 | **47 407** | **1.473** |

> TunerPro : Ctrl+A → saisir raw 47 407 (toutes cellules flat)

---

### ② `ip_mff_cor_opm_1_2` — Multiplicateur injection Valvetronic mode 2 @ 0x4E554

**Structure :** map 12×16, flat  
**Rôle :** Copie du facteur pour le mode d'injection 2 en Valvetronic. Si cette map diverge d'opm_1_1, l'injection devient asymétrique selon les conditions de charge → comportement instable, STFT oscillant.

| | ◀ Raw stock | ◀ Valeur stock | ▶ Raw E85 | ▶ Valeur E85 |
|---|---|---|---|---|
| `ip_mff_cor_opm_1_2` | 32 770 | 1.016 | **47 407** | **1.473** |

> Même valeur qu'opm_1_1 — modifier les deux simultanément.

---

### ③ `ip_mff_cor_opm_2_1` — Multiplicateur injection papillonné (GD) mode 1 @ 0x4E6D4

**Structure :** map 10×12, flat  
**Rôle :** Facteur pour le mode papillonné (Gedrosselt) — actif quand le Valvetronic est désactivé (démarrage froid, mode dégradé). **Point critique N52 :** si seules les maps opm_1_* sont modifiées et opm_2_* restent stock, le moteur injecte selon les valeurs essence dès que le Valvetronic se désactive → lean brutal non anticipé.

| | ◀ Raw stock | ◀ Valeur stock | ▶ Raw E85 | ▶ Valeur E85 |
|---|---|---|---|---|
| `ip_mff_cor_opm_2_1` | 32 770 | 1.016 | **47 407** | **1.473** |

> Même cible que les maps Valvetronic — la structure 10×12 diffère mais toutes les cellules sont flat.

---

### ④ `ip_mff_cor_opm_2_2` — Multiplicateur injection papillonné (GD) mode 2 @ 0x4E7C4

**Structure :** map 10×12, flat  
**Rôle :** Copie du facteur papillonné pour le mode 2. Les 4 maps (opm_1_1, opm_1_2, opm_2_1, opm_2_2) couvrent ensemble tous les modes de fonctionnement du N52 — toutes doivent avoir la même valeur E85.

| | ◀ Raw stock | ◀ Valeur stock | ▶ Raw E85 | ▶ Valeur E85 |
|---|---|---|---|---|
| `ip_mff_cor_opm_2_2` | 32 770 | 1.016 | **47 407** | **1.473** |

> **Vérification des 4 maps :** STFT ralenti chaud (TCO > 80°C) cible ±5%. Si STFT > +15% → raw trop faible. Si STFT < −15% → raw trop élevé.

---

### ⑤ `c_tco_n_mff_cst` — Seuil TCO activation cranking enrichi @ 0x44F2F

**Équation :** `0.75 × raw − 48`  
**Rôle :** Température de liquide de refroidissement en dessous de laquelle les tables de cranking enrichies s'appliquent. L'éthanol s'évapore difficilement sous 25°C (ébullition à 78°C vs −40°C essence) — sans ce seuil relevé, le moteur démarre en mélange pauvre par temps frais.

| | ◀ Raw stock | ◀ Valeur stock | ▶ Raw E85 | ▶ Valeur E85 |
|---|---|---|---|---|
| `c_tco_n_mff_cst` | 87 | 17.25 °C | **97** | **25.00 °C** |

> Détails complets dans [§5 — Démarrage à froid](05_DEMARRAGE_FROID.md)

---

### ⑥ `c_t_ti_dly_fl_1` — Délai enrichissement WOT (MT, copie 1) @ 0x44EC4

**Équation :** `0.010 × raw` (secondes)  
**Rôle :** Délai entre la détection pleine charge et l'application de l'enrichissement WOT. Stock 200 ms — conçu pour éviter des enrichissements transitoires intempestifs sur essence. Sur E85, ce délai crée un bref lean en boucle ouverte à chaque appui franc → à éliminer.

| | ◀ Raw stock | ◀ Valeur stock | ▶ Raw E85 | ▶ Valeur E85 |
|---|---|---|---|---|
| `c_t_ti_dly_fl_1` | 20 | 0.200 s | **0** | **0.000 s** |

> Détails complets dans [§6 — Délai WOT](08_DELAI_WOT.md)

---

### ⑦ `c_t_ti_dly_fl_2` — Délai enrichissement WOT (MT, copie 2) @ 0x44EC6

**Équation :** `0.010 × raw` (secondes)  
**Rôle :** Copie du délai WOT — même logique que c_t_ti_dly_fl_1. Les deux copies doivent être mises à zéro ensemble : si l'une reste à 20, l'ECU peut utiliser l'ancienne valeur selon le contexte d'exécution.

| | ◀ Raw stock | ◀ Valeur stock | ▶ Raw E85 | ▶ Valeur E85 |
|---|---|---|---|---|
| `c_t_ti_dly_fl_2` | 20 | 0.200 s | **0** | **0.000 s** |

> Détails complets dans [§6 — Délai WOT](08_DELAI_WOT.md)

---

### ⑧ `c_iga_ini` — Avance initiale cranking @ 0x44B2A — OPTIONNEL

**Équation :** `0.375 × raw − 35.625` (°CRK avant PMH)  
**Rôle :** Avance appliquée lors du premier cycle d'allumage au cranking. Stock = 6.0°CRK. Sur E85 correctement calibré (§5), le démarrage doit être < 3 tours sans toucher cette valeur. Ne modifier qu'en dernier recours si démarrage > 5 tours malgré ip_mff_cst_opm_* corrects.

| | ◀ Raw stock | ◀ Valeur stock | ▶ Raw E85 | ▶ Valeur E85 |
|---|---|---|---|---|
| `c_iga_ini` (inchangé) | 111 | 6.0 °CRK | **111** | **6.0 °CRK** |
| `c_iga_ini` (option +1°) | 111 | 6.0 °CRK | **114** | **6.75 °CRK** |
| `c_iga_ini` (option +2°) | 111 | 6.0 °CRK | **116** | **7.88 °CRK** |

> Détails complets dans [§9 — Avance cranking](11_AVANCE_CRANKING.md)

