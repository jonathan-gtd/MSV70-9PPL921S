# §6 — Délai enrichissement WOT : c_t_ti_dly_fl_1 et c_t_ti_dly_fl_2

### 📋 Tables à modifier

| Paramètre | Adresse | Équation | Rôle |
|---|---|---|---|
| `c_t_ti_dly_fl_1` | 0x44EC4 | 0.010 × X (s) | Délai avant enrichissement WOT |
| `c_t_ti_dly_fl_2` | 0x44EC6 | 0.010 × X (s) | Idem |

### 🔨 Procédure

```
TunerPro → c_t_ti_dly_fl_1 et _2 → raw 0 → 0.000 s
```

<a id="pencil-dly"></a>

### ✏️ Avant / Après

| Paramètre | ◀ Raw stock | ◀ Valeur stock | ▶ Raw E85 | ▶ Valeur E85 |
|---|---|---|---|---|
| `c_t_ti_dly_fl_1` | 20 | 0.200 s | **0** | **0.000 s** |
| `c_t_ti_dly_fl_2` | 20 | 0.200 s | **0** | **0.000 s** |

### ✅ Vérification

Enrichissement WOT immédiat à pleine charge (aucun délai perceptible).

---
