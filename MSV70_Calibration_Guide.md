# MSV70 — Guide de Calibration : DTC Sonde Aval & Avance à l'Allumage

> BIN de référence : `VB67774_921S_Full.bin` (2560 Ko, complet)
> XDF : fichier projet TunerPro actif

---

## 1. Désactiver le DTC Sonde Lambda Aval (Post-cat O2)

### Contexte

Le MSV70 supervise la sonde lambda aval (Hinterkat) via trois catégories de diagnostic :
- `EGCP_O2_sensor_downstream_diag` — diagnostic principal de présence/plausibilité
- `EGCP_O2_sensor_probe_diag_plaus` — contrôle de plausibilité du signal
- `EGCP_Downstream_ls_read_det` — détection de lecture / seuils thermiques

### Tables à modifier

| Paramètre | Type | Adresse | Catégorie | Rôle |
|---|---|---|---|---|
| `K_ETA_LAMBDABAS` | CONST (1 val) | `0x5D554` | Seuil efficacité catalyseur OBD | Seuil rendement catalyseur — **déjà à 100 % (désactivé)** |
| `ip_fac_maf_obd_lsh_down` | 1×6 | `0x483BA` | `EGCP_O2_sensor_downstream_diag` | Fenêtre MAF de validation du diagnostic aval |
| `ip_r_it_thd_obd_lsh_down` | 1×4 | `0x48530` | `EGCP_O2_sensor_downstream_diag` | Seuil résistance interne sonde (Ohm) |
| `ip_fac_cycnr_max_ls_down_act` | 1×8 | `0x483B2` | `EGCP_O2_sensor_probe_diag_plaus` | Compteur cycles max validation (ms) |
| `ip_t_temp_thd_lsh_down` | 1×6 | `0x48564` | `EGCP_Downstream_ls_read_det` | Seuils température activation lecture |

### Stratégie recommandée — sans toucher au catalyseur OBD

`K_ETA_LAMBDABAS` est déjà à **100,00 %** dans ton BIN — le seuil rendement catalyseur est déjà au maximum, ce DTC-là est déjà pratiquement désactivé côté efficacité.

Pour supprimer définitivement le DTC sonde aval **sans flasher une valeur invalide** :

#### Option A — Repousser la fenêtre MAF hors plage réelle (`ip_fac_maf_obd_lsh_down`)

Valeurs actuelles (axe MAF en kg/h : 25 / 35 / 50 / 65 / 90 / 100) :
```
[0.1875, 0.3125, 0.8125, 6.0625, 15.125, 15.9375]
```

**Action :** mettre toutes les valeurs à `0` — la fenêtre de déclenchement du diagnostic devient nulle à tous les points MAF. Le diagnostic ne peut plus valider son compteur, donc jamais de DTC.

| MAF (kg/h) | Valeur actuelle | → Valeur cible |
|---|---|---|
| 25 | 0.1875 | **0** |
| 35 | 0.3125 | **0** |
| 50 | 0.8125 | **0** |
| 65 | 6.0625 | **0** |
| 90 | 15.125 | **0** |
| 100 | 15.9375 | **0** |

#### Option B — Monter le seuil résistance interne à l'infini (`ip_r_it_thd_obd_lsh_down`)

Seuil résistance interne sonde (4 valeurs, adresse `0x48530`).
**Action :** mettre toutes les valeurs à la valeur maximale encodable — la sonde ne satisfera jamais le critère de « sonde chaude et active », le diagnostic ne se valide pas.

#### Option C — Monter les seuils température (`ip_t_temp_thd_lsh_down`)

Seuil température de lecture (6 valeurs, adresse `0x48564`).
**Action :** mettre toutes les valeurs à une température impossible à atteindre (ex. 300 °C si la plage le permet) — la condition thermique d'activation du diagnostic n'est jamais satisfaite.

### Recommandation finale

> **Applique l'Option A (`ip_fac_maf_obd_lsh_down` → tout à 0).** C'est le moyen le plus propre et le plus ciblé : seule la validation du compteur de cycles est bloquée, aucune valeur de monitoring live n'est altérée. Le reste du système lambda aval continue de fonctionner normalement pour le contrôle moteur.

---

## 2. Avance à l'Allumage — Guide Complet des Tables

### Architecture allumage MSV70

```
ip_iga_min_n_maf_ext          ← avance de base étendue (cartographie principale alternative)
ip_iga_min_n_maf_opm_1        ← avance de base OPM1 (mode normal)
ip_iga_min_n_maf_opm_2        ← avance de base OPM2 (mode dégradé / secours)
ip_iga_min_n_maf_puc_opm_1    ← avance base préchauffage catalyseur OPM1
ip_iga_min_n_maf_puc_opm_2    ← avance base préchauffage catalyseur OPM2

ip_iga_bas_max_knk__n__maf    ← plafond knock (avance maximale autorisée avant knock)
ip_iga_ofs_max_knk__n__maf    ← offset de retard knock (retrait appliqué en cas de cliquetis)

ip_iga_ref_mon                ← référence monitoring (lecture seule)
ip_iga_ref_mon2               ← référence monitoring 2 (lecture seule)

ip_iga_dif_min_cyc_amp_at/mt  ← correction min. par amplitude de cliquetis (AT/MT)
ip_iga_dif_min_delta          ← correction min. delta allumage
ip_iga_dif_min_tco            ← correction min. par température caloporteur
ip_iga_dif_min_teg__teg       ← correction min. par température gaz échappement
```

### Tables de base — détail

#### `ip_iga_min_n_maf_opm_1` — Avance de base, Mode Normal (carte principale)

- **Adresse** : `0x4347A`
- **Dimensions** : 8×6 (RPM × charge)
- **Unité** : °CRK (degrés vilebrequin)
- **Axes RPM** : 320 / 960 / 1600 / 2400 / 3520 / 4800 / 5760 / 6528 tr/min
- **Axes charge (mg/stk)** : 0.36 / 0.61 / 0.87 / 1.12 / 1.63 / 2.14

**Plage actuelle** : −20.25° à +24°

C'est **la table principale** à modifier pour retravailler l'avance de base.

---

#### `ip_iga_min_n_maf_opm_2` — Avance de base, Mode Dégradé

- **Adresse** : `0x434AA`
- **Dimensions** : 8×6
- **Unité** : °CRK
- **Plage actuelle** : −20.25° à +19.875°

Mode secours (capteur MAF défaillant ou autre défaut). Modifier en parallèle avec OPM1 si tu changes la stratégie globale.

---

#### `ip_iga_min_n_maf_ext` — Avance de Base Étendue

- **Adresse** : `0x4344A`
- **Dimensions** : 8×6
- **Unité** : °CRK

Alternative cartographique (mode extended). Souvent proche d'OPM1 mais peut couvrir une plage de charge différente. Modifier si l'ECU bascule dessus (conditions particulières de charge élevée ou mode spécifique).

---

#### `ip_iga_min_n_maf_puc_opm_1` — Avance Préchauffage Catalyseur OPM1

- **Adresse** : `0x434DA`
- **Dimensions** : 8×6
- **Unité** : °CRK

Utilisée pendant la phase de montée en température catalyseur après démarrage froid. Typiquement retardée volontairement pour chauffer le cat plus vite. **Ne pas avancer ces valeurs** — cela ralentirait le chauffage cat et peut générer des DTC.

---

#### `ip_iga_min_n_maf_puc_opm_2` — Avance Préchauffage Catalyseur OPM2

- **Adresse** : `0x4350A`
- **Dimensions** : 8×6
- **Unité** : °CRK

Même rôle que PUC OPM1 en mode dégradé. À laisser tel quel.

---

### Tables de limitation knock

#### `ip_iga_bas_max_knk__n__maf` — Plafond Knock (avance max autorisée)

- **Adresse** : `0x4323A`
- **Dimensions** : 8×8 (RPM × charge)
- **Unité** : °CRK
- **Axes RPM** : 608 / 1024 / 1600 / 2400 / 3520 / 4800 / 5760 / 7008 tr/min
- **Axes charge (mg/stk)** : 0.55 / 0.80 / 1.00 / 1.25 / 1.50 / 1.75 / 2.00 / 2.25

**Plage actuelle** : −15° à +43.5°

Cette table définit le **plafond d'avance que l'ECU ne dépassera jamais**, même si la table de base (OPM1) demande plus. C'est la limite haute du système knock.

> **Si tu augmentes OPM1 mais que le knock ceiling est inférieur → l'ECU plafonne à la valeur du ceiling.** Il faut relever les deux en cohérence.

---

#### `ip_iga_ofs_max_knk__n__maf` — Offset de Retard Knock

- **Adresse** : `0x4353A`
- **Dimensions** : 8×8
- **Unité** : °CRK (valeurs négatives = retard)
- **Plage actuelle** : 0° à −11.25°

Valeur de retard maximale que l'ECU peut appliquer en cas de cliquetis détecté. L'ECU retire cette avance depuis la valeur courante jusqu'à résolution du cliquetis.

> En usage route normal : **ne pas modifier**. En piste / carburant E85 custom : peut être réduit (ex. −5° au lieu de −11.25°) si le carburant garantit l'absence de knock.

---

### Tables de monitoring (lecture seule)

#### `ip_iga_ref_mon` et `ip_iga_ref_mon2`

- **Adresses** : `0x493AB` / `0x493EB`
- **Dimensions** : 8×8

Référence interne pour le monitoring OBD de l'allumage. Ces tables sont comparées à l'avance réelle pour détecter des dérives. **Ne pas modifier** — cela fausserait le monitoring sans gain de performance.

---

### Tables de correction minimale (correctifs de sécurité)

| Paramètre | Rôle | Action recommandée |
|---|---|---|
| `ip_iga_dif_min_cyc_amp_at` | Correction min. par amplitude cliquetis (boîte auto) | Ne pas toucher |
| `ip_iga_dif_min_cyc_amp_mt` | Correction min. par amplitude cliquetis (boîte manuelle) | Ne pas toucher |
| `ip_iga_dif_min_delta` | Correction min. delta allumage cycle à cycle | Ne pas toucher |
| `ip_iga_dif_min_tco` | Correction min. par température caloporteur | Ne pas toucher (gère le retard à froid) |
| `ip_iga_dif_min_teg__teg` | Correction min. par température gaz échappement | Ne pas toucher (protection thermique) |

---

### Workflow de modification — Avance à l'Allumage

#### Étape 1 — Identifier la zone à modifier

Utilise le msv70-analyzer (onglet STFT Map > knock retard heatmap) pour voir quelles cellules (RPM, charge) présentent du retard knock. Cible uniquement ces cellules.

#### Étape 2 — Vérifier la cohérence OPM1 ↔ Ceiling

Pour chaque cellule (RPM_i, Load_j) que tu veux modifier :

```
Avance effective = MIN(OPM1[i,j], ip_iga_bas_max_knk[i,j])
```

Si tu veux +2° dans OPM1 mais que le ceiling est déjà au même niveau → relève d'abord le ceiling.

#### Étape 3 — Modifier dans TunerPro

1. Ouvrir `ip_iga_min_n_maf_opm_1` — modifier les cellules cibles
2. Vérifier `ip_iga_bas_max_knk__n__maf` — s'assurer que le plafond ≥ nouvelle valeur OPM1
3. Si mode dégradé courant : appliquer les mêmes corrections dans OPM2

#### Étape 4 — Validation sur route

- Logger avec le msv70-analyzer
- Vérifier l'onglet **Timeseries** : surveiller `knock_status` et `ign_deg`
- Vérifier l'onglet **STFT Map > Knock Retard** : aucune cellule ne doit passer au rouge après modification

---

### Tableau récapitulatif — Priorité de modification

| Table | Modifiable | Quand | Impact |
|---|---|---|---|
| `ip_iga_min_n_maf_opm_1` | ✅ Oui | Recalibration avance base | Avance effective mode normal |
| `ip_iga_bas_max_knk__n__maf` | ✅ Oui (avec précaution) | Si OPM1 > ceiling actuel | Plafond knock — protège le moteur |
| `ip_iga_ofs_max_knk__n__maf` | ⚠️ Avec précaution | Carburant premium garanti | Profondeur de retard knock |
| `ip_iga_min_n_maf_opm_2` | ✅ Oui | En parallèle d'OPM1 | Mode dégradé |
| `ip_iga_min_n_maf_ext` | ✅ Si nécessaire | Mode étendu actif | Charge élevée mode ext. |
| Tables PUC | ❌ Non | — | Chauffage catalyseur |
| Tables `ref_mon` | ❌ Non | — | Monitoring OBD uniquement |
| Tables `dif_min_*` | ❌ Non | — | Sécurité système knock |

---

*Document généré le 2026-05-20 — BIN VB67774_921S_Full.bin*
