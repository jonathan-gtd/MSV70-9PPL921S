# §16 — Phasage injection EOI : ip_eoi_1_bas

| Paramètre | Adresse | Structure |
|---|---|---|
| `ip_eoi_1_bas` | 0x4E914 | 8×6, uint16, ×0.375, f(TI × RPM), °CRK |

**Valeurs stock (°CRK après PMH) :**
```
        TI (ms) →   0.4    2.0    3.7    6.0   12.8   14.0
 512 rpm :          213.0  214.9  225.8  234.0  197.6  199.9
 704 rpm :          216.8  217.5  228.0  234.8  199.1  199.9
 992 rpm :          223.1  225.4  234.0  241.9  171.8  169.5
1504 rpm :          232.1  232.1  235.1  240.8   90.0   85.1
2016 rpm :          238.5  237.4  235.9  238.1   82.1   66.4
3008 rpm :          238.9  241.1  236.6  230.6  102.8   79.5
4512 rpm :          235.1  240.8  240.0  209.3   97.5   70.1
6496 rpm :          225.8  230.3  225.4  205.5   97.5   70.5
```

Sur E85, la durée d'injection augmente de +45% → injection commence plus tôt pour le même angle EOI → reste dans la fenêtre acceptable pour les injecteurs stock sur toute la plage de TI concernée.

**Verdict : ne pas modifier avec les injecteurs stock.** Si injecteurs haute débit remplacés (TI réduite) : recalculer EOI — modification avancée hors scope.

---

