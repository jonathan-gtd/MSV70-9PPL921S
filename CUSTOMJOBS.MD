# 🏎️ Guide de Logging E85 - BMW N52 (MSV70)

Ce guide détaille la configuration et l'interprétation des 4 jobs personnalisés pour Testo. Chaque job est limité à 12 paramètres maximum pour garantir une vitesse d'acquisition fluide.

---

## 🟢 JOB 01 : 01_E85_WARMUP_STAT
**Usage :** Au garage, du démarrage à froid jusqu'à la température de fonctionnement.
**Objectif :** Valider le "Cold Start" et la stabilisation du mélange au ralenti.

### Détail des IDs :
* **0x4300 (Cooling water temp) :** Température du liquide de refroidissement. Permet de savoir quand le moteur quitte la stratégie de départ à froid.
* **0x4807 (Engine speed) :** Régime moteur (tr/min). Pour vérifier la stabilité du ralenti.
* **0x5A42 (Injection time cyl 1) :** Temps d'ouverture des injecteurs (ms). Doit être élevé à froid puis descendre vers 2.5-3.5ms à chaud.
* **0x5A50 (Lambda Bank 1) :** Richesse réelle lue par la sonde 1. Doit tendre vers 1.00 à chaud.
* **0x5A51 (Lambda Bank 2) :** Richesse réelle lue par la sonde 2. Pour vérifier l'équilibre entre les bancs.
* **0x5A81 (Integrator Bank 1) :** Correction immédiate (STFT). Si > 0, le DME ajoute de l'essence.
* **0x5A82 (Integrator Bank 2) :** Idem pour le banc 2.
* **0x5A83 (Adapt. Offset 1) :** **CRUCIAL** - Correction spécifique au ralenti. Si la valeur est très positive, il manque d'essence au ralenti.
* **0x5A84 (Adapt. Offset 2) :** Idem pour le banc 2.
* **0x5A85 (Adapt. Multiplicative 1) :** Apprentissage global (LTFT). Reflète l'enrichissement global dû à l'éthanol.
* **0x5A86 (Adapt. Multiplicative 2) :** Idem pour le banc 2.

---

## 🔴 JOB 02 : 02_E85_WOT_PULL
**Usage :** En roulant, 3ème vitesse, pied au plancher (2000 à 7000 tr/min).
**Objectif :** Vérifier la sécurité du mélange à pleine charge et la saturation des injecteurs.

### Détail des IDs :
* **0x4704 (Lambda setpoint) :** La cible de richesse demandée par la cartographie (ex: 0.85).
* **0x4807 (Engine speed) :** Régime moteur. Indispensable pour l'axe de rotation des graphiques.
* **0x480B (Pedal value %) :** Position de la pédale. Confirme que le test est bien effectué à 100% de charge.
* **0x5812 (Measured air mass) :** Débit d'air réel (kg/h). Indique si le moteur "respire" bien.
* **0x5A42 (Injection time cyl 1) :** **VITAL** - Si > 18.5ms à haut régime, les injecteurs sont saturés (ouverts à 100%).
* **0x5A50 (Lambda Bank 1) :** Richesse réelle. Doit être proche du setpoint (0x4704). Danger si > 0.90 à haut régime.
* **0x5A51 (Lambda Bank 2) :** Idem pour le banc 2.
* **0x5A81 (Integrator Bank 1) :** Correction instantanée. Si elle sature à +25%, le système d'essence est à bout.
* **0x5A82 (Integrator Bank 2) :** Idem pour le banc 2.
* **0x5A85 (Adapt. Multiplicative 1) :** Taux d'enrichissement mémorisé.
* **0x5A86 (Adapt. Multiplicative 2) :** Idem pour le banc 2.

---

## ⚡ JOB 03 : 03_E85_KNOCK_IGNITION
**Usage :** Après avoir validé la richesse du Job 02.
**Objectif :** Optimiser l'avance à l'allumage et détecter d'éventuels cliquetis.

### Détail des IDs :
* **0x4102 (VVT Eccentric shaft) :** Angle du Valvetronic. Doit être à ~250° (ouverture max) lors d'une accélération.
* **0x4300 (Cooling water temp) :** Surveillance de la température pour éviter la surchauffe durant les tests.
* **0x4807 (Engine speed) :** Régime moteur.
* **0x480B (Pedal value %) :** Position pédale.
* **0x5813 (Relative load) :** Charge moteur calculée. Détermine la case utilisée dans la table d'allumage.
* **0x5A3E (Knock signal 1 rel.) :** Activité du capteur de cliquetis cylindre 1. Un pic indique une combustion anormale.
* **0x5A40 (Knock signal 6 rel.) :** Activité du cliquetis cylindre 6 (souvent le plus chaud).
* **0x5A49 (Ignition cyl 1) :** Avance à l'allumage finale en degrés. Plus l'avance est grande, plus le rendement est élevé.

---

## 🔍 JOB 04 : 04_E85_HEALTH_CHECK
**Usage :** Diagnostic général ou en cas de comportement moteur anormal.
**Objectif :** Vérifier la santé physique des capteurs (Sondes Lambda et Débitmètre).

### Détail des IDs :
* **0x4807 (Engine speed) :** Régime moteur.
* **0x5812 (Measured air mass) :** Masse d'air mesurée par le débitmètre (MAF).
* **0x5818 (Calculated air mass) :** Masse d'air théorique calculée par l'ECU. Une différence > 10% avec le MAF indique une fuite d'air.
* **0x5A11 (Voltage Lambda 1) :** Tension brute (V) de la sonde 1. Permet de voir si elle est HS ou encrassée.
* **0x5A12 (Voltage Lambda 2) :** Tension brute (V) de la sonde 2.
* **0x5A50 (Lambda Bank 1) :** Richesse réelle convertie.
* **0x5A51 (Lambda Bank 2) :** Richesse réelle convertie.

---

### 🚦 Seuils de Sécurité N52 (E85)
* **Injection Time :** < 18.5 ms (au-delà, l'injecteur sature).
* **Lambda (WOT) :** Entre 0.82 et 0.88.
* **Température Eau :** Entre 85°C et 110°C.