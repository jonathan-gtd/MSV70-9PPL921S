# KOMBI — Combine

🟢 Module `KOMBI` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟢 Module sans fonction de securite. Sauvegarde du `.TRC` quand meme.

---

## Source

| | |
|---|---|
| Variante retenue | `KMBI_PL2.C06` |
| Fonctions codables | **237** |
| Avec valeurs extraites | 237 (100 %) |
| Avec explication rédigée | 6 |
| Fichiers DATEN analysés | 5 |

Variantes disponibles : `KMBI_PL2.C05`, `KMBI_PL2.C06`, `KMBI_PL2.C07`, `KMBI_PL2.C08`, `KMBI_PL2.C09`

> La variante réellement installée est identifiée par NCS Dummy à l'ouverture du `.TRC`.
> Si elle diffère de celle retenue ici, la liste change.

---

## Les 237 fonctions codables

| FSW | Traduction | Valeurs possibles | Explication |
|---|---|---|---|
| `ACC_ALIVE_ZAEHLER` | regulateur adaptatif · signal de presence · compteur | `nicht_aktiv` · `aktiv` |  |
| `ACC_AUSSTATTUNG` | regulateur adaptatif · **AUSSTATTUNG** | `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `ACC_CSUM_MONITOR` | regulateur adaptatif · somme de controle · surveillance | `nicht_aktiv` · `aktiv` |  |
| `ACC_FILTER_FALLEND` | regulateur adaptatif · filtre · **FALLEND** | `wert_01` |  |
| `ACC_FILTER_STEIGEND` | regulateur adaptatif · filtre · **STEIGEND** | `wert_01` |  |
| `ACC_ID_MONITOR` | regulateur adaptatif · identifiant · surveillance | `nicht_aktiv` · `aktiv` |  |
| `ACC_ZEIGER_AUF_NULL` | regulateur adaptatif · **ZEIGER** · ouverture · **NULL** | `nicht_aktiv` · `aktiv` |  |
| `AFS_ALIVE_ZAEHLER` | phares adaptatifs (AFS) · signal de presence · compteur | `nicht_aktiv` · `aktiv` |  |
| `AFS_ID_MONITOR` | phares adaptatifs (AFS) · identifiant · surveillance | `nicht_aktiv` · `aktiv` |  |
| `AHM_ALIVE` | **AHM** · signal de presence | `nicht_aktiv` · `aktiv` |  |
| `ARS_ALIVE_ZAEHLER` | **ARS** · signal de presence · compteur | `nicht_aktiv` · `aktiv` |  |
| `ARS_ID_MONITOR` | **ARS** · identifiant · surveillance | `nicht_aktiv` · `aktiv` |  |
| `AUSSENTEMP_ANZ_FEHLER_1` | temperature exterieure · nombre · erreur · 1 | `wert_01` |  |
| `AUSSENTEMP_ANZ_FEHLER_2` | temperature exterieure · nombre · erreur · 2 | `wert_01` |  |
| `AUSSENTEMP_DELTA_K1` | temperature exterieure · ecart · **K1** | `wert_01` |  |
| `AUSSENTEMP_DELTA_K2` | temperature exterieure · ecart · **K2** | `wert_01` |  |
| `AUSSENTEMP_SPERR_FAKTOR` | temperature exterieure · blocage · facteur | `wert_01` |  |
| `AUSSENTEMP_SPERR_OFFSET` | temperature exterieure · blocage · decalage | `wert_01` |  |
| `BC_2_ENABLE` | ordinateur de bord · 2 · activation | `nicht_aktiv` · `aktiv` |  |
| `BC_DIGITAL_V` | ordinateur de bord · numerique · _V_ | `nicht_aktiv` · `aktiv` | Affichage de la vitesse en chiffres sur l'ordinateur de bord. |
| `BC_DIGITAL_V_KORREKTUR` | ordinateur de bord · numerique · _V_ · correction | `nicht_aktiv` · `aktiv` | Correction optimiste du compteur. Desactiver pour afficher la vitesse reelle. |
| `BC_LEERZEILE` | ordinateur de bord · **LEERZEILE** | `nicht_aktiv` · `aktiv` |  |
| `BC_LIMIT_ENABLE` | ordinateur de bord · limite · activation | `nicht_aktiv` · `aktiv` |  |
| `BC_V_KORREKTUR` | ordinateur de bord · _V_ · correction | `nicht_aktiv` · `aktiv` |  |
| `CAS_ALIVE` | **CAS** · signal de presence | `nicht_aktiv` · `aktiv` |  |
| `CBS_AVKM_START` | entretien selon besoin (CBS) · **AVKM** · demarrage | `wert_01` |  |
| `CBS_EINH_01H` | entretien selon besoin (CBS) · unite · [0x01] | `prozent` · `wegbas` · `zeitbas` · `wegzeit` |  |
| `CBS_EINH_02H` | entretien selon besoin (CBS) · unite · [0x02] | `prozent` · `wegbas` · `zeitbas` · `wegzeit` |  |
| `CBS_EINH_03H` | entretien selon besoin (CBS) · unite · [0x03] | `prozent` · `wegbas` · `zeitbas` |  |
| `CBS_EINH_04H` | entretien selon besoin (CBS) · unite · [0x04] | `prozent` · `wegbas` · `zeitbas` · `wegzeit` |  |
| `CBS_EINH_06H` | entretien selon besoin (CBS) · unite · [0x06] | `prozent` · `wegbas` · `zeitbas` |  |
| `CBS_EINH_07H` | entretien selon besoin (CBS) · unite · [0x07] | `prozent` · `wegbas` · `zeitbas` |  |
| `CBS_EINH_08H` | entretien selon besoin (CBS) · unite · [0x08] | `prozent` · `wegbas` · `zeitbas` · `wegzeit` |  |
| `CBS_EINH_10H` | entretien selon besoin (CBS) · unite · [0x10] | `prozent` · `wegbas` · `zeitbas` |  |
| `CBS_EINH_11H` | entretien selon besoin (CBS) · unite · [0x11] | `prozent` · `wegbas` · `zeitbas` · `wegzeit` |  |
| `CBS_EINH_12H` | entretien selon besoin (CBS) · unite · [0x12] | `prozent` · `wegbas` · `zeitbas` |  |
| `CBS_EINH_13H` | entretien selon besoin (CBS) · unite · [0x13] | `prozent` · `wegbas` · `zeitbas` · `wegzeit` |  |
| `CBS_EINH_14H` | entretien selon besoin (CBS) · unite · [0x14] | `prozent` · `wegbas` · `zeitbas` · `wegzeit` |  |
| `CBS_EINH_20H` | entretien selon besoin (CBS) · unite · [0x20] | `prozent` · `wegbas` · `zeitbas` |  |
| `CBS_EINH_21H` | entretien selon besoin (CBS) · unite · [0x21] | `prozent` · `wegbas` · `zeitbas` |  |
| `CBS_GELB` | entretien selon besoin (CBS) · **GELB** | `wert_01` · `wert_02` |  |
| `CBS_GELB_HUAU` | entretien selon besoin (CBS) · **GELB** · **HUAU** | `wert_01` |  |
| `CBS_GELB_KM_MIN` | entretien selon besoin (CBS) · **GELB** · **KM** · minimum | `wert_01` |  |
| `CBS_INTD_03H` | entretien selon besoin (CBS) · **INTD** · [0x03] | `wert_01` |  |
| `CBS_INTD_11H` | entretien selon besoin (CBS) · **INTD** · [0x11] | `wert_01` |  |
| `CBS_INTD_13H` | entretien selon besoin (CBS) · **INTD** · [0x13] | `wert_01` |  |
| `CBS_INTD_14H` | entretien selon besoin (CBS) · **INTD** · [0x14] | `wert_01` |  |
| `CBS_INTW_10H` | entretien selon besoin (CBS) · **INTW** · [0x10] | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `CBS_INTW_11H` | entretien selon besoin (CBS) · **INTW** · [0x11] | `wert_01` · `wert_02` · `wert_03` |  |
| `CBS_INTW_12H` | entretien selon besoin (CBS) · **INTW** · [0x12] | `wert_01` |  |
| `CBS_RES_SPERR` | entretien selon besoin (CBS) · remise a zero · blocage | `wert_01` |  |
| `CBS_SCEN_01H` | entretien selon besoin (CBS) · scenario · [0x01] | `aktiv` · `nicht_aktiv` |  |
| `CBS_SCEN_02H` | entretien selon besoin (CBS) · scenario · [0x02] | `aktiv` · `nicht_aktiv` |  |
| `CBS_SCEN_03H` | entretien selon besoin (CBS) · scenario · [0x03] | `aktiv` · `nicht_aktiv` |  |
| `CBS_SCEN_04H` | entretien selon besoin (CBS) · scenario · [0x04] | `aktiv` · `nicht_aktiv` |  |
| `CBS_SCEN_06H` | entretien selon besoin (CBS) · scenario · [0x06] | `aktiv` · `nicht_aktiv` |  |
| `CBS_SCEN_07H` | entretien selon besoin (CBS) · scenario · [0x07] | `aktiv` · `nicht_aktiv` |  |
| `CBS_SCEN_08H` | entretien selon besoin (CBS) · scenario · [0x08] | `aktiv` · `nicht_aktiv` |  |
| `CBS_SCEN_10H` | entretien selon besoin (CBS) · scenario · [0x10] | `aktiv` · `nicht_aktiv` |  |
| `CBS_SCEN_11H` | entretien selon besoin (CBS) · scenario · [0x11] | `aktiv` · `nicht_aktiv` |  |
| `CBS_SCEN_12H` | entretien selon besoin (CBS) · scenario · [0x12] | `aktiv` · `nicht_aktiv` |  |
| `CBS_SCEN_13H` | entretien selon besoin (CBS) · scenario · [0x13] | `aktiv` · `nicht_aktiv` |  |
| `CBS_SCEN_14H` | entretien selon besoin (CBS) · scenario · [0x14] | `aktiv` · `nicht_aktiv` |  |
| `CBS_SCEN_20H` | entretien selon besoin (CBS) · scenario · [0x20] | `aktiv` · `nicht_aktiv` |  |
| `CBS_SCEN_21H` | entretien selon besoin (CBS) · scenario · [0x21] | `aktiv` · `nicht_aktiv` |  |
| `CBS_STAT_01H` | entretien selon besoin (CBS) · etat · [0x01] | `anzeige` · `sperre` · `erprobung` |  |
| `CBS_STAT_02H` | entretien selon besoin (CBS) · etat · [0x02] | `anzeige` · `sperre` · `erprobung` |  |
| `CBS_STAT_03H` | entretien selon besoin (CBS) · etat · [0x03] | `anzeige` · `sperre` · `erprobung` |  |
| `CBS_STAT_04H` | entretien selon besoin (CBS) · etat · [0x04] | `anzeige` · `sperre` · `erprobung` |  |
| `CBS_STAT_06H` | entretien selon besoin (CBS) · etat · [0x06] | `anzeige` · `sperre` · `erprobung` |  |
| `CBS_STAT_07H` | entretien selon besoin (CBS) · etat · [0x07] | `anzeige` · `sperre` · `erprobung` |  |
| `CBS_STAT_08H` | entretien selon besoin (CBS) · etat · [0x08] | `anzeige` · `sperre` · `erprobung` |  |
| `CBS_STAT_10H` | entretien selon besoin (CBS) · etat · [0x10] | `anzeige` · `sperre` · `erprobung` |  |
| `CBS_STAT_11H` | entretien selon besoin (CBS) · etat · [0x11] | `anzeige` · `sperre` · `erprobung` |  |
| `CBS_STAT_12H` | entretien selon besoin (CBS) · etat · [0x12] | `anzeige` · `sperre` · `erprobung` |  |
| `CBS_STAT_13H` | entretien selon besoin (CBS) · etat · [0x13] | `anzeige` · `sperre` · `erprobung` |  |
| `CBS_STAT_14H` | entretien selon besoin (CBS) · etat · [0x14] | `anzeige` · `sperre` · `erprobung` |  |
| `CBS_STAT_20H` | entretien selon besoin (CBS) · etat · [0x20] | `anzeige` · `sperre` · `erprobung` |  |
| `CBS_STAT_21H` | entretien selon besoin (CBS) · etat · [0x21] | `anzeige` · `sperre` · `erprobung` |  |
| `CBS_TZ_SCHALTER` | entretien selon besoin (CBS) · **TZ** · **SCHALTER** | `tagz` · `borddatum` |  |
| `CC_15_AUS` | Check Control · 15 · arret | `wert_01` |  |
| `CC_15_EIN` | Check Control · 15 · marche | `benzin` · `diesel` |  |
| `CC_BLINKFREQUENZ_LANGSAM` | Check Control · **BLINKFREQUENZ** · **LANGSAM** | `wert_01` |  |
| `CC_BLINKFREQUENZ_SCHNELL` | Check Control · **BLINKFREQUENZ** · **SCHNELL** | `wert_01` |  |
| `CC_BREMSWARNL_NUR_ROT` | Check Control · **BREMSWARNL** · **NUR** · **ROT** | `nicht_aktiv` · `aktiv` |  |
| `CC_CSUM` | Check Control · somme de controle | `grundcode_var` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_10` |  |
| `CC_ML` | Check Control · **ML** | `wert_01` |  |
| `CC_NR_ROLLE` | Check Control · numero · **ROLLE** | `nicht_aktiv` · `aktiv` |  |
| `CC_R_AUS` | Check Control · _R_ · arret | `wert_01` |  |
| `CC_R_EIN` | Check Control · _R_ · marche | `wert_01` |  |
| `CC_TEMPERATURWARNUNG` | Check Control · **TEMPERATURWARNUNG** | `nicht_aktiv` · `aktiv` |  |
| `CC_ZUENDSCHLUESSEL` | Check Control · **ZUENDSCHLUESSEL** | `nicht_aktiv` · `aktiv` | Alerte de portiere ouverte lorsque la cle est en contact. |
| `CODING_VALID_BMW` | **CODING** · **VALID** · BMW | `wert_01` |  |
| `CRASH_TH_DIMMUNG` | choc · **TH** · **DIMMUNG** | `wert_01` |  |
| `CSUM_ACC` | somme de controle · regulateur adaptatif | `grundcode_var` |  |
| `CSUM_AKUSTIK_KLANG` | somme de controle · **AKUSTIK** · sonorite | `grundcode_var` · `wert_01` |  |
| `CSUM_ATEMP` | somme de controle · **ATEMP** | `grundcode_var` |  |
| `CSUM_AUSSTATTUNG_1` | somme de controle · **AUSSTATTUNG** · 1 | `grundcode_var` |  |
| `CSUM_AUSSTATTUNG_PL2` | somme de controle · **AUSSTATTUNG** · **PL2** | `grundcode_var` |  |
| `CSUM_CBS_01` | somme de controle · entretien selon besoin (CBS) · 01 | `grundcode_var` |  |
| `CSUM_CC_ALIVE` | somme de controle · Check Control · signal de presence | `grundcode_var` |  |
| `CSUM_KODIERUNG_VALID` | somme de controle · **KODIERUNG** · **VALID** | `grundcode_var` |  |
| `CSUM_ST_ANZEIGE_1` | somme de controle · **ST** · affichage · 1 | `grundcode_var` |  |
| `CSUM_TANK_INPUT` | somme de controle · reservoir · entree | `grundcode_var` |  |
| `CSUM_TANK_PROZESS` | somme de controle · reservoir · **PROZESS** | `grundcode_var` |  |
| `CSUM_WEG` | somme de controle · course | `grundcode_var` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `DIMM_MN_MAX` | gradation · **MN** · maximum | `wert_01` |  |
| `DIMM_MN_MIN` | gradation · **MN** · minimum | `wert_01` |  |
| `DME_ALIVE_ZAEHLER` | **DME** · signal de presence · compteur | `nicht_aktiv` · `aktiv` |  |
| `DME_ID_MONITOR` | **DME** · identifiant · surveillance | `nicht_aktiv` · `aktiv` |  |
| `DSC_ALIVE_ZAEHLER` | controle de stabilite · signal de presence · compteur | `nicht_aktiv` · `aktiv` |  |
| `DSC_ALLRAD_CC_WAHL` | controle de stabilite · **ALLRAD** · Check Control · **WAHL** | `nicht_aktiv` · `aktiv` |  |
| `DSC_ID_MONITOR` | controle de stabilite · identifiant · surveillance | `nicht_aktiv` · `aktiv` |  |
| `DSC_MIT_RPA_CC` | controle de stabilite · avec · indicateur de crevaison · Check Control | `nicht_aktiv` · `aktiv` |  |
| `DWA_VERBAUT` | alarme antivol · installe | `nicht_aktiv` · `aktiv` |  |
| `EGS_ALIVE_ZAEHLER` | calculateur de boite · signal de presence · compteur | `nicht_aktiv` · `aktiv` |  |
| `EGS_CSUM_MONITOR` | calculateur de boite · somme de controle · surveillance | `nicht_aktiv` · `aktiv` |  |
| `EGS_ID_MONITOR` | calculateur de boite · identifiant · surveillance | `nicht_aktiv` · `aktiv` |  |
| `EHC_ALIVE` | **EHC** · signal de presence | `nicht_aktiv` · `aktiv` |  |
| `EKP_ALIVE_ZAEHLER` | pompe a carburant · signal de presence · compteur | `nicht_aktiv` · `aktiv` |  |
| `EKP_ID_MONITOR` | pompe a carburant · identifiant · surveillance | `nicht_aktiv` · `aktiv` |  |
| `EPS_ALIVE_MONITOR` | **EPS** · signal de presence · surveillance | `nicht_aktiv` · `aktiv` |  |
| `EPS_ID_MONITOR` | **EPS** · identifiant · surveillance | `nicht_aktiv` · `aktiv` |  |
| `FB_ALLE_TUEREN` | telecommande · **ALLE** · portes | `nicht_aktiv` · `aktiv` |  |
| `FB_FA_TUER` | telecommande · conducteur · porte | `nicht_aktiv` · `aktiv` |  |
| `FDC_ID_MONITOR` | **FDC** · identifiant · surveillance | `nicht_aktiv` · `aktiv` |  |
| `FDC_VERBAUT` | **FDC** · installe | `nicht_aktiv` · `aktiv` |  |
| `GETRIEBE_ART` | boite de vitesses · type | `automatik` · `ssg` · `handschalter` |  |
| `GPS_UHR` | **GPS** · heure | `nicht_aktiv` · `aktiv` |  |
| `HDC_ID_MONITOR` | assistance en descente · identifiant · surveillance | `nicht_aktiv` · `aktiv` |  |
| `HDC_VERBAUT` | assistance en descente · installe | `nicht_aktiv` · `aktiv` |  |
| `HEIMLEUCHTEN_10S` | eclairage d'accompagnement · **10S** | `nicht_aktiv` · `aktiv` |  |
| `HEIMLEUCHTEN_150S` | eclairage d'accompagnement · **150S** | `nicht_aktiv` · `aktiv` |  |
| `HEIMLEUCHTEN_240S` | eclairage d'accompagnement · **240S** | `nicht_aktiv` · `aktiv` |  |
| `HEIMLEUCHTEN_40S` | eclairage d'accompagnement · **40S** | `nicht_aktiv` · `aktiv` |  |
| `HEIMLEUCHTEN_90S` | eclairage d'accompagnement · **90S** | `nicht_aktiv` · `aktiv` |  |
| `HEIMLEUCHTEN_AUS` | eclairage d'accompagnement · arret | `nicht_aktiv` · `aktiv` |  |
| `HU_VERBAUT` | **HU** · installe | `radiostufe_1` · `radiostufe_2` · `radiostufe_3_4` |  |
| `K410D_C_WEG_1` | [K-Wort K410D] · C · course · 1 | `wert_01` · `wert_02` |  |
| `K410D_C_WEG_2` | [K-Wort K410D] · C · course · 2 | `wert_01` · `wert_02` |  |
| `K410D_C_WEG_3` | [K-Wort K410D] · C · course · 3 | `wert_01` · `wert_02` |  |
| `K410D_C_WEG_4` | [K-Wort K410D] · C · course · 4 | `wert_01` · `wert_02` |  |
| `K410D_D_WEG` | [K-Wort K410D] · D · course | `wert_01` · `wert_02` |  |
| `K4110_C_VERBRAUCH_1` | [K-Wort K4110] · C · consommation · 1 | `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `K4110_C_VERBRAUCH_2` | [K-Wort K4110] · C · consommation · 2 | `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `K4110_C_VERBRAUCH_3` | [K-Wort K4110] · C · consommation · 3 | `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `K4110_C_VERBRAUCH_4` | [K-Wort K4110] · C · consommation · 4 | `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `K4110_D_VERBRAUCH` | [K-Wort K4110] · D · consommation | `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `K4113_C_ZEIT_1` | [K-Wort K4113] · C · duree · 1 | `wert_01` · `wert_02` |  |
| `K4113_C_ZEIT_2` | [K-Wort K4113] · C · duree · 2 | `wert_01` · `wert_02` |  |
| `K4113_C_ZEIT_3` | [K-Wort K4113] · C · duree · 3 | `wert_01` · `wert_02` |  |
| `K4113_C_ZEIT_4` | [K-Wort K4113] · C · duree · 4 | `wert_01` · `wert_02` |  |
| `K4113_D_ZEIT` | [K-Wort K4113] · D · duree | `wert_01` · `wert_02` |  |
| `K4116_C_TEMPERATUR_1` | [K-Wort K4116] · C · temperature · 1 | `wert_01` · `wert_02` |  |
| `K4116_C_TEMPERATUR_2` | [K-Wort K4116] · C · temperature · 2 | `wert_01` · `wert_02` |  |
| `K4116_C_TEMPERATUR_3` | [K-Wort K4116] · C · temperature · 3 | `wert_01` · `wert_02` |  |
| `K4116_C_TEMPERATUR_4` | [K-Wort K4116] · C · temperature · 4 | `wert_01` · `wert_02` |  |
| `K4116_D_TEMPERATUR` | [K-Wort K4116] · D · temperature | `wert_01` · `wert_02` |  |
| `K4119_C_DRUCK_1` | [K-Wort K4119] · C · pression · 1 | `wert_01` · `wert_02` · `wert_03` |  |
| `K4119_C_DRUCK_2` | [K-Wort K4119] · C · pression · 2 | `wert_01` · `wert_02` · `wert_03` |  |
| `K4119_C_DRUCK_3` | [K-Wort K4119] · C · pression · 3 | `wert_01` · `wert_02` · `wert_03` |  |
| `K4119_C_DRUCK_4` | [K-Wort K4119] · C · pression · 4 | `wert_01` · `wert_02` · `wert_03` |  |
| `K4119_D_DRUCK` | [K-Wort K4119] · D · pression | `wert_01` · `wert_02` · `wert_03` |  |
| `K430C_C_DATUM_1` | [K-Wort K430C] · C · date · 1 | `wert_01` · `wert_02` |  |
| `K430C_C_DATUM_2` | [K-Wort K430C] · C · date · 2 | `wert_01` · `wert_02` |  |
| `K430C_C_DATUM_3` | [K-Wort K430C] · C · date · 3 | `wert_01` · `wert_02` |  |
| `K430C_C_DATUM_4` | [K-Wort K430C] · C · date · 4 | `wert_01` · `wert_02` |  |
| `K430C_D_DATUM` | [K-Wort K430C] · D · date | `wert_01` · `wert_02` |  |
| `K4310_C_SPRACHE_1` | [K-Wort K4310] · C · langue · 1 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+15)* |  |
| `K4310_C_SPRACHE_2` | [K-Wort K4310] · C · langue · 2 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+15)* |  |
| `K4310_C_SPRACHE_3` | [K-Wort K4310] · C · langue · 3 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+15)* |  |
| `K4310_C_SPRACHE_4` | [K-Wort K4310] · C · langue · 4 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+15)* |  |
| `K4310_D_SPRACHE` | [K-Wort K4310] · D · langue | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+15)* |  |
| `KL_PREDRIVE_AUSWAHL_1` | borne · avant roulage · **AUSWAHL** · 1 | `wert_01` · `wert_02` |  |
| `KL_PREDRIVE_AUSWAHL_2` | borne · avant roulage · **AUSWAHL** · 2 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `KUEHL_MTL_AKTIV` | refroidissement · **MTL** · actif | `aktiv_low` · `aktiv_high` |  |
| `LEHNE_BF_ALIVE_ZAEHLER` | dossier · passager · signal de presence · compteur | `nicht_aktiv` · `aktiv` |  |
| `LEHNE_BF_ID_MONITOR` | dossier · passager · identifiant · surveillance | `nicht_aktiv` · `aktiv` |  |
| `LEHNE_FA_ALIVE_ZAEHLER` | dossier · conducteur · signal de presence · compteur | `nicht_aktiv` · `aktiv` |  |
| `LEHNE_FA_ID_MONITOR` | dossier · conducteur · identifiant · surveillance | `nicht_aktiv` · `aktiv` |  |
| `LICHT_AUT_WERT_1` | lumiere · automatique · valeur · 1 | `nicht_aktiv` · `aktiv` | Seuil de declenchement de l'allumage automatique des feux. Les trois valeurs se modifient ensemble. |
| `LICHT_AUT_WERT_2` | lumiere · automatique · valeur · 2 | `nicht_aktiv` · `aktiv` | Seuil de declenchement de l'allumage automatique des feux. Les trois valeurs se modifient ensemble. |
| `LICHT_AUT_WERT_3` | lumiere · automatique · valeur · 3 | `nicht_aktiv` · `aktiv` | Seuil de declenchement de l'allumage automatique des feux. Les trois valeurs se modifient ensemble. |
| `LL_RL_FZG` | **LL** · feu arriere · vehicule | `nicht_aktiv` · `aktiv` |  |
| `LM_ALIVE` | **LM** · signal de presence | `nicht_aktiv` · `aktiv` |  |
| `MMI_ENA_PIA` | interface homme-machine · activation · **PIA** | `nicht_aktiv` · `aktiv` |  |
| `MMI_ENA_RPA_RDC` | interface homme-machine · activation · indicateur de crevaison · pression des pneus | `nicht_aktiv` · `aktiv` |  |
| `MMI_ENA_UHR_DATUM` | interface homme-machine · activation · heure · date | `nicht_aktiv` · `aktiv` |  |
| `MOTORKONSTANTE` | **MOTORKONSTANTE** | `wert_01` |  |
| `OHNE_QUITT_ENT` | sans · acquittement sonore · des- | `nicht_aktiv` · `aktiv` |  |
| `OHNE_QUITT_VER` | sans · acquittement sonore · version | `nicht_aktiv` · `aktiv` |  |
| `PREDRIVE_ABB_MOT` | avant roulage · **ABB** · **MOT** | `nicht_aktiv` · `aktiv` |  |
| `PREDRIVE_PERIODENDAUER` | avant roulage · **PERIODENDAUER** | `wert_01` |  |
| `PREDRIVE_START_MIT_OBD` | avant roulage · demarrage · avec · **OBD** | `nicht_aktiv` · `aktiv` |  |
| `QUITT_AKUST_ENT` | acquittement sonore · sonore · des- | `nicht_aktiv` · `aktiv` |  |
| `QUITT_AKUST_VER` | acquittement sonore · sonore · version | `nicht_aktiv` · `aktiv` |  |
| `QUITT_OPT_AKUST_ENT` | acquittement sonore · option · sonore · des- | `nicht_aktiv` · `aktiv` |  |
| `QUITT_OPT_AKUST_VER` | acquittement sonore · option · sonore · version | `nicht_aktiv` · `aktiv` |  |
| `QUITT_OPT_ENT` | acquittement sonore · option · des- | `nicht_aktiv` · `aktiv` |  |
| `QUITT_OPT_VER` | acquittement sonore · option · version | `nicht_aktiv` · `aktiv` |  |
| `RDC_ALIVE` | pression des pneus · signal de presence | `nicht_aktiv` · `aktiv` |  |
| `RDC_VERBAUT` | pression des pneus · installe | `nicht_aktiv` · `aktiv` |  |
| `SFY_ALIVE_ZAEHLER` | **SFY** · signal de presence · compteur | `nicht_aktiv` · `aktiv` |  |
| `SFY_ID_MONITOR` | **SFY** · identifiant · surveillance | `nicht_aktiv` · `aktiv` |  |
| `SHOW_CC_MSG_NR` | **SHOW** · Check Control · **MSG** · numero | `nicht_aktiv` · `aktiv` |  |
| `SHOW_OEL_STAND` | **SHOW** · huile · etat | `nicht_aktiv` · `aktiv` |  |
| `SITZ_AUT_VERST_AUS` | siege · automatique · amplification · arret | `nicht_aktiv` · `aktiv` |  |
| `SITZ_AUT_VERST_ENT` | siege · automatique · amplification · des- | `nicht_aktiv` · `aktiv` |  |
| `SITZ_AUT_VERST_ENT_T` | siege · automatique · amplification · des- · T | `nicht_aktiv` · `aktiv` |  |
| `SITZ_L_UEBERWACHUNG` | siege · _L_ · surveillance | `nicht_aktiv` · `aktiv` |  |
| `TAG_LICHT_AN` | **TAG** · lumiere · **AN** | `nicht_aktiv` · `aktiv` |  |
| `TAG_LICHT_AUS` | **TAG** · lumiere · arret | `nicht_aktiv` · `aktiv` |  |
| `TANK_BERUHIGUNGSZEIT` | reservoir · **BERUHIGUNGSZEIT** | `wert_01` |  |
| `TANK_FILTER_FALLEND` | reservoir · filtre · **FALLEND** | `wert_01` · `wert_02` · `wert_03` |  |
| `TANK_FILTER_STEIGEND` | reservoir · filtre · **STEIGEND** | `wert_01` · `wert_02` · `wert_03` |  |
| `TANK_MITTLUNGSZEIT` | reservoir · **MITTLUNGSZEIT** | `wert_01` |  |
| `TANK_OHM_FEHLER_OG` | reservoir · **OHM** · erreur · **OG** | `wert_01` |  |
| `TANK_OHM_FEHLER_UG` | reservoir · **OHM** · erreur · **UG** | `wert_01` |  |
| `TANK_RED_REICHWEITE` | reservoir · reduction · **REICHWEITE** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `TANK_RW_WARNUNG` | reservoir · **RW** · alerte | `wert_01` |  |
| `TANK_SCHWELLE_AUF_AB` | reservoir · seuil · ouverture · **AB** | `wert_01` |  |
| `TANK_SCHWELLE_RESERVE` | reservoir · seuil · **RESERVE** | `wert_01` · `wert_02` · `wert_03` |  |
| `TANK_SICHERHEITSFAKTOR` | reservoir · **SICHERHEITSFAKTOR** | `wert_01` |  |
| `TANK_VOLL_ERKENNUNG` | reservoir · **VOLL** · **ERKENNUNG** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` |  |
| `TCM_ALIVE_MONITOR` | **TCM** · signal de presence · surveillance | `nicht_aktiv` · `aktiv` |  |
| `TCM_ID_MONITOR` | **TCM** · identifiant · surveillance | `nicht_aktiv` · `aktiv` |  |
| `TEMPOMAT_SETZ_ANZ_DAUER` | **TEMPOMAT** · positionnement · nombre · duree | `wert_01` · `wert_02` |  |
| `TIPP_BLINKEN_1X` | impulsion · **BLINKEN** · **1X** | `nicht_aktiv` · `aktiv` |  |
| `TIPP_BLINKEN_3X` | impulsion · **BLINKEN** · **3X** | `nicht_aktiv` · `aktiv` |  |
| `WARNFELD_INFO_DME` | **WARNFELD** · **INFO** · **DME** | `nicht_aktiv` · `aktiv` |  |
| `WARNUNG_GESCHW_LIMIT` | alerte · vitesse · limite | `nicht_aktiv` |  |
| `ZGM_ALIVE_ZAEHLER` | **ZGM** · signal de presence · compteur | `nicht_aktiv` · `aktiv` |  |
| `ZGM_ID_MONITOR` | **ZGM** · identifiant · surveillance | `nicht_aktiv` · `aktiv` |  |
| `ZV_VER_AUS` | verrouillage centralise · version · arret | `nicht_aktiv` · `aktiv` |  |
| `ZV_VER_FAHRT` | verrouillage centralise · version · roulage | `nicht_aktiv` · `aktiv` |  |
| `ZV_VER_FAHRT_ZEIT` | verrouillage centralise · version · roulage · duree | `nicht_aktiv` · `aktiv` |  |
| `ZV_VER_ZEIT` | verrouillage centralise · version · duree | `nicht_aktiv` · `aktiv` |  |

---

**Traduction** : décomposition du nom allemand — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments en **gras** n'ont pas de traduction établie, ceux entre `_underscores_` sont ambigus.

**Valeurs possibles** : extraites des DATEN. `aktiv`/`nicht_aktiv` = interrupteur,
`wert_xx` = valeur numérique, autres = choix nommés.

**Explication** : rédigée uniquement pour les fonctions identifiées avec certitude.
Une case vide ne veut pas dire que la fonction est sans intérêt — seulement que son effet
n'a pas été établi de source sûre.
