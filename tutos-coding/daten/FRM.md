# FRM — Module de plancher

🟠 Module `FRM` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Source

| | |
|---|---|
| Variante retenue | `FRMFA.C10` |
| Fonctions codables | **438** |
| Avec valeurs extraites | 437 (99 %) |
| Avec explication rédigée | 75 |
| Fichiers DATEN analysés | 7 |

Variantes disponibles : `FRMFA.C09`, `FRMFA.C10`, `FRMFA.C11`, `FRMFA.C12`, `FRMFA.C21`, `FRM2_E89.C08`, `FRM2_E89.C09`

> La variante réellement installée est identifiée par NCS Dummy à l'ouverture du `.TRC`.
> Si elle diffère de celle retenue ici, la liste change.

---

## Les 438 fonctions codables

| FSW | Traduction | Valeurs possibles | Explication |
|---|---|---|---|
| `ABSCHALTUNG_RFS` | coupure · feu de recul | `aktiv` · `nicht_aktiv` |  |
| `ABSCHWAECH_D_TAU` | **ABSCHWAECH** · D · _TAU_ | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `ABSCHW_D_TAU_HI` | attenuation · D · _TAU_ · haut | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `ABSCHW_D_TAU_LO` | attenuation · D · _TAU_ · bas | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `ABSTELLPOS_CUSTOM` | **ABSTELLPOS** · **CUSTOM** | `nicht_aktiv` · `aktiv` |  |
| `ABSTELLPOS_SMC` | **ABSTELLPOS** · **SMC** | `nicht_aktiv` · `aktiv` |  |
| `AEND_V_SCHWENK_N_AUSSEN` | **AEND** · _V_ · pivotement · N · exterieur | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+10)* |  |
| `ALC_ABTASTRATE_DYN` | phares directionnels (ALC) · **ABTASTRATE** · dynamique | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `ALC_ABTASTRATE_TREND` | phares directionnels (ALC) · **ABTASTRATE** · **TREND** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `ALC_ABWEICHUNG_TREND` | phares directionnels (ALC) · **ABWEICHUNG** · **TREND** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `ALC_AUSWERT_GRADIENT` | phares directionnels (ALC) · **AUSWERT** · **GRADIENT** | `aktiv` · `nicht_aktiv` |  |
| `ALC_AUSWERT_PROP_ABWEICH` | phares directionnels (ALC) · **AUSWERT** · **PROP** · **ABWEICH** | `aktiv` · `nicht_aktiv` |  |
| `ALC_CC_ALC_ERR_OFF` | phares directionnels (ALC) · Check Control · phares directionnels (ALC) · erreur · arret | `nicht_aktiv` · `aktiv` |  |
| `ALC_DAUER_DYN_NACHFUEHR` | phares directionnels (ALC) · duree · dynamique · **NACHFUEHR** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `ALC_DEF_LWR_SENSOR_HI` | phares directionnels (ALC) · defaut · correcteur de site des phares · capteur · haut | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+18)* |  |
| `ALC_DIFFERENZ_MSP_DYN_STA` | phares directionnels (ALC) · **DIFFERENZ** · **MSP** · dynamique · **STA** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `ALC_DISABLE_AUSGL_NICK_GW` | phares directionnels (ALC) · desactivation · **AUSGL** · **NICK** · **GW** | `nicht_aktiv` · `aktiv` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` … *(+20)* |  |
| `ALC_EINSCHALTBEDINGUNG` | phares directionnels (ALC) · **EINSCHALTBEDINGUNG** | `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `ALC_EPSILON_DELTA_A` | phares directionnels (ALC) · **EPSILON** · ecart · A | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+12)* |  |
| `ALC_EPSILON_DELTA_AV` | phares directionnels (ALC) · **EPSILON** · ecart · **AV** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+12)* |  |
| `ALC_ERR_BLK_FREQ` | phares directionnels (ALC) · erreur · clignotant · frequence | `wert_01` · `wert_02` |  |
| `ALC_FAHRZEUG` | phares directionnels (ALC) · **FAHRZEUG** | `aktiv` · `nicht_aktiv` |  |
| `ALC_GRADIENT_SCHLECHTWEG` | phares directionnels (ALC) · **GRADIENT** · **SCHLECHTWEG** | `aktiv` · `nicht_aktiv` |  |
| `ALC_HOEHENWEG` | phares directionnels (ALC) · **HOEHENWEG** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `ALC_KLS_ENABLE` | phares directionnels (ALC) · **KLS** · activation | `nicht_aktiv` · `aktiv` |  |
| `ALC_LWR_NEIGUNG_KONSTANTE` | phares directionnels (ALC) · correcteur de site des phares · inclinaison · **KONSTANTE** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+10)* |  |
| `ALC_LWR_VORZEICHEN` | phares directionnels (ALC) · correcteur de site des phares · **VORZEICHEN** | `nicht_aktiv` · `vorn_aktiv` · `aktiv` · `beide_aktiv` |  |
| `ALC_MINDESTANZAHL_VZ_WECH` | phares directionnels (ALC) · **MINDESTANZAHL** · **VZ** · **WECH** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `ALC_MIN_STEIGUNG_VZ_WECHS` | phares directionnels (ALC) · minimum · **STEIGUNG** · **VZ** · **WECHS** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `ALC_MITTELUNGSINTERV_DYN` | phares directionnels (ALC) · **MITTELUNGSINTERV** · dynamique | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `ALC_MITTELUNGSINTERV_TREN` | phares directionnels (ALC) · **MITTELUNGSINTERV** · **TREN** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `ALC_MITTELUNGSINT_VZ_WECH` | phares directionnels (ALC) · **MITTELUNGSINT** · **VZ** · **WECH** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `ALC_NACHLAUF_SCHLECHTWEG` | phares directionnels (ALC) · marche a suivre / temporisation · **SCHLECHTWEG** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `ALC_OFFSET_LWR` | phares directionnels (ALC) · decalage · correcteur de site des phares | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+25)* |  |
| `ALC_SAETT_TIMER_SCHLECHTW` | phares directionnels (ALC) · **SAETT** · minuterie · **SCHLECHTW** | `aktiv` · `nicht_aktiv` |  |
| `ALC_SCHWENK_LATCH` | phares directionnels (ALC) · pivotement · **LATCH** | `nicht_aktiv` · `aktiv` |  |
| `ALC_SCHW_GRAD_BESCHLEUNIG` | phares directionnels (ALC) · seuil · degre · **BESCHLEUNIG** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+11)* |  |
| `ALC_SCHW_GRAD_BREMSEN` | phares directionnels (ALC) · seuil · degre · **BREMSEN** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+11)* |  |
| `ALC_SCHW_GRAD_SCHLECHTWEG` | phares directionnels (ALC) · seuil · degre · **SCHLECHTWEG** | `wert_01` |  |
| `ALC_SCHW_PROP_ABWEICHUNG` | phares directionnels (ALC) · seuil · **PROP** · **ABWEICHUNG** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+10)* |  |
| `ALC_SCHW_TREND` | phares directionnels (ALC) · seuil · **TREND** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `ALC_VMAX_BESCHLEUNIGUNG` | phares directionnels (ALC) · **VMAX** · **BESCHLEUNIGUNG** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+8)* |  |
| `ALC_VMAX_BREMSEN` | phares directionnels (ALC) · **VMAX** · **BREMSEN** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+8)* |  |
| `ALC_VMIN_BESCHLEUNIGUNG` | phares directionnels (ALC) · **VMIN** · **BESCHLEUNIGUNG** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+8)* |  |
| `ALC_VMIN_BREMSEN` | phares directionnels (ALC) · **VMIN** · **BREMSEN** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+8)* |  |
| `AL_KL50_SCHUTZ` | feux de croisement · borne 50 (demarreur) · protection | `aktiv` · `nicht_aktiv` |  |
| `ASP_KEY_MEM_OHNE_MASTER` | retroviseur exterieur · **KEY** · **MEM** · sans · **MASTER** | `aktiv` · `nicht_aktiv` |  |
| `ASP_LIN_VERBAUT` | retroviseur exterieur · bus LIN · installe | `nicht_aktiv` · `aktiv` |  |
| `AUTOM_LICHT_ABSCHALTUNG` | automatique · lumiere · coupure | `nicht_aktiv` · `aktiv` |  |
| `AUTOM_LICHT_KONTROLLE` | automatique · lumiere · **KONTROLLE** | `nicht_aktiv` · `aktiv` |  |
| `AUTO_LWR_ENABLE` | automatique · correcteur de site des phares · activation | `nicht_aktiv` · `aktiv` |  |
| `BEHOERDENFZG` | vehicule administratif | `nicht_aktiv` · `aktiv` |  |
| `BEHOERDFZG_FL_ASYNCHR` | **BEHOERDFZG** · feux de route · **ASYNCHR** | `nicht_aktiv` · `aktiv` |  |
| `BFD_1_ALGORITHMUS` | feux stop adaptatifs (Brake Force Display) · 1 · **ALGORITHMUS** | `nicht_aktiv` · `aktiv` |  |
| `BFD_1_BFD_AUSG_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 1 · feux stop adaptatifs (Brake Force Display) · **AUSG** · apparition · image | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `BFD_1_BLINK_FREQ_AUS` | feux stop adaptatifs (Brake Force Display) · 1 · clignotant · frequence · arret | `wert_01` |  |
| `BFD_1_BLINK_FREQ_EIN` | feux stop adaptatifs (Brake Force Display) · 1 · clignotant · frequence · marche | `wert_01` |  |
| `BFD_1_BL_M_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 1 · feu stop · M · apparition · image | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `BFD_1_ENTPRELL_ABS` | feux stop adaptatifs (Brake Force Display) · 1 · anti-rebond · ABS | `wert_01` · `wert_02` |  |
| `BFD_1_ENTPRELL_AX` | feux stop adaptatifs (Brake Force Display) · 1 · anti-rebond · **AX** | `wert_01` |  |
| `BFD_1_MIN_GESCHW` | feux stop adaptatifs (Brake Force Display) · 1 · minimum · vitesse | `wert_01` |  |
| `BFD_1_NSL_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 1 · antibrouillard arriere · apparition · image | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `BFD_1_RL_BL_1_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 1 · feu arriere · feu stop · 1 · apparition · image | `wert_00` · `wert_01` · `wert_03` · `wert_04` |  |
| `BFD_1_RL_BL_2_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 1 · feu arriere · feu stop · 2 · apparition · image | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `BFD_1_RL_BL_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 1 · feu arriere · feu stop · apparition · image | `wert_00` · `wert_01` |  |
| `BFD_1_STUFE_DELAY` | feux stop adaptatifs (Brake Force Display) · 1 · niveau · delai | `wert_01` |  |
| `BFD_1_THERM_TIMEOUT` | feux stop adaptatifs (Brake Force Display) · 1 · **THERM** · delai d'attente | `wert_01` · `wert_02` |  |
| `BFD_1_WEGFALL` | feux stop adaptatifs (Brake Force Display) · 1 · **WEGFALL** | `nicht_aktiv` · `aktiv` · `wert_01` · `wert_02` · `wert_03` |  |
| `BFD_2_ALGORITHMUS` | feux stop adaptatifs (Brake Force Display) · 2 · **ALGORITHMUS** | `nicht_aktiv` · `aktiv` |  |
| `BFD_2_BFD_AUSG_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 2 · feux stop adaptatifs (Brake Force Display) · **AUSG** · apparition · image | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `BFD_2_BLINK_FREQ_AUS` | feux stop adaptatifs (Brake Force Display) · 2 · clignotant · frequence · arret | `wert_01` |  |
| `BFD_2_BLINK_FREQ_EIN` | feux stop adaptatifs (Brake Force Display) · 2 · clignotant · frequence · marche | `wert_01` |  |
| `BFD_2_BL_M_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 2 · feu stop · M · apparition · image | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `BFD_2_ENTPRELL_ABS` | feux stop adaptatifs (Brake Force Display) · 2 · anti-rebond · ABS | `wert_01` · `wert_02` |  |
| `BFD_2_ENTPRELL_AX` | feux stop adaptatifs (Brake Force Display) · 2 · anti-rebond · **AX** | `wert_01` |  |
| `BFD_2_MIN_GESCHW` | feux stop adaptatifs (Brake Force Display) · 2 · minimum · vitesse | `wert_01` |  |
| `BFD_2_NSL_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 2 · antibrouillard arriere · apparition · image | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `BFD_2_RL_BL_1_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 2 · feu arriere · feu stop · 1 · apparition · image | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `BFD_2_RL_BL_2_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 2 · feu arriere · feu stop · 2 · apparition · image | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `BFD_2_RL_BL_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 2 · feu arriere · feu stop · apparition · image | `wert_00` · `wert_01` |  |
| `BFD_2_STUFE_DELAY` | feux stop adaptatifs (Brake Force Display) · 2 · niveau · delai | `wert_01` |  |
| `BFD_2_THERM_TIMEOUT` | feux stop adaptatifs (Brake Force Display) · 2 · **THERM** · delai d'attente | `wert_01` |  |
| `BFD_2_WEGFALL` | feux stop adaptatifs (Brake Force Display) · 2 · **WEGFALL** | `nicht_aktiv` · `aktiv` · `wert_01` · `wert_02` · `wert_03` |  |
| `BFD_BREMSL_FKT` | feux stop adaptatifs (Brake Force Display) · feu stop · fonction | `aktiv` · `nicht_aktiv` |  |
| `BFD_ERS_DEF_RL_BL` | feux stop adaptatifs (Brake Force Display) · **ERS** · defaut · feu arriere · feu stop | `aktiv` · `nicht_aktiv` |  |
| `BFD_ERS_DEF_RL_BL_1` | feux stop adaptatifs (Brake Force Display) · **ERS** · defaut · feu arriere · feu stop · 1 | `aktiv` · `nicht_aktiv` |  |
| `BI_XENON_BEI_LH` | **BI** · xenon · lors de · **LH** | `nicht_aktiv` · `aktiv` |  |
| `BI_XENON_FL_ABSCHALT_OPT` | **BI** · xenon · feux de route · coupure · option | `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `BI_XENON_VERBAUT` | **BI** · xenon · installe | `nicht_aktiv` · `aktiv` |  |
| `BLEND_CHECK_IST_WINKEL` | **BLEND** · controle · **IST** · **WINKEL** | `nicht_aktiv` · `aktiv` |  |
| `BLEND_CHECK_SOLL_WINKEL` | **BLEND** · controle · **SOLL** · **WINKEL** | `nicht_aktiv` · `aktiv` |  |
| `BLINKIMPULS_CRASHBLK` | **BLINKIMPULS** · **CRASHBLK** | `wert_01` |  |
| `BLINKIMPULS_WARNBLK` | **BLINKIMPULS** · **WARNBLK** | `wert_01` |  |
| `CAN_11H` | bus CAN · [0x11] | `nicht_aktiv` · `aktiv` |  |
| `CBD_ZEICHN_INDEX` | [variante CBD] · **ZEICHN** · **INDEX** | `wert_01` |  |
| `CC_MELD_BFD_BL` | Check Control · message · feux stop adaptatifs (Brake Force Display) · feu stop | `aktiv` · `nicht_aktiv` |  |
| `CC_MELD_NSL_BFD` | Check Control · message · antibrouillard arriere · feux stop adaptatifs (Brake Force Display) | `aktiv` · `nicht_aktiv` |  |
| `CC_MELD_NSL_SL` | Check Control · message · antibrouillard arriere · feux de position | `aktiv` · `nicht_aktiv` |  |
| `CHECK_FGN_SMC_ALC_OFF` | controle · **FGN** · **SMC** · phares directionnels (ALC) · arret | `nicht_aktiv` · `aktiv` |  |
| `CHECK_SMC_ERROR` | controle · **SMC** · **ERROR** | `nicht_aktiv` · `aktiv` · `wert_01` · `wert_02` | Surveillance des defauts de phare adaptatif. |
| `CKM_ALC` | memoire par cle (Car Key Memory) · phares directionnels (ALC) | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `CKM_ALC_DEF` | memoire par cle (Car Key Memory) · phares directionnels (ALC) · defaut | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `COMFORT_ACCESS_LICHT` | **COMFORT** · **ACCESS** · lumiere | `nicht_aktiv` · `aktiv` |  |
| `CRASH_58G_EIN` | choc · **58G** · marche | `nicht_aktiv` · `aktiv` |  |
| `CRASH_WARNBLINKEN` | choc · **WARNBLINKEN** | `nicht_aktiv` · `aktiv` |  |
| `C_METER_NACH_INNEN_HI` | C · **METER** · apres · interieur · haut | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+10)* |  |
| `DEF_HEIMLEUCHTEN_CKM` | defaut · eclairage d'accompagnement · memoire par cle (Car Key Memory) | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `DEF_QUITT_BLK_ENTSI_CKM` | defaut · acquittement sonore · clignotant · **ENTSI** · memoire par cle (Car Key Memory) | `nicht_aktiv` · `aktiv` |  |
| `DEF_QUITT_BLK_SICH_CKM` | defaut · acquittement sonore · clignotant · **SICH** · memoire par cle (Car Key Memory) | `nicht_aktiv` · `aktiv` |  |
| `DEF_TAGFAHRLICHT_ECE_CKM` | defaut · feux de jour · norme europeenne · memoire par cle (Car Key Memory) | `nicht_aktiv` · `aktiv` |  |
| `DEF_ZYKL_TIPP_BLK_CKM` | defaut · cycle · impulsion · clignotant · memoire par cle (Car Key Memory) | `wert_01` · `wert_02` |  |
| `DRIFT_DEKR` | derive · **DEKR** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `DRIFT_INKR` | derive · **INKR** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `DRIFT_SCHWELLE` | derive · seuil | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `DRL_V_GEDIMMT` | feux de jour · _V_ · **GEDIMMT** | `nicht_aktiv` · `aktiv` |  |
| `DUAL_SP__AL_L` | bi-zone · point de passage de rapport · feux de croisement · _L_ | `nicht_aktiv` · `aktiv` |  |
| `DUAL_SP__AL_R` | bi-zone · point de passage de rapport · feux de croisement · _R_ | `nicht_aktiv` · `aktiv` |  |
| `DUAL_SP__BLK_H_L` | bi-zone · point de passage de rapport · clignotant · _H_ · _L_ | `nicht_aktiv` · `aktiv` |  |
| `DUAL_SP__BLK_H_R` | bi-zone · point de passage de rapport · clignotant · _H_ · _R_ | `nicht_aktiv` · `aktiv` |  |
| `DUAL_SP__BLK_V_L` | bi-zone · point de passage de rapport · clignotant · _V_ · _L_ | `nicht_aktiv` · `aktiv` |  |
| `DUAL_SP__BLK_V_R` | bi-zone · point de passage de rapport · clignotant · _V_ · _R_ | `nicht_aktiv` · `aktiv` |  |
| `DUAL_SP__BL_L` | bi-zone · point de passage de rapport · feu stop · _L_ | `nicht_aktiv` · `aktiv` |  |
| `DUAL_SP__BL_M_1` | bi-zone · point de passage de rapport · feu stop · M · 1 | `nicht_aktiv` · `aktiv` |  |
| `DUAL_SP__BL_R` | bi-zone · point de passage de rapport · feu stop · _R_ | `nicht_aktiv` · `aktiv` |  |
| `DUAL_SP__FL_L` | bi-zone · point de passage de rapport · feux de route · _L_ | `nicht_aktiv` · `aktiv` |  |
| `DUAL_SP__FL_R` | bi-zone · point de passage de rapport · feux de route · _R_ | `nicht_aktiv` · `aktiv` |  |
| `DUAL_SP__KZL` | bi-zone · point de passage de rapport · eclairage de plaque | `nicht_aktiv` · `aktiv` |  |
| `DUAL_SP__NSL_L` | bi-zone · point de passage de rapport · antibrouillard arriere · _L_ | `nicht_aktiv` · `aktiv` |  |
| `DUAL_SP__NSL_R` | bi-zone · point de passage de rapport · antibrouillard arriere · _R_ | `nicht_aktiv` · `aktiv` |  |
| `DUAL_SP__NSW_L` | bi-zone · point de passage de rapport · antibrouillard avant · _L_ | `nicht_aktiv` · `aktiv` |  |
| `DUAL_SP__NSW_R` | bi-zone · point de passage de rapport · antibrouillard avant · _R_ | `nicht_aktiv` · `aktiv` |  |
| `DUAL_SP__RFS_L` | bi-zone · point de passage de rapport · feu de recul · _L_ | `nicht_aktiv` · `aktiv` |  |
| `DUAL_SP__RFS_R` | bi-zone · point de passage de rapport · feu de recul · _R_ | `nicht_aktiv` · `aktiv` |  |
| `DUAL_SP__RL_BL_1_L` | bi-zone · point de passage de rapport · feu arriere · feu stop · 1 · _L_ | `nicht_aktiv` · `aktiv` |  |
| `DUAL_SP__RL_BL_1_R` | bi-zone · point de passage de rapport · feu arriere · feu stop · 1 · _R_ | `nicht_aktiv` · `aktiv` |  |
| `DUAL_SP__RL_BL_2_L` | bi-zone · point de passage de rapport · feu arriere · feu stop · 2 · _L_ | `nicht_aktiv` · `aktiv` |  |
| `DUAL_SP__RL_BL_2_R` | bi-zone · point de passage de rapport · feu arriere · feu stop · 2 · _R_ | `nicht_aktiv` · `aktiv` |  |
| `DUAL_SP__SL_V_L` | bi-zone · point de passage de rapport · feux de position · _V_ · _L_ | `nicht_aktiv` · `aktiv` |  |
| `DUAL_SP__SL_V_R` | bi-zone · point de passage de rapport · feux de position · _V_ · _R_ | `nicht_aktiv` · `aktiv` |  |
| `DUAL_SP___BFD_L` | bi-zone · point de passage de rapport · feux stop adaptatifs (Brake Force Display) · _L_ | `nicht_aktiv` · `aktiv` |  |
| `DUAL_SP___BFD_R` | bi-zone · point de passage de rapport · feux stop adaptatifs (Brake Force Display) · _R_ | `nicht_aktiv` · `aktiv` |  |
| `DYNAM_LWR_ENABLE` | **DYNAM** · correcteur de site des phares · activation | `nicht_aktiv` · `aktiv` |  |
| `D_TAU_MITTEL_GIERR` | D · _TAU_ · moyen · **GIERR** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `D_TAU_MITTEL_LENKW` | D · _TAU_ · moyen · **LENKW** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `ENABLE_GURTBRINGER` | activation · avance-ceinture | `nicht_aktiv` · `aktiv` |  |
| `ERR_LWR_L_IGNORIEREN` | erreur · correcteur de site des phares · _L_ · **IGNORIEREN** | `nicht_aktiv` · `aktiv` |  |
| `ERR_LWR_R_IGNORIEREN` | erreur · correcteur de site des phares · _R_ · **IGNORIEREN** | `nicht_aktiv` · `aktiv` |  |
| `ERR_SCHWENK_L_IGNORIEREN` | erreur · pivotement · _L_ · **IGNORIEREN** | `nicht_aktiv` · `aktiv` |  |
| `ERR_SCHWENK_R_IGNORIEREN` | erreur · pivotement · _R_ · **IGNORIEREN** | `nicht_aktiv` · `aktiv` |  |
| `FAHRWEG_BORDSTEIN_AUTOM` | **FAHRWEG** · bordure de trottoir · automatique | `wert_01` |  |
| `FEHLERMELDUNG_CC_AL_L` | message d'erreur · Check Control · feux de croisement · _L_ | `nicht_aktiv` · `aktiv` | Message d'erreur Check Control pour cette lampe. |
| `FEHLERMELDUNG_CC_AL_R` | message d'erreur · Check Control · feux de croisement · _R_ | `nicht_aktiv` · `aktiv` | Message d'erreur Check Control pour cette lampe. |
| `FEHLERMELDUNG_CC_BFD_L` | message d'erreur · Check Control · feux stop adaptatifs (Brake Force Display) · _L_ | `aktiv` · `nicht_aktiv` | Message d'erreur Check Control pour cette lampe. |
| `FEHLERMELDUNG_CC_BFD_R` | message d'erreur · Check Control · feux stop adaptatifs (Brake Force Display) · _R_ | `aktiv` · `nicht_aktiv` | Message d'erreur Check Control pour cette lampe. |
| `FEHLERMELDUNG_CC_BLK_H_L` | message d'erreur · Check Control · clignotant · _H_ · _L_ | `nicht_aktiv` · `aktiv` | Message d'erreur Check Control pour cette lampe. |
| `FEHLERMELDUNG_CC_BLK_H_R` | message d'erreur · Check Control · clignotant · _H_ · _R_ | `nicht_aktiv` · `aktiv` | Message d'erreur Check Control pour cette lampe. |
| `FEHLERMELDUNG_CC_BLK_V_L` | message d'erreur · Check Control · clignotant · _V_ · _L_ | `nicht_aktiv` · `aktiv` | Message d'erreur Check Control pour cette lampe. |
| `FEHLERMELDUNG_CC_BLK_V_R` | message d'erreur · Check Control · clignotant · _V_ · _R_ | `nicht_aktiv` · `aktiv` | Message d'erreur Check Control pour cette lampe. |
| `FEHLERMELDUNG_CC_BL_L` | message d'erreur · Check Control · feu stop · _L_ | `nicht_aktiv` · `aktiv` | Message d'erreur Check Control pour cette lampe. |
| `FEHLERMELDUNG_CC_BL_M_1` | message d'erreur · Check Control · feu stop · M · 1 | `nicht_aktiv` · `aktiv` | Message d'erreur Check Control pour cette lampe. |
| `FEHLERMELDUNG_CC_BL_R` | message d'erreur · Check Control · feu stop · _R_ | `nicht_aktiv` · `aktiv` | Message d'erreur Check Control pour cette lampe. |
| `FEHLERMELDUNG_CC_FL_L` | message d'erreur · Check Control · feux de route · _L_ | `nicht_aktiv` · `aktiv` | Message d'erreur Check Control pour cette lampe. |
| `FEHLERMELDUNG_CC_FL_R` | message d'erreur · Check Control · feux de route · _R_ | `nicht_aktiv` · `aktiv` | Message d'erreur Check Control pour cette lampe. |
| `FEHLERMELDUNG_CC_KZL` | message d'erreur · Check Control · eclairage de plaque | `nicht_aktiv` · `aktiv` | Message d'erreur Check Control pour cette lampe. |
| `FEHLERMELDUNG_CC_NSL_L` | message d'erreur · Check Control · antibrouillard arriere · _L_ | `aktiv` · `nicht_aktiv` | Message d'erreur Check Control pour cette lampe. |
| `FEHLERMELDUNG_CC_NSL_R` | message d'erreur · Check Control · antibrouillard arriere · _R_ | `aktiv` · `nicht_aktiv` | Message d'erreur Check Control pour cette lampe. |
| `FEHLERMELDUNG_CC_NSW_L` | message d'erreur · Check Control · antibrouillard avant · _L_ | `nicht_aktiv` · `aktiv` | Message d'erreur Check Control pour cette lampe. |
| `FEHLERMELDUNG_CC_NSW_R` | message d'erreur · Check Control · antibrouillard avant · _R_ | `nicht_aktiv` · `aktiv` | Message d'erreur Check Control pour cette lampe. |
| `FEHLERMELDUNG_CC_RFS_L` | message d'erreur · Check Control · feu de recul · _L_ | `aktiv` · `nicht_aktiv` | Message d'erreur Check Control pour cette lampe. |
| `FEHLERMELDUNG_CC_RFS_R` | message d'erreur · Check Control · feu de recul · _R_ | `aktiv` · `nicht_aktiv` | Message d'erreur Check Control pour cette lampe. |
| `FEHLERMELDUNG_CC_SL_V_L` | message d'erreur · Check Control · feux de position · _V_ · _L_ | `nicht_aktiv` · `aktiv` | Message d'erreur Check Control pour cette lampe. |
| `FEHLERMELDUNG_CC_SL_V_R` | message d'erreur · Check Control · feux de position · _V_ · _R_ | `nicht_aktiv` · `aktiv` | Message d'erreur Check Control pour cette lampe. |
| `FEHLERMELD_CC_RL_BL_1_L` | **FEHLERMELD** · Check Control · feu arriere · feu stop · 1 · _L_ | `nicht_aktiv` · `aktiv` |  |
| `FEHLERMELD_CC_RL_BL_1_R` | **FEHLERMELD** · Check Control · feu arriere · feu stop · 1 · _R_ | `nicht_aktiv` · `aktiv` |  |
| `FEHLERMELD_CC_RL_BL_2_L` | **FEHLERMELD** · Check Control · feu arriere · feu stop · 2 · _L_ | `nicht_aktiv` · `aktiv` |  |
| `FEHLERMELD_CC_RL_BL_2_R` | **FEHLERMELD** · Check Control · feu arriere · feu stop · 2 · _R_ | `nicht_aktiv` · `aktiv` |  |
| `FH_CC_MELDUNGEN` | leve-vitre · Check Control · **MELDUNGEN** | `aktiv` · `nicht_aktiv` |  |
| `FH_FAHRERTUER_TIPP_AUF` | leve-vitre · **FAHRERTUER** · impulsion · ouverture | `aktiv` · `nicht_aktiv` |  |
| `FH_FAHRERTUER_TIPP_ZU` | leve-vitre · **FAHRERTUER** · impulsion · fermeture | `aktiv` · `nicht_aktiv` |  |
| `FL_ABSCHALT_OPT` | feux de route · coupure · option | `aktiv` · `nicht_aktiv` |  |
| `FL_UND_NSW_NICHT_GLEICHZ` | feux de route · **UND** · antibrouillard avant · non · **GLEICHZ** | `nicht_aktiv` · `aktiv` | Autorise les antibrouillards en meme temps que les feux de route. |
| `FSP_ISP_SPERREN` | **FSP** · **ISP** · **SPERREN** | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `FZG_RADSTAND` | vehicule · **RADSTAND** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+9)* |  |
| `GB_DELTA_POS` | **GB** · ecart · position | `wert_01` |  |
| `GB_GESCHW` | **GB** · vitesse | `wert_01` |  |
| `GB_POS_AUS` | **GB** · position · arret | `wert_01` |  |
| `GB_T_EINFAHREN` | **GB** · T · **EINFAHREN** | `wert_01` |  |
| `GB_T_NOTLAUF` | **GB** · T · mode degrade | `wert_01` · `wert_02` |  |
| `GB_USE_SBE` | **GB** · **USE** · **SBE** | `nicht_aktiv` · `aktiv` |  |
| `GESCHW_ASP_AUTO_AUSKLAPP` | vitesse · retroviseur exterieur · automatique · **AUSKLAPP** | `wert_01` |  |
| `GESCHW_DOWN` | vitesse · bas | `wert_01` |  |
| `GESCHW_UP` | vitesse · haut | `wert_01` |  |
| `GETRIEBE_HANDSCHALTER` | boite de vitesses · **HANDSCHALTER** | `aktiv` · `nicht_aktiv` |  |
| `HECKDECK_REVERS_BLK` | **HECKDECK** · **REVERS** · clignotant | `wert_01` · `wert_02` |  |
| `HECKD_REV_BLK_DOPPELB` | **HECKD** · revision · clignotant · **DOPPELB** | `nicht_aktiv` · `aktiv` |  |
| `HEIMLEUCHTEN` | eclairage d'accompagnement | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `HEIMLEUCHTEN_ABBRUCH` | eclairage d'accompagnement · **ABBRUCH** | `aktiv` · `nicht_aktiv` |  |
| `HEIMLEUCHTEN_ENABLE` | eclairage d'accompagnement · activation | `nicht_aktiv` · `aktiv` |  |
| `HEIMLEUCHTEN_VERLAENG` | eclairage d'accompagnement · **VERLAENG** | `aktiv` · `nicht_aktiv` |  |
| `IBUS_R_GANG` | **IBUS** · _R_ · **GANG** | `nicht_aktiv` · `aktiv` |  |
| `IB_2MIN_OPTION` | **IB** · **2MIN** · option | `aktiv` · `nicht_aktiv` |  |
| `IB_SOFTON_OPTION` | **IB** · **SOFTON** · option | `aktiv` · `nicht_aktiv` |  |
| `IK_BUS_TEILNEMER` | **IK** · bus · **TEILNEMER** | `nicht_aktiv` · `aktiv` |  |
| `KALTUEBERWACHUNG_AL_L` | surveillance a froid · feux de croisement · _L_ | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a froid. Desactiver supprime le message de defaut pour cette lampe. |
| `KALTUEBERWACHUNG_AL_R` | surveillance a froid · feux de croisement · _R_ | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a froid. Desactiver supprime le message de defaut pour cette lampe. |
| `KALTUEBERWACHUNG_BFD_L` | surveillance a froid · feux stop adaptatifs (Brake Force Display) · _L_ | `aktiv` · `nicht_aktiv` | Surveillance de l'ampoule a froid. Desactiver supprime le message de defaut pour cette lampe. |
| `KALTUEBERWACHUNG_BFD_R` | surveillance a froid · feux stop adaptatifs (Brake Force Display) · _R_ | `aktiv` · `nicht_aktiv` | Surveillance de l'ampoule a froid. Desactiver supprime le message de defaut pour cette lampe. |
| `KALTUEBERWACHUNG_BLK_H_L` | surveillance a froid · clignotant · _H_ · _L_ | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a froid. Desactiver supprime le message de defaut pour cette lampe. |
| `KALTUEBERWACHUNG_BLK_H_R` | surveillance a froid · clignotant · _H_ · _R_ | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a froid. Desactiver supprime le message de defaut pour cette lampe. |
| `KALTUEBERWACHUNG_BLK_V_L` | surveillance a froid · clignotant · _V_ · _L_ | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a froid. Desactiver supprime le message de defaut pour cette lampe. |
| `KALTUEBERWACHUNG_BLK_V_R` | surveillance a froid · clignotant · _V_ · _R_ | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a froid. Desactiver supprime le message de defaut pour cette lampe. |
| `KALTUEBERWACHUNG_BL_L` | surveillance a froid · feu stop · _L_ | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a froid. Desactiver supprime le message de defaut pour cette lampe. |
| `KALTUEBERWACHUNG_BL_M_1` | surveillance a froid · feu stop · M · 1 | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a froid. Desactiver supprime le message de defaut pour cette lampe. |
| `KALTUEBERWACHUNG_BL_R` | surveillance a froid · feu stop · _R_ | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a froid. Desactiver supprime le message de defaut pour cette lampe. |
| `KALTUEBERWACHUNG_FL_L` | surveillance a froid · feux de route · _L_ | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a froid. Desactiver supprime le message de defaut pour cette lampe. |
| `KALTUEBERWACHUNG_FL_R` | surveillance a froid · feux de route · _R_ | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a froid. Desactiver supprime le message de defaut pour cette lampe. |
| `KALTUEBERWACHUNG_KZL` | surveillance a froid · eclairage de plaque | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a froid. Desactiver supprime le message de defaut pour cette lampe. |
| `KALTUEBERWACHUNG_NSL_L` | surveillance a froid · antibrouillard arriere · _L_ | `aktiv` · `nicht_aktiv` | Surveillance de l'ampoule a froid. Desactiver supprime le message de defaut pour cette lampe. |
| `KALTUEBERWACHUNG_NSL_R` | surveillance a froid · antibrouillard arriere · _R_ | `aktiv` · `nicht_aktiv` | Surveillance de l'ampoule a froid. Desactiver supprime le message de defaut pour cette lampe. |
| `KALTUEBERWACHUNG_NSW_L` | surveillance a froid · antibrouillard avant · _L_ | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a froid. Desactiver supprime le message de defaut pour cette lampe. |
| `KALTUEBERWACHUNG_NSW_R` | surveillance a froid · antibrouillard avant · _R_ | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a froid. Desactiver supprime le message de defaut pour cette lampe. |
| `KALTUEBERWACHUNG_RFS_L` | surveillance a froid · feu de recul · _L_ | `aktiv` · `nicht_aktiv` | Surveillance de l'ampoule a froid. Desactiver supprime le message de defaut pour cette lampe. |
| `KALTUEBERWACHUNG_RFS_R` | surveillance a froid · feu de recul · _R_ | `aktiv` · `nicht_aktiv` | Surveillance de l'ampoule a froid. Desactiver supprime le message de defaut pour cette lampe. |
| `KALTUEBERWACHUNG_SL_V_L` | surveillance a froid · feux de position · _V_ · _L_ | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a froid. Desactiver supprime le message de defaut pour cette lampe. |
| `KALTUEBERWACHUNG_SL_V_R` | surveillance a froid · feux de position · _V_ · _R_ | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a froid. Desactiver supprime le message de defaut pour cette lampe. |
| `KALTUEBERW_RL_BL_1_L` | **KALTUEBERW** · feu arriere · feu stop · 1 · _L_ | `nicht_aktiv` · `aktiv` |  |
| `KALTUEBERW_RL_BL_1_R` | **KALTUEBERW** · feu arriere · feu stop · 1 · _R_ | `nicht_aktiv` · `aktiv` |  |
| `KALTUEBERW_RL_BL_2_L` | **KALTUEBERW** · feu arriere · feu stop · 2 · _L_ | `nicht_aktiv` · `aktiv` |  |
| `KALTUEBERW_RL_BL_2_R` | **KALTUEBERW** · feu arriere · feu stop · 2 · _R_ | `nicht_aktiv` · `aktiv` |  |
| `KEINE_ENTRFLANKE_4_IB` | aucun · **ENTRFLANKE** · 4 · **IB** | `aktiv` · `nicht_aktiv` |  |
| `KEINE_L_R_SYNCHRONISATION` | aucun · _L_ · _R_ · **SYNCHRONISATION** | `nicht_aktiv` · `aktiv` |  |
| `KEINE_PARKLICHT_FUNKT` | aucun · feu de stationnement · **FUNKT** | `nicht_aktiv` · `aktiv` |  |
| `KENNL_MAX_UEBERST` | courbe caracteristique · maximum · survirage | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `KENNL_REDUZ_MAX_SCHWENK` | courbe caracteristique · **REDUZ** · maximum · pivotement | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `KLASSIF_DRIFT_ENA` | **KLASSIF** · derive · activation | `nicht_aktiv` · `aktiv` |  |
| `KLASSIF_LENKM_ENA` | **KLASSIF** · **LENKM** · activation | `nicht_aktiv` · `aktiv` |  |
| `KLASSIF_UEBERST_ENA` | **KLASSIF** · survirage · activation | `nicht_aktiv` · `aktiv` |  |
| `KLS_BEI_AL_AKTIV` | **KLS** · lors de · feux de croisement · actif | `nicht_aktiv` · `aktiv` |  |
| `KL_VA_EIN_BEI_ENTRIEGELN` | borne · essieu avant · marche · lors de · **ENTRIEGELN** | `aktiv` · `nicht_aktiv` |  |
| `KURZSCHLUSS_CHECK_TIME` | court-circuit · controle · duree | `wert_01` |  |
| `KURZSCHLUSS_DIAGNOSE_OPT` | court-circuit · **DIAGNOSE** · option | `nicht_aktiv` · `aktiv` |  |
| `KURZSCHLUSS_ENTPRELL` | court-circuit · anti-rebond | `wert_01` |  |
| `KURZSCHL_WIERHOL_ZAEHLER` | **KURZSCHL** · **WIERHOL** · compteur | `wert_01` · `wert_02` |  |
| `LENKSTOCK_UMSCHALT_VERZ` | **LENKSTOCK** · **UMSCHALT** · temporisation | `wert_01` |  |
| `LENKWINKEL_BEI_KLR` | angle de braquage · lors de · regulation de climatisation | `aktiv` · `nicht_aktiv` |  |
| `LENKWINKEL_UEBERSCHREITEN` | angle de braquage · **UEBERSCHREITEN** | `wert_01` |  |
| `LENKWINKEL_UNTERSCHREITEN` | angle de braquage · **UNTERSCHREITEN** | `wert_01` |  |
| `LENK_UEBERSETZUNG` | direction · **UEBERSETZUNG** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+15)* |  |
| `LENK_UEBERSETZUNG_LO` | direction · **UEBERSETZUNG** · bas | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+12)* |  |
| `LEUCHTDICHTE_EXPO` | **LEUCHTDICHTE** · **EXPO** | `nicht_aktiv` · `aktiv` |  |
| `LICHTZ_R_L_FAKTOR` | **LICHTZ** · _R_ · _L_ · facteur | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `LICHT_AN_LED` | lumiere · **AN** · LED | `nicht_aktiv` · `aktiv` |  |
| `LICHT_AN_WARNUNG` | lumiere · **AN** · alerte | `nicht_aktiv` · `aktiv` |  |
| `LICHT_AN_WARN_KL_R` | lumiere · **AN** · alerte · borne · _R_ | `nicht_aktiv` · `aktiv` |  |
| `LINKSVERKEHR` | **LINKSVERKEHR** | `nicht_aktiv` · `aktiv` |  |
| `LOAD_DUMP_DIMM_WERL_BLK` | charge · extraction memoire · gradation · **WERL** · clignotant | `wert_01` |  |
| `LOAD_DUMP_MAX_TIME` | charge · extraction memoire · maximum · duree | `wert_01` · `wert_02` |  |
| `LOAD_DUMP_PRELL` | charge · extraction memoire · **PRELL** | `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `LOAD_DUMP_SCHUTZ_AKTIV` | charge · extraction memoire · protection · actif | `aktiv` · `nicht_aktiv` |  |
| `LOAD_DUMP_SCHUTZ_BLK_Z` | charge · extraction memoire · protection · clignotant · Z | `aktiv` · `nicht_aktiv` |  |
| `LOAD_DUMP_SCHUTZ_FL` | charge · extraction memoire · protection · feux de route | `aktiv` · `nicht_aktiv` |  |
| `LOAD_DUMP_SCHUTZ_KL58G` | charge · extraction memoire · protection · **KL58G** | `aktiv` · `nicht_aktiv` |  |
| `LOAD_DUMP_SCHUTZ_KZL` | charge · extraction memoire · protection · eclairage de plaque | `aktiv` · `nicht_aktiv` |  |
| `LOAD_DUMP_SCHUTZ_NSL` | charge · extraction memoire · protection · antibrouillard arriere | `aktiv` · `nicht_aktiv` |  |
| `LOAD_DUMP_SCHUTZ_NSW` | charge · extraction memoire · protection · antibrouillard avant | `aktiv` · `nicht_aktiv` |  |
| `LOAD_DUMP_SCHUTZ_RFS` | charge · extraction memoire · protection · feu de recul | `aktiv` · `nicht_aktiv` |  |
| `LOAD_DUMP_SCHUTZ_RL_BL_2` | charge · extraction memoire · protection · feu arriere · feu stop · 2 | `aktiv` · `nicht_aktiv` |  |
| `LOAD_DUMP_SCHUTZ_SL_V` | charge · extraction memoire · protection · feux de position · _V_ | `aktiv` · `nicht_aktiv` |  |
| `LWR_ABSTELLPOS` | correcteur de site des phares · **ABSTELLPOS** | `wert_01` · `wert_12` · `wert_02` · `wert_11` · `wert_0` · `wert_1` · `wert_2` · `wert_3` … *(+11)* |  |
| `LWR_ABS_MIN_STELLUNG_2` | correcteur de site des phares · ABS · minimum · position · 2 | `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `LWR_ANSTEUERN` | correcteur de site des phares · **ANSTEUERN** | `aktiv` · `nicht_aktiv` |  |
| `LWR_DIAG_PARAM` | correcteur de site des phares · **DIAG** · **PARAM** | `wert_01` · `wert_02` |  |
| `LWR_FEHLER_BEWERTUNG` | correcteur de site des phares · erreur · **BEWERTUNG** | `wert_01` · `wert_02` |  |
| `LWR_HEBEL` | correcteur de site des phares · **HEBEL** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `LWR_HYSTERESE_START` | correcteur de site des phares · **HYSTERESE** · demarrage | `wert_01` · `wert_02` |  |
| `LWR_SCHRITTABSTAND_GRENZE` | correcteur de site des phares · **SCHRITTABSTAND** · limite | `wert_01` |  |
| `LWR_SCHRITTE_STELLUNG_0` | correcteur de site des phares · **SCHRITTE** · position · 0 | `wert_01` · `wert_02` |  |
| `LWR_SCHRITTE_STELLUNG_1` | correcteur de site des phares · **SCHRITTE** · position · 1 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_06` |  |
| `LWR_SCHRITTE_STELLUNG_2` | correcteur de site des phares · **SCHRITTE** · position · 2 | `wert_01` · `wert_02` · `wert_03` · `wert_06` |  |
| `LWR_TEMPERATUR_GRENZE` | correcteur de site des phares · temperature · limite | — |  |
| `LWR_TERMO_MODEL_EIN` | correcteur de site des phares · **TERMO** · **MODEL** · marche | `aktiv` |  |
| `LWR_TESTBETRIEB` | correcteur de site des phares · **TESTBETRIEB** | `aktiv` · `nicht_aktiv` |  |
| `LWR_VORBESTROMUNG` | correcteur de site des phares · **VORBESTROMUNG** | `wert_01` · `wert_02` |  |
| `MAX_DELTA_KEINE_VERR` | maximum · ecart · aucun · verrouillage | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `MAX_GESCHW_EINKLAPPEN_2` | maximum · vitesse · rabattement · 2 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `polnisch` · `dynamik_2` · `zieltermin` |  |
| `MAX_SW_SCHWENK_V_HORIZ` | maximum · logiciel · pivotement · _V_ · **HORIZ** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+10)* |  |
| `MAX_SW_SCHWENK_V_VERTI` | maximum · logiciel · pivotement · _V_ · **VERTI** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+11)* |  |
| `MAX_SW_SCHWENK_W_AUSSEN` | maximum · logiciel · pivotement · W · exterieur | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+12)* |  |
| `MAX_SW_SCHWENK_W_INNEN` | maximum · logiciel · pivotement · W · interieur | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+10)* |  |
| `MAX_VERRINGERUNG_V_SCHW` | maximum · **VERRINGERUNG** · _V_ · seuil | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+10)* |  |
| `MELDUNG_AENDERUNG_DIMMER` | message · **AENDERUNG** · **DIMMER** | `wert_01` |  |
| `MEM_POS_ANFAHREN` | **MEM** · position · **ANFAHREN** | `wert_01` |  |
| `MERKM_1_BEGR` | caracteristique · 1 · **BEGR** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `MERKM_1_DEKR` | caracteristique · 1 · **DEKR** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `MERKM_1_INKR` | caracteristique · 1 · **INKR** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `MERKM_1_MAX` | caracteristique · 1 · maximum | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `MIND_ANZ_ZYKL_TIPP_BLK` | minimum · nombre · cycle · impulsion · clignotant | `wert_01` · `wert_02` | Nombre de clignotements sur simple impulsion du comodo. |
| `MIN_DELTA_MAX_VERR` | minimum · ecart · maximum · verrouillage | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `MIN_SW_SCHWENK_V_HORIZ` | minimum · logiciel · pivotement · _V_ · **HORIZ** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `MIN_SW_SCHWENK_V_VERTI` | minimum · logiciel · pivotement · _V_ · **VERTI** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+8)* |  |
| `NACHLAUF_KLEMME_VA` | marche a suivre / temporisation · **KLEMME** · essieu avant | `wert_01` · `wert_02` |  |
| `NORM_LEUCHTWEITE` | norme · **LEUCHTWEITE** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+9)* |  |
| `NOTLAUF_FKT_GESCHW` | mode degrade · fonction · vitesse | `wert_01` · `wert_02` |  |
| `NOTLAUF_FKT_SCHALTERSTELL` | mode degrade · fonction · **SCHALTERSTELL** | `aktiv` · `nicht_aktiv` |  |
| `NOTLAUF_FKT_TIMER` | mode degrade · fonction · minuterie | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `NSL_ERS_DEF_RL_BL_1` | antibrouillard arriere · **ERS** · defaut · feu arriere · feu stop · 1 | `aktiv` · `nicht_aktiv` |  |
| `NSL_VERBAU_OTP` | antibrouillard arriere · **VERBAU** · **OTP** | `wert_01` · `wert_02` · `wert_03` |  |
| `OHNE_GIERRATE` | sans · vitesse de lacet | `nicht_aktiv` · `aktiv` |  |
| `OPTIONEN_SL_ABSCH` | **OPTIONEN** · feux de position · coupure | `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `PANIKMODUS_EKS` | mode panique · systeme electronique de cle | `aktiv` · `nicht_aktiv` |  |
| `POTI_DIMM_INVERS` | potentiometre · gradation · **INVERS** | `nicht_aktiv` · `aktiv` |  |
| `POTI_LWR_INVERS` | potentiometre · correcteur de site des phares · **INVERS** | `nicht_aktiv` · `aktiv` |  |
| `PRE_SLEEP_ZEIT` | pre- · **SLEEP** · duree | `wert_01` · `wert_02` · `wert_03` |  |
| `PROZENT_EE` | pourcentage · **EE** | `wert_01` · `wert_02` |  |
| `QUITT_BLK_ENTSICHERN` | acquittement sonore · clignotant · deverrouillage | `nicht_aktiv` · `aktiv` |  |
| `QUITT_BLK_IGNORE_MOTORH` | acquittement sonore · clignotant · **IGNORE** · **MOTORH** | `nicht_aktiv` · `aktiv` |  |
| `QUITT_BLK_SICHERN` | acquittement sonore · clignotant · verrouillage | `nicht_aktiv` · `aktiv` |  |
| `QUITT_BLK_ZU_CHECK` | acquittement sonore · clignotant · fermeture · controle | `nicht_aktiv` · `aktiv` |  |
| `RAUSCH` | **RAUSCH** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `RECHTSVERKEHR` | **RECHTSVERKEHR** | `nicht_aktiv` · `aktiv` |  |
| `RECHTS_LENKER_ASP` | droite · volant · retroviseur exterieur | `aktiv` · `nicht_aktiv` |  |
| `RECHTS_LENKER_FH_H` | droite · volant · leve-vitre · _H_ | `aktiv` · `nicht_aktiv` |  |
| `RECHTS_LENKER_FH_V` | droite · volant · leve-vitre · _V_ | `aktiv` · `nicht_aktiv` |  |
| `REDUZIERUNG_V_SCHWENK_HI` | reduction · _V_ · pivotement · haut | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `REDUZIERUNG_V_SCHWENK_LO` | reduction · _V_ · pivotement · bas | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `RL_BL_1_BREMSL_FKT` | feu arriere · feu stop · 1 · feu stop · fonction | `aktiv` · `nicht_aktiv` |  |
| `RL_BL_1_PARKL_FKT` | feu arriere · feu stop · 1 · feu de stationnement · fonction | `aktiv` · `nicht_aktiv` |  |
| `RL_BL_1_PO_RL_FKT` | feu arriere · feu stop · 1 · **PO** · feu arriere · fonction | `aktiv` · `nicht_aktiv` |  |
| `RL_BL_2_BREMSL_FKT` | feu arriere · feu stop · 2 · feu stop · fonction | `aktiv` · `nicht_aktiv` |  |
| `RL_BL_2_PARKL_FKT` | feu arriere · feu stop · 2 · feu de stationnement · fonction | `aktiv` · `nicht_aktiv` |  |
| `RL_BL_2_PO_RL_FKT` | feu arriere · feu stop · 2 · **PO** · feu arriere · fonction | `aktiv` · `nicht_aktiv` |  |
| `RL_BL_BREMSL_FKT` | feu arriere · feu stop · feu stop · fonction | `aktiv` · `nicht_aktiv` |  |
| `RL_BL_PARKL_FKT` | feu arriere · feu stop · feu de stationnement · fonction | `aktiv` · `nicht_aktiv` |  |
| `RL_BL_PO_RL_FKT` | feu arriere · feu stop · **PO** · feu arriere · fonction | `aktiv` · `nicht_aktiv` |  |
| `SCHALTER_R_GANG` | **SCHALTER** · _R_ · **GANG** | `nicht_aktiv` · `aktiv` |  |
| `SCHW_TAU_LW_GR` | seuil · _TAU_ · **LW** · **GR** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `SICHERHEITS_FZG_LM` | **SICHERHEITS** · vehicule · **LM** | `aktiv` · `nicht_aktiv` |  |
| `SIDEMARKER_US` | feu lateral (US) · marche americain | `nicht_aktiv` · `aktiv` | Feux lateraux de gabarit, obligatoires sur le marche americain. |
| `SL_BL_CC_MELD_E8X` | feux de position · feu stop · Check Control · message · **E8X** | `aktiv` · `nicht_aktiv` |  |
| `SL_BL_ERS_DEF_RL_BL_1` | feux de position · feu stop · **ERS** · defaut · feu arriere · feu stop · 1 | `aktiv` · `nicht_aktiv` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `SL_PARKL_UNTERSP_ABSCHALT` | feux de position · feu de stationnement · **UNTERSP** · coupure | `nicht_aktiv` · `aktiv` |  |
| `SL_V_PARKL_FKT` | feux de position · _V_ · feu de stationnement · fonction | `aktiv` · `nicht_aktiv` |  |
| `SL_V_PO_RL_FKT` | feux de position · _V_ · **PO** · feu arriere · fonction | `aktiv` · `nicht_aktiv` |  |
| `SPIEGEL_BORDSTEIN_AUTOM` | retroviseur · bordure de trottoir · automatique | `aktiv` · `nicht_aktiv` | Le retroviseur droit s'abaisse en marche arriere pour voir le trottoir. |
| `SPIEGEL_EINKLAPPEN` | retroviseur · rabattement | `aktiv` · `nicht_aktiv` | Autorise le rabattement electrique des retroviseurs. |
| `SPIEGEL_ELEKTROCHROM` | retroviseur · **ELEKTROCHROM** | `aktiv` · `nicht_aktiv` | Retroviseurs photosensibles (electrochromes). |
| `SPIEGEL_HEIZUNG` | retroviseur · **HEIZUNG** | `aktiv` · `nicht_aktiv` |  |
| `SPIEGEL_KOMFORT_EINKLAPP` | retroviseur · confort · rabattement | `aktiv` · `nicht_aktiv` | Rabattement des retroviseurs par la telecommande au verrouillage. |
| `SPIEGEL_MEMORY` | retroviseur · memoire | `aktiv` · `nicht_aktiv` |  |
| `SPIEGEL_STAND_HEIZUNG` | retroviseur · etat · **HEIZUNG** | `aktiv` · `nicht_aktiv` |  |
| `SP_GRENZE` | point de passage de rapport · limite | `wert_01` · `wert_02` · `wert_03` |  |
| `SP_NACHREG_LANGSAM` | point de passage de rapport · **NACHREG** · **LANGSAM** | `nicht_aktiv` · `aktiv` |  |
| `STANDVERBRAUCHER_ANMELDEN` | **STANDVERBRAUCHER** · **ANMELDEN** | `aktiv` · `nicht_aktiv` |  |
| `SW_NULL_BEREICH` | logiciel · **NULL** · plage | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `TAGFAHRLICHT_ECE` | feux de jour · norme europeenne | `nicht_aktiv` · `aktiv` |  |
| `TASTVERH_BEL_FLC_LED` | rapport cyclique · **BEL** · _FLC_ · LED | `wert_01` · `wert_02` |  |
| `TASTVERH_BEL_WBL_MIN` | rapport cyclique · **BEL** · **WBL** · minimum | `wert_01` · `wert_02` · `wert_03` |  |
| `TASTVERH_KL_58G_MIN` | rapport cyclique · borne · **58G** · minimum | `wert_01` |  |
| `TAU_ABSCHWAECHUNG` | _TAU_ · **ABSCHWAECHUNG** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+8)* |  |
| `TIPP_BLK_ABBRUCH` | impulsion · clignotant · **ABBRUCH** | `nicht_aktiv` · `aktiv` |  |
| `TIPP_BLK_LOESCHT_IMMER` | impulsion · clignotant · **LOESCHT** · toujours | `nicht_aktiv` · `aktiv` |  |
| `TUERAUF_STOP_MAUT` | porte ouverte · arret · peage | `aktiv` · `nicht_aktiv` | Interrompt la remontee automatique de vitre (fonction peage) des qu'une portiere est ouverte. |
| `TYP_FH_BEDIENBLOCK` | type · leve-vitre · **BEDIENBLOCK** | `nicht_aktiv` · `aktiv` |  |
| `T_FILTER_DYN_LZ` | T · filtre · dynamique · **LZ** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `T_FILTER_GIERRATE` | T · filtre · vitesse de lacet | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `T_FILTER_LENKWINKEL` | T · filtre · angle de braquage | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `T_FILTER_TAU_LW` | T · filtre · _TAU_ · **LW** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `T_FILTER_UEBERST` | T · filtre · survirage | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `T_FILTER_V_VEH` | T · filtre · _V_ · **VEH** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `T_F_GIERRATE_DEF` | T · F · vitesse de lacet · defaut | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `UMSCHALTOPT_LICHTSCHALTER` | **UMSCHALTOPT** · **LICHTSCHALTER** | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `UNP_HEIMLEUCHTEN_CKM` | _UNP_ · eclairage d'accompagnement · memoire par cle (Car Key Memory) | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `UNP_QUITT_BLK_ENTSI_CKM` | _UNP_ · acquittement sonore · clignotant · **ENTSI** · memoire par cle (Car Key Memory) | `nicht_aktiv` · `aktiv` |  |
| `UNP_QUITT_BLK_SICH_CKM` | _UNP_ · acquittement sonore · clignotant · **SICH** · memoire par cle (Car Key Memory) | `nicht_aktiv` · `aktiv` |  |
| `UNP_TAGFAHRLICHT_ECE_CKM` | _UNP_ · feux de jour · norme europeenne · memoire par cle (Car Key Memory) | `nicht_aktiv` · `aktiv` |  |
| `UNP_ZYKL_TIPP_BLK_CKM` | _UNP_ · cycle · impulsion · clignotant · memoire par cle (Car Key Memory) | `wert_01` · `wert_02` |  |
| `USA_REVERSE` | marche americain · **REVERSE** | `aktiv` · `nicht_aktiv` |  |
| `U_LWR_POTI_STELLUNG_0` | U · correcteur de site des phares · potentiometre · position · 0 | `wert_01` |  |
| `U_LWR_POTI_STELLUNG_1` | U · correcteur de site des phares · potentiometre · position · 1 | `wert_01` |  |
| `U_LWR_POTI_STELLUNG_2` | U · correcteur de site des phares · potentiometre · position · 2 | `wert_01` |  |
| `U_POTI_DIMM_DELTA` | U · potentiometre · gradation · ecart | `wert_01` |  |
| `U_POTI_DIMM_MIN` | U · potentiometre · gradation · minimum | `wert_01` |  |
| `VIER_TUERER` | **VIER** · **TUERER** | `nicht_aktiv` · `aktiv` |  |
| `VORFELD_BEL_GEDIMMT` | **VORFELD** · **BEL** · **GEDIMMT** | `aktiv` · `nicht_aktiv` |  |
| `V_HI_ABSCHW_D_TAU` | _V_ · haut · attenuation · D · _TAU_ | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `V_HI_AENDER_LENK_UEBERS` | _V_ · haut · **AENDER** · direction · **UEBERS** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `V_HI_REDUZIER_V_SCHWENK` | _V_ · haut · reduction · _V_ · pivotement | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `V_HI_RED_SCHWENK_W` | _V_ · haut · reduction · pivotement · W | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `V_HI_SW_MITLENKEN` | _V_ · haut · logiciel · suivi de direction | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `V_HI_SW_NACH_INNEN` | _V_ · haut · logiciel · apres · interieur | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `V_HI_TAU_GIERRATE` | _V_ · haut · _TAU_ · vitesse de lacet | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `V_LO_ABSCHW_D_TAU` | _V_ · bas · attenuation · D · _TAU_ | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `V_LO_AENDER_LENK_UEBERS` | _V_ · bas · **AENDER** · direction · **UEBERS** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `V_LO_REDUZIER_V_SCHWENK` | _V_ · bas · reduction · _V_ · pivotement | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `V_LO_RED_SCHWENK_W` | _V_ · bas · reduction · pivotement · W | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `V_LO_SW_MITLENKEN` | _V_ · bas · logiciel · suivi de direction | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `V_LO_SW_NACH_INNEN` | _V_ · bas · logiciel · apres · interieur | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `V_LO_TAU_GIERRATE` | _V_ · bas · _TAU_ · vitesse de lacet | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `V_M_HI_REDUZIER_V_SCHWENK` | _V_ · M · haut · reduction · _V_ · pivotement | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `V_M_LO_REDUZIER_V_SCHWENK` | _V_ · M · bas · reduction · _V_ · pivotement | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `V_REFERENZLAUF_EN` | _V_ · **REFERENZLAUF** · **EN** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `WARMUEBERWACHUNG_AL_L` | surveillance a chaud · feux de croisement · _L_ | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a chaud. Desactiver supprime le message de defaut pour cette lampe. |
| `WARMUEBERWACHUNG_AL_R` | surveillance a chaud · feux de croisement · _R_ | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a chaud. Desactiver supprime le message de defaut pour cette lampe. |
| `WARMUEBERWACHUNG_BFD_L` | surveillance a chaud · feux stop adaptatifs (Brake Force Display) · _L_ | `aktiv` · `nicht_aktiv` | Surveillance de l'ampoule a chaud. Desactiver supprime le message de defaut pour cette lampe. |
| `WARMUEBERWACHUNG_BFD_R` | surveillance a chaud · feux stop adaptatifs (Brake Force Display) · _R_ | `aktiv` · `nicht_aktiv` · `wert_01` | Surveillance de l'ampoule a chaud. Desactiver supprime le message de defaut pour cette lampe. |
| `WARMUEBERWACHUNG_BLK_H_L` | surveillance a chaud · clignotant · _H_ · _L_ | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a chaud. Desactiver supprime le message de defaut pour cette lampe. |
| `WARMUEBERWACHUNG_BLK_H_R` | surveillance a chaud · clignotant · _H_ · _R_ | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a chaud. Desactiver supprime le message de defaut pour cette lampe. |
| `WARMUEBERWACHUNG_BLK_V_L` | surveillance a chaud · clignotant · _V_ · _L_ | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a chaud. Desactiver supprime le message de defaut pour cette lampe. |
| `WARMUEBERWACHUNG_BLK_V_R` | surveillance a chaud · clignotant · _V_ · _R_ | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a chaud. Desactiver supprime le message de defaut pour cette lampe. |
| `WARMUEBERWACHUNG_BL_L` | surveillance a chaud · feu stop · _L_ | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a chaud. Desactiver supprime le message de defaut pour cette lampe. |
| `WARMUEBERWACHUNG_BL_M_1` | surveillance a chaud · feu stop · M · 1 | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a chaud. Desactiver supprime le message de defaut pour cette lampe. |
| `WARMUEBERWACHUNG_BL_R` | surveillance a chaud · feu stop · _R_ | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a chaud. Desactiver supprime le message de defaut pour cette lampe. |
| `WARMUEBERWACHUNG_FL_L` | surveillance a chaud · feux de route · _L_ | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a chaud. Desactiver supprime le message de defaut pour cette lampe. |
| `WARMUEBERWACHUNG_FL_R` | surveillance a chaud · feux de route · _R_ | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a chaud. Desactiver supprime le message de defaut pour cette lampe. |
| `WARMUEBERWACHUNG_KZL` | surveillance a chaud · eclairage de plaque | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a chaud. Desactiver supprime le message de defaut pour cette lampe. |
| `WARMUEBERWACHUNG_NSL_L` | surveillance a chaud · antibrouillard arriere · _L_ | `aktiv` · `nicht_aktiv` | Surveillance de l'ampoule a chaud. Desactiver supprime le message de defaut pour cette lampe. |
| `WARMUEBERWACHUNG_NSL_R` | surveillance a chaud · antibrouillard arriere · _R_ | `aktiv` · `nicht_aktiv` | Surveillance de l'ampoule a chaud. Desactiver supprime le message de defaut pour cette lampe. |
| `WARMUEBERWACHUNG_NSW_L` | surveillance a chaud · antibrouillard avant · _L_ | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a chaud. Desactiver supprime le message de defaut pour cette lampe. |
| `WARMUEBERWACHUNG_NSW_R` | surveillance a chaud · antibrouillard avant · _R_ | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a chaud. Desactiver supprime le message de defaut pour cette lampe. |
| `WARMUEBERWACHUNG_RFS_L` | surveillance a chaud · feu de recul · _L_ | `aktiv` · `nicht_aktiv` | Surveillance de l'ampoule a chaud. Desactiver supprime le message de defaut pour cette lampe. |
| `WARMUEBERWACHUNG_RFS_R` | surveillance a chaud · feu de recul · _R_ | `aktiv` · `nicht_aktiv` | Surveillance de l'ampoule a chaud. Desactiver supprime le message de defaut pour cette lampe. |
| `WARMUEBERWACHUNG_SL_V_L` | surveillance a chaud · feux de position · _V_ · _L_ | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a chaud. Desactiver supprime le message de defaut pour cette lampe. |
| `WARMUEBERWACHUNG_SL_V_R` | surveillance a chaud · feux de position · _V_ · _R_ | `nicht_aktiv` · `aktiv` | Surveillance de l'ampoule a chaud. Desactiver supprime le message de defaut pour cette lampe. |
| `WARMUEBERW_RL_BL_1_L` | **WARMUEBERW** · feu arriere · feu stop · 1 · _L_ | `nicht_aktiv` · `aktiv` |  |
| `WARMUEBERW_RL_BL_1_R` | **WARMUEBERW** · feu arriere · feu stop · 1 · _R_ | `nicht_aktiv` · `aktiv` |  |
| `WARMUEBERW_RL_BL_2_L` | **WARMUEBERW** · feu arriere · feu stop · 2 · _L_ | `nicht_aktiv` · `aktiv` |  |
| `WARMUEBERW_RL_BL_2_R` | **WARMUEBERW** · feu arriere · feu stop · 2 · _R_ | `nicht_aktiv` · `aktiv` |  |
| `WARNBLK_ENERGY_SAVE_MODE` | **WARNBLK** · **ENERGY** · **SAVE** · mode | `nicht_aktiv` · `aktiv` |  |
| `WERT_CRASHVERHALTEN` | valeur · **CRASHVERHALTEN** | `wert_01` |  |
| `XENON_AL_VERBAUT` | xenon · feux de croisement · installe | `nicht_aktiv` · `aktiv` |  |
| `XENON_LI` | xenon · **LI** | `wert_01` · `wert_02` |  |
| `XENON_WIEDERHOL_Z_OPT` | xenon · **WIEDERHOL** · Z · option | `nicht_aktiv` · `aktiv` |  |
| `XENON_WIEDER_ZUEND` | xenon · **WIEDER** · **ZUEND** | `nicht_aktiv` · `aktiv` |  |
| `XENON_WIEDER_Z_AUS_PHASE` | xenon · **WIEDER** · Z · arret · **PHASE** | `wert_01` |  |
| `ZEITK_TRIGGER1` | **ZEITK** · **TRIGGER1** | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* |  |
| `ZUWACHS_TASTVERH_FLC_LED` | **ZUWACHS** · rapport cyclique · _FLC_ · LED | `wert_01` · `wert_02` |  |
| `ZUWACHS_TASTVERH_KL_58G` | **ZUWACHS** · rapport cyclique · borne · **58G** | `wert_01` |  |
| `ZUWACHS_TASTVERH_WBL_BEL` | **ZUWACHS** · rapport cyclique · **WBL** · **BEL** | `wert_01` · `wert_02` |  |

---

**Traduction** : décomposition du nom allemand — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments en **gras** n'ont pas de traduction établie, ceux entre `_underscores_` sont ambigus.

**Valeurs possibles** : extraites des DATEN. `aktiv`/`nicht_aktiv` = interrupteur,
`wert_xx` = valeur numérique, autres = choix nommés.

**Explication** : rédigée uniquement pour les fonctions identifiées avec certitude.
Une case vide ne veut pas dire que la fonction est sans intérêt — seulement que son effet
n'a pas été établi de source sûre.
