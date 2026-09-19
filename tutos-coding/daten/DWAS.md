# DWAS — Sirene d'alarme

🟠 Module `DWAS` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Identification

| | |
|---|---|
| Variante | **`DWASINE.C04`** |
| Identifiée par | **lecture NCS Expert sur le véhicule** |
| Fonctions retenues | **26** |
| Vérification | 100 % des fonctions dans les blocs du `.TRC` |
| Avec valeurs | 15 |
| Avec explication rédigée | 26 |
| Blocs d'adresses du TRC | 2 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 26 fonctions codables

| FSW | Décomposition | Valeurs | Signification |
|---|---|---|---|
| `ALARMTYP` | type d'alarme | `ece` · `us` · `gb` | Type d'alarme selon le marche : ECE (Europe), US, GB. |
| `ALARM_LEVEL_X` | alarme · niveau · X | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` | *Alarm level X-axis* |
| `ALARM_LEVEL_Y` | alarme · niveau · Y | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` | *Alarm level Y-axis* |
| `ALARM_PAUSE_TIME` | alarme · pause · duree | — | *Alarm pause time [duree s = valeur / 2]* |
| `ALARM_TIME_EXT_SUPPLY` | alarme · duree · externe · alimentation | — | *Alarm time with external battery supply [duree s = valeur / 2]* |
| `ALARM_TIME_SELF_SUPPLY` | alarme · duree · auto · alimentation | — | *Alarm time with internal backup battery supply [duree s = valeur / 2]* |
| `BACK_UP_BATTERY_CAPACITY` | retour · haut · batterie · capacite | `wert_02` | *Backup battery capacity [capacite mAh = valeur x 10]* |
| `BACK_UP_BATTERY_DATA` | retour · haut · batterie · donnees | `primary_battery` · `rechargeable_battery` | *Backup battery valeur* |
| `BUZZER_HIGH_LEVEL` | avertisseur · haut · niveau | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` | Niveau sonore haut du bip d'alarme — wert_01 est le maximum. |
| `BUZZER_LOW_LEVEL` | avertisseur · bas · niveau | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` | Niveau sonore bas du bip d'alarme — wert_01 est le maximum. |
| `BUZZER_SEMIPERIOD` | avertisseur · demi-periode | `wert_01` | Niveau sonore de l'avertisseur d'alarme. |
| `BUZZER_TIME` | avertisseur · duree | — | Niveau sonore de l'avertisseur d'alarme. |
| `CODIERUNG_ERFOLGT` | codage · effectue | `codierung_erfolgt` | *Coding completed* |
| `CONTROL_BYTE` | commande · octet | `ok` · `default_value_used` · `default_value_in_use` | *Control byte* |
| `DUTY_CICLE_EXT_SUPPLY` | rapport cyclique · cycle · externe · alimentation | — | *Duty cycle with external battery supply [pourcentage = valeur]* |
| `ECE_MODULATION` | norme europeenne · modulation | — | *Modulation half period for Europe siren sound* |
| `ECE_SEMIPERIOD` | norme europeenne · demi-periode | — | *Half period for Europe siren sound* |
| `MAX_ALARM_TEMP` | maximum · alarme · temperature | `wert_02` · `wert_03` | *Maximum temperature for self-powered alarm* |
| `MAX_CHARGING_TEMP` | maximum · charge · temperature | `wert_02` | *Maximum temperature for internal backup battery charging* |
| `MIN_ALARM_TEMP` | minimum · alarme · temperature | `wert_02` · `minus20c` | *Minimum temperature for self-powered alarm* |
| `MIN_CHARGING_TEMP` | minimum · charge · temperature | `wert_02` · `minus5c` | *Minimum temperature for internal backup battery charging* |
| `POWER_REDUCTION_DUTY` | puissance · reduction · rapport cyclique | — | *Duty cycle in power-saving mode* |
| `POWER_REDUCTION_TEMP` | puissance · reduction · temperature | `wert_02` · `minus5c` | *Temperature threshold for power-saving mode* |
| `USA_MAX_SEMIPERIOD` | marche americain · maximum · demi-periode | — | *Maximum half period for USA siren sound* |
| `USA_MIN_SEMIPERIOD` | marche americain · minimum · demi-periode | — | *Minimum half period for USA siren sound* |
| `USA_SWEEP` | marche americain · balayage | — | *Maximum sweep half period for USA siren sound* |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Signification** — texte en clair. En romain : rédigé à la main. *En italique* : description
de la base communautaire NCS Dummy, en anglais, non encore traduite.
Les notations d'unités (`[durée s = valeur / 10]`) sont traduites.
