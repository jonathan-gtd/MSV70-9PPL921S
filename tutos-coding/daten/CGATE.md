# CGATE — CCC - passerelle

🟠 Module `CGATE` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Source

| | |
|---|---|
| Variante retenue | `CCC_GW.C03` |
| FSW dans cette variante | **8** |
| FSW toutes variantes confondues | 8 |
| Fichiers DATEN analyses | 7 |

Variantes disponibles : `CCC_GW.C02`, `CCC_GW.C03`, `CCC_GW.C04`, `CCC_GW.C05`, `CCC_GW.C06`, `CCC_GW.C07`, `CCC_GW.C08`

> La variante reellement installee sur le vehicule est identifiee par NCS Dummy
> a l'ouverture du `.TRC`. Si elle differe de celle retenue ici, la liste change.

---

## Les 8 fonctions codables

| FSW | Traduction |
|---|---|
| `AUDIO_SYSTEM` | audio · **SYSTEM** |
| `DIM_MODE` | **DIM** · mode |
| `PDC` | radars |
| `PDC_SIGNAL` | radars · **SIGNAL** |
| `PDC_TYPE` | radars · type |
| `REVERSE_GONG` | **REVERSE** · gong |
| `RING_CCC` | **RING** · systeme multimedia CCC |
| `SPEED_MODE` | vitesse · mode |

---

Les fragments en **gras** n'ont pas de traduction etablie — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments entre `_underscores_` sont ambigus : leur sens depend du module.
