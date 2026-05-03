# Enrichissement de base — Facteur global injection E85

Sur E85, l'AFR stœchiométrique passe de 14.7:1 (essence) à 9.55:1 → **+54% de masse carburant** à injecter à chaque cycle. Le MSV70 dispose d'un scalaire global `c_ti_as_global` qui multiplie le temps d'injection dans **tous les contextes** (ralenti, charge, WOT, cranking, after-start). C'est le paramètre le plus direct pour une conversion E85 propre.

**Calcul du facteur selon le titre éthanol réel :**

```
Facteur théorique = 14.7 / AFR_stoich_blend
```

| Teneur éthanol | AFR stœchio | Facteur théorique | `c_ti_as_global` retenu |
|---|---|---|---|
| E65 | 10.41:1 | 1.41 | 1.41 |
| E70 | 10.18:1 | 1.44 | 1.44 |
| E75 | 9.97:1  | 1.47 | 1.47 |
| **E85 ← cible recommandée** | **9.55:1** | **1.54** | **1.47** |

> Le facteur retenu E85 est 1.47 (légèrement en-dessous du théorique 1.54) : le LTFT en boucle fermée finalise l'ajustement. Côté riche = safe. Côté pauvre = danger.

> Calibrer sur E85 même avec du E70 réel : en boucle ouverte (WOT, transitions), le LTFT ne corrige pas.

---

## `c_ti_as_global` — Multiplicateur global temps injection

| Champ | Valeur |
|---|---|
| Adresse | 0x45AE2 (partial) |
| Type | Scalaire uint16 big-endian |
| Unité | facteur (sans dimension) |
| Formule XDF | `0.000244 × raw` |

**Avant / Après :**

| | ◀ Stock S7581293 | ✅ E85 |
|---|---|---|
| `c_ti_as_global` | **0.999** | **1.47** |

**Procédure :** TunerPro → ouvrir `c_ti_as_global` → saisir **1.47** → valider.

---

**Vérification :**

| Signal OBD | ✅ Cible | ⚠️ Action |
|---|---|---|
| STFT ralenti chaud (TCO > 80°C) | −5% à +5% | > +15% stable → augmenter `c_ti_as_global` de +0.03 |
| STFT ralenti chaud | | < −15% stable → réduire de −0.03 |
| LTFT après 50 km | −5% à +8% | Dérive > +15% → c_ti_as_global trop faible |
| LTFT après 50 km | | Dérive < −10% stable → c_ti_as_global trop élevé |
