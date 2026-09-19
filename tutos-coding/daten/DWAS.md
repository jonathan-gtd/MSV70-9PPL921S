# DWAS — Sirene d'alarme

🟠 Module `DWAS` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Source

| | |
|---|---|
| Variante retenue | `DWASINE.C03` |
| Fonctions codables | **26** |
| Avec valeurs extraites | 15 (57 %) |
| Avec explication rédigée | 5 |
| Fichiers DATEN analysés | 4 |

Variantes disponibles : `DWASINE.C02`, `DWASINE.C03`, `DWASINE.C04`, `DWASINE.C05`

> La variante réellement installée est identifiée par NCS Dummy à l'ouverture du `.TRC`.
> Si elle diffère de celle retenue ici, la liste change.

---

## Les 26 fonctions codables

| FSW | Traduction | Valeurs possibles | Explication |
|---|---|---|---|
| `ALARMTYP` | **ALARMTYP** | `ece` · `us` · `gb` | Type d'alarme selon le marche : ECE (Europe), US, GB. |
| `ALARM_LEVEL_X` | alarme · niveau · X | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `ALARM_LEVEL_Y` | alarme · niveau · Y | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `ALARM_PAUSE_TIME` | alarme · **PAUSE** · duree | — |  |
| `ALARM_TIME_EXT_SUPPLY` | alarme · duree · **EXT** · alimentation | — |  |
| `ALARM_TIME_SELF_SUPPLY` | alarme · duree · **SELF** · alimentation | — |  |
| `BACK_UP_BATTERY_CAPACITY` | **BACK** · haut · **BATTERY** · **CAPACITY** | `wert_02` |  |
| `BACK_UP_BATTERY_DATA` | **BACK** · haut · **BATTERY** · **DATA** | `primary_battery` · `rechargeable_battery` |  |
| `BUZZER_HIGH_LEVEL` | avertisseur · haut · niveau | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` | Niveau sonore haut du bip d'alarme — wert_01 est le maximum. |
| `BUZZER_LOW_LEVEL` | avertisseur · bas · niveau | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` | Niveau sonore bas du bip d'alarme — wert_01 est le maximum. |
| `BUZZER_SEMIPERIOD` | avertisseur · **SEMIPERIOD** | `wert_01` | Niveau sonore de l'avertisseur d'alarme. |
| `BUZZER_TIME` | avertisseur · duree | — | Niveau sonore de l'avertisseur d'alarme. |
| `CODIERUNG_ERFOLGT` | codage · effectue | `codierung_erfolgt` |  |
| `CONTROL_BYTE` | **CONTROL** · **BYTE** | `ok` · `default_value_used` · `default_value_in_use` |  |
| `DUTY_CICLE_EXT_SUPPLY` | rapport cyclique · **CICLE** · **EXT** · alimentation | — |  |
| `ECE_MODULATION` | norme europeenne · **MODULATION** | — |  |
| `ECE_SEMIPERIOD` | norme europeenne · **SEMIPERIOD** | — |  |
| `MAX_ALARM_TEMP` | maximum · alarme · temperature | `wert_02` · `wert_03` |  |
| `MAX_CHARGING_TEMP` | maximum · **CHARGING** · temperature | `wert_02` |  |
| `MIN_ALARM_TEMP` | minimum · alarme · temperature | `wert_02` · `minus20c` |  |
| `MIN_CHARGING_TEMP` | minimum · **CHARGING** · temperature | `wert_02` · `minus5c` |  |
| `POWER_REDUCTION_DUTY` | **POWER** · **REDUCTION** · rapport cyclique | — |  |
| `POWER_REDUCTION_TEMP` | **POWER** · **REDUCTION** · temperature | `wert_02` · `minus5c` |  |
| `USA_MAX_SEMIPERIOD` | marche americain · maximum · **SEMIPERIOD** | — |  |
| `USA_MIN_SEMIPERIOD` | marche americain · minimum · **SEMIPERIOD** | — |  |
| `USA_SWEEP` | marche americain · **SWEEP** | — |  |

---

**Traduction** : décomposition du nom allemand — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments en **gras** n'ont pas de traduction établie, ceux entre `_underscores_` sont ambigus.

**Valeurs possibles** : extraites des DATEN. `aktiv`/`nicht_aktiv` = interrupteur,
`wert_xx` = valeur numérique, autres = choix nommés.

**Explication** : rédigée uniquement pour les fonctions identifiées avec certitude.
Une case vide ne veut pas dire que la fonction est sans intérêt — seulement que son effet
n'a pas été établi de source sûre.
