# §12 — Limites LTFT / STFT

> **⚠️ Les vraies limites MSV70 sont −8% / +12% — pas ±25%**

Les limites réelles de ce bin MSV70 sont asymétriques :

| Trim | Valeur réelle |
|---|---|
| LTFT max positif | **+12 %** |
| LTFT max négatif | **−8 %** |
| STFT fenêtre dynamique | **±7.7 %** |

L'asymétrie (−8%/+12%) est intentionnelle : le MSV70 tolère mieux un mélange légèrement riche qu'un mélange pauvre.

---

### ① `c_fac_max_h_rng_lam_ad` — Limite haute LTFT, haute charge @ 0x47F4C

**Rôle :** Plafond de correction LTFT (long term fuel trim) pour les conditions de haute charge. Si l'adaptation atteint cette limite, l'ECU ne peut plus corriger vers le riche au-delà → mélange pauvre non compensé en haute charge.

| | ◀ Raw stock | ◀ Valeur stock | ▶ E85 |
|---|---|---|---|
| `c_fac_max_h_rng_lam_ad` | (voir bin) | **+12.0 %** | **Ne pas modifier** — surveiller |

> Sur E85 calibré E85 (raw 47407) avec carburant E70 réel → LTFT ≈ −6 à −8% : dans les limites. Calibré E85 + carburant E60 hivernal → LTFT cible ≈ −13% : dépasse le plafond → affiner ip_mff_cor_opm vers E70 (raw 44 581).

---

### ② `c_fac_max_l_rng_lam_ad` — Limite haute LTFT, basse charge @ 0x47F4E

**Rôle :** Plafond LTFT pour les conditions de basse charge (ralenti, décélération légère). Même logique que haute charge — deux plages séparées pour une meilleure précision de l'adaptation.

| | ◀ Raw stock | ◀ Valeur stock | ▶ E85 |
|---|---|---|---|
| `c_fac_max_l_rng_lam_ad` | (voir bin) | **+12.0 %** | **Ne pas modifier** — surveiller |

> En basse charge, les LTFT E85 sont généralement moins décalés qu'en haute charge (boucle fermée plus stable au ralenti).

---

### ③ `c_fac_min_h_rng_lam_ad` — Limite basse LTFT, haute charge @ 0x47F50

**Rôle :** Plancher de correction LTFT en haute charge. Encodé à 92% → correction minimale = −8% (100% − 92%). Si le moteur est trop riche, l'ECU peut corriger jusqu'à −8% maximum.

| | ◀ Raw stock | ◀ Valeur stock | ▶ E85 |
|---|---|---|---|
| `c_fac_min_h_rng_lam_ad` | (voir bin) | **92.0 % (= −8%)** | **Ne pas modifier** |

> Planche vers le riche : si le LTFT atteint −8% en permanence, le facteur injecteur est trop élevé. Réduire ip_mff_cor_opm de 2–3% (raw 44 581 pour E70).

---

### ④ `c_fac_min_l_rng_lam_ad` — Limite basse LTFT, basse charge @ 0x47F52

**Rôle :** Plancher LTFT pour la basse charge. Même valeur que haute charge (92% = −8%).

| | ◀ Raw stock | ◀ Valeur stock | ▶ E85 |
|---|---|---|---|
| `c_fac_min_l_rng_lam_ad` | (voir bin) | **92.0 % (= −8%)** | **Ne pas modifier** |

---

### ⑤ `c_lam_mv_dyw_dly` — Fenêtre dynamique STFT @ 0x44B3E

**Rôle :** Amplitude de la fenêtre de correction court terme (STFT). Sur ce bin : ±7.7%. Le STFT oscille à l'intérieur de cette fenêtre en boucle fermée. Si le STFT sort régulièrement de ±7.7%, le LTFT prend le relais pour recentrer.

| | ◀ Raw stock | ◀ Valeur stock | ▶ E85 |
|---|---|---|---|
| `c_lam_mv_dyw_dly` | (voir bin) | **±7.7 %** | **Ne pas modifier** |

> Sur E85 correctement calibré, le STFT doit osciller dans ±5% et le LTFT se stabiliser à ±3–5% après 500 km.

---

**Verdict global : ne pas modifier ces limites.** Si le LTFT plafonne à −8% en permanence : affiner `ip_mff_cor_opm` vers E70 (raw 44 581) selon le titre réel de vos stations.

---

