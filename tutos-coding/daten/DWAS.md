# DWAS — Sirene d'alarme

🟠 Module `DWAS` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Source

| | |
|---|---|
| Variante retenue | `DWASINE.C03` |
| FSW dans cette variante | **26** |
| FSW toutes variantes confondues | 27 |
| Fichiers DATEN analyses | 4 |

Variantes disponibles : `DWASINE.C02`, `DWASINE.C03`, `DWASINE.C04`, `DWASINE.C05`

> La variante reellement installee sur le vehicule est identifiee par NCS Dummy
> a l'ouverture du `.TRC`. Si elle differe de celle retenue ici, la liste change.

---

## Les 26 fonctions codables

| FSW | Traduction |
|---|---|
| `ALARMTYP` | **ALARMTYP** |
| `ALARM_LEVEL_X` | alarme · niveau · X |
| `ALARM_LEVEL_Y` | alarme · niveau · Y |
| `ALARM_PAUSE_TIME` | alarme · **PAUSE** · duree |
| `ALARM_TIME_EXT_SUPPLY` | alarme · duree · **EXT** · alimentation |
| `ALARM_TIME_SELF_SUPPLY` | alarme · duree · **SELF** · alimentation |
| `BACK_UP_BATTERY_CAPACITY` | **BACK** · haut · **BATTERY** · **CAPACITY** |
| `BACK_UP_BATTERY_DATA` | **BACK** · haut · **BATTERY** · **DATA** |
| `BUZZER_HIGH_LEVEL` | avertisseur · haut · niveau |
| `BUZZER_LOW_LEVEL` | avertisseur · bas · niveau |
| `BUZZER_SEMIPERIOD` | avertisseur · **SEMIPERIOD** |
| `BUZZER_TIME` | avertisseur · duree |
| `CODIERUNG_ERFOLGT` | codage · effectue |
| `CONTROL_BYTE` | **CONTROL** · **BYTE** |
| `DUTY_CICLE_EXT_SUPPLY` | rapport cyclique · **CICLE** · **EXT** · alimentation |
| `ECE_MODULATION` | norme europeenne · **MODULATION** |
| `ECE_SEMIPERIOD` | norme europeenne · **SEMIPERIOD** |
| `MAX_ALARM_TEMP` | maximum · alarme · temperature |
| `MAX_CHARGING_TEMP` | maximum · **CHARGING** · temperature |
| `MIN_ALARM_TEMP` | minimum · alarme · temperature |
| `MIN_CHARGING_TEMP` | minimum · **CHARGING** · temperature |
| `POWER_REDUCTION_DUTY` | **POWER** · **REDUCTION** · rapport cyclique |
| `POWER_REDUCTION_TEMP` | **POWER** · **REDUCTION** · temperature |
| `USA_MAX_SEMIPERIOD` | marche americain · maximum · **SEMIPERIOD** |
| `USA_MIN_SEMIPERIOD` | marche americain · minimum · **SEMIPERIOD** |
| `USA_SWEEP` | marche americain · **SWEEP** |

---

Les fragments en **gras** n'ont pas de traduction etablie — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments entre `_underscores_` sont ambigus : leur sens depend du module.
