# CGATE — CCC - passerelle

🟠 Module `CGATE` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Source

| | |
|---|---|
| Variante retenue | `CCC_GW.C03` |
| Fonctions codables | **8** |
| Avec valeurs extraites | 8 (100 %) |
| Avec explication rédigée | 0 |
| Fichiers DATEN analysés | 7 |

Variantes disponibles : `CCC_GW.C02`, `CCC_GW.C03`, `CCC_GW.C04`, `CCC_GW.C05`, `CCC_GW.C06`, `CCC_GW.C07`, `CCC_GW.C08`

> La variante réellement installée est identifiée par NCS Dummy à l'ouverture du `.TRC`.
> Si elle diffère de celle retenue ici, la liste change.

---

## Les 8 fonctions codables

| FSW | Traduction | Valeurs possibles | Explication |
|---|---|---|---|
| `AUDIO_SYSTEM` | audio · **SYSTEM** | `e60_stereo` · `e60_hifi` · `e60_top_hifi` · `pl2_stereo` · `pl2_hifi` · `pl2_top_hifi` · `e70_stereo` · `e70_hifi` … *(+39)* |  |
| `DIM_MODE` | **DIM** · mode | `standard` · `use_last` |  |
| `PDC` | radars | `nicht_aktiv` · `aktiv` |  |
| `PDC_SIGNAL` | radars · **SIGNAL** | `pdc_no` · `pdc_rear` · `pdc_front` · `pdc_front_rear` |  |
| `PDC_TYPE` | radars · type | `rear` · `full` |  |
| `REVERSE_GONG` | **REVERSE** · gong | `nicht_aktiv` · `aktiv` |  |
| `RING_CCC` | **RING** · systeme multimedia CCC | `aktiv` · `nicht_aktiv` |  |
| `SPEED_MODE` | vitesse · mode | `speed_mode_intern` · `speed_mode_extern` |  |

---

**Traduction** : décomposition du nom allemand — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments en **gras** n'ont pas de traduction établie, ceux entre `_underscores_` sont ambigus.

**Valeurs possibles** : extraites des DATEN. `aktiv`/`nicht_aktiv` = interrupteur,
`wert_xx` = valeur numérique, autres = choix nommés.

**Explication** : rédigée uniquement pour les fonctions identifiées avec certitude.
Une case vide ne veut pas dire que la fonction est sans intérêt — seulement que son effet
n'a pas été établi de source sûre.
