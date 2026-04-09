# TUTO RPM Protection : Modification des Limiteurs de Régime — Siemens MSV70 / N52B30

> **Véhicule ciblé :** BMW E90/E91/E92/E93 — Moteur N52B30 — Calculateur Siemens MSV70  
> **Fichier de base :** VB67774_921S_Full.bin  
> **Version :** 1.0 — Données réelles extraites du bin — 2026-04-08

---

## 📌 Pourquoi modifier les limiteurs RPM ?

Le N52B30 a une plage rouge à **7000 RPM** et une limite électronique à ~6980 RPM (MT). Les raisons de modifier ces valeurs sont :

- **Protection moteur renforcée** : abaisser le cut à 6500–6600 RPM pour protéger un moteur à kilométrage élevé ou après révision
- **Conversion E85** : sur éthanol, il n'y a pas de raison de monter plus haut — la puissance supplémentaire E85 vient de l'avance, pas du régime
- **Piste / trackday** : certains pilotes préfèrent un cut franc plus bas pour éviter les rebonds de limiteur
- **Moteur modifié** : bielle forgée, pistons, arbre à cames — le cut peut être remonté au-delà du stock

> **Note :** Si l'objectif est uniquement de protéger le moteur à froid (protection thermique pendant la montée en température), agir sur **ip_n_max_1__tco** et **ip_n_max_2__tco** qui limitent le régime en fonction de la TCO. Les tables id_n_max_* s'appliquent **en permanence** quel que soit la température.

---

## ⚙️ Paramètres à modifier

### Groupe 1 — Limite RPM principale f(rapport de boîte)

Ces tables définissent le régime maximum autorisé pour chaque rapport. **9 points** = 9 positions possibles (généralement : 1ère, 2ème, 3ème, 4ème, 5ème, 6ème, marche arrière + modes spéciaux).

| Paramètre | Description | Boîte |
|---|---|---|
| `id_n_max_mt` | Limite de base | MT (manuelle) |
| `id_n_max_at` | Limite de base | AT (automatique) |
| `id_n_max_h_mt` | Limite haute temporaire | MT |
| `id_n_max_h_at` | Limite haute temporaire | AT |

#### Valeurs stock 9PPL921S

L'axe X est un **index Siemens** (0–8), pas un numéro de rapport direct :

| Index | Signification |
|-------|--------------|
| 0 | Neutre / rapport non détecté |
| 1 | 1ère |
| 2 | 2ème |
| 3 | 3ème |
| 4 | 4ème |
| 5 | 5ème |
| 6 | 6ème |
| 7 | Marche arrière |
| 8 | N/A / indéfini |

**id_n_max_mt** (MT basic) :
```
Index :   0      1      2      3      4      5      6      7 (R)  8 (N/A)
         [N]   [1st]  [2nd]  [3rd]  [4th]  [5th]  [6th]  [R]    [N/A]
RPM   : 6800   6980   6980   6980   6980   6800   6980   6400   6400
```
> Le 5ème rapport (idx 5) est à 6800 RPM — calibration BMW délibérée.

**id_n_max_at** (AT basic) :
```
Index :   0      1      2      3      4      5      6      7 (R)  8 (N/A)
RPM   : 6400   7000   7000   7000   7000   6800   7000   6400   6400
```

**id_n_max_h_mt** (MT high — fenêtre temporaire) :
```
RPM   : 6802   6982   6982   6982   6982   6982   6982   6600   6600
```

**id_n_max_h_at** (AT high) :
```
RPM   : 6600   7100   7100   7100   7100   7000   7000   6600   6600
```

---

### Groupe 2 — Limite RPM f(température coolant TCO)

Ces courbes **plafonnent le régime pendant le réchauffage moteur**. Elles s'appliquent en parallèle des tables f(gear) — c'est la valeur la plus restrictive des deux qui s'applique.

| Paramètre | Zone | Description |
|---|---|---|
| `ip_n_max_1__tco` | Orange (avertissement) | 200 RPM sous le hard cut — le voyant s'allume |
| `ip_n_max_2__tco` | Rouge (hard cut) | Coupe injection effective |

#### Valeurs stock 9PPL921S

| TCO | Orange (avertissement) | Rouge (hard cut) |
|-----|----------------------|-----------------|
| **−30°C** | 5200 RPM | 5400 RPM |
| **0°C** | 5600 RPM | 5800 RPM |
| **30°C** | 6000 RPM | 6200 RPM |
| **90°C** | 6820 RPM | 7020 RPM |

---

## 🔧 Comment modifier dans TunerPro RT

### Modifier id_n_max_mt / id_n_max_at

1. Ouvrir TunerPro RT → charger le XDF + BIN
2. Dans l'arbre des paramètres : rechercher `id_n_max_mt`
3. Double-clic → la table s'ouvre (9 colonnes, 1 ligne)
4. Modifier directement les valeurs RPM dans les cellules
5. **Toujours modifier id_n_max_h_mt en cohérence** — la limite haute ne peut pas être inférieure à la limite de base

> **Règle :** `id_n_max_h_mt[i]` ≥ `id_n_max_mt[i]` pour tout rapport i.

### Modifier ip_n_max_2__tco (hard cut f(TCO))

1. Rechercher `ip_n_max_2__tco` → courbe 4 points
2. Axe X = TCO (°C) : −30 / 0 / 30 / 90
3. Modifier les valeurs RPM (axe Z)
4. **Toujours modifier ip_n_max_1__tco simultanément** en gardant un écart de 200 RPM entre les deux courbes

---

## 📐 Exemples de modifications

### Scénario 1 — Protection renforcée moteur à kilométrage élevé

Abaisser le cut à 6500 RPM sur tous les rapports, limiter à 6000 RPM à froid (< 60°C) :

**id_n_max_mt** (proposé) :
```
Index :   0      1      2      3      4      5      6      7 (R)  8 (N/A)
RPM   : 6200   6500   6500   6500   6500   6200   6500   6000   6000
```

**ip_n_max_2__tco** (proposé) :
```
TCO  : −30°C    0°C   30°C   90°C
RPM  :  4800   5200   5600   6500
```
**ip_n_max_1__tco** (proposé — toujours 200 RPM en dessous) :
```
TCO  : −30°C    0°C   30°C   90°C
RPM  :  4600   5000   5400   6300
```

---

### Scénario 2 — Usage piste, cut franc à 6800 RPM

Garder le même plafond que stock mais uniformiser sur tous les rapports :

**id_n_max_mt** (proposé) :
```
Index :   0      1      2      3      4      5      6      7 (R)  8 (N/A)
RPM   : 6800   6800   6800   6800   6800   6800   6800   6400   6400
```

---

### Scénario 3 — Moteur préparé, débrider à 7200 RPM

> ⚠️ **Uniquement si le moteur est préparé (arbre à cames, bielle forgée).** Le N52B30 stock n'est pas conçu pour dépasser 7200 RPM en usage prolongé.

**id_n_max_mt** (proposé) :
```
Index :   0      1      2      3      4      5      6      7 (R)  8 (N/A)
RPM   : 7000   7200   7200   7200   7200   7000   7200   6800   6800
```

---

## ⚠️ Points de vigilance

| Risque | Précaution |
|---|---|
| `id_n_max_h_*` < `id_n_max_*` | Interdit — le contrôleur peut se comporter de façon imprévisible |
| Cut trop bas en 1ère/marche arrière | Rester à ≥ 5000 RPM pour conserver la maniabilité à basse vitesse |
| Dépasser 7200 RPM stock N52B30 | Risque de casse bielle/pistons sur moteur non préparé |
| ip_n_max_1 ≥ ip_n_max_2 | Toujours 200 RPM d'écart — sinon la zone orange chevauche le hard cut |
| Oublier de corriger le checksum | Flash refusé au démarrage — utiliser TunerPro "Checksum → Fix" avant flash |

---

## 🔗 Liens

- [TUTO E85.md](TUTO%20E85.md) — Conversion éthanol complète
- [TUTO VMAX.md](TUTO%20VMAX.md) — Modifier la vitesse maximale
- [README.md](README.md) — Documentation générale MSV70
