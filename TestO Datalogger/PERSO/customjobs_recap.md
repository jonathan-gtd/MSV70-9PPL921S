# Recap des customjobs MSV70

⚠️ **Un seul job actif à la fois**, jamais combiner — sinon corruption de scaling.

## Conditions communes à tous les jobs (6 canaux)

| Opcode | Variable | Unité |
|---|---|---|
| 0x4807 | RPM | rpm |
| 0x5804 | Load calc | % |
| 0x4300 | Coolant | °C |
| 0x4200 | IAT (intake air) | °C |
| 0x480B | Pedal | % |
| 0x5AB1 | Speed | km/h |

## Contenu spécifique de chaque job (10 canaux thématiques)

### LAMBDA — mixture E85
| Opcode | Variable | Unité |
|---|---|---|
| 0x5A50 / 0x5A51 | Lambda upstream B1 / B2 | – |
| 0x5A81 / 0x5A82 | STFT Bank (FAC_LAM_LIM) B1 / B2 | % |
| 0x5806 / 0x5808 | STFT Group (OBD_LAM_COR) B1 / B2 | % |
| 0x5A85 / 0x5A86 | LTFT mul B1 / B2 | % |
| 0x5A83 / 0x5A84 | LTFT add B1 / B2 | mg/stk |

### POSTCAT — diag P2096/P2098
| Opcode | Variable | Unité |
|---|---|---|
| 0x5A13 / 0x5A14 | LSH downstream B1 / B2 | V |
| 0x5A11 / 0x5A12 | LSU upstream B1 / B2 | V |
| 0x58F5 / 0x58F6 | Shift input post-cat B1 / B2 | V |
| 0x5A87 / 0x5A88 | Adapt post-cat B1 / B2 | – |
| 0x5A81 / 0x5A82 | STFT Bank B1 / B2 | % |

### KNOCK — allumage
| Opcode | Variable | Unité |
|---|---|---|
| 0x5A49 | Avance Cyl1 | °CRK |
| 0x5A36 | Status knock | 0/1 |
| 0x5A37 → 0x5A3C | Knock voltage Cyl 1 / 2 / 3 / 4 / 5 / 6 | V |
| 0x58B1 | Spark burn duration Cyl1 | ms |
| 0x4402 | Oil temp | °C |

### CAT_EFF — efficacité catalyseur
| Opcode | Variable | Unité |
|---|---|---|
| 0x5A13 / 0x5A14 | LSH downstream B1 / B2 | V |
| 0x5A91 / 0x5A92 | Cat diag value B1 / B2 | – |
| 0x5896 / 0x5897 | Temp gaz post-cat B1 / B2 | °C |
| 0x5A50 / 0x5A51 | Lambda upstream B1 / B2 | – |
| 0x5A81 / 0x5A82 | STFT Bank B1 / B2 | % |

### INJ — équilibre cylindres
| Opcode | Variable | Unité |
|---|---|---|
| 0x5A42 → 0x5A47 | Inj time Cyl 1 / 2 / 3 / 4 / 5 / 6 | ms |
| 0x5A50 / 0x5A51 | Lambda upstream B1 / B2 | – |
| 0x5A49 | Avance Cyl1 | °CRK |
| 0x5A36 | Status knock | 0/1 |

### VANOS — VANOS + Valvetronic
| Opcode | Variable | Unité |
|---|---|---|
| 0x4500 / 0x4501 | Excenter VVT actuel / target | ° |
| 0x4505 / 0x4508 | Cames admission target / actuel | °CRK |
| 0x5A94 / 0x4509 | Cames échappement target / actuel | °CRK |
| 0x5A50 / 0x5A51 | Lambda upstream B1 / B2 | – |
| 0x5A49 | Avance Cyl1 | °CRK |
| 0x5A81 | STFT Bank B1 | % |

### FUEL_SYS — système carburant
| Opcode | Variable | Unité |
|---|---|---|
| 0x580A | Pression rail | kPa |
| 0x5ABA | PWM pompe | % |
| 0x58AF | Débit consigne pompe | l/h |
| 0x583B | Niveau tank | l |
| 0x5A50 / 0x5A51 | Lambda upstream B1 / B2 | – |
| 0x5A81 / 0x5A82 | STFT Bank B1 / B2 | % |
| 0x5A83 | LTFT add B1 | mg/stk |
| 0x5A42 | Inj time Cyl1 | ms |
