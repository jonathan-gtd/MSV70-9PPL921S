# KOMBI — Combine

🟢 Module `KOMBI` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟢 Module sans fonction de securite. Sauvegarde du `.TRC` quand meme.

---

## Source

| | |
|---|---|
| Variante retenue | `KMBI_PL2.C06` |
| FSW dans cette variante | **237** |
| FSW toutes variantes confondues | 336 |
| Fichiers DATEN analyses | 5 |

Variantes disponibles : `KMBI_PL2.C05`, `KMBI_PL2.C06`, `KMBI_PL2.C07`, `KMBI_PL2.C08`, `KMBI_PL2.C09`

> La variante reellement installee sur le vehicule est identifiee par NCS Dummy
> a l'ouverture du `.TRC`. Si elle differe de celle retenue ici, la liste change.

---

## Les 237 fonctions codables

| FSW | Traduction |
|---|---|
| `ACC_ALIVE_ZAEHLER` | regulateur adaptatif · signal de presence · compteur |
| `ACC_AUSSTATTUNG` | regulateur adaptatif · **AUSSTATTUNG** |
| `ACC_CSUM_MONITOR` | regulateur adaptatif · somme de controle · surveillance |
| `ACC_FILTER_FALLEND` | regulateur adaptatif · filtre · **FALLEND** |
| `ACC_FILTER_STEIGEND` | regulateur adaptatif · filtre · **STEIGEND** |
| `ACC_ID_MONITOR` | regulateur adaptatif · identifiant · surveillance |
| `ACC_ZEIGER_AUF_NULL` | regulateur adaptatif · **ZEIGER** · ouverture · **NULL** |
| `AFS_ALIVE_ZAEHLER` | phares adaptatifs (AFS) · signal de presence · compteur |
| `AFS_ID_MONITOR` | phares adaptatifs (AFS) · identifiant · surveillance |
| `AHM_ALIVE` | **AHM** · signal de presence |
| `ARS_ALIVE_ZAEHLER` | **ARS** · signal de presence · compteur |
| `ARS_ID_MONITOR` | **ARS** · identifiant · surveillance |
| `AUSSENTEMP_ANZ_FEHLER_1` | temperature exterieure · nombre · erreur · 1 |
| `AUSSENTEMP_ANZ_FEHLER_2` | temperature exterieure · nombre · erreur · 2 |
| `AUSSENTEMP_DELTA_K1` | temperature exterieure · ecart · **K1** |
| `AUSSENTEMP_DELTA_K2` | temperature exterieure · ecart · **K2** |
| `AUSSENTEMP_SPERR_FAKTOR` | temperature exterieure · blocage · facteur |
| `AUSSENTEMP_SPERR_OFFSET` | temperature exterieure · blocage · decalage |
| `BC_2_ENABLE` | ordinateur de bord · 2 · activation |
| `BC_DIGITAL_V` | ordinateur de bord · numerique · _V_ |
| `BC_DIGITAL_V_KORREKTUR` | ordinateur de bord · numerique · _V_ · correction |
| `BC_LEERZEILE` | ordinateur de bord · **LEERZEILE** |
| `BC_LIMIT_ENABLE` | ordinateur de bord · limite · activation |
| `BC_V_KORREKTUR` | ordinateur de bord · _V_ · correction |
| `CAS_ALIVE` | **CAS** · signal de presence |
| `CBS_AVKM_START` | entretien selon besoin (CBS) · **AVKM** · demarrage |
| `CBS_EINH_01H` | entretien selon besoin (CBS) · unite · [0x01] |
| `CBS_EINH_02H` | entretien selon besoin (CBS) · unite · [0x02] |
| `CBS_EINH_03H` | entretien selon besoin (CBS) · unite · [0x03] |
| `CBS_EINH_04H` | entretien selon besoin (CBS) · unite · [0x04] |
| `CBS_EINH_06H` | entretien selon besoin (CBS) · unite · [0x06] |
| `CBS_EINH_07H` | entretien selon besoin (CBS) · unite · [0x07] |
| `CBS_EINH_08H` | entretien selon besoin (CBS) · unite · [0x08] |
| `CBS_EINH_10H` | entretien selon besoin (CBS) · unite · [0x10] |
| `CBS_EINH_11H` | entretien selon besoin (CBS) · unite · [0x11] |
| `CBS_EINH_12H` | entretien selon besoin (CBS) · unite · [0x12] |
| `CBS_EINH_13H` | entretien selon besoin (CBS) · unite · [0x13] |
| `CBS_EINH_14H` | entretien selon besoin (CBS) · unite · [0x14] |
| `CBS_EINH_20H` | entretien selon besoin (CBS) · unite · [0x20] |
| `CBS_EINH_21H` | entretien selon besoin (CBS) · unite · [0x21] |
| `CBS_GELB` | entretien selon besoin (CBS) · **GELB** |
| `CBS_GELB_HUAU` | entretien selon besoin (CBS) · **GELB** · **HUAU** |
| `CBS_GELB_KM_MIN` | entretien selon besoin (CBS) · **GELB** · **KM** · minimum |
| `CBS_INTD_03H` | entretien selon besoin (CBS) · **INTD** · [0x03] |
| `CBS_INTD_11H` | entretien selon besoin (CBS) · **INTD** · [0x11] |
| `CBS_INTD_13H` | entretien selon besoin (CBS) · **INTD** · [0x13] |
| `CBS_INTD_14H` | entretien selon besoin (CBS) · **INTD** · [0x14] |
| `CBS_INTW_10H` | entretien selon besoin (CBS) · **INTW** · [0x10] |
| `CBS_INTW_11H` | entretien selon besoin (CBS) · **INTW** · [0x11] |
| `CBS_INTW_12H` | entretien selon besoin (CBS) · **INTW** · [0x12] |
| `CBS_RES_SPERR` | entretien selon besoin (CBS) · remise a zero · blocage |
| `CBS_SCEN_01H` | entretien selon besoin (CBS) · scenario · [0x01] |
| `CBS_SCEN_02H` | entretien selon besoin (CBS) · scenario · [0x02] |
| `CBS_SCEN_03H` | entretien selon besoin (CBS) · scenario · [0x03] |
| `CBS_SCEN_04H` | entretien selon besoin (CBS) · scenario · [0x04] |
| `CBS_SCEN_06H` | entretien selon besoin (CBS) · scenario · [0x06] |
| `CBS_SCEN_07H` | entretien selon besoin (CBS) · scenario · [0x07] |
| `CBS_SCEN_08H` | entretien selon besoin (CBS) · scenario · [0x08] |
| `CBS_SCEN_10H` | entretien selon besoin (CBS) · scenario · [0x10] |
| `CBS_SCEN_11H` | entretien selon besoin (CBS) · scenario · [0x11] |
| `CBS_SCEN_12H` | entretien selon besoin (CBS) · scenario · [0x12] |
| `CBS_SCEN_13H` | entretien selon besoin (CBS) · scenario · [0x13] |
| `CBS_SCEN_14H` | entretien selon besoin (CBS) · scenario · [0x14] |
| `CBS_SCEN_20H` | entretien selon besoin (CBS) · scenario · [0x20] |
| `CBS_SCEN_21H` | entretien selon besoin (CBS) · scenario · [0x21] |
| `CBS_STAT_01H` | entretien selon besoin (CBS) · etat · [0x01] |
| `CBS_STAT_02H` | entretien selon besoin (CBS) · etat · [0x02] |
| `CBS_STAT_03H` | entretien selon besoin (CBS) · etat · [0x03] |
| `CBS_STAT_04H` | entretien selon besoin (CBS) · etat · [0x04] |
| `CBS_STAT_06H` | entretien selon besoin (CBS) · etat · [0x06] |
| `CBS_STAT_07H` | entretien selon besoin (CBS) · etat · [0x07] |
| `CBS_STAT_08H` | entretien selon besoin (CBS) · etat · [0x08] |
| `CBS_STAT_10H` | entretien selon besoin (CBS) · etat · [0x10] |
| `CBS_STAT_11H` | entretien selon besoin (CBS) · etat · [0x11] |
| `CBS_STAT_12H` | entretien selon besoin (CBS) · etat · [0x12] |
| `CBS_STAT_13H` | entretien selon besoin (CBS) · etat · [0x13] |
| `CBS_STAT_14H` | entretien selon besoin (CBS) · etat · [0x14] |
| `CBS_STAT_20H` | entretien selon besoin (CBS) · etat · [0x20] |
| `CBS_STAT_21H` | entretien selon besoin (CBS) · etat · [0x21] |
| `CBS_TZ_SCHALTER` | entretien selon besoin (CBS) · **TZ** · **SCHALTER** |
| `CC_15_AUS` | Check Control · 15 · arret |
| `CC_15_EIN` | Check Control · 15 · marche |
| `CC_BLINKFREQUENZ_LANGSAM` | Check Control · **BLINKFREQUENZ** · **LANGSAM** |
| `CC_BLINKFREQUENZ_SCHNELL` | Check Control · **BLINKFREQUENZ** · **SCHNELL** |
| `CC_BREMSWARNL_NUR_ROT` | Check Control · **BREMSWARNL** · **NUR** · **ROT** |
| `CC_CSUM` | Check Control · somme de controle |
| `CC_ML` | Check Control · **ML** |
| `CC_NR_ROLLE` | Check Control · numero · **ROLLE** |
| `CC_R_AUS` | Check Control · _R_ · arret |
| `CC_R_EIN` | Check Control · _R_ · marche |
| `CC_TEMPERATURWARNUNG` | Check Control · **TEMPERATURWARNUNG** |
| `CC_ZUENDSCHLUESSEL` | Check Control · **ZUENDSCHLUESSEL** |
| `CODING_VALID_BMW` | **CODING** · **VALID** · BMW |
| `CRASH_TH_DIMMUNG` | choc · **TH** · **DIMMUNG** |
| `CSUM_ACC` | somme de controle · regulateur adaptatif |
| `CSUM_AKUSTIK_KLANG` | somme de controle · **AKUSTIK** · sonorite |
| `CSUM_ATEMP` | somme de controle · **ATEMP** |
| `CSUM_AUSSTATTUNG_1` | somme de controle · **AUSSTATTUNG** · 1 |
| `CSUM_AUSSTATTUNG_PL2` | somme de controle · **AUSSTATTUNG** · **PL2** |
| `CSUM_CBS_01` | somme de controle · entretien selon besoin (CBS) · 01 |
| `CSUM_CC_ALIVE` | somme de controle · Check Control · signal de presence |
| `CSUM_KODIERUNG_VALID` | somme de controle · **KODIERUNG** · **VALID** |
| `CSUM_ST_ANZEIGE_1` | somme de controle · **ST** · affichage · 1 |
| `CSUM_TANK_INPUT` | somme de controle · reservoir · entree |
| `CSUM_TANK_PROZESS` | somme de controle · reservoir · **PROZESS** |
| `CSUM_WEG` | somme de controle · course |
| `DIMM_MN_MAX` | gradation · **MN** · maximum |
| `DIMM_MN_MIN` | gradation · **MN** · minimum |
| `DME_ALIVE_ZAEHLER` | **DME** · signal de presence · compteur |
| `DME_ID_MONITOR` | **DME** · identifiant · surveillance |
| `DSC_ALIVE_ZAEHLER` | controle de stabilite · signal de presence · compteur |
| `DSC_ALLRAD_CC_WAHL` | controle de stabilite · **ALLRAD** · Check Control · **WAHL** |
| `DSC_ID_MONITOR` | controle de stabilite · identifiant · surveillance |
| `DSC_MIT_RPA_CC` | controle de stabilite · avec · indicateur de crevaison · Check Control |
| `DWA_VERBAUT` | alarme antivol · installe |
| `EGS_ALIVE_ZAEHLER` | calculateur de boite · signal de presence · compteur |
| `EGS_CSUM_MONITOR` | calculateur de boite · somme de controle · surveillance |
| `EGS_ID_MONITOR` | calculateur de boite · identifiant · surveillance |
| `EHC_ALIVE` | **EHC** · signal de presence |
| `EKP_ALIVE_ZAEHLER` | pompe a carburant · signal de presence · compteur |
| `EKP_ID_MONITOR` | pompe a carburant · identifiant · surveillance |
| `EPS_ALIVE_MONITOR` | **EPS** · signal de presence · surveillance |
| `EPS_ID_MONITOR` | **EPS** · identifiant · surveillance |
| `FB_ALLE_TUEREN` | telecommande · **ALLE** · portes |
| `FB_FA_TUER` | telecommande · conducteur · porte |
| `FDC_ID_MONITOR` | **FDC** · identifiant · surveillance |
| `FDC_VERBAUT` | **FDC** · installe |
| `GETRIEBE_ART` | boite de vitesses · type |
| `GPS_UHR` | **GPS** · heure |
| `HDC_ID_MONITOR` | assistance en descente · identifiant · surveillance |
| `HDC_VERBAUT` | assistance en descente · installe |
| `HEIMLEUCHTEN_10S` | eclairage d'accompagnement · **10S** |
| `HEIMLEUCHTEN_150S` | eclairage d'accompagnement · **150S** |
| `HEIMLEUCHTEN_240S` | eclairage d'accompagnement · **240S** |
| `HEIMLEUCHTEN_40S` | eclairage d'accompagnement · **40S** |
| `HEIMLEUCHTEN_90S` | eclairage d'accompagnement · **90S** |
| `HEIMLEUCHTEN_AUS` | eclairage d'accompagnement · arret |
| `HU_VERBAUT` | **HU** · installe |
| `K410D_C_WEG_1` | [K-Wort K410D] · C · course · 1 |
| `K410D_C_WEG_2` | [K-Wort K410D] · C · course · 2 |
| `K410D_C_WEG_3` | [K-Wort K410D] · C · course · 3 |
| `K410D_C_WEG_4` | [K-Wort K410D] · C · course · 4 |
| `K410D_D_WEG` | [K-Wort K410D] · D · course |
| `K4110_C_VERBRAUCH_1` | [K-Wort K4110] · C · consommation · 1 |
| `K4110_C_VERBRAUCH_2` | [K-Wort K4110] · C · consommation · 2 |
| `K4110_C_VERBRAUCH_3` | [K-Wort K4110] · C · consommation · 3 |
| `K4110_C_VERBRAUCH_4` | [K-Wort K4110] · C · consommation · 4 |
| `K4110_D_VERBRAUCH` | [K-Wort K4110] · D · consommation |
| `K4113_C_ZEIT_1` | [K-Wort K4113] · C · duree · 1 |
| `K4113_C_ZEIT_2` | [K-Wort K4113] · C · duree · 2 |
| `K4113_C_ZEIT_3` | [K-Wort K4113] · C · duree · 3 |
| `K4113_C_ZEIT_4` | [K-Wort K4113] · C · duree · 4 |
| `K4113_D_ZEIT` | [K-Wort K4113] · D · duree |
| `K4116_C_TEMPERATUR_1` | [K-Wort K4116] · C · temperature · 1 |
| `K4116_C_TEMPERATUR_2` | [K-Wort K4116] · C · temperature · 2 |
| `K4116_C_TEMPERATUR_3` | [K-Wort K4116] · C · temperature · 3 |
| `K4116_C_TEMPERATUR_4` | [K-Wort K4116] · C · temperature · 4 |
| `K4116_D_TEMPERATUR` | [K-Wort K4116] · D · temperature |
| `K4119_C_DRUCK_1` | [K-Wort K4119] · C · pression · 1 |
| `K4119_C_DRUCK_2` | [K-Wort K4119] · C · pression · 2 |
| `K4119_C_DRUCK_3` | [K-Wort K4119] · C · pression · 3 |
| `K4119_C_DRUCK_4` | [K-Wort K4119] · C · pression · 4 |
| `K4119_D_DRUCK` | [K-Wort K4119] · D · pression |
| `K430C_C_DATUM_1` | [K-Wort K430C] · C · date · 1 |
| `K430C_C_DATUM_2` | [K-Wort K430C] · C · date · 2 |
| `K430C_C_DATUM_3` | [K-Wort K430C] · C · date · 3 |
| `K430C_C_DATUM_4` | [K-Wort K430C] · C · date · 4 |
| `K430C_D_DATUM` | [K-Wort K430C] · D · date |
| `K4310_C_SPRACHE_1` | [K-Wort K4310] · C · langue · 1 |
| `K4310_C_SPRACHE_2` | [K-Wort K4310] · C · langue · 2 |
| `K4310_C_SPRACHE_3` | [K-Wort K4310] · C · langue · 3 |
| `K4310_C_SPRACHE_4` | [K-Wort K4310] · C · langue · 4 |
| `K4310_D_SPRACHE` | [K-Wort K4310] · D · langue |
| `KL_PREDRIVE_AUSWAHL_1` | borne · avant roulage · **AUSWAHL** · 1 |
| `KL_PREDRIVE_AUSWAHL_2` | borne · avant roulage · **AUSWAHL** · 2 |
| `KUEHL_MTL_AKTIV` | refroidissement · **MTL** · actif |
| `LEHNE_BF_ALIVE_ZAEHLER` | dossier · passager · signal de presence · compteur |
| `LEHNE_BF_ID_MONITOR` | dossier · passager · identifiant · surveillance |
| `LEHNE_FA_ALIVE_ZAEHLER` | dossier · conducteur · signal de presence · compteur |
| `LEHNE_FA_ID_MONITOR` | dossier · conducteur · identifiant · surveillance |
| `LICHT_AUT_WERT_1` | lumiere · automatique · valeur · 1 |
| `LICHT_AUT_WERT_2` | lumiere · automatique · valeur · 2 |
| `LICHT_AUT_WERT_3` | lumiere · automatique · valeur · 3 |
| `LL_RL_FZG` | **LL** · feu arriere · vehicule |
| `LM_ALIVE` | **LM** · signal de presence |
| `MMI_ENA_PIA` | interface homme-machine · activation · **PIA** |
| `MMI_ENA_RPA_RDC` | interface homme-machine · activation · indicateur de crevaison · pression des pneus |
| `MMI_ENA_UHR_DATUM` | interface homme-machine · activation · heure · date |
| `MOTORKONSTANTE` | **MOTORKONSTANTE** |
| `OHNE_QUITT_ENT` | sans · acquittement sonore · des- |
| `OHNE_QUITT_VER` | sans · acquittement sonore · version |
| `PREDRIVE_ABB_MOT` | avant roulage · **ABB** · **MOT** |
| `PREDRIVE_PERIODENDAUER` | avant roulage · **PERIODENDAUER** |
| `PREDRIVE_START_MIT_OBD` | avant roulage · demarrage · avec · **OBD** |
| `QUITT_AKUST_ENT` | acquittement sonore · sonore · des- |
| `QUITT_AKUST_VER` | acquittement sonore · sonore · version |
| `QUITT_OPT_AKUST_ENT` | acquittement sonore · option · sonore · des- |
| `QUITT_OPT_AKUST_VER` | acquittement sonore · option · sonore · version |
| `QUITT_OPT_ENT` | acquittement sonore · option · des- |
| `QUITT_OPT_VER` | acquittement sonore · option · version |
| `RDC_ALIVE` | pression des pneus · signal de presence |
| `RDC_VERBAUT` | pression des pneus · installe |
| `SFY_ALIVE_ZAEHLER` | **SFY** · signal de presence · compteur |
| `SFY_ID_MONITOR` | **SFY** · identifiant · surveillance |
| `SHOW_CC_MSG_NR` | **SHOW** · Check Control · **MSG** · numero |
| `SHOW_OEL_STAND` | **SHOW** · huile · etat |
| `SITZ_AUT_VERST_AUS` | siege · automatique · amplification · arret |
| `SITZ_AUT_VERST_ENT` | siege · automatique · amplification · des- |
| `SITZ_AUT_VERST_ENT_T` | siege · automatique · amplification · des- · T |
| `SITZ_L_UEBERWACHUNG` | siege · _L_ · surveillance |
| `TAG_LICHT_AN` | **TAG** · lumiere · **AN** |
| `TAG_LICHT_AUS` | **TAG** · lumiere · arret |
| `TANK_BERUHIGUNGSZEIT` | reservoir · **BERUHIGUNGSZEIT** |
| `TANK_FILTER_FALLEND` | reservoir · filtre · **FALLEND** |
| `TANK_FILTER_STEIGEND` | reservoir · filtre · **STEIGEND** |
| `TANK_MITTLUNGSZEIT` | reservoir · **MITTLUNGSZEIT** |
| `TANK_OHM_FEHLER_OG` | reservoir · **OHM** · erreur · **OG** |
| `TANK_OHM_FEHLER_UG` | reservoir · **OHM** · erreur · **UG** |
| `TANK_RED_REICHWEITE` | reservoir · reduction · **REICHWEITE** |
| `TANK_RW_WARNUNG` | reservoir · **RW** · alerte |
| `TANK_SCHWELLE_AUF_AB` | reservoir · seuil · ouverture · **AB** |
| `TANK_SCHWELLE_RESERVE` | reservoir · seuil · **RESERVE** |
| `TANK_SICHERHEITSFAKTOR` | reservoir · **SICHERHEITSFAKTOR** |
| `TANK_VOLL_ERKENNUNG` | reservoir · **VOLL** · **ERKENNUNG** |
| `TCM_ALIVE_MONITOR` | **TCM** · signal de presence · surveillance |
| `TCM_ID_MONITOR` | **TCM** · identifiant · surveillance |
| `TEMPOMAT_SETZ_ANZ_DAUER` | **TEMPOMAT** · positionnement · nombre · duree |
| `TIPP_BLINKEN_1X` | impulsion · **BLINKEN** · **1X** |
| `TIPP_BLINKEN_3X` | impulsion · **BLINKEN** · **3X** |
| `WARNFELD_INFO_DME` | **WARNFELD** · **INFO** · **DME** |
| `WARNUNG_GESCHW_LIMIT` | alerte · vitesse · limite |
| `ZGM_ALIVE_ZAEHLER` | **ZGM** · signal de presence · compteur |
| `ZGM_ID_MONITOR` | **ZGM** · identifiant · surveillance |
| `ZV_VER_AUS` | verrouillage centralise · version · arret |
| `ZV_VER_FAHRT` | verrouillage centralise · version · roulage |
| `ZV_VER_FAHRT_ZEIT` | verrouillage centralise · version · roulage · duree |
| `ZV_VER_ZEIT` | verrouillage centralise · version · duree |

---

Les fragments en **gras** n'ont pas de traduction etablie — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments entre `_underscores_` sont ambigus : leur sens depend du module.
