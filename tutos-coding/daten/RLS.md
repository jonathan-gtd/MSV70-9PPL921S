# RLS — Capteur pluie / luminosite

🟢 Module `RLS` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟢 Module sans fonction de securite. Sauvegarde du `.TRC` quand meme.

---

## Source

| | |
|---|---|
| Variante retenue | `RLS_PL2.C07` |
| Fonctions codables | **10** |
| Avec valeurs extraites | 10 (100 %) |
| Avec explication rédigée | 0 |
| Fichiers DATEN analysés | 3 |

Variantes disponibles : `RLS_PL2.C07`, `RLS_PL2.C08`, `RLS_E65.C06`

> La variante réellement installée est identifiée par NCS Dummy à l'ouverture du `.TRC`.
> Si elle diffère de celle retenue ici, la liste change.

---

## Les 10 fonctions codables

| FSW | Traduction | Valeurs possibles | Explication |
|---|---|---|---|
| `DEF_FLC_SCHWELLWERT_SATZ` | defaut · _FLC_ · valeur de seuil · **SATZ** | `wert_01` · `wert_02` · `wert_03` |  |
| `EMPF_OHNE_RAENDELRAD` | **EMPF** · sans · **RAENDELRAD** | `empfindlichkeitsstufe_1` · `empfindlichkeitsstufe_2` · `empfindlichkeitsstufe_3` · `empfindlichkeitsstufe_4` |  |
| `FLC_SCHWELLWERT_SATZ` | _FLC_ · valeur de seuil · **SATZ** | `wert_01` · `wert_02` · `wert_03` |  |
| `KONFIG_NACHL_REGEN` | configuration · **NACHL** · pluie | `wert_01` |  |
| `KONFIG_RLS_GESCHW` | configuration · capteur pluie/lumiere · vitesse | `aktiv` · `nicht_aktiv` |  |
| `KONFIG_RLS_RAENDELRAD` | configuration · capteur pluie/lumiere · **RAENDELRAD** | `aktiv` · `nicht_aktiv` |  |
| `KONFIG_RLS_REGEN` | configuration · capteur pluie/lumiere · pluie | `aktiv` · `nicht_aktiv` |  |
| `TYP_FRONTSCHEIBE` | type · **FRONTSCHEIBE** | `serienscheibe` · `klimakomfort` |  |
| `WISCHEN_STUFE2` | essuyage · niveau 2 | `aktiv` · `nicht_aktiv` |  |
| `WISCHERMOTOR_RSK` | **WISCHERMOTOR** · **RSK** | `aktiv` · `nicht_aktiv` |  |

---

**Traduction** : décomposition du nom allemand — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments en **gras** n'ont pas de traduction établie, ceux entre `_underscores_` sont ambigus.

**Valeurs possibles** : extraites des DATEN. `aktiv`/`nicht_aktiv` = interrupteur,
`wert_xx` = valeur numérique, autres = choix nommés.

**Explication** : rédigée uniquement pour les fonctions identifiées avec certitude.
Une case vide ne veut pas dire que la fonction est sans intérêt — seulement que son effet
n'a pas été établi de source sûre.
