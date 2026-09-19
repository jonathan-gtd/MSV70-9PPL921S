# 6BMOT — Calculateur moteur (DME)

🔴 Module `6BMOT` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🔴 **Module a ne pas coder.** Liste fournie pour comprendre, pas pour modifier.

---

## Source

| | |
|---|---|
| Variante retenue | `MSV70.C02` |
| Fonctions codables | **2** |
| Avec valeurs extraites | 2 (100 %) |
| Avec explication rédigée | 0 |
| Fichiers DATEN analysés | 4 |

Variantes disponibles : `MSV70.C01`, `MSV70.C02`, `MSV70.C03`, `MSV70.C04`

> La variante réellement installée est identifiée par NCS Dummy à l'ouverture du `.TRC`.
> Si elle diffère de celle retenue ici, la liste change.

---

## Les 2 fonctions codables

| FSW | Traduction | Valeurs possibles | Explication |
|---|---|---|---|
| `OLULCODIERUNG` | **OLULCODIERUNG** | `wert_00` · `wert_01` · `wert_02` |  |
| `VMAXCODIERUNG_MSV70` | **VMAXCODIERUNG** · **MSV70** | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |

---

**Traduction** : décomposition du nom allemand — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments en **gras** n'ont pas de traduction établie, ceux entre `_underscores_` sont ambigus.

**Valeurs possibles** : extraites des DATEN. `aktiv`/`nicht_aktiv` = interrupteur,
`wert_xx` = valeur numérique, autres = choix nommés.

**Explication** : rédigée uniquement pour les fonctions identifiées avec certitude.
Une case vide ne veut pas dire que la fonction est sans intérêt — seulement que son effet
n'a pas été établi de source sûre.
