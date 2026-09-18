# CTUNE — CCC - tuner

🟠 Module `CTUNE` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Source

| | |
|---|---|
| Variante retenue | `CCC_ANT.C05` |
| FSW dans cette variante | **4** |
| FSW toutes variantes confondues | 5 |
| Fichiers DATEN analyses | 9 |

Variantes disponibles : `CCC_ANT.C04`, `CCC_ANT.C05`, `CCC_ANT.C06`, `CCC_ANT.C07`, `CCC_ANT.C08`, `CCC_ANT.C09`, `CCC_ANT.C0A`, `CCC_ANT.C0B`, `CCC_ANT.C0C`

> La variante reellement installee sur le vehicule est identifiee par NCS Dummy
> a l'ouverture du `.TRC`. Si elle differe de celle retenue ici, la liste change.

---

## Les 4 fonctions codables

| FSW | Traduction |
|---|---|
| `LAENDERVARIANTE_TUNER_ANT` | variante pays · **TUNER** · **ANT** |
| `RDS_AF_FUNKTION_CCC` | **RDS** · **AF** · fonction · systeme multimedia CCC |
| `REGIONALISIERUNG` | **REGIONALISIERUNG** |
| `TP_FUNKTION_ANT` | **TP** · fonction · **ANT** |

---

Les fragments en **gras** n'ont pas de traduction etablie — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments entre `_underscores_` sont ambigus : leur sens depend du module.
