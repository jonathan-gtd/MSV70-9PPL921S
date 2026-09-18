# RLS — Capteur pluie / luminosite

🟢 Module `RLS` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟢 Module sans fonction de securite. Sauvegarde du `.TRC` quand meme.

---

## Source

| | |
|---|---|
| Variante retenue | `RLS_PL2.C07` |
| FSW dans cette variante | **10** |
| FSW toutes variantes confondues | 14 |
| Fichiers DATEN analyses | 3 |

Variantes disponibles : `RLS_PL2.C07`, `RLS_PL2.C08`, `RLS_E65.C06`

> La variante reellement installee sur le vehicule est identifiee par NCS Dummy
> a l'ouverture du `.TRC`. Si elle differe de celle retenue ici, la liste change.

---

## Les 10 fonctions codables

| FSW | Traduction |
|---|---|
| `DEF_FLC_SCHWELLWERT_SATZ` | defaut · _FLC_ · valeur de seuil · **SATZ** |
| `EMPF_OHNE_RAENDELRAD` | **EMPF** · sans · **RAENDELRAD** |
| `FLC_SCHWELLWERT_SATZ` | _FLC_ · valeur de seuil · **SATZ** |
| `KONFIG_NACHL_REGEN` | configuration · **NACHL** · pluie |
| `KONFIG_RLS_GESCHW` | configuration · capteur pluie/lumiere · vitesse |
| `KONFIG_RLS_RAENDELRAD` | configuration · capteur pluie/lumiere · **RAENDELRAD** |
| `KONFIG_RLS_REGEN` | configuration · capteur pluie/lumiere · pluie |
| `TYP_FRONTSCHEIBE` | type · **FRONTSCHEIBE** |
| `WISCHEN_STUFE2` | essuyage · niveau 2 |
| `WISCHERMOTOR_RSK` | **WISCHERMOTOR** · **RSK** |

---

Les fragments en **gras** n'ont pas de traduction etablie — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments entre `_underscores_` sont ambigus : leur sens depend du module.
