# TestO Datalogger — Configuration BMW N52 (MSV70)

Jobs de logging K-line configurés dans `config/customjobs.xml`. Chaque job utilise `STATUS_MESSWERTBLOCK_LESEN` et est limité à 12 paramètres pour garantir une fréquence d'acquisition fluide.

---

## JOB 01 — 01_E85_WARMUP_STAT

**Usage :** Au garage, du démarrage à froid jusqu'à la température de fonctionnement.  
**Objectif :** Valider le cold start et la stabilisation du mélange au ralenti.

| Adresse | Signal | Description |
|---|---|---|
| 0x4300 | Cooling water temp | Température liquide de refroidissement (°C) |
| 0x4807 | Engine speed | Régime moteur (tr/min) |
| 0x5A42 | Injection time cyl 1 | Durée ouverture injecteur (ms) — doit descendre vers 2.5–3.5 ms à chaud |
| 0x5A50 | Lambda Bank 1 | Richesse réelle sonde 1 — doit tendre vers 1.00 à chaud |
| 0x5A51 | Lambda Bank 2 | Richesse réelle sonde 2 — équilibre entre bancs |
| 0x5A81 | Integrator Bank 1 | STFT B1 — correction immédiate (> 0 = ECU ajoute de l'essence) |
| 0x5A82 | Integrator Bank 2 | STFT B2 |
| 0x5A83 | Adapt. Offset 1 | LTFT idle B1 — correction spécifique ralenti. Très positif = manque d'essence au ralenti |
| 0x5A84 | Adapt. Offset 2 | LTFT idle B2 |
| 0x5A85 | Adapt. Multiplicative 1 | LTFT global B1 — apprentissage global (enrichissement éthanol) |
| 0x5A86 | Adapt. Multiplicative 2 | LTFT global B2 |

---

## JOB 02 — 02_E85_WOT_PULL

**Usage :** En roulant, 3ème vitesse, pied au plancher (2000 à 7000 tr/min).  
**Objectif :** Vérifier la richesse WOT et la saturation des injecteurs.

| Adresse | Signal | Description |
|---|---|---|
| 0x4704 | Lambda setpoint | Cible de richesse demandée par la cartographie (ex : 0.85 WOT) |
| 0x4807 | Engine speed | Régime moteur (tr/min) |
| 0x480B | Pedal value % | Position pédale — confirme le test à 100% charge |
| 0x5812 | Measured air mass | Débit d'air réel MAF (kg/h) |
| 0x5A42 | Injection time cyl 1 | Si > 18.5 ms à haut régime → injecteurs saturés |
| 0x5A50 | Lambda Bank 1 | Richesse réelle B1 — doit suivre le setpoint. Danger si > 0.90 WOT |
| 0x5A51 | Lambda Bank 2 | Richesse réelle B2 |
| 0x5A81 | Integrator Bank 1 | STFT B1 — si sature à +25%, le système est à bout |
| 0x5A82 | Integrator Bank 2 | STFT B2 |
| 0x5A85 | Adapt. Multiplicative 1 | LTFT global B1 |
| 0x5A86 | Adapt. Multiplicative 2 | LTFT global B2 |

---

## JOB 03 — 03_E85_KNOCK_IGNITION

**Usage :** Après avoir validé la richesse avec le Job 02.  
**Objectif :** Optimiser l'avance à l'allumage et détecter les cliquetis.

| Adresse | Signal | Description |
|---|---|---|
| 0x4102 | VVT Eccentric shaft | Angle arbre Valvetronic — doit être ~250° (ouverture max) en accélération |
| 0x4300 | Cooling water temp | Température — surveiller la surchauffe pendant les tests |
| 0x4807 | Engine speed | Régime moteur (tr/min) |
| 0x480B | Pedal value % | Position pédale |
| 0x5813 | Relative load | Charge moteur calculée (%) — détermine la case dans la table allumage |
| 0x5A3E | Knock signal cyl 1 | Activité capteur cliquetis cylindre 1 — pic = combustion anormale |
| 0x5A40 | Knock signal cyl 6 | Activité capteur cliquetis cylindre 6 (souvent le plus chaud) |
| 0x5A49 | Ignition angle cyl 1 | Avance allumage finale (°) — plus c'est élevé, meilleur le rendement |

---

## JOB 04 — 04_E85_HEALTH_CHECK

**Usage :** Diagnostic général ou comportement moteur anormal.  
**Objectif :** Vérifier la santé des capteurs (sondes lambda et débitmètre).

| Adresse | Signal | Description |
|---|---|---|
| 0x4807 | Engine speed | Régime moteur (tr/min) |
| 0x5812 | Measured air mass | Masse d'air mesurée par le MAF (kg/h) |
| 0x5818 | Calculated air mass | Masse d'air théorique calculée par l'ECU — écart > 10% avec 0x5812 = fuite d'air |
| 0x5A11 | Voltage Lambda 1 | Tension brute sonde 1 (V) — détecte une sonde HS ou encrassée |
| 0x5A12 | Voltage Lambda 2 | Tension brute sonde 2 (V) |
| 0x5A50 | Lambda Bank 1 | Richesse réelle B1 |
| 0x5A51 | Lambda Bank 2 | Richesse réelle B2 |

---

## Seuils de sécurité N52 (E85)

| Paramètre | Limite | Conséquence si dépassé |
|---|---|---|
| Injection Time (Ti) | < 18.5 ms @ 6500 RPM | Saturation injecteur → mélange pauvre non compensable |
| Lambda WOT | 0.82 – 0.88 λ | > 0.90 → danger soupapes échappement |
| STFT | ±10% nominal | > +20% → couper immédiatement |
| LTFT | ±10% après 500 km | > ±15% → recalibrer les correcteurs injecteurs |
| Température eau | 85°C – 110°C | > 110°C → arrêt test, vérifier refroidissement |
