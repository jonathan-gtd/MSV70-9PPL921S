# 6BMOT — Calculateur moteur (DME)

🔴 Module `6BMOT` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🔴 **Module a ne pas coder.** Cette liste est fournie pour comprendre, pas pour modifier.

---

## Source

| | |
|---|---|
| Variante retenue | `MSV70.C02` |
| FSW dans cette variante | **2** |
| FSW toutes variantes confondues | 4 |
| Fichiers DATEN analyses | 4 |

Variantes disponibles : `MSV70.C01`, `MSV70.C02`, `MSV70.C03`, `MSV70.C04`

> La variante reellement installee sur le vehicule est identifiee par NCS Dummy
> a l'ouverture du `.TRC`. Si elle differe de celle retenue ici, la liste change.

---

## Les 2 fonctions codables

| FSW | Traduction |
|---|---|
| `OLULCODIERUNG` | **OLULCODIERUNG** |
| `VMAXCODIERUNG_MSV70` | **VMAXCODIERUNG** · **MSV70** |

---

Les fragments en **gras** n'ont pas de traduction etablie — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments entre `_underscores_` sont ambigus : leur sens depend du module.
