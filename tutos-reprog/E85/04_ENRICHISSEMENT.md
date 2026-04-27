# Enrichissement de base — Facteur injecteur E85

Sur E85, l'AFR stœchiométrique passe de 14.7:1 (essence) à 9.55:1 → **+45% de masse carburant** à injecter à chaque cycle. Le MSV70 N52 utilise 4 maps multiplicatrices (`ip_mff_cor_opm_*`) qui couvrent l'intégralité des contextes d'injection. **Toutes les 4 doivent être modifiées simultanément** — une seule laissée à stock annule partiellement les 3 autres lors des commutations de mode.

**Calcul du facteur selon le titre éthanol réel :**

```
AFR_blend = 1 / ( E_fraction/9.0 + (1−E_fraction)/14.7 )
Facteur   = (14.7 / AFR_blend) × 0.94
```

| Teneur éthanol | AFR stœchio | Facteur cible | Valeur à saisir dans le bin |
|---|---|---|---|
| E65 | 10.41:1 | ×1.33 | 1.351 |
| E70 | 10.18:1 | ×1.36 | 1.382 |
| E75 | 9.97:1  | ×1.39 | 1.412 |
| **E85 ← cible recommandée** | **9.55:1** | **×1.45** | **1.473** |

> La colonne "Valeur à saisir" = stock VB67774 (1.016) × facteur cible. Exemple E85 : 1.016 × 1.45 = **1.473**. C'est ce nombre qu'on entre dans TunerPro, pas le facteur.

> Calibrer sur E85 même avec du E70 réel : en boucle ouverte (WOT, transitions), le LTFT ne corrige pas. Côté riche = safe. Côté pauvre = danger.

**Procédure commune aux 4 maps :** TunerPro → ouvrir la map → Ctrl+A → saisir **1.473** → valider → répéter ×4. (Pour E70 : saisir 1.382. Pour E75 : 1.412.)

---

<a id="p1"></a>
## ① `ip_mff_cor_opm_1_1` — Multiplicateur injection Valvetronic, mode 1

| Champ | Valeur |
|---|---|
| Adresse | 0x4E3D4 |
| Type | Map 12×16 (flat — toutes cellules identiques) |
| Unité | facteur (sans dimension) |

**Rôle :** Facteur multiplicateur global de la masse carburant calculée, actif en mode Valvetronic pour la première séquence d'injection. Sur essence ≈ 1.016. Sur E85, porter à 1.473 pour compenser l'AFR plus bas. C'est la map principale visible dans la plupart des logiciels de tuning.

**Avant / Après :**

| | ◀ Stock VB67774 | ✅ E85 |
|---|---|---|
| `ip_mff_cor_opm_1_1` | 1.016 | **1.473** |

**Vérification :**

| Signal OBD | ✅ Cible | ⚠️ Action |
|---|---|---|
| STFT ralenti chaud (TCO > 80°C) | −5% à +5% | > +15% → augmenter de 2–3% / < −15% → réduire de 2–3% |

---

<a id="p2"></a>
## ② `ip_mff_cor_opm_1_2` — Multiplicateur injection Valvetronic, mode 2

| Champ | Valeur |
|---|---|
| Adresse | 0x4E554 |
| Type | Map 12×16 (flat — toutes cellules identiques) |
| Unité | facteur (sans dimension) |

**Rôle :** Deuxième copie du facteur en mode Valvetronic. L'ECU alterne entre les deux modes selon les conditions de charge et de régime. Si cette map diverge d'opm_1_1, l'injection devient asymétrique → STFT oscillant selon la charge.

**Avant / Après :**

| | ◀ Stock VB67774 | ✅ E85 |
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
| Type | Map 10×12 (flat — toutes cellules identiques) |
| Unité | facteur (sans dimension) |

**Rôle :** Facteur pour le mode papillonné (Gedrosselt = Valvetronic désactivé). Actif au démarrage froid et en mode dégradé. **Point critique N52 :** si seules les maps opm_1_\* sont modifiées et opm_2_\* restent à 1.016, le moteur injecte en mode essence dès que le Valvetronic se désactive → lean brutal, non détectable par la sonde lambda.

**Avant / Après :**

| | ◀ Stock VB67774 | ✅ E85 |
|---|---|---|
| `ip_mff_cor_opm_2_1` | 1.016 | **1.473** |

**Vérification :**

| Signal OBD | ✅ Cible | ⚠️ Action |
|---|---|---|
| STFT au démarrage froid (TCO < 40°C) | −10% à +10% | Instable → vérifier que opm_2_\* sont bien modifiées |

---

<a id="p4"></a>
## ④ `ip_mff_cor_opm_2_2` — Multiplicateur injection papillonné (GD), mode 2

| Champ | Valeur |
|---|---|
| Adresse | 0x4E7C4 |
| Type | Map 10×12 (flat — toutes cellules identiques) |
| Unité | facteur (sans dimension) |

**Rôle :** Copie du facteur papillonné pour le mode 2. Les 4 maps ensemble (opm_1_1, opm_1_2, opm_2_1, opm_2_2) couvrent l'intégralité des contextes d'injection du N52. Toutes doivent avoir la même valeur cible E85 — une seule laissée à stock annule partiellement les 3 autres lors des commutations de mode.

**Avant / Après :**

| | ◀ Stock VB67774 | ✅ E85 |
|---|---|---|
| `ip_mff_cor_opm_2_2` | 1.016 | **1.473** |

**Vérification :**

| Signal OBD | ✅ Cible | ⚠️ Action |
|---|---|---|
| STFT toutes conditions (chaud, froid, WOT) | ±10% max | Décalage persistant → vérifier les 4 maps toutes à 1.473 |
