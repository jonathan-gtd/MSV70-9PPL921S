# PDC — Radars de recul

🟢 Module `PDC` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟢 Module sans fonction de securite. Sauvegarde du `.TRC` quand meme.

---

## Identification

| | |
|---|---|
| Variante | **`PDC_E87.C05`** |
| Identifiée par | **lecture NCS Expert sur le véhicule** |
| Fonctions retenues | **48** |
| Vérification | 100 % des fonctions dans les blocs du `.TRC` |
| Avec valeurs | 47 |
| Avec explication rédigée | 0 |
| Blocs d'adresses du TRC | 10 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 48 fonctions codables

| FSW | Traduction | Valeurs | Explication |
|---|---|---|---|
| `ABSCHALTWEG` | course de coupure | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `ABSCHALT_GESCHWINDIGKEIT` | coupure · vitesse | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `ABSTAND_DAUERTON_KOPPLUNG` | distance · son continu · couplage | `nicht_aktiv` · `aktiv` |  |
| `ABSTAND_ECKE_MIT_VO` | distance · coin · avec · bon de commande (?) | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `ABSTAND_ECK_MIT_HI` | distance · coin · avec · haut | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `ABSTAND_MIT_MIT_HI` | distance · avec · avec · haut | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `ABSTAND_MIT_MIT_VO` | distance · avec · avec · bon de commande (?) | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `AHK_OFFSET_FRONT` | attelage · decalage · avant | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `AHK_OFFSET_HINTEN` | attelage · decalage · arriere | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `AKUSTIK_RECHTS_LINKS` | acoustique · droite · gauche | `nicht_aktiv` · `aktiv` |  |
| `AKUSTISCHE_FEHLERMELDUNG` | sonore · message d'erreur | `nicht_aktiv` · `aktiv` |  |
| `ANHAENGERKUPPLUNG` | attelage | `nicht_aktiv` · `aktiv` |  |
| `ANZAHL_WANDLER` | nombre · convertisseur | — |  |
| `DAUERTON` | son continu | `nicht_aktiv` · `aktiv` |  |
| `DAUERTON_BEREICH_HINTEN` | son continu · plage · arriere | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `DAUERTON_BEREICH_VORNE` | son continu · plage · avant | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `EINSCHALTMELDUNG_IMMER` | message de mise en marche · toujours | `nicht_aktiv` · `aktiv` |  |
| `EINSCHALTVEROEGERUNG_ZEIT` | temporisation d'activation · duree | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `EINSCHALTVERZOEGERUNG` | temporisation d'activation | `nicht_aktiv` · `aktiv` |  |
| `FAHRZEUG_STILLSTAND` | vehicule · arret | `nicht_aktiv` · `aktiv` |  |
| `FREQUENZ_TONGEBER_HINTEN` | frequence · avertisseur sonore · arriere | `wert_01` |  |
| `FREQUENZ_TONGEBER_VORNE` | frequence · avertisseur sonore · avant | `wert_01` · `wert_02` |  |
| `LAUTSTAERKE_HINTEN` | volume sonore · arriere | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+50)* |  |
| `LAUTSTAERKE_VORN` | volume sonore · avant | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `MESSBEREICHE_ECKE_VORNE` | plages de mesure · coin · avant | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `MESSBEREICHE_MITTE_HINTEN` | plages de mesure · milieu · arriere | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `MESSBEREICHE_MITTE_VORNE` | plages de mesure · milieu · avant | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `MESSBEREICH_ECKE_HINTEN` | plage de mesure · coin · arriere | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `OFFSET_ECKE_HINTEN` | decalage · coin · arriere | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `OFFSET_ECKE_VORNE` | decalage · coin · avant | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `OFFSET_MITTE_VORNE` | decalage · milieu · avant | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `OFFSET_MIT_HINTEN` | decalage · avec · arriere | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `PDC_DEAKTIVIERUNG` | radars · desactivation | `nicht_aktiv` · `aktiv` |  |
| `PDC_SCHALTER` | radars · interrupteur | `nicht_aktiv` · `aktiv` |  |
| `SC_FLAG_DT_FRONT` | _SC_ · indicateur · donnees (?) · avant | `nicht_aktiv` · `aktiv` · `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` … *(+63)* |  |
| `SC_FLAG_DT_REAR` | _SC_ · indicateur · donnees (?) · arriere | `nicht_aktiv` · `aktiv` · `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` … *(+52)* |  |
| `SC_FLAG_LP_FRONT` | _SC_ · indicateur · haut-parleur (?) · avant | `nicht_aktiv` · `aktiv` |  |
| `SC_FLAG_LP_REAR` | _SC_ · indicateur · haut-parleur (?) · arriere | `nicht_aktiv` · `aktiv` |  |
| `SC_FLAG_OE_FRONT` | _SC_ · indicateur · ouverture (?) · avant | `nicht_aktiv` · `aktiv` |  |
| `SC_FLAG_OE_REAR` | _SC_ · indicateur · ouverture (?) · arriere | `nicht_aktiv` · `aktiv` |  |
| `SC_FLAG_WD_FRONT` | _SC_ · indicateur · chien de garde (?) · avant | `nicht_aktiv` · `aktiv` |  |
| `SC_FLAG_WD_REAR` | _SC_ · indicateur · chien de garde (?) · arriere | `nicht_aktiv` · `aktiv` |  |
| `STILLSTAND_ZEIT` | arret · duree | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `TONAUSGABE_CAN` | sortie audio · bus CAN | `nicht_aktiv` · `aktiv` |  |
| `TONAUSGABE_LSP` | sortie audio · haut-parleur | `nicht_aktiv` · `aktiv` |  |
| `TON_ANZEIGEDAUER` | son · duree d'affichage | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* |  |
| `WANDERKENNUNG` | detection de derive | `nicht_aktiv` · `aktiv` |  |
| `WANDLER_STOERUNG` | convertisseur · perturbation | `nicht_aktiv` · `aktiv` |  |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Explication** — rédigée seulement quand l'effet est établi de source sûre.
