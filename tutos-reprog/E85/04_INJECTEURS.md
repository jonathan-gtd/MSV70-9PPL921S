# §2 — Mise à l'échelle des injecteurs

> Sur E85, l'AFR stœchiométrique passe de 14.7:1 (essence) à 9.55:1 → **+45% de masse carburant** à injecter à chaque cycle. Le MSV70 N52 utilise 4 maps multiplicatrices (`ip_mff_cor_opm_*`) pour scaler l'injection selon le mode de fonctionnement. Le facteur `c_fac_mff_ti_stnd` (conversion MFF→TI) est géré séparément par le knowledge `/01_injecteurs.json` et ne se modifie pas via TunerPro sur ce bin (overflow XDF).

**Procédure commune aux 4 maps :** TunerPro → ouvrir la map → Ctrl+A → saisir **1.473** → valider → répéter ×4.

**Calcul du facteur selon le titre éthanol réel :**

```
AFR_blend = 1 / ( E_fraction/9.0 + (1−E_fraction)/14.7 )
Facteur   = (14.7 / AFR_blend) × 0.94
```

| Teneur éthanol | AFR stœchio | Facteur cible |
|---|---|---|
| E65 | 10.41:1 | ×1.33 |
| E70 | 10.18:1 | ×1.36 |
| E75 | 9.97:1  | ×1.39 |
| **E85 ← cible recommandée** | **9.55:1** | **×1.45** |

> Calibrer sur E85 même avec du E70 réel : en boucle ouverte (WOT, transitions), le LTFT ne corrige pas. Côté riche = safe. Côté pauvre = danger.

---

<a id="p1"></a>
## ① `ip_mff_cor_opm_1_1` — Multiplicateur injection Valvetronic, mode 1

| Champ | Valeur |
|---|---|
| Adresse | 0x4E3D4 |
| Structure | Map 12×16, toutes cellules identiques (flat) |

**Rôle :** Facteur multiplicateur global de la masse carburant calculée, actif en mode Valvetronic pour la première séquence d'injection. Sur essence, ce facteur est ≈ 1.0 (injection quasi au stœchio). Sur E85, il doit être porté à 1.473 pour compenser l'AFR plus bas. C'est la map principale visible dans la plupart des logiciels de tuning.

**Avant / Après :**

| | ◀ Avant — Stock | ✅ Après — E85 |
|---|---|---|
| `ip_mff_cor_opm_1_1` | 1.016 | **1.473** |

**Vérification :**

| Signal OBD | ✅ Cible | ⚠️ Action |
|---|---|---|
| STFT ralenti chaud (TCO > 80°C) | −5% à +5% | > +15% → augmenter la valeur de 2–3% / < −15% → réduire de 2–3% |

---

<a id="p2"></a>
## ② `ip_mff_cor_opm_1_2` — Multiplicateur injection Valvetronic, mode 2

| Champ | Valeur |
|---|---|
| Adresse | 0x4E554 |
| Structure | Map 12×16, toutes cellules identiques (flat) |

**Rôle :** Deuxième copie du facteur pour le mode d'injection 2 en Valvetronic. L'ECU alterne entre les deux modes selon les conditions de charge et de régime. Si cette map diverge d'opm_1_1, l'injection devient asymétrique → STFT oscillant, comportement instable selon la charge.

**Avant / Après :**

| | ◀ Avant — Stock | ✅ Après — E85 |
|---|---|---|
| `ip_mff_cor_opm_1_2` | 1.016 | **1.473** |

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

**Rôle :** Facteur pour le mode papillonné (Gedrosselt = throttle body actif, Valvetronic désactivé). Ce mode est actif au démarrage froid et en mode dégradé. **Point critique N52 :** si seules les maps opm_1_* sont modifiées et opm_2_* restent stock à 1.016, le moteur injecte en mode essence dès que le Valvetronic se désactive → lean brutal, non détectable.

**Avant / Après :**

| | ◀ Avant — Stock | ✅ Après — E85 |
|---|---|---|
| `ip_mff_cor_opm_2_1` | 1.016 | **1.473** |

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

**Rôle :** Copie du facteur papillonné pour le mode 2. Les 4 maps ensemble (opm_1_1, opm_1_2, opm_2_1, opm_2_2) couvrent l'intégralité des contextes d'injection du N52. Toutes doivent avoir la même valeur cible E85 — une seule laissée à stock annule partiellement les 3 autres lors des commutations de mode.

**Avant / Après :**

| | ◀ Avant — Stock | ✅ Après — E85 |
|---|---|---|
| `ip_mff_cor_opm_2_2` | 1.016 | **1.473** |

**Vérification :**

| Signal OBD | ✅ Cible | ⚠️ Action |
|---|---|---|
| STFT toutes conditions (chaud, froid, WOT) | ±10% max | Décalage persistant → vérifier les 4 maps modifiées |

---

<a id="p5"></a>
## ⑤ `ip_ti_min` — Dead time injecteur f(tension batterie)

| Champ | Valeur |
|---|---|
| Adresse | (voir XDF — courbe 1D) |
| Structure | Courbe 1D f(Ubatt) |

**Rôle :** Dead time électromécanique — délai entre la commande d'ouverture et l'ouverture réelle de l'aiguille. Ce délai dépend de la tension batterie : moins de tension = plus de temps pour magnétiser la bobine. Le MSV70 ajoute ce délai à chaque TI calculé : `TI_total = TI_calculé + dead_time(Ubatt)`. Sur les injecteurs stock Bosch EV14 13537531634, la table est calibrée d'usine. **Avec injecteurs stock : ne pas modifier.** Recalibration nécessaire uniquement si les injecteurs sont remplacés.

**Avant / Après :**

| Tension batterie | ◀ Avant — Stock (Bosch EV14 13537531634) | ✅ Après — E85, injecteurs stock | ▶ Nouveaux injecteurs |
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
4. STFT > +15% → couper, augmenter la valeur de 2–3%, reflasher, recommencer
5. Ne jamais dépasser 3 min avec STFT > +20%
```
