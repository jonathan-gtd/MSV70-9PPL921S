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

### ✏️ Avant / Après

| Paramètre | ◀ Raw stock | ◀ Valeur stock | ▶ Raw E85 | ▶ Valeur E85 |
|---|---|---|---|---|
| `c_t_ti_dly_fl_1` (MT) | 20 | 0.200 s | **0** | **0.000 s** |
| `c_t_ti_dly_fl_2` (MT) | 20 | 0.200 s | **0** | **0.000 s** |
| `c_t_ti_dly_fl_at_1` (AT) | 20 | 0.200 s | **0** | **0.000 s** |
| `c_t_ti_dly_fl_at_2` (AT) | 20 | 0.200 s | **0** | **0.000 s** |

### ✅ Vérification

Enrichissement WOT immédiat à pleine charge (aucun délai perceptible).

---
