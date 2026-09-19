# CHOST — CCC - unite centrale

🟠 Module `CHOST` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Identification

| | |
|---|---|
| Variante | **`CCC_HST.C06`** |
| Identifiée par | **lecture NCS Expert sur le véhicule** |
| Fonctions retenues | **19** |
| Vérification | 100 % des fonctions dans les blocs du `.TRC` |
| Avec valeurs | 18 |
| Avec explication rédigée | 0 |
| Blocs d'adresses du TRC | 5 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 19 fonctions codables

| FSW | Traduction | Valeurs | Explication |
|---|---|---|---|
| `AEREA_CODE_DVD` | zone · code · DVD | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `AMBANDS_HST` | bandes AM · unite centrale | `nicht_aktiv` · `aktiv` |  |
| `BAUART` | type de construction | `limousine` · `touring` · `coupe` · `cabrio` · `compact` · `sicherheitsfahrzeug` |  |
| `BAUREIHE` | serie | `e65` · `rr1` · `e66` · `e67` · `e60` · `e63` · `e64` · `e61` … *(+15)* |  |
| `CDC` | [variante CDC] | `nicht_aktiv` · `aktiv` |  |
| `CID_CCC` | ecran central · systeme multimedia CCC | `cid_high` · `cid_low` |  |
| `CID_EXT_CCC_H` | ecran central · externe · systeme multimedia CCC · _H_ | `size_400` · `size_640` · `iso` · `shifted_jis` · `china` · `default` |  |
| `CID_TYPE` | ecran central · type | `static` · `flap` |  |
| `CODIERUNG_ERFOLGT` | codage · effectue | `wert_01` · `codierung_erfolgt` |  |
| `DIGITAL_AUDIO_SOURCE` | numerique · audio · source | — |  |
| `DISPLAY_TYPE` | affichage · type | `siemens_8_8_fixed_old` · `siemens_8_8_fixed_new` · `borg_optrex_8_8_fixed` · `alpine_sharp_8_8_fixed` · `siemens_6_5_flap_old` · `siemens_6_5_flap_new` · `alpine_sharp_6_5_fixed` · `invalid` |  |
| `FOND_HIGH_HST` | places arriere · haut · unite centrale | `nicht_aktiv` · `aktiv` |  |
| `FOND_LOW` | places arriere · bas | `nicht_aktiv` · `aktiv` |  |
| `LAENDERVARIANTE_GT` | variante pays · type (?) | `ece` · `us` · `australien` · `japan` |  |
| `MMC` | carte memoire | `nicht_aktiv` · `aktiv` |  |
| `PDC` | radars | `nicht_aktiv` · `aktiv` |  |
| `PDC_TYPE` | radars · type | `rear` · `full` |  |
| `TRF` | trafic | `nicht_aktiv` · `aktiv` |  |
| `WEATHERBAND_HST` | bande meteo · unite centrale | `nicht_aktiv` · `aktiv` |  |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Explication** — rédigée seulement quand l'effet est établi de source sûre.
