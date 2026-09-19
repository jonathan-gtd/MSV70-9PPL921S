# RLS — Capteur pluie / luminosite

🟢 Module `RLS` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟢 Module sans fonction de securite. Sauvegarde du `.TRC` quand meme.

---

## Identification

| | |
|---|---|
| Variante | **`RLS_PL2.C08`** |
| Identifiée par | recoupement avec `RLS.TRC` lu sur le véhicule |
| Fonctions retenues | **12** |
| Écartées (hors blocs du TRC) | 0 |
| Avec valeurs | 12 |
| Avec explication rédigée | 0 |
| Blocs d'adresses du TRC | 4 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 12 fonctions codables

| FSW | Traduction | Valeurs | Explication |
|---|---|---|---|
| `DEF_FLC_SCHWELLWERT_SATZ` | defaut · _FLC_ · valeur de seuil · jeu | `wert_01` · `wert_02` · `wert_03` |  |
| `EMPF_OHNE_RAENDELRAD` | **EMPF** · sans · molette | `empfindlichkeitsstufe_1` · `empfindlichkeitsstufe_2` · `empfindlichkeitsstufe_3` · `empfindlichkeitsstufe_4` |  |
| `FLC_SCHWELLWERT_SATZ` | _FLC_ · valeur de seuil · jeu | `wert_01` · `wert_02` · `wert_03` |  |
| `KONFIG_NACHL_REGEN` | configuration · temporisation · pluie | `wert_01` |  |
| `KONFIG_RLS_GESCHW` | configuration · capteur pluie/lumiere · vitesse | `aktiv` · `nicht_aktiv` |  |
| `KONFIG_RLS_RAENDELRAD` | configuration · capteur pluie/lumiere · molette | `aktiv` · `nicht_aktiv` |  |
| `KONFIG_RLS_REGEN` | configuration · capteur pluie/lumiere · pluie | `aktiv` · `nicht_aktiv` |  |
| `TYP_FRONTSCHEIBE` | type · **FRONTSCHEIBE** | `serienscheibe` · `klimakomfort` |  |
| `UEBERWISCHZEIT_1` | duree de balayage supplementaire · 1 | `wert_01` · `wert_02` |  |
| `UEBERWISCHZEIT_2` | duree de balayage supplementaire · 2 | `wert_01` · `wert_02` |  |
| `WISCHEN_STUFE2` | essuyage · niveau 2 | `aktiv` · `nicht_aktiv` |  |
| `WISCHERMOTOR_RSK` | **WISCHERMOTOR** · **RSK** | `aktiv` · `nicht_aktiv` |  |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Explication** — rédigée seulement quand l'effet est établi de source sûre.
