# §17 — Stratégie de chauffe catalyseur

> Paramètres de retard d'allumage appliqués au démarrage froid pour générer des EGT élevées et accélérer le light-off catalyseur. Sur E85, la combustion est plus froide (~30–50°C de moins en EGT) → le catalyseur met un peu plus longtemps à monter en température. **Ne pas modifier si les catalyseurs sont présents.** Les valeurs sont identiques entre le bin stock et le bin E85 de référence.

---

<a id="p1"></a>
## ① `ip_fac_eff_iga_ch_cold_opm_1` — Retard allumage chauffe catalyseur, Valvetronic

| Champ | Valeur |
|---|---|
| Adresse | 0x4A444 |
| Structure | Map 10×10 |
| Axes | X = TPS% (10–65%), Y = RPM (704–5888) |

**Rôle :** Retard d'allumage appliqué en mode Valvetronic pendant la phase de chauffe catalyseur (démarrage froid, avant que le catalyseur atteigne sa température de fonctionnement). Un allumage retardé augmente les EGT et accélère le light-off. Sur E85, la combustion est légèrement plus froide — le catalyseur met environ 20% plus longtemps à atteindre sa température. Modifier uniquement si les catalyseurs sont retirés (voir note ci-dessous).

**Avant / Après :**

**◀ Avant — Stock = E85 bin (inchangé — ne pas modifier si catalyseurs présents) (°CRK)**

| RPM (tr/min) \ TPS (%) | 10 | 15 | 20 | 25 | 30 | 35 | 40 | 45 | 50 | 65 |
|---|---|---|---|---|---|---|---|---|---|---|
| 704 | 5.98 | 6.95 | 6.51 | 6.51 | 5.98 | 5.50 | 4.58 | 4.49 | 4.36 | 4.40 |
| 896 | 5.98 | 7.30 | 6.86 | 6.69 | 6.25 | 5.54 | 4.93 | 4.58 | 4.49 | 4.49 |
| 1056 | 6.12 | 7.48 | 7.48 | 6.25 | 5.76 | 5.24 | 5.02 | 4.88 | 4.62 | 4.62 |
| 1216 | 6.20 | 7.48 | 7.26 | 5.98 | 5.50 | 5.02 | 4.80 | 4.80 | 4.75 | 4.75 |
| 1504 | 5.28 | 7.35 | 6.25 | 5.50 | 5.24 | 4.75 | 4.75 | 4.75 | 4.53 | 4.75 |
| 1760 | 4.22 | 7.00 | 5.63 | 4.88 | 4.71 | 4.75 | 4.80 | 4.80 | 4.80 | 4.88 |
| 2016 | 3.83 | 5.94 | 4.44 | 4.14 | 3.92 | 4.53 | 4.58 | 4.80 | 4.80 | 4.66 |
| 2304 | 3.39 | 4.71 | 3.08 | 3.30 | 2.99 | 3.65 | 4.31 | 4.58 | 4.80 | 4.40 |
| 2848 | 3.30 | 4.14 | 2.82 | 3.30 | 3.78 | 4.40 | 4.71 | 5.10 | 5.37 | 5.06 |
| 5888 | 5.76 | 4.84 | 4.53 | 4.88 | 4.84 | 5.72 | 6.03 | 5.02 | 4.40 | 4.75 |

> Si catalyseurs retirés : mettre toutes les cellules à **0.000 °CRK** (aucun retard).

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| DTC P0420 / P0430 (premiers 200–500 km E85) | Normaux — disparaissent après stabilisation LTFT | Persistent > 1000 km → catalyseurs défaillants |
| Démarrage froid, régime de ralenti initial | Stable, montée progressive | Instable → vérifier §5 cranking, pas chauffe cat |

---

<a id="p2"></a>
## ② `ip_fac_eff_iga_ch_cold_opm_2` — Retard allumage chauffe catalyseur, papillonné (GD)

| Champ | Valeur |
|---|---|
| Adresse | 0x4A4A8 |
| Structure | Map 10×10 |
| Axes | X = TPS% (10–105%), Y = RPM (512–5888) |

**Rôle :** Même rôle qu'opm_1 mais pour le mode papillonné (Valvetronic désactivé). Active au démarrage froid avant que le Valvetronic soit opérationnel. Les axes TPS couvrent une plage plus large (jusqu'à 105%) car le mode papillonné autorise des ouvertures plus importantes. Modifier uniquement si les catalyseurs sont retirés — même logique qu'opm_1.

**Avant / Après :**

**◀ Avant — Stock = E85 bin (inchangé — ne pas modifier si catalyseurs présents) (°CRK)**

| RPM (tr/min) \ TPS (%) | 10 | 20 | 25 | 30 | 35 | 40 | 50 | 65 | 85 | 105 |
|---|---|---|---|---|---|---|---|---|---|---|
| 512 | 3.30 | 4.71 | 5.63 | 6.12 | 6.42 | 6.34 | 5.50 | 5.50 | 5.50 | 5.50 |
| 704 | 3.30 | 4.14 | 4.93 | 5.46 | 5.76 | 5.76 | 5.50 | 4.84 | 4.84 | 4.84 |
| 896 | 3.78 | 4.00 | 4.05 | 4.09 | 4.71 | 5.54 | 6.20 | 4.75 | 4.75 | 4.75 |
| 1088 | 3.74 | 3.92 | 3.96 | 4.05 | 4.40 | 5.54 | 5.50 | 5.32 | 5.32 | 5.32 |
| 1344 | 3.34 | 3.96 | 4.14 | 4.27 | 4.49 | 4.75 | 5.06 | 5.15 | 5.15 | 5.15 |
| 1760 | 2.77 | 3.56 | 3.96 | 4.22 | 4.49 | 4.62 | 4.75 | 4.58 | 4.40 | 4.40 |
| 2304 | 2.64 | 4.09 | 4.62 | 4.84 | 5.10 | 5.24 | 5.41 | 4.97 | 4.88 | 5.06 |
| 3360 | 3.17 | 4.44 | 4.97 | 5.15 | 5.32 | 5.02 | 5.37 | 5.19 | 4.53 | 4.53 |
| 4544 | 4.00 | 4.80 | 5.24 | 5.50 | 5.63 | 5.72 | 5.81 | 5.85 | 4.71 | 4.71 |
| 5888 | 3.92 | 4.75 | 5.19 | 5.76 | 5.81 | 5.68 | 5.46 | 5.68 | 6.03 | 6.07 |

> Si catalyseurs retirés : mettre toutes les cellules à **0.000 °CRK** (aucun retard).

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| DTC P0420 / P0430 | Normaux les 200–500 premiers km | Persistants → catalyseurs défaillants ou LTFT non convergés |
| Sans catalyseurs | Aucun retard allumage à froid | P0420/P0430 → masquer séparément dans le bin |
