# §8 — Warm-up lambda au ralenti

> Deux paramètres couvrent la phase de chauffe : `ip_fac_lamb_wup` (tous régimes — **modifié dans le bin E85 de référence**) et `ip_fac_lamb_wup_is` (ralenti uniquement — stock non modifié). Intervenir sur `ip_fac_lamb_wup_is` seulement si le ralenti reste instable malgré les tables cranking et `ip_fac_lamb_wup` corrects.

---

<a id="p1"></a>
## ① `ip_fac_lamb_wup_is` — Facteur warm-up lambda, ralenti uniquement

| Champ | Valeur |
|---|---|
| Adresse | 0x42788 |
| Structure | Map 3×4, uint8 |
| Équation | `0.007813 × raw` |
| Axes | X = MAF (65 / 100 / 200 / 300 mg/stk), Y = RPM (704 / 1216 / 1760 tr/min) |

**Rôle :** Facteur multiplicateur sur la consigne lambda pendant la montée en température, actif uniquement au ralenti (idle speed). Distinct de `ip_fac_lamb_wup` (coldstart) qui couvre toute la phase de chauffe à tous régimes — `ip_fac_lamb_wup_is` n'intervient qu'aux faibles RPM et faible MAF. Sur E85, le ralenti peut être instable pendant le warm-up si la vaporisation de l'éthanol est insuffisante aux basses temperatures. Un léger enrichissement (+2 à +5%) aux cellules basses MAF / bas RPM peut stabiliser.

**Avant / Après :**

```
STOCK (= E85 bin de référence — inchangé) :
MAF (mg/stk) →   65.0  100.0  200.0  300.0
 704 rpm :        1.00   1.00   1.00   1.00
1216 rpm :        1.00   1.00   1.00   1.00
1760 rpm :        1.00   1.00   1.00   1.00

OBJECTIF E85 (si ralenti instable pendant warm-up) :
MAF (mg/stk) →   65.0  100.0  200.0  300.0
 704 rpm :        1.05   1.03   1.00   1.00
1216 rpm :        1.03   1.02   1.00   1.00
1760 rpm :        1.00   1.00   1.00   1.00
```

Encodage uint8 : 1.05 = raw 134 / 1.03 = raw 132 / 1.02 = raw 131 / 1.00 = raw 128.

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Ralenti pendant warm-up (TCO 30–70°C) | Stable, aucune oscillation RPM | Instable → augmenter cellules 704 rpm / 65–100 mg/stk de +1–2% |
| STFT ralenti chaud (TCO > 80°C) | −5% à +5% | Riche → facteur trop élevé, réduire |

> Modifier uniquement si ralenti instable pendant warm-up malgré `ip_mff_cst_opm_*` et `ip_fac_lamb_wup` (coldstart) corrects. Diagnostiquer §5 en priorité.

---

<a id="p2"></a>
## ② `ip_fac_lamb_wup` — Facteur warm-up lambda, tous régimes

| Champ | Valeur |
|---|---|
| Adresse | 0x42764 |
| Structure | Map 6×6, uint8 |
| Équation | `0.007813 × raw` |
| Axes | X = MAF (0.83–5.32 mg/stk), Y = RPM (704–3008 tr/min) |

**Rôle :** Facteur multiplicateur sur la consigne lambda pendant la phase de chauffe, actif à **tous les régimes** (contrairement à `ip_fac_lamb_wup_is` qui ne couvre que le ralenti). S'applique avant que la boucle fermée lambda soit stabilisée. Sur E85, le bin de référence applique +3 à +8% d'enrichissement aux bas régimes (704–2016 RPM) et basse charge, ce qui compense la moins bonne vaporisation de l'éthanol pendant le warm-up. **Ce paramètre EST modifié dans le bin E85 de référence** — stock = tout à 1.000.

**Avant / Après :**

```
STOCK (toutes cellules identiques) :
MAF (mg/stk) →  0.83   1.57   2.33   3.92   4.66   5.32
 704 rpm :       1.000  1.000  1.000  1.000  1.000  1.000
1216 rpm :       1.000  1.000  1.000  1.000  1.000  1.000
1760 rpm :       1.000  1.000  1.000  1.000  1.000  1.000
2016 rpm :       1.000  1.000  1.000  1.000  1.000  1.000
2496 rpm :       1.000  1.000  1.000  1.000  1.000  1.000
3008 rpm :       1.000  1.000  1.000  1.000  1.000  1.000

E85 BIN DE RÉFÉRENCE :
MAF (mg/stk) →  0.83   1.57   2.33   3.92   4.66   5.32
 704 rpm :       1.031  1.063  1.078  1.047  1.070  1.078
1216 rpm :       1.031  1.047  1.070  1.047  1.063  1.078
1760 rpm :       1.031  1.039  1.047  1.031  1.039  1.047
2016 rpm :       1.000  1.031  1.031  1.031  1.031  1.031
2496 rpm :       1.000  1.000  1.000  1.000  1.000  1.000
3008 rpm :       1.000  1.000  1.000  1.000  1.000  1.000
```

Encodage uint8 : 1.078 = raw 138 / 1.063 = raw 136 / 1.047 = raw 134 / 1.039 = raw 133 / 1.031 = raw 132 / 1.000 = raw 128.

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| STFT pendant warm-up (TCO 30–70°C) | −10% à +10% | > +15% → ip_fac_lamb_wup trop faible, augmenter cellules bas RPM |
| STFT moteur chaud (TCO > 80°C) | Non affecté | ip_fac_lamb_wup ne s'applique que pendant le warm-up |
| Ralenti instable warm-up malgré ip_fac_lamb_wup correct | — | Voir `ip_fac_lamb_wup_is` ① pour correction spécifique au ralenti |
