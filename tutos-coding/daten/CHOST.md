# CHOST — CCC - unite centrale

🟠 Module `CHOST` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Source

| | |
|---|---|
| Variante retenue | `CCC_HST.C02` |
| Fonctions codables | **18** |
| Avec valeurs extraites | 17 (94 %) |
| Avec explication rédigée | 0 |
| Fichiers DATEN analysés | 7 |

Variantes disponibles : `CCC_HST.C01`, `CCC_HST.C02`, `CCC_HST.C03`, `CCC_HST.C04`, `CCC_HST.C05`, `CCC_HST.C06`, `CCC_HST.C07`

> La variante réellement installée est identifiée par NCS Dummy à l'ouverture du `.TRC`.
> Si elle diffère de celle retenue ici, la liste change.

---

## Les 18 fonctions codables

| FSW | Traduction | Valeurs possibles | Explication |
|---|---|---|---|
| `AEREA_CODE_DVD` | **AEREA** · **CODE** · DVD | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `AMBANDS_HST` | **AMBANDS** · **HST** | `nicht_aktiv` · `aktiv` |  |
| `BAUART` | type de construction | `limousine` · `touring` · `coupe` · `cabrio` · `compact` · `sicherheitsfahrzeug` |  |
| `BAUREIHE` | **BAUREIHE** | `e65` · `rr1` · `e66` · `e67` · `e60` · `e63` · `e64` · `e61` … *(+11)* |  |
| `CDC` | [variante CDC] | `nicht_aktiv` · `aktiv` |  |
| `CID_CCC` | ecran central · systeme multimedia CCC | `cid_high` · `cid_low` |  |
| `CID_EXT_CCC_H` | ecran central · **EXT** · systeme multimedia CCC · _H_ | `size_400` · `size_640` |  |
| `CID_TYPE` | ecran central · type | `static` · `flap` |  |
| `CODIERUNG_ERFOLGT` | codage · effectue | `wert_01` · `codierung_erfolgt` |  |
| `DIGITAL_AUDIO_SOURCE` | numerique · audio · **SOURCE** | — |  |
| `FOND_HIGH_HST` | places arriere · haut · **HST** | `nicht_aktiv` · `aktiv` |  |
| `FOND_LOW` | places arriere · bas | `nicht_aktiv` · `aktiv` |  |
| `LAENDERVARIANTE_GT` | variante pays · **GT** | `ece` · `us` · `australien` · `japan` |  |
| `MMC` | **MMC** | `nicht_aktiv` · `aktiv` |  |
| `PDC` | radars | `nicht_aktiv` · `aktiv` |  |
| `PDC_TYPE` | radars · type | `rear` · `full` |  |
| `TRF` | **TRF** | `nicht_aktiv` · `aktiv` |  |
| `WEATHERBAND_HST` | **WEATHERBAND** · **HST** | `nicht_aktiv` · `aktiv` |  |

---

**Traduction** : décomposition du nom allemand — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments en **gras** n'ont pas de traduction établie, ceux entre `_underscores_` sont ambigus.

**Valeurs possibles** : extraites des DATEN. `aktiv`/`nicht_aktiv` = interrupteur,
`wert_xx` = valeur numérique, autres = choix nommés.

**Explication** : rédigée uniquement pour les fonctions identifiées avec certitude.
Une case vide ne veut pas dire que la fonction est sans intérêt — seulement que son effet
n'a pas été établi de source sûre.
