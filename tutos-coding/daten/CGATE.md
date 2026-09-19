# CGATE — CCC - passerelle

🟠 Module `CGATE` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Identification

| | |
|---|---|
| Variante | **`CCC_GW.C02`** |
| Identifiée par | **lecture NCS Expert sur le véhicule** |
| Fonctions retenues | **7** |
| Vérification | 100 % des fonctions dans les blocs du `.TRC` |
| Avec valeurs | 7 |
| Avec explication rédigée | 0 |
| Blocs d'adresses du TRC | 4 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 7 fonctions codables

| FSW | Traduction | Valeurs | Explication |
|---|---|---|---|
| `AUDIO_SYSTEM` | audio · systeme | `e60_stereo` · `e60_hifi` · `e60_top_hifi` · `pl2_stereo` · `pl2_hifi` · `pl2_top_hifi` · `e70_stereo` · `e70_hifi` … *(+37)* |  |
| `PDC` | radars | `nicht_aktiv` · `aktiv` |  |
| `PDC_SIGNAL` | radars · signal | `pdc_no` · `pdc_rear` · `pdc_front` · `pdc_front_rear` |  |
| `PDC_TYPE` | radars · type | `rear` · `full` |  |
| `REVERSE_GONG` | inversion · gong | `nicht_aktiv` · `aktiv` |  |
| `RING_CCC` | anneau · systeme multimedia CCC | `aktiv` · `nicht_aktiv` |  |
| `SPEED_MODE` | vitesse · mode | `speed_mode_intern` · `speed_mode_extern` |  |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Explication** — rédigée seulement quand l'effet est établi de source sûre.
