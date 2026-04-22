# §4 — Film mural (Wall Film Correction)

> Le film de carburant sur les parois du collecteur est **~25–35% plus épais sur E85** (éthanol s'évapore moins vite, viscosité plus élevée à froid). En accélération, ce film absorbe du carburant — la chambre reçoit moins que prévu → lean transitoire. Les 4 tables positives doivent être multipliées par ×1.25. Les tables négatives (décélération) ne sont pas modifiées en première intention.

**Procédure commune aux 4 tables :** TunerPro → ouvrir la table → Ctrl+A → Scale ×1.25 → valider → répéter ×4.

**Pourquoi 4 tables ?**

| Dimension | Tables "slow" | Tables "fast" |
|---|---|---|
| Dynamique | Film résiduel (accumulation sur plusieurs secondes) | Réponse instantanée au tip-in brutal |
| Valeurs | Plus élevées (couche permanente) | Plus faibles (correction transitoire) |
| Axes | TCO × RPM | TCO × RPM |

| Dimension | Tables "opm_1" (Valvetronic) | Tables "opm_2" (papillonné GD) |
|---|---|---|
| Mode actif | Fonctionnement normal N52 | Démarrage froid, mode dégradé |
| Valeurs opm_2 slow | Nettement plus élevées (film plus épais en mode GD) | — |

---

## ① `ip_ti_tco_pos_slow_wf_opm_1` — Film lent positif, Valvetronic

| Champ | Valeur |
|---|---|
| Adresse | 0x4CBFC |
| Structure | Map 8×8, uint16 |
| Équation | `0.100 × raw_u16be` |
| Axes | X = TCO (°C), Y = RPM (608–5600) |

**Rôle :** Correction d'enrichissement pour le film de carburant lent (composante résiduelle, constante de temps de plusieurs secondes) en mode Valvetronic, direction positive (accélération / augmentation de charge). C'est la table principale du film mural — elle gère l'enrichissement continu pour compenser le film qui s'accumule sur les parois entre les injections. Sur E85, ce film est plus épais à cause de la viscosité plus élevée de l'éthanol et de sa plus faible pression de vapeur → multiplier par ×1.25.

**Avant / Après :**

```
AVANT (stock) :
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :   36.0   36.0   32.0   26.0   18.0    8.0    4.0    2.0
 896 rpm :   38.0   38.0   34.0   28.0   20.0   10.0    6.0    2.0
1216 rpm :   40.0   40.0   36.0   30.0   22.0   12.0    8.0    4.0
1600 rpm :   42.0   42.0   38.0   32.0   24.0   14.0   10.0    6.0
2208 rpm :   46.0   46.0   42.0   36.0   28.0   18.0   14.0   10.0
3008 rpm :   50.0   50.0   46.0   40.0   32.0   22.0   18.0   14.0
4192 rpm :   54.0   54.0   50.0   44.0   36.0   26.0   22.0   18.0
5600 rpm :   58.0   58.0   54.0   48.0   40.0   30.0   26.0   22.0

APRÈS (×1.25) :
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :   45.0   45.0   40.0   32.5   22.5   10.0    5.0    2.5
 896 rpm :   47.5   47.5   42.5   35.0   25.0   12.5    7.5    2.5
1216 rpm :   50.0   50.0   45.0   37.5   27.5   15.0   10.0    5.0
1600 rpm :   52.5   52.5   47.5   40.0   30.0   17.5   12.5    7.5
2208 rpm :   57.5   57.5   52.5   45.0   35.0   22.5   17.5   12.5
3008 rpm :   62.5   62.5   57.5   50.0   40.0   27.5   22.5   17.5
4192 rpm :   67.5   67.5   62.5   55.0   45.0   32.5   27.5   22.5
5600 rpm :   72.5   72.5   67.5   60.0   50.0   37.5   32.5   27.5
```

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Accélération 80% moteur à 50°C | Réponse nette, aucun trou | À-coup riche → table slow trop élevée, −5% |
| Ralenti après décélération | Stable, pas d'oscillation RPM | Instable → interaction film/lambda, pas cette table |

---

## ② `ip_ti_tco_pos_slow_wf_opm_2` — Film lent positif, papillonné (GD)

| Champ | Valeur |
|---|---|
| Adresse | 0x4CC7C |
| Structure | Map 8×8, uint16 |
| Équation | `0.100 × raw_u16be` |
| Axes | X = TCO (°C), Y = RPM (608–5600) |

**Rôle :** Même rôle qu'opm_1 mais pour le mode papillonné (Valvetronic désactivé). Les valeurs stock opm_2 sont nettement plus élevées que opm_1 à froid (ex. 224.7 vs 36.0 à 608 rpm, −30°C) car le mode GD a une géométrie d'admission différente qui crée davantage de film. Le facteur ×1.25 s'applique identiquement — les valeurs absolues après multiplication sont simplement plus élevées.

**Avant / Après :**

```
AVANT (stock) :
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :  224.7  180.1  134.1  114.0   90.0   26.0   20.0   20.0
 896 rpm :  224.7  180.1  132.7  113.5   88.5   28.0   20.0   20.0
1216 rpm :  224.7  180.1  132.9  107.0   81.0   28.0   20.0   20.0
1600 rpm :  224.7  180.1  133.5   76.5   57.1   30.0   20.0   18.0
2208 rpm :  224.7  180.3  134.6   76.7   57.6   32.0   22.0   18.0
3008 rpm :  224.7  185.4  144.7  105.0   78.5   36.0   26.0   26.0
4192 rpm :  234.9  194.5  155.0  127.7   96.6   52.3   37.8   33.6
5600 rpm :  259.4  215.5  172.3  142.7  112.5   69.8   52.9   50.7

APRÈS (×1.25) :
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :  280.9  225.1  167.6  142.5  112.5   32.5   25.0   25.0
 896 rpm :  280.9  225.1  165.9  141.9  110.6   35.0   25.0   25.0
1216 rpm :  280.9  225.1  166.1  133.8  101.3   35.0   25.0   25.0
1600 rpm :  280.9  225.1  166.9   95.6   71.4   37.5   25.0   22.5
2208 rpm :  280.9  225.4  168.3   95.9   72.0   40.0   27.5   22.5
3008 rpm :  280.9  231.8  180.9  131.3   98.1   45.0   32.5   32.5
4192 rpm :  293.6  243.1  193.8  159.6  120.8   65.4   47.3   42.0
5600 rpm :  324.3  269.4  215.4  178.4  140.6   87.3   66.1   63.4
```

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Démarrage froid, accélération douce | Réponse lisse | Trou au démarrage en mode GD → table opm_2 non modifiée |
| Transition Valvetronic → GD | Sans heurt | Heurt → différence opm_1 / opm_2 trop importante |

---

## ③ `ip_ti_tco_pos_fast_wf_opm_1` — Film rapide positif, Valvetronic

| Champ | Valeur |
|---|---|
| Adresse | 0x443FC |
| Structure | Map 8×8, uint8 |
| Équation | `0.500 × raw_u8` |
| Axes | X = TCO (°C), Y = RPM (608–5600) |

**Rôle :** Correction d'enrichissement pour la composante rapide du film mural (réponse instantanée au tip-in brutal), mode Valvetronic. Quand la pédale est enfoncée brusquement, le débit d'air augmente soudainement alors que le film n'a pas encore eu le temps de répondre — cette table injecte un surplus immédiat. Les valeurs sont ~0.4–0.6× plus faibles que la table slow (correction transitoire vs couche permanente). Sur E85, le film plus épais nécessite une correction rapide plus importante → ×1.25.

**Avant / Après :**

```
AVANT (stock) :
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :   22.0   20.5   18.5   15.0   11.0    7.5    6.0    4.5
 896 rpm :   23.5   21.5   19.5   16.0   12.0    8.0    6.5    5.0
1216 rpm :   25.5   23.0   21.0   17.5   13.5    9.0    7.0    5.5
1600 rpm :   28.0   25.5   23.0   19.5   15.5   10.5    8.0    6.5
2208 rpm :   32.5   29.5   26.5   23.0   18.5   13.0   10.5    9.0
3008 rpm :   39.0   35.5   32.0   28.0   23.0   17.0   14.0   12.5
4192 rpm :   52.5   47.5   43.0   38.0   32.0   25.0   21.5   19.5
5600 rpm :   72.5   66.5   60.0   53.5   45.5   37.0   33.0   31.0

APRÈS (×1.25) :
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :   27.5   25.6   23.1   18.8   13.8    9.4    7.5    5.6
 896 rpm :   29.4   26.9   24.4   20.0   15.0   10.0    8.1    6.3
1216 rpm :   31.9   28.8   26.3   21.9   16.9   11.3    8.8    6.9
1600 rpm :   35.0   31.9   28.8   24.4   19.4   13.1   10.0    8.1
2208 rpm :   40.6   36.9   33.1   28.8   23.1   16.3   13.1   11.3
3008 rpm :   48.8   44.4   40.0   35.0   28.8   21.3   17.5   15.6
4192 rpm :   65.6   59.4   53.8   47.5   40.0   31.3   26.9   24.4
5600 rpm :   90.6   83.1   75.0   66.9   56.9   46.3   41.3   38.8
```

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Accélération 30% brusque, moteur à 50°C | Lisse, aucun trou | Trou < 0.5s → table fast +10% ligne TCO correspondante |
| Accélération progressive | Lisse | Trou → table slow insuffisante (pas fast) |

---

## ④ `ip_ti_tco_pos_fast_wf_opm_2` — Film rapide positif, papillonné (GD)

| Champ | Valeur |
|---|---|
| Adresse | 0x4443C |
| Structure | Map 8×8, uint8 |
| Équation | `0.500 × raw_u8` |
| Axes | X = TCO (°C), Y = RPM (608–5600) |

**Rôle :** Même rôle qu'opm_1 mais pour le mode papillonné. Les valeurs stock opm_2 fast sont intermédiaires entre opm_1 fast et opm_2 slow. Active lors des accélérations brutales en mode GD (démarrage froid, mode dégradé). Facteur ×1.25 identique aux 3 autres tables.

**Avant / Après :**

```
AVANT (stock) :
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :   80.0   66.0   46.5   42.0   24.5   12.0   10.0   10.0
 896 rpm :   80.0   66.0   46.0   41.0   26.0   14.5   11.5   11.5
1216 rpm :   80.0   66.0   47.0   37.0   25.5   16.5   13.0   13.0
1600 rpm :   80.0   66.0   48.0   34.5   25.0   19.0   14.5   14.5
2208 rpm :   80.0   66.0   50.5   35.0   27.5   22.0   16.0   16.0
3008 rpm :   81.5   68.5   52.5   42.5   33.0   26.5   17.0   17.0
4192 rpm :   88.0   73.0   57.0   49.0   40.0   30.5   23.0   20.5
5600 rpm :   94.0   78.5   61.5   54.5   47.5   34.5   28.5   26.0

APRÈS (×1.25) :
TCO (°C) →  -30.0  -15.0    0.0   15.0   35.2   65.2   84.8  114.8
 608 rpm :  100.0   82.5   58.1   52.5   30.6   15.0   12.5   12.5
 896 rpm :  100.0   82.5   57.5   51.3   32.5   18.1   14.4   14.4
1216 rpm :  100.0   82.5   58.8   46.3   31.9   20.6   16.3   16.3
1600 rpm :  100.0   82.5   60.0   43.1   31.3   23.8   18.1   18.1
2208 rpm :  100.0   82.5   63.1   43.8   34.4   27.5   20.0   20.0
3008 rpm :  101.9   85.6   65.6   53.1   41.3   33.1   21.3   21.3
4192 rpm :  110.0   91.3   71.3   61.3   50.0   38.1   28.8   25.6
5600 rpm :  117.5   98.1   76.9   68.1   59.4   43.1   35.6   32.5
```

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Accélération brusque mode GD (démarrage froid) | Aucun trou bref | Trou → opm_2 fast insuffisant, +10% colonnes froides |
| Toutes tables ×1.25 appliquées | Transitions lisses | Trou persistant → affiner ligne TCO correspondante +10% |

---

## Note — Tables négatives et Valvetronic

**Tables `_neg_*_wf` (non modifiées en première intention) :**
Les tables négatives gèrent la récupération de film en tip-out (levée de pied) — elles sont statiques dans le bin. La sous-correction produit un mélange légèrement riche en décélération (sans risque), contrairement aux tables positives où la sous-correction produit un lean transitoire (risque réel). Corriger en deuxième étape (×1.25 identique), une fois les tables positives validées sur route.

**Particularité Valvetronic N52 :**
Les tables `ip_fac_ti_maf_sp_wf_*_opm_1` gèrent le film mural induit par les changements de levée Valvetronic. Sur E85, un +15–20% peut être nécessaire si des ratés apparaissent lors d'accélérations progressives (pas brutales). À régler uniquement si les 4 tables principales ×1.25 sont validées et que des trous persistent exclusivement lors de transitions Valvetronic douces.

**Test pratique recommandé :**
```
1. Démarrer moteur froid
2. Conduire 10 min à vitesse modérée (TCO monte à ~50°C)
3. À feu rouge (TCO ~50°C, ralenti) : accélérer progressivement 30%
   → Doit être LISSE, aucun trou ni hésitation
4. Accélérer brusquement 80% :
   → Réponse nette, pas de latence

Trou/hésitation → film insuffisant → +10% sur ip_ti_tco_pos_fast ligne correspondante
À-coup riche → film trop fort → −5% sur ip_ti_tco_pos_slow
```
