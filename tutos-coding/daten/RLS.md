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
| Avec explication rédigée | 12 |
| Blocs d'adresses du TRC | 2 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 12 fonctions codables

| FSW | Décomposition | Valeurs | Signification |
|---|---|---|---|
| `DEF_FLC_SCHWELLWERT_SATZ` | defaut · _FLC_ · valeur de seuil · jeu | `wert_01` · `wert_02` · `wert_03` · `wert_04` | *Automatic light control (FLC) default sensitivity set [wert_01: high sensitivity, wert_02: normal sensitivity, wert_03: very high sensitivity, wert_04: high sensitivity]* |
| `EMPF_OHNE_RAENDELRAD` | sensibilite · sans · molette | `empfindlichkeitsstufe_1` · `empfindlichkeitsstufe_2` · `empfindlichkeitsstufe_3` · `empfindlichkeitsstufe_4` | *Sensitivity without wiper stalk thumbwheel* |
| `FLC_SCHWELLWERT_SATZ` | _FLC_ · valeur de seuil · jeu | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `l4` | *Automatic light control (FLC) sensitivity set [wert_01: high sensitivity, wert_02: normal sensitivity, wert_03: very high sensitivity, wert_04: high sensitivity]* |
| `KONFIG_NACHL_REGEN` | configuration · temporisation · pluie | `wert_01` | *Light sensor delay when raining [duree min = valeur]* |
| `KONFIG_RLS_GESCHW` | configuration · capteur pluie/lumiere · vitesse | `aktiv` · `nicht_aktiv` | *Switch on headlights at high speeds* |
| `KONFIG_RLS_RAENDELRAD` | configuration · capteur pluie/lumiere · molette | `aktiv` · `nicht_aktiv` | *Wiper stalk thumbwheel for adjusting rain sensor sensitiy* |
| `KONFIG_RLS_REGEN` | configuration · capteur pluie/lumiere · pluie | `aktiv` · `nicht_aktiv` | *Switch on headlights when raining* |
| `TYP_FRONTSCHEIBE` | type · pare-brise | `serienscheibe` · `klimakomfort` | *Windscreen type* |
| `TYP_RLS_OPTIK` | type · capteur pluie/lumiere · optique | `aktiv` · `nicht_aktiv` | *Clear/blue optical element type* |
| `V_HIGH_SPEED_WISCHEN` | _V_ · haut · vitesse · essuyage | `wert_01` · `l2` · `wert_02` · `e70_e70m` · `wert_03` | *Speed threshold for high-speed wiping* |
| `WISCHEN_STUFE2` | essuyage · niveau 2 | `aktiv` · `nicht_aktiv` | *2-stage windscreen wipers* |
| `WISCHERMOTOR_RSK` | moteur d'essuie-glace · capteur (?) | `aktiv` · `nicht_aktiv` | *Windscreen wipers reset contact [wert_01: ground when not parked, wert_02: ground when parked, aktiv: with reset contact, nicht_aktiv: without reset contact]* |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Signification** — texte en clair. En romain : rédigé à la main. *En italique* : description
de la base communautaire NCS Dummy, en anglais, non encore traduite.
Les notations d'unités (`[durée s = valeur / 10]`) sont traduites.
