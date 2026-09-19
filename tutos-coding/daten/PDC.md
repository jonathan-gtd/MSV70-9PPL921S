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
| Signification (EN) | 27 |
| Explication (FR) | 27 |
| Blocs d'adresses du TRC | 10 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 48 fonctions codables

| FSW | Décomposition | Valeurs | Signification (EN) | Explication (FR) |
|---|---|---|---|---|
| `ABSCHALTWEG` | course de coupure | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* | Switch-off distance | Distance de coupure |
| `ABSCHALT_GESCHWINDIGKEIT` | coupure · vitesse | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* | Switch-off speed | Vitesse de coupure |
| `ABSTAND_DAUERTON_KOPPLUNG` | distance · son continu · couplage | `nicht_aktiv` · `aktiv` | Distance continuous tone coupling | Distance de declenchement du son continu |
| `ABSTAND_ECKE_MIT_VO` | distance · coin · avec · bon de commande (?) | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* | Distance corner center front | Distance des capteurs d'angle avant |
| `ABSTAND_ECK_MIT_HI` | distance · coin · avec · haut | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* | Distance corner center rear | Distance des capteurs d'angle arriere |
| `ABSTAND_MIT_MIT_HI` | distance · avec · avec · haut | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* | Distance center center rear | Distance des capteurs centraux arriere |
| `ABSTAND_MIT_MIT_VO` | distance · avec · avec · bon de commande (?) | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* | Distance center center front | Distance des capteurs centraux avant |
| `AHK_OFFSET_FRONT` | attelage · decalage · avant | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* | Trailer tow hitch offset front | Decalage de la boule d'attelage, avant |
| `AHK_OFFSET_HINTEN` | attelage · decalage · arriere | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* | Trailer tow hitch offset rear | Decalage de la boule d'attelage, arriere |
| `AKUSTIK_RECHTS_LINKS` | acoustique · droite · gauche | `nicht_aktiv` · `aktiv` | Acoustic right-left | Signal sonore droite-gauche |
| `AKUSTISCHE_FEHLERMELDUNG` | sonore · message d'erreur | `nicht_aktiv` · `aktiv` | Acoustic fault notification | Signalisation sonore des defauts |
| `ANHAENGERKUPPLUNG` | attelage | `nicht_aktiv` · `aktiv` | Trailer tow hitch installed | Attelage installe |
| `ANZAHL_WANDLER` | nombre · convertisseur | — | Number of sensors | Nombre de capteurs |
| `DAUERTON` | son continu | `nicht_aktiv` · `aktiv` | Continuous tone | Son continu |
| `DAUERTON_BEREICH_HINTEN` | son continu · plage · arriere | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* | Continuous tone measuring range rear | Plage de mesure du son continu a l'arriere |
| `DAUERTON_BEREICH_VORNE` | son continu · plage · avant | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* | Continuous tone measuring range front | Plage de mesure du son continu a l'avant |
| `EINSCHALTMELDUNG_IMMER` | message de mise en marche · toujours | `nicht_aktiv` · `aktiv` | Switch-on notification each time | Signal de mise en marche a chaque activation |
| `EINSCHALTVEROEGERUNG_ZEIT` | temporisation d'activation · duree | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* | Switch-on delay time | Duree de temporisation d'activation |
| `EINSCHALTVERZOEGERUNG` | temporisation d'activation | `nicht_aktiv` · `aktiv` | Switch-on delay | Temporisation d'activation |
| `FAHRZEUG_STILLSTAND` | vehicule · arret | `nicht_aktiv` · `aktiv` | — | — |
| `FREQUENZ_TONGEBER_HINTEN` | frequence · avertisseur sonore · arriere | `wert_01` | — | — |
| `FREQUENZ_TONGEBER_VORNE` | frequence · avertisseur sonore · avant | `wert_01` · `wert_02` | — | — |
| `LAUTSTAERKE_HINTEN` | volume sonore · arriere | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+50)* | Sound volume rear | Volume sonore arriere |
| `LAUTSTAERKE_VORN` | volume sonore · avant | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* | Sound volume front | Volume sonore avant |
| `MESSBEREICHE_ECKE_VORNE` | plages de mesure · coin · avant | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* | Measuring ranges front corners | Plages de mesure des capteurs d'angle avant |
| `MESSBEREICHE_MITTE_HINTEN` | plages de mesure · milieu · arriere | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* | Measuring ranges rear center | Plages de mesure des capteurs centraux arriere |
| `MESSBEREICHE_MITTE_VORNE` | plages de mesure · milieu · avant | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* | Measuring ranges front center | Plages de mesure des capteurs centraux avant |
| `MESSBEREICH_ECKE_HINTEN` | plage de mesure · coin · arriere | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* | Measuring range rear corners | Plage de mesure des capteurs d'angle arriere |
| `OFFSET_ECKE_HINTEN` | decalage · coin · arriere | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* | — | — |
| `OFFSET_ECKE_VORNE` | decalage · coin · avant | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* | — | — |
| `OFFSET_MITTE_VORNE` | decalage · milieu · avant | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* | — | — |
| `OFFSET_MIT_HINTEN` | decalage · avec · arriere | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* | — | — |
| `PDC_DEAKTIVIERUNG` | radars · desactivation | `nicht_aktiv` · `aktiv` | Park distance control (PDC) deactivation | Desactivation des radars de stationnement (PDC) |
| `PDC_SCHALTER` | radars · interrupteur | `nicht_aktiv` · `aktiv` | Park distance control (PDC) button | Bouton des radars de stationnement (PDC) |
| `SC_FLAG_DT_FRONT` | _SC_ · indicateur · donnees (?) · avant | `nicht_aktiv` · `aktiv` · `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` … *(+63)* | — | — |
| `SC_FLAG_DT_REAR` | _SC_ · indicateur · donnees (?) · arriere | `nicht_aktiv` · `aktiv` · `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` … *(+52)* | — | — |
| `SC_FLAG_LP_FRONT` | _SC_ · indicateur · haut-parleur (?) · avant | `nicht_aktiv` · `aktiv` | — | — |
| `SC_FLAG_LP_REAR` | _SC_ · indicateur · haut-parleur (?) · arriere | `nicht_aktiv` · `aktiv` | — | — |
| `SC_FLAG_OE_FRONT` | _SC_ · indicateur · ouverture (?) · avant | `nicht_aktiv` · `aktiv` | — | — |
| `SC_FLAG_OE_REAR` | _SC_ · indicateur · ouverture (?) · arriere | `nicht_aktiv` · `aktiv` | — | — |
| `SC_FLAG_WD_FRONT` | _SC_ · indicateur · chien de garde (?) · avant | `nicht_aktiv` · `aktiv` | — | — |
| `SC_FLAG_WD_REAR` | _SC_ · indicateur · chien de garde (?) · arriere | `nicht_aktiv` · `aktiv` | — | — |
| `STILLSTAND_ZEIT` | arret · duree | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* | — | — |
| `TONAUSGABE_CAN` | sortie audio · bus CAN | `nicht_aktiv` · `aktiv` | — | — |
| `TONAUSGABE_LSP` | sortie audio · haut-parleur | `nicht_aktiv` · `aktiv` | — | — |
| `TON_ANZEIGEDAUER` | son · duree d'affichage | `e81` · `e82` · `e82_lci` · `e82_m` · `e84` · `e87_mue` · `e87` · `e87_sport` … *(+48)* | — | — |
| `WANDERKENNUNG` | detection de derive | `nicht_aktiv` · `aktiv` | — | — |
| `WANDLER_STOERUNG` | convertisseur · perturbation | `nicht_aktiv` · `aktiv` | — | — |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Signification (EN)** — description issue de la base communautaire NCS Dummy, verbatim.

**Explication (FR)** — traduction française. Un tiret signifie que la ligne n'est pas
encore traduite. Les notations d'unités sont converties : `[durée s = valeur / 10]`.
