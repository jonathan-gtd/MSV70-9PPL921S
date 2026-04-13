# §8 — Warm-up lambda au ralenti : ip_fac_lamb_wup_is

**Paramètre :** `ip_fac_lamb_wup_is` @ 0x42788 — facteur warm-up spécifique au **ralenti** (idle speed)

| Structure | Axe X | Axe Y |
|---|---|---|
| 3×4, uint8, ×0.007813 | MAF : 65/100/200/300 mg/stk | RPM : 704/1216/1760 tr/min |

**Valeurs stock :**
```
MAF (mg/stk) →  65.0  100.0  200.0  300.0
 704 rpm :       1.00   1.00   1.00   1.00
1216 rpm :       1.00   1.00   1.00   1.00
1760 rpm :       1.00   1.00   1.00   1.00
```

**Distinction avec `ip_fac_lamb_wup` (§2.3) :** la table §2.3 couvre toute la phase de chauffe (tous régimes). `ip_fac_lamb_wup_is` est uniquement actif au **ralenti** pendant le warm-up.

**Modification suggérée (si ralenti instable après correction cranking) :**
```
MAF (mg/stk) →  65.0  100.0  200.0  300.0
 704 rpm :       1.05   1.03   1.00   1.00
1216 rpm :       1.03   1.02   1.00   1.00
1760 rpm :       1.00   1.00   1.00   1.00
```
> Encode en uint8 : 1.05 = raw 134, 1.03 = raw 132, 1.02 = raw 131.

**Verdict : modifier uniquement si** ralenti instable pendant warm-up malgré `ip_mff_cst_opm_*` et `ip_fac_lamb_wup` corrects.

---

