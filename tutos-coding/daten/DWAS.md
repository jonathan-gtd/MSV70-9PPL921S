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
| Signification (EN) | 26 |
| Explication (FR) | 26 |
| Blocs d'adresses du TRC | 2 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 26 fonctions codables

| FSW | Décomposition | Valeurs | Signification (EN) | Explication (FR) |
|---|---|---|---|---|
| `ALARMTYP` | type d'alarme | `ece` · `us` · `gb` | Alarm type [ece: 1 cycle Europe sound, us: 8 cycles USA sound, gb: 8 cycles Europe sound (United Kingdom)] | Type d'alarme selon le marche : ECE (Europe), US, GB. |
| `ALARM_LEVEL_X` | alarme · niveau · X | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` | Alarm level X-axis | Niveau d'alarme, axe X |
| `ALARM_LEVEL_Y` | alarme · niveau · Y | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` | Alarm level Y-axis | Niveau d'alarme, axe Y |
| `ALARM_PAUSE_TIME` | alarme · pause · duree | — | Alarm pause time [time_s=data/2] | Duree de pause entre cycles d'alarme [duree s = valeur / 2] |
| `ALARM_TIME_EXT_SUPPLY` | alarme · duree · externe · alimentation | — | Alarm time with external battery supply [time_s=data/2] | Duree d'alarme sur alimentation externe [duree s = valeur / 2] |
| `ALARM_TIME_SELF_SUPPLY` | alarme · duree · auto · alimentation | — | Alarm time with internal backup battery supply [time_s=data/2] | Duree d'alarme sur batterie de secours interne [duree s = valeur / 2] |
| `BACK_UP_BATTERY_CAPACITY` | retour · haut · batterie · capacite | `wert_02` | Backup battery capacity [capacity_mAh=data*10] | Capacite de la batterie de secours [capacite mAh = valeur x 10] |
| `BACK_UP_BATTERY_DATA` | retour · haut · batterie · donnees | `primary_battery` · `rechargeable_battery` | Backup battery data | Donnees de la batterie de secours |
| `BUZZER_HIGH_LEVEL` | avertisseur · haut · niveau | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` | High level for buzzer sound | Niveau sonore haut du bip d'alarme — wert_01 est le maximum. |
| `BUZZER_LOW_LEVEL` | avertisseur · bas · niveau | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` | Low level for buzzer sound | Niveau sonore bas du bip d'alarme — wert_01 est le maximum. |
| `BUZZER_SEMIPERIOD` | avertisseur · demi-periode | `wert_01` | Half period for buzzer sound | Niveau sonore de l'avertisseur d'alarme. |
| `BUZZER_TIME` | avertisseur · duree | — | Duration for buzzer sound | Niveau sonore de l'avertisseur d'alarme. |
| `CODIERUNG_ERFOLGT` | codage · effectue | `codierung_erfolgt` | Coding completed | Codage termine |
| `CONTROL_BYTE` | commande · octet | `ok` · `default_value_used` · `default_value_in_use` | Control byte | Octet de commande |
| `DUTY_CICLE_EXT_SUPPLY` | rapport cyclique · cycle · externe · alimentation | — | Duty cycle with external battery supply [percentage_%=data] | Rapport cyclique sur alimentation externe [pourcentage = valeur] |
| `ECE_MODULATION` | norme europeenne · modulation | — | Modulation half period for Europe siren sound | Demi-periode de modulation du son de sirene Europe |
| `ECE_SEMIPERIOD` | norme europeenne · demi-periode | — | Half period for Europe siren sound | Demi-periode du son de sirene Europe |
| `MAX_ALARM_TEMP` | maximum · alarme · temperature | `wert_02` · `wert_03` | Maximum temperature for self-powered alarm | Temperature maximale de fonctionnement de l'alarme autoalimentee |
| `MAX_CHARGING_TEMP` | maximum · charge · temperature | `wert_02` | Maximum temperature for internal backup battery charging | Temperature maximale de charge de la batterie de secours |
| `MIN_ALARM_TEMP` | minimum · alarme · temperature | `wert_02` · `minus20c` | Minimum temperature for self-powered alarm | Temperature minimale de fonctionnement de l'alarme autoalimentee |
| `MIN_CHARGING_TEMP` | minimum · charge · temperature | `wert_02` · `minus5c` | Minimum temperature for internal backup battery charging | Temperature minimale de charge de la batterie de secours |
| `POWER_REDUCTION_DUTY` | puissance · reduction · rapport cyclique | — | Duty cycle in power-saving mode | Rapport cyclique en mode economie d'energie |
| `POWER_REDUCTION_TEMP` | puissance · reduction · temperature | `wert_02` · `minus5c` | Temperature threshold for power-saving mode | Seuil de temperature du mode economie d'energie |
| `USA_MAX_SEMIPERIOD` | marche americain · maximum · demi-periode | — | Maximum half period for USA siren sound | Demi-periode maximale du son de sirene USA |
| `USA_MIN_SEMIPERIOD` | marche americain · minimum · demi-periode | — | Minimum half period for USA siren sound | Demi-periode minimale du son de sirene USA |
| `USA_SWEEP` | marche americain · balayage | — | Maximum sweep half period for USA siren sound | Demi-periode maximale de balayage du son de sirene USA |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Signification (EN)** — description issue de la base communautaire NCS Dummy, verbatim.

**Explication (FR)** — traduction française. Un tiret signifie que la ligne n'est pas
encore traduite. Les notations d'unités sont converties : `[durée s = valeur / 10]`.
