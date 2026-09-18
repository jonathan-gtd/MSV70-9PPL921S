# EKP — Pompe a carburant

🔴 Module `EKP` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🔴 **Module a ne pas coder.** Cette liste est fournie pour comprendre, pas pour modifier.

---

## Source

| | |
|---|---|
| Variante retenue | `EKPM60_2.C03` |
| FSW dans cette variante | **28** |
| FSW toutes variantes confondues | 28 |
| Fichiers DATEN analyses | 4 |

Variantes disponibles : `EKPM60_2.C03`, `EKPM60_3.C11`, `EKPM60_3.C12`, `EKPM60_3.C13`

> La variante reellement installee sur le vehicule est identifiee par NCS Dummy
> a l'ouverture du `.TRC`. Si elle differe de celle retenue ici, la liste change.

---

## Les 28 fonctions codables

| FSW | Traduction |
|---|---|
| `AMS_R1_SRON` | _AMS_ · **R1** · **SRON** |
| `AMS_R2_SRON_STAT` | _AMS_ · **R2** · **SRON** · etat |
| `AMS_R3_R5_SRON_MIN` | _AMS_ · **R3** · **R5** · **SRON** · minimum |
| `AMS_R4_SROFF` | _AMS_ · **R4** · **SROFF** |
| `AMS_R6_R8_ACU_ON` | _AMS_ · **R6** · **R8** · **ACU** · marche |
| `AMS_R7_AV_TRIMM` | _AMS_ · **R7** · **AV** · **TRIMM** |
| `AMS_R9_R10_OVERC` | _AMS_ · **R9** · **R10** · **OVERC** |
| `DATUM_MONAT` | date · **MONAT** |
| `DATUM_TAG` | date · **TAG** |
| `DEBUG_BYTE1` | deverminage · **BYTE1** |
| `DEBUG_BYTE2` | deverminage · **BYTE2** |
| `DEBUG_BYTE3` | deverminage · **BYTE3** |
| `KEINE_SPANNUNG_KL30` | aucun · tension · borne 30 (permanent) |
| `LIMIT_M_DREHZAHL` | limite · M · regime |
| `MIN_SPANNUNG_KL30` | minimum · tension · borne 30 (permanent) |
| `MOTORPOLE` | **MOTORPOLE** |
| `PUMPEN_ID` | **PUMPEN** · identifiant |
| `RIPPLEFILTER_LP_1` | **RIPPLEFILTER** · **LP** · 1 |
| `RIPPLEFILTER_LP_2` | **RIPPLEFILTER** · **LP** · 2 |
| `RIPPLEFILTER_NO` | **RIPPLEFILTER** · non |
| `SPARE_1` | reserve · 1 |
| `SPARE_2` | reserve · 2 |
| `SPARE_3` | reserve · 3 |
| `SPARE_4` | reserve · 4 |
| `SPARE_5` | reserve · 5 |
| `SPARE_6` | reserve · 6 |
| `UEBERTEMP_LEVEL1` | **UEBERTEMP** · **LEVEL1** |
| `UEBERTEMP_LEVEL2` | **UEBERTEMP** · **LEVEL2** |

---

Les fragments en **gras** n'ont pas de traduction etablie — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments entre `_underscores_` sont ambigus : leur sens depend du module.
