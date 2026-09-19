# CAS — Acces et antidemarrage

🔴 Module `CAS` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🔴 **Module a ne pas coder.** Liste fournie pour comprendre, pas pour modifier.

---

## Identification

| | |
|---|---|
| Variante | **`CAS3.C09`** |
| Identifiée par | recoupement avec `CAS.TRC` lu sur le véhicule |
| Fonctions retenues | **255** |
| Écartées (hors blocs du TRC) | 0 |
| Avec valeurs | 253 |
| Avec explication rédigée | 7 |
| Blocs d'adresses du TRC | 10 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 255 fonctions codables

| FSW | Traduction | Valeurs | Explication |
|---|---|---|---|
| `ABZUGSPERRE_PLOCK_C6` | blocage de retrait · verrouillage · identifiant interne (?) | `aktiv` · `nicht_aktiv` |  |
| `ANLASSERSCHUTZ_3SEK` | protection du demarreur · 3 s | `aktiv` · `nicht_aktiv` |  |
| `ANLASSERSPERRE_KPS` | blocage du demarreur · capteur de position (?) | `aktiv` · `nicht_aktiv` |  |
| `ANLASSERSPERRE_P` | blocage du demarreur · P | `aktiv` · `nicht_aktiv` |  |
| `ANLASSSPERRE_BLS` | blocage du demarreur · contacteur de feu stop | `aktiv` · `nicht_aktiv` | Autorise le demarrage sans appuyer sur la pedale de frein. |
| `ANSTEUERZEIT` | duree de commande | `wert_01` |  |
| `AUSWERTUNG_B_DRUCK` | evaluation · B · pression | `aktiv` · `nicht_aktiv` |  |
| `AUSWERTUNG_P_CAN` | evaluation · P · bus CAN | `aktiv` · `nicht_aktiv` |  |
| `AUSWERTUNG_P_HW` | evaluation · P · materiel (?) | `aktiv` · `nicht_aktiv` |  |
| `AUT_OFFNEN_HK_IN_ENTLPOS` | automatique · ouverture · hayon · entree · position de decharge | `aktiv` · `nicht_aktiv` |  |
| `BAUART` | type de construction | `limousine` · `coupe` · `touring` · `cabrio` · `roadster` · `compact` · `sicherheitsfahrzeug` · `light_protection` · `sport_coupe` |  |
| `BAUREIHE` | serie | `e65` · `e66` · `e67` · `e60` · `e61` · `e63` · `e64` · `e90` … *(+21)* |  |
| `BEDINGUNG_CC_10KM` | condition · Check Control · 10 km | `aktiv` · `nicht_aktiv` |  |
| `BOS_DATEN_ANFRAGEN` | vehicule prioritaire (?) · donnees · requetes | `aktiv` · `nicht_aktiv` |  |
| `BREMSE_DRUCK` | frein · pression | `wert_01` |  |
| `CAN_MEHRFACH_SICHERN` | bus CAN · multiple · verrouillage | `aktiv` · `nicht_aktiv` |  |
| `CBS_VERSION` | entretien selon besoin (CBS) · version | `wert_01` · `wert_02` · `wert_03` · `dynamik_2` |  |
| `CCM_0X42_IM_NACHLAUF` | message Check Control · valeur 0x42 (?) · dans · temporisation | `aktiv` · `nicht_aktiv` |  |
| `CCM_SEND_INVALID` | message Check Control · emission · invalide | `aktiv` · `nicht_aktiv` |  |
| `CHIPCARD` | carte a puce | `aktiv` · `nicht_aktiv` |  |
| `CLASS_PWR` | classe · alimentation | `keine` · `klasse_1` · `klasse_2` · `invalid` · `e92_m` |  |
| `CLT_TASTE` | climatisation (?) · touche | `aktiv` · `nicht_aktiv` |  |
| `CRASH_BESCHLEUNIGUNG` | choc · acceleration | `wert_01` · `wert_02` |  |
| `CRASH_SCHWERE` | choc · gravite | `wert_01` · `wert_02` |  |
| `DELAY_KL50L` | delai · borne 50L | `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `DELAY_KLR_AUS` | delai · regulation de climatisation · arret | `wert_01` · `wert_02` |  |
| `EE_TRSP_CCM_0CD` | memoire EEPROM · transport · message Check Control · identifiant (?) | `aktiv` · `nicht_aktiv` |  |
| `EE_TRSP_DEBUG` | memoire EEPROM · transport · deverminage | `aktiv` · `nicht_aktiv` |  |
| `EE_TRSP_PERM` | memoire EEPROM · transport · permanent | `aktiv` · `nicht_aktiv` |  |
| `EE_ZEIT_ABFR` | memoire EEPROM · duree · interrogation | `wert_01` |  |
| `EE_ZEIT_NLF` | memoire EEPROM · duree · feu arriere de brouillard (?) | `wert_01` |  |
| `EINSTIEGSHILFE_DEFAULT` | aide a l'acces · valeur par defaut | `aktiv` · `nicht_aktiv` |  |
| `EINSTIEGSHILFE_OHNE_NR` | aide a l'acces · sans · numero | `aktiv` · `nicht_aktiv` |  |
| `EINSTIEGSHILFE_PORT` | aide a l'acces · port | `aktiv` · `nicht_aktiv` |  |
| `EINSTIEGSHILFE_SENDER_1` | aide a l'acces · emetteur · 1 | `aktiv` · `nicht_aktiv` |  |
| `EINSTIEGSHILFE_SENDER_2` | aide a l'acces · emetteur · 2 | `aktiv` · `nicht_aktiv` |  |
| `EINSTIEGSHILFE_SENDER_3` | aide a l'acces · emetteur · 3 | `aktiv` · `nicht_aktiv` |  |
| `EINSTIEGSHILFE_SENDER_4` | aide a l'acces · emetteur · 4 | `aktiv` · `nicht_aktiv` |  |
| `ELV_AUT_FCT` | blocage de colonne de direction · automatique · fonction | `aktiv` · `nicht_aktiv` |  |
| `ELV_CCM_NEU` | blocage de colonne de direction · message Check Control · nouveau | `aktiv` · `nicht_aktiv` |  |
| `ELV_RESET_FCT` | blocage de colonne de direction · remise a zero · fonction | `aktiv` · `nicht_aktiv` |  |
| `ENABLE_CLT_NACH_CRASH` | activation · climatisation (?) · apres · choc | `aktiv` · `nicht_aktiv` |  |
| `ERROR_EJECT` | erreur · ejection | `aktiv` · `nicht_aktiv` |  |
| `ER_KEYOUT_AUTOVR` | deverrouillage (?) · cle retiree · verrouillage automatique | `aktiv` · `nicht_aktiv` | Deverrouille les portes au retrait de la cle. |
| `ER_TANKKLAPPE_SER` | deverrouillage (?) · trappe a carburant · serie (?) | `aktiv` · `nicht_aktiv` |  |
| `ER_TANKKLAPPE_VR` | deverrouillage (?) · trappe a carburant · avant droit | `aktiv` · `nicht_aktiv` |  |
| `ES_EINMALIG_CLT` | unite de commande (?) · une seule fois · climatisation (?) | `aktiv` · `nicht_aktiv` |  |
| `EWS_EGS` | antidemarrage electronique · calculateur de boite | `nicht_aktiv` · `aktiv` |  |
| `EWS_FEHLER_MOTORLAUF` | antidemarrage electronique · erreur · moteur tournant | `nicht_aktiv` · `aktiv` |  |
| `EWS_NACHLAUF_KLR_AUS` | antidemarrage electronique · temporisation · regulation de climatisation · arret | `aktiv` · `nicht_aktiv` |  |
| `EWS_SCHRITTWEITE_2` | antidemarrage electronique · pas · 2 | `nicht_aktiv` · `aktiv` |  |
| `FBDIN_9_6_KBAUD` | entree telecommande · 9 · 6 · kBaud | `aktiv` · `nicht_aktiv` |  |
| `FBD_AKTIV_KEYIN` | telecommande · actif · cle inseree | `aktiv` · `nicht_aktiv` |  |
| `FBD_AKTIV_KL15AUS` | telecommande · actif · borne 15 coupee | `aktiv` · `nicht_aktiv` |  |
| `FBD_COMFORT_OPEN_LF_PING` | telecommande · confort · ouvert · avant gauche (?) · test de presence | `aktiv` · `nicht_aktiv` |  |
| `FBD_DISFERTMODE` | telecommande · mode fin de chaine | `aktiv` · `nicht_aktiv` |  |
| `FBD_DISTRANSMODE` | telecommande · mode transport | `aktiv` · `nicht_aktiv` |  |
| `FBD_DISWERKSTMODE` | telecommande · mode atelier | `aktiv` · `nicht_aktiv` |  |
| `FBD_HECKKLAPPE_TOTZEIT` | telecommande · hayon · temps mort | `wert_01` · `wert_02` |  |
| `FBD_HKL_NUR_ER` | telecommande · hayon · uniquement · deverrouillage (?) | `aktiv` · `nicht_aktiv` |  |
| `FBD_HSF_0` | telecommande · toit ouvrant · 0 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` · `wert_09` |  |
| `FBD_HSF_1` | telecommande · toit ouvrant · 1 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` · `wert_09` |  |
| `FBD_LOAD_POS_HK_HK` | telecommande · charge · position · hayon · hayon | `aktiv` · `nicht_aktiv` |  |
| `FBD_LOAD_POS_LF_PING` | telecommande · charge · position · avant gauche (?) · test de presence | `aktiv` · `nicht_aktiv` |  |
| `FBD_MOTORLAUF` | telecommande · moteur tournant | `wert_01` |  |
| `FBD_MOTORSTART` | telecommande · demarrage moteur | `wert_01` |  |
| `FBD_NO_LOAD_POS` | telecommande · non · charge · position | `aktiv` · `nicht_aktiv` |  |
| `FBD_PANIKDAUER` | telecommande · duree d'alarme panique | `wert_01` · `wert_02` · `wert_03` |  |
| `FBD_PANIK_TOTZEIT` | telecommande · mode panique · temps mort | `wert_01` |  |
| `FBD_SERVICEFKT_AKTIV` | telecommande · fonction d'entretien · actif | `aktiv` · `nicht_aktiv` |  |
| `FBD_TESTKEY_AKTIV` | telecommande · cle de test · actif | `aktiv` · `nicht_aktiv` |  |
| `FHM_CA_OHNE_EKS` | module leve-vitre · acces confort (?) · sans · systeme electronique de cle | `aktiv` · `nicht_aktiv` |  |
| `FHM_CA_OHNE_EKS_DELAY` | module leve-vitre · acces confort (?) · sans · systeme electronique de cle · delai | `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `FH_ABSCHALTKRITERIUM` | leve-vitre · critere de coupure | `aktiv` · `nicht_aktiv` |  |
| `FH_CABRIO_VERDECK_MODUL` | leve-vitre · cabriolet · capote · module | `aktiv` · `nicht_aktiv` |  |
| `FH_DISAKL15` | leve-vitre · desactivation borne 15 | `aktiv` · `nicht_aktiv` |  |
| `FH_DISAKLR` | leve-vitre · desactivation (?) | `aktiv` · `nicht_aktiv` |  |
| `FH_DISASHD_KL15` | leve-vitre · desactivation du toit ouvrant · borne 15 (contact) | `aktiv` · `nicht_aktiv` |  |
| `FH_DISCLSYDEFECT` | leve-vitre · defaut systeme (?) | `aktiv` · `nicht_aktiv` |  |
| `FH_DISFBDDEFCT` | leve-vitre · defaut de telecommande (?) | `aktiv` · `nicht_aktiv` |  |
| `FH_DISFERTMODE` | leve-vitre · mode fin de chaine | `aktiv` · `nicht_aktiv` |  |
| `FH_DISTGDEFECT` | leve-vitre · defaut de distance (?) | `aktiv` · `nicht_aktiv` |  |
| `FH_DISTRANSMODE` | leve-vitre · mode transport | `aktiv` · `nicht_aktiv` |  |
| `FH_DISWERKSTMODE` | leve-vitre · mode atelier | `aktiv` · `nicht_aktiv` |  |
| `FH_ECEFIN` | leve-vitre · norme ECE Finlande | `aktiv` · `nicht_aktiv` |  |
| `FH_HINTEN` | leve-vitre · arriere | `aktiv` · `nicht_aktiv` |  |
| `FH_KINDERSICHERUNG` | leve-vitre · securite enfants | `aktiv` · `nicht_aktiv` |  |
| `FH_KMFRTFBDTIM` | leve-vitre · temporisation confort telecommande | `aktiv` · `nicht_aktiv` |  |
| `FH_KOMFORTF_MIT_SUBST` | leve-vitre · fermeture confort · avec · substitution | `aktiv` · `nicht_aktiv` | Fermeture confort des vitres avec substitution. |
| `FH_RELEASE_CA_BASIC` | leve-vitre · relachement · acces confort (?) · base | `aktiv` · `nicht_aktiv` |  |
| `FH_RELEASE_TOP_ZWSTLG` | leve-vitre · relachement · haut · position intermediaire | `aktiv` · `nicht_aktiv` |  |
| `FH_RESETTIMEOUTKL50` | leve-vitre · delai de reinitialisation borne 50 | `aktiv` · `nicht_aktiv` |  |
| `FH_SCHLIESSEN_EINSTG` | leve-vitre · fermeture · acces | `aktiv` · `nicht_aktiv` |  |
| `FH_SICHERHEITSFAHRZEUG` | leve-vitre · vehicule de securite | `aktiv` · `nicht_aktiv` |  |
| `FH_TUERBED_EINSTG` | leve-vitre · commande de porte · acces | `aktiv` · `nicht_aktiv` |  |
| `FH_USACANADA` | leve-vitre · USA / Canada | `aktiv` · `nicht_aktiv` |  |
| `FS_DATEN_ANFRAGEN` | siege conducteur (?) · donnees · requetes | `aktiv` · `nicht_aktiv` |  |
| `GESCHW_REQ_CC` | vitesse · requete · Check Control | `wert_01` · `wert_02` |  |
| `GESCHW_SPERRE_TOEHK` | vitesse · blocage · _TOEHK_ | `wert_01` |  |
| `GETRIEBE_GAENGE` | boite de vitesses · rapports | — |  |
| `GETRIEBE_TYP` | boite de vitesses · type | `handschaltung` · `automatik` · `schrittschaltung` · `ssg_schaltung` · `dkg_schaltung` |  |
| `HECKSCHEIBE_ENTR_SENDER_1` | lunette arriere · deverrouillage · emetteur · 1 | `aktiv` · `nicht_aktiv` |  |
| `HECKSCHEIBE_ENTR_SENDER_2` | lunette arriere · deverrouillage · emetteur · 2 | `aktiv` · `nicht_aktiv` |  |
| `HECKSCHEIBE_ENTR_SENDER_3` | lunette arriere · deverrouillage · emetteur · 3 | `aktiv` · `nicht_aktiv` |  |
| `HECKSCHEIBE_ENTR_SENDER_4` | lunette arriere · deverrouillage · emetteur · 4 | `aktiv` · `nicht_aktiv` |  |
| `HECKSCHEI_ENTR_OHNE_NR` | lunette arriere · deverrouillage · sans · numero | `aktiv` · `nicht_aktiv` |  |
| `HINTERE_TUEREN_MINI` | arriere · portes · MINI | `aktiv` · `nicht_aktiv` |  |
| `HKAT_EO_HS_OHNE_NR` | catalyseur · _EO_ · lunette arriere (?) · sans · numero | `aktiv` · `nicht_aktiv` |  |
| `HKAT_EO_HS_SCHL_NR01` | catalyseur · _EO_ · lunette arriere (?) · cle / fermeture · [NR01] | `aktiv` · `nicht_aktiv` |  |
| `HKAT_EO_HS_SCHL_NR02` | catalyseur · _EO_ · lunette arriere (?) · cle / fermeture · [NR02] | `aktiv` · `nicht_aktiv` |  |
| `HKAT_EO_HS_SCHL_NR03` | catalyseur · _EO_ · lunette arriere (?) · cle / fermeture · [NR03] | `aktiv` · `nicht_aktiv` |  |
| `HKAT_EO_HS_SCHL_NR04` | catalyseur · _EO_ · lunette arriere (?) · cle / fermeture · [NR04] | `aktiv` · `nicht_aktiv` |  |
| `HKL_LIFT_BEI_CA` | hayon · levee · lors de · acces confort (?) | `aktiv` · `nicht_aktiv` |  |
| `HK_ENABLE_OPEN` | hayon · activation · ouvert | `aktiv` · `nicht_aktiv` |  |
| `HK_GESPERRT_IN_GESICHERT` | hayon · bloque · entree · verrouille | `aktiv` · `nicht_aktiv` |  |
| `HK_V_VER_MODUS` | hayon · _V_ · version · mode | `modus_00` · `modus_01` · `modus_02` · `modus_03` |  |
| `HS_ENABLE_OPEN` | lunette arriere (?) · activation · ouvert | `aktiv` · `nicht_aktiv` |  |
| `KL15HW01_ALLWAYS` | borne 15 · toujours | `aktiv` · `nicht_aktiv` |  |
| `KL30G_EIN_HKT_AUSSEN_07` | borne 30G · marche · bouton de hayon · exterieur · 07 | `aktiv` · `nicht_aktiv` |  |
| `KL30G_EIN_HKT_FBD_07` | borne 30G · marche · bouton de hayon · telecommande · 07 | `aktiv` · `nicht_aktiv` |  |
| `KL30G_NACHLAUF_NACH_SZ1` | borne 30G · temporisation · apres · centre de commande 1 (?) | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` |  |
| `KL30G_TIMEOUT` | borne 30G · delai d'attente | `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `KLASSE_BATTERIE` | classe · batterie | `wert_00` |  |
| `KLR_AUS_TIMEOUT` | regulation de climatisation · arret · delai d'attente | `wert_01` · `wert_02` · `wert_03` |  |
| `KLR_AUS_TK` | regulation de climatisation · arret · telecommunication (?) | `aktiv` · `nicht_aktiv` |  |
| `KLR_EIN_GESCHW` | regulation de climatisation · marche · vitesse | `nicht_aktiv` · `aktiv` |  |
| `KLSTRG_DISFERTMODE` | commande de climatisation · mode fin de chaine | `aktiv` · `nicht_aktiv` |  |
| `KLSTRG_DISTRANSMODE` | commande de climatisation · mode transport | `aktiv` · `nicht_aktiv` |  |
| `KLSTRG_DISWERKSTMODE` | commande de climatisation · mode atelier | `aktiv` · `nicht_aktiv` |  |
| `KLSTRG_GRB_ID` | commande de climatisation · groupe (?) · identifiant | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` |  |
| `KLSTRG_TESTER` | commande de climatisation · testeur | `aktiv` · `nicht_aktiv` |  |
| `KMFRT_OEFFNEN` | confort · ouverture | `wert_01` |  |
| `KMFRT_OEFF_HINTEN` | confort · ouverture · arriere | `wert_01` · `wert_02` |  |
| `KMFRT_OEFF_SHD` | confort · ouverture · toit ouvrant | `wert_01` · `wert_02` |  |
| `KMFRT_SCHLIESSEN` | confort · fermeture | `wert_01` | Temporisation avant rabattement des retroviseurs : 00 immediat, 05 = 0,5 s, 0A = 1 s, 0F = 2,5 s. |
| `KMFRT_SCHL_VORNE` | confort · cle / fermeture · avant | `wert_01` |  |
| `KOMFORTOEFFNUNG` | ouverture confort | `aktiv` · `nicht_aktiv` |  |
| `KOMFORTOEFFNUNG_FB` | ouverture confort · telecommande | `aktiv` · `nicht_aktiv` |  |
| `KOMFORTSCHLIESSUNG` | fermeture confort | `aktiv` · `nicht_aktiv` |  |
| `KOMFORTSCHLIESSUNG_FB` | fermeture confort · telecommande | `aktiv` · `nicht_aktiv` | Fermeture des vitres et du toit par maintien de la telecommande. |
| `KOMFORTSCHLIESSUNG_PA` | fermeture confort · _PA_ | `aktiv` · `nicht_aktiv` | Fermeture confort par le contacteur de porte. |
| `KOMFORTSTART` | demarrage confort | `aktiv` · `nicht_aktiv` |  |
| `KOMFORT_AUS_IN_50MS` | confort · arret · entree · 50 ms | `wert_02` · `wert_04` · `wert_05` · `wert_01` · `wert_03` |  |
| `KOMFORT_EIN` | confort · marche | `aktiv` · `nicht_aktiv` |  |
| `KOMFORT_EJECT` | confort · ejection | `aktiv` · `nicht_aktiv` | Ejection automatique de la cle du neiman. |
| `KOMFORT_SCHL_SHD` | confort · cle / fermeture · toit ouvrant | `wert_01` · `wert_02` |  |
| `LAENDERVARIANTE_BNDB` | variante pays · bande B | `ece` · `us` · `deutschland` · `gb` · `japan` · `australien` · `korea` · `brasilien` … *(+13)* |  |
| `MAX_KMFRT_UNTERB` | maximum · confort · interruption | `wert_01` |  |
| `MAX_KMFRT_UNTERB_FBD` | maximum · confort · interruption · telecommande | `wert_01` |  |
| `MFS_KEY_PLGD` | volant multifonction · cle · diagnostic (?) | `aktiv` · `nicht_aktiv` |  |
| `MFS_MIT_TON` | volant multifonction · avec · son | `aktiv` · `nicht_aktiv` |  |
| `MIND_KMFRT_NACH_UNTERB` | minimum · confort · apres · interruption | `wert_01` |  |
| `MOTORFERNSTART_CAN` | demarrage moteur a distance · bus CAN | `aktiv` · `nicht_aktiv` |  |
| `MOTORFERNSTART_GENERELL` | demarrage moteur a distance · general | `aktiv` · `nicht_aktiv` |  |
| `MOTORHAUBENKONTAKT` | contacteur de capot | `aktiv` · `nicht_aktiv` |  |
| `MOTOR_ART` | moteur · type | `benzin_direkt` · `benzin_normal` · `diesel_direkt` |  |
| `MOTOR_HUBRAUM` | moteur · cylindree | `n43b16` · `n43b20` · `n45b16` · `n45b20` · `n46b20` · `m47d20` · `n47d20` · `elmotor` … *(+38)* |  |
| `NEUES_FSE_KONZEPT` | nouveau · kit mains libres · concept | `aktiv` · `nicht_aktiv` |  |
| `NLF_AUS_ALLE_TUEREN` | feu arriere de brouillard (?) · arret · tous · portes | `aktiv` · `nicht_aktiv` |  |
| `OSFG_MSA_NO_KL15_AUTO_OFF` | capteur exterieur (?) · Start & Stop · non · borne 15 (contact) · automatique · arret | `nicht_aktiv` · `aktiv` |  |
| `PASSIVE_ACCESS` | passif · acces | `aktiv` · `nicht_aktiv` |  |
| `PA_CHIPCARD_ACTIVE` | _PA_ · carte a puce · actif | `aktiv` · `nicht_aktiv` |  |
| `PA_CONFIG_FBDREFCHECK` | _PA_ · configuration · controle de reference telecommande | `aktiv` · `nicht_aktiv` |  |
| `PA_DISABLE_HK_REV` | _PA_ · desactivation · hayon · revision | `aktiv` · `nicht_aktiv` |  |
| `PA_FBD_REF` | _PA_ · telecommande · reference | `wert_01` |  |
| `PA_HA_SEARCH_ACTIVE` | _PA_ · essieu arriere · recherche · actif | `aktiv` · `nicht_aktiv` |  |
| `PA_HK_2SEARCH` | _PA_ · hayon · recherche | `aktiv` · `nicht_aktiv` |  |
| `PA_HK_BACK_IR_SEARCH` | _PA_ · hayon · retour · infrarouge · recherche | `aktiv` · `nicht_aktiv` |  |
| `PA_HK_INTERIOR_SEARCH` | _PA_ · hayon · interieur · recherche | `aktiv` · `nicht_aktiv` |  |
| `PA_HK_PGS_SEARCH` | _PA_ · hayon · detection de place de stationnement · recherche | `aktiv` · `nicht_aktiv` |  |
| `PA_HK_REOPEN_CTL_DEP` | _PA_ · hayon · reouverture · commande · dependant | `aktiv` · `nicht_aktiv` |  |
| `PA_HK_REOPEN_CTR_LOCK` | _PA_ · hayon · reouverture · compteur · blocage | `aktiv` · `nicht_aktiv` |  |
| `PA_HK_REOPEN_INDICATE` | _PA_ · hayon · reouverture · indication | `aktiv` · `nicht_aktiv` |  |
| `PA_HK_REV_ZYKL` | _PA_ · hayon · revision · cycle | `aktiv` · `nicht_aktiv` |  |
| `PA_HK_RSCR_ACTIVE` | _PA_ · hayon · ecran (?) · actif | `aktiv` · `nicht_aktiv` |  |
| `PA_ID_GEBER` | _PA_ · identifiant · capteur | `wert_01` · `wert_02` |  |
| `PA_LF_PING_PGS` | _PA_ · avant gauche (?) · test de presence · detection de place de stationnement | `aktiv` · `nicht_aktiv` |  |
| `PA_LF_PING_TAGE` | _PA_ · avant gauche (?) · test de presence · jours | `aktiv` · `nicht_aktiv` |  |
| `PA_MAUT_CLOSE_SILENT` | _PA_ · peage · fermeture · silencieux | `aktiv` · `nicht_aktiv` |  |
| `PA_MONITOR_ACTIVE` | _PA_ · surveillance · actif | `aktiv` · `nicht_aktiv` |  |
| `PA_SEARCH_HK_VORRASTE` | _PA_ · recherche · hayon · premier cran | `aktiv` · `nicht_aktiv` |  |
| `PA_SILENTTIMEOUT` | _PA_ · delai de silence | `wert_01` |  |
| `PA_SILENT_TO_4TAGE` | _PA_ · silencieux · toit ouvrant · 4 jours | `aktiv` · `nicht_aktiv` |  |
| `PA_SUPERRANGE_R56` | _PA_ · grande portee · serie R56 | `aktiv` · `nicht_aktiv` |  |
| `PA_TAGE_REAR_RL_MIRROW` | _PA_ · jours · arriere · feu arriere · retroviseur | `aktiv` · `nicht_aktiv` |  |
| `PA_TBEREITPE` | _PA_ · pret a fonctionner (?) | `wert_01` |  |
| `PA_TBEREIT_HK` | _PA_ · pret a fonctionner (?) · hayon | `wert_01` |  |
| `PA_TBEREIT_PG` | _PA_ · pret a fonctionner (?) · groupe de parametres (?) | `wert_01` · `wert_02` |  |
| `PA_TRUNK_HIGH` | _PA_ · coffre · haut | `aktiv` · `nicht_aktiv` |  |
| `PA_TSPERRVR` | _PA_ · blocage de porte (?) | `wert_01` |  |
| `PA_TVRERFATH` | _PA_ · reconnaissance (?) | `wert_01` |  |
| `PERSONALISIERUNG_AUS` | personnalisation · arret | `aktiv` · `nicht_aktiv` |  |
| `RSSI_THRESHOLD` | puissance du signal · seuil | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `SCHIEBEHEBEDACH` | toit ouvrant relevable | `aktiv` · `nicht_aktiv` |  |
| `SEL_ZV_SENDER_1` | selection · verrouillage centralise · emetteur · 1 | `aktiv` · `nicht_aktiv` |  |
| `SEL_ZV_SENDER_2` | selection · verrouillage centralise · emetteur · 2 | `aktiv` · `nicht_aktiv` |  |
| `SEL_ZV_SENDER_3` | selection · verrouillage centralise · emetteur · 3 | `aktiv` · `nicht_aktiv` |  |
| `SEL_ZV_SENDER_4` | selection · verrouillage centralise · emetteur · 4 | `aktiv` · `nicht_aktiv` |  |
| `SEL_ZV_SENDER_DEFAULT` | selection · verrouillage centralise · emetteur · valeur par defaut | `aktiv` · `nicht_aktiv` |  |
| `SEL_ZV_SENDER_PORT` | selection · verrouillage centralise · emetteur · port | `aktiv` · `nicht_aktiv` |  |
| `SENDMHKCCMLIMIT` | limite d'envoi de message Check Control (?) | `wert_01` · `wert_02` |  |
| `SICHERHEITSFAHRZEUG` | vehicule de securite | `aktiv` · `nicht_aktiv` |  |
| `SLEEPTIMER_FH_KLRAUS` | minuterie de veille · leve-vitre · climatisation coupee (?) | `wert_01` · `wert_02` · `wert_03` |  |
| `SONDERTASTE_INNENLICHT` | touche speciale · eclairage interieur | `innenlicht_ein` · `nicht_aktiv` |  |
| `SONDERTASTE_PANIKALARM` | touche speciale · alarme panique | `panikalarm` · `nicht_aktiv` |  |
| `SPERRE_CRASH_ER` | blocage · choc · deverrouillage (?) | `aktiv` · `nicht_aktiv` |  |
| `SPERRE_HECKKLAPPE_KLR` | blocage · hayon · regulation de climatisation | `aktiv` · `nicht_aktiv` |  |
| `SPERRE_HKL_KEY_VALID_07` | blocage · hayon · cle · valide · 07 | `aktiv` · `nicht_aktiv` |  |
| `SPERRE_KLRAUS_SST` | blocage · climatisation coupee (?) · _SST_ | `aktiv` · `nicht_aktiv` |  |
| `SQCNT_UPDATE` | compteur de sequence · mise a jour | `aktiv` · `nicht_aktiv` |  |
| `SS_GETRIEBE_7` | capteur (?) · boite de vitesses · 7 | `aktiv` · `nicht_aktiv` |  |
| `STARTABBRUCH_BREMSE` | interruption de demarrage · frein | `aktiv` · `nicht_aktiv` |  |
| `STARTABBRUCH_KLR` | interruption de demarrage · regulation de climatisation | `aktiv` · `nicht_aktiv` |  |
| `STARTABBRUCH_P` | interruption de demarrage · P | `aktiv` · `nicht_aktiv` |  |
| `STARTDELAY_K_START` | delai de demarrage · K · demarrage | `aktiv` · `nicht_aktiv` |  |
| `STARTDELAY_K_START_R5X` | delai de demarrage · K · demarrage · serie R5x | `aktiv` · `nicht_aktiv` |  |
| `STARTWUNSCH_KL50` | demande de demarrage · borne 50 (demarreur) | `aktiv` · `nicht_aktiv` |  |
| `STROM_KS_MAX` | courant · court-circuit (?) · maximum | `wert_01` · `ewp` · `dynamik_2` · `hifi_mulf_e91` · `mit_fl_und_tfl` · `e92_m` · `tvm_isdb_t` · `wert_02` · `wert_03` |  |
| `STROM_KS_MIN` | courant · court-circuit (?) · minimum | `wert_01` |  |
| `STROM_RUHE` | courant · repos | `wert_01` |  |
| `TC_KL15_AUTO_OFF` | controle de traction (?) · borne 15 (contact) · automatique · arret | `aktiv` · `nicht_aktiv` |  |
| `TC_KL15_AUTO_OFF_LIGHT` | controle de traction (?) · borne 15 (contact) · automatique · arret · lumiere | `aktiv` · `nicht_aktiv` |  |
| `TC_KL15_AUTO_OFF_OSFG` | controle de traction (?) · borne 15 (contact) · automatique · arret · capteur exterieur (?) | `aktiv` · `nicht_aktiv` |  |
| `TC_MSA_IGNORE_ENG_RUN` | controle de traction (?) · Start & Stop · ignorer · moteur · marche | `aktiv` · `nicht_aktiv` |  |
| `TC_MSA_NO_KL15_AUTO_OFF` | controle de traction (?) · Start & Stop · non · borne 15 (contact) · automatique · arret | `aktiv` · `nicht_aktiv` |  |
| `TC_NOT_P_NO_KL15_AUTO_OFF` | controle de traction (?) · secours (?) · P · non · borne 15 (contact) · automatique · arret | `aktiv` · `nicht_aktiv` |  |
| `TC_TIME_KL15_AUTO_OFF` | controle de traction (?) · duree · borne 15 (contact) · automatique · arret | `wert_00` · `wert_01` · `wert_02` |  |
| `TEST_DUTY_CYCLE` | test · rapport cyclique · cycle | `aktiv` · `nicht_aktiv` |  |
| `TIME_KL15_AUT_OFF_DR_OPEN` | duree · borne 15 (contact) · automatique · arret · pression (?) · ouvert | `wert_01` · `wert_02` |  |
| `TOEHK_EO_HS_OHNE_NR` | _TOEHK_ · _EO_ · lunette arriere (?) · sans · numero | `aktiv` · `nicht_aktiv` |  |
| `TOEHK_EO_HS_SCHL_NR01` | _TOEHK_ · _EO_ · lunette arriere (?) · cle / fermeture · [NR01] | `aktiv` · `nicht_aktiv` |  |
| `TOEHK_EO_HS_SCHL_NR02` | _TOEHK_ · _EO_ · lunette arriere (?) · cle / fermeture · [NR02] | `aktiv` · `nicht_aktiv` |  |
| `TOEHK_EO_HS_SCHL_NR03` | _TOEHK_ · _EO_ · lunette arriere (?) · cle / fermeture · [NR03] | `aktiv` · `nicht_aktiv` |  |
| `TOEHK_EO_HS_SCHL_NR04` | _TOEHK_ · _EO_ · lunette arriere (?) · cle / fermeture · [NR04] | `aktiv` · `nicht_aktiv` |  |
| `TYP_LENKUNG` | type · direction | `linkslenker` · `rechtslenker` |  |
| `VERRIEGELUNGSSCHWELLE` | seuil de verrouillage | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `VER_AUT_X_MIN_DEFAULT` | version · automatique · X · minimum · valeur par defaut | `aktiv` · `nicht_aktiv` |  |
| `VER_AUT_X_MIN_OHNE_NR` | version · automatique · X · minimum · sans · numero | `aktiv` · `nicht_aktiv` |  |
| `VER_AUT_X_MIN_PORT` | version · automatique · X · minimum · port | `aktiv` · `nicht_aktiv` |  |
| `VER_AUT_X_MIN_SENDER_1` | version · automatique · X · minimum · emetteur · 1 | `aktiv` · `nicht_aktiv` |  |
| `VER_AUT_X_MIN_SENDER_2` | version · automatique · X · minimum · emetteur · 2 | `aktiv` · `nicht_aktiv` |  |
| `VER_AUT_X_MIN_SENDER_3` | version · automatique · X · minimum · emetteur · 3 | `aktiv` · `nicht_aktiv` |  |
| `VER_AUT_X_MIN_SENDER_4` | version · automatique · X · minimum · emetteur · 4 | `aktiv` · `nicht_aktiv` |  |
| `VR_GESCHW_EINMALIG` | avant droit · vitesse · une seule fois | `aktiv` · `nicht_aktiv` |  |
| `VR_KLR` | avant droit · regulation de climatisation | `aktiv` · `nicht_aktiv` |  |
| `VR_KLR_EINMALIG` | avant droit · regulation de climatisation · une seule fois | `aktiv` · `nicht_aktiv` |  |
| `ZEIT_AUTO_ZV_VR` | duree · automatique · verrouillage centralise · avant droit | `wert_01` |  |
| `ZEIT_BREMSE` | duree · frein | `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `ZEIT_KL50_ANSTRG` | duree · borne 50 (demarreur) · commande | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `ZV_AUTHENTISIERUNG` | verrouillage centralise · authentification | `aktiv` · `nicht_aktiv` |  |
| `ZV_DISFERTMODE` | verrouillage centralise · mode fin de chaine | `aktiv` · `nicht_aktiv` |  |
| `ZV_DISTRANSMODE` | verrouillage centralise · mode transport | `aktiv` · `nicht_aktiv` |  |
| `ZV_DISWERKSTMODE` | verrouillage centralise · mode atelier | `aktiv` · `nicht_aktiv` |  |
| `ZV_SELEKTIV` | verrouillage centralise · selectif | `aktiv` · `nicht_aktiv` |  |
| `ZYLINDER_ZAHL` | cylindres · nombre | — |  |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Explication** — rédigée seulement quand l'effet est établi de source sûre.
