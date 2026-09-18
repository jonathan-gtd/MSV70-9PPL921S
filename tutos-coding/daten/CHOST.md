# CHOST — CCC - unite centrale

🟠 Module `CHOST` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Source

| | |
|---|---|
| Variante retenue | `CCC_HST.C02` |
| FSW dans cette variante | **18** |
| FSW toutes variantes confondues | 19 |
| Fichiers DATEN analyses | 7 |

Variantes disponibles : `CCC_HST.C01`, `CCC_HST.C02`, `CCC_HST.C03`, `CCC_HST.C04`, `CCC_HST.C05`, `CCC_HST.C06`, `CCC_HST.C07`

> La variante reellement installee sur le vehicule est identifiee par NCS Dummy
> a l'ouverture du `.TRC`. Si elle differe de celle retenue ici, la liste change.

---

## Les 18 fonctions codables

| FSW | Traduction |
|---|---|
| `AEREA_CODE_DVD` | **AEREA** · **CODE** · DVD |
| `AMBANDS_HST` | **AMBANDS** · **HST** |
| `BAUART` | type de construction |
| `BAUREIHE` | **BAUREIHE** |
| `CDC` | [index variante CDC] |
| `CID_CCC` | ecran central · systeme multimedia CCC |
| `CID_EXT_CCC_H` | ecran central · **EXT** · systeme multimedia CCC · _H_ |
| `CID_TYPE` | ecran central · type |
| `CODIERUNG_ERFOLGT` | codage · effectue |
| `DIGITAL_AUDIO_SOURCE` | numerique · audio · **SOURCE** |
| `FOND_HIGH_HST` | places arriere · haut · **HST** |
| `FOND_LOW` | places arriere · bas |
| `LAENDERVARIANTE_GT` | variante pays · **GT** |
| `MMC` | **MMC** |
| `PDC` | radars |
| `PDC_TYPE` | radars · type |
| `TRF` | **TRF** |
| `WEATHERBAND_HST` | **WEATHERBAND** · **HST** |

---

Les fragments en **gras** n'ont pas de traduction etablie — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments entre `_underscores_` sont ambigus : leur sens depend du module.
