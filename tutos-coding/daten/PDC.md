# PDC — Radars de recul

🟢 Module `PDC` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟢 Module sans fonction de securite. Sauvegarde du `.TRC` quand meme.

---

## Identification

| | |
|---|---|
| Variante | **`PDC_E87.C05`** |
| Identifiée par | recoupement avec `PDC.TRC` lu sur le véhicule |
| Fonctions retenues | **48** |
| Écartées (hors blocs du TRC) | 0 |
| Avec valeurs | 47 |
| Avec explication rédigée | 0 |
| Blocs d'adresses du TRC | 12 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 48 fonctions codables

| FSW | Traduction | Valeurs | Explication |
|---|---|---|---|
| `ABSCHALTWEG` | **ABSCHALTWEG** | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `ABSCHALT_GESCHWINDIGKEIT` | coupure · **GESCHWINDIGKEIT** | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `ABSTAND_DAUERTON_KOPPLUNG` | distance · son continu · **KOPPLUNG** | `nicht_aktiv` · `aktiv` |  |
| `ABSTAND_ECKE_MIT_VO` | distance · coin · avec · **VO** | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `ABSTAND_ECK_MIT_HI` | distance · **ECK** · avec · haut | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `ABSTAND_MIT_MIT_HI` | distance · avec · avec · haut | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `ABSTAND_MIT_MIT_VO` | distance · avec · avec · **VO** | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `AHK_OFFSET_FRONT` | attelage · decalage · avant | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `AHK_OFFSET_HINTEN` | attelage · decalage · arriere | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `AKUSTIK_RECHTS_LINKS` | acoustique · droite · gauche | `nicht_aktiv` · `aktiv` |  |
| `AKUSTISCHE_FEHLERMELDUNG` | sonore · message d'erreur | `nicht_aktiv` · `aktiv` |  |
| `ANHAENGERKUPPLUNG` | **ANHAENGERKUPPLUNG** | `nicht_aktiv` · `aktiv` |  |
| `ANZAHL_WANDLER` | nombre · convertisseur | — |  |
| `DAUERTON` | son continu | `nicht_aktiv` · `aktiv` |  |
| `DAUERTON_BEREICH_HINTEN` | son continu · plage · arriere | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `DAUERTON_BEREICH_VORNE` | son continu · plage · avant | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `EINSCHALTMELDUNG_IMMER` | **EINSCHALTMELDUNG** · toujours | `nicht_aktiv` · `aktiv` |  |
| `EINSCHALTVEROEGERUNG_ZEIT` | **EINSCHALTVEROEGERUNG** · duree | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `EINSCHALTVERZOEGERUNG` | **EINSCHALTVERZOEGERUNG** | `nicht_aktiv` · `aktiv` |  |
| `FAHRZEUG_STILLSTAND` | **FAHRZEUG** · arret | `nicht_aktiv` · `aktiv` |  |
| `FREQUENZ_TONGEBER_HINTEN` | frequence · avertisseur sonore · arriere | `wert_01` |  |
| `FREQUENZ_TONGEBER_VORNE` | frequence · avertisseur sonore · avant | `wert_01` · `wert_02` |  |
| `LAUTSTAERKE_HINTEN` | volume sonore · arriere | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+50)* |  |
| `LAUTSTAERKE_VORN` | volume sonore · **VORN** | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `MESSBEREICHE_ECKE_VORNE` | plages de mesure · coin · avant | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `MESSBEREICHE_MITTE_HINTEN` | plages de mesure · milieu · arriere | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `MESSBEREICHE_MITTE_VORNE` | plages de mesure · milieu · avant | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `MESSBEREICH_ECKE_HINTEN` | **MESSBEREICH** · coin · arriere | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `OFFSET_ECKE_HINTEN` | decalage · coin · arriere | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `OFFSET_ECKE_VORNE` | decalage · coin · avant | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `OFFSET_MITTE_VORNE` | decalage · milieu · avant | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `OFFSET_MIT_HINTEN` | decalage · avec · arriere | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `PDC_DEAKTIVIERUNG` | radars · **DEAKTIVIERUNG** | `nicht_aktiv` · `aktiv` |  |
| `PDC_SCHALTER` | radars · interrupteur | `nicht_aktiv` · `aktiv` |  |
| `SC_FLAG_DT_FRONT` | _SC_ · indicateur · **DT** · avant | `nicht_aktiv` · `aktiv` · `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` … *(+63)* |  |
| `SC_FLAG_DT_REAR` | _SC_ · indicateur · **DT** · arriere | `nicht_aktiv` · `aktiv` · `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` … *(+52)* |  |
| `SC_FLAG_LP_FRONT` | _SC_ · indicateur · **LP** · avant | `nicht_aktiv` · `aktiv` |  |
| `SC_FLAG_LP_REAR` | _SC_ · indicateur · **LP** · arriere | `nicht_aktiv` · `aktiv` |  |
| `SC_FLAG_OE_FRONT` | _SC_ · indicateur · **OE** · avant | `nicht_aktiv` · `aktiv` |  |
| `SC_FLAG_OE_REAR` | _SC_ · indicateur · **OE** · arriere | `nicht_aktiv` · `aktiv` |  |
| `SC_FLAG_WD_FRONT` | _SC_ · indicateur · **WD** · avant | `nicht_aktiv` · `aktiv` |  |
| `SC_FLAG_WD_REAR` | _SC_ · indicateur · **WD** · arriere | `nicht_aktiv` · `aktiv` |  |
| `STILLSTAND_ZEIT` | arret · duree | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `TONAUSGABE_CAN` | sortie audio · bus CAN | `nicht_aktiv` · `aktiv` |  |
| `TONAUSGABE_LSP` | sortie audio · **LSP** | `nicht_aktiv` · `aktiv` |  |
| `TON_ANZEIGEDAUER` | son · duree d'affichage | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `WANDERKENNUNG` | **WANDERKENNUNG** | `nicht_aktiv` · `aktiv` |  |
| `WANDLER_STOERUNG` | convertisseur · **STOERUNG** | `nicht_aktiv` · `aktiv` |  |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Explication** — rédigée seulement quand l'effet est établi de source sûre.
