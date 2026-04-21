# §9 — Avance cranking : c_iga_ini (optionnel)

### 📋 Tables à modifier

| Paramètre | Adresse | Équation | Modification |
|---|---|---|---|
| `c_iga_ini` | 0x44B2A | 0.375 × X − 35.625 °CRK | inchangé stock → **+1° à +2°** uniquement si démarrage > 5 tours |

### 🔨 Procédure

À tester seulement si le démarrage reste difficile malgré calibration §2 (ip_mff_cst_opm_*).

<a id="pencil-iga"></a>

### ✏️ Avant / Après

| Paramètre | ◀ Raw stock | ◀ Valeur stock | ▶ Raw E85 | ▶ Valeur E85 |
|---|---|---|---|---|
| `c_iga_ini` | 111 | 6.0 °CRK | **111** | **6.0 °CRK (inchangé)** |

> Modifier uniquement si démarrage > 5 tours malgré §5 (ip_mff_cst_opm_*) correctement calibré. Cible : raw 114 → **+6.75 °CRK** ou raw 116 → **+7.88 °CRK** (+1° à +2° progressivement).

### ✅ Vérification

Démarrage < 3 tours. Intervenir uniquement si > 5 tours malgré §5 correctement calibré.

---
