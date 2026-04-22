# §17 — Stratégie de chauffe catalyseur

| Paramètre | Adresse | Structure | Équation |
|---|---|---|---|
| `ip_fac_eff_iga_ch_cold_opm_1` | 0x4A444 | 10×10, uint8, f(TPS% × RPM) | 0.044 × X (°CRK retard) |
| `ip_fac_eff_iga_ch_cold_opm_2` | 0x4A4A8 | 10×10, uint8, f(TPS% × RPM) | 0.044 × X (°CRK retard) |

Facteurs de retard d'allumage appliqués pendant la chauffe catalyseur pour générer des EGT élevées et accélérer le light-off. L'éthanol brûle à une température légèrement plus basse → EGT démarrage E85 ~30–50°C inférieures → catalyseur met un peu plus longtemps à monter en température.

**Conséquences pratiques :** DTC P0420/P0430 peuvent apparaître les 200–500 premiers km → normaux, disparaissent après stabilisation des LTFT.

### ✏️ Avant / Après

**Verdict : ne pas modifier si les catalyseurs sont présents.** Les deux bins (stock et E85 de référence) ont des valeurs identiques.

**`ip_fac_eff_iga_ch_cold_opm_1` @ 0x4A444** (Valvetronic) — retard °CRK :

```
STOCK = E85 BIN (inchangé) :
TPS% →      10    15    20    25    30    35    40    45    50    65
 704 rpm :  5.98  6.95  6.51  6.51  5.98  5.50  4.58  4.49  4.36  4.40
 896 rpm :  5.98  7.30  6.86  6.69  6.25  5.54  4.93  4.58  4.49  4.49
1056 rpm :  6.12  7.48  7.48  6.25  5.76  5.24  5.02  4.88  4.62  4.62
1216 rpm :  6.20  7.48  7.26  5.98  5.50  5.02  4.80  4.80  4.75  4.75
1504 rpm :  5.28  7.35  6.25  5.50  5.24  4.75  4.75  4.75  4.53  4.75
1760 rpm :  4.22  7.00  5.63  4.88  4.71  4.75  4.80  4.80  4.80  4.88
2016 rpm :  3.83  5.94  4.44  4.14  3.92  4.53  4.58  4.80  4.80  4.66
2304 rpm :  3.39  4.71  3.08  3.30  2.99  3.65  4.31  4.58  4.80  4.40
2848 rpm :  3.30  4.14  2.82  3.30  3.78  4.40  4.71  5.10  5.37  5.06
5888 rpm :  5.76  4.84  4.53  4.88  4.84  5.72  6.03  5.02  4.40  4.75
```

**`ip_fac_eff_iga_ch_cold_opm_2` @ 0x4A4A8** (papillonné) — retard °CRK :

```
STOCK = E85 BIN (inchangé) :
TPS% →      10    20    25    30    35    40    50    65    85   105
 512 rpm :  3.30  4.71  5.63  6.12  6.42  6.34  5.50  5.50  5.50  5.50
 704 rpm :  3.30  4.14  4.93  5.46  5.76  5.76  5.50  4.84  4.84  4.84
 896 rpm :  3.78  4.00  4.05  4.09  4.71  5.54  6.20  4.75  4.75  4.75
1088 rpm :  3.74  3.92  3.96  4.05  4.40  5.54  5.50  5.32  5.32  5.32
1344 rpm :  3.34  3.96  4.14  4.27  4.49  4.75  5.06  5.15  5.15  5.15
1760 rpm :  2.77  3.56  3.96  4.22  4.49  4.62  4.75  4.58  4.40  4.40
2304 rpm :  2.64  4.09  4.62  4.84  5.10  5.24  5.41  4.97  4.88  5.06
3360 rpm :  3.17  4.44  4.97  5.15  5.32  5.02  5.37  5.19  4.53  4.53
4544 rpm :  4.00  4.80  5.24  5.50  5.63  5.72  5.81  5.85  4.71  4.71
5888 rpm :  3.92  4.75  5.19  5.76  5.81  5.68  5.46  5.68  6.03  6.07
```

> ⚠️ **Sans catalyseurs (dépollution supprimée) :** les deux tables doivent être **mises à zéro** (toutes cellules → raw 0). Le retard de chauffe n'a plus de raison d'être et pénalise inutilement l'allumage à froid. DTC P0420/P0430 devront être masqués séparément dans le bin.
