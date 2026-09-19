# CTUNE — CCC - tuner

🟠 Module `CTUNE` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Source

| | |
|---|---|
| Variante retenue | `CCC_ANT.C05` |
| Fonctions codables | **4** |
| Avec valeurs extraites | 4 (100 %) |
| Avec explication rédigée | 0 |
| Fichiers DATEN analysés | 9 |

Variantes disponibles : `CCC_ANT.C04`, `CCC_ANT.C05`, `CCC_ANT.C06`, `CCC_ANT.C07`, `CCC_ANT.C08`, `CCC_ANT.C09`, `CCC_ANT.C0A`, `CCC_ANT.C0B`, `CCC_ANT.C0C`

> La variante réellement installée est identifiée par NCS Dummy à l'ouverture du `.TRC`.
> Si elle diffère de celle retenue ici, la liste change.

---

## Les 4 fonctions codables

| FSW | Traduction | Valeurs possibles | Explication |
|---|---|---|---|
| `LAENDERVARIANTE_TUNER_ANT` | variante pays · **TUNER** · **ANT** | `no_area` · `ece` · `us` · `japan` · `oceanien` |  |
| `RDS_AF_FUNKTION_CCC` | **RDS** · **AF** · fonction · systeme multimedia CCC | `rds_aus` · `rds_ein_af_aus` · `rds_ein_af_manuell` · `rds_ein_af_automatisch` |  |
| `REGIONALISIERUNG` | **REGIONALISIERUNG** | `reg_aus` · `reg_ein` |  |
| `TP_FUNKTION_ANT` | **TP** · fonction · **ANT** | `nicht_aktiv` · `tp` |  |

---

**Traduction** : décomposition du nom allemand — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments en **gras** n'ont pas de traduction établie, ceux entre `_underscores_` sont ambigus.

**Valeurs possibles** : extraites des DATEN. `aktiv`/`nicht_aktiv` = interrupteur,
`wert_xx` = valeur numérique, autres = choix nommés.

**Explication** : rédigée uniquement pour les fonctions identifiées avec certitude.
Une case vide ne veut pas dire que la fonction est sans intérêt — seulement que son effet
n'a pas été établi de source sûre.
