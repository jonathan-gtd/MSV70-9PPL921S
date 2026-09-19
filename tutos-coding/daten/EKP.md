# EKP — Pompe a carburant

🔴 Module `EKP` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🔴 **Module a ne pas coder.** Liste fournie pour comprendre, pas pour modifier.

---

## Identification

| | |
|---|---|
| Variante | **`EKPM60_2.C03`** |
| Identifiée par | **lecture NCS Expert sur le véhicule** |
| Fonctions retenues | **28** |
| Vérification | 100 % des fonctions dans les blocs du `.TRC` |
| Avec valeurs | 28 |
| Avec explication rédigée | 17 |
| Blocs d'adresses du TRC | 3 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 28 fonctions codables

| FSW | Décomposition | Valeurs | Signification |
|---|---|---|---|
| `AMS_R1_SRON` | _AMS_ · parametre R1 · sirene activee (?) | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_99` … *(+26)* |  |
| `AMS_R2_SRON_STAT` | _AMS_ · parametre R2 (?) · sirene activee (?) · etat | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_99` … *(+26)* |  |
| `AMS_R3_R5_SRON_MIN` | _AMS_ · parametre R3 · parametre R5 · sirene activee (?) · minimum | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_99` … *(+26)* |  |
| `AMS_R4_SROFF` | _AMS_ · 4 cylindres en ligne · sirene desactivee (?) | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_99` … *(+26)* |  |
| `AMS_R6_R8_ACU_ON` | _AMS_ · 6 cylindres en ligne · 8 cylindres (?) · unite de commande (?) · marche | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_99` … *(+26)* |  |
| `AMS_R7_AV_TRIMM` | _AMS_ · parametre R7 · audio-video (?) · ajustement | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_99` … *(+26)* |  |
| `AMS_R9_R10_OVERC` | _AMS_ · parametre R9 · parametre R10 · surintensite | `wert_01` · `wert_02` · `wert_06` · `wert_07` · `wert_04` · `wert_05` · `wert_99` · `wert_03` … *(+26)* |  |
| `DATUM_MONAT` | date · mois | `wert_01` · `wert_02` · `wert_05` · `wert_07` · `wert_23` · `wert_14` · `wert_99` · `wert_103` … *(+26)* | *Date month* |
| `DATUM_TAG` | date · jour | `wert_07` · `wert_08` · `wert_23` · `wert_02` · `wert_05` · `wert_103` · `wert_14` · `wert_107` … *(+26)* | *Date day* |
| `DEBUG_BYTE1` | deverminage · [BYTE1] | `wert_03` · `wert_99` · `wert_06` · `wert_09` · `wert_11` · `wert_12` · `wert_13` · `wert_14` … *(+26)* | *Debug byte 1* |
| `DEBUG_BYTE2` | deverminage · [BYTE2] | `wert_99` · `wert_08` · `wert_26` · `wert_106` · `wert_01` · `wert_02` · `wert_05` · `wert_07` … *(+26)* | *Debug byte 2* |
| `DEBUG_BYTE3` | deverminage · [BYTE3] | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_07` · `wert_99` · `wert_06` … *(+26)* | *Debug byte 3* |
| `KEINE_SPANNUNG_KL30` | aucun · tension · borne 30 (permanent) | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_99` … *(+26)* | *No battery supply voltage threshold [voltage_V=(valeur*30)/1023]* |
| `LIMIT_M_DREHZAHL` | limite · M · regime | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_99` … *(+26)* | *Limit fuel pump rotation speed* |
| `MIN_SPANNUNG_KL30` | minimum · tension · borne 30 (permanent) | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_99` … *(+26)* | *Minimum voltage from battery supply [voltage_V=(valeur*30)/1023]* |
| `MOTORPOLE` | nombre de poles | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_99` … *(+26)* |  |
| `PUMPEN_ID` | pompes · identifiant | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_99` · `wert_101` … *(+26)* | *Pump identification* |
| `RIPPLEFILTER_LP_1` | filtre d'ondulation · haut-parleur (?) · 1 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_99` · `wert_103` … *(+26)* |  |
| `RIPPLEFILTER_LP_2` | filtre d'ondulation · haut-parleur (?) · 2 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_99` · `wert_103` · `wert_104` … *(+26)* |  |
| `RIPPLEFILTER_NO` | filtre d'ondulation · non | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_99` … *(+26)* |  |
| `SPARE_1` | reserve · 1 | `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_99` · `wert_101` … *(+26)* | *Reserved 1* |
| `SPARE_2` | reserve · 2 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_99` … *(+26)* | *Reserved 2* |
| `SPARE_3` | reserve · 3 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_99` … *(+26)* | *Reserved 3* |
| `SPARE_4` | reserve · 4 | `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_99` · `wert_101` … *(+26)* | *Reserved 4* |
| `SPARE_5` | reserve · 5 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_99` … *(+26)* | *Reserved 5* |
| `SPARE_6` | reserve · 6 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_99` … *(+26)* | *Reserved 6* |
| `UEBERTEMP_LEVEL1` | surtemperature · niveau 1 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_99` … *(+26)* | *Overtemperature level 1* |
| `UEBERTEMP_LEVEL2` | surtemperature · niveau 2 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_99` … *(+26)* | *Overtemperature level 2* |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Signification** — texte en clair. En romain : rédigé à la main. *En italique* : description
de la base communautaire NCS Dummy, en anglais, non encore traduite.
Les notations d'unités (`[durée s = valeur / 10]`) sont traduites.
