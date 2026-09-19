# ABG — Airbags

🔴 Module `ABG` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🔴 **Module a ne pas coder.** Liste fournie pour comprendre, pas pour modifier.

---

## Source

| | |
|---|---|
| Variante retenue | `MRS5.C06` |
| Fonctions codables | **51** |
| Avec valeurs extraites | 51 (100 %) |
| Avec explication rédigée | 2 |
| Fichiers DATEN analysés | 8 |

Variantes disponibles : `MRS5.C05`, `MRS5.C06`, `MRS5.C07`, `MRS5.C08`, `MRS5CD.C01`, `MRS5CD.C20`, `MRS5CD.C21`, `MRS5CD.C22`

> La variante réellement installée est identifiée par NCS Dummy à l'ouverture du `.TRC`.
> Si elle diffère de celle retenue ici, la liste change.

---

## Les 51 fonctions codables

| FSW | Traduction | Valeurs possibles | Explication |
|---|---|---|---|
| `AIRBAG_BEIFAHRER_1` | airbag · passager · 1 | `nicht_aktiv` · `aktiv` |  |
| `AIRBAG_FAHRER_1` | airbag · conducteur · 1 | `nicht_aktiv` · `aktiv` |  |
| `AWL_1` | **AWL** · 1 | `nicht_aktiv` · `aktiv` |  |
| `BAUREIHE` | **BAUREIHE** | `e81` · `e82` · `e84` · `e87` · `e88` · `e89` · `e90` · `e91` · `e92` · `e93` |  |
| `BEIFAHRERBAG_STUFE2_1` | **BEIFAHRERBAG** · niveau 2 · 1 | `nicht_aktiv` · `aktiv` |  |
| `BPLUS_TRENNUNG1_1` | **BPLUS** · **TRENNUNG1** · 1 | `nicht_aktiv` · `aktiv` |  |
| `CC_GURTWARNUNG_1` | Check Control · **GURTWARNUNG** · 1 | `nicht_aktiv` · `aktiv` |  |
| `CURTAIN_BEIFAHRER_1` | **CURTAIN** · passager · 1 | `nicht_aktiv` · `aktiv` |  |
| `CURTAIN_FAHRER_1` | **CURTAIN** · conducteur · 1 | `nicht_aktiv` · `aktiv` |  |
| `FAHRERBAG_STUFE2_1` | **FAHRERBAG** · niveau 2 · 1 | `nicht_aktiv` · `aktiv` |  |
| `GURTKONTAKT_BEIFAHRER_1` | contact de ceinture · passager · 1 | `nicht_aktiv` · `aktiv` |  |
| `GURTKONTAKT_FAHRER_1` | contact de ceinture · conducteur · 1 | `nicht_aktiv` · `aktiv` |  |
| `GURTKONTAKT_FONDLINKS_1` | contact de ceinture · **FONDLINKS** · 1 | `nicht_aktiv` · `aktiv` |  |
| `GURTKONTAKT_FONDRECHTS_1` | contact de ceinture · **FONDRECHTS** · 1 | `nicht_aktiv` · `aktiv` |  |
| `GURTSTRAFFER_BEIFAHRER_1` | **GURTSTRAFFER** · passager · 1 | `nicht_aktiv` · `aktiv` |  |
| `GURTSTRAFFER_FAHRER_1` | **GURTSTRAFFER** · conducteur · 1 | `nicht_aktiv` · `aktiv` |  |
| `GURTSTRAFFER_FONDLINKS_1` | **GURTSTRAFFER** · **FONDLINKS** · 1 | `nicht_aktiv` · `aktiv` |  |
| `GURTSTRAFFER_FONDRECHTS_1` | **GURTSTRAFFER** · **FONDRECHTS** · 1 | `nicht_aktiv` · `aktiv` |  |
| `GURTSTRAMM_SEITENCRASH_1` | **GURTSTRAMM** · **SEITENCRASH** · 1 | `nicht_aktiv` · `aktiv` |  |
| `GURT_NICHT_AB_GK` | ceinture · non · **AB** · **GK** | `nicht_aktiv` · `aktiv` · `e87` · `e90_vorserie` · `tel_mute_nav_mix` · `e91_ppp` · `e92_ppp` · `tfl_s` |  |
| `ITS_GEGENUEBER_1` | **ITS** · **GEGENUEBER** · 1 | `nicht_aktiv` · `aktiv` |  |
| `ITS_SCHRAEGCRASH_1` | **ITS** · **SCHRAEGCRASH** · 1 | `nicht_aktiv` · `aktiv` |  |
| `KNIEBAG_BEIFAHRER_1` | **KNIEBAG** · passager · 1 | `nicht_aktiv` · `aktiv` |  |
| `KNIEBAG_FAHRER_1` | **KNIEBAG** · conducteur · 1 | `nicht_aktiv` · `aktiv` |  |
| `LANGER_CURTAIN_1` | **LANGER** · **CURTAIN** · 1 | `curtain` · `its` |  |
| `LENKSEITE_1` | **LENKSEITE** · 1 | `links` · `rechts` |  |
| `MRSA_TUERE_1` | **MRSA** · **TUERE** · 1 | `nicht_aktiv` · `aktiv` |  |
| `OC3_1` | **OC3** · 1 | `nicht_aktiv` · `aktiv` |  |
| `POL_1` | **POL** · 1 | `nicht_aktiv` · `aktiv` |  |
| `POSI_1` | **POSI** · 1 | `nicht_aktiv` · `aktiv` |  |
| `POS_1` | position · 1 | `nicht_aktiv` · `aktiv` |  |
| `ROLLOVER_OPTION_1` | **ROLLOVER** · option · 1 | `nicht_aktiv` · `aktiv` |  |
| `SBE1_1` | **SBE1** · 1 | `nicht_aktiv` · `aktiv` |  |
| `SBE1_STATUS_VERZ_1` | **SBE1** · etat · temporisation · 1 | `nicht_aktiv` · `aktiv` |  |
| `SBE_SCHWELLE_ON` | **SBE** · seuil · marche | `wert_01` |  |
| `SBE_STEP_DOWN` | **SBE** · **STEP** · bas | `wert_01` |  |
| `SBE_STEP_UP` | **SBE** · **STEP** · haut | `wert_01` |  |
| `SBR_BEIFAHRER_1` | rappel de ceinture · passager · 1 | `nicht_aktiv` · `aktiv` | Rappel de ceinture passager (gong et temoin). |
| `SBR_FAHRER_1` | rappel de ceinture · conducteur · 1 | `nicht_aktiv` · `aktiv` | Rappel de ceinture conducteur (gong et temoin). |
| `SEAT_BELT_DISTANCE` | **SEAT** · **BELT** · **DISTANCE** | `wert_01` |  |
| `SEAT_BELT_MONITOR` | **SEAT** · **BELT** · surveillance | `wert_01` |  |
| `SEAT_BELT_SPEED` | **SEAT** · **BELT** · vitesse | `wert_01` |  |
| `SIDEBAG_FOND_LINKS_1` | airbag lateral · places arriere · gauche · 1 | `nicht_aktiv` · `aktiv` |  |
| `SIDEBAG_FOND_RECHTS_1` | airbag lateral · places arriere · droite · 1 | `nicht_aktiv` · `aktiv` |  |
| `SIDEBAG_LINKS_1` | airbag lateral · gauche · 1 | `nicht_aktiv` · `aktiv` |  |
| `SIDEBAG_RECHTS_1` | airbag lateral · droite · 1 | `nicht_aktiv` · `aktiv` |  |
| `SLV_BEIFAHRER_1` | **SLV** · passager · 1 | `nicht_aktiv` · `aktiv` |  |
| `SLV_FAHRER_1` | **SLV** · conducteur · 1 | `nicht_aktiv` · `aktiv` |  |
| `UPFRONT_LINKS_1` | **UPFRONT** · gauche · 1 | `nicht_aktiv` · `aktiv` |  |
| `UPFRONT_RECHTS_1` | **UPFRONT** · droite · 1 | `nicht_aktiv` · `aktiv` |  |
| `US_SCHWELLEN_1` | marche americain · **SCHWELLEN** · 1 | `nicht_aktiv` · `aktiv` |  |

---

**Traduction** : décomposition du nom allemand — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments en **gras** n'ont pas de traduction établie, ceux entre `_underscores_` sont ambigus.

**Valeurs possibles** : extraites des DATEN. `aktiv`/`nicht_aktiv` = interrupteur,
`wert_xx` = valeur numérique, autres = choix nommés.

**Explication** : rédigée uniquement pour les fonctions identifiées avec certitude.
Une case vide ne veut pas dire que la fonction est sans intérêt — seulement que son effet
n'a pas été établi de source sûre.
