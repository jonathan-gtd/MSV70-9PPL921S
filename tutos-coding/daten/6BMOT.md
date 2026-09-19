# 6BMOT — Calculateur moteur (DME)

🔴 Module `6BMOT` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🔴 **Module a ne pas coder.** Liste fournie pour comprendre, pas pour modifier.

---

## Identification

| | |
|---|---|
| Variante | **`MSV70.C03`** |
| Identifiée par | **lecture NCS Expert sur le véhicule** |
| Fonctions retenues | **4** |
| Vérification | 100 % des fonctions dans les blocs du `.TRC` |
| Avec valeurs | 4 |
| Avec explication rédigée | 0 |
| Blocs d'adresses du TRC | 5 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 4 fonctions codables

| FSW | Traduction | Valeurs | Explication |
|---|---|---|---|
| `KATENTFALLCODIERUNG` | codage sans catalyseur | `wert_00` · `wert_01` |  |
| `MILANSTEUERUNG` | commande de temoin (?) | `wert_01` · `wert_02` |  |
| `OLULCODIERUNG` | codage | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `VMAXCODIERUNG_MSV70` | codage de la vitesse maximale · calculateur moteur MSV70 | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Explication** — rédigée seulement quand l'effet est établi de source sûre.
