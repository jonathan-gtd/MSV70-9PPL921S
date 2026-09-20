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
| Signification (EN) | 19 |
| Explication (FR) | 19 |
| Blocs d'adresses du TRC | 5 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 19 fonctions codables

| FSW | Décomposition | Valeurs | Signification (EN) | Explication (FR) |
|---|---|---|---|---|
| `AEREA_CODE_DVD` | zone · code · DVD | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` | Dvd region code | Code region DVD |
| `AMBANDS_HST` | bandes AM · unite centrale | `nicht_aktiv` · `aktiv` | AM radio band in source selection | Bande AM proposee dans le choix des sources |
| `BAUART` | type de construction | `limousine` · `touring` · `coupe` · `cabrio` · `compact` · `sicherheitsfahrzeug` | Model variant | Variante de modele |
| `BAUREIHE` | serie | `e65` · `rr1` · `e66` · `e67` · `e60` · `e63` · `e64` · `e61` … *(+15)* | Chassis | Serie du vehicule |
| `CDC` | [variante CDC] | `nicht_aktiv` · `aktiv` | CD changer | Changeur de CD |
| `CID_CCC` | ecran central · systeme multimedia CCC | `cid_high` · `cid_low` | Central information display (CID) | Ecran central (CID) |
| `CID_EXT_CCC_H` | ecran central · externe · systeme multimedia CCC · _H_ | `size_400` · `size_640` · `iso` · `shifted_jis` · `china` · `default` | Central information display (CID) horizontal extended display size | Largeur d'affichage etendue de l'ecran central (CID) |
| `CID_TYPE` | ecran central · type | `static` · `flap` | Central information display (CID) type | Type d'ecran central (CID) |
| `CODIERUNG_ERFOLGT` | codage · effectue | `wert_01` · `codierung_erfolgt` | Coding completed | Codage termine |
| `DIGITAL_AUDIO_SOURCE` | numerique · audio · source | — | Digital audio source | Source audio numerique |
| `DISPLAY_TYPE` | affichage · type | `siemens_8_8_fixed_old` · `siemens_8_8_fixed_new` · `borg_optrex_8_8_fixed` · `alpine_sharp_8_8_fixed` · `siemens_6_5_flap_old` · `siemens_6_5_flap_new` · `alpine_sharp_6_5_fixed` · `invalid` | Display type | Type d'affichage |
| `FOND_HIGH_HST` | places arriere · haut · unite centrale | `nicht_aktiv` · `aktiv` | Rear monitor | Ecran arriere |
| `FOND_LOW` | places arriere · bas | `nicht_aktiv` · `aktiv` | Rear headphones | Casques arriere |
| `LAENDERVARIANTE_GT` | variante pays · type (?) | `ece` · `us` · `australien` · `japan` | Country variant navigation | Variante pays de la navigation |
| `MMC` | carte memoire | `nicht_aktiv` · `aktiv` | Multimedia changer (MMC) | Changeur multimedia (MMC) |
| `PDC` | radars | `nicht_aktiv` · `aktiv` | Park distance control (PDC) | Radars de stationnement (PDC) |
| `PDC_TYPE` | radars · type | `rear` · `full` | Park distance control (PDC) type | Type de radars de stationnement (PDC) |
| `TRF` | trafic | `nicht_aktiv` · `aktiv` | Traffic channel in source selection | Canal trafic propose dans le choix des sources |
| `WEATHERBAND_HST` | bande meteo · unite centrale | `nicht_aktiv` · `aktiv` | Weather band in source selection | Bande meteo proposee dans le choix des sources |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Signification (EN)** — description issue de la base communautaire NCS Dummy, verbatim.

**Explication (FR)** — traduction française ou interprétation explicitement marquée
« Déduction du libellé ». Cette dernière reste à confirmer ; elle ne documente pas
les valeurs ni un effet testé sur le véhicule. Un tiret signifie qu’aucune explication
n’est établie. Voir [la méthode et les sources](TRADUCTIONS.md). Les notations d'unités sont converties : `[durée s = valeur / 10]`.
