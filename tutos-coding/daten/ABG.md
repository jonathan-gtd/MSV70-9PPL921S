# ABG — Airbags

🔴 Module `ABG` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🔴 **Module a ne pas coder.** Liste fournie pour comprendre, pas pour modifier.

---

## Identification

| | |
|---|---|
| Variante | **`MRS5CD.C22`** |
| Identifiée par | recoupement avec `ABG.TRC` lu sur le véhicule |
| Fonctions retenues | **67** |
| Écartées (hors blocs du TRC) | 0 |
| Avec valeurs | 67 |
| Avec explication rédigée | 5 |
| Blocs d'adresses du TRC | 14 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 67 fonctions codables

| FSW | Traduction | Valeurs | Explication |
|---|---|---|---|
| `AACN_1` | appel d'urgence automatique · 1 | `nicht_aktiv` · `aktiv` |  |
| `ADAPTIVER_BEIFAHRERAIRBAG` | adaptatif · airbag passager | `nicht_aktiv` · `aktiv` |  |
| `AGKL_1` | classe de capteur (?) · 1 | `nicht_aktiv` · `aktiv` |  |
| `AGKR_1` | capteur (?) · 1 | `nicht_aktiv` · `aktiv` |  |
| `AIRBAG_BEIFAHRER_1` | airbag · passager · 1 | `nicht_aktiv` · `aktiv` |  |
| `AIRBAG_FAHRER_1` | airbag · conducteur · 1 | `nicht_aktiv` · `aktiv` |  |
| `AKS_LINKS_1_CD` | systeme d'alarme (?) · gauche · 1 · CD | `nicht_aktiv` · `aktiv` |  |
| `AKS_RECHTS_1_CD` | systeme d'alarme (?) · droite · 1 · CD | `nicht_aktiv` · `aktiv` |  |
| `AUFROLLSTRAMMER_LINKS_1` | enrouleur pretensionneur · gauche · 1 | `nicht_aktiv` · `aktiv` |  |
| `AUFROLLSTRAMMER_RECHTS_1` | enrouleur pretensionneur · droite · 1 | `nicht_aktiv` · `aktiv` |  |
| `AUFROLL_SEITENCRASH` | enroulement · choc lateral | `nicht_aktiv` · `aktiv` · `e90_ece` · `tel_mute_nav_mix` · `e91_ece` · `e90_us` · `e91_us` · `e92_ece` … *(+9)* |  |
| `BAUREIHE_MRS5CD` | serie · module airbag | `e88` · `e81` · `e82` · `e82_bev` · `e84` · `e87` · `e89` · `e90` … *(+3)* |  |
| `BEIFAHRERBAG_STUFE2_2` | airbag passager · niveau 2 · 2 | `nicht_aktiv` · `aktiv` |  |
| `BPLUS_TRENNUNG1_1` | borne plus · separation 1 · 1 | `nicht_aktiv` · `aktiv` |  |
| `CC_GURTWARNUNG_1` | Check Control · alerte de ceinture · 1 | `nicht_aktiv` · `aktiv` |  |
| `CURTAIN_BEIFAHRER_1` | airbag rideau · passager · 1 | `nicht_aktiv` · `aktiv` |  |
| `CURTAIN_FAHRER_1` | airbag rideau · conducteur · 1 | `nicht_aktiv` · `aktiv` |  |
| `FAHRERBAG_STUFE2_1` | airbag conducteur · niveau 2 · 1 | `nicht_aktiv` · `aktiv` |  |
| `GURTKONTAKT_BEIFAHRER_1` | contact de ceinture · passager · 1 | `nicht_aktiv` · `aktiv` |  |
| `GURTKONTAKT_FAHRER_1` | contact de ceinture · conducteur · 1 | `nicht_aktiv` · `aktiv` |  |
| `GURTSTRAFFER_BEIFAHRER_1` | pretensionneur de ceinture · passager · 1 | `nicht_aktiv` · `aktiv` |  |
| `GURTSTRAFFER_FAHRER_1` | pretensionneur de ceinture · conducteur · 1 | `nicht_aktiv` · `aktiv` |  |
| `GURTSTRAFFER_FONDLINKS_1` | pretensionneur de ceinture · arriere gauche · 1 | `nicht_aktiv` · `aktiv` |  |
| `GURTSTRAFFER_FONDRECHTS_1` | pretensionneur de ceinture · arriere droit · 1 | `nicht_aktiv` · `aktiv` |  |
| `GURTSTRAMM_SEITENCRASH_1` | tension de ceinture · choc lateral · 1 | `nicht_aktiv` · `aktiv` |  |
| `GURT_NICHT_AB_GK` | ceinture · non · a partir de · grandeur (?) | `nicht_aktiv` · `aktiv` |  |
| `ITS_GEGENUEBER_1` | airbag de tete (ITS) · par rapport a · 1 | `nicht_aktiv` · `aktiv` |  |
| `ITS_SCHRAEGCRASH_1` | airbag de tete (ITS) · choc oblique · 1 | `nicht_aktiv` · `aktiv` |  |
| `KNIEAIRBAG_BEIFAHRER_1` | airbag genoux · passager · 1 | `nicht_aktiv` · `aktiv` |  |
| `KNIEAIRBAG_FAHRER_1` | airbag genoux · conducteur · 1 | `nicht_aktiv` · `aktiv` |  |
| `KNIEBAG_BEIFAHRER_1` | airbag genoux · passager · 1 | `nicht_aktiv` · `aktiv` |  |
| `KNIEBAG_FAHRER_1` | airbag genoux · conducteur · 1 | `nicht_aktiv` · `aktiv` |  |
| `LANGER_CURTAIN_1` | long · airbag rideau · 1 | `curtain` · `its` |  |
| `LENKSEITE_1` | cote conduite · 1 | `links` · `rechts` |  |
| `MRSA_TUERE_1_C22` | systeme de retenue (?) · porte · 1 · [variante C22] | `nicht_aktiv` · `aktiv` |  |
| `OC3_1` | detection d'occupation (?) · 1 | `nicht_aktiv` · `aktiv` |  |
| `O_GW_BF_1_MCD` | O · passerelle · passager · 1 · diagnostic (?) | `nicht_aktiv` · `aktiv` |  |
| `O_GW_F_1` | O · passerelle · F · 1 | `nicht_aktiv` · `aktiv` |  |
| `POL_1` | polarite · 1 | `nicht_aktiv` · `aktiv` |  |
| `POL_MAX_SCHWELLE` | polarite · maximum · seuil | `wert_01` · `wert_02` |  |
| `POL_MIN_SCHWELLE` | polarite · minimum · seuil | `wert_01` · `wert_02` |  |
| `POS_1` | position · 1 | `nicht_aktiv` · `aktiv` |  |
| `ROLLOVER_OPTION_1` | retournement · option · 1 | `nicht_aktiv` · `aktiv` |  |
| `SBE1_1_CD` | detection d'occupation du siege 1 · 1 · CD | `nicht_aktiv` · `aktiv` |  |
| `SBE1_STATUS_VERZ_MCD` | detection d'occupation du siege 1 · etat · temporisation · diagnostic (?) | `nicht_aktiv` · `aktiv` |  |
| `SBE_SCHWELLE_ON` | detection d'occupation du siege · seuil · marche | `wert_01` |  |
| `SBE_STEP_DOWN` | detection d'occupation du siege · pas · bas | `wert_01` |  |
| `SBE_STEP_UP` | detection d'occupation du siege · pas · haut | `wert_01` |  |
| `SBR_BEIFAHRER_2_CD` | rappel de ceinture · passager · 2 · CD | `nicht_aktiv` · `aktiv` | Rappel de ceinture. |
| `SBR_FAHRER_1` | rappel de ceinture · conducteur · 1 | `nicht_aktiv` · `aktiv` | Rappel de ceinture conducteur (gong et temoin). |
| `SBR_PREWARNING_BF_MRS` | rappel de ceinture · pre-alerte · passager · systeme de retenue | `nicht_aktiv` · `aktiv` | Rappel de ceinture. |
| `SBR_PREWARNING_FA_MRS` | rappel de ceinture · pre-alerte · conducteur · systeme de retenue | `nicht_aktiv` · `aktiv` | Rappel de ceinture. |
| `SBR_SENSOR_BEIFAHRER_MCD` | rappel de ceinture · capteur · passager · diagnostic (?) | `nicht_aktiv` · `aktiv` | Rappel de ceinture. |
| `SEAT_BELT_DISTANCE` | siege · ceinture · distance | `wert_01` |  |
| `SEAT_BELT_MONITOR` | siege · ceinture · surveillance | `wert_01` |  |
| `SEAT_BELT_SPEED` | siege · ceinture · vitesse | `wert_01` |  |
| `SIDEBAG_LINKS_1` | airbag lateral · gauche · 1 | `nicht_aktiv` · `aktiv` |  |
| `SIDEBAG_RECHTS_1` | airbag lateral · droite · 1 | `nicht_aktiv` · `aktiv` |  |
| `SLV_BEIFAHRER_1` | blocage de colonne · passager · 1 | `nicht_aktiv` · `aktiv` |  |
| `SLV_FAHRER_1` | blocage de colonne · conducteur · 1 | `nicht_aktiv` · `aktiv` |  |
| `SPSBF_1` | siege passager (?) · 1 | `nicht_aktiv` · `aktiv` |  |
| `SPSF_1_CD` | siege conducteur (?) · 1 · CD | `nicht_aktiv` · `aktiv` |  |
| `TCU_UEBERWACHUNG_4_22` | unite telephone · surveillance · 4 · 22 | `nicht_aktiv` · `aktiv` |  |
| `TROMMELSPEICHER` | memoire tampon | `nicht_aktiv` · `aktiv` |  |
| `UPFRONT_LINKS_1` | airbag frontal · gauche · 1 | `nicht_aktiv` · `aktiv` |  |
| `UPFRONT_RECHTS_1` | airbag frontal · droite · 1 | `nicht_aktiv` · `aktiv` |  |
| `US_SCHWELLEN_1` | marche americain · seuils · 1 | `nicht_aktiv` · `aktiv` |  |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Explication** — rédigée seulement quand l'effet est établi de source sûre.
