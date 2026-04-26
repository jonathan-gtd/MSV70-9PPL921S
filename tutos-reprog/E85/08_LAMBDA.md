# Lambda — Corrections STFT/LTFT, warm-up et richesse WOT

Ce fichier couvre trois aspects liés à la gestion lambda :
1. **Plafonds de correction** (anti-DTC break-in) — obligatoires avant le premier flash
2. **Warm-up lambda** — enrichissement pendant la chauffe
3. **Lambda cible WOT** — optionnel

Les limites LTFT stock (−8% / +12%) sont documentées en fin de fichier — **ne pas modifier**.

---

## OBLIGATOIRE — Anti-DTC break-in

<a id="p1"></a>
## ① `ip_fac_lamb_max_fsd_1` — Plafond correction WRAF instantanée, mode 1

| Champ | Valeur |
|---|---|
| Adresse | (voir XDF) |
| Type | Courbe 6 points |
| Unité | % |

**Rôle :** Limite haute de la correction STFT instantanée autorisée par le système WRAF (Wide Range Air Fuel). Si le STFT monte au-delà de ce plafond, l'ECU peut lever un DTC → voyant moteur. Sur E85 en break-in (0–500 km), le STFT peut atteindre +18 à +22% pendant la convergence — au-delà du plafond stock de 30%. Élargir pendant le rodage, resserrer une fois stabilisé.

**Avant / Après :**

| Cellule | 1 | 2 | 3 | 4 | 5 | 6 |
|---|---|---|---|---|---|---|
| ◀ Stock VB67774 (%) | 100.0 | 35.0 | **30.0** | **30.0** | 35.0 | 100.0 |
| ✅ E85 Break-in (%) | 100.0 | 45.0 | **40.0** | **40.0** | 45.0 | 100.0 |
| ✅ E85 Stabilisé (%) | 100.0 | 38.0 | **35.0** | **35.0** | 38.0 | 100.0 |

> Les cellules 1 et 6 sont déjà à 100% stock. Les cellules centrales à 30% sont le vrai goulot d'étranglement sur E85 (correction E85 ≈ +30–36% sur stoichio).

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Voyant moteur dans les 500 premiers km | Absent | Allumé → cellules 3/4 trop serrées, monter à 40% |
| STFT boucle fermée | ±15% max | > ±30% continu → calibration injecteurs à revoir |

---

<a id="p2"></a>
## ② `ip_fac_lamb_max_fsd_2` — Plafond correction WRAF instantanée, mode 2

| Champ | Valeur |
|---|---|
| Adresse | (voir XDF) |
| Type | Courbe 6 points |
| Unité | % |

**Rôle :** Copie du plafond WRAF pour le mode 2. Même logique qu'opm_1. Les deux modes doivent être cohérents — si mode 1 est élargi et mode 2 reste stock, le voyant peut s'allumer lors des commutations de mode de régulation lambda.

**Avant / Après :**

Identique à `ip_fac_lamb_max_fsd_1` — mêmes valeurs :

| Cellule | 1 | 2 | 3 | 4 | 5 | 6 |
|---|---|---|---|---|---|---|
| ◀ Stock VB67774 (%) | 100.0 | 35.0 | **30.0** | **30.0** | 35.0 | 100.0 |
| ✅ E85 Break-in (%) | 100.0 | 45.0 | **40.0** | **40.0** | 45.0 | 100.0 |
| ✅ E85 Stabilisé (%) | 100.0 | 38.0 | **35.0** | **35.0** | 38.0 | 100.0 |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Voyant moteur intermittent | Absent | Intermittent → mode 2 non modifié, commutation déclenche DTC |

---

<a id="p3"></a>
## ③ `c_lamb_delta_i_max_lam_adj` — Plafond LTFT intégral

| Champ | Valeur |
|---|---|
| Adresse | (voir XDF) |
| Type | Constante scalaire |
| Unité | λ |

**Rôle :** Valeur maximale d'accumulation de l'intégrateur LTFT (long term fuel trim). Si l'adaptation atteint ce plafond, l'ECU ne peut plus compenser et déclare "adaptation at limit" → DTC voyant moteur. Sur E85 en break-in, le LTFT peut vouloir s'accumuler jusqu'à +25% pendant les premiers 200 km. Stock réel lu sur VB67774 : **0.050 λ** — multiplier par 5 avant le premier flash.

**Avant / Après :**

| Phase | ◀ Stock VB67774 | ✅ Break-in (0–500 km) | ✅ Stabilisé (> 500 km) |
|---|---|---|---|
| `c_lamb_delta_i_max_lam_adj` | **0.050 λ** (5%) | **0.25 λ** (25%) | **0.15 λ** (15%) |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| LTFT après 500 km | ±5% | > ±10% → calibration injecteurs à affiner |
| Voyant "adaptation" | Absent | Allumé → LTFT max trop serré, élargir |

---

**Procédure complète prévention DTC break-in :**

```
Avant premier flash E85 :
  1. Monter ip_fac_lamb_max_fsd_1/2 cellules 3/4 à 40%
  2. Monter c_lamb_delta_i_max_lam_adj à 0.25 λ

Après 300–500 km :
  3. Lire LTFT avec ISTA ou scanner OBD → cible ±5%
  4. Si stable → resserrer : cellules FSD à 35%, LTFT max à 0.15 λ
  5. Reflasher — valeurs serrées évitent les faux DTC futurs

Voyant MIL allumé pendant break-in :
  → DTC "fuel trim" → FSD/LTFT trop serrés → élargir et reflasher
  → NE PAS effacer le DTC sans reflasher : il revient immédiatement
```

---

## OBLIGATOIRE — Warm-up lambda

<a id="p4"></a>
## ④ `ip_fac_lamb_wup` — Facteur warm-up lambda, tous régimes

| Champ | Valeur |
|---|---|
| Adresse | 0x42764 |
| Type | Map 6×6 |
| Unité | facteur (sans dimension) |
| Axes | X = MAF (65–500 mg/stk), Y = RPM (704–3008 tr/min) |

**Rôle :** Facteur multiplicateur sur la consigne lambda pendant la phase de chauffe, actif à **tous les régimes**. S'applique avant que la boucle fermée lambda soit stabilisée. Sur E85, enrichissement nécessaire aux bas régimes / basse charge pour compenser la moins bonne vaporisation de l'éthanol pendant le warm-up. **Ce paramètre EST modifié sur E85** — stock = tout à 1.000.

**◀ Avant — Stock (facteur λ)**

| RPM (tr/min) \ MAF (mg/stk) | **65** | **100** | **200** | **300** | **400** | **500** |
|---|---|---|---|---|---|---|
| 704 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 |
| 1216 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 |
| 1760 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 |
| 2016 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 |
| 2496 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 |
| 3008 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 |

**✏️ Delta E85 vs Stock — enrichissement +3 à +8% basses charges (≤ 200 mg/stk, ≤ 2016 rpm)**

| RPM (tr/min) \ MAF (mg/stk) | **65** | **100** | **200** | **300** | **400** | **500** |
|---|---|---|---|---|---|---|
| 704 | **+8%** | **+6%** | **+5%** | **+3%** | 0% | 0% |
| 1216 | **+6%** | **+5%** | **+4%** | **+3%** | 0% | 0% |
| 1760 | **+5%** | **+4%** | **+3%** | 0% | 0% | 0% |
| 2016 | **+3%** | **+3%** | 0% | 0% | 0% | 0% |
| 2496 | 0% | 0% | 0% | 0% | 0% | 0% |
| 3008 | 0% | 0% | 0% | 0% | 0% | 0% |

**✅ Après — E85 (facteur λ)**

| RPM (tr/min) \ MAF (mg/stk) | **65** | **100** | **200** | **300** | **400** | **500** |
|---|---|---|---|---|---|---|
| 704 | **1.08** | **1.06** | **1.05** | **1.03** | 1.00 | 1.00 |
| 1216 | **1.06** | **1.05** | **1.04** | **1.03** | 1.00 | 1.00 |
| 1760 | **1.05** | **1.04** | **1.03** | 1.00 | 1.00 | 1.00 |
| 2016 | **1.03** | **1.03** | 1.00 | 1.00 | 1.00 | 1.00 |
| 2496 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| 3008 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| STFT pendant warm-up (TCO 30–70°C) | −10% à +10% | > +15% → ip_fac_lamb_wup trop faible, augmenter cellules bas RPM |
| STFT moteur chaud (TCO > 80°C) | Non affecté | ip_fac_lamb_wup ne s'applique que pendant le warm-up |

---

## OPTIONNEL

<a id="p5"></a>
## ⑤ `ip_fac_lamb_wup_is` — Facteur warm-up lambda, ralenti uniquement

| Champ | Valeur |
|---|---|
| Adresse | 0x42788 |
| Type | Map 3×4 |
| Unité | facteur (sans dimension) |
| Axes | X = MAF (65–300 mg/stk), Y = RPM (704–1760 tr/min) |

**Rôle :** Facteur multiplicateur sur la consigne lambda pendant le warm-up, actif uniquement au ralenti. Sur E85, le ralenti peut être instable pendant le warm-up si la vaporisation de l'éthanol est insuffisante aux basses températures. Un léger enrichissement (+2 à +5%) aux cellules basses MAF / bas RPM peut stabiliser. **Modifier uniquement si ralenti instable pendant warm-up malgré `ip_mff_cst_opm_*` et `ip_fac_lamb_wup` corrects.**

**◀ Avant — Stock (facteur λ)**

| RPM (tr/min) \ MAF (mg/stk) | 65.0 | 100.0 | 200.0 | 300.0 |
|---|---|---|---|---|
| 704 | 1.000 | 1.000 | 1.000 | 1.000 |
| 1216 | 1.000 | 1.000 | 1.000 | 1.000 |
| 1760 | 1.000 | 1.000 | 1.000 | 1.000 |

**✏️ +2 à +5% cellules 704–1216 rpm / 65–100 mg/stk (si ralenti instable pendant warm-up)**

**✅ Après — E85 (facteur λ)**

| RPM (tr/min) \ MAF (mg/stk) | **65** | **100** | **200** | **300** |
|---|---|---|---|---|
| 704 | **1.050** | **1.031** | 1.000 | 1.000 |
| 1216 | **1.031** | **1.016** | 1.000 | 1.000 |
| 1760 | 1.000 | 1.000 | 1.000 | 1.000 |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Ralenti pendant warm-up (TCO 30–70°C) | Stable, aucune oscillation RPM | Instable → augmenter cellules 704 rpm / 65–100 mg/stk de +1–2% |
| STFT ralenti chaud (TCO > 80°C) | −5% à +5% | Riche → facteur trop élevé, réduire |

---

<a id="p6"></a>
## ⑥ `ip_lamb_fl__n` — Lambda cible pleine charge f(RPM)

| Champ | Valeur |
|---|---|
| Adresse | 0x436A2 |
| Type | Courbe 1×12 |
| Unité | λ |
| Axes | X = RPM (608–6496 tr/min) |

**Rôle :** Consigne de richesse ciblée par le calculateur en mode pleine charge (WOT). Stock N52B30 : déjà enrichi à λ 0.920 en WOT (descend à 0.871 à 6500 RPM). Sur E85, cette richesse protège le moteur — **rien d'obligatoire à modifier**. Option B : dé-enrichir légèrement (λ 0.940–0.950) pour gagner un peu de puissance en exploitant la chaleur de vaporisation de l'éthanol.

**◀ Avant — Stock (λ)**

| RPM (tr/min) | 608 | 992 | 1216 | 1600 | 2016 | 2496 | 3008 | 3520 | 4128 | 4800 | 5504 | 6496 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| λ | 0.920 | 0.920 | 0.913 | 0.920 | 0.920 | 0.920 | 0.920 | 0.920 | 0.920 | 0.920 | 0.901 | 0.871 |

Option A (recommandée) : **laisser stock** — λ 0.920 WOT déjà présent, E85 ne nécessite pas d'enrichissement supplémentaire.

**✅ Après — Option B : dé-enrichissement léger (gain puissance)**

| RPM (tr/min) | 608 | 992 | 1216 | 1600 | 2016 | 2496 | 3008 | 3520 | 4128 | 4800 | 5504 | 6496 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| λ | **0.950** | **0.950** | **0.945** | **0.950** | **0.950** | **0.945** | **0.945** | **0.940** | **0.935** | **0.930** | **0.920** | **0.900** |

> Conserver les cellules 5504 et 6496 rpm à λ bas (0.900–0.920) : protection thermique soupapes à très haut régime.

**Vérification :**

| Signal | ✅ Cible | ⚠️ Action |
|---|---|---|
| Lambda WOT (sonde large bande) | 0.90–0.95 | Hors plage → ajuster ip_lamb_fl__n |
| LTFT roulage | ±5% | > +10% → ip_mff_cor_opm trop petit |

---

## INFO — Limites LTFT stock MSV70 (ne pas modifier)

> **Les vraies limites sur ce bin sont −8% / +12% — asymétriques.** Si le LTFT plafonne en permanence : affiner le facteur injecteur (`ip_mff_cor_opm_*`) plutôt que d'élargir ces limites.

| Paramètre | Adresse | Valeur stock | Modification E85 |
|---|---|---|---|
| `c_fac_max_h_rng_lam_ad` — limite haute LTFT haute charge | 0x47F4C | **+12.0%** | Ne pas modifier — surveiller |
| `c_fac_max_l_rng_lam_ad` — limite haute LTFT basse charge | 0x47F4E | **+12.0%** | Ne pas modifier — surveiller |
| `c_fac_min_h_rng_lam_ad` — limite basse LTFT haute charge | 0x47F50 | **92.0% (= −8%)** | Ne pas modifier |
| `c_fac_min_l_rng_lam_ad` — limite basse LTFT basse charge | 0x47F52 | **92.0% (= −8%)** | Ne pas modifier |
| `c_lam_mv_dyw_dly` — fenêtre dynamique STFT | 0x44B3E | **±7.7%** | Ne pas modifier |

L'asymétrie (−8% / +12%) est intentionnelle — le MSV70 tolère mieux un mélange légèrement riche qu'un mélange pauvre. Calibrer `ip_mff_cor_opm_*` pour que le LTFT se stabilise à ±5% — jamais aux limites.

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| LTFT haute charge | −8% à +12% | Plafonne à +12% → ip_mff_cor_opm_* trop petit, augmenter |
| LTFT plafonné à −8% en permanence | Absent | Plafonné → ip_mff_cor_opm_* trop élevé, réduire vers E70 (raw 44 581) |
| LTFT après 500 km | ±5% | Instable → convergence en cours ou calibration à affiner |
