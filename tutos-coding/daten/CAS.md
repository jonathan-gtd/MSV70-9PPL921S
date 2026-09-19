# CAS — Acces et antidemarrage

🔴 Module `CAS` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🔴 **Module a ne pas coder.** Liste fournie pour comprendre, pas pour modifier.

---

## Source

| | |
|---|---|
| Variante retenue | `CAS3.C06` |
| Fonctions codables | **229** |
| Avec valeurs extraites | 227 (99 %) |
| Avec explication rédigée | 7 |
| Fichiers DATEN analysés | 3 |

Variantes disponibles : `CAS3.C06`, `CAS3.C08`, `CAS3.C09`

> La variante réellement installée est identifiée par NCS Dummy à l'ouverture du `.TRC`.
> Si elle diffère de celle retenue ici, la liste change.

---

## Les 229 fonctions codables

| FSW | Traduction | Valeurs possibles | Explication |
|---|---|---|---|
| `ABZUGSPERRE_PLOCK_C6` | **ABZUGSPERRE** · **PLOCK** · **C6** | `aktiv` · `nicht_aktiv` |  |
| `ANLASSERSCHUTZ_3SEK` | **ANLASSERSCHUTZ** · **3SEK** | `aktiv` · `nicht_aktiv` |  |
| `ANLASSERSPERRE_KPS` | **ANLASSERSPERRE** · **KPS** | `aktiv` · `nicht_aktiv` |  |
| `ANLASSERSPERRE_P` | **ANLASSERSPERRE** · P | `aktiv` · `nicht_aktiv` |  |
| `ANLASSSPERRE_BLS` | blocage du demarreur · **BLS** | `aktiv` · `nicht_aktiv` | Autorise le demarrage sans appuyer sur la pedale de frein. |
| `ANLASSSPERRE_KLR` | blocage du demarreur · regulation de climatisation | `aktiv` · `nicht_aktiv` |  |
| `ANSTEUERZEIT` | duree de commande | `wert_01` |  |
| `AUSWERTUNG_B_DRUCK` | **AUSWERTUNG** · B · pression | `aktiv` · `nicht_aktiv` |  |
| `AUSWERTUNG_P_CAN` | **AUSWERTUNG** · P · bus CAN | `aktiv` · `nicht_aktiv` |  |
| `AUSWERTUNG_P_HW` | **AUSWERTUNG** · P · **HW** | `aktiv` · `nicht_aktiv` |  |
| `AUT_OFFNEN_HK_IN_ENTLPOS` | automatique · **OFFNEN** · hayon · **IN** · **ENTLPOS** | `aktiv` · `nicht_aktiv` |  |
| `BAUART` | type de construction | `limousine` · `coupe` · `touring` · `cabrio` · `roadster` · `compact` · `sicherheitsfahrzeug` · `light_protection` |  |
| `BAUREIHE` | **BAUREIHE** | `e65` · `e66` · `e67` · `e60` · `e61` · `e63` · `e64` · `e72` … *(+16)* |  |
| `BEDINGUNG_CC_10KM` | **BEDINGUNG** · Check Control · **10KM** | `aktiv` · `nicht_aktiv` |  |
| `BOS_DATEN_ANFRAGEN` | **BOS** · **DATEN** · **ANFRAGEN** | `aktiv` · `nicht_aktiv` |  |
| `BREMSE_DRUCK` | frein · pression | `wert_01` |  |
| `CAN_MEHRFACH_SICHERN` | bus CAN · **MEHRFACH** · verrouillage | `aktiv` · `nicht_aktiv` |  |
| `CBS_VERSION` | entretien selon besoin (CBS) · **VERSION** | `wert_01` · `wert_02` · `wert_03` · `dynamik_2` |  |
| `CCM_SEND_INVALID` | **CCM** · **SEND** · **INVALID** | `aktiv` · `nicht_aktiv` |  |
| `CHIPCARD` | **CHIPCARD** | `aktiv` · `nicht_aktiv` |  |
| `CLASS_PWR` | **CLASS** · **PWR** | `keine` · `klasse_1` · `klasse_2` · `invalid` · `e92_m` |  |
| `CLT_TASTE` | **CLT** · **TASTE** | `aktiv` · `nicht_aktiv` |  |
| `CRASH_BESCHLEUNIGUNG` | choc · **BESCHLEUNIGUNG** | `wert_01` · `wert_02` |  |
| `CRASH_SCHWERE` | choc · **SCHWERE** | `wert_01` · `wert_02` |  |
| `DELAY_KL50L` | delai · **KL50L** | `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `DELAY_KLR_AUS` | delai · regulation de climatisation · arret | `wert_01` · `wert_02` |  |
| `EE_TRSP_CCM_0CD` | **EE** · **TRSP** · **CCM** · **0CD** | `aktiv` · `nicht_aktiv` |  |
| `EE_TRSP_DEBUG` | **EE** · **TRSP** · deverminage | `aktiv` · `nicht_aktiv` |  |
| `EE_TRSP_PERM` | **EE** · **TRSP** · **PERM** | `aktiv` · `nicht_aktiv` |  |
| `EE_ZEIT_ABFR` | **EE** · duree · **ABFR** | `wert_01` |  |
| `EE_ZEIT_NLF` | **EE** · duree · **NLF** | `wert_01` |  |
| `EINSTIEGSHILFE_DEFAULT` | aide a l'acces · **DEFAULT** | `aktiv` · `nicht_aktiv` |  |
| `EINSTIEGSHILFE_OHNE_NR` | aide a l'acces · sans · numero | `aktiv` · `nicht_aktiv` |  |
| `EINSTIEGSHILFE_PORT` | aide a l'acces · **PORT** | `aktiv` · `nicht_aktiv` |  |
| `EINSTIEGSHILFE_SENDER_1` | aide a l'acces · emetteur · 1 | `aktiv` · `nicht_aktiv` |  |
| `EINSTIEGSHILFE_SENDER_2` | aide a l'acces · emetteur · 2 | `aktiv` · `nicht_aktiv` |  |
| `EINSTIEGSHILFE_SENDER_3` | aide a l'acces · emetteur · 3 | `aktiv` · `nicht_aktiv` |  |
| `EINSTIEGSHILFE_SENDER_4` | aide a l'acces · emetteur · 4 | `aktiv` · `nicht_aktiv` |  |
| `ELV_AUT_FCT` | blocage de colonne de direction · automatique · **FCT** | `aktiv` · `nicht_aktiv` |  |
| `ELV_RESET_FCT` | blocage de colonne de direction · remise a zero · **FCT** | `aktiv` · `nicht_aktiv` |  |
| `ELV_TESTER` | blocage de colonne de direction · **TESTER** | `aktiv` · `nicht_aktiv` |  |
| `ENABLE_CLT_NACH_CRASH` | activation · **CLT** · apres · choc | `aktiv` · `nicht_aktiv` |  |
| `ERROR_EJECT` | **ERROR** · ejection | `aktiv` · `nicht_aktiv` |  |
| `ER_KEYOUT_AUTOVR` | **ER** · **KEYOUT** · **AUTOVR** | `aktiv` · `nicht_aktiv` | Deverrouille les portes au retrait de la cle. |
| `ER_TANKKLAPPE_SER` | **ER** · **TANKKLAPPE** · **SER** | `aktiv` · `nicht_aktiv` |  |
| `ER_TANKKLAPPE_VR` | **ER** · **TANKKLAPPE** · **VR** | `aktiv` · `nicht_aktiv` |  |
| `ES_EINMALIG_CLT` | **ES** · **EINMALIG** · **CLT** | `aktiv` · `nicht_aktiv` |  |
| `EWS_EGS` | **EWS** · calculateur de boite | `nicht_aktiv` · `aktiv` |  |
| `EWS_FEHLER_MOTORLAUF` | **EWS** · erreur · **MOTORLAUF** | `nicht_aktiv` · `aktiv` |  |
| `EWS_NACHLAUF_KLR_AUS` | **EWS** · marche a suivre / temporisation · regulation de climatisation · arret | `aktiv` · `nicht_aktiv` |  |
| `EWS_SCHRITTWEITE_2` | **EWS** · **SCHRITTWEITE** · 2 | `nicht_aktiv` · `aktiv` |  |
| `FBDIN_9_6_KBAUD` | **FBDIN** · 9 · 6 · **KBAUD** | `aktiv` · `nicht_aktiv` |  |
| `FBD_AKTIV_KEYIN` | telecommande · actif · **KEYIN** | `aktiv` · `nicht_aktiv` |  |
| `FBD_AKTIV_KL15AUS` | telecommande · actif · **KL15AUS** | `aktiv` · `nicht_aktiv` |  |
| `FBD_DISFERTMODE` | telecommande · **DISFERTMODE** | `aktiv` · `nicht_aktiv` |  |
| `FBD_DISTRANSMODE` | telecommande · **DISTRANSMODE** | `aktiv` · `nicht_aktiv` |  |
| `FBD_DISWERKSTMODE` | telecommande · **DISWERKSTMODE** | `aktiv` · `nicht_aktiv` |  |
| `FBD_HECKKLAPPE_TOTZEIT` | telecommande · hayon · **TOTZEIT** | `wert_01` · `wert_02` |  |
| `FBD_HKL_NUR_ER` | telecommande · hayon · **NUR** · **ER** | `aktiv` · `nicht_aktiv` |  |
| `FBD_HSF_0` | telecommande · toit ouvrant · 0 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` · `wert_09` |  |
| `FBD_HSF_1` | telecommande · toit ouvrant · 1 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` · `wert_09` |  |
| `FBD_LOAD_POS_HK_HK` | telecommande · charge · position · hayon · hayon | `aktiv` · `nicht_aktiv` |  |
| `FBD_MOTORLAUF` | telecommande · **MOTORLAUF** | `wert_01` |  |
| `FBD_MOTORSTART` | telecommande · **MOTORSTART** | `wert_01` |  |
| `FBD_NO_LOAD_POS_C06` | telecommande · non · charge · position · [variante C06] | `aktiv` · `nicht_aktiv` |  |
| `FBD_PANIKDAUER` | telecommande · **PANIKDAUER** | `wert_01` · `wert_02` · `wert_03` |  |
| `FBD_PANIK_TOTZEIT` | telecommande · **PANIK** · **TOTZEIT** | `wert_01` |  |
| `FBD_SERVICEFKT_AKTIV` | telecommande · **SERVICEFKT** · actif | `aktiv` · `nicht_aktiv` |  |
| `FBD_TESTKEY_AKTIV` | telecommande · **TESTKEY** · actif | `aktiv` · `nicht_aktiv` |  |
| `FHM_CA_OHNE_EKS` | **FHM** · **CA** · sans · systeme electronique de cle | `aktiv` · `nicht_aktiv` |  |
| `FHM_CA_OHNE_EKS_DELAY` | **FHM** · **CA** · sans · systeme electronique de cle · delai | `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `FH_ABSCHALTKRITERIUM` | leve-vitre · **ABSCHALTKRITERIUM** | `aktiv` · `nicht_aktiv` |  |
| `FH_CABRIO_VERDECK_MODUL` | leve-vitre · **CABRIO** · **VERDECK** · **MODUL** | `aktiv` · `nicht_aktiv` |  |
| `FH_DISAKL15` | leve-vitre · **DISAKL15** | `aktiv` · `nicht_aktiv` |  |
| `FH_DISAKLR` | leve-vitre · **DISAKLR** | `aktiv` · `nicht_aktiv` |  |
| `FH_DISASHD_KL15` | leve-vitre · **DISASHD** · borne 15 (contact) | `aktiv` · `nicht_aktiv` |  |
| `FH_DISCLSYDEFECT` | leve-vitre · **DISCLSYDEFECT** | `aktiv` · `nicht_aktiv` |  |
| `FH_DISFBDDEFCT` | leve-vitre · **DISFBDDEFCT** | `aktiv` · `nicht_aktiv` |  |
| `FH_DISFERTMODE` | leve-vitre · **DISFERTMODE** | `aktiv` · `nicht_aktiv` |  |
| `FH_DISTGDEFECT` | leve-vitre · **DISTGDEFECT** | `aktiv` · `nicht_aktiv` |  |
| `FH_DISTRANSMODE` | leve-vitre · **DISTRANSMODE** | `aktiv` · `nicht_aktiv` |  |
| `FH_DISWERKSTMODE` | leve-vitre · **DISWERKSTMODE** | `aktiv` · `nicht_aktiv` |  |
| `FH_ECEFIN` | leve-vitre · **ECEFIN** | `aktiv` · `nicht_aktiv` |  |
| `FH_HINTEN` | leve-vitre · arriere | `aktiv` · `nicht_aktiv` |  |
| `FH_KINDERSICHERUNG` | leve-vitre · **KINDERSICHERUNG** | `aktiv` · `nicht_aktiv` |  |
| `FH_KMFRTFBDTIM` | leve-vitre · **KMFRTFBDTIM** | `aktiv` · `nicht_aktiv` |  |
| `FH_KOMFORTF_MIT_SUBST` | leve-vitre · **KOMFORTF** · avec · **SUBST** | `aktiv` · `nicht_aktiv` | Fermeture confort des vitres avec substitution. |
| `FH_RELEASE_CA_BASIC` | leve-vitre · **RELEASE** · **CA** · **BASIC** | `aktiv` · `nicht_aktiv` |  |
| `FH_RESETTIMEOUTKL50` | leve-vitre · **RESETTIMEOUTKL50** | `aktiv` · `nicht_aktiv` |  |
| `FH_SCHLIESSEN_EINSTG` | leve-vitre · fermeture · **EINSTG** | `aktiv` · `nicht_aktiv` |  |
| `FH_SICHERHEITSFAHRZEUG` | leve-vitre · **SICHERHEITSFAHRZEUG** | `aktiv` · `nicht_aktiv` |  |
| `FH_TUERBED_EINSTG` | leve-vitre · **TUERBED** · **EINSTG** | `aktiv` · `nicht_aktiv` |  |
| `FH_UMLSCH` | leve-vitre · **UMLSCH** | `aktiv` · `nicht_aktiv` |  |
| `FH_USACANADA` | leve-vitre · **USACANADA** | `aktiv` · `nicht_aktiv` |  |
| `FS_DATEN_ANFRAGEN` | **FS** · **DATEN** · **ANFRAGEN** | `aktiv` · `nicht_aktiv` |  |
| `GESCHW_REQ_CC` | vitesse · **REQ** · Check Control | `wert_01` · `wert_02` |  |
| `GESCHW_SPERRE_TOEHK` | vitesse · blocage · _TOEHK_ | `wert_01` |  |
| `GETRIEBE_GAENGE` | boite de vitesses · **GAENGE** | — |  |
| `GETRIEBE_TYP_C6` | boite de vitesses · type · **C6** | `handschaltung` · `automatik` · `schrittschaltung` · `ssg_schaltung` · `dkg_schaltung` |  |
| `HECKSCHEIBE_ENTR_SENDER_1` | lunette arriere · deverrouillage · emetteur · 1 | `aktiv` · `nicht_aktiv` |  |
| `HECKSCHEIBE_ENTR_SENDER_2` | lunette arriere · deverrouillage · emetteur · 2 | `aktiv` · `nicht_aktiv` |  |
| `HECKSCHEIBE_ENTR_SENDER_3` | lunette arriere · deverrouillage · emetteur · 3 | `aktiv` · `nicht_aktiv` |  |
| `HECKSCHEIBE_ENTR_SENDER_4` | lunette arriere · deverrouillage · emetteur · 4 | `aktiv` · `nicht_aktiv` |  |
| `HECKSCHEI_ENTR_OHNE_NR` | **HECKSCHEI** · deverrouillage · sans · numero | `aktiv` · `nicht_aktiv` |  |
| `HKAT_EO_HS_OHNE_NR` | catalyseur · _EO_ · **HS** · sans · numero | `aktiv` · `nicht_aktiv` |  |
| `HKAT_EO_HS_SCHL_NR01` | catalyseur · _EO_ · **HS** · cle / fermeture · **NR01** | `aktiv` · `nicht_aktiv` |  |
| `HKAT_EO_HS_SCHL_NR02` | catalyseur · _EO_ · **HS** · cle / fermeture · **NR02** | `aktiv` · `nicht_aktiv` |  |
| `HKAT_EO_HS_SCHL_NR03` | catalyseur · _EO_ · **HS** · cle / fermeture · **NR03** | `aktiv` · `nicht_aktiv` |  |
| `HKAT_EO_HS_SCHL_NR04` | catalyseur · _EO_ · **HS** · cle / fermeture · **NR04** | `aktiv` · `nicht_aktiv` |  |
| `HKL_LIFT_BEI_CA` | hayon · **LIFT** · lors de · **CA** | `aktiv` · `nicht_aktiv` |  |
| `HK_ENABLE_OPEN` | hayon · activation · ouvert | `aktiv` · `nicht_aktiv` |  |
| `HK_GESPERRT_IN_GESICHERT` | hayon · **GESPERRT** · **IN** · **GESICHERT** | `aktiv` · `nicht_aktiv` |  |
| `HS_ENABLE_OPEN` | **HS** · activation · ouvert | `aktiv` · `nicht_aktiv` |  |
| `KISITMRMAX` | **KISITMRMAX** | `wert_01` |  |
| `KL15HW01_ALLWAYS` | **KL15HW01** · **ALLWAYS** | `aktiv` · `nicht_aktiv` |  |
| `KL30G_EIN_HKT_AUSSEN` | **KL30G** · marche · **HKT** · exterieur | `aktiv` · `nicht_aktiv` |  |
| `KL30G_EIN_HKT_FBD` | **KL30G** · marche · **HKT** · telecommande | `aktiv` · `nicht_aktiv` |  |
| `KL30G_NACHLAUF_NACH_SZ1` | **KL30G** · marche a suivre / temporisation · apres · **SZ1** | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` |  |
| `KL30G_TIMEOUT` | **KL30G** · delai d'attente | `wert_01` · `wert_02` · `wert_03` |  |
| `KLASSE_BATTERIE` | **KLASSE** · batterie | `wert_00` |  |
| `KLR_AUS_TIMEOUT` | regulation de climatisation · arret · delai d'attente | `wert_01` · `wert_02` · `wert_03` |  |
| `KLR_AUS_TKSITZ` | regulation de climatisation · arret · **TKSITZ** | `aktiv` · `nicht_aktiv` |  |
| `KLR_EIN_GESCHW` | regulation de climatisation · marche · vitesse | `nicht_aktiv` · `aktiv` |  |
| `KLSTRG_DISFERTMODE` | commande de climatisation · **DISFERTMODE** | `aktiv` · `nicht_aktiv` |  |
| `KLSTRG_DISTRANSMODE` | commande de climatisation · **DISTRANSMODE** | `aktiv` · `nicht_aktiv` |  |
| `KLSTRG_DISWERKSTMODE` | commande de climatisation · **DISWERKSTMODE** | `aktiv` · `nicht_aktiv` |  |
| `KLSTRG_TESTER` | commande de climatisation · **TESTER** | `aktiv` · `nicht_aktiv` |  |
| `KMFRT_OEFFNEN` | confort · ouverture | `wert_01` |  |
| `KMFRT_OEFF_HINTEN` | confort · **OEFF** · arriere | `wert_01` · `wert_02` |  |
| `KMFRT_OEFF_SHD` | confort · **OEFF** · toit ouvrant | `wert_01` · `wert_02` |  |
| `KMFRT_SCHLIESSEN` | confort · fermeture | `wert_01` | Temporisation avant rabattement des retroviseurs : 00 immediat, 05 = 0,5 s, 0A = 1 s, 0F = 2,5 s. |
| `KMFRT_SCHL_VORNE` | confort · cle / fermeture · avant | `wert_01` |  |
| `KOMFORTOEFFNUNG` | **KOMFORTOEFFNUNG** | `aktiv` · `nicht_aktiv` |  |
| `KOMFORTOEFFNUNG_FB` | **KOMFORTOEFFNUNG** · telecommande | `aktiv` · `nicht_aktiv` |  |
| `KOMFORTSCHLIESSUNG` | fermeture confort | `aktiv` · `nicht_aktiv` |  |
| `KOMFORTSCHLIESSUNG_FB` | fermeture confort · telecommande | `aktiv` · `nicht_aktiv` | Fermeture des vitres et du toit par maintien de la telecommande. |
| `KOMFORTSCHLIESSUNG_PA` | fermeture confort · _PA_ | `aktiv` · `nicht_aktiv` | Fermeture confort par le contacteur de porte. |
| `KOMFORTSTART` | **KOMFORTSTART** | `aktiv` · `nicht_aktiv` |  |
| `KOMFORT_AUS_IN_50MS` | confort · arret · **IN** · **50MS** | `wert_02` · `wert_04` · `wert_05` · `wert_01` · `wert_03` |  |
| `KOMFORT_EIN` | confort · marche | `aktiv` · `nicht_aktiv` |  |
| `KOMFORT_EJECT` | confort · ejection | `aktiv` · `nicht_aktiv` | Ejection automatique de la cle du neiman. |
| `KOMFORT_SCHL_SHD` | confort · cle / fermeture · toit ouvrant | `wert_01` · `wert_02` |  |
| `LAENDERVARIANTE_BNDB` | variante pays · **BNDB** | `ece` · `us` · `deutschland` · `gb` · `japan` · `australien` · `korea` · `brasilien` … *(+13)* |  |
| `MAX_KMFRT_UNTERB` | maximum · confort · **UNTERB** | `wert_01` |  |
| `MAX_KMFRT_UNTERB_FBD` | maximum · confort · **UNTERB** · telecommande | `wert_01` |  |
| `MFS_KEY_PLGD` | **MFS** · **KEY** · **PLGD** | `aktiv` · `nicht_aktiv` |  |
| `MFS_MIT_TON` | **MFS** · avec · **TON** | `aktiv` · `nicht_aktiv` |  |
| `MIND_KMFRT_NACH_UNTERB` | minimum · confort · apres · **UNTERB** | `wert_01` |  |
| `MOTORFERNSTART_CAN` | **MOTORFERNSTART** · bus CAN | `aktiv` · `nicht_aktiv` |  |
| `MOTORFERNSTART_GENERELL` | **MOTORFERNSTART** · **GENERELL** | `aktiv` · `nicht_aktiv` |  |
| `MOTORHAUBENKONTAKT` | **MOTORHAUBENKONTAKT** | `aktiv` · `nicht_aktiv` |  |
| `MOTOR_ART` | moteur · type | `benzin_direkt` · `benzin_normal` · `diesel_direkt` |  |
| `MOTOR_HUBRAUM` | moteur · **HUBRAUM** | `n43b16` · `n43b20` · `n45b16` · `n45b20` · `n46b20` · `m47d20` · `n47d20` · `n52b24` … *(+34)* |  |
| `PASSIVE_ACCESS` | **PASSIVE** · **ACCESS** | `aktiv` · `nicht_aktiv` |  |
| `PA_CHIPCARD_ACTIVE` | _PA_ · **CHIPCARD** · actif | `aktiv` · `nicht_aktiv` |  |
| `PA_CONFIG_FBDREFCHECK` | _PA_ · **CONFIG** · **FBDREFCHECK** | `aktiv` · `nicht_aktiv` |  |
| `PA_DISABLE_HK_REV` | _PA_ · desactivation · hayon · revision | `aktiv` · `nicht_aktiv` |  |
| `PA_FBD_REF` | _PA_ · telecommande · reference | `wert_01` |  |
| `PA_HA_SEARCH_ACTIVE` | _PA_ · essieu arriere · recherche · actif | `aktiv` · `nicht_aktiv` |  |
| `PA_HK_BACK_IR_SEARCH` | _PA_ · hayon · **BACK** · **IR** · recherche | `aktiv` · `nicht_aktiv` |  |
| `PA_HK_INTERIOR_SEARCH` | _PA_ · hayon · **INTERIOR** · recherche | `aktiv` · `nicht_aktiv` |  |
| `PA_HK_PGS_SEARCH` | _PA_ · hayon · **PGS** · recherche | `aktiv` · `nicht_aktiv` |  |
| `PA_HK_REOPEN_CTL_DEP` | _PA_ · hayon · **REOPEN** · **CTL** · **DEP** | `aktiv` · `nicht_aktiv` |  |
| `PA_HK_REOPEN_CTR_LOCK` | _PA_ · hayon · **REOPEN** · **CTR** · blocage | `aktiv` · `nicht_aktiv` |  |
| `PA_HK_RSCR_ACTIVE` | _PA_ · hayon · **RSCR** · actif | `aktiv` · `nicht_aktiv` |  |
| `PA_ID_GEBER` | _PA_ · identifiant · **GEBER** | `wert_01` · `wert_02` |  |
| `PA_SILENTTIMEOUT` | _PA_ · **SILENTTIMEOUT** | `wert_01` |  |
| `PA_TBEREITPE` | _PA_ · **TBEREITPE** | `wert_01` |  |
| `PA_TBEREIT_PG` | _PA_ · **TBEREIT** · **PG** | `wert_01` · `wert_02` |  |
| `PA_TRUNK_HIGH` | _PA_ · **TRUNK** · haut | `aktiv` · `nicht_aktiv` |  |
| `PA_TSPERRVR` | _PA_ · **TSPERRVR** | `wert_01` |  |
| `PA_TVRERFATH` | _PA_ · **TVRERFATH** | `wert_01` |  |
| `PERSONALISIERUNG_AUS` | **PERSONALISIERUNG** · arret | `aktiv` · `nicht_aktiv` |  |
| `RSSI_THRESHOLD` | **RSSI** · **THRESHOLD** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `SCHIEBEHEBEDACH` | toit ouvrant relevable | `aktiv` · `nicht_aktiv` |  |
| `SEL_ZV_SENDER_1` | selection · verrouillage centralise · emetteur · 1 | `aktiv` · `nicht_aktiv` |  |
| `SEL_ZV_SENDER_2` | selection · verrouillage centralise · emetteur · 2 | `aktiv` · `nicht_aktiv` |  |
| `SEL_ZV_SENDER_3` | selection · verrouillage centralise · emetteur · 3 | `aktiv` · `nicht_aktiv` |  |
| `SEL_ZV_SENDER_4` | selection · verrouillage centralise · emetteur · 4 | `aktiv` · `nicht_aktiv` |  |
| `SEL_ZV_SENDER_DEFAULT` | selection · verrouillage centralise · emetteur · **DEFAULT** | `aktiv` · `nicht_aktiv` |  |
| `SEL_ZV_SENDER_PORT` | selection · verrouillage centralise · emetteur · **PORT** | `aktiv` · `nicht_aktiv` |  |
| `SENDMHKCCMLIMIT` | **SENDMHKCCMLIMIT** | `wert_01` · `wert_02` |  |
| `SICHERHEITSFAHRZEUG` | **SICHERHEITSFAHRZEUG** | `aktiv` · `nicht_aktiv` |  |
| `SLEEPTIMER_FH_KLRAUS` | **SLEEPTIMER** · leve-vitre · **KLRAUS** | `wert_01` · `wert_02` · `wert_03` |  |
| `SONDERTASTE_INNENLICHT` | **SONDERTASTE** · **INNENLICHT** | `innenlicht_ein` · `nicht_aktiv` |  |
| `SONDERTASTE_PANIKALARM` | **SONDERTASTE** · **PANIKALARM** | `panikalarm` · `nicht_aktiv` |  |
| `SPERRE_CRASH_ER` | blocage · choc · **ER** | `aktiv` · `nicht_aktiv` |  |
| `SPERRE_HECKKLAPPE_KLR` | blocage · hayon · regulation de climatisation | `aktiv` · `nicht_aktiv` |  |
| `SPERRE_HKL_KEY_VALID` | blocage · hayon · **KEY** · **VALID** | `aktiv` · `nicht_aktiv` |  |
| `SPERRE_KLRAUS_SST` | blocage · **KLRAUS** · _SST_ | `aktiv` · `nicht_aktiv` |  |
| `SQCNT_UPDATE` | **SQCNT** · **UPDATE** | `aktiv` · `nicht_aktiv` |  |
| `SS_GETRIEBE` | **SS** · boite de vitesses | `aktiv` · `nicht_aktiv` |  |
| `STARTABBRUCH_BREMSE` | **STARTABBRUCH** · frein | `aktiv` · `nicht_aktiv` |  |
| `STARTABBRUCH_P` | **STARTABBRUCH** · P | `aktiv` · `nicht_aktiv` |  |
| `STARTABBR_KLR` | **STARTABBR** · regulation de climatisation | `aktiv` · `nicht_aktiv` |  |
| `STARTDELAY_K_START` | **STARTDELAY** · K · demarrage | `aktiv` · `nicht_aktiv` |  |
| `STARTWUNSCH_KL50` | **STARTWUNSCH** · borne 50 (demarreur) | `aktiv` · `nicht_aktiv` |  |
| `STROM_KS_MAX` | courant · **KS** · maximum | `wert_01` · `ewp` · `dynamik_2` · `hifi_mulf_e91` · `mit_fl_und_tfl` · `e92_m` · `tvm_isdb_t` · `wert_02` · `wert_03` |  |
| `STROM_KS_MIN` | courant · **KS** · minimum | `wert_01` |  |
| `STROM_RUHE` | courant · **RUHE** | `wert_01` |  |
| `TEST_DUTY_CYCLE` | **TEST** · rapport cyclique · **CYCLE** | `aktiv` · `nicht_aktiv` |  |
| `TEST_LED_KS_AKTIV` | **TEST** · LED · **KS** · actif | `nicht_aktiv` · `aktiv` |  |
| `TOEHK_EO_HS_OHNE_NR` | _TOEHK_ · _EO_ · **HS** · sans · numero | `aktiv` · `nicht_aktiv` |  |
| `TOEHK_EO_HS_SCHL_NR01` | _TOEHK_ · _EO_ · **HS** · cle / fermeture · **NR01** | `aktiv` · `nicht_aktiv` |  |
| `TOEHK_EO_HS_SCHL_NR02` | _TOEHK_ · _EO_ · **HS** · cle / fermeture · **NR02** | `aktiv` · `nicht_aktiv` |  |
| `TOEHK_EO_HS_SCHL_NR03` | _TOEHK_ · _EO_ · **HS** · cle / fermeture · **NR03** | `aktiv` · `nicht_aktiv` |  |
| `TOEHK_EO_HS_SCHL_NR04` | _TOEHK_ · _EO_ · **HS** · cle / fermeture · **NR04** | `aktiv` · `nicht_aktiv` |  |
| `TRANSPONDER` | **TRANSPONDER** | `aktiv` · `nicht_aktiv` |  |
| `TYP_LENKUNG` | type · **LENKUNG** | `linkslenker` · `rechtslenker` |  |
| `VERRIEGELUNGSSCHWELLE` | **VERRIEGELUNGSSCHWELLE** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `VER_AUT_X_MIN_DEFAULT` | version · automatique · X · minimum · **DEFAULT** | `aktiv` · `nicht_aktiv` |  |
| `VER_AUT_X_MIN_OHNE_NR` | version · automatique · X · minimum · sans · numero | `aktiv` · `nicht_aktiv` |  |
| `VER_AUT_X_MIN_PORT` | version · automatique · X · minimum · **PORT** | `aktiv` · `nicht_aktiv` |  |
| `VER_AUT_X_MIN_SENDER_1` | version · automatique · X · minimum · emetteur · 1 | `aktiv` · `nicht_aktiv` |  |
| `VER_AUT_X_MIN_SENDER_2` | version · automatique · X · minimum · emetteur · 2 | `aktiv` · `nicht_aktiv` |  |
| `VER_AUT_X_MIN_SENDER_3` | version · automatique · X · minimum · emetteur · 3 | `aktiv` · `nicht_aktiv` |  |
| `VER_AUT_X_MIN_SENDER_4` | version · automatique · X · minimum · emetteur · 4 | `aktiv` · `nicht_aktiv` |  |
| `VR_GESCHW_EINMALIG` | **VR** · vitesse · **EINMALIG** | `aktiv` · `nicht_aktiv` |  |
| `VR_KLR` | **VR** · regulation de climatisation | `aktiv` · `nicht_aktiv` |  |
| `VR_KLR_EINMALIG` | **VR** · regulation de climatisation · **EINMALIG** | `aktiv` · `nicht_aktiv` |  |
| `ZEIT_AUTO_ZV_VR` | duree · automatique · verrouillage centralise · **VR** | `wert_01` |  |
| `ZEIT_BREMSE` | duree · frein | `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `ZEIT_KL50_ANSTRG` | duree · borne 50 (demarreur) · **ANSTRG** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `ZV_AUTHENTISIERUNG` | verrouillage centralise · **AUTHENTISIERUNG** | `aktiv` · `nicht_aktiv` |  |
| `ZV_DISFERTMODE` | verrouillage centralise · **DISFERTMODE** | `aktiv` · `nicht_aktiv` |  |
| `ZV_DISTRANSMODE` | verrouillage centralise · **DISTRANSMODE** | `aktiv` · `nicht_aktiv` |  |
| `ZV_DISWERKSTMODE` | verrouillage centralise · **DISWERKSTMODE** | `aktiv` · `nicht_aktiv` |  |
| `ZV_SELEKTIV` | verrouillage centralise · **SELEKTIV** | `aktiv` · `nicht_aktiv` |  |
| `ZYLINDER_ZAHL` | **ZYLINDER** · **ZAHL** | — |  |

---

**Traduction** : décomposition du nom allemand — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments en **gras** n'ont pas de traduction établie, ceux entre `_underscores_` sont ambigus.

**Valeurs possibles** : extraites des DATEN. `aktiv`/`nicht_aktiv` = interrupteur,
`wert_xx` = valeur numérique, autres = choix nommés.

**Explication** : rédigée uniquement pour les fonctions identifiées avec certitude.
Une case vide ne veut pas dire que la fonction est sans intérêt — seulement que son effet
n'a pas été établi de source sûre.
