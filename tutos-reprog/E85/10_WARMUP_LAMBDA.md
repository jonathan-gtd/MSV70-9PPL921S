# §8 — Warm-up lambda au ralenti : ip_fac_lamb_wup_is

**Paramètre :** `ip_fac_lamb_wup_is` @ 0x42788 — facteur warm-up spécifique au **ralenti** (idle speed)

| Structure | Axe X | Axe Y |
|---|---|---|
| 3×4, uint8, ×0.007813 | MAF : 65/100/200/300 mg/stk | RPM : 704/1216/1760 tr/min |

**Distinction avec `ip_fac_lamb_wup` ([coldstart §3](../../coldstart/03_LAMBDA_CHAUFFE.md)) :** la table coldstart couvre toute la phase de chauffe (tous régimes, axes MAF × RPM). `ip_fac_lamb_wup_is` est uniquement actif au **ralenti** pendant le warm-up.

### ✏️ Avant / Après

```
STOCK :
MAF (mg/stk) →  65.0  100.0  200.0  300.0
 704 rpm :       1.00   1.00   1.00   1.00
1216 rpm :       1.00   1.00   1.00   1.00
1760 rpm :       1.00   1.00   1.00   1.00

E85 BIN (inchangé — même valeurs) :
MAF (mg/stk) →  65.0  100.0  200.0  300.0
 704 rpm :       1.00   1.00   1.00   1.00
1216 rpm :       1.00   1.00   1.00   1.00
1760 rpm :       1.00   1.00   1.00   1.00

OBJECTIF E85 (si ralenti instable) :
MAF (mg/stk) →  65.0  100.0  200.0  300.0
 704 rpm :       1.05   1.03   1.00   1.00
1216 rpm :       1.03   1.02   1.00   1.00
1760 rpm :       1.00   1.00   1.00   1.00
```
> Encode en uint8 : 1.05 = raw 134, 1.03 = raw 132, 1.02 = raw 131.

**Verdict : modifier uniquement si** ralenti instable pendant warm-up malgré `ip_mff_cst_opm_*` et `ip_fac_lamb_wup` (coldstart) corrects.

---

