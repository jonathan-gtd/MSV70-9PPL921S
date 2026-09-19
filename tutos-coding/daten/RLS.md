# RLS — Capteur pluie / luminosite

🟢 Module `RLS` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟢 Module sans fonction de securite. Sauvegarde du `.TRC` quand meme.

---

## Identification

| | |
|---|---|
| Variante | **`RLS_E65.C06`** |
| Identifiée par | **lecture NCS Expert sur le véhicule** |
| Fonctions retenues | **12** |
| Vérification | 100 % des fonctions dans les blocs du `.TRC` |
| Avec valeurs | 12 |
| Signification (EN) | 12 |
| Explication (FR) | 12 |
| Blocs d'adresses du TRC | 2 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 12 fonctions codables

| FSW | Décomposition | Valeurs | Signification (EN) | Explication (FR) |
|---|---|---|---|---|
| `DEF_FLC_SCHWELLWERT_SATZ` | defaut · _FLC_ · valeur de seuil · jeu | `wert_01` · `wert_02` · `wert_03` · `wert_04` | Automatic light control (FLC) default sensitivity set [wert_01: high sensitivity, wert_02: normal sensitivity, wert_03: very high sensitivity, wert_04: high sensitivity] | Sensibilite par defaut de l'allumage automatique (FLC) [wert_01 : elevee, wert_02 : normale, wert_03 : tres elevee, wert_04 : elevee] |
| `EMPF_OHNE_RAENDELRAD` | sensibilite · sans · molette | `empfindlichkeitsstufe_1` · `empfindlichkeitsstufe_2` · `empfindlichkeitsstufe_3` · `empfindlichkeitsstufe_4` | Sensitivity without wiper stalk thumbwheel | Sensibilite sans molette au comodo d'essuie-glace |
| `FLC_SCHWELLWERT_SATZ` | _FLC_ · valeur de seuil · jeu | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `l4` | Automatic light control (FLC) sensitivity set [wert_01: high sensitivity, wert_02: normal sensitivity, wert_03: very high sensitivity, wert_04: high sensitivity] | Sensibilite de l'allumage automatique (FLC) [wert_01 : elevee, wert_02 : normale, wert_03 : tres elevee, wert_04 : elevee] |
| `KONFIG_NACHL_REGEN` | configuration · temporisation · pluie | `wert_01` | Light sensor delay when raining [time_min=data] | Temporisation du capteur de luminosite sous la pluie [duree min = valeur] |
| `KONFIG_RLS_GESCHW` | configuration · capteur pluie/lumiere · vitesse | `aktiv` · `nicht_aktiv` | Switch on headlights at high speeds | Allumage des feux a vitesse elevee |
| `KONFIG_RLS_RAENDELRAD` | configuration · capteur pluie/lumiere · molette | `aktiv` · `nicht_aktiv` | Wiper stalk thumbwheel for adjusting rain sensor sensitiy | Molette du comodo pour regler la sensibilite du capteur de pluie |
| `KONFIG_RLS_REGEN` | configuration · capteur pluie/lumiere · pluie | `aktiv` · `nicht_aktiv` | Switch on headlights when raining | Allumage des feux sous la pluie |
| `TYP_FRONTSCHEIBE` | type · pare-brise | `serienscheibe` · `klimakomfort` | Windscreen type | Type de pare-brise |
| `TYP_RLS_OPTIK` | type · capteur pluie/lumiere · optique | `aktiv` · `nicht_aktiv` | Clear/blue optical element type | Type d'element optique clair ou bleute |
| `V_HIGH_SPEED_WISCHEN` | _V_ · haut · vitesse · essuyage | `wert_01` · `l2` · `wert_02` · `e70_e70m` · `wert_03` | Speed threshold for high-speed wiping | Seuil de vitesse du balayage rapide |
| `WISCHEN_STUFE2` | essuyage · niveau 2 | `aktiv` · `nicht_aktiv` | 2-stage windscreen wipers | Essuie-glaces a deux vitesses |
| `WISCHERMOTOR_RSK` | moteur d'essuie-glace · capteur (?) | `aktiv` · `nicht_aktiv` | Windscreen wipers reset contact [wert_01: ground when not parked, wert_02: ground when parked, aktiv: with reset contact, nicht_aktiv: without reset contact] | Contact de repos des essuie-glaces [wert_01 : masse hors position repos, wert_02 : masse en position repos, aktiv : avec contact de repos, nicht_aktiv : sans] |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Signification (EN)** — description issue de la base communautaire NCS Dummy, verbatim.

**Explication (FR)** — traduction française. Un tiret signifie que la ligne n'est pas
encore traduite. Les notations d'unités sont converties : `[durée s = valeur / 10]`.
