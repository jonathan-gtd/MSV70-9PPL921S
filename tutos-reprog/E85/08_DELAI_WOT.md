# §6 — Délai enrichissement WOT : c_t_ti_dly_fl_1 et c_t_ti_dly_fl_2

### 📋 Tables à modifier

| Paramètre | Adresse | Équation | Rôle | Boîte |
|---|---|---|---|---|
| `c_t_ti_dly_fl_1` | 0x44EC4 | 0.010 × X (s) | Délai avant enrichissement WOT | MT |
| `c_t_ti_dly_fl_2` | 0x44EC6 | 0.010 × X (s) | Idem | MT |
| `c_t_ti_dly_fl_at_1` | 0x44EC8 | 0.010 × X (s) | Délai avant enrichissement WOT | AT |
| `c_t_ti_dly_fl_at_2` | 0x44ECA | 0.010 × X (s) | Idem | AT |

### 🔨 Procédure

```
TunerPro → modifier les 2 copies MT (ou AT selon boîte) → raw 0 → 0.000 s
```

<a id="pencil-dly"></a>

---

### ① `c_t_ti_dly_fl_1` — Délai WOT boîte manuelle, copie 1 @ 0x44EC4

**Équation :** `0.010 × raw` (secondes)  
**Rôle :** Délai entre la détection de l'état pleine charge (full load flag) et l'activation effective de l'enrichissement WOT sur le temps d'injection, pour les boîtes manuelles. Le MSV70 possède deux copies MT — toutes les deux doivent être mises à zéro.

| | ◀ Raw stock | ◀ Valeur stock | ▶ Raw E85 | ▶ Valeur E85 |
|---|---|---|---|---|
| `c_t_ti_dly_fl_1` | 20 | **0.200 s (200 ms)** | **0** | **0.000 s** |

> Sur E85, 200 ms de délai = bref lean transitoire à chaque pleine charge = risque de détonation non détectée.

---

### ② `c_t_ti_dly_fl_2` — Délai WOT boîte manuelle, copie 2 @ 0x44EC6

**Équation :** `0.010 × raw` (secondes)  
**Rôle :** Deuxième copie du délai WOT MT. L'ECU utilise ces deux copies dans des contextes d'exécution différents — si l'une reste à 20, l'enrichissement peut encore être retardé selon la situation moteur.

| | ◀ Raw stock | ◀ Valeur stock | ▶ Raw E85 | ▶ Valeur E85 |
|---|---|---|---|---|
| `c_t_ti_dly_fl_2` | 20 | **0.200 s (200 ms)** | **0** | **0.000 s** |

> Modifier en même temps que c_t_ti_dly_fl_1.

---

### ③ `c_t_ti_dly_fl_at_1` — Délai WOT boîte automatique, copie 1 @ 0x44EC8

**Équation :** `0.010 × raw` (secondes)  
**Rôle :** Même logique que la version MT, mais pour les boîtes automatiques. Modifier uniquement si le véhicule est équipé d'une ZF 6HP.

| | ◀ Raw stock | ◀ Valeur stock | ▶ Raw E85 | ▶ Valeur E85 |
|---|---|---|---|---|
| `c_t_ti_dly_fl_at_1` | 20 | **0.200 s (200 ms)** | **0** | **0.000 s** |

> Modifier uniquement sur véhicule AT. Sur MT, laisser stock (l'ECU n'utilise pas ces copies).

---

### ④ `c_t_ti_dly_fl_at_2` — Délai WOT boîte automatique, copie 2 @ 0x44ECA

**Équation :** `0.010 × raw` (secondes)  
**Rôle :** Deuxième copie du délai WOT AT. Même logique que _at_1 — modifier les deux ensemble.

| | ◀ Raw stock | ◀ Valeur stock | ▶ Raw E85 | ▶ Valeur E85 |
|---|---|---|---|---|
| `c_t_ti_dly_fl_at_2` | 20 | **0.200 s (200 ms)** | **0** | **0.000 s** |

> Modifier uniquement sur véhicule AT.

---

### ✅ Vérification

Enrichissement WOT immédiat à pleine charge (aucun délai perceptible).

---
