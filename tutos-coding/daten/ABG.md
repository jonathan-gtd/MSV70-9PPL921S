# ABG — Airbags

🔴 Module `ABG` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🔴 **Module a ne pas coder.** Liste fournie pour comprendre, pas pour modifier.

---

## Identification

| | |
|---|---|
| Variante | **`MRS5.C08`** |
| Identifiée par | **lecture NCS Expert sur le véhicule** |
| Fonctions retenues | **53** |
| Vérification | 100 % des fonctions dans les blocs du `.TRC` |
| Avec valeurs | 53 |
| Signification (EN) | 43 |
| Explication (FR) | 43 |
| Blocs d'adresses du TRC | 14 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 53 fonctions codables

| FSW | Décomposition | Valeurs | Signification (EN) | Explication (FR) |
|---|---|---|---|---|
| `AIRBAG_BEIFAHRER_1` | airbag · passager · 1 | `nicht_aktiv` · `aktiv` | Passenger airbag 1 | Airbag passager 1 |
| `AIRBAG_FAHRER_1` | airbag · conducteur · 1 | `nicht_aktiv` · `aktiv` | Driver airbag 1 | Airbag conducteur 1 |
| `AWL_1` | temoin d'alerte (?) · 1 | `nicht_aktiv` · `aktiv` | — | — |
| `BAUREIHE` | serie | `e88` · `e81` · `e82` · `e84` · `e87` · `e89` · `e90` · `e91` · `e92` · `e93` | Chassis | Serie du vehicule |
| `BEIFAHRERBAG_STUFE2_1` | airbag passager · niveau 2 · 1 | `nicht_aktiv` · `aktiv` | Passenger airbag stage 2 1 | Airbag passager niveau 2, 1 |
| `BPLUS_TRENNUNG1_1` | borne plus · separation 1 · 1 | `nicht_aktiv` · `aktiv` | Battery cut-off 1 1 | Coupure de batterie 1, 1 |
| `CC_GURTWARNUNG_1` | Check Control · alerte de ceinture · 1 | `nicht_aktiv` · `aktiv` | Check control notification for seat belt warning 1 | Message Check Control pour l'alerte de ceinture 1 |
| `CURTAIN_BEIFAHRER_1` | airbag rideau · passager · 1 | `nicht_aktiv` · `aktiv` | — | — |
| `CURTAIN_FAHRER_1` | airbag rideau · conducteur · 1 | `nicht_aktiv` · `aktiv` | — | — |
| `FAHRERBAG_STUFE2_1` | airbag conducteur · niveau 2 · 1 | `nicht_aktiv` · `aktiv` | Driver airbag stage 2 1 | Airbag conducteur niveau 2, 1 |
| `GURTKONTAKT_BEIFAHRER_1` | contact de ceinture · passager · 1 | `nicht_aktiv` · `aktiv` | Passenger seat belt contact 1 | Contact de ceinture passager 1 |
| `GURTKONTAKT_FAHRER_1` | contact de ceinture · conducteur · 1 | `nicht_aktiv` · `aktiv` | Driver seat belt contact 1 | Contact de ceinture conducteur 1 |
| `GURTKONTAKT_FONDLINKS_1` | contact de ceinture · arriere gauche · 1 | `nicht_aktiv` · `aktiv` | Rear left seat belt contact 1 | Contact de ceinture arriere gauche 1 |
| `GURTKONTAKT_FONDRECHTS_1` | contact de ceinture · arriere droit · 1 | `nicht_aktiv` · `aktiv` | Right rear seat belt contact 1 | Contact de ceinture arriere droit 1 |
| `GURTSTRAFFER_BEIFAHRER_1` | pretensionneur de ceinture · passager · 1 | `nicht_aktiv` · `aktiv` | Passenger seat belt tensioner 1 | Pretensionneur de ceinture passager 1 |
| `GURTSTRAFFER_FAHRER_1` | pretensionneur de ceinture · conducteur · 1 | `nicht_aktiv` · `aktiv` | Driver seat belt tensioner 1 | Pretensionneur de ceinture conducteur 1 |
| `GURTSTRAFFER_FONDLINKS_1` | pretensionneur de ceinture · arriere gauche · 1 | `nicht_aktiv` · `aktiv` | Rear left seat belt tensioner 1 | Pretensionneur de ceinture arriere gauche 1 |
| `GURTSTRAFFER_FONDRECHTS_1` | pretensionneur de ceinture · arriere droit · 1 | `nicht_aktiv` · `aktiv` | Right rear seat belt tensioner 1 | Pretensionneur de ceinture arriere droit 1 |
| `GURTSTRAMM_SEITENCRASH_1` | tension de ceinture · choc lateral · 1 | `nicht_aktiv` · `aktiv` | Deploy seat belt tensioner in side-impact crash 1 | Declenchement du pretensionneur en choc lateral 1 |
| `GURT_NICHT_AB_GK` | ceinture · non · a partir de · grandeur (?) | `nicht_aktiv` · `aktiv` · `e92_ece` · `e92_us` · `e87` · `e82` · `e81` · `e91_us` … *(+5)* | — | — |
| `ITS_GEGENUEBER_1` | airbag de tete (ITS) · par rapport a · 1 | `nicht_aktiv` · `aktiv` | — | — |
| `ITS_SCHRAEGCRASH_1` | airbag de tete (ITS) · choc oblique · 1 | `nicht_aktiv` · `aktiv` | Deploy head airbag in offset-impact crash 1 | Declenchement de l'airbag de tete en choc decale 1 |
| `KNIEBAG_BEIFAHRER_1` | airbag genoux · passager · 1 | `nicht_aktiv` · `aktiv` | Passenger knee airbag 1 | Airbag genoux passager 1 |
| `KNIEBAG_FAHRER_1` | airbag genoux · conducteur · 1 | `nicht_aktiv` · `aktiv` | Driver knee airbag 1 | Airbag genoux conducteur 1 |
| `LANGER_CURTAIN_1` | long · airbag rideau · 1 | `curtain` · `its` | — | — |
| `LENKSEITE_1` | cote conduite · 1 | `links` · `rechts` | Steering side 1 | Cote de conduite 1 |
| `MRSA_TUERE_1` | systeme de retenue (?) · porte · 1 | `nicht_aktiv` · `aktiv` | Door satellites 1 | Capteurs satellites de porte 1 |
| `OC3_1` | detection d'occupation (?) · 1 | `nicht_aktiv` · `aktiv` | Seat occupancy detection (OC3) 1 | Detection d'occupation du siege (OC3) 1 |
| `POL_1` | polarite · 1 | `nicht_aktiv` · `aktiv` | Passenger airbag off light 1 | Temoin d'airbag passager desactive 1 |
| `POL_MAX_SCHWELLE` | polarite · maximum · seuil | `wert_01` · `wert_02` | Maximum pulse-width modulation (PWM) for passenger airbag off light | Rapport cyclique maximal du temoin d'airbag passager desactive |
| `POL_MIN_SCHWELLE` | polarite · minimum · seuil | `wert_01` · `wert_02` | Minimum pulse-width modulation (PWM) for passenger airbag off light | Rapport cyclique minimal du temoin d'airbag passager desactive |
| `POSI_1` | position · 1 | `nicht_aktiv` · `aktiv` | — | — |
| `POS_1` | position · 1 | `nicht_aktiv` · `aktiv` | Passenger airbag off switch 1 | Interrupteur de desactivation de l'airbag passager 1 |
| `ROLLOVER_OPTION_1` | retournement · option · 1 | `nicht_aktiv` · `aktiv` | Roll-over protection option 1 | Option arceau de securite 1 |
| `SBE1_1` | detection d'occupation du siege 1 · 1 | `nicht_aktiv` · `aktiv` | Seat occupancy detection (SBE1) 1 | Detection d'occupation du siege (SBE1) 1 |
| `SBE1_STATUS_VERZ_1` | detection d'occupation du siege 1 · etat · temporisation · 1 | `nicht_aktiv` · `aktiv` | Delay seat occupancy detection (SBE1) status 1 | Temporisation de l'etat de detection d'occupation du siege (SBE1) 1 |
| `SBE_SCHWELLE_ON` | detection d'occupation du siege · seuil · marche | `wert_01` | — | — |
| `SBE_STEP_DOWN` | detection d'occupation du siege · pas · bas | `wert_01` | — | — |
| `SBE_STEP_UP` | detection d'occupation du siege · pas · haut | `wert_01` | — | — |
| `SBR_BEIFAHRER_1` | rappel de ceinture · passager · 1 | `nicht_aktiv` · `aktiv` | Passenger seat belt warning 1 | Rappel de ceinture passager (gong et temoin). |
| `SBR_FAHRER_1` | rappel de ceinture · conducteur · 1 | `nicht_aktiv` · `aktiv` | Driver seat belt warning 1 | Rappel de ceinture conducteur (gong et temoin). |
| `SEAT_BELT_DISTANCE` | siege · ceinture · distance | `wert_01` | Driving distance before seat belt warning [distance_m=data*10] | Distance parcourue avant l'alerte de ceinture [distance m = valeur x 10] |
| `SEAT_BELT_MONITOR` | siege · ceinture · surveillance | `wert_01` | Delay for new seat belt warning [time_s=data] | Delai avant une nouvelle alerte de ceinture [duree s = valeur] |
| `SEAT_BELT_SPEED` | siege · ceinture · vitesse | `wert_01` | Minimum speed for seat belt warning [speed_km/h=data] | Vitesse minimale de declenchement de l'alerte de ceinture [vitesse km/h = valeur] |
| `SIDEBAG_FOND_LINKS_1` | airbag lateral · places arriere · gauche · 1 | `nicht_aktiv` · `aktiv` | Rear left side airbag 1 | Airbag lateral arriere gauche 1 |
| `SIDEBAG_FOND_RECHTS_1` | airbag lateral · places arriere · droite · 1 | `nicht_aktiv` · `aktiv` | Right rear side airbag 1 | Airbag lateral arriere droit 1 |
| `SIDEBAG_LINKS_1` | airbag lateral · gauche · 1 | `nicht_aktiv` · `aktiv` | Left side airbag 1 | Airbag lateral gauche 1 |
| `SIDEBAG_RECHTS_1` | airbag lateral · droite · 1 | `nicht_aktiv` · `aktiv` | Right side airbag 1 | Airbag lateral droit 1 |
| `SLV_BEIFAHRER_1` | blocage de colonne · passager · 1 | `nicht_aktiv` · `aktiv` | Passenger flip-forward seat backrest lock 1 | Verrouillage du dossier rabattable passager 1 |
| `SLV_FAHRER_1` | blocage de colonne · conducteur · 1 | `nicht_aktiv` · `aktiv` | Driver flip-forward seat backrest lock 1 | Verrouillage du dossier rabattable conducteur 1 |
| `UPFRONT_LINKS_1` | airbag frontal · gauche · 1 | `nicht_aktiv` · `aktiv` | Left up-front crash sensor 1 | Capteur de choc avant gauche 1 |
| `UPFRONT_RECHTS_1` | airbag frontal · droite · 1 | `nicht_aktiv` · `aktiv` | Right up-front crash sensor 1 | Capteur de choc avant droit 1 |
| `US_SCHWELLEN_1` | marche americain · seuils · 1 | `nicht_aktiv` · `aktiv` | USA thresholds 1 | Seuils marche americain 1 |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Signification (EN)** — description issue de la base communautaire NCS Dummy, verbatim.

**Explication (FR)** — traduction française. Un tiret signifie que la ligne n'est pas
encore traduite. Les notations d'unités sont converties : `[durée s = valeur / 10]`.
