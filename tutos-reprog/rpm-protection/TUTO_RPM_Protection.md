# RPM Protection — Siemens MSV70 / N52B30

> **Véhicule :** BMW E90/E91/E92/E93 — N52B30 — Siemens MSV70  
> **Bin de référence :** VB67774_921S_Full.bin — SW 9PPL921S

---

## Deux systèmes indépendants

Le MSV70 gère la limitation de régime via deux mécanismes distincts :

| Système | Paramètres | Quand actif |
|---|---|---|
| **Limite permanente f(rapport)** | `id_n_max_mt/at`, `id_n_max_h_mt/at` | En permanence, quelle que soit la température |
| **Limite de chauffe f(TCO)** | `ip_n_max_1__tco`, `ip_n_max_2__tco` | Uniquement pendant la montée en température |

Les deux s'appliquent **simultanément** — c'est la valeur la plus restrictive des deux qui s'applique à chaque instant.

---

## Groupe 1 — Limite permanente f(rapport de boîte)

### id_n_max_mt / id_n_max_at

Tables de 9 points définissant le régime maximum pour chaque position de boîte.

**Index Siemens → rapport :**

| Index | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
|---|---|---|---|---|---|---|---|---|---|
| Signification | Neutre | 1ère | 2ème | 3ème | 4ème | 5ème | 6ème | Marche AR | N/A |

**id_n_max_mt — stock 9PPL921S :**
```
Index :  0     1     2     3     4     5     6     7(R)  8
RPM   : 6800  6980  6980  6980  6980  6800  6980  6400  6400
```
> BMW bride volontairement le 5ème rapport (index 5) à 6800 RPM.

**id_n_max_at — stock :**
```
Index :  0     1     2     3     4     5     6     7(R)  8
RPM   : 6400  7000  7000  7000  7000  6800  7000  6400  6400
```

### id_n_max_h_mt / id_n_max_h_at

Limite "haute" — fenêtre temporaire activée à haute vitesse (`c_vs_thd_n_max_h_mt` = 254 km/h). En dessous de 254 km/h, c'est `id_n_max_mt` qui s'applique.

**id_n_max_h_mt — stock :**
```
RPM : 6802  6982  6982  6982  6982  6982  6982  6600  6600
```

**Règle impérative :** `id_n_max_h_mt[i]` ≥ `id_n_max_mt[i]` pour tout rapport. Une limite haute inférieure à la limite de base crée un comportement indéfini.

---

## Groupe 2 — Limite de chauffe f(TCO)

### ip_n_max_1__tco (avertissement) / ip_n_max_2__tco (hard cut)

Courbes 4 points f(TCO) limitant le régime pendant la montée en température. `ip_n_max_1` déclenche le voyant d'avertissement, `ip_n_max_2` coupe l'injection. L'écart entre les deux doit rester à **200 RPM**.

**Stock 9PPL921S :**

| TCO | ip_n_max_1 (voyant) | ip_n_max_2 (cut) |
|-----|---------------------|------------------|
| −30°C | 5200 RPM | 5400 RPM |
| 0°C | 5600 RPM | 5800 RPM |
| 30°C | 6000 RPM | 6200 RPM |
| 90°C | 6820 RPM | 7020 RPM |

À 90°C, la limite f(TCO) est plus haute que la limite f(rapport) — elle n'est donc plus la contrainte active une fois le moteur chaud.

---

## Procédure TunerPro RT

### Modifier id_n_max_mt

1. Charger XDF + BIN
2. Rechercher `id_n_max_mt` → table 9 colonnes, 1 ligne
3. Modifier les valeurs RPM
4. **Répercuter sur `id_n_max_h_mt`** — la limite haute doit toujours être ≥ à la limite de base

### Modifier ip_n_max_2__tco

1. Rechercher `ip_n_max_2__tco` → courbe 4 points, axe X = TCO
2. Modifier les RPM
3. **Répercuter sur `ip_n_max_1__tco`** en gardant exactement 200 RPM d'écart

---

## Scénarios

### Scénario 1 — Protection renforcée (moteur kilométré ou après révision)

Cut à 6500 RPM sur tous les rapports, plus restrictif à froid :

```
id_n_max_mt :  6200  6500  6500  6500  6500  6200  6500  6000  6000
id_n_max_h_mt: 6200  6500  6500  6500  6500  6500  6500  6200  6200

ip_n_max_2__tco : −30°C→4800 / 0°C→5200 / 30°C→5600 / 90°C→6500
ip_n_max_1__tco : −30°C→4600 / 0°C→5000 / 30°C→5400 / 90°C→6300
```

### Scénario 2 — Uniformisation stock (5ème rapport débridé)

Aligner tous les rapports à 6980 RPM comme les autres :

```
id_n_max_mt :  6800  6980  6980  6980  6980  6980  6980  6400  6400
```

### Scénario 3 — Moteur préparé, extension à 7200 RPM

> N52B30 stock non conçu pour dépasser 7200 RPM en usage prolongé. Réserver aux moteurs préparés (cames, bielles forgées).

```
id_n_max_mt :  7000  7200  7200  7200  7200  7000  7200  6800  6800
id_n_max_h_mt: 7000  7200  7200  7200  7200  7200  7200  7000  7000
```

---

## Points de vigilance

| Risque | Précaution |
|---|---|
| `id_n_max_h_*` < `id_n_max_*` | Interdit — comportement calculateur indéfini |
| Cut < 5000 RPM en 1ère/marche arrière | Perte de maniabilité à basse vitesse |
| Dépasser 7200 RPM sur N52B30 stock | Risque mécanique (bielle, pistons) |
| Écart `ip_n_max_1` / `ip_n_max_2` ≠ 200 RPM | Zone orange et hard cut se chevauchent |
| Oublier le checksum | Flash refusé — `XDF → Checksum → Fix` avant flash |

---

## Liens

- [TUTO E85 — Sommaire](../E85/00_SOMMAIRE.md)
- [TUTO VMAX](../vmax/TUTO_VMAX.md)
- [Warm-up — Limite RPM à froid](../warmup/05_RALENTI.md)
