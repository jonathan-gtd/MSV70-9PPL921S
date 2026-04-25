# §8 — Warm-up lambda au ralenti

> Deux paramètres couvrent la phase de chauffe : `ip_fac_lamb_wup` (tous régimes — **modifié dans le bin E85 de référence**) et `ip_fac_lamb_wup_is` (ralenti uniquement — stock non modifié). Intervenir sur `ip_fac_lamb_wup_is` seulement si le ralenti reste instable malgré les tables cranking et `ip_fac_lamb_wup` corrects.

---

<a id="p1"></a>
## ① `ip_fac_lamb_wup` — Facteur warm-up lambda, tous régimes

| Champ | Valeur |
|---|---|
| Adresse | 0x2764 |
| Structure | Map 6×6 |
| Axes | X = MAF (mg/stk), Y = RPM |

**Rôle :** Facteur multiplicateur sur la consigne lambda pendant la phase de chauffe, actif à **tous les régimes**. S'applique avant que la boucle fermée lambda soit stabilisée. Sur E85, enrichissement nécessaire aux bas régimes / basse charge pour compenser la moins bonne vaporisation de l'éthanol pendant le warm-up. **Ce paramètre EST modifié sur E85** — stock = tout à 1.000.

**◀ Avant — Stock (facteur λ)** *(axes lus sur VB67774)*

| RPM (tr/min) \ MAF (mg/stk) | **65** | **100** | **200** | **300** | **400** | **500** |
|---|---|---|---|---|---|---|
| 704 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 |
| 1216 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 |
| 1760 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 |
| 2016 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 |
| 2496 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 |
| 3008 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 |

**✏️ Opération — enrichissement +3 à +8% cellules bas RPM / basse charge (≤ 200 mg/stk, ≤ 2016 rpm)**

**✅ Après — E85 (facteur λ)**

| RPM (tr/min) \ MAF (mg/stk) | **65** | **100** | **200** | **300** | **400** | **500** |
|---|---|---|---|---|---|---|
| 704 | **1.078** | **1.063** | **1.047** | **1.031** | 1.000 | 1.000 |
| 1216 | **1.063** | **1.047** | **1.039** | **1.031** | 1.000 | 1.000 |
| 1760 | **1.047** | **1.039** | **1.031** | 1.000 | 1.000 | 1.000 |
| 2016 | **1.031** | **1.031** | 1.000 | 1.000 | 1.000 | 1.000 |
| 2496 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 |
| 3008 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 | 1.000 |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| STFT pendant warm-up (TCO 30–70°C) | −10% à +10% | > +15% → ip_fac_lamb_wup trop faible, augmenter cellules bas RPM |
| STFT moteur chaud (TCO > 80°C) | Non affecté | ip_fac_lamb_wup ne s'applique que pendant le warm-up |
| Ralenti instable warm-up malgré ip_fac_lamb_wup correct | — | Voir `ip_fac_lamb_wup_is` ② pour correction spécifique au ralenti |

---

<a id="p2"></a>
## ② `ip_fac_lamb_wup_is` — Facteur warm-up lambda, ralenti uniquement

| Champ | Valeur |
|---|---|
| Adresse | 0x42788 |
| Structure | Map 3×4 |
| Axes | X = MAF (mg/stk), Y = RPM |

**Rôle :** Facteur multiplicateur sur la consigne lambda pendant la montée en température, actif uniquement au ralenti (idle speed). Sur E85, le ralenti peut être instable pendant le warm-up si la vaporisation de l'éthanol est insuffisante aux basses températures. Un léger enrichissement (+2 à +5%) aux cellules basses MAF / bas RPM peut stabiliser.

**◀ Avant — Stock (facteur λ)**

| RPM (tr/min) \ MAF (mg/stk) | 65.0 | 100.0 | 200.0 | 300.0 |
|---|---|---|---|---|
| 704 | 1.000 | 1.000 | 1.000 | 1.000 |
| 1216 | 1.000 | 1.000 | 1.000 | 1.000 |
| 1760 | 1.000 | 1.000 | 1.000 | 1.000 |

**✏️ Opération — +2 à +5% cellules 704–1216 rpm / 65–100 mg/stk (si ralenti instable pendant warm-up)**

**✅ Après — E85 (facteur λ)**

| RPM (tr/min) \ MAF (mg/stk) | 65.0 | 100.0 | 200.0 | 300.0 |
|---|---|---|---|---|
| 704 | **1.050** | **1.031** | 1.000 | 1.000 |
| 1216 | **1.031** | **1.016** | 1.000 | 1.000 |
| 1760 | 1.000 | 1.000 | 1.000 | 1.000 |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Ralenti pendant warm-up (TCO 30–70°C) | Stable, aucune oscillation RPM | Instable → augmenter cellules 704 rpm / 65–100 mg/stk de +1–2% |
| STFT ralenti chaud (TCO > 80°C) | −5% à +5% | Riche → facteur trop élevé, réduire |

> Modifier uniquement si ralenti instable pendant warm-up malgré `ip_mff_cst_opm_*` et `ip_fac_lamb_wup` corrects. Diagnostiquer §5 en priorité.
