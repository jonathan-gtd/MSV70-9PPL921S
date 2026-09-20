# CAS — Acces et antidemarrage

🔴 Module `CAS` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🔴 **Module a ne pas coder.** Liste fournie pour comprendre, pas pour modifier.

---

## Identification

| | |
|---|---|
| Variante | **`CAS3.C06`** |
| Identifiée par | **lecture NCS Expert sur le véhicule** |
| Fonctions retenues | **229** |
| Vérification | 100 % des fonctions dans les blocs du `.TRC` |
| Avec valeurs | 227 |
| Signification (EN) | 176 |
| Explication (FR) | 185 |
| Blocs d'adresses du TRC | 10 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 229 fonctions codables

| FSW | Décomposition | Valeurs | Signification (EN) | Explication (FR) |
|---|---|---|---|---|
| `ABZUGSPERRE_PLOCK_C6` | blocage de retrait · verrouillage · identifiant interne (?) | `aktiv` · `nicht_aktiv` | Engine switch-off only with transmission park/neutral signal | Coupure moteur uniquement si la boite signale P ou N |
| `ANLASSERSCHUTZ_3SEK` | protection du demarreur · 3 s | `aktiv` · `nicht_aktiv` | Starter protection (switch off starter after 3 seconds without engine turning) | Protection du demarreur (coupure apres 3 secondes sans rotation moteur) |
| `ANLASSERSPERRE_KPS` | blocage du demarreur · capteur de position (?) | `aktiv` · `nicht_aktiv` | Engine start only with clutch pedal depressed | Demarrage moteur uniquement pedale d'embrayage enfoncee |
| `ANLASSERSPERRE_P` | blocage du demarreur · P | `aktiv` · `nicht_aktiv` | Engine start only with transmission in park/neutral | Demarrage moteur uniquement boite sur P ou N |
| `ANLASSSPERRE_BLS` | blocage du demarreur · contacteur de feu stop | `aktiv` · `nicht_aktiv` | Engine start only with brake pedal depressed | Autorise le demarrage sans appuyer sur la pedale de frein. |
| `ANLASSSPERRE_KLR` | blocage du demarreur · borne R (accessoires) | `aktiv` · `nicht_aktiv` | New engine start (when stalled) only after switching off ignition | Nouveau demarrage apres calage uniquement apres coupure du contact |
| `ANSTEUERZEIT` | duree de commande | `wert_01` | Control time | Duree de commande |
| `AUSWERTUNG_B_DRUCK` | evaluation · B · pression | `aktiv` · `nicht_aktiv` | Evaluate brake pressure signal when brake light switch defective (linked to [BREMSE_DRUCK]) | Exploiter le signal de pression de frein si le contacteur de feu stop est defaillant (lie a BREMSE_DRUCK) |
| `AUSWERTUNG_P_CAN` | evaluation · P · bus CAN | `aktiv` · `nicht_aktiv` | Evaluate transmission park/neutral signal from CAN-bus | Exploiter le signal P/N de la boite via le bus CAN |
| `AUSWERTUNG_P_HW` | evaluation · P · materiel (?) | `aktiv` · `nicht_aktiv` | Evaluate transmission park/neutral signal from electronic transmission control (EGS) | Exploiter le signal P/N transmis par le calculateur de boite (EGS) |
| `AUT_OFFNEN_HK_IN_ENTLPOS` | automatique · ouverture · hayon · entree · position de decharge | `aktiv` · `nicht_aktiv` | Automatically open trunk lid after reaching convertible comfort loading position (linked to [FBD_LOAD_POS_HK_HK]) | Ouverture automatique du coffre une fois la position de chargement cabriolet atteinte (lie a FBD_LOAD_POS_HK_HK) |
| `BAUART` | type de construction | `limousine` · `coupe` · `touring` · `cabrio` · `roadster` · `compact` · `sicherheitsfahrzeug` · `light_protection` | Model variant | Variante de modele |
| `BAUREIHE` | serie | `e65` · `e66` · `e67` · `e60` · `e61` · `e63` · `e64` · `e72` … *(+16)* | Chassis | Serie du vehicule |
| `BEDINGUNG_CC_10KM` | condition · Check Control · 10 km | `aktiv` · `nicht_aktiv` | — | — |
| `BOS_DATEN_ANFRAGEN` | vehicule prioritaire (?) · donnees · requetes | `aktiv` · `nicht_aktiv` | — | — |
| `BREMSE_DRUCK` | frein · pression | `wert_01` | Brake pressure to detect depressed brake pedal when brake light switch defective [pressure_bar=data/10] (linked to [AUSWERTUNG_B_DRUCK]) | Pression de frein servant a detecter l'appui pedale si le contacteur est defaillant [pression bar = valeur / 10] (lie a AUSWERTUNG_B_DRUCK) |
| `CAN_MEHRFACH_SICHERN` | bus CAN · multiple · verrouillage | `aktiv` · `nicht_aktiv` | Send CAN-bus lock signal multiple times | Emettre plusieurs fois le signal de verrouillage sur le bus CAN |
| `CBS_VERSION` | entretien selon besoin (CBS) · version | `wert_01` · `wert_02` · `wert_03` · `dynamik_2` | Condition based service (CBS) version | Version de l'entretien selon besoin (CBS) |
| `CCM_SEND_INVALID` | message Check Control · emission · invalide | `aktiv` · `nicht_aktiv` | Check control notification for invalid CAN-bus signal from door contacts | Message Check Control en cas de signal CAN invalide des contacteurs de porte |
| `CHIPCARD` | carte a puce | `aktiv` · `nicht_aktiv` | Enable transponder data | Activer les donnees du transpondeur |
| `CLASS_PWR` | classe · alimentation | `keine` · `klasse_1` · `klasse_2` · `invalid` · `e92_m` | Power class | Classe de puissance |
| `CLT_TASTE` | climatisation (?) · touche | `aktiv` · `nicht_aktiv` | Center lock button | Bouton de verrouillage central |
| `CRASH_BESCHLEUNIGUNG` | choc · acceleration | `wert_01` · `wert_02` | — | Déduction du libellé : Paramètre d’accélération utilisé pour la détection d’un choc ; seuil et unité non établis ici. |
| `CRASH_SCHWERE` | choc · gravite | `wert_01` · `wert_02` | — | Déduction du libellé : Paramètre relatif à la gravité d’un choc ; la correspondance des valeurs reste à vérifier. |
| `DELAY_KL50L` | delai · borne 50L | `wert_01` · `wert_02` · `wert_03` · `wert_04` | Start delay for engines with Valvetronic variable valve timing [time_ms=data*5] | Temporisation de demarrage pour moteurs Valvetronic [duree ms = valeur x 5] |
| `DELAY_KLR_AUS` | delai · borne R (accessoires) · arret | `wert_01` · `wert_02` | Accessories switch-off delay (prevents false ) [time_ms=data*5] | Temporisation de coupure des accessoires [duree ms = valeur x 5] |
| `EE_TRSP_CCM_0CD` | memoire EEPROM · transport · message Check Control · identifiant (?) | `aktiv` · `nicht_aktiv` | — | — |
| `EE_TRSP_DEBUG` | memoire EEPROM · transport · deverminage | `aktiv` · `nicht_aktiv` | — | — |
| `EE_TRSP_PERM` | memoire EEPROM · transport · permanent | `aktiv` · `nicht_aktiv` | — | — |
| `EE_ZEIT_ABFR` | memoire EEPROM · duree · interrogation | `wert_01` | — | — |
| `EE_ZEIT_NLF` | memoire EEPROM · duree · feu arriere de brouillard (?) | `wert_01` | — | — |
| `EINSTIEGSHILFE_DEFAULT` | aide a l'acces · valeur par defaut | `aktiv` · `nicht_aktiv` | Default car/key memory for easy entry | Valeur par defaut de la memoire par cle pour l'aide a l'acces |
| `EINSTIEGSHILFE_OHNE_NR` | aide a l'acces · sans · numero | `aktiv` · `nicht_aktiv` | Unpersonalized car/key memory for easy entry | Memoire par cle non personnalisee pour l'aide a l'acces |
| `EINSTIEGSHILFE_PORT` | aide a l'acces · port | `aktiv` · `nicht_aktiv` | — | — |
| `EINSTIEGSHILFE_SENDER_1` | aide a l'acces · emetteur · 1 | `aktiv` · `nicht_aktiv` | Car/key memory 1 for easy entry | Memoire par cle 1 pour l'aide a l'acces |
| `EINSTIEGSHILFE_SENDER_2` | aide a l'acces · emetteur · 2 | `aktiv` · `nicht_aktiv` | Car/key memory 2 for easy entry | Memoire par cle 2 pour l'aide a l'acces |
| `EINSTIEGSHILFE_SENDER_3` | aide a l'acces · emetteur · 3 | `aktiv` · `nicht_aktiv` | Car/key memory 3 for easy entry | Memoire par cle 3 pour l'aide a l'acces |
| `EINSTIEGSHILFE_SENDER_4` | aide a l'acces · emetteur · 4 | `aktiv` · `nicht_aktiv` | Car/key memory 4 for easy entry | Memoire par cle 4 pour l'aide a l'acces |
| `ELV_AUT_FCT` | blocage de colonne de direction · automatique · fonction | `aktiv` · `nicht_aktiv` | Electrical steering lock (ELV) automatic function | Fonction automatique du blocage electrique de colonne (ELV) |
| `ELV_RESET_FCT` | blocage de colonne de direction · remise a zero · fonction | `aktiv` · `nicht_aktiv` | Electrical steering lock (ELV) reset function | Fonction de reinitialisation du blocage electrique de colonne (ELV) |
| `ELV_TESTER` | blocage de colonne de direction · testeur | `aktiv` · `nicht_aktiv` | Debug information for electrical steering lock (ELV) on CAN-bus | Informations de deverminage du blocage de colonne (ELV) sur le bus CAN |
| `ENABLE_CLT_NACH_CRASH` | activation · climatisation (?) · apres · choc | `aktiv` · `nicht_aktiv` | Center lock button can lock vehicle after crash | Le bouton de verrouillage central peut verrouiller apres un choc |
| `ERROR_EJECT` | erreur · ejection | `aktiv` · `nicht_aktiv` | Automatically eject faulty or invalid key | Ejection automatique d'une cle defectueuse ou invalide |
| `ER_KEYOUT_AUTOVR` | deverrouillage (?) · cle retiree · verrouillage automatique | `aktiv` · `nicht_aktiv` | Automatically unlock after removing ignition key (or switching off engine with comfort access) | Deverrouille les portes au retrait de la cle. |
| `ER_TANKKLAPPE_SER` | deverrouillage (?) · trappe a carburant · serie (?) | `aktiv` · `nicht_aktiv` | Fuel tank lid unlocks also with selective unlocking instead of only with global unlocking (linked to [ZV_SELEKTIV]) | La trappe a carburant se deverrouille aussi en deverrouillage selectif, et non seulement global (lie a ZV_SELEKTIV) |
| `ER_TANKKLAPPE_VR` | deverrouillage (?) · trappe a carburant · avant droit | `aktiv` · `nicht_aktiv` | — | — |
| `ES_EINMALIG_CLT` | unite de commande (?) · une seule fois · climatisation (?) | `aktiv` · `nicht_aktiv` | — | — |
| `EWS_EGS` | antidemarrage electronique · calculateur de boite | `nicht_aktiv` · `aktiv` | Electronic immobilizer system (EWS) for electronic transmission control (EGS) | Antidemarrage electronique (EWS) pour le calculateur de boite (EGS) |
| `EWS_FEHLER_MOTORLAUF` | antidemarrage electronique · erreur · moteur tournant | `nicht_aktiv` · `aktiv` | — | — |
| `EWS_NACHLAUF_KLR_AUS` | antidemarrage electronique · temporisation · borne R (accessoires) · arret | `aktiv` · `nicht_aktiv` | — | Déduction du libellé : Temporisation de l’antidémarrage électronique après coupure de la borne accessoires R. |
| `EWS_SCHRITTWEITE_2` | antidemarrage electronique · pas · 2 | `nicht_aktiv` · `aktiv` | — | — |
| `FBDIN_9_6_KBAUD` | entree telecommande · 9 · 6 · kBaud | `aktiv` · `nicht_aktiv` | Remote control receiver with 9600 baud communication rate | Recepteur de telecommande a 9600 bauds |
| `FBD_AKTIV_KEYIN` | telecommande · actif · cle inseree | `aktiv` · `nicht_aktiv` | Remote control buttons enabled with inserted key | Boutons de telecommande actifs cle inseree |
| `FBD_AKTIV_KL15AUS` | telecommande · actif · borne 15 coupee | `aktiv` · `nicht_aktiv` | Remote control buttons enabled with inserted key and ignition off | Boutons de telecommande actifs cle inseree et contact coupe |
| `FBD_DISFERTMODE` | telecommande · mode fin de chaine | `aktiv` · `nicht_aktiv` | Disable manufacturing mode for remote control | Desactiver le mode fin de chaine pour la telecommande |
| `FBD_DISTRANSMODE` | telecommande · mode transport | `aktiv` · `nicht_aktiv` | Disable transport mode for remote control | Desactiver le mode transport pour la telecommande |
| `FBD_DISWERKSTMODE` | telecommande · mode atelier | `aktiv` · `nicht_aktiv` | Disable workshop mode for remote control | Desactiver le mode atelier pour la telecommande |
| `FBD_HECKKLAPPE_TOTZEIT` | telecommande · hayon · temps mort | `wert_01` · `wert_02` | Time to hold remote control trunk lid button to release trunk lid [time_s=data/10] | Duree d'appui sur le bouton de coffre de la telecommande pour l'ouvrir [duree s = valeur / 10] |
| `FBD_HKL_NUR_ER` | telecommande · hayon · uniquement · deverrouillage (?) | `aktiv` · `nicht_aktiv` | Trunk lid release with remote control only when vehicle unlocked | Ouverture du coffre par telecommande uniquement vehicule deverrouille |
| `FBD_HSF_0` | telecommande · toit ouvrant · 0 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` · `wert_09` | — | — |
| `FBD_HSF_1` | telecommande · toit ouvrant · 1 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` · `wert_09` | — | — |
| `FBD_LOAD_POS_HK_HK` | telecommande · charge · position · hayon · hayon | `aktiv` · `nicht_aktiv` | Convertible comfort loading position (press remote control trunk lid button twice to partially open convertible top to facilitate better access to the luggage compartment) | Position de chargement cabriolet : deux appuis sur le bouton de coffre entrouvrent la capote pour degager l'acces au coffre |
| `FBD_MOTORLAUF` | telecommande · moteur tournant | `wert_01` | Time for engine to run after remote engine start [time_s=data/2] | Duree de fonctionnement du moteur apres demarrage a distance [duree s = valeur / 2] |
| `FBD_MOTORSTART` | telecommande · demarrage moteur | `wert_01` | Timeout for remote engine start until engine runs [time_s=data/2] | Delai d'attente du demarrage a distance avant abandon [duree s = valeur / 2] |
| `FBD_NO_LOAD_POS_C06` | telecommande · non · charge · position · [variante C06] | `aktiv` · `nicht_aktiv` | Disable convertible comfort loading position | Desactiver la position de chargement cabriolet |
| `FBD_PANIKDAUER` | telecommande · duree d'alarme panique | `wert_01` · `wert_02` · `wert_03` | Panic mode duration (when not aborted with remote control) [time_min=data] (linked to [SONDERTASTE_PANIKALARM]) | Duree du mode panique s'il n'est pas interrompu par la telecommande [duree min = valeur] (lie a SONDERTASTE_PANIKALARM) |
| `FBD_PANIK_TOTZEIT` | telecommande · mode panique · temps mort | `wert_01` | Time to hold remote control lock button to trigger panic mode [time_s=data/10] (linked to [SONDERTASTE_PANIKALARM]) | Duree d'appui sur le bouton de verrouillage pour declencher le mode panique [duree s = valeur / 10] (lie a SONDERTASTE_PANIKALARM) |
| `FBD_SERVICEFKT_AKTIV` | telecommande · fonction d'entretien · actif | `aktiv` · `nicht_aktiv` | Activate unpersonalized car/key memory for workshop mode after unlocking with key in door lock until next unlock with remote control | Activer la memoire par cle non personnalisee en mode atelier apres ouverture a la cle, jusqu'au prochain deverrouillage par telecommande |
| `FBD_TESTKEY_AKTIV` | telecommande · cle de test · actif | `aktiv` · `nicht_aktiv` | Special remote control for range measuring | Telecommande speciale pour mesure de portee |
| `FHM_CA_OHNE_EKS` | module leve-vitre · acces confort (?) · sans · protection antipincement | `aktiv` · `nicht_aktiv` | — | Déduction du libellé : Configuration des lève-vitres avec accès confort sans protection antipincement ; ce libellé décrit une fonction, pas une recommandation de codage. |
| `FHM_CA_OHNE_EKS_DELAY` | module leve-vitre · acces confort (?) · sans · protection antipincement · delai | `wert_01` · `wert_02` · `wert_03` · `wert_04` | — | Déduction du libellé : Délai associé à la configuration des lève-vitres avec accès confort sans protection antipincement. |
| `FH_ABSCHALTKRITERIUM` | leve-vitre · critere de coupure | `aktiv` · `nicht_aktiv` | Power windows behavior after ignition switched off and door opened [us: disabled, ece: enabled until door closed, aktiv: disabled, nicht_aktiv: enabled until door closed] | Comportement des leve-vitres apres coupure du contact et ouverture de porte [us : desactives, ece : actifs jusqu'a fermeture de la porte, aktiv : desactives, nicht_aktiv : actifs jusqu'a fermeture] |
| `FH_CABRIO_VERDECK_MODUL` | leve-vitre · cabriolet · capote · module | `aktiv` · `nicht_aktiv` | Power windows controlled by convertible top module | Leve-vitres pilotes par le module de capote |
| `FH_DISAKL15` | leve-vitre · desactivation borne 15 | `aktiv` · `nicht_aktiv` | Disable power windows without ignition switched on (stops movement immediatly after switching off accessories regardless of switch-off conditions) | Desactiver les leve-vitres sans contact mis (arret immediat du mouvement des la coupure des accessoires) |
| `FH_DISAKLR` | leve-vitre · desactivation (?) | `aktiv` · `nicht_aktiv` | Disable power windows without accessories switched on (stops movement immediatly after switching off accessories regardless of switch-off conditions) | Desactiver les leve-vitres sans accessoires actifs (arret immediat du mouvement des la coupure) |
| `FH_DISASHD_KL15` | leve-vitre · desactivation du toit ouvrant · borne 15 (contact) | `aktiv` · `nicht_aktiv` | Disable sliding/tilting sunroof without ignition switched on (stops movement immediatly after switching off accessories regardless of switch-off conditions) | Desactiver le toit ouvrant sans contact mis (arret immediat du mouvement des la coupure des accessoires) |
| `FH_DISCLSYDEFECT` | leve-vitre · defaut systeme (?) | `aktiv` · `nicht_aktiv` | Disable comfort closing with key in door lock for faulty door lock | Desactiver la fermeture confort par la cle en cas de serrure defaillante |
| `FH_DISFBDDEFCT` | leve-vitre · defaut de telecommande (?) | `aktiv` · `nicht_aktiv` | Disable comfort closing with remote control for faulty remote control | Desactiver la fermeture confort par telecommande en cas de telecommande defaillante |
| `FH_DISFERTMODE` | leve-vitre · mode fin de chaine | `aktiv` · `nicht_aktiv` | Disable manufacturing mode for power windows | Desactiver le mode fin de chaine pour les leve-vitres |
| `FH_DISTGDEFECT` | leve-vitre · defaut de distance (?) | `aktiv` · `nicht_aktiv` | Disable comfort closing with comfort access for faulty door handle electronics (TAGE) | Desactiver la fermeture confort par acces confort en cas d'electronique de poignee defaillante (TAGE) |
| `FH_DISTRANSMODE` | leve-vitre · mode transport | `aktiv` · `nicht_aktiv` | Disable transport mode for power windows | Desactiver le mode transport pour les leve-vitres |
| `FH_DISWERKSTMODE` | leve-vitre · mode atelier | `aktiv` · `nicht_aktiv` | Disable workshop mode for power windows | Desactiver le mode atelier pour les leve-vitres |
| `FH_ECEFIN` | leve-vitre · norme ECE Finlande | `aktiv` · `nicht_aktiv` | — | — |
| `FH_HINTEN` | leve-vitre · arriere | `aktiv` · `nicht_aktiv` | Rear power windows | Leve-vitres arriere |
| `FH_KINDERSICHERUNG` | leve-vitre · securite enfants | `aktiv` · `nicht_aktiv` | Power windows child safety button | Bouton de securite enfants des leve-vitres |
| `FH_KMFRTFBDTIM` | leve-vitre · temporisation confort telecommande | `aktiv` · `nicht_aktiv` | Maximum duration comfort opening/closing interruption for remote control [aktiv: use time from [MAX_KMFRT_UNTERB_FBD], nicht_aktiv: use time from [MAX_KMFRT_UNTERB]] | Duree maximale d'interruption de l'ouverture/fermeture confort par telecommande [aktiv : duree de MAX_KMFRT_UNTERB_FBD, nicht_aktiv : duree de MAX_KMFRT_UNTERB] |
| `FH_KOMFORTF_MIT_SUBST` | leve-vitre · fermeture confort · avec · substitution | `aktiv` · `nicht_aktiv` | Comfort opening/closing with substitution for convertible rear power window | Fermeture confort des vitres avec substitution. |
| `FH_RELEASE_CA_BASIC` | leve-vitre · relachement · acces confort (?) · base | `aktiv` · `nicht_aktiv` | — | — |
| `FH_RESETTIMEOUTKL50` | leve-vitre · delai de reinitialisation borne 50 | `aktiv` · `nicht_aktiv` | — | — |
| `FH_SCHLIESSEN_EINSTG` | leve-vitre · fermeture · acces | `aktiv` · `nicht_aktiv` | Easy entry opened driver power window closes when locking vehicle | La vitre conducteur ouverte par l'aide a l'acces se referme au verrouillage |
| `FH_SICHERHEITSFAHRZEUG` | leve-vitre · vehicule de securite | `aktiv` · `nicht_aktiv` | Power windows controlled by security vehicle module | Leve-vitres pilotes par le module vehicule de securite |
| `FH_TUERBED_EINSTG` | leve-vitre · commande de porte · acces | `aktiv` · `nicht_aktiv` | Open power window only after opening door (easy entry for frameless coup� door in narrow spaces) | Descente de vitre seulement apres ouverture de porte (acces facilite pour portes sans encadrement en espace etroit) |
| `FH_UMLSCH` | leve-vitre · commutation | `aktiv` · `nicht_aktiv` | — | — |
| `FH_USACANADA` | leve-vitre · USA / Canada | `aktiv` · `nicht_aktiv` | — | — |
| `FS_DATEN_ANFRAGEN` | siege conducteur (?) · donnees · requetes | `aktiv` · `nicht_aktiv` | — | — |
| `GESCHW_REQ_CC` | vitesse · requete · Check Control | `wert_01` · `wert_02` | — | — |
| `GESCHW_SPERRE_TOEHK` | vitesse · blocage · _TOEHK_ | `wert_01` | Threshold speed for disabling trunk lid opening [speed_km/h=data] | Vitesse au-dela de laquelle l'ouverture du coffre est bloquee [vitesse km/h = valeur] |
| `GETRIEBE_GAENGE` | boite de vitesses · rapports | — | Transmission gear count | Nombre de rapports de la boite |
| `GETRIEBE_TYP_C6` | boite de vitesses · type · identifiant interne (?) | `handschaltung` · `automatik` · `schrittschaltung` · `ssg_schaltung` · `dkg_schaltung` | Transmission type | Type de boite de vitesses |
| `HECKSCHEIBE_ENTR_SENDER_1` | lunette arriere · deverrouillage · emetteur · 1 | `aktiv` · `nicht_aktiv` | Car/key memory 1 for release touring rear window | Memoire par cle 1 pour l'ouverture de la lunette arriere de break |
| `HECKSCHEIBE_ENTR_SENDER_2` | lunette arriere · deverrouillage · emetteur · 2 | `aktiv` · `nicht_aktiv` | Car/key memory 2 for release touring rear window | Memoire par cle 2 pour l'ouverture de la lunette arriere de break |
| `HECKSCHEIBE_ENTR_SENDER_3` | lunette arriere · deverrouillage · emetteur · 3 | `aktiv` · `nicht_aktiv` | Car/key memory 3 for release touring rear window | Memoire par cle 3 pour l'ouverture de la lunette arriere de break |
| `HECKSCHEIBE_ENTR_SENDER_4` | lunette arriere · deverrouillage · emetteur · 4 | `aktiv` · `nicht_aktiv` | Car/key memory 4 for release touring rear window | Memoire par cle 4 pour l'ouverture de la lunette arriere de break |
| `HECKSCHEI_ENTR_OHNE_NR` | lunette arriere · deverrouillage · sans · numero | `aktiv` · `nicht_aktiv` | Unpersonalized car/key memory for release touring rear window | Memoire par cle non personnalisee pour l'ouverture de la lunette arriere de break |
| `HKAT_EO_HS_OHNE_NR` | catalyseur · _EO_ · lunette arriere (?) · sans · numero | `aktiv` · `nicht_aktiv` | Unpersonalized car/key memory for outside trunk lid button opens touring rear window instead of trunk lid | Memoire par cle non personnalisee : le bouton exterieur de coffre ouvre la lunette arriere au lieu du hayon |
| `HKAT_EO_HS_SCHL_NR01` | catalyseur · _EO_ · lunette arriere (?) · cle / fermeture · [NR01] | `aktiv` · `nicht_aktiv` | Car/key memory 1 for outside trunk lid button opens touring rear window instead of trunk lid | Memoire par cle 1 : le bouton exterieur de coffre ouvre la lunette arriere au lieu du hayon |
| `HKAT_EO_HS_SCHL_NR02` | catalyseur · _EO_ · lunette arriere (?) · cle / fermeture · [NR02] | `aktiv` · `nicht_aktiv` | Car/key memory 2 for outside trunk lid button opens touring rear window instead of trunk lid | Memoire par cle 2 : le bouton exterieur de coffre ouvre la lunette arriere au lieu du hayon |
| `HKAT_EO_HS_SCHL_NR03` | catalyseur · _EO_ · lunette arriere (?) · cle / fermeture · [NR03] | `aktiv` · `nicht_aktiv` | Car/key memory 3 for outside trunk lid button opens touring rear window instead of trunk lid | Memoire par cle 3 : le bouton exterieur de coffre ouvre la lunette arriere au lieu du hayon |
| `HKAT_EO_HS_SCHL_NR04` | catalyseur · _EO_ · lunette arriere (?) · cle / fermeture · [NR04] | `aktiv` · `nicht_aktiv` | Car/key memory 4 for outside trunk lid button opens touring rear window instead of trunk lid | Memoire par cle 4 : le bouton exterieur de coffre ouvre la lunette arriere au lieu du hayon |
| `HKL_LIFT_BEI_CA` | hayon · levee · lors de · acces confort (?) | `aktiv` · `nicht_aktiv` | Pressing remote control trunk lid button stops automatic trunk lid opening with comfort access | Un appui sur le bouton de coffre de la telecommande interrompt l'ouverture automatique par acces confort |
| `HK_ENABLE_OPEN` | hayon · activation · ouvert | `aktiv` · `nicht_aktiv` | Actuate trunk lid release motor even if already opened | Actionner le moteur d'ouverture de coffre meme coffre deja ouvert |
| `HK_GESPERRT_IN_GESICHERT` | hayon · bloque · entree · verrouille | `aktiv` · `nicht_aktiv` | Disable trunk lid release with remote control when locked (requires unlocking first) | Interdire l'ouverture du coffre par telecommande vehicule verrouille (deverrouillage prealable requis) |
| `HS_ENABLE_OPEN` | lunette arriere (?) · activation · ouvert | `aktiv` · `nicht_aktiv` | Actuate touring rear window release motor even if already opened | Actionner le moteur d'ouverture de lunette arriere meme deja ouverte |
| `KISITMRMAX` | duree maximale siege (?) | `wert_01` | — | — |
| `KL15HW01_ALLWAYS` | borne 15 · toujours | `aktiv` · `nicht_aktiv` | — | — |
| `KL30G_EIN_HKT_AUSSEN` | borne 30G · marche · bouton de hayon · exterieur | `aktiv` · `nicht_aktiv` | Automatically switch on terminal 30g (battery supply with time-based switch-off) after pressing outside trunk lid button | Mise sous tension automatique de la borne 30g apres appui sur le bouton exterieur de coffre |
| `KL30G_EIN_HKT_FBD` | borne 30G · marche · bouton de hayon · telecommande | `aktiv` · `nicht_aktiv` | Automatically switch on terminal 30g (battery supply with time-based switch-off) after pressing remote control trunk lid button | Mise sous tension automatique de la borne 30g apres appui sur le bouton de coffre de la telecommande |
| `KL30G_NACHLAUF_NACH_SZ1` | borne 30G · temporisation · apres · centre de commande 1 (?) | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` | — | — |
| `KL30G_TIMEOUT` | borne 30G · delai d'attente | `wert_01` · `wert_02` · `wert_03` | Automatically switch off terminal 30g (battery supply with time-based switch-off) after timeout [time_min=data] | Coupure automatique de la borne 30g apres temporisation [duree min = valeur] |
| `KLASSE_BATTERIE` | classe · batterie | `wert_00` | Battery type | Type de batterie |
| `KLR_AUS_TIMEOUT` | borne R (accessoires) · arret · delai d'attente | `wert_01` · `wert_02` · `wert_03` | Automatically switch off accessories after timeout [time_min=data] | Coupure automatique des accessoires apres temporisation [duree min = valeur] |
| `KLR_AUS_TKSITZ` | borne R (accessoires) · arret · siege climatise (?) | `aktiv` · `nicht_aktiv` | Automatically switch off accessories when opening door | Coupure automatique des accessoires a l'ouverture d'une porte |
| `KLR_EIN_GESCHW` | borne R (accessoires) · marche · vitesse | `nicht_aktiv` · `aktiv` | Automatically switch on accessories for rolling vehicle (engine not running) | Mise sous tension automatique des accessoires vehicule roulant moteur eteint |
| `KLSTRG_DISFERTMODE` | commande de climatisation · mode fin de chaine | `aktiv` · `nicht_aktiv` | Disable manufacturing mode for terminal control | Desactiver le mode fin de chaine pour la gestion des bornes |
| `KLSTRG_DISTRANSMODE` | commande de climatisation · mode transport | `aktiv` · `nicht_aktiv` | Disable transport mode for terminal control | Desactiver le mode transport pour la gestion des bornes |
| `KLSTRG_DISWERKSTMODE` | commande de climatisation · mode atelier | `aktiv` · `nicht_aktiv` | Disable workshop mode for terminal control | Desactiver le mode atelier pour la gestion des bornes |
| `KLSTRG_TESTER` | commande de climatisation · testeur | `aktiv` · `nicht_aktiv` | Debug information for terminal control on CAN-bus | Informations de deverminage de la gestion des bornes sur le bus CAN |
| `KMFRT_OEFFNEN` | confort · ouverture | `wert_01` | Time to hold remote control unlock button or key in door lock to start comfort opening [CAS: time_s=data/100, CAS2,CAS3: time_s=data/10] | Duree d'appui sur le bouton de deverrouillage ou de rotation de la cle pour lancer l'ouverture confort [duree s = valeur / 10 sur CAS2/CAS3] |
| `KMFRT_OEFF_HINTEN` | confort · ouverture · arriere | `wert_01` · `wert_02` | Delay comfort opening rear power windows [CAS: time_s=data/100, CAS2,CAS3: time_s=data/10] | Temporisation avant ouverture confort des vitres arriere [duree s = valeur / 10 sur CAS2/CAS3] |
| `KMFRT_OEFF_SHD` | confort · ouverture · toit ouvrant | `wert_01` · `wert_02` | Delay comfort opening sliding/tilting sunroof [CAS: time_s=data/100, CAS2,CAS3: time_s=data/10] | Temporisation avant ouverture confort du toit ouvrant [duree s = valeur / 10 sur CAS2/CAS3] |
| `KMFRT_SCHLIESSEN` | confort · fermeture | `wert_01` | Time to hold remote control lock button or key in door lock to start comfort closing [CAS: time_s=data/100, CAS2,CAS3: time_s=data/10] | Temporisation avant rabattement des retroviseurs : 00 immediat, 05 = 0,5 s, 0A = 1 s, 0F = 2,5 s. |
| `KMFRT_SCHL_VORNE` | confort · cle / fermeture · avant | `wert_01` | Delay comfort closing front power windows [CAS: time_s=data/100, CAS2,CAS3: time_s=data/10] | Temporisation avant fermeture confort des vitres avant [duree s = valeur / 10 sur CAS2/CAS3] |
| `KOMFORTOEFFNUNG` | ouverture confort | `aktiv` · `nicht_aktiv` | Comfort opening (power windows, sunroof or convertible top) | Ouverture confort (vitres, toit ouvrant ou capote) |
| `KOMFORTOEFFNUNG_FB` | ouverture confort · telecommande | `aktiv` · `nicht_aktiv` | Comfort opening with remote control (power windows, sunroof or convertible top) | Ouverture confort par telecommande (vitres, toit ouvrant ou capote) |
| `KOMFORTSCHLIESSUNG` | fermeture confort | `aktiv` · `nicht_aktiv` | Comfort closing (power windows, sunroof or convertible top) | Fermeture confort (vitres, toit ouvrant ou capote) |
| `KOMFORTSCHLIESSUNG_FB` | fermeture confort · telecommande | `aktiv` · `nicht_aktiv` | Comfort closing with remote control (power windows, sunroof or convertible top) | Fermeture des vitres et du toit par maintien de la telecommande. |
| `KOMFORTSCHLIESSUNG_PA` | fermeture confort · _PA_ | `aktiv` · `nicht_aktiv` | Comfort closing with comfort access (power windows, sunroof or convertible top) | Fermeture confort par le contacteur de porte. |
| `KOMFORTSTART` | demarrage confort | `aktiv` · `nicht_aktiv` | Comfort starting engine (press start/stop button only briefly to start engine) | Demarrage confort : une breve impulsion sur le bouton start/stop suffit |
| `KOMFORT_AUS_IN_50MS` | confort · arret · entree · 50 ms | `wert_02` · `wert_04` · `wert_05` · `wert_01` · `wert_03` | Time to hold start/stop button for automatically ejecting key [time_ms=data*50] (linked to [KOMFORT_EJECT]) | Duree d'appui sur le bouton start/stop pour ejecter automatiquement la cle [duree ms = valeur x 50] (lie a KOMFORT_EJECT) |
| `KOMFORT_EIN` | confort · marche | `aktiv` · `nicht_aktiv` | Automatically switch on accessories after inserting valid key | Mise sous tension automatique des accessoires a l'insertion d'une cle valide |
| `KOMFORT_EJECT` | confort · ejection | `aktiv` · `nicht_aktiv` | Automatically eject key after switching off ignition or accessories by holding start/stop button (linked to [KOMFORT_AUS_IN_50MS]) | Ejection automatique de la cle du neiman. |
| `KOMFORT_SCHL_SHD` | confort · cle / fermeture · toit ouvrant | `wert_01` · `wert_02` | Delay comfort closing sliding/tilting sunroof [CAS: time_s=data/100, CAS2,CAS3: time_s=data/10] | Temporisation avant fermeture confort du toit ouvrant [duree s = valeur / 10 sur CAS2/CAS3] |
| `LAENDERVARIANTE_BNDB` | variante pays · bande B | `ece` · `us` · `deutschland` · `gb` · `japan` · `australien` · `korea` · `brasilien` … *(+13)* | Country variant | Variante pays |
| `MAX_KMFRT_UNTERB` | maximum · confort · interruption | `wert_01` | Maximum duration comfort opening/closing interruption for comfort access and key in door lock [CAS: time_s=data/100, CAS2,CAS3: time_s=data/10] | Duree maximale d'interruption de l'ouverture/fermeture confort par acces confort ou cle en serrure [duree s = valeur / 10 sur CAS2/CAS3] |
| `MAX_KMFRT_UNTERB_FBD` | maximum · confort · interruption · telecommande | `wert_01` | Maximum duration comfort opening/closing interruption for remote control [CAS: time_s=data/100, CAS2,CAS3: time_s=data/10] | Duree maximale d'interruption de l'ouverture/fermeture confort par telecommande [duree s = valeur / 10 sur CAS2/CAS3] |
| `MFS_KEY_PLGD` | volant multifonction · cle · diagnostic (?) | `aktiv` · `nicht_aktiv` | Disable remote engine start with inserted key | Interdire le demarrage a distance si la cle est inseree |
| `MFS_MIT_TON` | volant multifonction · avec · son | `aktiv` · `nicht_aktiv` | Remote engine start with alarm sound | Demarrage a distance accompagne d'un signal sonore d'alarme |
| `MIND_KMFRT_NACH_UNTERB` | minimum · confort · apres · interruption | `wert_01` | Time to hold remote control unlock button or key in door lock to continue comfort opening/closing after interruption [CAS: time_s=data/100, CAS2,CAS3: time_s=data/10] | Duree d'appui pour reprendre l'ouverture/fermeture confort apres interruption [duree s = valeur / 10 sur CAS2/CAS3] |
| `MOTORFERNSTART_CAN` | demarrage moteur a distance · bus CAN | `aktiv` · `nicht_aktiv` | Remote engine start over CAN-bus | Demarrage moteur a distance via le bus CAN |
| `MOTORFERNSTART_GENERELL` | demarrage moteur a distance · general | `aktiv` · `nicht_aktiv` | Remote engine start | Demarrage moteur a distance |
| `MOTORHAUBENKONTAKT` | contacteur de capot | `aktiv` · `nicht_aktiv` | Engine hood contact | Contacteur de capot moteur |
| `MOTOR_ART` | moteur · type | `benzin_direkt` · `benzin_normal` · `diesel_direkt` | Engine type | Type de moteur |
| `MOTOR_HUBRAUM` | moteur · cylindree | `n43b16` · `n43b20` · `n45b16` · `n45b20` · `n46b20` · `m47d20` · `n47d20` · `n52b24` … *(+34)* | Engine displacement | Cylindree |
| `PASSIVE_ACCESS` | passif · acces | `aktiv` · `nicht_aktiv` | Comfort access | Acces confort |
| `PA_CHIPCARD_ACTIVE` | _PA_ · carte a puce · actif | `aktiv` · `nicht_aktiv` | Enable writing data to comfort access transponder | Autoriser l'ecriture de donnees dans le transpondeur d'acces confort |
| `PA_CONFIG_FBDREFCHECK` | _PA_ · configuration · controle de reference telecommande | `aktiv` · `nicht_aktiv` | — | — |
| `PA_DISABLE_HK_REV` | _PA_ · desactivation · hayon · revision | `aktiv` · `nicht_aktiv` | — | — |
| `PA_FBD_REF` | _PA_ · telecommande · reference | `wert_01` | — | — |
| `PA_HA_SEARCH_ACTIVE` | _PA_ · essieu arriere · recherche · actif | `aktiv` · `nicht_aktiv` | — | — |
| `PA_HK_BACK_IR_SEARCH` | _PA_ · hayon · retour · infrarouge · recherche | `aktiv` · `nicht_aktiv` | — | — |
| `PA_HK_INTERIOR_SEARCH` | _PA_ · hayon · interieur · recherche | `aktiv` · `nicht_aktiv` | — | — |
| `PA_HK_PGS_SEARCH` | _PA_ · hayon · detection de place de stationnement · recherche | `aktiv` · `nicht_aktiv` | — | — |
| `PA_HK_REOPEN_CTL_DEP` | _PA_ · hayon · reouverture · commande · dependant | `aktiv` · `nicht_aktiv` | — | — |
| `PA_HK_REOPEN_CTR_LOCK` | _PA_ · hayon · reouverture · compteur · blocage | `aktiv` · `nicht_aktiv` | — | — |
| `PA_HK_RSCR_ACTIVE` | _PA_ · hayon · ecran (?) · actif | `aktiv` · `nicht_aktiv` | — | — |
| `PA_ID_GEBER` | _PA_ · identifiant · capteur | `wert_01` · `wert_02` | — | — |
| `PA_SILENTTIMEOUT` | _PA_ · delai de silence | `wert_01` | — | — |
| `PA_TBEREITPE` | _PA_ · pret a fonctionner (?) | `wert_01` | — | — |
| `PA_TBEREIT_PG` | _PA_ · pret a fonctionner (?) · groupe de parametres (?) | `wert_01` · `wert_02` | — | — |
| `PA_TRUNK_HIGH` | _PA_ · coffre · haut | `aktiv` · `nicht_aktiv` | — | — |
| `PA_TSPERRVR` | _PA_ · blocage de porte (?) | `wert_01` | — | — |
| `PA_TVRERFATH` | _PA_ · reconnaissance (?) | `wert_01` | — | — |
| `PERSONALISIERUNG_AUS` | personnalisation · arret | `aktiv` · `nicht_aktiv` | Disable car/key memory personalization for individual keys | Desactiver la personnalisation de la memoire par cle pour chaque cle |
| `RSSI_THRESHOLD` | puissance du signal · seuil | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` | — | Déduction du libellé : Seuil de puissance du signal radio reçu ; unité et conversion des valeurs non établies. |
| `SCHIEBEHEBEDACH` | toit ouvrant relevable | `aktiv` · `nicht_aktiv` | Sliding/tilting sunroof | Toit ouvrant coulissant/entrebaillant |
| `SEL_ZV_SENDER_1` | selection · verrouillage centralise · emetteur · 1 | `aktiv` · `nicht_aktiv` | Car/key memory 1 for pressing remote control unlock button once unlocks only driver door, twice unlocks all doors | Memoire par cle 1 : une pression ouvre la porte conducteur, deux pressions ouvrent tout |
| `SEL_ZV_SENDER_2` | selection · verrouillage centralise · emetteur · 2 | `aktiv` · `nicht_aktiv` | Car/key memory 2 for pressing remote control unlock button once unlocks only driver door, twice unlocks all doors | Memoire par cle 2 : une pression ouvre la porte conducteur, deux pressions ouvrent tout |
| `SEL_ZV_SENDER_3` | selection · verrouillage centralise · emetteur · 3 | `aktiv` · `nicht_aktiv` | Car/key memory 3 for pressing remote control unlock button once unlocks only driver door, twice unlocks all doors | Memoire par cle 3 : une pression ouvre la porte conducteur, deux pressions ouvrent tout |
| `SEL_ZV_SENDER_4` | selection · verrouillage centralise · emetteur · 4 | `aktiv` · `nicht_aktiv` | Car/key memory 4 for pressing remote control unlock button once unlocks only driver door, twice unlocks all doors | Memoire par cle 4 : une pression ouvre la porte conducteur, deux pressions ouvrent tout |
| `SEL_ZV_SENDER_DEFAULT` | selection · verrouillage centralise · emetteur · valeur par defaut | `aktiv` · `nicht_aktiv` | Default car/key memory for pressing remote control unlock button once unlocks only driver door, twice unlocks all doors | Valeur par defaut : une pression ouvre la porte conducteur, deux pressions ouvrent tout |
| `SEL_ZV_SENDER_PORT` | selection · verrouillage centralise · emetteur · port | `aktiv` · `nicht_aktiv` | — | — |
| `SENDMHKCCMLIMIT` | limite d'envoi de message Check Control (?) | `wert_01` · `wert_02` | Check control notification for engine hood open warning when exceeding certain speed [data>0 ? speed_km/h=data*5 : disabled] | Message Check Control de capot ouvert au-dela d'une certaine vitesse [valeur > 0 : vitesse km/h = valeur x 5, sinon desactive] |
| `SICHERHEITSFAHRZEUG` | vehicule de securite | `aktiv` · `nicht_aktiv` | Security vehicle | Vehicule de securite |
| `SLEEPTIMER_FH_KLRAUS` | minuterie de veille · leve-vitre · borne R coupee (accessoires) | `wert_01` · `wert_02` · `wert_03` | Time power windows can be operated after switching off accessories without opening/closing door or locking vehicle [time_s=data*6] | Duree pendant laquelle les leve-vitres restent utilisables apres coupure des accessoires [duree s = valeur x 6] |
| `SONDERTASTE_INNENLICHT` | touche speciale · eclairage interieur | `innenlicht_ein` · `nicht_aktiv` | Find vehicle functionality (interior lights switch on after pressing remote control lock button with vehicle locked) | Fonction retrouver le vehicule : l'eclairage interieur s'allume a l'appui sur verrouillage, vehicule deja verrouille |
| `SONDERTASTE_PANIKALARM` | touche speciale · alarme panique | `panikalarm` · `nicht_aktiv` | Panic mode (trigger alarm and close power windows by holding remote control button 3) (linked to [FBD_PANIK_TOTZEIT]) | Mode panique : declenche l'alarme et ferme les vitres par appui maintenu sur le bouton 3 (lie a FBD_PANIK_TOTZEIT) |
| `SPERRE_CRASH_ER` | blocage · choc · deverrouillage (?) | `aktiv` · `nicht_aktiv` | Unlock vehicle after crash | Deverrouillage du vehicule apres un choc |
| `SPERRE_HECKKLAPPE_KLR` | blocage · hayon · borne R (accessoires) | `aktiv` · `nicht_aktiv` | Disable opening trunk lid with accessories switched on | Interdire l'ouverture du coffre accessoires sous tension |
| `SPERRE_HKL_KEY_VALID` | blocage · hayon · cle · valide | `aktiv` · `nicht_aktiv` | Disable remote control trunk lid button after inserting key | Desactiver le bouton de coffre de la telecommande une fois la cle inseree |
| `SPERRE_KLRAUS_SST` | blocage · borne R coupee (accessoires) · _SST_ | `aktiv` · `nicht_aktiv` | Disable switching off accessories with start/stop button (accessories switch off after ejecting key or locking vehicle) | Interdire la coupure des accessoires par le bouton start/stop (coupure a l'ejection de la cle ou au verrouillage) |
| `SQCNT_UPDATE` | compteur de sequence · mise a jour | `aktiv` · `nicht_aktiv` | — | — |
| `SS_GETRIEBE` | capteur (?) · boite de vitesses | `aktiv` · `nicht_aktiv` | Fault memory handling for sport sequential gearbox (SSG) | Gestion de la memoire de defauts de la boite sequentielle sport (SSG) |
| `STARTABBRUCH_BREMSE` | interruption de demarrage · frein | `aktiv` · `nicht_aktiv` | Abort engine start when releasing brake pedal | Interrompre le demarrage si la pedale de frein est relachee |
| `STARTABBRUCH_P` | interruption de demarrage · P | `aktiv` · `nicht_aktiv` | Abort engine start when exiting transmission park/neutral | Interrompre le demarrage si la boite quitte P ou N |
| `STARTABBR_KLR` | interruption de demarrage · borne R (accessoires) | `aktiv` · `nicht_aktiv` | Abort engine start when releasing clutch pedal | Interrompre le demarrage si la pedale d'embrayage est relachee |
| `STARTDELAY_K_START` | delai de demarrage · K · demarrage | `aktiv` · `nicht_aktiv` | Temperature-dependent start delay for diesel engine pre-glowing | Temporisation de demarrage selon la temperature pour le prechauffage diesel |
| `STARTWUNSCH_KL50` | demande de demarrage · borne 50 (demarreur) | `aktiv` · `nicht_aktiv` | — | Déduction du libellé : Prise en compte de la demande de démarrage liée à la borne 50 du démarreur. |
| `STROM_KS_MAX` | courant · court-circuit (?) · maximum | `wert_01` · `ewp` · `dynamik_2` · `hifi_mulf_e91` · `mit_fl_und_tfl` · `e92_m` · `tvm_isdb_t` · `wert_02` · `wert_03` | Maximum current for short circuit detection [current_A=data*10] | Courant maximal de detection de court-circuit [courant A = valeur x 10] |
| `STROM_KS_MIN` | courant · court-circuit (?) · minimum | `wert_01` | Minimum current for short circuit detection [current_A=data*10] | Courant minimal de detection de court-circuit [courant A = valeur x 10] |
| `STROM_RUHE` | courant · repos | `wert_01` | Maximum permitted current in sleep mode [current_mA=data*20] | Courant maximal admis en veille [courant mA = valeur x 20] |
| `TEST_DUTY_CYCLE` | test · rapport cyclique | `aktiv` · `nicht_aktiv` | — | Déduction du libellé : Rapport cyclique utilisé pour un test ; le signal testé n’est pas identifié par ce seul nom. |
| `TEST_LED_KS_AKTIV` | test · LED · court-circuit (?) · actif | `nicht_aktiv` · `aktiv` | Short circuit detection for start/stop button LED | Detection de court-circuit sur la LED du bouton start/stop |
| `TOEHK_EO_HS_OHNE_NR` | _TOEHK_ · _EO_ · lunette arriere (?) · sans · numero | `aktiv` · `nicht_aktiv` | Unpersonalized car/key memory for footwell trunk lid button opens touring rear window instead of trunk lid | Memoire par cle non personnalisee : le bouton de plancher ouvre la lunette arriere au lieu du hayon |
| `TOEHK_EO_HS_SCHL_NR01` | _TOEHK_ · _EO_ · lunette arriere (?) · cle / fermeture · [NR01] | `aktiv` · `nicht_aktiv` | Car/key memory 1 for footwell trunk lid button opens touring rear window instead of trunk lid | Memoire par cle 1 : le bouton de plancher ouvre la lunette arriere au lieu du hayon |
| `TOEHK_EO_HS_SCHL_NR02` | _TOEHK_ · _EO_ · lunette arriere (?) · cle / fermeture · [NR02] | `aktiv` · `nicht_aktiv` | Car/key memory 2 for footwell trunk lid button opens touring rear window instead of trunk lid | Memoire par cle 2 : le bouton de plancher ouvre la lunette arriere au lieu du hayon |
| `TOEHK_EO_HS_SCHL_NR03` | _TOEHK_ · _EO_ · lunette arriere (?) · cle / fermeture · [NR03] | `aktiv` · `nicht_aktiv` | Car/key memory 3 for footwell trunk lid button opens touring rear window instead of trunk lid | Memoire par cle 3 : le bouton de plancher ouvre la lunette arriere au lieu du hayon |
| `TOEHK_EO_HS_SCHL_NR04` | _TOEHK_ · _EO_ · lunette arriere (?) · cle / fermeture · [NR04] | `aktiv` · `nicht_aktiv` | Car/key memory 4 for footwell trunk lid button opens touring rear window instead of trunk lid | Memoire par cle 4 : le bouton de plancher ouvre la lunette arriere au lieu du hayon |
| `TRANSPONDER` | transpondeur | `aktiv` · `nicht_aktiv` | Transponder | Transpondeur |
| `TYP_LENKUNG` | type · direction | `linkslenker` · `rechtslenker` | Steering type | Type de direction |
| `VERRIEGELUNGSSCHWELLE` | seuil de verrouillage | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` | Threshold speed for speed-dependent locking [CAS,CAS2,CAS3: speed_km/h=data] | Vitesse de declenchement du verrouillage automatique [vitesse km/h = valeur] |
| `VER_AUT_X_MIN_DEFAULT` | version · automatique · X · minimum · valeur par defaut | `aktiv` · `nicht_aktiv` | Default car/key memory for automatically re-lock after being unlocked for a certain time without opening a door (linked to [ZEIT_AUTO_ZV_VR]) | Valeur par defaut du reverrouillage automatique apres un certain temps sans ouverture de porte (lie a ZEIT_AUTO_ZV_VR) |
| `VER_AUT_X_MIN_OHNE_NR` | version · automatique · X · minimum · sans · numero | `aktiv` · `nicht_aktiv` | Car/key memory for automatically re-lock after being unlocked for a certain time without opening a door (linked to [ZEIT_AUTO_ZV_VR]) | Memoire par cle pour le reverrouillage automatique apres un certain temps sans ouverture de porte (lie a ZEIT_AUTO_ZV_VR) |
| `VER_AUT_X_MIN_PORT` | version · automatique · X · minimum · port | `aktiv` · `nicht_aktiv` | — | — |
| `VER_AUT_X_MIN_SENDER_1` | version · automatique · X · minimum · emetteur · 1 | `aktiv` · `nicht_aktiv` | Car/key memory 1 for Automatically re-lock after being unlocked for a certain time without opening a door (linked to [ZEIT_AUTO_ZV_VR]) | Memoire par cle 1 pour le reverrouillage automatique sans ouverture de porte (lie a ZEIT_AUTO_ZV_VR) |
| `VER_AUT_X_MIN_SENDER_2` | version · automatique · X · minimum · emetteur · 2 | `aktiv` · `nicht_aktiv` | Car/key memory 2 for Automatically re-lock after being unlocked for a certain time without opening a door (linked to [ZEIT_AUTO_ZV_VR]) | Memoire par cle 2 pour le reverrouillage automatique sans ouverture de porte (lie a ZEIT_AUTO_ZV_VR) |
| `VER_AUT_X_MIN_SENDER_3` | version · automatique · X · minimum · emetteur · 3 | `aktiv` · `nicht_aktiv` | Car/key memory 3 for Automatically re-lock after being unlocked for a certain time without opening a door (linked to [ZEIT_AUTO_ZV_VR]) | Memoire par cle 3 pour le reverrouillage automatique sans ouverture de porte (lie a ZEIT_AUTO_ZV_VR) |
| `VER_AUT_X_MIN_SENDER_4` | version · automatique · X · minimum · emetteur · 4 | `aktiv` · `nicht_aktiv` | Car/key memory 4 for Automatically re-lock after being unlocked for a certain time without opening a door (linked to [ZEIT_AUTO_ZV_VR]) | Memoire par cle 4 pour le reverrouillage automatique sans ouverture de porte (lie a ZEIT_AUTO_ZV_VR) |
| `VR_GESCHW_EINMALIG` | avant droit · vitesse · une seule fois | `aktiv` · `nicht_aktiv` | Automatically lock once when exceeding a certain speed | Verrouillage automatique unique au franchissement d'une certaine vitesse |
| `VR_KLR` | avant droit · borne R (accessoires) | `aktiv` · `nicht_aktiv` | Automatically lock after switching on accessories | Verrouillage automatique a la mise sous tension des accessoires |
| `VR_KLR_EINMALIG` | avant droit · borne R (accessoires) · une seule fois | `aktiv` · `nicht_aktiv` | Automatically lock once after switching on accessories | Verrouillage automatique unique a la mise sous tension des accessoires |
| `ZEIT_AUTO_ZV_VR` | duree · automatique · verrouillage centralise · avant droit | `wert_01` | Time until automatically re-lock after being unlocked without opening a door [time_s=data*5] | Delai avant reverrouillage automatique sans ouverture de porte [duree s = valeur x 5] |
| `ZEIT_BREMSE` | duree · frein | `wert_01` · `wert_02` · `wert_03` · `wert_04` | Time brake or clutch pedal must be depressed before engine can be started [time_ms=data*5] | Duree d'appui requise sur le frein ou l'embrayage avant autorisation de demarrage [duree ms = valeur x 5] |
| `ZEIT_KL50_ANSTRG` | duree · borne 50 (demarreur) · commande | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` | — | Déduction du libellé : Durée de commande de la borne 50 du démarreur ; unité à vérifier dans les données du module. |
| `ZV_AUTHENTISIERUNG` | verrouillage centralise · authentification | `aktiv` · `nicht_aktiv` | Central locking authentification | Authentification du verrouillage centralise |
| `ZV_DISFERTMODE` | verrouillage centralise · mode fin de chaine | `aktiv` · `nicht_aktiv` | Disable manufacturing mode for central locking | Desactiver le mode fin de chaine pour le verrouillage centralise |
| `ZV_DISTRANSMODE` | verrouillage centralise · mode transport | `aktiv` · `nicht_aktiv` | Disable transport mode for central locking | Desactiver le mode transport pour le verrouillage centralise |
| `ZV_DISWERKSTMODE` | verrouillage centralise · mode atelier | `aktiv` · `nicht_aktiv` | Disable workshop mode for central locking | Desactiver le mode atelier pour le verrouillage centralise |
| `ZV_SELEKTIV` | verrouillage centralise · selectif | `aktiv` · `nicht_aktiv` | Pressing remote control unlock button once unlocks only driver door, twice unlocks all doors | Une pression sur le bouton de deverrouillage n'ouvre que la porte conducteur, deux pressions ouvrent toutes les portes |
| `ZYLINDER_ZAHL` | cylindres · nombre | — | Cylinder count | Nombre de cylindres |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Signification (EN)** — description issue de la base communautaire NCS Dummy, verbatim.

**Explication (FR)** — traduction française ou interprétation explicitement marquée
« Déduction du libellé ». Cette dernière reste à confirmer ; elle ne documente pas
les valeurs ni un effet testé sur le véhicule. Un tiret signifie qu’aucune explication
n’est établie. Voir [la méthode et les sources](TRADUCTIONS.md). Les notations d'unités sont converties : `[durée s = valeur / 10]`.
