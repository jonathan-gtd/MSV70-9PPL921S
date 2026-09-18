# ABG — Airbags

🔴 Module `ABG` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🔴 **Module a ne pas coder.** Cette liste est fournie pour comprendre, pas pour modifier.

---

## Source

| | |
|---|---|
| Variante retenue | `MRS5.C06` |
| FSW dans cette variante | **51** |
| FSW toutes variantes confondues | 85 |
| Fichiers DATEN analyses | 8 |

Variantes disponibles : `MRS5.C05`, `MRS5.C06`, `MRS5.C07`, `MRS5.C08`, `MRS5CD.C01`, `MRS5CD.C20`, `MRS5CD.C21`, `MRS5CD.C22`

> La variante reellement installee sur le vehicule est identifiee par NCS Dummy
> a l'ouverture du `.TRC`. Si elle differe de celle retenue ici, la liste change.

---

## Les 51 fonctions codables

| FSW | Traduction |
|---|---|
| `AIRBAG_BEIFAHRER_1` | airbag · passager · 1 |
| `AIRBAG_FAHRER_1` | airbag · conducteur · 1 |
| `AWL_1` | **AWL** · 1 |
| `BAUREIHE` | **BAUREIHE** |
| `BEIFAHRERBAG_STUFE2_1` | **BEIFAHRERBAG** · niveau 2 · 1 |
| `BPLUS_TRENNUNG1_1` | **BPLUS** · **TRENNUNG1** · 1 |
| `CC_GURTWARNUNG_1` | Check Control · **GURTWARNUNG** · 1 |
| `CURTAIN_BEIFAHRER_1` | **CURTAIN** · passager · 1 |
| `CURTAIN_FAHRER_1` | **CURTAIN** · conducteur · 1 |
| `FAHRERBAG_STUFE2_1` | **FAHRERBAG** · niveau 2 · 1 |
| `GURTKONTAKT_BEIFAHRER_1` | contact de ceinture · passager · 1 |
| `GURTKONTAKT_FAHRER_1` | contact de ceinture · conducteur · 1 |
| `GURTKONTAKT_FONDLINKS_1` | contact de ceinture · **FONDLINKS** · 1 |
| `GURTKONTAKT_FONDRECHTS_1` | contact de ceinture · **FONDRECHTS** · 1 |
| `GURTSTRAFFER_BEIFAHRER_1` | **GURTSTRAFFER** · passager · 1 |
| `GURTSTRAFFER_FAHRER_1` | **GURTSTRAFFER** · conducteur · 1 |
| `GURTSTRAFFER_FONDLINKS_1` | **GURTSTRAFFER** · **FONDLINKS** · 1 |
| `GURTSTRAFFER_FONDRECHTS_1` | **GURTSTRAFFER** · **FONDRECHTS** · 1 |
| `GURTSTRAMM_SEITENCRASH_1` | **GURTSTRAMM** · **SEITENCRASH** · 1 |
| `GURT_NICHT_AB_GK` | ceinture · non · **AB** · **GK** |
| `ITS_GEGENUEBER_1` | **ITS** · **GEGENUEBER** · 1 |
| `ITS_SCHRAEGCRASH_1` | **ITS** · **SCHRAEGCRASH** · 1 |
| `KNIEBAG_BEIFAHRER_1` | **KNIEBAG** · passager · 1 |
| `KNIEBAG_FAHRER_1` | **KNIEBAG** · conducteur · 1 |
| `LANGER_CURTAIN_1` | **LANGER** · **CURTAIN** · 1 |
| `LENKSEITE_1` | **LENKSEITE** · 1 |
| `MRSA_TUERE_1` | **MRSA** · **TUERE** · 1 |
| `OC3_1` | **OC3** · 1 |
| `POL_1` | **POL** · 1 |
| `POSI_1` | **POSI** · 1 |
| `POS_1` | position · 1 |
| `ROLLOVER_OPTION_1` | **ROLLOVER** · option · 1 |
| `SBE1_1` | **SBE1** · 1 |
| `SBE1_STATUS_VERZ_1` | **SBE1** · etat · temporisation · 1 |
| `SBE_SCHWELLE_ON` | **SBE** · seuil · marche |
| `SBE_STEP_DOWN` | **SBE** · **STEP** · bas |
| `SBE_STEP_UP` | **SBE** · **STEP** · haut |
| `SBR_BEIFAHRER_1` | rappel de ceinture · passager · 1 |
| `SBR_FAHRER_1` | rappel de ceinture · conducteur · 1 |
| `SEAT_BELT_DISTANCE` | **SEAT** · **BELT** · **DISTANCE** |
| `SEAT_BELT_MONITOR` | **SEAT** · **BELT** · surveillance |
| `SEAT_BELT_SPEED` | **SEAT** · **BELT** · vitesse |
| `SIDEBAG_FOND_LINKS_1` | airbag lateral · places arriere · gauche · 1 |
| `SIDEBAG_FOND_RECHTS_1` | airbag lateral · places arriere · droite · 1 |
| `SIDEBAG_LINKS_1` | airbag lateral · gauche · 1 |
| `SIDEBAG_RECHTS_1` | airbag lateral · droite · 1 |
| `SLV_BEIFAHRER_1` | **SLV** · passager · 1 |
| `SLV_FAHRER_1` | **SLV** · conducteur · 1 |
| `UPFRONT_LINKS_1` | **UPFRONT** · gauche · 1 |
| `UPFRONT_RECHTS_1` | **UPFRONT** · droite · 1 |
| `US_SCHWELLEN_1` | marche americain · **SCHWELLEN** · 1 |

---

Les fragments en **gras** n'ont pas de traduction etablie — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments entre `_underscores_` sont ambigus : leur sens depend du module.
