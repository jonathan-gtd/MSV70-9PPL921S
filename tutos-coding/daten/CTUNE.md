# CTUNE — CCC - tuner

🟠 Module `CTUNE` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Identification

| | |
|---|---|
| Variante | **`CCC_ANT.C07`** |
| Identifiée par | recoupement avec `CTUNE.TRC` lu sur le véhicule |
| Fonctions retenues | **5** |
| Écartées (hors blocs du TRC) | 0 |
| Avec valeurs | 5 |
| Avec explication rédigée | 0 |
| Blocs d'adresses du TRC | 1 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 5 fonctions codables

| FSW | Traduction | Valeurs | Explication |
|---|---|---|---|
| `IBOC_CCC` | **IBOC** · systeme multimedia CCC | `nicht_aktiv` · `aktiv` |  |
| `LAENDERVARIANTE_TUNER_ANT` | variante pays · tuner · antenne | `no_area` · `ece` · `us` · `japan` · `oceanien` |  |
| `RDS_AF_FUNKTION_CCC` | **RDS** · **AF** · fonction · systeme multimedia CCC | `rds_aus` · `rds_ein_af_aus` · `rds_ein_af_manuell` · `rds_ein_af_automatisch` |  |
| `REGIONALISIERUNG` | **REGIONALISIERUNG** | `reg_aus` · `reg_ein` |  |
| `TP_FUNKTION_ANT` | **TP** · fonction · antenne | `nicht_aktiv` · `tp` |  |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Explication** — rédigée seulement quand l'effet est établi de source sûre.
