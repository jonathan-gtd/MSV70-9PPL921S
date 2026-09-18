# CAS — Acces et antidemarrage

🔴 Module `CAS` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🔴 **Module a ne pas coder.** Cette liste est fournie pour comprendre, pas pour modifier.

---

## Source

| | |
|---|---|
| Variante retenue | `CAS3.C06` |
| FSW dans cette variante | **229** |
| FSW toutes variantes confondues | 270 |
| Fichiers DATEN analyses | 3 |

Variantes disponibles : `CAS3.C06`, `CAS3.C08`, `CAS3.C09`

> La variante reellement installee sur le vehicule est identifiee par NCS Dummy
> a l'ouverture du `.TRC`. Si elle differe de celle retenue ici, la liste change.

---

## Les 229 fonctions codables

| FSW | Traduction |
|---|---|
| `ABZUGSPERRE_PLOCK_C6` | **ABZUGSPERRE** · **PLOCK** · **C6** |
| `ANLASSERSCHUTZ_3SEK` | **ANLASSERSCHUTZ** · **3SEK** |
| `ANLASSERSPERRE_KPS` | **ANLASSERSPERRE** · **KPS** |
| `ANLASSERSPERRE_P` | **ANLASSERSPERRE** · P |
| `ANLASSSPERRE_BLS` | blocage du demarreur · **BLS** |
| `ANLASSSPERRE_KLR` | blocage du demarreur · regulation de climatisation |
| `ANSTEUERZEIT` | duree de commande |
| `AUSWERTUNG_B_DRUCK` | **AUSWERTUNG** · B · pression |
| `AUSWERTUNG_P_CAN` | **AUSWERTUNG** · P · bus CAN |
| `AUSWERTUNG_P_HW` | **AUSWERTUNG** · P · **HW** |
| `AUT_OFFNEN_HK_IN_ENTLPOS` | automatique · **OFFNEN** · hayon · **IN** · **ENTLPOS** |
| `BAUART` | type de construction |
| `BAUREIHE` | **BAUREIHE** |
| `BEDINGUNG_CC_10KM` | **BEDINGUNG** · Check Control · **10KM** |
| `BOS_DATEN_ANFRAGEN` | **BOS** · **DATEN** · **ANFRAGEN** |
| `BREMSE_DRUCK` | frein · pression |
| `CAN_MEHRFACH_SICHERN` | bus CAN · **MEHRFACH** · verrouillage |
| `CBS_VERSION` | entretien selon besoin (CBS) · **VERSION** |
| `CCM_SEND_INVALID` | **CCM** · **SEND** · **INVALID** |
| `CHIPCARD` | **CHIPCARD** |
| `CLASS_PWR` | **CLASS** · **PWR** |
| `CLT_TASTE` | **CLT** · **TASTE** |
| `CRASH_BESCHLEUNIGUNG` | choc · **BESCHLEUNIGUNG** |
| `CRASH_SCHWERE` | choc · **SCHWERE** |
| `DELAY_KL50L` | delai · **KL50L** |
| `DELAY_KLR_AUS` | delai · regulation de climatisation · arret |
| `EE_TRSP_CCM_0CD` | **EE** · **TRSP** · **CCM** · **0CD** |
| `EE_TRSP_DEBUG` | **EE** · **TRSP** · deverminage |
| `EE_TRSP_PERM` | **EE** · **TRSP** · **PERM** |
| `EE_ZEIT_ABFR` | **EE** · duree · **ABFR** |
| `EE_ZEIT_NLF` | **EE** · duree · **NLF** |
| `EINSTIEGSHILFE_DEFAULT` | aide a l'acces · **DEFAULT** |
| `EINSTIEGSHILFE_OHNE_NR` | aide a l'acces · sans · numero |
| `EINSTIEGSHILFE_PORT` | aide a l'acces · **PORT** |
| `EINSTIEGSHILFE_SENDER_1` | aide a l'acces · emetteur · 1 |
| `EINSTIEGSHILFE_SENDER_2` | aide a l'acces · emetteur · 2 |
| `EINSTIEGSHILFE_SENDER_3` | aide a l'acces · emetteur · 3 |
| `EINSTIEGSHILFE_SENDER_4` | aide a l'acces · emetteur · 4 |
| `ELV_AUT_FCT` | blocage de colonne de direction · automatique · **FCT** |
| `ELV_RESET_FCT` | blocage de colonne de direction · remise a zero · **FCT** |
| `ELV_TESTER` | blocage de colonne de direction · **TESTER** |
| `ENABLE_CLT_NACH_CRASH` | activation · **CLT** · apres · choc |
| `ERROR_EJECT` | **ERROR** · ejection |
| `ER_KEYOUT_AUTOVR` | **ER** · **KEYOUT** · **AUTOVR** |
| `ER_TANKKLAPPE_SER` | **ER** · **TANKKLAPPE** · **SER** |
| `ER_TANKKLAPPE_VR` | **ER** · **TANKKLAPPE** · **VR** |
| `ES_EINMALIG_CLT` | **ES** · **EINMALIG** · **CLT** |
| `EWS_EGS` | **EWS** · calculateur de boite |
| `EWS_FEHLER_MOTORLAUF` | **EWS** · erreur · **MOTORLAUF** |
| `EWS_NACHLAUF_KLR_AUS` | **EWS** · marche a suivre / temporisation · regulation de climatisation · arret |
| `EWS_SCHRITTWEITE_2` | **EWS** · **SCHRITTWEITE** · 2 |
| `FBDIN_9_6_KBAUD` | **FBDIN** · 9 · 6 · **KBAUD** |
| `FBD_AKTIV_KEYIN` | telecommande · actif · **KEYIN** |
| `FBD_AKTIV_KL15AUS` | telecommande · actif · **KL15AUS** |
| `FBD_DISFERTMODE` | telecommande · **DISFERTMODE** |
| `FBD_DISTRANSMODE` | telecommande · **DISTRANSMODE** |
| `FBD_DISWERKSTMODE` | telecommande · **DISWERKSTMODE** |
| `FBD_HECKKLAPPE_TOTZEIT` | telecommande · hayon · **TOTZEIT** |
| `FBD_HKL_NUR_ER` | telecommande · hayon · **NUR** · **ER** |
| `FBD_HSF_0` | telecommande · toit ouvrant · 0 |
| `FBD_HSF_1` | telecommande · toit ouvrant · 1 |
| `FBD_LOAD_POS_HK_HK` | telecommande · charge · position · hayon · hayon |
| `FBD_MOTORLAUF` | telecommande · **MOTORLAUF** |
| `FBD_MOTORSTART` | telecommande · **MOTORSTART** |
| `FBD_NO_LOAD_POS_C06` | telecommande · non · charge · position · [index variante C06] |
| `FBD_PANIKDAUER` | telecommande · **PANIKDAUER** |
| `FBD_PANIK_TOTZEIT` | telecommande · **PANIK** · **TOTZEIT** |
| `FBD_SERVICEFKT_AKTIV` | telecommande · **SERVICEFKT** · actif |
| `FBD_TESTKEY_AKTIV` | telecommande · **TESTKEY** · actif |
| `FHM_CA_OHNE_EKS` | **FHM** · **CA** · sans · systeme electronique de cle |
| `FHM_CA_OHNE_EKS_DELAY` | **FHM** · **CA** · sans · systeme electronique de cle · delai |
| `FH_ABSCHALTKRITERIUM` | leve-vitre · **ABSCHALTKRITERIUM** |
| `FH_CABRIO_VERDECK_MODUL` | leve-vitre · **CABRIO** · **VERDECK** · **MODUL** |
| `FH_DISAKL15` | leve-vitre · **DISAKL15** |
| `FH_DISAKLR` | leve-vitre · **DISAKLR** |
| `FH_DISASHD_KL15` | leve-vitre · **DISASHD** · borne 15 (contact) |
| `FH_DISCLSYDEFECT` | leve-vitre · **DISCLSYDEFECT** |
| `FH_DISFBDDEFCT` | leve-vitre · **DISFBDDEFCT** |
| `FH_DISFERTMODE` | leve-vitre · **DISFERTMODE** |
| `FH_DISTGDEFECT` | leve-vitre · **DISTGDEFECT** |
| `FH_DISTRANSMODE` | leve-vitre · **DISTRANSMODE** |
| `FH_DISWERKSTMODE` | leve-vitre · **DISWERKSTMODE** |
| `FH_ECEFIN` | leve-vitre · **ECEFIN** |
| `FH_HINTEN` | leve-vitre · arriere |
| `FH_KINDERSICHERUNG` | leve-vitre · **KINDERSICHERUNG** |
| `FH_KMFRTFBDTIM` | leve-vitre · **KMFRTFBDTIM** |
| `FH_KOMFORTF_MIT_SUBST` | leve-vitre · **KOMFORTF** · avec · **SUBST** |
| `FH_RELEASE_CA_BASIC` | leve-vitre · **RELEASE** · **CA** · **BASIC** |
| `FH_RESETTIMEOUTKL50` | leve-vitre · **RESETTIMEOUTKL50** |
| `FH_SCHLIESSEN_EINSTG` | leve-vitre · fermeture · **EINSTG** |
| `FH_SICHERHEITSFAHRZEUG` | leve-vitre · **SICHERHEITSFAHRZEUG** |
| `FH_TUERBED_EINSTG` | leve-vitre · **TUERBED** · **EINSTG** |
| `FH_UMLSCH` | leve-vitre · **UMLSCH** |
| `FH_USACANADA` | leve-vitre · **USACANADA** |
| `FS_DATEN_ANFRAGEN` | **FS** · **DATEN** · **ANFRAGEN** |
| `GESCHW_REQ_CC` | vitesse · **REQ** · Check Control |
| `GESCHW_SPERRE_TOEHK` | vitesse · blocage · _TOEHK_ |
| `GETRIEBE_GAENGE` | boite de vitesses · **GAENGE** |
| `GETRIEBE_TYP_C6` | boite de vitesses · type · **C6** |
| `HECKSCHEIBE_ENTR_SENDER_1` | lunette arriere · deverrouillage · emetteur · 1 |
| `HECKSCHEIBE_ENTR_SENDER_2` | lunette arriere · deverrouillage · emetteur · 2 |
| `HECKSCHEIBE_ENTR_SENDER_3` | lunette arriere · deverrouillage · emetteur · 3 |
| `HECKSCHEIBE_ENTR_SENDER_4` | lunette arriere · deverrouillage · emetteur · 4 |
| `HECKSCHEI_ENTR_OHNE_NR` | **HECKSCHEI** · deverrouillage · sans · numero |
| `HKAT_EO_HS_OHNE_NR` | catalyseur · _EO_ · **HS** · sans · numero |
| `HKAT_EO_HS_SCHL_NR01` | catalyseur · _EO_ · **HS** · cle / fermeture · **NR01** |
| `HKAT_EO_HS_SCHL_NR02` | catalyseur · _EO_ · **HS** · cle / fermeture · **NR02** |
| `HKAT_EO_HS_SCHL_NR03` | catalyseur · _EO_ · **HS** · cle / fermeture · **NR03** |
| `HKAT_EO_HS_SCHL_NR04` | catalyseur · _EO_ · **HS** · cle / fermeture · **NR04** |
| `HKL_LIFT_BEI_CA` | hayon · **LIFT** · lors de · **CA** |
| `HK_ENABLE_OPEN` | hayon · activation · ouvert |
| `HK_GESPERRT_IN_GESICHERT` | hayon · **GESPERRT** · **IN** · **GESICHERT** |
| `HS_ENABLE_OPEN` | **HS** · activation · ouvert |
| `KISITMRMAX` | **KISITMRMAX** |
| `KL15HW01_ALLWAYS` | **KL15HW01** · **ALLWAYS** |
| `KL30G_EIN_HKT_AUSSEN` | **KL30G** · marche · **HKT** · exterieur |
| `KL30G_EIN_HKT_FBD` | **KL30G** · marche · **HKT** · telecommande |
| `KL30G_NACHLAUF_NACH_SZ1` | **KL30G** · marche a suivre / temporisation · apres · **SZ1** |
| `KL30G_TIMEOUT` | **KL30G** · delai d'attente |
| `KLASSE_BATTERIE` | **KLASSE** · batterie |
| `KLR_AUS_TIMEOUT` | regulation de climatisation · arret · delai d'attente |
| `KLR_AUS_TKSITZ` | regulation de climatisation · arret · **TKSITZ** |
| `KLR_EIN_GESCHW` | regulation de climatisation · marche · vitesse |
| `KLSTRG_DISFERTMODE` | commande de climatisation · **DISFERTMODE** |
| `KLSTRG_DISTRANSMODE` | commande de climatisation · **DISTRANSMODE** |
| `KLSTRG_DISWERKSTMODE` | commande de climatisation · **DISWERKSTMODE** |
| `KLSTRG_TESTER` | commande de climatisation · **TESTER** |
| `KMFRT_OEFFNEN` | confort · ouverture |
| `KMFRT_OEFF_HINTEN` | confort · **OEFF** · arriere |
| `KMFRT_OEFF_SHD` | confort · **OEFF** · toit ouvrant |
| `KMFRT_SCHLIESSEN` | confort · fermeture |
| `KMFRT_SCHL_VORNE` | confort · cle / fermeture · avant |
| `KOMFORTOEFFNUNG` | **KOMFORTOEFFNUNG** |
| `KOMFORTOEFFNUNG_FB` | **KOMFORTOEFFNUNG** · telecommande |
| `KOMFORTSCHLIESSUNG` | fermeture confort |
| `KOMFORTSCHLIESSUNG_FB` | fermeture confort · telecommande |
| `KOMFORTSCHLIESSUNG_PA` | fermeture confort · _PA_ |
| `KOMFORTSTART` | **KOMFORTSTART** |
| `KOMFORT_AUS_IN_50MS` | confort · arret · **IN** · **50MS** |
| `KOMFORT_EIN` | confort · marche |
| `KOMFORT_EJECT` | confort · ejection |
| `KOMFORT_SCHL_SHD` | confort · cle / fermeture · toit ouvrant |
| `LAENDERVARIANTE_BNDB` | variante pays · **BNDB** |
| `MAX_KMFRT_UNTERB` | maximum · confort · **UNTERB** |
| `MAX_KMFRT_UNTERB_FBD` | maximum · confort · **UNTERB** · telecommande |
| `MFS_KEY_PLGD` | **MFS** · **KEY** · **PLGD** |
| `MFS_MIT_TON` | **MFS** · avec · **TON** |
| `MIND_KMFRT_NACH_UNTERB` | minimum · confort · apres · **UNTERB** |
| `MOTORFERNSTART_CAN` | **MOTORFERNSTART** · bus CAN |
| `MOTORFERNSTART_GENERELL` | **MOTORFERNSTART** · **GENERELL** |
| `MOTORHAUBENKONTAKT` | **MOTORHAUBENKONTAKT** |
| `MOTOR_ART` | moteur · type |
| `MOTOR_HUBRAUM` | moteur · **HUBRAUM** |
| `PASSIVE_ACCESS` | **PASSIVE** · **ACCESS** |
| `PA_CHIPCARD_ACTIVE` | _PA_ · **CHIPCARD** · actif |
| `PA_CONFIG_FBDREFCHECK` | _PA_ · **CONFIG** · **FBDREFCHECK** |
| `PA_DISABLE_HK_REV` | _PA_ · desactivation · hayon · revision |
| `PA_FBD_REF` | _PA_ · telecommande · reference |
| `PA_HA_SEARCH_ACTIVE` | _PA_ · essieu arriere · recherche · actif |
| `PA_HK_BACK_IR_SEARCH` | _PA_ · hayon · **BACK** · **IR** · recherche |
| `PA_HK_INTERIOR_SEARCH` | _PA_ · hayon · **INTERIOR** · recherche |
| `PA_HK_PGS_SEARCH` | _PA_ · hayon · **PGS** · recherche |
| `PA_HK_REOPEN_CTL_DEP` | _PA_ · hayon · **REOPEN** · **CTL** · **DEP** |
| `PA_HK_REOPEN_CTR_LOCK` | _PA_ · hayon · **REOPEN** · **CTR** · blocage |
| `PA_HK_RSCR_ACTIVE` | _PA_ · hayon · **RSCR** · actif |
| `PA_ID_GEBER` | _PA_ · identifiant · **GEBER** |
| `PA_SILENTTIMEOUT` | _PA_ · **SILENTTIMEOUT** |
| `PA_TBEREITPE` | _PA_ · **TBEREITPE** |
| `PA_TBEREIT_PG` | _PA_ · **TBEREIT** · **PG** |
| `PA_TRUNK_HIGH` | _PA_ · **TRUNK** · haut |
| `PA_TSPERRVR` | _PA_ · **TSPERRVR** |
| `PA_TVRERFATH` | _PA_ · **TVRERFATH** |
| `PERSONALISIERUNG_AUS` | **PERSONALISIERUNG** · arret |
| `RSSI_THRESHOLD` | **RSSI** · **THRESHOLD** |
| `SCHIEBEHEBEDACH` | toit ouvrant relevable |
| `SEL_ZV_SENDER_1` | selection · verrouillage centralise · emetteur · 1 |
| `SEL_ZV_SENDER_2` | selection · verrouillage centralise · emetteur · 2 |
| `SEL_ZV_SENDER_3` | selection · verrouillage centralise · emetteur · 3 |
| `SEL_ZV_SENDER_4` | selection · verrouillage centralise · emetteur · 4 |
| `SEL_ZV_SENDER_DEFAULT` | selection · verrouillage centralise · emetteur · **DEFAULT** |
| `SEL_ZV_SENDER_PORT` | selection · verrouillage centralise · emetteur · **PORT** |
| `SENDMHKCCMLIMIT` | **SENDMHKCCMLIMIT** |
| `SICHERHEITSFAHRZEUG` | **SICHERHEITSFAHRZEUG** |
| `SLEEPTIMER_FH_KLRAUS` | **SLEEPTIMER** · leve-vitre · **KLRAUS** |
| `SONDERTASTE_INNENLICHT` | **SONDERTASTE** · **INNENLICHT** |
| `SONDERTASTE_PANIKALARM` | **SONDERTASTE** · **PANIKALARM** |
| `SPERRE_CRASH_ER` | blocage · choc · **ER** |
| `SPERRE_HECKKLAPPE_KLR` | blocage · hayon · regulation de climatisation |
| `SPERRE_HKL_KEY_VALID` | blocage · hayon · **KEY** · **VALID** |
| `SPERRE_KLRAUS_SST` | blocage · **KLRAUS** · _SST_ |
| `SQCNT_UPDATE` | **SQCNT** · **UPDATE** |
| `SS_GETRIEBE` | **SS** · boite de vitesses |
| `STARTABBRUCH_BREMSE` | **STARTABBRUCH** · frein |
| `STARTABBRUCH_P` | **STARTABBRUCH** · P |
| `STARTABBR_KLR` | **STARTABBR** · regulation de climatisation |
| `STARTDELAY_K_START` | **STARTDELAY** · K · demarrage |
| `STARTWUNSCH_KL50` | **STARTWUNSCH** · borne 50 (demarreur) |
| `STROM_KS_MAX` | courant · **KS** · maximum |
| `STROM_KS_MIN` | courant · **KS** · minimum |
| `STROM_RUHE` | courant · **RUHE** |
| `TEST_DUTY_CYCLE` | **TEST** · rapport cyclique · **CYCLE** |
| `TEST_LED_KS_AKTIV` | **TEST** · LED · **KS** · actif |
| `TOEHK_EO_HS_OHNE_NR` | _TOEHK_ · _EO_ · **HS** · sans · numero |
| `TOEHK_EO_HS_SCHL_NR01` | _TOEHK_ · _EO_ · **HS** · cle / fermeture · **NR01** |
| `TOEHK_EO_HS_SCHL_NR02` | _TOEHK_ · _EO_ · **HS** · cle / fermeture · **NR02** |
| `TOEHK_EO_HS_SCHL_NR03` | _TOEHK_ · _EO_ · **HS** · cle / fermeture · **NR03** |
| `TOEHK_EO_HS_SCHL_NR04` | _TOEHK_ · _EO_ · **HS** · cle / fermeture · **NR04** |
| `TRANSPONDER` | **TRANSPONDER** |
| `TYP_LENKUNG` | type · **LENKUNG** |
| `VERRIEGELUNGSSCHWELLE` | **VERRIEGELUNGSSCHWELLE** |
| `VER_AUT_X_MIN_DEFAULT` | version · automatique · X · minimum · **DEFAULT** |
| `VER_AUT_X_MIN_OHNE_NR` | version · automatique · X · minimum · sans · numero |
| `VER_AUT_X_MIN_PORT` | version · automatique · X · minimum · **PORT** |
| `VER_AUT_X_MIN_SENDER_1` | version · automatique · X · minimum · emetteur · 1 |
| `VER_AUT_X_MIN_SENDER_2` | version · automatique · X · minimum · emetteur · 2 |
| `VER_AUT_X_MIN_SENDER_3` | version · automatique · X · minimum · emetteur · 3 |
| `VER_AUT_X_MIN_SENDER_4` | version · automatique · X · minimum · emetteur · 4 |
| `VR_GESCHW_EINMALIG` | **VR** · vitesse · **EINMALIG** |
| `VR_KLR` | **VR** · regulation de climatisation |
| `VR_KLR_EINMALIG` | **VR** · regulation de climatisation · **EINMALIG** |
| `ZEIT_AUTO_ZV_VR` | duree · automatique · verrouillage centralise · **VR** |
| `ZEIT_BREMSE` | duree · frein |
| `ZEIT_KL50_ANSTRG` | duree · borne 50 (demarreur) · **ANSTRG** |
| `ZV_AUTHENTISIERUNG` | verrouillage centralise · **AUTHENTISIERUNG** |
| `ZV_DISFERTMODE` | verrouillage centralise · **DISFERTMODE** |
| `ZV_DISTRANSMODE` | verrouillage centralise · **DISTRANSMODE** |
| `ZV_DISWERKSTMODE` | verrouillage centralise · **DISWERKSTMODE** |
| `ZV_SELEKTIV` | verrouillage centralise · **SELEKTIV** |
| `ZYLINDER_ZAHL` | **ZYLINDER** · **ZAHL** |

---

Les fragments en **gras** n'ont pas de traduction etablie — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments entre `_underscores_` sont ambigus : leur sens depend du module.
