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
| Signification (EN) | 4 |
| Explication (FR) | 4 |
| Blocs d'adresses du TRC | 5 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 4 fonctions codables

| FSW | Décomposition | Valeurs | Signification (EN) | Explication (FR) |
|---|---|---|---|---|
| `KATENTFALLCODIERUNG` | codage sans catalyseur | `wert_00` · `wert_01` | Catalytic converter elimination [wert_00: with catalytic converter, wert_01: without catalytic converter] (changes may require STAT_BSZ_WERT=0) | Suppression du catalyseur [wert_00 : avec catalyseur, wert_01 : sans catalyseur] (peut necessiter STAT_BSZ_WERT=0) |
| `MILANSTEUERUNG` | commande de temoin (?) | `wert_01` · `wert_02` | Check engine light (MIL) control [wert_01: enabled, wert_02: disabled] (changes may require STAT_BSZ_WERT=0) | Pilotage du temoin moteur (MIL) [wert_01 : actif, wert_02 : inactif] (peut necessiter STAT_BSZ_WERT=0) |
| `OLULCODIERUNG` | codage | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` | Low/high-power class coding [wert_00: low-power class, wert_01: medium-power class, wert_02: high-power class] (changes may require STAT_BSZ_WERT=0) | Codage de la classe de puissance [wert_00 : basse, wert_01 : moyenne, wert_02 : haute] (peut necessiter STAT_BSZ_WERT=0) |
| `VMAXCODIERUNG_MSV70` | codage de la vitesse maximale · calculateur moteur MSV70 | `wert_00` · `wert_01` · `wert_02` · `wert_03` | Speed governor coding (changes may require STAT_BSZ_WERT=0) | Codage du limiteur de vitesse maximale (peut necessiter STAT_BSZ_WERT=0) |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Signification (EN)** — description issue de la base communautaire NCS Dummy, verbatim.

**Explication (FR)** — traduction française. Un tiret signifie que la ligne n'est pas
encore traduite. Les notations d'unités sont converties : `[durée s = valeur / 10]`.
