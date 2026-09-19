# DWAS — Sirene d'alarme

🟠 Module `DWAS` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Identification

| | |
|---|---|
| Variante | **`DWASINE.C02`** |
| Identifiée par | recoupement avec `DWAS.TRC` lu sur le véhicule |
| Fonctions retenues | **26** |
| Écartées (hors blocs du TRC) | 0 |
| Avec valeurs | 15 |
| Avec explication rédigée | 5 |
| Blocs d'adresses du TRC | 4 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 26 fonctions codables

| FSW | Traduction | Valeurs | Explication |
|---|---|---|---|
| `ALARMTYP` | **ALARMTYP** | `ece` · `us` · `gb` | Type d'alarme selon le marche : ECE (Europe), US, GB. |
| `ALARM_LEVEL_X` | alarme · niveau · X | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `ALARM_LEVEL_Y` | alarme · niveau · Y | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `ALARM_PAUSE_TIME` | alarme · pause · duree | — |  |
| `ALARM_TIME_EXT_SUPPLY` | alarme · duree · externe · alimentation | — |  |
| `ALARM_TIME_SELF_SUPPLY` | alarme · duree · **SELF** · alimentation | — |  |
| `BACK_UP_BATTERY_CAPACITY` | **BACK** · haut · batterie · **CAPACITY** | `wert_02` |  |
| `BACK_UP_BATTERY_DATA` | **BACK** · haut · batterie · donnees | `rechargeable_battery` · `primary_battery` |  |
| `BUZZER_HIGH_LEVEL` | avertisseur · haut · niveau | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` | Niveau sonore haut du bip d'alarme — wert_01 est le maximum. |
| `BUZZER_LOW_LEVEL` | avertisseur · bas · niveau | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` | Niveau sonore bas du bip d'alarme — wert_01 est le maximum. |
| `BUZZER_SEMIPERIOD` | avertisseur · demi-periode | `wert_01` | Niveau sonore de l'avertisseur d'alarme. |
| `BUZZER_TIME` | avertisseur · duree | — | Niveau sonore de l'avertisseur d'alarme. |
| `CODIERUNG_ERFOLGT` | codage · effectue | `codierung_erfolgt` |  |
| `CONTROL_BYTE` | **CONTROL** · **BYTE** | `ok` · `default_value_used` · `default_value_in_use` |  |
| `DUTY_CICLE_EXT_SUPPLY` | rapport cyclique · **CICLE** · externe · alimentation | — |  |
| `ECE_MODULATION` | norme europeenne · **MODULATION** | — |  |
| `ECE_SEMIPERIOD` | norme europeenne · demi-periode | — |  |
| `MAX_ALARM_TEMP` | maximum · alarme · temperature | `wert_02` · `wert_03` |  |
| `MAX_CHARGING_TEMP` | maximum · charge · temperature | `wert_02` |  |
| `MIN_ALARM_TEMP` | minimum · alarme · temperature | `minus20c` · `wert_02` |  |
| `MIN_CHARGING_TEMP` | minimum · charge · temperature | `minus5c` · `wert_02` |  |
| `POWER_REDUCTION_DUTY` | puissance · reduction · rapport cyclique | — |  |
| `POWER_REDUCTION_TEMP` | puissance · reduction · temperature | `minus5c` · `wert_02` |  |
| `USA_MAX_SEMIPERIOD` | marche americain · maximum · demi-periode | — |  |
| `USA_MIN_SEMIPERIOD` | marche americain · minimum · demi-periode | — |  |
| `USA_SWEEP` | marche americain · **SWEEP** | — |  |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Explication** — rédigée seulement quand l'effet est établi de source sûre.
