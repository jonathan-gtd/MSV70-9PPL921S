# TestO Datalogger — MSV70 Custom Jobs

> **Véhicule :** BMW E9x N52B30 — Siemens MSV70 9PPL921S  
> **Protocole :** K-line — job `STATUS_MESSWERTBLOCK_LESEN`  
> **Format argument :** `mode;adresse1;adresse2;...`  
> **Fichier :** `config/customjobs.xml`

---

## Référence adresses MSV70

| Adresse | Signal | Unité | Confirmé |
|---|---|---|---|
| `0x4102` | Température air admission (IAT) | °C | ✓ |
| `0x4300` | Température liquide refroidissement (TCO) | °C | ✓ log |
| `0x4704` | Pression carburant | hPa | ~ |
| `0x4807` | Régime moteur (RPM) | tr/min | ✓ log |
| `0x480B` | Charge relative moteur | % | ✓ |
| `0x5812` | Débit air mesuré (MAF) | kg/h | ✓ trad. |
| `0x5813` | Débit air calculé | kg/h | ~ |
| `0x5818` | Débit air calculé (variante) | kg/h | ✓ trad. |
| `0x5A11` | Tension sonde lambda B1 | V | ✓ |
| `0x5A12` | Tension sonde lambda B2 | V | ✓ |
| `0x5A3E` | Angle allumage | °CRK | ✓ |
| `0x5A40` | Retard anti-cliquetis | °CRK | ✓ |
| `0x5A42` | Temps injection cylindre 1 | ms | ✓ log+trad. |
| `0x5A49` | Niveau signal cliquetis | V | ~ |
| `0x5A50` | Lambda actuel B1 | λ | ✓ log+trad. |
| `0x5A51` | Lambda actuel B2 | λ | ✓ log |
| `0x5A81` | STFT B1 (intégrateur court terme) | % | ✓ log |
| `0x5A82` | STFT B2 | % | ✓ log |
| `0x5A83` | LTFT multiplicatif B1 | % | ✓ log |
| `0x5A84` | LTFT multiplicatif B2 | % | ✓ log |
| `0x5A85` | LTFT additif B1 | mg/stk | ✓ log |
| `0x5A86` | LTFT additif B2 | mg/stk | ✓ log |

> `✓ log` = confirmé par header CSV  `✓ trad.` = confirmé par jobs_all_en_EN.txt  `~` = inféré par contexte

---

## Jobs configurés

### 01 — E85_WARMUP_STAT
*Observation démarrage et chauffe au garage*

| Adresse | Signal |
|---|---|
| `0x4300` | TCO |
| `0x4807` | RPM |
| `0x5A42` | Temps injection cyl.1 |
| `0x5A50` | Lambda B1 |
| `0x5A51` | Lambda B2 |
| `0x5A81` | STFT B1 |
| `0x5A82` | STFT B2 |
| `0x5A83` | LTFT mul. B1 |
| `0x5A84` | LTFT mul. B2 |
| `0x5A85` | LTFT add. B1 |
| `0x5A86` | LTFT add. B2 |

---

### 02 — E85_WOT_PULL
*Richesse et saturation injecteurs en roulant*

| Adresse | Signal |
|---|---|
| `0x4704` | Pression carburant |
| `0x4807` | RPM |
| `0x480B` | Charge relative |
| `0x5812` | MAF |
| `0x5A42` | Temps injection cyl.1 |
| `0x5A50` | Lambda B1 |
| `0x5A51` | Lambda B2 |
| `0x5A81` | STFT B1 |
| `0x5A82` | STFT B2 |
| `0x5A85` | LTFT add. B1 |
| `0x5A86` | LTFT add. B2 |

---

### 03 — E85_KNOCK_IGNITION
*Sécurité avance à l'allumage*

| Adresse | Signal |
|---|---|
| `0x4102` | IAT |
| `0x4300` | TCO |
| `0x4807` | RPM |
| `0x480B` | Charge relative |
| `0x5813` | Débit air calculé |
| `0x5A3E` | Angle allumage |
| `0x5A40` | Retard cliquetis |
| `0x5A49` | Niveau signal cliquetis |

---

### 04 — E85_HEALTH_CHECK
*Diagnostic sondes lambda et MAF*

| Adresse | Signal |
|---|---|
| `0x4807` | RPM |
| `0x5812` | MAF mesuré |
| `0x5818` | MAF calculé |
| `0x5A11` | Tension sonde B1 |
| `0x5A12` | Tension sonde B2 |
| `0x5A50` | Lambda B1 |
| `0x5A51` | Lambda B2 |
