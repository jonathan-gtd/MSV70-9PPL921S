# FRM — Module de plancher

🟠 Module `FRM` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Source

| | |
|---|---|
| Variante retenue | `FRMFA.C10` |
| FSW dans cette variante | **438** |
| FSW toutes variantes confondues | 857 |
| Fichiers DATEN analyses | 7 |

Variantes disponibles : `FRMFA.C09`, `FRMFA.C10`, `FRMFA.C11`, `FRMFA.C12`, `FRMFA.C21`, `FRM2_E89.C08`, `FRM2_E89.C09`

> La variante reellement installee sur le vehicule est identifiee par NCS Dummy
> a l'ouverture du `.TRC`. Si elle differe de celle retenue ici, la liste change.

---

## Les 438 fonctions codables

| FSW | Traduction |
|---|---|
| `ABSCHALTUNG_RFS` | coupure · feu de recul |
| `ABSCHWAECH_D_TAU` | **ABSCHWAECH** · D · _TAU_ |
| `ABSCHW_D_TAU_HI` | attenuation · D · _TAU_ · haut |
| `ABSCHW_D_TAU_LO` | attenuation · D · _TAU_ · bas |
| `ABSTELLPOS_CUSTOM` | **ABSTELLPOS** · **CUSTOM** |
| `ABSTELLPOS_SMC` | **ABSTELLPOS** · **SMC** |
| `AEND_V_SCHWENK_N_AUSSEN` | **AEND** · _V_ · pivotement · N · exterieur |
| `ALC_ABTASTRATE_DYN` | phares directionnels (ALC) · **ABTASTRATE** · dynamique |
| `ALC_ABTASTRATE_TREND` | phares directionnels (ALC) · **ABTASTRATE** · **TREND** |
| `ALC_ABWEICHUNG_TREND` | phares directionnels (ALC) · **ABWEICHUNG** · **TREND** |
| `ALC_AUSWERT_GRADIENT` | phares directionnels (ALC) · **AUSWERT** · **GRADIENT** |
| `ALC_AUSWERT_PROP_ABWEICH` | phares directionnels (ALC) · **AUSWERT** · **PROP** · **ABWEICH** |
| `ALC_CC_ALC_ERR_OFF` | phares directionnels (ALC) · Check Control · phares directionnels (ALC) · erreur · arret |
| `ALC_DAUER_DYN_NACHFUEHR` | phares directionnels (ALC) · duree · dynamique · **NACHFUEHR** |
| `ALC_DEF_LWR_SENSOR_HI` | phares directionnels (ALC) · defaut · correcteur de site des phares · capteur · haut |
| `ALC_DIFFERENZ_MSP_DYN_STA` | phares directionnels (ALC) · **DIFFERENZ** · **MSP** · dynamique · **STA** |
| `ALC_DISABLE_AUSGL_NICK_GW` | phares directionnels (ALC) · desactivation · **AUSGL** · **NICK** · **GW** |
| `ALC_EINSCHALTBEDINGUNG` | phares directionnels (ALC) · **EINSCHALTBEDINGUNG** |
| `ALC_EPSILON_DELTA_A` | phares directionnels (ALC) · **EPSILON** · ecart · A |
| `ALC_EPSILON_DELTA_AV` | phares directionnels (ALC) · **EPSILON** · ecart · **AV** |
| `ALC_ERR_BLK_FREQ` | phares directionnels (ALC) · erreur · clignotant · frequence |
| `ALC_FAHRZEUG` | phares directionnels (ALC) · **FAHRZEUG** |
| `ALC_GRADIENT_SCHLECHTWEG` | phares directionnels (ALC) · **GRADIENT** · **SCHLECHTWEG** |
| `ALC_HOEHENWEG` | phares directionnels (ALC) · **HOEHENWEG** |
| `ALC_KLS_ENABLE` | phares directionnels (ALC) · **KLS** · activation |
| `ALC_LWR_NEIGUNG_KONSTANTE` | phares directionnels (ALC) · correcteur de site des phares · inclinaison · **KONSTANTE** |
| `ALC_LWR_VORZEICHEN` | phares directionnels (ALC) · correcteur de site des phares · **VORZEICHEN** |
| `ALC_MINDESTANZAHL_VZ_WECH` | phares directionnels (ALC) · **MINDESTANZAHL** · **VZ** · **WECH** |
| `ALC_MIN_STEIGUNG_VZ_WECHS` | phares directionnels (ALC) · minimum · **STEIGUNG** · **VZ** · **WECHS** |
| `ALC_MITTELUNGSINTERV_DYN` | phares directionnels (ALC) · **MITTELUNGSINTERV** · dynamique |
| `ALC_MITTELUNGSINTERV_TREN` | phares directionnels (ALC) · **MITTELUNGSINTERV** · **TREN** |
| `ALC_MITTELUNGSINT_VZ_WECH` | phares directionnels (ALC) · **MITTELUNGSINT** · **VZ** · **WECH** |
| `ALC_NACHLAUF_SCHLECHTWEG` | phares directionnels (ALC) · marche a suivre / temporisation · **SCHLECHTWEG** |
| `ALC_OFFSET_LWR` | phares directionnels (ALC) · decalage · correcteur de site des phares |
| `ALC_SAETT_TIMER_SCHLECHTW` | phares directionnels (ALC) · **SAETT** · minuterie · **SCHLECHTW** |
| `ALC_SCHWENK_LATCH` | phares directionnels (ALC) · pivotement · **LATCH** |
| `ALC_SCHW_GRAD_BESCHLEUNIG` | phares directionnels (ALC) · seuil · degre · **BESCHLEUNIG** |
| `ALC_SCHW_GRAD_BREMSEN` | phares directionnels (ALC) · seuil · degre · **BREMSEN** |
| `ALC_SCHW_GRAD_SCHLECHTWEG` | phares directionnels (ALC) · seuil · degre · **SCHLECHTWEG** |
| `ALC_SCHW_PROP_ABWEICHUNG` | phares directionnels (ALC) · seuil · **PROP** · **ABWEICHUNG** |
| `ALC_SCHW_TREND` | phares directionnels (ALC) · seuil · **TREND** |
| `ALC_VMAX_BESCHLEUNIGUNG` | phares directionnels (ALC) · **VMAX** · **BESCHLEUNIGUNG** |
| `ALC_VMAX_BREMSEN` | phares directionnels (ALC) · **VMAX** · **BREMSEN** |
| `ALC_VMIN_BESCHLEUNIGUNG` | phares directionnels (ALC) · **VMIN** · **BESCHLEUNIGUNG** |
| `ALC_VMIN_BREMSEN` | phares directionnels (ALC) · **VMIN** · **BREMSEN** |
| `AL_KL50_SCHUTZ` | feux de croisement · borne 50 (demarreur) · protection |
| `ASP_KEY_MEM_OHNE_MASTER` | retroviseur exterieur · **KEY** · **MEM** · sans · **MASTER** |
| `ASP_LIN_VERBAUT` | retroviseur exterieur · bus LIN · installe |
| `AUTOM_LICHT_ABSCHALTUNG` | automatique · lumiere · coupure |
| `AUTOM_LICHT_KONTROLLE` | automatique · lumiere · **KONTROLLE** |
| `AUTO_LWR_ENABLE` | automatique · correcteur de site des phares · activation |
| `BEHOERDENFZG` | vehicule administratif |
| `BEHOERDFZG_FL_ASYNCHR` | **BEHOERDFZG** · feux de route · **ASYNCHR** |
| `BFD_1_ALGORITHMUS` | feux stop adaptatifs (Brake Force Display) · 1 · **ALGORITHMUS** |
| `BFD_1_BFD_AUSG_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 1 · feux stop adaptatifs (Brake Force Display) · **AUSG** · apparition · image |
| `BFD_1_BLINK_FREQ_AUS` | feux stop adaptatifs (Brake Force Display) · 1 · clignotant · frequence · arret |
| `BFD_1_BLINK_FREQ_EIN` | feux stop adaptatifs (Brake Force Display) · 1 · clignotant · frequence · marche |
| `BFD_1_BL_M_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 1 · feu stop · M · apparition · image |
| `BFD_1_ENTPRELL_ABS` | feux stop adaptatifs (Brake Force Display) · 1 · anti-rebond · ABS |
| `BFD_1_ENTPRELL_AX` | feux stop adaptatifs (Brake Force Display) · 1 · anti-rebond · **AX** |
| `BFD_1_MIN_GESCHW` | feux stop adaptatifs (Brake Force Display) · 1 · minimum · vitesse |
| `BFD_1_NSL_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 1 · antibrouillard arriere · apparition · image |
| `BFD_1_RL_BL_1_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 1 · feu arriere · feu stop · 1 · apparition · image |
| `BFD_1_RL_BL_2_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 1 · feu arriere · feu stop · 2 · apparition · image |
| `BFD_1_RL_BL_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 1 · feu arriere · feu stop · apparition · image |
| `BFD_1_STUFE_DELAY` | feux stop adaptatifs (Brake Force Display) · 1 · niveau · delai |
| `BFD_1_THERM_TIMEOUT` | feux stop adaptatifs (Brake Force Display) · 1 · **THERM** · delai d'attente |
| `BFD_1_WEGFALL` | feux stop adaptatifs (Brake Force Display) · 1 · **WEGFALL** |
| `BFD_2_ALGORITHMUS` | feux stop adaptatifs (Brake Force Display) · 2 · **ALGORITHMUS** |
| `BFD_2_BFD_AUSG_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 2 · feux stop adaptatifs (Brake Force Display) · **AUSG** · apparition · image |
| `BFD_2_BLINK_FREQ_AUS` | feux stop adaptatifs (Brake Force Display) · 2 · clignotant · frequence · arret |
| `BFD_2_BLINK_FREQ_EIN` | feux stop adaptatifs (Brake Force Display) · 2 · clignotant · frequence · marche |
| `BFD_2_BL_M_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 2 · feu stop · M · apparition · image |
| `BFD_2_ENTPRELL_ABS` | feux stop adaptatifs (Brake Force Display) · 2 · anti-rebond · ABS |
| `BFD_2_ENTPRELL_AX` | feux stop adaptatifs (Brake Force Display) · 2 · anti-rebond · **AX** |
| `BFD_2_MIN_GESCHW` | feux stop adaptatifs (Brake Force Display) · 2 · minimum · vitesse |
| `BFD_2_NSL_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 2 · antibrouillard arriere · apparition · image |
| `BFD_2_RL_BL_1_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 2 · feu arriere · feu stop · 1 · apparition · image |
| `BFD_2_RL_BL_2_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 2 · feu arriere · feu stop · 2 · apparition · image |
| `BFD_2_RL_BL_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 2 · feu arriere · feu stop · apparition · image |
| `BFD_2_STUFE_DELAY` | feux stop adaptatifs (Brake Force Display) · 2 · niveau · delai |
| `BFD_2_THERM_TIMEOUT` | feux stop adaptatifs (Brake Force Display) · 2 · **THERM** · delai d'attente |
| `BFD_2_WEGFALL` | feux stop adaptatifs (Brake Force Display) · 2 · **WEGFALL** |
| `BFD_BREMSL_FKT` | feux stop adaptatifs (Brake Force Display) · feu stop · fonction |
| `BFD_ERS_DEF_RL_BL` | feux stop adaptatifs (Brake Force Display) · **ERS** · defaut · feu arriere · feu stop |
| `BFD_ERS_DEF_RL_BL_1` | feux stop adaptatifs (Brake Force Display) · **ERS** · defaut · feu arriere · feu stop · 1 |
| `BI_XENON_BEI_LH` | **BI** · xenon · lors de · **LH** |
| `BI_XENON_FL_ABSCHALT_OPT` | **BI** · xenon · feux de route · coupure · option |
| `BI_XENON_VERBAUT` | **BI** · xenon · installe |
| `BLEND_CHECK_IST_WINKEL` | **BLEND** · controle · **IST** · **WINKEL** |
| `BLEND_CHECK_SOLL_WINKEL` | **BLEND** · controle · **SOLL** · **WINKEL** |
| `BLINKIMPULS_CRASHBLK` | **BLINKIMPULS** · **CRASHBLK** |
| `BLINKIMPULS_WARNBLK` | **BLINKIMPULS** · **WARNBLK** |
| `CAN_11H` | bus CAN · [0x11] |
| `CBD_ZEICHN_INDEX` | [index variante CBD] · **ZEICHN** · **INDEX** |
| `CC_MELD_BFD_BL` | Check Control · message · feux stop adaptatifs (Brake Force Display) · feu stop |
| `CC_MELD_NSL_BFD` | Check Control · message · antibrouillard arriere · feux stop adaptatifs (Brake Force Display) |
| `CC_MELD_NSL_SL` | Check Control · message · antibrouillard arriere · feux de position |
| `CHECK_FGN_SMC_ALC_OFF` | controle · **FGN** · **SMC** · phares directionnels (ALC) · arret |
| `CHECK_SMC_ERROR` | controle · **SMC** · **ERROR** |
| `CKM_ALC` | memoire par cle (Car Key Memory) · phares directionnels (ALC) |
| `CKM_ALC_DEF` | memoire par cle (Car Key Memory) · phares directionnels (ALC) · defaut |
| `COMFORT_ACCESS_LICHT` | **COMFORT** · **ACCESS** · lumiere |
| `CRASH_58G_EIN` | choc · **58G** · marche |
| `CRASH_WARNBLINKEN` | choc · **WARNBLINKEN** |
| `C_METER_NACH_INNEN_HI` | C · **METER** · apres · interieur · haut |
| `DEF_HEIMLEUCHTEN_CKM` | defaut · eclairage d'accompagnement · memoire par cle (Car Key Memory) |
| `DEF_QUITT_BLK_ENTSI_CKM` | defaut · acquittement sonore · clignotant · **ENTSI** · memoire par cle (Car Key Memory) |
| `DEF_QUITT_BLK_SICH_CKM` | defaut · acquittement sonore · clignotant · **SICH** · memoire par cle (Car Key Memory) |
| `DEF_TAGFAHRLICHT_ECE_CKM` | defaut · feux de jour · norme europeenne · memoire par cle (Car Key Memory) |
| `DEF_ZYKL_TIPP_BLK_CKM` | defaut · cycle · impulsion · clignotant · memoire par cle (Car Key Memory) |
| `DRIFT_DEKR` | derive · **DEKR** |
| `DRIFT_INKR` | derive · **INKR** |
| `DRIFT_SCHWELLE` | derive · seuil |
| `DRL_V_GEDIMMT` | feux de jour · _V_ · **GEDIMMT** |
| `DUAL_SP__AL_L` | bi-zone · point de passage de rapport · feux de croisement · _L_ |
| `DUAL_SP__AL_R` | bi-zone · point de passage de rapport · feux de croisement · _R_ |
| `DUAL_SP__BLK_H_L` | bi-zone · point de passage de rapport · clignotant · _H_ · _L_ |
| `DUAL_SP__BLK_H_R` | bi-zone · point de passage de rapport · clignotant · _H_ · _R_ |
| `DUAL_SP__BLK_V_L` | bi-zone · point de passage de rapport · clignotant · _V_ · _L_ |
| `DUAL_SP__BLK_V_R` | bi-zone · point de passage de rapport · clignotant · _V_ · _R_ |
| `DUAL_SP__BL_L` | bi-zone · point de passage de rapport · feu stop · _L_ |
| `DUAL_SP__BL_M_1` | bi-zone · point de passage de rapport · feu stop · M · 1 |
| `DUAL_SP__BL_R` | bi-zone · point de passage de rapport · feu stop · _R_ |
| `DUAL_SP__FL_L` | bi-zone · point de passage de rapport · feux de route · _L_ |
| `DUAL_SP__FL_R` | bi-zone · point de passage de rapport · feux de route · _R_ |
| `DUAL_SP__KZL` | bi-zone · point de passage de rapport · eclairage de plaque |
| `DUAL_SP__NSL_L` | bi-zone · point de passage de rapport · antibrouillard arriere · _L_ |
| `DUAL_SP__NSL_R` | bi-zone · point de passage de rapport · antibrouillard arriere · _R_ |
| `DUAL_SP__NSW_L` | bi-zone · point de passage de rapport · antibrouillard avant · _L_ |
| `DUAL_SP__NSW_R` | bi-zone · point de passage de rapport · antibrouillard avant · _R_ |
| `DUAL_SP__RFS_L` | bi-zone · point de passage de rapport · feu de recul · _L_ |
| `DUAL_SP__RFS_R` | bi-zone · point de passage de rapport · feu de recul · _R_ |
| `DUAL_SP__RL_BL_1_L` | bi-zone · point de passage de rapport · feu arriere · feu stop · 1 · _L_ |
| `DUAL_SP__RL_BL_1_R` | bi-zone · point de passage de rapport · feu arriere · feu stop · 1 · _R_ |
| `DUAL_SP__RL_BL_2_L` | bi-zone · point de passage de rapport · feu arriere · feu stop · 2 · _L_ |
| `DUAL_SP__RL_BL_2_R` | bi-zone · point de passage de rapport · feu arriere · feu stop · 2 · _R_ |
| `DUAL_SP__SL_V_L` | bi-zone · point de passage de rapport · feux de position · _V_ · _L_ |
| `DUAL_SP__SL_V_R` | bi-zone · point de passage de rapport · feux de position · _V_ · _R_ |
| `DUAL_SP___BFD_L` | bi-zone · point de passage de rapport · feux stop adaptatifs (Brake Force Display) · _L_ |
| `DUAL_SP___BFD_R` | bi-zone · point de passage de rapport · feux stop adaptatifs (Brake Force Display) · _R_ |
| `DYNAM_LWR_ENABLE` | **DYNAM** · correcteur de site des phares · activation |
| `D_TAU_MITTEL_GIERR` | D · _TAU_ · moyen · **GIERR** |
| `D_TAU_MITTEL_LENKW` | D · _TAU_ · moyen · **LENKW** |
| `ENABLE_GURTBRINGER` | activation · avance-ceinture |
| `ERR_LWR_L_IGNORIEREN` | erreur · correcteur de site des phares · _L_ · **IGNORIEREN** |
| `ERR_LWR_R_IGNORIEREN` | erreur · correcteur de site des phares · _R_ · **IGNORIEREN** |
| `ERR_SCHWENK_L_IGNORIEREN` | erreur · pivotement · _L_ · **IGNORIEREN** |
| `ERR_SCHWENK_R_IGNORIEREN` | erreur · pivotement · _R_ · **IGNORIEREN** |
| `FAHRWEG_BORDSTEIN_AUTOM` | **FAHRWEG** · bordure de trottoir · automatique |
| `FEHLERMELDUNG_CC_AL_L` | message d'erreur · Check Control · feux de croisement · _L_ |
| `FEHLERMELDUNG_CC_AL_R` | message d'erreur · Check Control · feux de croisement · _R_ |
| `FEHLERMELDUNG_CC_BFD_L` | message d'erreur · Check Control · feux stop adaptatifs (Brake Force Display) · _L_ |
| `FEHLERMELDUNG_CC_BFD_R` | message d'erreur · Check Control · feux stop adaptatifs (Brake Force Display) · _R_ |
| `FEHLERMELDUNG_CC_BLK_H_L` | message d'erreur · Check Control · clignotant · _H_ · _L_ |
| `FEHLERMELDUNG_CC_BLK_H_R` | message d'erreur · Check Control · clignotant · _H_ · _R_ |
| `FEHLERMELDUNG_CC_BLK_V_L` | message d'erreur · Check Control · clignotant · _V_ · _L_ |
| `FEHLERMELDUNG_CC_BLK_V_R` | message d'erreur · Check Control · clignotant · _V_ · _R_ |
| `FEHLERMELDUNG_CC_BL_L` | message d'erreur · Check Control · feu stop · _L_ |
| `FEHLERMELDUNG_CC_BL_M_1` | message d'erreur · Check Control · feu stop · M · 1 |
| `FEHLERMELDUNG_CC_BL_R` | message d'erreur · Check Control · feu stop · _R_ |
| `FEHLERMELDUNG_CC_FL_L` | message d'erreur · Check Control · feux de route · _L_ |
| `FEHLERMELDUNG_CC_FL_R` | message d'erreur · Check Control · feux de route · _R_ |
| `FEHLERMELDUNG_CC_KZL` | message d'erreur · Check Control · eclairage de plaque |
| `FEHLERMELDUNG_CC_NSL_L` | message d'erreur · Check Control · antibrouillard arriere · _L_ |
| `FEHLERMELDUNG_CC_NSL_R` | message d'erreur · Check Control · antibrouillard arriere · _R_ |
| `FEHLERMELDUNG_CC_NSW_L` | message d'erreur · Check Control · antibrouillard avant · _L_ |
| `FEHLERMELDUNG_CC_NSW_R` | message d'erreur · Check Control · antibrouillard avant · _R_ |
| `FEHLERMELDUNG_CC_RFS_L` | message d'erreur · Check Control · feu de recul · _L_ |
| `FEHLERMELDUNG_CC_RFS_R` | message d'erreur · Check Control · feu de recul · _R_ |
| `FEHLERMELDUNG_CC_SL_V_L` | message d'erreur · Check Control · feux de position · _V_ · _L_ |
| `FEHLERMELDUNG_CC_SL_V_R` | message d'erreur · Check Control · feux de position · _V_ · _R_ |
| `FEHLERMELD_CC_RL_BL_1_L` | **FEHLERMELD** · Check Control · feu arriere · feu stop · 1 · _L_ |
| `FEHLERMELD_CC_RL_BL_1_R` | **FEHLERMELD** · Check Control · feu arriere · feu stop · 1 · _R_ |
| `FEHLERMELD_CC_RL_BL_2_L` | **FEHLERMELD** · Check Control · feu arriere · feu stop · 2 · _L_ |
| `FEHLERMELD_CC_RL_BL_2_R` | **FEHLERMELD** · Check Control · feu arriere · feu stop · 2 · _R_ |
| `FH_CC_MELDUNGEN` | leve-vitre · Check Control · **MELDUNGEN** |
| `FH_FAHRERTUER_TIPP_AUF` | leve-vitre · **FAHRERTUER** · impulsion · ouverture |
| `FH_FAHRERTUER_TIPP_ZU` | leve-vitre · **FAHRERTUER** · impulsion · fermeture |
| `FL_ABSCHALT_OPT` | feux de route · coupure · option |
| `FL_UND_NSW_NICHT_GLEICHZ` | feux de route · **UND** · antibrouillard avant · non · **GLEICHZ** |
| `FSP_ISP_SPERREN` | **FSP** · **ISP** · **SPERREN** |
| `FZG_RADSTAND` | vehicule · **RADSTAND** |
| `GB_DELTA_POS` | **GB** · ecart · position |
| `GB_GESCHW` | **GB** · vitesse |
| `GB_POS_AUS` | **GB** · position · arret |
| `GB_T_EINFAHREN` | **GB** · T · **EINFAHREN** |
| `GB_T_NOTLAUF` | **GB** · T · mode degrade |
| `GB_USE_SBE` | **GB** · **USE** · **SBE** |
| `GESCHW_ASP_AUTO_AUSKLAPP` | vitesse · retroviseur exterieur · automatique · **AUSKLAPP** |
| `GESCHW_DOWN` | vitesse · bas |
| `GESCHW_UP` | vitesse · haut |
| `GETRIEBE_HANDSCHALTER` | boite de vitesses · **HANDSCHALTER** |
| `HECKDECK_REVERS_BLK` | **HECKDECK** · **REVERS** · clignotant |
| `HECKD_REV_BLK_DOPPELB` | **HECKD** · revision · clignotant · **DOPPELB** |
| `HEIMLEUCHTEN` | eclairage d'accompagnement |
| `HEIMLEUCHTEN_ABBRUCH` | eclairage d'accompagnement · **ABBRUCH** |
| `HEIMLEUCHTEN_ENABLE` | eclairage d'accompagnement · activation |
| `HEIMLEUCHTEN_VERLAENG` | eclairage d'accompagnement · **VERLAENG** |
| `IBUS_R_GANG` | **IBUS** · _R_ · **GANG** |
| `IB_2MIN_OPTION` | **IB** · **2MIN** · option |
| `IB_SOFTON_OPTION` | **IB** · **SOFTON** · option |
| `IK_BUS_TEILNEMER` | **IK** · bus · **TEILNEMER** |
| `KALTUEBERWACHUNG_AL_L` | surveillance a froid · feux de croisement · _L_ |
| `KALTUEBERWACHUNG_AL_R` | surveillance a froid · feux de croisement · _R_ |
| `KALTUEBERWACHUNG_BFD_L` | surveillance a froid · feux stop adaptatifs (Brake Force Display) · _L_ |
| `KALTUEBERWACHUNG_BFD_R` | surveillance a froid · feux stop adaptatifs (Brake Force Display) · _R_ |
| `KALTUEBERWACHUNG_BLK_H_L` | surveillance a froid · clignotant · _H_ · _L_ |
| `KALTUEBERWACHUNG_BLK_H_R` | surveillance a froid · clignotant · _H_ · _R_ |
| `KALTUEBERWACHUNG_BLK_V_L` | surveillance a froid · clignotant · _V_ · _L_ |
| `KALTUEBERWACHUNG_BLK_V_R` | surveillance a froid · clignotant · _V_ · _R_ |
| `KALTUEBERWACHUNG_BL_L` | surveillance a froid · feu stop · _L_ |
| `KALTUEBERWACHUNG_BL_M_1` | surveillance a froid · feu stop · M · 1 |
| `KALTUEBERWACHUNG_BL_R` | surveillance a froid · feu stop · _R_ |
| `KALTUEBERWACHUNG_FL_L` | surveillance a froid · feux de route · _L_ |
| `KALTUEBERWACHUNG_FL_R` | surveillance a froid · feux de route · _R_ |
| `KALTUEBERWACHUNG_KZL` | surveillance a froid · eclairage de plaque |
| `KALTUEBERWACHUNG_NSL_L` | surveillance a froid · antibrouillard arriere · _L_ |
| `KALTUEBERWACHUNG_NSL_R` | surveillance a froid · antibrouillard arriere · _R_ |
| `KALTUEBERWACHUNG_NSW_L` | surveillance a froid · antibrouillard avant · _L_ |
| `KALTUEBERWACHUNG_NSW_R` | surveillance a froid · antibrouillard avant · _R_ |
| `KALTUEBERWACHUNG_RFS_L` | surveillance a froid · feu de recul · _L_ |
| `KALTUEBERWACHUNG_RFS_R` | surveillance a froid · feu de recul · _R_ |
| `KALTUEBERWACHUNG_SL_V_L` | surveillance a froid · feux de position · _V_ · _L_ |
| `KALTUEBERWACHUNG_SL_V_R` | surveillance a froid · feux de position · _V_ · _R_ |
| `KALTUEBERW_RL_BL_1_L` | **KALTUEBERW** · feu arriere · feu stop · 1 · _L_ |
| `KALTUEBERW_RL_BL_1_R` | **KALTUEBERW** · feu arriere · feu stop · 1 · _R_ |
| `KALTUEBERW_RL_BL_2_L` | **KALTUEBERW** · feu arriere · feu stop · 2 · _L_ |
| `KALTUEBERW_RL_BL_2_R` | **KALTUEBERW** · feu arriere · feu stop · 2 · _R_ |
| `KEINE_ENTRFLANKE_4_IB` | aucun · **ENTRFLANKE** · 4 · **IB** |
| `KEINE_L_R_SYNCHRONISATION` | aucun · _L_ · _R_ · **SYNCHRONISATION** |
| `KEINE_PARKLICHT_FUNKT` | aucun · feu de stationnement · **FUNKT** |
| `KENNL_MAX_UEBERST` | courbe caracteristique · maximum · survirage |
| `KENNL_REDUZ_MAX_SCHWENK` | courbe caracteristique · **REDUZ** · maximum · pivotement |
| `KLASSIF_DRIFT_ENA` | **KLASSIF** · derive · activation |
| `KLASSIF_LENKM_ENA` | **KLASSIF** · **LENKM** · activation |
| `KLASSIF_UEBERST_ENA` | **KLASSIF** · survirage · activation |
| `KLS_BEI_AL_AKTIV` | **KLS** · lors de · feux de croisement · actif |
| `KL_VA_EIN_BEI_ENTRIEGELN` | borne · essieu avant · marche · lors de · **ENTRIEGELN** |
| `KURZSCHLUSS_CHECK_TIME` | court-circuit · controle · duree |
| `KURZSCHLUSS_DIAGNOSE_OPT` | court-circuit · **DIAGNOSE** · option |
| `KURZSCHLUSS_ENTPRELL` | court-circuit · anti-rebond |
| `KURZSCHL_WIERHOL_ZAEHLER` | **KURZSCHL** · **WIERHOL** · compteur |
| `LENKSTOCK_UMSCHALT_VERZ` | **LENKSTOCK** · **UMSCHALT** · temporisation |
| `LENKWINKEL_BEI_KLR` | angle de braquage · lors de · regulation de climatisation |
| `LENKWINKEL_UEBERSCHREITEN` | angle de braquage · **UEBERSCHREITEN** |
| `LENKWINKEL_UNTERSCHREITEN` | angle de braquage · **UNTERSCHREITEN** |
| `LENK_UEBERSETZUNG` | direction · **UEBERSETZUNG** |
| `LENK_UEBERSETZUNG_LO` | direction · **UEBERSETZUNG** · bas |
| `LEUCHTDICHTE_EXPO` | **LEUCHTDICHTE** · **EXPO** |
| `LICHTZ_R_L_FAKTOR` | **LICHTZ** · _R_ · _L_ · facteur |
| `LICHT_AN_LED` | lumiere · **AN** · LED |
| `LICHT_AN_WARNUNG` | lumiere · **AN** · alerte |
| `LICHT_AN_WARN_KL_R` | lumiere · **AN** · alerte · borne · _R_ |
| `LINKSVERKEHR` | **LINKSVERKEHR** |
| `LOAD_DUMP_DIMM_WERL_BLK` | charge · extraction memoire · gradation · **WERL** · clignotant |
| `LOAD_DUMP_MAX_TIME` | charge · extraction memoire · maximum · duree |
| `LOAD_DUMP_PRELL` | charge · extraction memoire · **PRELL** |
| `LOAD_DUMP_SCHUTZ_AKTIV` | charge · extraction memoire · protection · actif |
| `LOAD_DUMP_SCHUTZ_BLK_Z` | charge · extraction memoire · protection · clignotant · Z |
| `LOAD_DUMP_SCHUTZ_FL` | charge · extraction memoire · protection · feux de route |
| `LOAD_DUMP_SCHUTZ_KL58G` | charge · extraction memoire · protection · **KL58G** |
| `LOAD_DUMP_SCHUTZ_KZL` | charge · extraction memoire · protection · eclairage de plaque |
| `LOAD_DUMP_SCHUTZ_NSL` | charge · extraction memoire · protection · antibrouillard arriere |
| `LOAD_DUMP_SCHUTZ_NSW` | charge · extraction memoire · protection · antibrouillard avant |
| `LOAD_DUMP_SCHUTZ_RFS` | charge · extraction memoire · protection · feu de recul |
| `LOAD_DUMP_SCHUTZ_RL_BL_2` | charge · extraction memoire · protection · feu arriere · feu stop · 2 |
| `LOAD_DUMP_SCHUTZ_SL_V` | charge · extraction memoire · protection · feux de position · _V_ |
| `LWR_ABSTELLPOS` | correcteur de site des phares · **ABSTELLPOS** |
| `LWR_ABS_MIN_STELLUNG_2` | correcteur de site des phares · ABS · minimum · position · 2 |
| `LWR_ANSTEUERN` | correcteur de site des phares · **ANSTEUERN** |
| `LWR_DIAG_PARAM` | correcteur de site des phares · **DIAG** · **PARAM** |
| `LWR_FEHLER_BEWERTUNG` | correcteur de site des phares · erreur · **BEWERTUNG** |
| `LWR_HEBEL` | correcteur de site des phares · **HEBEL** |
| `LWR_HYSTERESE_START` | correcteur de site des phares · **HYSTERESE** · demarrage |
| `LWR_SCHRITTABSTAND_GRENZE` | correcteur de site des phares · **SCHRITTABSTAND** · limite |
| `LWR_SCHRITTE_STELLUNG_0` | correcteur de site des phares · **SCHRITTE** · position · 0 |
| `LWR_SCHRITTE_STELLUNG_1` | correcteur de site des phares · **SCHRITTE** · position · 1 |
| `LWR_SCHRITTE_STELLUNG_2` | correcteur de site des phares · **SCHRITTE** · position · 2 |
| `LWR_TEMPERATUR_GRENZE` | correcteur de site des phares · temperature · limite |
| `LWR_TERMO_MODEL_EIN` | correcteur de site des phares · **TERMO** · **MODEL** · marche |
| `LWR_TESTBETRIEB` | correcteur de site des phares · **TESTBETRIEB** |
| `LWR_VORBESTROMUNG` | correcteur de site des phares · **VORBESTROMUNG** |
| `MAX_DELTA_KEINE_VERR` | maximum · ecart · aucun · verrouillage |
| `MAX_GESCHW_EINKLAPPEN_2` | maximum · vitesse · rabattement · 2 |
| `MAX_SW_SCHWENK_V_HORIZ` | maximum · logiciel · pivotement · _V_ · **HORIZ** |
| `MAX_SW_SCHWENK_V_VERTI` | maximum · logiciel · pivotement · _V_ · **VERTI** |
| `MAX_SW_SCHWENK_W_AUSSEN` | maximum · logiciel · pivotement · W · exterieur |
| `MAX_SW_SCHWENK_W_INNEN` | maximum · logiciel · pivotement · W · interieur |
| `MAX_VERRINGERUNG_V_SCHW` | maximum · **VERRINGERUNG** · _V_ · seuil |
| `MELDUNG_AENDERUNG_DIMMER` | message · **AENDERUNG** · **DIMMER** |
| `MEM_POS_ANFAHREN` | **MEM** · position · **ANFAHREN** |
| `MERKM_1_BEGR` | caracteristique · 1 · **BEGR** |
| `MERKM_1_DEKR` | caracteristique · 1 · **DEKR** |
| `MERKM_1_INKR` | caracteristique · 1 · **INKR** |
| `MERKM_1_MAX` | caracteristique · 1 · maximum |
| `MIND_ANZ_ZYKL_TIPP_BLK` | minimum · nombre · cycle · impulsion · clignotant |
| `MIN_DELTA_MAX_VERR` | minimum · ecart · maximum · verrouillage |
| `MIN_SW_SCHWENK_V_HORIZ` | minimum · logiciel · pivotement · _V_ · **HORIZ** |
| `MIN_SW_SCHWENK_V_VERTI` | minimum · logiciel · pivotement · _V_ · **VERTI** |
| `NACHLAUF_KLEMME_VA` | marche a suivre / temporisation · **KLEMME** · essieu avant |
| `NORM_LEUCHTWEITE` | norme · **LEUCHTWEITE** |
| `NOTLAUF_FKT_GESCHW` | mode degrade · fonction · vitesse |
| `NOTLAUF_FKT_SCHALTERSTELL` | mode degrade · fonction · **SCHALTERSTELL** |
| `NOTLAUF_FKT_TIMER` | mode degrade · fonction · minuterie |
| `NSL_ERS_DEF_RL_BL_1` | antibrouillard arriere · **ERS** · defaut · feu arriere · feu stop · 1 |
| `NSL_VERBAU_OTP` | antibrouillard arriere · **VERBAU** · **OTP** |
| `OHNE_GIERRATE` | sans · vitesse de lacet |
| `OPTIONEN_SL_ABSCH` | **OPTIONEN** · feux de position · coupure |
| `PANIKMODUS_EKS` | mode panique · systeme electronique de cle |
| `POTI_DIMM_INVERS` | potentiometre · gradation · **INVERS** |
| `POTI_LWR_INVERS` | potentiometre · correcteur de site des phares · **INVERS** |
| `PRE_SLEEP_ZEIT` | pre- · **SLEEP** · duree |
| `PROZENT_EE` | pourcentage · **EE** |
| `QUITT_BLK_ENTSICHERN` | acquittement sonore · clignotant · deverrouillage |
| `QUITT_BLK_IGNORE_MOTORH` | acquittement sonore · clignotant · **IGNORE** · **MOTORH** |
| `QUITT_BLK_SICHERN` | acquittement sonore · clignotant · verrouillage |
| `QUITT_BLK_ZU_CHECK` | acquittement sonore · clignotant · fermeture · controle |
| `RAUSCH` | **RAUSCH** |
| `RECHTSVERKEHR` | **RECHTSVERKEHR** |
| `RECHTS_LENKER_ASP` | droite · volant · retroviseur exterieur |
| `RECHTS_LENKER_FH_H` | droite · volant · leve-vitre · _H_ |
| `RECHTS_LENKER_FH_V` | droite · volant · leve-vitre · _V_ |
| `REDUZIERUNG_V_SCHWENK_HI` | reduction · _V_ · pivotement · haut |
| `REDUZIERUNG_V_SCHWENK_LO` | reduction · _V_ · pivotement · bas |
| `RL_BL_1_BREMSL_FKT` | feu arriere · feu stop · 1 · feu stop · fonction |
| `RL_BL_1_PARKL_FKT` | feu arriere · feu stop · 1 · feu de stationnement · fonction |
| `RL_BL_1_PO_RL_FKT` | feu arriere · feu stop · 1 · **PO** · feu arriere · fonction |
| `RL_BL_2_BREMSL_FKT` | feu arriere · feu stop · 2 · feu stop · fonction |
| `RL_BL_2_PARKL_FKT` | feu arriere · feu stop · 2 · feu de stationnement · fonction |
| `RL_BL_2_PO_RL_FKT` | feu arriere · feu stop · 2 · **PO** · feu arriere · fonction |
| `RL_BL_BREMSL_FKT` | feu arriere · feu stop · feu stop · fonction |
| `RL_BL_PARKL_FKT` | feu arriere · feu stop · feu de stationnement · fonction |
| `RL_BL_PO_RL_FKT` | feu arriere · feu stop · **PO** · feu arriere · fonction |
| `SCHALTER_R_GANG` | **SCHALTER** · _R_ · **GANG** |
| `SCHW_TAU_LW_GR` | seuil · _TAU_ · **LW** · **GR** |
| `SICHERHEITS_FZG_LM` | **SICHERHEITS** · vehicule · **LM** |
| `SIDEMARKER_US` | feu lateral (US) · marche americain |
| `SL_BL_CC_MELD_E8X` | feux de position · feu stop · Check Control · message · **E8X** |
| `SL_BL_ERS_DEF_RL_BL_1` | feux de position · feu stop · **ERS** · defaut · feu arriere · feu stop · 1 |
| `SL_PARKL_UNTERSP_ABSCHALT` | feux de position · feu de stationnement · **UNTERSP** · coupure |
| `SL_V_PARKL_FKT` | feux de position · _V_ · feu de stationnement · fonction |
| `SL_V_PO_RL_FKT` | feux de position · _V_ · **PO** · feu arriere · fonction |
| `SPIEGEL_BORDSTEIN_AUTOM` | retroviseur · bordure de trottoir · automatique |
| `SPIEGEL_EINKLAPPEN` | retroviseur · rabattement |
| `SPIEGEL_ELEKTROCHROM` | retroviseur · **ELEKTROCHROM** |
| `SPIEGEL_HEIZUNG` | retroviseur · **HEIZUNG** |
| `SPIEGEL_KOMFORT_EINKLAPP` | retroviseur · confort · rabattement |
| `SPIEGEL_MEMORY` | retroviseur · memoire |
| `SPIEGEL_STAND_HEIZUNG` | retroviseur · etat · **HEIZUNG** |
| `SP_GRENZE` | point de passage de rapport · limite |
| `SP_NACHREG_LANGSAM` | point de passage de rapport · **NACHREG** · **LANGSAM** |
| `STANDVERBRAUCHER_ANMELDEN` | **STANDVERBRAUCHER** · **ANMELDEN** |
| `SW_NULL_BEREICH` | logiciel · **NULL** · plage |
| `TAGFAHRLICHT_ECE` | feux de jour · norme europeenne |
| `TASTVERH_BEL_FLC_LED` | rapport cyclique · **BEL** · _FLC_ · LED |
| `TASTVERH_BEL_WBL_MIN` | rapport cyclique · **BEL** · **WBL** · minimum |
| `TASTVERH_KL_58G_MIN` | rapport cyclique · borne · **58G** · minimum |
| `TAU_ABSCHWAECHUNG` | _TAU_ · **ABSCHWAECHUNG** |
| `TIPP_BLK_ABBRUCH` | impulsion · clignotant · **ABBRUCH** |
| `TIPP_BLK_LOESCHT_IMMER` | impulsion · clignotant · **LOESCHT** · toujours |
| `TUERAUF_STOP_MAUT` | porte ouverte · arret · peage |
| `TYP_FH_BEDIENBLOCK` | type · leve-vitre · **BEDIENBLOCK** |
| `T_FILTER_DYN_LZ` | T · filtre · dynamique · **LZ** |
| `T_FILTER_GIERRATE` | T · filtre · vitesse de lacet |
| `T_FILTER_LENKWINKEL` | T · filtre · angle de braquage |
| `T_FILTER_TAU_LW` | T · filtre · _TAU_ · **LW** |
| `T_FILTER_UEBERST` | T · filtre · survirage |
| `T_FILTER_V_VEH` | T · filtre · _V_ · **VEH** |
| `T_F_GIERRATE_DEF` | T · F · vitesse de lacet · defaut |
| `UMSCHALTOPT_LICHTSCHALTER` | **UMSCHALTOPT** · **LICHTSCHALTER** |
| `UNP_HEIMLEUCHTEN_CKM` | _UNP_ · eclairage d'accompagnement · memoire par cle (Car Key Memory) |
| `UNP_QUITT_BLK_ENTSI_CKM` | _UNP_ · acquittement sonore · clignotant · **ENTSI** · memoire par cle (Car Key Memory) |
| `UNP_QUITT_BLK_SICH_CKM` | _UNP_ · acquittement sonore · clignotant · **SICH** · memoire par cle (Car Key Memory) |
| `UNP_TAGFAHRLICHT_ECE_CKM` | _UNP_ · feux de jour · norme europeenne · memoire par cle (Car Key Memory) |
| `UNP_ZYKL_TIPP_BLK_CKM` | _UNP_ · cycle · impulsion · clignotant · memoire par cle (Car Key Memory) |
| `USA_REVERSE` | marche americain · **REVERSE** |
| `U_LWR_POTI_STELLUNG_0` | U · correcteur de site des phares · potentiometre · position · 0 |
| `U_LWR_POTI_STELLUNG_1` | U · correcteur de site des phares · potentiometre · position · 1 |
| `U_LWR_POTI_STELLUNG_2` | U · correcteur de site des phares · potentiometre · position · 2 |
| `U_POTI_DIMM_DELTA` | U · potentiometre · gradation · ecart |
| `U_POTI_DIMM_MIN` | U · potentiometre · gradation · minimum |
| `VIER_TUERER` | **VIER** · **TUERER** |
| `VORFELD_BEL_GEDIMMT` | **VORFELD** · **BEL** · **GEDIMMT** |
| `V_HI_ABSCHW_D_TAU` | _V_ · haut · attenuation · D · _TAU_ |
| `V_HI_AENDER_LENK_UEBERS` | _V_ · haut · **AENDER** · direction · **UEBERS** |
| `V_HI_REDUZIER_V_SCHWENK` | _V_ · haut · reduction · _V_ · pivotement |
| `V_HI_RED_SCHWENK_W` | _V_ · haut · reduction · pivotement · W |
| `V_HI_SW_MITLENKEN` | _V_ · haut · logiciel · suivi de direction |
| `V_HI_SW_NACH_INNEN` | _V_ · haut · logiciel · apres · interieur |
| `V_HI_TAU_GIERRATE` | _V_ · haut · _TAU_ · vitesse de lacet |
| `V_LO_ABSCHW_D_TAU` | _V_ · bas · attenuation · D · _TAU_ |
| `V_LO_AENDER_LENK_UEBERS` | _V_ · bas · **AENDER** · direction · **UEBERS** |
| `V_LO_REDUZIER_V_SCHWENK` | _V_ · bas · reduction · _V_ · pivotement |
| `V_LO_RED_SCHWENK_W` | _V_ · bas · reduction · pivotement · W |
| `V_LO_SW_MITLENKEN` | _V_ · bas · logiciel · suivi de direction |
| `V_LO_SW_NACH_INNEN` | _V_ · bas · logiciel · apres · interieur |
| `V_LO_TAU_GIERRATE` | _V_ · bas · _TAU_ · vitesse de lacet |
| `V_M_HI_REDUZIER_V_SCHWENK` | _V_ · M · haut · reduction · _V_ · pivotement |
| `V_M_LO_REDUZIER_V_SCHWENK` | _V_ · M · bas · reduction · _V_ · pivotement |
| `V_REFERENZLAUF_EN` | _V_ · **REFERENZLAUF** · **EN** |
| `WARMUEBERWACHUNG_AL_L` | surveillance a chaud · feux de croisement · _L_ |
| `WARMUEBERWACHUNG_AL_R` | surveillance a chaud · feux de croisement · _R_ |
| `WARMUEBERWACHUNG_BFD_L` | surveillance a chaud · feux stop adaptatifs (Brake Force Display) · _L_ |
| `WARMUEBERWACHUNG_BFD_R` | surveillance a chaud · feux stop adaptatifs (Brake Force Display) · _R_ |
| `WARMUEBERWACHUNG_BLK_H_L` | surveillance a chaud · clignotant · _H_ · _L_ |
| `WARMUEBERWACHUNG_BLK_H_R` | surveillance a chaud · clignotant · _H_ · _R_ |
| `WARMUEBERWACHUNG_BLK_V_L` | surveillance a chaud · clignotant · _V_ · _L_ |
| `WARMUEBERWACHUNG_BLK_V_R` | surveillance a chaud · clignotant · _V_ · _R_ |
| `WARMUEBERWACHUNG_BL_L` | surveillance a chaud · feu stop · _L_ |
| `WARMUEBERWACHUNG_BL_M_1` | surveillance a chaud · feu stop · M · 1 |
| `WARMUEBERWACHUNG_BL_R` | surveillance a chaud · feu stop · _R_ |
| `WARMUEBERWACHUNG_FL_L` | surveillance a chaud · feux de route · _L_ |
| `WARMUEBERWACHUNG_FL_R` | surveillance a chaud · feux de route · _R_ |
| `WARMUEBERWACHUNG_KZL` | surveillance a chaud · eclairage de plaque |
| `WARMUEBERWACHUNG_NSL_L` | surveillance a chaud · antibrouillard arriere · _L_ |
| `WARMUEBERWACHUNG_NSL_R` | surveillance a chaud · antibrouillard arriere · _R_ |
| `WARMUEBERWACHUNG_NSW_L` | surveillance a chaud · antibrouillard avant · _L_ |
| `WARMUEBERWACHUNG_NSW_R` | surveillance a chaud · antibrouillard avant · _R_ |
| `WARMUEBERWACHUNG_RFS_L` | surveillance a chaud · feu de recul · _L_ |
| `WARMUEBERWACHUNG_RFS_R` | surveillance a chaud · feu de recul · _R_ |
| `WARMUEBERWACHUNG_SL_V_L` | surveillance a chaud · feux de position · _V_ · _L_ |
| `WARMUEBERWACHUNG_SL_V_R` | surveillance a chaud · feux de position · _V_ · _R_ |
| `WARMUEBERW_RL_BL_1_L` | **WARMUEBERW** · feu arriere · feu stop · 1 · _L_ |
| `WARMUEBERW_RL_BL_1_R` | **WARMUEBERW** · feu arriere · feu stop · 1 · _R_ |
| `WARMUEBERW_RL_BL_2_L` | **WARMUEBERW** · feu arriere · feu stop · 2 · _L_ |
| `WARMUEBERW_RL_BL_2_R` | **WARMUEBERW** · feu arriere · feu stop · 2 · _R_ |
| `WARNBLK_ENERGY_SAVE_MODE` | **WARNBLK** · **ENERGY** · **SAVE** · mode |
| `WERT_CRASHVERHALTEN` | valeur · **CRASHVERHALTEN** |
| `XENON_AL_VERBAUT` | xenon · feux de croisement · installe |
| `XENON_LI` | xenon · **LI** |
| `XENON_WIEDERHOL_Z_OPT` | xenon · **WIEDERHOL** · Z · option |
| `XENON_WIEDER_ZUEND` | xenon · **WIEDER** · **ZUEND** |
| `XENON_WIEDER_Z_AUS_PHASE` | xenon · **WIEDER** · Z · arret · **PHASE** |
| `ZEITK_TRIGGER1` | **ZEITK** · **TRIGGER1** |
| `ZUWACHS_TASTVERH_FLC_LED` | **ZUWACHS** · rapport cyclique · _FLC_ · LED |
| `ZUWACHS_TASTVERH_KL_58G` | **ZUWACHS** · rapport cyclique · borne · **58G** |
| `ZUWACHS_TASTVERH_WBL_BEL` | **ZUWACHS** · rapport cyclique · **WBL** · **BEL** |

---

Les fragments en **gras** n'ont pas de traduction etablie — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments entre `_underscores_` sont ambigus : leur sens depend du module.
