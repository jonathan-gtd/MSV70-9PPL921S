# §7 — Consigne Lambda WOT (Richesse Cible)

> 💡 Le N52B30 est **déjà enrichi à λ 0.920 en WOT** (stock). Sur E85, rien d'obligatoire. En option, dé-enrichir légèrement (0.940–0.950) pour gagner un peu de puissance.

### 📋 Tables à modifier

| Paramètre | Adresse | Structure | Modification |
|---|---|---|---|
| `ip_lamb_fl__n` | 0x436A2 | Courbe 1×12, f(RPM) | **Optionnel** — stock λ 0.920 ou monter à 0.940–0.950 |

### 🔨 Procédure

| Option | Action |
|---|---|
| **A — Recommandée** | Ne rien modifier — stock λ 0.920 suffisant et sécurisé |
| **B — Optionnelle** | Cellules 608–4800 rpm → 0.940–0.950 / conserver 5504 et 6496 rpm bas (protection soupapes) |

**Valeurs STOCK `ip_lamb_fl__n` @0x436A2 :**

```
RPM  : 608   992   1216   1600   2016   2496   3008   3520   4128   4800   5504   6496
λ    : 0.920 0.920 0.913  0.920  0.920  0.920  0.920  0.920  0.920  0.920  0.901  0.871
```

> **Conclusion :** Le N52B30 stock est **déjà enrichi à lambda ≈ 0.920 en pleine charge** (et descend à 0.871 à 6500 rpm). Il n'y a **rien à faire** pour « ajouter » de la richesse WOT — elle est déjà présente.

### Comprendre Lambda sur l'E85

```
Lambda = 1.00 sur E85 ≠ Lambda = 1.00 sur essence

Sur essence :  Lambda 1.00 → AFR 14.7:1 → stœchiométrique
Sur E85 :      Lambda 1.00 → AFR 9.8:1  → stœchiométrique E85

Le calculateur gère cela AUTOMATIQUEMENT si :
  1. ip_mff_cor_opm est correctement scalé (×1.33 à ×1.45 selon votre titre éthanol)
  2. La sonde lambda lit correctement (lambda reste relative, pas absolue)
  3. La boucle fermée se régule sur la sonde lambda
```

Le système de boucle fermée se corrige automatiquement si `ip_mff_cor_opm` est correctement calibré.

### Quand (et comment) modifier le lambda WOT sur E85

Le stock est déjà à λ 0.920 en WOT, ce qui est **assez riche**. Sur E85 pur, la chaleur de vaporisation élevée (~840 kJ/kg vs 305 kJ/kg essence) refroidit fortement la chambre et protège mécaniquement contre la détonation. Vous avez donc deux options :

**Option A — Garder `ip_lamb_fl__n` stock.** Le λ 0.920 stock injecte ~9% de carburant en plus qu'à stœchio. Sur E85, cette richesse protège toujours — aucun problème de fiabilité.

**Option B — Dé-enrichir légèrement pour gagner un peu de puissance.**
Sur E85, λ 0.94–0.96 en WOT est un bon compromis (plus proche de stoechio = couple maxi, mais encore assez riche pour une marge de sécurité). Modification proposée sur `ip_lamb_fl__n` :

<a id="pencil-lambda-wot"></a>

### ✏️ Avant / Après (option B — dé-enrichissement WOT)

```
RPM    : 608   992   1216   1600   2016   2496   3008   3520   4128   4800   5504   6496
Stock  : 0.920 0.920 0.913  0.920  0.920  0.920  0.920  0.920  0.920  0.920  0.901  0.871
E85    : 0.950 0.950 0.945  0.950  0.950  0.945  0.945  0.940  0.935  0.930  0.920  0.900
```

> Conserver les cellules 5504 et 6496 rpm à une valeur basse (0.900–0.920) : protection thermique des soupapes d'échappement à très haut régime.

### ✅ Vérification

| Signal | ✅ Cible | ⚠️ Si hors cible |
|---|---|---|
| Lambda WOT (sonde large bande) | **0.90 – 0.95** | — |
| LTFT roulage | **±5%** | > +10% → `ip_mff_cor_opm_*` trop petit |
| | | < −10% → `ip_mff_cor_opm_*` trop grand |

### Corrections LTFT persistants

Si les LTFT restent décalés après stabilisation :
- **LTFT > +10%** : mélange trop pauvre → `ip_mff_cor_opm_*` trop petit, augmenter
- **LTFT < −10%** : mélange trop riche → `ip_mff_cor_opm_*` trop grand, réduire

Dans les deux cas, la correction se fait sur le facteur injecteur, pas sur les consignes lambda.

---


---

## Récapitulatif — Valeurs Avant / Après

### ⑥ ip_lamb_fl__n — Lambda WOT @ 0x436A2 (1×12, f(RPM)) — OPTIONNEL

```
RPM    : 608   992   1216   1600   2016   2496   3008   3520   4128   4800   5504   6496

Stock  : 0.920 0.920 0.913  0.920  0.920  0.920  0.920  0.920  0.920  0.920  0.901  0.871

Option A (recommandée) : LAISSER STOCK — déjà λ 0.920 en WOT, aucune modification

Option B (optionnelle — dé-enrichissement léger pour la puissance) :
E85 B  : 0.950 0.950 0.945  0.950  0.950  0.945  0.945  0.940  0.935  0.930  0.920  0.900
```

---
