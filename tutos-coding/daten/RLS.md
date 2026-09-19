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
| Avec explication rédigée | 0 |
| Blocs d'adresses du TRC | 2 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 12 fonctions codables

| FSW | Traduction | Valeurs | Explication |
|---|---|---|---|
| `DEF_FLC_SCHWELLWERT_SATZ` | defaut · _FLC_ · valeur de seuil · jeu | `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `EMPF_OHNE_RAENDELRAD` | sensibilite · sans · molette | `empfindlichkeitsstufe_1` · `empfindlichkeitsstufe_2` · `empfindlichkeitsstufe_3` · `empfindlichkeitsstufe_4` |  |
| `FLC_SCHWELLWERT_SATZ` | _FLC_ · valeur de seuil · jeu | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `l4` |  |
| `KONFIG_NACHL_REGEN` | configuration · temporisation · pluie | `wert_01` |  |
| `KONFIG_RLS_GESCHW` | configuration · capteur pluie/lumiere · vitesse | `aktiv` · `nicht_aktiv` |  |
| `KONFIG_RLS_RAENDELRAD` | configuration · capteur pluie/lumiere · molette | `aktiv` · `nicht_aktiv` |  |
| `KONFIG_RLS_REGEN` | configuration · capteur pluie/lumiere · pluie | `aktiv` · `nicht_aktiv` |  |
| `TYP_FRONTSCHEIBE` | type · pare-brise | `serienscheibe` · `klimakomfort` |  |
| `TYP_RLS_OPTIK` | type · capteur pluie/lumiere · optique | `aktiv` · `nicht_aktiv` |  |
| `V_HIGH_SPEED_WISCHEN` | _V_ · haut · vitesse · essuyage | `wert_01` · `l2` · `wert_02` · `e70_e70m` · `wert_03` |  |
| `WISCHEN_STUFE2` | essuyage · niveau 2 | `aktiv` · `nicht_aktiv` |  |
| `WISCHERMOTOR_RSK` | moteur d'essuie-glace · capteur (?) | `aktiv` · `nicht_aktiv` |  |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Explication** — rédigée seulement quand l'effet est établi de source sûre.
