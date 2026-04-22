# §8 — Warm-up lambda au ralenti

> Facteur d'enrichissement lambda spécifique au ralenti pendant la phase de chauffe. Distinct de `ip_fac_lamb_wup` (coldstart) qui couvre tous les régimes. Modification optionnelle — intervenir uniquement si le ralenti est instable pendant le warm-up malgré les tables cranking et chauffe corrects.

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
