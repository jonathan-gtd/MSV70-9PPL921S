# 6BMOT — Calculateur moteur (DME)

🔴 Module `6BMOT` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🔴 **Module a ne pas coder.** Liste fournie pour comprendre, pas pour modifier.

---

## Identification

| | |
|---|---|
| Variante | **`MSV70.C01`** |
| Identifiée par | recoupement avec `6BMOT.TRC` lu sur le véhicule |
| Fonctions retenues | **4** |
| Écartées (hors blocs du TRC) | 0 |
| Avec valeurs | 4 |
| Avec explication rédigée | 0 |
| Blocs d'adresses du TRC | 5 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 4 fonctions codables

| FSW | Traduction | Valeurs | Explication |
|---|---|---|---|
| `KATENTFALLCODIERUNG` | **KATENTFALLCODIERUNG** | `wert_00` · `wert_01` |  |
| `MILANSTEUERUNG` | **MILANSTEUERUNG** | `wert_01` · `wert_02` |  |
| `OLULCODIERUNG` | **OLULCODIERUNG** | `wert_00` · `wert_01` · `wert_02` |  |
| `VMAXCODIERUNG_MSV70` | **VMAXCODIERUNG** · **MSV70** | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Explication** — rédigée seulement quand l'effet est établi de source sûre.
