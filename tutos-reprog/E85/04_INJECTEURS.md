# §2 — Mise à l'échelle des injecteurs

> Sur E85, l'AFR stœchiométrique passe de 14.7:1 (essence) à 9.55:1 → **+45% de masse carburant** à injecter à chaque cycle. Le MSV70 N52 utilise 4 maps multiplicatrices (`ip_mff_cor_opm_*`) pour scaler l'injection selon le mode de fonctionnement. Le facteur `c_fac_mff_ti_stnd` (conversion MFF→TI) est géré séparément par le knowledge `/01_injecteurs.json` et ne se modifie pas via TunerPro sur ce bin (overflow XDF).

**Procédure commune aux 4 maps :** TunerPro → ouvrir la map → Ctrl+A → saisir raw **47 407** → valider → répéter ×4.

**Calcul du facteur selon le titre éthanol réel :**

```
AFR_blend = 1 / ( E_fraction/9.0 + (1−E_fraction)/14.7 )
Facteur   = (14.7 / AFR_blend) × 0.94
```

| Teneur éthanol | AFR stœchio | Facteur | Raw cible |
|---|---|---|---|
| E65 | 10.41:1 | ×1.33 | 43 613 |
| E70 | 10.18:1 | ×1.36 | 44 581 |
| E75 | 9.97:1  | ×1.39 | 45 548 |
| **E85 ← cible recommandée** | **9.55:1** | **×1.45** | **47 407** |

> Calibrer sur E85 même avec du E70 réel : en boucle ouverte (WOT, transitions), le LTFT ne corrige pas. Côté riche = safe. Côté pauvre = danger.

---

<a id="p1"></a>
## ① `ip_mff_cor_opm_1_1` — Multiplicateur injection Valvetronic, mode 1

| Champ | Valeur |
|---|---|
| Adresse | 0x4E3D4 |
| Structure | Map 12×16, toutes cellules identiques (flat) |
| Équation | `0.031250 × raw − 1.000` |

**Rôle :** Facteur multiplicateur global de la masse carburant calculée, actif en mode Valvetronic pour la première séquence d'injection. Sur essence, ce facteur est ≈ 1.0 (injection quasi au stœchio). Sur E85, il doit être porté à 1.473 pour compenser l'AFR plus bas. C'est la map principale visible dans la plupart des logiciels de tuning.

**Avant / Après :**

| | ◀ Raw stock | ◀ Valeur stock | ▶ Raw E85 | ▶ Valeur E85 |
|---|---|---|---|---|
| `ip_mff_cor_opm_1_1` | 32 770 | 1.016 | **47 407** | **1.473** |

**Vérification :**

| Signal OBD | ✅ Cible | ⚠️ Action |
|---|---|---|
| STFT ralenti chaud (TCO > 80°C) | −5% à +5% | > +15% → raw +2–3% / < −15% → raw −2–3% |

---

<a id="p2"></a>
## ② `ip_mff_cor_opm_1_2` — Multiplicateur injection Valvetronic, mode 2

| Champ | Valeur |
|---|---|
| Adresse | 0x4E554 |
| Structure | Map 12×16, toutes cellules identiques (flat) |
| Équation | `0.031250 × raw − 1.000` |

**Rôle :** Deuxième copie du facteur pour le mode d'injection 2 en Valvetronic. L'ECU alterne entre les deux modes selon les conditions de charge et de régime. Si cette map diverge d'opm_1_1, l'injection devient asymétrique → STFT oscillant, comportement instable selon la charge.

**Avant / Après :**

| | ◀ Raw stock | ◀ Valeur stock | ▶ Raw E85 | ▶ Valeur E85 |
|---|---|---|---|---|
| `ip_mff_cor_opm_1_2` | 32 770 | 1.016 | **47 407** | **1.473** |

**Vérification :**

| Signal OBD | ✅ Cible | ⚠️ Action |
|---|---|---|
| STFT ralenti chaud (TCO > 80°C) | −5% à +5% | Même action qu'opm_1_1 — les 4 maps sont solidaires |

---

<a id="p3"></a>
## ③ `ip_mff_cor_opm_2_1` — Multiplicateur injection papillonné (GD), mode 1

| Champ | Valeur |
|---|---|
| Adresse | 0x4E6D4 |
| Structure | Map 10×12, toutes cellules identiques (flat) |
| Équation | `0.031250 × raw − 1.000` |

**Rôle :** Facteur pour le mode papillonné (Gedrosselt = throttle body actif, Valvetronic désactivé). Ce mode est actif au démarrage froid et en mode dégradé. **Point critique N52 :** si seules les maps opm_1_* sont modifiées et opm_2_* restent stock à 1.016, le moteur injecte en mode essence dès que le Valvetronic se désactive → lean brutal, non détectable.

**Avant / Après :**

| | ◀ Raw stock | ◀ Valeur stock | ▶ Raw E85 | ▶ Valeur E85 |
|---|---|---|---|---|
| `ip_mff_cor_opm_2_1` | 32 770 | 1.016 | **47 407** | **1.473** |

**Vérification :**

| Signal OBD | ✅ Cible | ⚠️ Action |
|---|---|---|
| STFT au démarrage froid (TCO < 40°C) | −10% à +10% | Instable → vérifier opm_2_* modifiées |

---

<a id="p4"></a>
## ④ `ip_mff_cor_opm_2_2` — Multiplicateur injection papillonné (GD), mode 2

| Champ | Valeur |
|---|---|
| Adresse | 0x4E7C4 |
| Structure | Map 10×12, toutes cellules identiques (flat) |
| Équation | `0.031250 × raw − 1.000` |

**Rôle :** Copie du facteur papillonné pour le mode 2. Les 4 maps ensemble (opm_1_1, opm_1_2, opm_2_1, opm_2_2) couvrent l'intégralité des contextes d'injection du N52. Toutes doivent avoir la même valeur cible E85 — une seule laissée à stock annule partiellement les 3 autres lors des commutations de mode.

**Avant / Après :**

| | ◀ Raw stock | ◀ Valeur stock | ▶ Raw E85 | ▶ Valeur E85 |
|---|---|---|---|---|
| `ip_mff_cor_opm_2_2` | 32 770 | 1.016 | **47 407** | **1.473** |

**Vérification :**

| Signal OBD | ✅ Cible | ⚠️ Action |
|---|---|---|
| STFT toutes conditions (chaud, froid, WOT) | ±10% max | Décalage persistant → vérifier les 4 maps modifiées |

---

<a id="p5"></a>
## ⑤ `c_t_ti_dly_fl_1` — Délai enrichissement WOT, boîte manuelle, copie 1

| Champ | Valeur |
|---|---|
| Adresse | 0x44EC4 |
| Structure | Constante scalaire |
| Équation | `0.010 × raw` (secondes) |

**Rôle :** Délai entre la détection de l'état pleine charge (full load flag) et l'application effective de l'enrichissement WOT sur le temps d'injection. Sur essence, 200 ms évite des enrichissements intempestifs lors de brèves sollicitations pédale. Sur E85, où l'enrichissement WOT est critique pour la protection moteur, ce délai crée un lean bref en boucle ouverte à chaque accélération franche — à éliminer.

**Avant / Après :**

| | ◀ Raw stock | ◀ Valeur stock | ▶ Raw E85 | ▶ Valeur E85 |
|---|---|---|---|---|
| `c_t_ti_dly_fl_1` | 20 | **0.200 s (200 ms)** | **0** | **0.000 s** |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Accélération franche WOT | Enrichissement immédiat, aucun délai perceptible | Trou de richesse → vérifier les 2 copies mises à zéro |

> Détails complets dans [§6 — Délai WOT](08_DELAI_WOT.md)

---

<a id="p6"></a>
## ⑥ `c_t_ti_dly_fl_2` — Délai enrichissement WOT, boîte manuelle, copie 2

| Champ | Valeur |
|---|---|
| Adresse | 0x44EC6 |
| Structure | Constante scalaire |
| Équation | `0.010 × raw` (secondes) |

**Rôle :** Deuxième copie du délai WOT MT. L'ECU utilise ces deux copies dans des contextes d'exécution différents. Si c_t_ti_dly_fl_1 est mis à zéro mais que _2 reste à 20, l'enrichissement peut encore être retardé selon la situation moteur. Les deux doivent être à zéro.

**Avant / Après :**

| | ◀ Raw stock | ◀ Valeur stock | ▶ Raw E85 | ▶ Valeur E85 |
|---|---|---|---|---|
| `c_t_ti_dly_fl_2` | 20 | **0.200 s (200 ms)** | **0** | **0.000 s** |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| WOT répété | Richesse WOT immédiate à chaque sollicitation | Délai intermittent → _2 non modifié |

> Détails complets dans [§6 — Délai WOT](08_DELAI_WOT.md)

---

<a id="p7"></a>
## ⑦ `c_tco_n_mff_cst` — Seuil TCO activation cranking enrichi

| Champ | Valeur |
|---|---|
| Adresse | 0x44F2F |
| Structure | Constante scalaire |
| Équation | `0.75 × raw − 48` (°C) |

**Rôle :** Température de liquide de refroidissement en dessous de laquelle les tables de cranking enrichies (ip_mff_cst_opm_*) s'appliquent. L'éthanol s'évapore difficilement sous 25°C (point d'ébullition 78°C vs −40°C pour l'essence). Sans ce seuil relevé, le moteur démarre en mode cranking stock (sous-enrichi) dès que TCO > 17°C → calage ou démarrage difficile par temps frais.

**Avant / Après :**

| | ◀ Raw stock | ◀ Valeur stock | ▶ Raw E85 | ▶ Valeur E85 |
|---|---|---|---|---|
| `c_tco_n_mff_cst` | 87 | **17.25 °C** | **97** | **25.00 °C** |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Démarrage par temps frais (15–25°C) | < 3 tours, moteur stable | Calage → seuil encore trop bas, passer à raw 103 (30°C) |
| Démarrage moteur chaud (TCO > 80°C) | Démarrage immédiat, STFT stable | STFT riche → seuil trop haut, réduire |

> Détails complets dans [§5 — Démarrage à froid](05_DEMARRAGE_FROID.md)

---

<a id="p8"></a>
## ⑧ Duty cycle injecteur — saturation haut régime (surveillance)

| Champ | Valeur |
|---|---|
| Paramètre calculé | Non encodé dans le bin |
| Formule | `DC (%) = TI_ms / T_cycle_ms × 100` |
| Limite safe | **85%** |

**Rôle :** Le duty cycle est le ratio temps d'ouverture / temps disponible par cycle moteur. Au-delà de 85%, l'injecteur entre en saturation : le débit plafonne, le mélange devient pauvre de façon non compensable. Sur E85, le TI est ×1.45 plus long qu'à l'essence → le risque apparaît plus tôt à haut régime. La saturation est silencieuse (boucle ouverte WOT = pas de sonde lambda active) et directement destructrice.

**Avant / Après — temps disponible par régime :**

| RPM | T_cycle | TI max safe (85%) |
|---|---|---|
| 4000 RPM | 30.0 ms | 25.5 ms |
| 5500 RPM | 21.8 ms | 18.5 ms |
| 6500 RPM | 18.5 ms | **15.7 ms** |

**Avant / Après — duty cycle estimé @ 6500 RPM WOT :**

| Condition | ◀ Stock essence | ▶ E85, injecteurs stock | ▶ E85, puissance augmentée |
|---|---|---|---|
| TI estimé | ~8–10 ms | ~12–14 ms | > 15 ms |
| Duty cycle | ~43–54% | ~65–76% | **> 81%** |
| Statut | Confortable | Dans les limites, marge faible | Zone critique |

**Vérification :**

| PID OBD | ✅ Cible | ⚠️ Action |
|---|---|---|
| Injection Time (Ti) | < 15.7 ms @ 6500 RPM | > 15 ms → injecteurs de remplacement nécessaires avant WOT |
| Duty Cycle calculé | < 85% à tous régimes | Proche 85% → ne pas augmenter la puissance sans nouveaux injecteurs |

**Formule pour nouveaux injecteurs :**

```
ip_mff_cor_nouveau = 1.016 × Facteur_E85 × (Débit_stock / Débit_nouveaux)

Exemple — injecteurs N54 (débit ~30% supérieur) sur E70 :
  = 1.016 × 1.36 × (1 / 1.30) ≈ 1.063  → raw ≈ 34 290
```

---

<a id="p9"></a>
## ⑨ `ip_ti_min` — Dead time injecteur f(tension batterie)

| Champ | Valeur |
|---|---|
| Adresse | (voir XDF — courbe 1D) |
| Structure | Courbe 1D f(Ubatt) |
| Équation | Directe (ms) |

**Rôle :** Dead time électromécanique — délai entre la commande d'ouverture et l'ouverture réelle de l'aiguille. Ce délai dépend de la tension batterie : moins de tension = plus de temps pour magnétiser la bobine. Le MSV70 ajoute ce délai à chaque TI calculé : `TI_total = TI_calculé + dead_time(Ubatt)`. Sur les injecteurs stock Bosch EV14 13537531634, la table est calibrée d'usine. **Avec injecteurs stock : ne pas modifier.** Recalibration nécessaire uniquement si les injecteurs sont remplacés.

**Avant / Après :**

| Tension batterie | ◀ Stock (Bosch EV14 13537531634) | ▶ E85, injecteurs stock | ▶ Nouveaux injecteurs |
|---|---|---|---|
| 7V   | ~1.20 ms | **Identique** | Datasheet constructeur |
| 8V   | ~1.00 ms | **Identique** | Datasheet constructeur |
| 9V   | ~0.85 ms | **Identique** | Datasheet constructeur |
| 10V  | ~0.72 ms | **Identique** | Datasheet constructeur |
| 11V  | ~0.62 ms | **Identique** | Datasheet constructeur |
| 12V  | ~0.55 ms | **Identique** | Datasheet constructeur |
| 13V  | ~0.50 ms | **Identique** | Datasheet constructeur |
| 14V  | ~0.46 ms | **Identique** | Datasheet constructeur |
| 14.4V | ~0.43 ms | **Identique** | Datasheet constructeur |

**Vérification :**

| Symptôme | Cause probable | Action |
|---|---|---|
| STFT très riche dès démarrage | Dead time trop long (trop d'injection) | Réduire dead time nouveaux injecteurs |
| STFT très pauvre dès démarrage | Dead time trop court (pas assez d'injection) | Augmenter dead time nouveaux injecteurs |
| STFT instable au ralenti uniquement | Dead time incorrect à 14V | Vérifier point de la courbe à tension alternateur |

---

## Note — Test au ralenti et sécurité STFT

> **Question fréquente :** peut-on laisser tourner le moteur au ralenti avec une richesse très pauvre le temps de vérifier les STFT ?

| Condition STFT | Durée max | Risque | Action |
|---|---|---|---|
| STFT +5% à +12% | Plusieurs minutes | Dans les limites — ECU compense | Observer, ajuster |
| STFT > +12% (plafond LTFT) | 2–3 min max | Légère surchauffe soupapes échappement | Couper, ajuster facteur |
| STFT > +12% ET LTFT bloqué à +12% | **< 30 secondes** | ECU ne compense plus → mélange pauvre permanent | **Couper immédiatement** |

**Procédure de test sécurisée :**
```
1. Démarrer à chaud (TCO > 80°C, STFT actif)
2. Laisser tourner 30 secondes → lire STFT moyen
3. STFT dans [−10%, +15%] → rouler, laisser LTFT converger
4. STFT > +15% → couper, ajuster raw +2–3%, reflasher, recommencer
5. Ne jamais dépasser 3 min avec STFT > +20%
```
