# FRM — Module de plancher

🟠 Module `FRM` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Identification

| | |
|---|---|
| Variante | **`FRMFA.C12`** |
| Identifiée par | **lecture NCS Expert sur le véhicule** |
| Fonctions retenues | **449** |
| Vérification | 100 % des fonctions dans les blocs du `.TRC` |
| Avec valeurs | 448 |
| Avec explication rédigée | 390 |
| Blocs d'adresses du TRC | 46 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 449 fonctions codables

| FSW | Décomposition | Valeurs | Signification |
|---|---|---|---|
| `ABSCHALTUNG_RFS` | coupure · feu de recul | `aktiv` · `nicht_aktiv` | *Switch-off reversing lights when trailer attached* |
| `ABSCHWAECH_D_TAU` | attenuation · D · _TAU_ | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Percentage of issued long-term filter time constants [pourcentage = valeur]* |
| `ABSCHW_D_TAU_HI` | attenuation · D · _TAU_ · haut | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Percentage of issued long-term filter time constants at maximum speed [pourcentage = valeur]* |
| `ABSCHW_D_TAU_LO` | attenuation · D · _TAU_ · bas | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Percentage of issued long-term filter time constants at minimum speed [pourcentage = valeur]* |
| `ABSTELLPOS_CUSTOM` | position de stationnement · personnalise | `nicht_aktiv` · `aktiv` | *Custom switch-off conditions for cornering lights* |
| `ABSTELLPOS_SMC` | position de stationnement · moteur de phare adaptatif | `nicht_aktiv` · `aktiv` | *Switch-off cornering lights when steering wheel returns (like turn signals)* |
| `AEND_V_SCHWENK_N_AUSSEN` | modification · _V_ · pivotement · N · exterieur | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+10)* |  |
| `ALC_ABTASTRATE_DYN` | phares directionnels (ALC) · frequence d'echantillonnage · dynamique | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+8)* |  |
| `ALC_ABTASTRATE_TREND` | phares directionnels (ALC) · frequence d'echantillonnage · tendance | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+8)* |  |
| `ALC_ABWEICHUNG_TREND` | phares directionnels (ALC) · ecart · tendance | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+8)* |  |
| `ALC_AUSWERT_GRADIENT` | phares directionnels (ALC) · evaluation · pente | `aktiv` · `nicht_aktiv` |  |
| `ALC_AUSWERT_PROP_ABWEICH` | phares directionnels (ALC) · evaluation · proportionnel · ecart | `aktiv` · `nicht_aktiv` |  |
| `ALC_CC_ALC_ERR_OFF` | phares directionnels (ALC) · Check Control · phares directionnels (ALC) · erreur · arret | `nicht_aktiv` · `aktiv` | *No check control notification for adaptive headlights (AHL) fault* |
| `ALC_DAUER_DYN_NACHFUEHR` | phares directionnels (ALC) · duree · dynamique · suivi | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+8)* |  |
| `ALC_DEF_LWR_SENSOR_HI` | phares directionnels (ALC) · defaut · correcteur de site des phares · capteur · haut | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+18)* | *Automatic vertical headlight aim control (LWR) rear height sensor substitute voltage when defective [voltage_mV=valeur*19.6]* |
| `ALC_DIFFERENZ_MSP_DYN_STA` | phares directionnels (ALC) · difference · protection moteur (?) · dynamique · demarreur (?) | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+8)* |  |
| `ALC_DISABLE_AUSGL_NICK_GW` | phares directionnels (ALC) · desactivation · compensation · tangage · passerelle | `nicht_aktiv` · `aktiv` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` … *(+20)* |  |
| `ALC_EINSCHALTBEDINGUNG` | phares directionnels (ALC) · condition d'activation | `wert_01` · `wert_02` · `wert_03` · `wert_04` | *Adaptive headlights (AHL) switch-on conditions [wert_01: with ignition and headlights on, wert_02: with accessories (ignition key position 1) and headlights on, wert_03: with lights on]* |
| `ALC_EPSILON_DELTA_A` | phares directionnels (ALC) · epsilon · ecart · A | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+12)* |  |
| `ALC_EPSILON_DELTA_AV` | phares directionnels (ALC) · epsilon · ecart · audio-video (?) | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+12)* |  |
| `ALC_ERR_BLK_FREQ` | phares directionnels (ALC) · erreur · clignotant · frequence | `wert_01` · `wert_02` | *Blinking frequency of automatic light control (FLC) LED for adaptive headlights (AHL) fault* |
| `ALC_FAHRZEUG` | phares directionnels (ALC) · vehicule | `aktiv` · `nicht_aktiv` | *Adaptive headlights (AHL)* |
| `ALC_GRADIENT_SCHLECHTWEG` | phares directionnels (ALC) · pente · mauvaise route | `aktiv` · `nicht_aktiv` |  |
| `ALC_HOEHENWEG` | phares directionnels (ALC) · course en hauteur | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+8)* |  |
| `ALC_KLS_ENABLE` | phares directionnels (ALC) · commande de climatisation (?) · activation | `nicht_aktiv` · `aktiv` | *Adaptive headlights (AHL) cornering lights enabled* |
| `ALC_LWR_NEIGUNG_KONSTANTE` | phares directionnels (ALC) · correcteur de site des phares · inclinaison · constante | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+10)* | *Automatic vertical headlight aim control (LWR) inclination constant* |
| `ALC_LWR_VORZEICHEN` | phares directionnels (ALC) · correcteur de site des phares · signe | `nicht_aktiv` · `vorn_aktiv` · `aktiv` · `beide_aktiv` | *Complementing height sensors of adaptive headlights (AHL) and automatic vertical headlight aim control (LWR)* |
| `ALC_MINDESTANZAHL_VZ_WECH` | phares directionnels (ALC) · nombre minimal · temporisation (?) · changement | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+8)* |  |
| `ALC_MIN_STEIGUNG_VZ_WECHS` | phares directionnels (ALC) · minimum · pente · temporisation (?) · changement | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+8)* |  |
| `ALC_MITTELUNGSINTERV_DYN` | phares directionnels (ALC) · intervalle de moyennage · dynamique | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+8)* |  |
| `ALC_MITTELUNGSINTERV_TREN` | phares directionnels (ALC) · intervalle de moyennage · separation | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+8)* |  |
| `ALC_MITTELUNGSINT_VZ_WECH` | phares directionnels (ALC) · intervalle de moyennage · temporisation (?) · changement | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+8)* |  |
| `ALC_NACHLAUF_SCHLECHTWEG` | phares directionnels (ALC) · temporisation · mauvaise route | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+8)* |  |
| `ALC_OFFSET_LWR` | phares directionnels (ALC) · decalage · correcteur de site des phares | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+25)* | *Automatic vertical headlight aim control (LWR) offset angle [angle_�=valeur/50]* |
| `ALC_SAETT_TIMER_SCHLECHTW` | phares directionnels (ALC) · saturation · minuterie · mauvaise route | `aktiv` · `nicht_aktiv` |  |
| `ALC_SCHWENK_LATCH` | phares directionnels (ALC) · pivotement · verrou | `nicht_aktiv` · `aktiv` | *Cornering lights enabled until adaptive headlights (AHL) switched off* |
| `ALC_SCHW_GRAD_BESCHLEUNIG` | phares directionnels (ALC) · seuil · degre · acceleration | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+11)* |  |
| `ALC_SCHW_GRAD_BREMSEN` | phares directionnels (ALC) · seuil · degre · freins | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+12)* |  |
| `ALC_SCHW_GRAD_SCHLECHTWEG` | phares directionnels (ALC) · seuil · degre · mauvaise route | `wert_01` |  |
| `ALC_SCHW_PROP_ABWEICHUNG` | phares directionnels (ALC) · seuil · proportionnel · ecart | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+11)* |  |
| `ALC_SCHW_TREND` | phares directionnels (ALC) · seuil · tendance | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+8)* |  |
| `ALC_VMAX_BESCHLEUNIGUNG` | phares directionnels (ALC) · vitesse maximale · acceleration | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+8)* |  |
| `ALC_VMAX_BREMSEN` | phares directionnels (ALC) · vitesse maximale · freins | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+8)* |  |
| `ALC_VMIN_BESCHLEUNIGUNG` | phares directionnels (ALC) · vitesse minimale · acceleration | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+8)* |  |
| `ALC_VMIN_BREMSEN` | phares directionnels (ALC) · vitesse minimale · freins | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+8)* |  |
| `AL_KL50_SCHUTZ` | feux de croisement · borne 50 (demarreur) · protection | `aktiv` · `nicht_aktiv` | *Switch-off low beams when starting engine* |
| `ASP_KEY_MEM_OHNE_MASTER` | retroviseur exterieur · cle · memoire · sans · cle maitre | `aktiv` · `nicht_aktiv` |  |
| `ASP_LIN_VERBAUT` | retroviseur exterieur · bus LIN · installe | `nicht_aktiv` · `aktiv` | *Outside mirrors with LIN-bus* |
| `AUTOM_LICHT_ABSCHALTUNG` | automatique · lumiere · coupure | `nicht_aktiv` · `aktiv` | *Automatically switch off lights when switching off ignition with light switch set to low beams* |
| `AUTOM_LICHT_KONTROLLE` | automatique · lumiere · controle | `nicht_aktiv` · `aktiv` | *Automatic light control (FLC) (turn on headlights automatically when dark)* |
| `AUTO_LWR_ENABLE` | automatique · correcteur de site des phares · activation | `nicht_aktiv` · `aktiv` | *Automatic vertical headlight aim control (LWR) enabled* |
| `BEHOERDENFZG` | vehicule administratif | `nicht_aktiv` · `aktiv` | *Law enforcement vehicle* |
| `BEHOERDFZG_FL_ASYNCHR` | vehicule administratif · feux de route · asynchrone | `nicht_aktiv` · `aktiv` | *Law enforcement vehicle asynchroneous high beams* |
| `BFD_1_ALGORITHMUS` | feux stop adaptatifs (Brake Force Display) · 1 · algorithme | `nicht_aktiv` · `aktiv` | *Brake force display stage 1 algorithm* |
| `BFD_1_BFD_AUSG_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 1 · feux stop adaptatifs (Brake Force Display) · sortie · apparition · image | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `BFD_1_BLINK_FREQ_AUS` | feux stop adaptatifs (Brake Force Display) · 1 · clignotant · frequence · arret | `wert_01` | *Brake force display stage 1 flash frequency off [time_ms=(valeur*40)+100]* |
| `BFD_1_BLINK_FREQ_EIN` | feux stop adaptatifs (Brake Force Display) · 1 · clignotant · frequence · marche | `wert_01` | *Brake force display stage 1 flash frequency on [time_ms=(valeur*40)+100]* |
| `BFD_1_BL_M_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 1 · feu stop · M · apparition · image | `wert_00` · `wert_01` · `wert_02` · `wert_03` | *Brake force display stage 1 third brake light appearance [nicht_aktiv: off, wert_01: on, wert_03: flashing]* |
| `BFD_1_ENTPRELL_ABS` | feux stop adaptatifs (Brake Force Display) · 1 · anti-rebond · ABS | `wert_01` · `wert_02` | *Brake force display stage 1 debouncing antilock braking system (ABS) signal [valeur>0 ? time_ms=valeur*80 : do not use ABS signal]* |
| `BFD_1_ENTPRELL_AX` | feux stop adaptatifs (Brake Force Display) · 1 · anti-rebond · axe (?) | `wert_01` | *Brake force display stage 1 debouncing deceleration signal [duree ms = valeur x 40]* |
| `BFD_1_MIN_GESCHW` | feux stop adaptatifs (Brake Force Display) · 1 · minimum · vitesse | `wert_01` | *Brake force display stage 1 minimum speed* |
| `BFD_1_NSL_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 1 · antibrouillard arriere · apparition · image | `wert_00` · `wert_01` · `wert_02` · `wert_03` | *Brake force display stage 1 rear fog lights appearance [nicht_aktiv: off, wert_01: on, wert_03: flashing]* |
| `BFD_1_RL_BL_1_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 1 · feu arriere · feu stop · 1 · apparition · image | `wert_00` · `wert_01` · `wert_03` · `wert_04` | *Brake force display stage 1 outer tail lights appearance [nicht_aktiv: off, wert_01: on, wert_03: flashing]* |
| `BFD_1_RL_BL_2_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 1 · feu arriere · feu stop · 2 · apparition · image | `wert_00` · `wert_01` · `wert_02` · `wert_03` | *Brake force display stage 1 inner tail lights appearance [nicht_aktiv: off, wert_01: on, wert_03: flashing]* |
| `BFD_1_RL_BL_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 1 · feu arriere · feu stop · apparition · image | `wert_00` · `wert_01` | *Brake force display stage 1 tail lights appearance [nicht_aktiv: off, wert_01: on, wert_03: flashing]* |
| `BFD_1_STUFE_DELAY` | feux stop adaptatifs (Brake Force Display) · 1 · niveau · delai | `wert_01` | *Brake force display stage 1 stage delay [duree ms = valeur x 10]* |
| `BFD_1_THERM_TIMEOUT` | feux stop adaptatifs (Brake Force Display) · 1 · thermique (?) · delai d'attente | `wert_01` · `wert_02` | *Brake force display stage 1 thermal timeout* |
| `BFD_1_WEGFALL` | feux stop adaptatifs (Brake Force Display) · 1 · suppression | `nicht_aktiv` · `aktiv` · `wert_01` · `wert_02` · `wert_03` | *Brake force display stage 1 cancelation [nicht_aktiv: by brake light switch, aktiv: by conditions]* |
| `BFD_2_ALGORITHMUS` | feux stop adaptatifs (Brake Force Display) · 2 · algorithme | `nicht_aktiv` · `aktiv` | *Brake force display stage 2 algorithm* |
| `BFD_2_BFD_AUSG_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 2 · feux stop adaptatifs (Brake Force Display) · sortie · apparition · image | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `BFD_2_BLINK_FREQ_AUS` | feux stop adaptatifs (Brake Force Display) · 2 · clignotant · frequence · arret | `wert_01` | *Brake force display stage 2 flash frequency off [time_ms=(valeur*40)+100]* |
| `BFD_2_BLINK_FREQ_EIN` | feux stop adaptatifs (Brake Force Display) · 2 · clignotant · frequence · marche | `wert_01` | *Brake force display stage 2 flash frequency on [time_ms=(valeur*40)+100]* |
| `BFD_2_BL_M_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 2 · feu stop · M · apparition · image | `wert_00` · `wert_01` · `wert_02` · `wert_03` | *Brake force display stage 1 third brake light appearance [nicht_aktiv: off, wert_01: on, wert_03: flashing]* |
| `BFD_2_ENTPRELL_ABS` | feux stop adaptatifs (Brake Force Display) · 2 · anti-rebond · ABS | `wert_01` · `wert_02` | *Brake force display stage 2 debouncing antilock braking system (ABS) signal [valeur>0 ? time_ms=valeur*80 : do not use ABS signal]* |
| `BFD_2_ENTPRELL_AX` | feux stop adaptatifs (Brake Force Display) · 2 · anti-rebond · axe (?) | `wert_01` | *Brake force display stage 2 debouncing deceleration signal [duree ms = valeur x 40]* |
| `BFD_2_MIN_GESCHW` | feux stop adaptatifs (Brake Force Display) · 2 · minimum · vitesse | `wert_01` | *Brake force display stage 2 minimum speed* |
| `BFD_2_NSL_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 2 · antibrouillard arriere · apparition · image | `wert_00` · `wert_01` · `wert_02` · `wert_03` | *Brake force display stage 1 rear fog lights appearance [nicht_aktiv: off, wert_01: on, wert_03: flashing]* |
| `BFD_2_RL_BL_1_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 2 · feu arriere · feu stop · 1 · apparition · image | `wert_00` · `wert_01` · `wert_02` · `wert_03` | *Brake force display stage 1 outer tail lights appearance [nicht_aktiv: off, wert_01: on, wert_03: flashing]* |
| `BFD_2_RL_BL_2_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 2 · feu arriere · feu stop · 2 · apparition · image | `wert_00` · `wert_01` · `wert_02` · `wert_03` | *Brake force display stage 1 inner tail lights appearance [nicht_aktiv: off, wert_01: on, wert_03: flashing]* |
| `BFD_2_RL_BL_ERSCH_BILD` | feux stop adaptatifs (Brake Force Display) · 2 · feu arriere · feu stop · apparition · image | `wert_00` · `wert_01` | *Brake force display stage 1 tail lights appearance [nicht_aktiv: off, wert_01: on, wert_03: flashing]* |
| `BFD_2_STUFE_DELAY` | feux stop adaptatifs (Brake Force Display) · 2 · niveau · delai | `wert_01` | *Brake force display stage 2 stage delay [duree ms = valeur x 10]* |
| `BFD_2_THERM_TIMEOUT` | feux stop adaptatifs (Brake Force Display) · 2 · thermique (?) · delai d'attente | `wert_01` | *Brake force display stage 2 thermal timeout* |
| `BFD_2_WEGFALL` | feux stop adaptatifs (Brake Force Display) · 2 · suppression | `nicht_aktiv` · `aktiv` · `wert_01` · `wert_02` · `wert_03` | *Brake force display stage 2 cancelation [nicht_aktiv: by brake light switch, aktiv: by conditions]* |
| `BFD_BREMSL_FKT` | feux stop adaptatifs (Brake Force Display) · feu stop · fonction | `aktiv` · `nicht_aktiv` | *Output brake force display as brake lights* |
| `BFD_ERS_DEF_RL_BL` | feux stop adaptatifs (Brake Force Display) · remplacement · defaut · feu arriere · feu stop | `aktiv` · `nicht_aktiv` | *Brake force display replaces defective tail light* |
| `BFD_ERS_DEF_RL_BL_1` | feux stop adaptatifs (Brake Force Display) · remplacement · defaut · feu arriere · feu stop · 1 | `aktiv` · `nicht_aktiv` | *Brake force display replaces defective outer tail light* |
| `BI_XENON_BEI_LH` | bi- (?) · xenon · lors de · gauche | `nicht_aktiv` · `aktiv` | *Flash-to-pass with bi-xenon lights instead of halogen lights* |
| `BI_XENON_FL_ABSCHALT_OPT` | bi- (?) · xenon · feux de route · coupure · option | `wert_01` · `wert_02` · `wert_03` · `wert_04` | *Bi-xenon high beams switch-off option [wert_01: switch off when the stalk is pushed to high beams, wert_02: switch off when the stalk is pushed to flash-to-pass]* |
| `BI_XENON_VERBAUT` | bi- (?) · xenon · installe | `nicht_aktiv` · `aktiv` | *Bi-xenon lights installed* |
| `BLEND_CHECK_IST_WINKEL` | eblouissement · controle · reel · angle | `nicht_aktiv` · `aktiv` | *Evaluate blinding of current angle (for fail-safe mode)* |
| `BLEND_CHECK_SOLL_WINKEL` | eblouissement · controle · consigne · angle | `nicht_aktiv` · `aktiv` | *Evaluate blinding of target angle (for fail-safe mode)* |
| `BLINKIMPULS_CRASHBLK` | impulsion de clignotant · blocage apres choc | `wert_01` | *Crash lights flashing [valeur=0: normal, valeur=1: normal, valeur=2: fast, valeur=3: double impulse]* |
| `BLINKIMPULS_WARNBLK` | impulsion de clignotant · feux de detresse | `wert_01` | *Hazard lights flashing [valeur=0: normal, valeur=1: normal, valeur=2: fast, valeur=3: double impulse]* |
| `CAN_11H` | bus CAN · [0x11] | `nicht_aktiv` · `aktiv` | *Control unit installed in vehicle with CAN11h-bus* |
| `CBD_ZEICHN_INDEX` | [variante CBD] · caractere · index | `wert_01` | *Coding description valeur index* |
| `CC_MELD_BFD_BL` | Check Control · message · feux stop adaptatifs (Brake Force Display) · feu stop | `aktiv` · `nicht_aktiv` | *Check control notification for brake force display displayed as brake light fault* |
| `CC_MELD_NSL_BFD` | Check Control · message · antibrouillard arriere · feux stop adaptatifs (Brake Force Display) | `aktiv` · `nicht_aktiv` | *Check control notification for rear fog light displayed as brake force display fault* |
| `CC_MELD_NSL_SL` | Check Control · message · antibrouillard arriere · feux de position | `aktiv` · `nicht_aktiv` | *Check control notification for rear fog light displayed as standing light fault* |
| `CC_MELD_PARKLICHT` | Check Control · message · feu de stationnement | `aktiv` · `nicht_aktiv` | *Check control notification for parking lights* |
| `CHECK_FGN_SMC_ALC_OFF` | controle · numero de chassis · moteur de phare adaptatif · phares directionnels (ALC) · arret | `nicht_aktiv` · `aktiv` |  |
| `CHECK_SMC_ERROR` | controle · moteur de phare adaptatif · erreur | `nicht_aktiv` · `aktiv` · `wert_01` · `wert_02` | Surveillance des defauts de phare adaptatif. |
| `CKM_ALC` | memoire par cle (Car Key Memory) · phares directionnels (ALC) | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Car/key memory for adaptive headlights (ALC)* |
| `CKM_ALC_DEF` | memoire par cle (Car Key Memory) · phares directionnels (ALC) · defaut | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Default car/key memory for adaptive headlights (ALC)* |
| `COMFORT_ACCESS_LICHT` | confort · acces · lumiere | `nicht_aktiv` · `aktiv` | *Switch off standing lights only when locking vehicle* |
| `CRASH_58G_EIN` | choc · borne 58G · marche | `nicht_aktiv` · `aktiv` | *Instrument lights switch on after crash* |
| `CRASH_WARNBLINKEN` | choc · feux de detresse | `nicht_aktiv` · `aktiv` | *Switch on hazard lights after crash* |
| `C_METER_NACH_INNEN_HI` | C · metre · apres · interieur · haut | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+10)* | *Adjustment range for headlights beam convergence [distance_cm=valeur]* |
| `DEF_HEIMLEUCHTEN_CKM` | defaut · eclairage d'accompagnement · memoire par cle (Car Key Memory) | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` | *Default car/key memory for follow-me-home lights (activated by flashing high beams with ignition off) [wert_01: off] [duree s = valeur]* |
| `DEF_QUITT_BLK_ENTSI_CKM` | defaut · acquittement sonore · clignotant · deverrouillage · memoire par cle (Car Key Memory) | `nicht_aktiv` · `aktiv` | *Default car/key memory for visual unlocking acknowledge without anti-theft alarm system (two turn signal flashes)* |
| `DEF_QUITT_BLK_SICH_CKM` | defaut · acquittement sonore · clignotant · verrouillage · memoire par cle (Car Key Memory) | `nicht_aktiv` · `aktiv` | *Default car/key memory for visual locking acknowledge without anti-theft alarm system (single turn signal flash)* |
| `DEF_TAGFAHRLICHT_ECE_CKM` | defaut · feux de jour · norme europeenne · memoire par cle (Car Key Memory) | `nicht_aktiv` · `aktiv` | *Default car/key memory for Europe daytime running lights* |
| `DEF_ZYKL_TIPP_BLK_CKM` | defaut · cycle · impulsion · clignotant · memoire par cle (Car Key Memory) | `wert_01` · `wert_02` | *Default car/key memory for one-touch turn signal functionality [blinkcount=valeur]* |
| `DRIFT_DEKR` | derive · decrement | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Decrement drift timer* |
| `DRIFT_INKR` | derive · increment | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Increment drift timer* |
| `DRIFT_SCHWELLE` | derive · seuil | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Reduction percentage for drift timer maximum value [pourcentage = valeur]* |
| `DRL_V_GEDIMMT` | feux de jour · _V_ · attenue | `nicht_aktiv` · `aktiv` | *Dimmed front daytime running lights* |
| `DUAL_SP__AL_L` | bi-zone · point de passage de rapport · feux de croisement · _L_ | `nicht_aktiv` · `aktiv` | *Dual voltage left low beam* |
| `DUAL_SP__AL_R` | bi-zone · point de passage de rapport · feux de croisement · _R_ | `nicht_aktiv` · `aktiv` | *Dual voltage right low beam* |
| `DUAL_SP__BLK_H_L` | bi-zone · point de passage de rapport · clignotant · _H_ · _L_ | `nicht_aktiv` · `aktiv` | *Dual voltage left rear turn signal* |
| `DUAL_SP__BLK_H_R` | bi-zone · point de passage de rapport · clignotant · _H_ · _R_ | `nicht_aktiv` · `aktiv` | *Dual voltage right rear turn signal* |
| `DUAL_SP__BLK_V_L` | bi-zone · point de passage de rapport · clignotant · _V_ · _L_ | `nicht_aktiv` · `aktiv` | *Dual voltage left front turn signal* |
| `DUAL_SP__BLK_V_R` | bi-zone · point de passage de rapport · clignotant · _V_ · _R_ | `nicht_aktiv` · `aktiv` | *Dual voltage right front turn signal* |
| `DUAL_SP__BL_L` | bi-zone · point de passage de rapport · feu stop · _L_ | `nicht_aktiv` · `aktiv` | *Dual voltage left brake light* |
| `DUAL_SP__BL_M_1` | bi-zone · point de passage de rapport · feu stop · M · 1 | `nicht_aktiv` · `aktiv` | *Dual voltage third brake light 1* |
| `DUAL_SP__BL_R` | bi-zone · point de passage de rapport · feu stop · _R_ | `nicht_aktiv` · `aktiv` | *Dual voltage right brake light* |
| `DUAL_SP__FL_L` | bi-zone · point de passage de rapport · feux de route · _L_ | `nicht_aktiv` · `aktiv` | *Dual voltage left high beam* |
| `DUAL_SP__FL_R` | bi-zone · point de passage de rapport · feux de route · _R_ | `nicht_aktiv` · `aktiv` | *Dual voltage right high beam* |
| `DUAL_SP__KZL` | bi-zone · point de passage de rapport · eclairage de plaque | `nicht_aktiv` · `aktiv` | *Dual voltage license plate lights* |
| `DUAL_SP__NSL_L` | bi-zone · point de passage de rapport · antibrouillard arriere · _L_ | `nicht_aktiv` · `aktiv` | *Dual voltage left rear fog light* |
| `DUAL_SP__NSL_R` | bi-zone · point de passage de rapport · antibrouillard arriere · _R_ | `nicht_aktiv` · `aktiv` | *Dual voltage right rear fog light* |
| `DUAL_SP__NSW_L` | bi-zone · point de passage de rapport · antibrouillard avant · _L_ | `nicht_aktiv` · `aktiv` | *Dual voltage left front fog light* |
| `DUAL_SP__NSW_R` | bi-zone · point de passage de rapport · antibrouillard avant · _R_ | `nicht_aktiv` · `aktiv` | *Dual voltage right front fog light* |
| `DUAL_SP__RFS_L` | bi-zone · point de passage de rapport · feu de recul · _L_ | `nicht_aktiv` · `aktiv` | *Dual voltage left reversing light* |
| `DUAL_SP__RFS_R` | bi-zone · point de passage de rapport · feu de recul · _R_ | `nicht_aktiv` · `aktiv` | *Dual voltage right reversing light* |
| `DUAL_SP__RL_BL_1_L` | bi-zone · point de passage de rapport · feu arriere · feu stop · 1 · _L_ | `nicht_aktiv` · `aktiv` | *Dual voltage left outer tail light* |
| `DUAL_SP__RL_BL_1_R` | bi-zone · point de passage de rapport · feu arriere · feu stop · 1 · _R_ | `nicht_aktiv` · `aktiv` | *Dual voltage right outer tail light* |
| `DUAL_SP__RL_BL_2_L` | bi-zone · point de passage de rapport · feu arriere · feu stop · 2 · _L_ | `nicht_aktiv` · `aktiv` | *Dual voltage left inner tail light* |
| `DUAL_SP__RL_BL_2_R` | bi-zone · point de passage de rapport · feu arriere · feu stop · 2 · _R_ | `nicht_aktiv` · `aktiv` | *Dual voltage right inner tail light* |
| `DUAL_SP__SL_V_L` | bi-zone · point de passage de rapport · feux de position · _V_ · _L_ | `nicht_aktiv` · `aktiv` | *Dual voltage left front standing light* |
| `DUAL_SP__SL_V_R` | bi-zone · point de passage de rapport · feux de position · _V_ · _R_ | `nicht_aktiv` · `aktiv` | *Dual voltage right front standing light* |
| `DUAL_SP___BFD_L` | bi-zone · point de passage de rapport · feux stop adaptatifs (Brake Force Display) · _L_ | `nicht_aktiv` · `aktiv` | *Dual voltage left brake force display* |
| `DUAL_SP___BFD_R` | bi-zone · point de passage de rapport · feux stop adaptatifs (Brake Force Display) · _R_ | `nicht_aktiv` · `aktiv` | *Dual voltage right brake force display* |
| `DYNAM_LWR_ENABLE` | dynamique · correcteur de site des phares · activation | `nicht_aktiv` · `aktiv` | *Dynamic vertical headlight aim control (LWR) enabled* |
| `D_TAU_MITTEL_GIERR` | D · _TAU_ · moyen · vitesse de lacet | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Long-term filter time constant for yaw rate [duree ms = valeur x 10]* |
| `D_TAU_MITTEL_LENKW` | D · _TAU_ · moyen · angle de braquage | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Long-term filter time constant for steering angle [duree ms = valeur x 10]* |
| `ENABLE_GURTBRINGER` | activation · avance-ceinture | `nicht_aktiv` · `aktiv` | *Automatic seat belt hand-over* |
| `ERR_LWR_L_IGNORIEREN` | erreur · correcteur de site des phares · _L_ · ignorer | `nicht_aktiv` · `aktiv` | *Ignore left vertical headlight aim control (LWR) fault* |
| `ERR_LWR_R_IGNORIEREN` | erreur · correcteur de site des phares · _R_ · ignorer | `nicht_aktiv` · `aktiv` | *Ignore right vertical headlight aim control (LWR) fault* |
| `ERR_SCHWENK_L_IGNORIEREN` | erreur · pivotement · _L_ · ignorer | `nicht_aktiv` · `aktiv` | *Ignore left cornering light fault* |
| `ERR_SCHWENK_R_IGNORIEREN` | erreur · pivotement · _R_ · ignorer | `nicht_aktiv` · `aktiv` | *Ignore right cornering light fault* |
| `FAHRWEG_BORDSTEIN_AUTOM` | course · bordure de trottoir · automatique | `wert_01` |  |
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
| `FEHLERMELD_CC_RL_BL_1_L` | message d'erreur · Check Control · feu arriere · feu stop · 1 · _L_ | `nicht_aktiv` · `aktiv` | *Check control notification for left outer tail light* |
| `FEHLERMELD_CC_RL_BL_1_R` | message d'erreur · Check Control · feu arriere · feu stop · 1 · _R_ | `nicht_aktiv` · `aktiv` | *Check control notification for right outer tail light* |
| `FEHLERMELD_CC_RL_BL_2_L` | message d'erreur · Check Control · feu arriere · feu stop · 2 · _L_ | `nicht_aktiv` · `aktiv` | *Check control notification for left inner tail light* |
| `FEHLERMELD_CC_RL_BL_2_R` | message d'erreur · Check Control · feu arriere · feu stop · 2 · _R_ | `nicht_aktiv` · `aktiv` | *Check control notification for right inner tail light* |
| `FH_CC_MELDUNGEN` | leve-vitre · Check Control · messages | `aktiv` · `nicht_aktiv` | *Check control notification for power windows fault* |
| `FH_FAHRERTUER_TIPP_AUF` | leve-vitre · porte conducteur · impulsion · ouverture | `aktiv` · `nicht_aktiv` | *One-touch opening driver power window* |
| `FH_FAHRERTUER_TIPP_ZU` | leve-vitre · porte conducteur · impulsion · fermeture | `aktiv` · `nicht_aktiv` | *One-touch closing driver power window* |
| `FL_ABSCHALT_OPT` | feux de route · coupure · option | `aktiv` · `nicht_aktiv` | *High beams switch-off option [nicht_aktiv: switch off when the stalk is pushed to flash-to-pass, aktiv: switch off when the stalk is pushed to high beams]* |
| `FL_ASSIST_CC_NOT_AKTIV` | feux de route · assistance · Check Control · secours (?) · actif | `aktiv` · `nicht_aktiv` | *Check control notification for high beam assist (FLA) fault* |
| `FL_ASSIST_DEF_AKTIV` | feux de route · assistance · defaut · actif | `aktiv` · `nicht_aktiv` | *High beam assist (FLA) default enabled* |
| `FL_ASSIST_FUNKTIONS_ANZ` | feux de route · assistance · fonctionnel · nombre | `aktiv` · `nicht_aktiv` | *High beam assist (FLA) function display* |
| `FL_ASSIST_ZUST_22_32` | feux de route · assistance · etat · 22 · 32 | `aktiv` · `nicht_aktiv` | *High beam assist (FLA) transition from 22 to 32* |
| `FL_AUS_BEI_BIX_IN_S_FLC` | feux de route · arret · lors de · bi-xenon · entree · S · _FLC_ | `aktiv` · `nicht_aktiv` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` | *High beams with bi-xenon lights and not with halogen lights when light switch set to automatic light control (FLC)* |
| `FL_UND_NSW_NICHT_GLEICHZ` | feux de route · et · antibrouillard avant · non · simultane | `nicht_aktiv` · `aktiv` | Autorise les antibrouillards en meme temps que les feux de route. |
| `FSP_ISP_SPERREN` | memoire de defauts (?) · traitement du signal (?) · blocage | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` | *Fault/information memory mask enabled [wert_00: none, wert_01: fault memory mask, wert_02: information memory mask, wert_03: both]* |
| `FZG_RADSTAND` | vehicule · empattement | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+9)* | *Vehicle wheelbase [length_m=valeur/50]* |
| `GB_DELTA_POS` | Grande-Bretagne · ecart · position | `wert_01` |  |
| `GB_GESCHW` | Grande-Bretagne · vitesse | `wert_01` |  |
| `GB_POS_AUS` | Grande-Bretagne · position · arret | `wert_01` |  |
| `GB_T_EINFAHREN` | Grande-Bretagne · T · rodage | `wert_01` |  |
| `GB_T_NOTLAUF` | Grande-Bretagne · T · mode degrade | `wert_01` · `wert_02` |  |
| `GB_USE_SBE` | Grande-Bretagne · utilisation · detection d'occupation du siege | `nicht_aktiv` · `aktiv` | *Automatic seat belt hand-over uses seat occupancy detection* |
| `GESCHW_ASP_AUTO_AUSKLAPP` | vitesse · retroviseur exterieur · automatique · depliage | `wert_01` | *Threshold speed for automatic folding out outside mirrors [vitesse km/h = valeur]* |
| `GESCHW_DOWN` | vitesse · bas | `wert_01` | *Lower speed limit [vitesse km/h = valeur]* |
| `GESCHW_UP` | vitesse · haut | `wert_01` | *Upper speed limit [vitesse km/h = valeur]* |
| `GETRIEBE_HANDSCHALTER` | boite de vitesses · commande manuelle | `aktiv` · `nicht_aktiv` | *Manual transmission* |
| `HECKDECK_REVERS_BLK` | couvercle de coffre · inversion · clignotant | `wert_01` · `wert_02` |  |
| `HECKD_REV_BLK_DOPPELB` | hayon · revision · clignotant · double | `nicht_aktiv` · `aktiv` |  |
| `HEIMLEUCHTEN` | eclairage d'accompagnement | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` | *Follow-me-home lights (activated by flashing high beams with ignition off) [wert_01: off] [duree s = valeur]* |
| `HEIMLEUCHTEN_ABBRUCH` | eclairage d'accompagnement · interruption | `aktiv` · `nicht_aktiv` | *Cancel follow-me-home lights with flash-to-pass / high beams* |
| `HEIMLEUCHTEN_ENABLE` | eclairage d'accompagnement · activation | `nicht_aktiv` · `aktiv` | *Follow-me-home lights enabled* |
| `HEIMLEUCHTEN_VERLAENG` | eclairage d'accompagnement · prolongation | `aktiv` · `nicht_aktiv` | *Extend follow-me-home lights with flash-to-pass* |
| `IBUS_R_GANG` | bus I · _R_ · rapport | `nicht_aktiv` · `aktiv` | *Reverse signal from I-bus* |
| `IB_2MIN_OPTION` | bus interne (?) · 2 min · option | `aktiv` · `nicht_aktiv` | *Interior lights 2 minutes option* |
| `IB_SOFTON_OPTION` | bus interne (?) · allumage progressif · option | `aktiv` · `nicht_aktiv` | *Interior lights soft-on option* |
| `IK_BUS_TEILNEMER` | circuit (?) · bus · participant | `nicht_aktiv` · `aktiv` | *I/K-bus device* |
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
| `KALTUEBERW_RL_BL_1_L` | surveillance a froid · feu arriere · feu stop · 1 · _L_ | `nicht_aktiv` · `aktiv` | *Cold monitoring of left outer tail light* |
| `KALTUEBERW_RL_BL_1_R` | surveillance a froid · feu arriere · feu stop · 1 · _R_ | `nicht_aktiv` · `aktiv` | *Cold monitoring of right outer tail light* |
| `KALTUEBERW_RL_BL_2_L` | surveillance a froid · feu arriere · feu stop · 2 · _L_ | `nicht_aktiv` · `aktiv` | *Cold monitoring of left inner tail light* |
| `KALTUEBERW_RL_BL_2_R` | surveillance a froid · feu arriere · feu stop · 2 · _R_ | `nicht_aktiv` · `aktiv` | *Cold monitoring of right inner tail light* |
| `KEINE_ENTRFLANKE_4_IB` | aucun · front de deverrouillage · 4 · bus interne (?) | `aktiv` · `nicht_aktiv` |  |
| `KEINE_L_R_SYNCHRONISATION` | aucun · _L_ · _R_ · synchronisation | `nicht_aktiv` · `aktiv` | *No synchronization between left and right stepper motor controller during reference run* |
| `KEINE_PARKLICHT_FUNKT` | aucun · feu de stationnement · fonction | `nicht_aktiv` · `aktiv` | *No parking lights functionality* |
| `KENNL_MAX_UEBERST` | courbe caracteristique · maximum · survirage | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Maximum oversteer characteristic curve* |
| `KENNL_REDUZ_MAX_SCHWENK` | courbe caracteristique · reduction · maximum · pivotement | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Reduction percentage for characteristic curve with headlights maximum swivelling angle [pourcentage = valeur]* |
| `KLASSIF_DRIFT_ENA` | classification · derive · activation | `nicht_aktiv` · `aktiv` | *Classifier for drift enabled* |
| `KLASSIF_LENKM_ENA` | classification · moment de direction · activation | `nicht_aktiv` · `aktiv` | *Classifier for steering abuse enabled* |
| `KLASSIF_UEBERST_ENA` | classification · survirage · activation | `nicht_aktiv` · `aktiv` | *Classifier for oversteer enabled* |
| `KLS_BEI_AL_AKTIV` | commande de climatisation (?) · lors de · feux de croisement · actif | `nicht_aktiv` · `aktiv` | *Cornering lights enabled when light switch set to low beams* |
| `KL_VA_EIN_BEI_ENTRIEGELN` | borne · essieu avant · marche · lors de · deverrouiller | `aktiv` · `nicht_aktiv` |  |
| `KURZSCHLUSS_CHECK_TIME` | court-circuit · controle · duree | `wert_01` | *Short circuit detection time* |
| `KURZSCHLUSS_DIAGNOSE_OPT` | court-circuit · diagnostic · option | `nicht_aktiv` · `aktiv` | *Short circuit detection switch-on delay* |
| `KURZSCHLUSS_ENTPRELL` | court-circuit · anti-rebond | `wert_01` | *Short circuit detection debounce count* |
| `KURZSCHL_WIERHOL_ZAEHLER` | court-circuit · repetition · compteur | `wert_01` · `wert_02` | *Short circuit repeat counter* |
| `LENKSTOCK_UMSCHALT_VERZ` | commodo · commutation · temporisation | `wert_01` | *Steering column stalk changeover delay [duree ms = valeur x 10]* |
| `LENKWINKEL_BEI_KLR` | angle de braquage · lors de · regulation de climatisation | `aktiv` · `nicht_aktiv` | *Steering angle evaluation enabled with accessories on (ignition key position 1)* |
| `LENKWINKEL_UEBERSCHREITEN` | angle de braquage · depassement superieur | `wert_01` | *Steering angle to exceed before turn signals switch-off in opposite direction is armed [angle_�=valeur*0.35]* |
| `LENKWINKEL_UNTERSCHREITEN` | angle de braquage · depassement inferieur | `wert_01` | *Steering angle in opposite direction before turn signals are switched off [angle_�=valeur*0.35]* |
| `LENK_UEBERSETZUNG` | direction · rapport de demultiplication | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+15)* | *Steering ratio [ratio=valeur/5]* |
| `LENK_UEBERSETZUNG_LO` | direction · rapport de demultiplication · bas | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+12)* | *Minimum speed steering ratio* |
| `LEUCHTDICHTE_EXPO` | luminance · exposition | `nicht_aktiv` · `aktiv` | *Exponential luminance change for instrument lights* |
| `LICHTZ_R_L_FAKTOR` | duree d'eclairage · _R_ · _L_ · facteur | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Amplification factor for left headlight with left-hand traffic / right headlight with right-hand traffic* |
| `LICHT_AN_LED` | lumiere · marche · LED | `nicht_aktiv` · `aktiv` | *Switch on automatic light control (FLC) LED when headlights are on* |
| `LICHT_AN_WARNUNG` | lumiere · marche · alerte | `nicht_aktiv` · `aktiv` | *Lights left on warning* |
| `LICHT_AN_WARN_KL_R` | lumiere · marche · alerte · borne · _R_ | `nicht_aktiv` · `aktiv` |  |
| `LINKSVERKEHR` | conduite a gauche | `nicht_aktiv` · `aktiv` | *Left-hand traffic* |
| `LOAD_DUMP_DIMM_WERL_BLK` | charge · extraction memoire · gradation · valeur (?) · clignotant | `wert_01` | *Load dump protection dimming value for turn signals* |
| `LOAD_DUMP_MAX_TIME` | charge · extraction memoire · maximum · duree | `wert_01` · `wert_02` | *Load dump protection timeout [time_s=valeur*4]* |
| `LOAD_DUMP_PRELL` | charge · extraction memoire · rebond | `wert_01` · `wert_02` · `wert_03` · `wert_04` | *Load dump protection debouncing [duree ms = valeur x 10]* |
| `LOAD_DUMP_SCHUTZ_AKTIV` | charge · extraction memoire · protection · actif | `aktiv` · `nicht_aktiv` | *Load dump protection enabled* |
| `LOAD_DUMP_SCHUTZ_BLK_Z` | charge · extraction memoire · protection · clignotant · Z | `aktiv` · `nicht_aktiv` | *Load dump protection for auxiliary turn signals* |
| `LOAD_DUMP_SCHUTZ_FL` | charge · extraction memoire · protection · feux de route | `aktiv` · `nicht_aktiv` | *Load dump protection for high beams* |
| `LOAD_DUMP_SCHUTZ_KL58G` | charge · extraction memoire · protection · borne 58G | `aktiv` · `nicht_aktiv` | *Load dump protection for instrument lights* |
| `LOAD_DUMP_SCHUTZ_KZL` | charge · extraction memoire · protection · eclairage de plaque | `aktiv` · `nicht_aktiv` | *Load dump protection for license plate lights* |
| `LOAD_DUMP_SCHUTZ_NSL` | charge · extraction memoire · protection · antibrouillard arriere | `aktiv` · `nicht_aktiv` | *Load dump protection for rear fog lights* |
| `LOAD_DUMP_SCHUTZ_NSW` | charge · extraction memoire · protection · antibrouillard avant | `aktiv` · `nicht_aktiv` | *Load dump protection for front fog lights* |
| `LOAD_DUMP_SCHUTZ_RFS` | charge · extraction memoire · protection · feu de recul | `aktiv` · `nicht_aktiv` | *Load dump protection for reversing lights* |
| `LOAD_DUMP_SCHUTZ_RL_BL_2` | charge · extraction memoire · protection · feu arriere · feu stop · 2 | `aktiv` · `nicht_aktiv` | *Load dump protection for inner tail lights* |
| `LOAD_DUMP_SCHUTZ_SL_V` | charge · extraction memoire · protection · feux de position · _V_ | `aktiv` · `nicht_aktiv` | *Load dump protection for front standing lights* |
| `LWR_ABSTELLPOS` | correcteur de site des phares · position de stationnement | `wert_01` · `wert_12` |  |
| `LWR_ABS_MIN_STELLUNG_2` | correcteur de site des phares · ABS · minimum · position · 2 | `wert_01` · `wert_02` · `wert_03` · `wert_04` | *Manual vertical headlight aim control (LWR) position 2 absolute minimum steps* |
| `LWR_ANSTEUERN` | correcteur de site des phares · commander | `aktiv` · `nicht_aktiv` | *Vertical headlight aim control (LWR)* |
| `LWR_DIAG_PARAM` | correcteur de site des phares · diagnostic · parametre | `wert_01` · `wert_02` | *Vertical headlight aim control (LWR) diagnostic parameters* |
| `LWR_FEHLER_BEWERTUNG` | correcteur de site des phares · erreur · evaluation | `wert_01` · `wert_02` | *Vertical headlight aim control (LWR) fault evaluation* |
| `LWR_HEBEL` | correcteur de site des phares · levier | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` | *Vertical headlight aim control (LWR) headlight lever length [length_mm=valeur]* |
| `LWR_HYSTERESE_START` | correcteur de site des phares · hysteresis · demarrage | `wert_01` · `wert_02` | *Manual vertical headlight aim control (LWR) hysteresis* |
| `LWR_SCHRITTABSTAND_GRENZE` | correcteur de site des phares · intervalle de pas · limite | `wert_01` | *Manual vertical headlight aim control (LWR) minimum step distance at low temperature* |
| `LWR_SCHRITTE_STELLUNG_0` | correcteur de site des phares · pas · position · 0 | `wert_01` · `wert_02` | *Manual vertical headlight aim control (LWR) position 0 steps* |
| `LWR_SCHRITTE_STELLUNG_1` | correcteur de site des phares · pas · position · 1 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_06` | *Manual vertical headlight aim control (LWR) position 1 steps* |
| `LWR_SCHRITTE_STELLUNG_2` | correcteur de site des phares · pas · position · 2 | `wert_01` · `wert_02` · `wert_03` · `wert_06` | *Manual vertical headlight aim control (LWR) position 2 steps* |
| `LWR_TEMPERATUR_GRENZE` | correcteur de site des phares · temperature · limite | — | *Vertical headlight aim control (LWR) lower temperature limit* |
| `LWR_TERMO_MODEL_EIN` | correcteur de site des phares · thermique · modele · marche | `aktiv` | *Vertical headlight aim control (LWR) temperature model enabled* |
| `LWR_TESTBETRIEB` | correcteur de site des phares · mode test | `aktiv` · `nicht_aktiv` | *Vertical headlight aim control (LWR) test operation* |
| `LWR_VORBESTROMUNG` | correcteur de site des phares · pre-alimentation | `wert_01` · `wert_02` |  |
| `MAX_BETAETIGUNG_FOMEHO` | maximum · actionnement · eclairage d'accompagnement (follow-me-home) | `wert_01` · `wert_02` | *Maximum number of follow-me-home activations [valeur>0 ? count=valeur-1 : unlimited]* |
| `MAX_DELTA_KEINE_VERR` | maximum · ecart · aucun · verrouillage | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Maximum delta between actual and target angle without reduction of headlights swivelling speed [angle_�=valeur/10]* |
| `MAX_GESCHW_EINKLAPPEN_2` | maximum · vitesse · rabattement · 2 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `polnisch` · `dynamik_2` · `zieltermin` | *Maximum speed for folding in outside mirrors [vitesse km/h = valeur]* |
| `MAX_SW_SCHWENK_V_HORIZ` | maximum · logiciel · pivotement · _V_ · horizontal | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+10)* | *Headlights maximum horizontal swivelling speed [speed_�/s=1.12^valeur]* |
| `MAX_SW_SCHWENK_V_VERTI` | maximum · logiciel · pivotement · _V_ · vertical | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+11)* | *Headlights maximum vertical swivelling speed [speed_�/s=(1.65^valeur)/2]* |
| `MAX_SW_SCHWENK_W_AUSSEN` | maximum · logiciel · pivotement · W · exterieur | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+12)* | *Headlights maximum swivelling angle to outside [angle_�=valeur/10]* |
| `MAX_SW_SCHWENK_W_INNEN` | maximum · logiciel · pivotement · W · interieur | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+10)* | *Headlights maximum swivelling angle to inside [angle_�=valeur/10]* |
| `MAX_VERRINGERUNG_V_SCHW` | maximum · reduction · _V_ · seuil | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+10)* | *Headlights maximum swivelling speed reduction* |
| `MELDUNG_AENDERUNG_DIMMER` | message · modification · variateur | `wert_01` | *Instrument lights dimmer change before K-bus message generated [angle_�=(valeur*120)/255]* |
| `MEM_POS_ANFAHREN` | memoire · position · demarrage | `wert_01` |  |
| `MERKM_1_BEGR` | caracteristique · 1 · limitation | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Limit for maximum steering wheel movement amplitude* |
| `MERKM_1_DEKR` | caracteristique · 1 · decrement | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Decrement steering wheel movement timer* |
| `MERKM_1_INKR` | caracteristique · 1 · increment | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Increment steering wheel movement timer* |
| `MERKM_1_MAX` | caracteristique · 1 · maximum | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Maximum steering wheel movement amplitude* |
| `MIND_ANZ_ZYKL_TIPP_BLK` | minimum · nombre · cycle · impulsion · clignotant | `wert_01` · `wert_02` | Nombre de clignotements sur simple impulsion du comodo. |
| `MIN_DELTA_MAX_VERR` | minimum · ecart · maximum · verrouillage | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Minimum delta between actual and target angle with maximum reduction of headlights swivelling speed [angle_�=valeur/10]* |
| `MIN_SW_SCHWENK_V_HORIZ` | minimum · logiciel · pivotement · _V_ · horizontal | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Headlights minimum horizontal swivelling speed [speed_�/s=1.12^valeur]* |
| `MIN_SW_SCHWENK_V_VERTI` | minimum · logiciel · pivotement · _V_ · vertical | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+8)* | *Headlights minimum vertical swivelling speed [speed_�/s=(1.65^valeur)/2]* |
| `NACHLAUF_KLEMME_VA` | temporisation · borne · essieu avant | `wert_01` |  |
| `NORM_LEUCHTWEITE` | norme · portee d'eclairage | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+9)* | *Normal headlights range [range_m=valeur]* |
| `NOTLAUF_FKT_GESCHW` | mode degrade · fonction · vitesse | `wert_01` · `wert_02` |  |
| `NOTLAUF_FKT_SCHALTERSTELL` | mode degrade · fonction · position du commodo | `aktiv` · `nicht_aktiv` |  |
| `NOTLAUF_FKT_TIMER` | mode degrade · fonction · minuterie | `wert_01` |  |
| `NSL_ERS_DEF_RL_BL_1` | antibrouillard arriere · remplacement · defaut · feu arriere · feu stop · 1 | `aktiv` · `nicht_aktiv` | *Rear fog light replaces defective outside tail light* |
| `NSL_VERBAU_OTP` | antibrouillard arriere · montage · memoire non reinscriptible | `wert_01` · `wert_02` · `wert_03` | *Rear fog lights installation option [wert_01: left and right rear fog light, wert_02: only right rear fog light, wert_03: only left rear fog light]* |
| `OHNE_GIERRATE` | sans · vitesse de lacet | `nicht_aktiv` · `aktiv` | *Without yaw rate* |
| `OPTIONEN_SL_ABSCH` | options · feux de position · coupure | `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `PANIKMODUS_EKS` | mode panique · systeme electronique de cle | `aktiv` · `nicht_aktiv` | *Panic close with jamming protection* |
| `POTI_DIMM_INVERS` | potentiometre · gradation · inverse | `nicht_aktiv` · `aktiv` | *Inverted potentiometer instrument for lights dimmer* |
| `POTI_LWR_INVERS` | potentiometre · correcteur de site des phares · inverse | `nicht_aktiv` · `aktiv` | *Inverted potentiometer for vertical headlight aim control (LWR)* |
| `PRE_SLEEP_ZEIT` | pre- · veille · duree | `wert_01` · `wert_02` · `wert_03` | *Time until entering sleep mode [wert_01: sleeps after locking vehicle, wert_02: stays awake for 6 more minutes after locking vehicle, wert_03: never sleeps]* |
| `PROZENT_EE` | pourcentage · memoire EEPROM | `wert_01` · `wert_02` | *Control range in percent [valeur = 0..99]* |
| `QUITT_BLK_ENTSICHERN` | acquittement sonore · clignotant · deverrouillage | `nicht_aktiv` · `aktiv` | *Visual unlocking acknowledge without anti-theft alarm system (two turn signal flashes)* |
| `QUITT_BLK_IGNORE_MOTORH` | acquittement sonore · clignotant · ignorer · capot moteur | `nicht_aktiv` · `aktiv` | *Visual locking acknowledge without checking hood* |
| `QUITT_BLK_SICHERN` | acquittement sonore · clignotant · verrouillage | `nicht_aktiv` · `aktiv` | *Visual locking acknowledge without anti-theft alarm system (single turn signal flash)* |
| `QUITT_BLK_ZU_CHECK` | acquittement sonore · clignotant · fermeture · controle | `nicht_aktiv` · `aktiv` | *Visual locking acknowledge without checking all doors and lids closed* |
| `RAUSCH` | bruit | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Minimum angle to detect steering wheel movement [angle_�=valeur/40]* |
| `RECHTSVERKEHR` | conduite a droite | `nicht_aktiv` · `aktiv` | *Right-hand traffic* |
| `RECHTS_LENKER_ASP` | droite · volant · retroviseur exterieur | `aktiv` · `nicht_aktiv` |  |
| `RECHTS_LENKER_FH_H` | droite · volant · leve-vitre · _H_ | `aktiv` · `nicht_aktiv` | *Right-hand drive rear power windows* |
| `RECHTS_LENKER_FH_V` | droite · volant · leve-vitre · _V_ | `aktiv` · `nicht_aktiv` | *Right-hand drive front power windows* |
| `REDUZIERUNG_V_SCHWENK_HI` | reduction · _V_ · pivotement · haut | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Reduction of the headlights swivelling speed above maximum speed* |
| `REDUZIERUNG_V_SCHWENK_LO` | reduction · _V_ · pivotement · bas | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Reduction of the headlights swivelling speed below minimum speed* |
| `RL_BL_1_BREMSL_FKT` | feu arriere · feu stop · 1 · feu stop · fonction | `aktiv` · `nicht_aktiv` | *Output outer tail lights as brake lights* |
| `RL_BL_1_PARKL_FKT` | feu arriere · feu stop · 1 · feu de stationnement · fonction | `aktiv` · `nicht_aktiv` | *Output outer tail lights as parking lights* |
| `RL_BL_1_PO_RL_FKT` | feu arriere · feu stop · 1 · position (?) · feu arriere · fonction | `aktiv` · `nicht_aktiv` | *Output outer tail lights as position/tail lights* |
| `RL_BL_2_BREMSL_FKT` | feu arriere · feu stop · 2 · feu stop · fonction | `aktiv` · `nicht_aktiv` | *Output inner tail lights as brake lights* |
| `RL_BL_2_PARKL_FKT` | feu arriere · feu stop · 2 · feu de stationnement · fonction | `aktiv` · `nicht_aktiv` | *Output inner tail lights as parking lights* |
| `RL_BL_2_PO_RL_FKT` | feu arriere · feu stop · 2 · position (?) · feu arriere · fonction | `aktiv` · `nicht_aktiv` | *Output inner tail lights as position/tail lights* |
| `RL_BL_BREMSL_FKT` | feu arriere · feu stop · feu stop · fonction | `aktiv` · `nicht_aktiv` | *Output tail lights as brake lights* |
| `RL_BL_PARKL_FKT` | feu arriere · feu stop · feu de stationnement · fonction | `aktiv` · `nicht_aktiv` | *Output tail lights as parking lights* |
| `RL_BL_PO_RL_FKT` | feu arriere · feu stop · position (?) · feu arriere · fonction | `aktiv` · `nicht_aktiv` | *Output tail lights as position/tail lights* |
| `SCHALTER_R_GANG` | interrupteur · _R_ · rapport | `nicht_aktiv` · `aktiv` | *Reverse signal directly from switch* |
| `SCHW_TAU_LW_GR` | seuil · _TAU_ · portee d'eclairage (?) · groupe (?) | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Threshold time constant steering angle in drifts [threshold_�=valeur]* |
| `SICHERHEITS_FZG_LM` | de securite · vehicule · module d'eclairage (?) | `aktiv` · `nicht_aktiv` | *Security vehicle* |
| `SIDEMARKER_US` | feu lateral (US) · marche americain | `nicht_aktiv` · `aktiv` | Feux lateraux de gabarit, obligatoires sur le marche americain. |
| `SL_ABSCH_OHNE_ECE` | feux de position · coupure · sans · norme europeenne | `nicht_aktiv` · `aktiv` | *Standing lights also switch off automatically without Europe daytime running lights* |
| `SL_BL_CC_MELD_E8X` | feux de position · feu stop · Check Control · message · serie E8x | `aktiv` · `nicht_aktiv` |  |
| `SL_BL_ERS_DEF_RL_BL_1` | feux de position · feu stop · remplacement · defaut · feu arriere · feu stop · 1 | `aktiv` · `nicht_aktiv` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `SL_PARKL_UNTERSP_ABSCHALT` | feux de position · feu de stationnement · sous-tension · coupure | `nicht_aktiv` · `aktiv` | *Parking lights switch off at undervoltage* |
| `SL_VERZOEGERUNG` | feux de position · temporisation | `wert_01` · `wert_02` · `wert_03` |  |
| `SL_V_PARKL_FKT` | feux de position · _V_ · feu de stationnement · fonction | `aktiv` · `nicht_aktiv` | *Output front standing lights as parking lights* |
| `SL_V_PO_RL_FKT` | feux de position · _V_ · position (?) · feu arriere · fonction | `aktiv` · `nicht_aktiv` | *Output front standing lights as position/tail lights* |
| `SMART_BEAM` | intelligent · faisceau | `aktiv` · `nicht_aktiv` | *High beam assist (FLA)* |
| `SMART_BEAM_ZUSTAND` | intelligent · faisceau · etat | `aktiv` · `nicht_aktiv` | *High beam assist (FLA) transition from 32 to 22* |
| `SPIEGEL_BORDSTEIN_AUTOM` | retroviseur · bordure de trottoir · automatique | `aktiv` · `nicht_aktiv` | Le retroviseur droit s'abaisse en marche arriere pour voir le trottoir. |
| `SPIEGEL_EINKLAPPEN` | retroviseur · rabattement | `aktiv` · `nicht_aktiv` | Autorise le rabattement electrique des retroviseurs. |
| `SPIEGEL_ELEKTROCHROM` | retroviseur · electrochrome | `aktiv` · `nicht_aktiv` | Retroviseurs photosensibles (electrochromes). |
| `SPIEGEL_HEIZUNG` | retroviseur · chauffage | `aktiv` · `nicht_aktiv` | *Outside mirrors heating* |
| `SPIEGEL_KOMFORT_EINKLAPP` | retroviseur · confort · rabattement | `aktiv` · `nicht_aktiv` | Rabattement des retroviseurs par la telecommande au verrouillage. |
| `SPIEGEL_MEMORY` | retroviseur · memoire | `aktiv` · `nicht_aktiv` | *Outside mirrors memory* |
| `SPIEGEL_STAND_HEIZUNG` | retroviseur · etat · chauffage | `aktiv` · `nicht_aktiv` | *Outside mirrors heating activated with independent park heating* |
| `SP_GRENZE` | point de passage de rapport · limite | `wert_01` · `wert_02` · `wert_03` | *Undervoltage limit [voltage_V=(valeur*18)/255]* |
| `SP_NACHREG_LANGSAM` | point de passage de rapport · regulation differee · lent | `nicht_aktiv` · `aktiv` | *Slowly follow voltage fluctuations [nicht_aktiv: follow directly, aktiv: follow slowly]* |
| `STANDVERBRAUCHER_ANMELDEN` | consommateur a l'arret · enregistrement | `aktiv` · `nicht_aktiv` |  |
| `SW_NULL_BEREICH` | logiciel · nul · plage | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Headlights dead zone [angle_�=valeur/10]* |
| `TAGFAHRLICHT_ECE` | feux de jour · norme europeenne | `nicht_aktiv` · `aktiv` | *Daytime running lights (Europe)* |
| `TASTVERH_BEL_FLC_LED` | rapport cyclique · eclairage · _FLC_ · LED | `wert_01` · `wert_02` | *Minimum brightness pulse duty factor for automatic light control (FLC) LED (base width)* |
| `TASTVERH_BEL_WBL_MIN` | rapport cyclique · eclairage · clignotant lateral · minimum | `wert_01` · `wert_02` · `wert_03` | *Minimum brightness pulse duty factor for hazard light switch (base width)* |
| `TASTVERH_KL_58G_MIN` | rapport cyclique · borne · borne 58G · minimum | `wert_01` | *Minimum brightness pulse duty factor for instrument lights (base width)* |
| `TAU_ABSCHWAECHUNG` | _TAU_ · attenuation | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+8)* | *Percentage of issued time constants [pourcentage = valeur]* |
| `TIPP_BLK_ABBRUCH` | impulsion · clignotant · interruption | `nicht_aktiv` · `aktiv` | *One-touch turn signal can be aborted* |
| `TIPP_BLK_LOESCHT_IMMER` | impulsion · clignotant · efface · toujours | `nicht_aktiv` · `aktiv` | *One-touch turn signal always resets (turn signals stop after completing the one-touch cycle even when the turn signals were activated before that)* |
| `TUERAUF_STOP_MAUT` | porte ouverte · arret · peage | `aktiv` · `nicht_aktiv` | Interrompt la remontee automatique de vitre (fonction peage) des qu'une portiere est ouverte. |
| `TYP_FH_BEDIENBLOCK` | type · leve-vitre · bloc de commande | `nicht_aktiv` · `aktiv` | *Power windows switch block type* |
| `T_FILTER_DYN_LZ` | T · filtre · dynamique · duree | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Filter time constant for dynamic control [duree ms = valeur x 10]* |
| `T_FILTER_GIERRATE` | T · filtre · vitesse de lacet | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Filter time constant for yaw rate [duree ms = valeur x 10]* |
| `T_FILTER_LENKWINKEL` | T · filtre · angle de braquage | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Filter time constant for steering angle [duree ms = valeur x 10]* |
| `T_FILTER_TAU_LW` | T · filtre · _TAU_ · portee d'eclairage (?) | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Filter time constant for steering angle [duree ms = valeur x 10]* |
| `T_FILTER_UEBERST` | T · filtre · survirage | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Filter time constant for oversteer [duree ms = valeur x 10]* |
| `T_FILTER_V_VEH` | T · filtre · _V_ · vehicule | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Filter time constant for speed-dependent steering ratio [duree ms = valeur x 10]* |
| `T_F_GIERRATE_DEF` | T · F · vitesse de lacet · defaut | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Filter time constant for calculated yaw rate [duree ms = valeur x 10]* |
| `UMSCHALTOPT_LICHTSCHALTER` | option de commutation · commodo d'eclairage | `wert_00` · `wert_01` · `wert_02` · `wert_03` | *Delayed switch-off from light switch position low beams and automatic light control (FLC)* |
| `UNP_HEIMLEUCHTEN_CKM` | _UNP_ · eclairage d'accompagnement · memoire par cle (Car Key Memory) | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` | *Unpersonalized car/key memory for follow-me-home lights (activated by flashing high beams with ignition off) [wert_01: off] [duree s = valeur]* |
| `UNP_QUITT_BLK_ENTSI_CKM` | _UNP_ · acquittement sonore · clignotant · deverrouillage · memoire par cle (Car Key Memory) | `nicht_aktiv` · `aktiv` | *Unpersonalized car/key memory for visual unlocking acknowledge without anti-theft alarm system (two turn signal flashes)* |
| `UNP_QUITT_BLK_SICH_CKM` | _UNP_ · acquittement sonore · clignotant · verrouillage · memoire par cle (Car Key Memory) | `nicht_aktiv` · `aktiv` | *Unpersonalized car/key memory for visual locking acknowledge without anti-theft alarm system (single turn signal flash)* |
| `UNP_TAGFAHRLICHT_ECE_CKM` | _UNP_ · feux de jour · norme europeenne · memoire par cle (Car Key Memory) | `nicht_aktiv` · `aktiv` | *Unpersonalized car/key memory for Europe daytime running lights* |
| `UNP_ZYKL_TIPP_BLK_CKM` | _UNP_ · cycle · impulsion · clignotant · memoire par cle (Car Key Memory) | `wert_01` · `wert_02` | *Unpersonalized car/key memory for one-touch turn signal functionality [blinkcount=valeur]* |
| `USA_REVERSE` | marche americain · inversion | `aktiv` · `nicht_aktiv` | *Power windows jamming protection larger move back (USA behavior)* |
| `U_LWR_POTI_STELLUNG_0` | U · correcteur de site des phares · potentiometre · position · 0 | `wert_01` | *Manual vertical headlight aim control (LWR) position 0 potentiometer voltage [voltage_V=(valeur*5)/255]* |
| `U_LWR_POTI_STELLUNG_1` | U · correcteur de site des phares · potentiometre · position · 1 | `wert_01` | *Manual vertical headlight aim control (LWR) position 1 potentiometer voltage [voltage_V=(valeur*5)/255]* |
| `U_LWR_POTI_STELLUNG_2` | U · correcteur de site des phares · potentiometre · position · 2 | `wert_01` | *Manual vertical headlight aim control (LWR) position 2 potentiometer voltage [voltage_V=(valeur*5)/255]* |
| `U_POTI_DIMM_DELTA` | U · potentiometre · gradation · ecart | `wert_01` | *Instrument lights dimmer potentiometer voltage span between minimum and maximum [voltage_V=(valeur*5)/255]* |
| `U_POTI_DIMM_MIN` | U · potentiometre · gradation · minimum | `wert_01` | *Instrument lights dimmer potentiometer minimum voltage [voltage_V=(valeur*5)/255]* |
| `VIER_TUERER` | quatre · portes | `nicht_aktiv` · `aktiv` | *4 doors* |
| `VORFELD_BEL_GEDIMMT` | zone avant · eclairage · attenue | `aktiv` · `nicht_aktiv` |  |
| `V_HI_ABSCHW_D_TAU` | _V_ · haut · attenuation · D · _TAU_ | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Maximum speed for beginning reduction of long-term filter time constant [vitesse km/h = valeur]* |
| `V_HI_AENDER_LENK_UEBERS` | _V_ · haut · modification · direction · vue d'ensemble (?) | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Maximum speed for steering ratio change [vitesse km/h = valeur]* |
| `V_HI_REDUZIER_V_SCHWENK` | _V_ · haut · reduction · _V_ · pivotement | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Maximum speed for reduction of headlights swivelling speed [vitesse km/h = valeur]* |
| `V_HI_RED_SCHWENK_W` | _V_ · haut · reduction · pivotement · W | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Maximum speed for beginning reduction of headlights swivelling angle [vitesse km/h = valeur]* |
| `V_HI_SW_MITLENKEN` | _V_ · haut · logiciel · suivi de direction | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Maximum speed for headlights straight-ahead [vitesse km/h = valeur]* |
| `V_HI_SW_NACH_INNEN` | _V_ · haut · logiciel · apres · interieur | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Maximum speed for headlights beam convergence [vitesse km/h = valeur]* |
| `V_HI_TAU_GIERRATE` | _V_ · haut · _TAU_ · vitesse de lacet | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Maximum speed for full incalculation of yaw rate [vitesse km/h = valeur]* |
| `V_LO_ABSCHW_D_TAU` | _V_ · bas · attenuation · D · _TAU_ | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Minimum speed for beginning reduction of long-term filter time constant [vitesse km/h = valeur]* |
| `V_LO_AENDER_LENK_UEBERS` | _V_ · bas · modification · direction · vue d'ensemble (?) | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Minimum speed for steering ratio change [vitesse km/h = valeur]* |
| `V_LO_REDUZIER_V_SCHWENK` | _V_ · bas · reduction · _V_ · pivotement | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Minimum speed for reduction of headlights swivelling speed [vitesse km/h = valeur]* |
| `V_LO_RED_SCHWENK_W` | _V_ · bas · reduction · pivotement · W | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Minimum speed for beginning reduction of headlights swivelling angle [vitesse km/h = valeur]* |
| `V_LO_SW_MITLENKEN` | _V_ · bas · logiciel · suivi de direction | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Minimum speed for headlights straight-ahead [vitesse km/h = valeur]* |
| `V_LO_SW_NACH_INNEN` | _V_ · bas · logiciel · apres · interieur | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Minimum speed for headlights beam convergence [vitesse km/h = valeur]* |
| `V_LO_TAU_GIERRATE` | _V_ · bas · _TAU_ · vitesse de lacet | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Minimum speed to start incalculating yaw rate [vitesse km/h = valeur]* |
| `V_M_HI_REDUZIER_V_SCHWENK` | _V_ · M · haut · reduction · _V_ · pivotement | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Medium-maximum speed for reduction of headlights swivelling speed [vitesse km/h = valeur]* |
| `V_M_LO_REDUZIER_V_SCHWENK` | _V_ · M · bas · reduction · _V_ · pivotement | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Medium-minimum speed for reduction of headlights swivelling speed [vitesse km/h = valeur]* |
| `V_REFERENZLAUF_EN` | _V_ · cycle de reference · active | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Maximum speed for stepper motor reference run [vitesse km/h = valeur]* |
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
| `WARMUEBERW_RL_BL_1_L` | surveillance a chaud · feu arriere · feu stop · 1 · _L_ | `nicht_aktiv` · `aktiv` | *Hot monitoring of left outer tail light* |
| `WARMUEBERW_RL_BL_1_R` | surveillance a chaud · feu arriere · feu stop · 1 · _R_ | `nicht_aktiv` · `aktiv` | *Hot monitoring of right outer tail light* |
| `WARMUEBERW_RL_BL_2_L` | surveillance a chaud · feu arriere · feu stop · 2 · _L_ | `nicht_aktiv` · `aktiv` | *Hot monitoring of left inner tail light* |
| `WARMUEBERW_RL_BL_2_R` | surveillance a chaud · feu arriere · feu stop · 2 · _R_ | `nicht_aktiv` · `aktiv` | *Hot monitoring of right inner tail light* |
| `WARNBLK_ENERGY_SAVE_MODE` | feux de detresse · energie · economie · mode | `nicht_aktiv` · `aktiv` | *Hazard lights energy saving mode* |
| `WERT_CRASHVERHALTEN` | valeur · comportement en cas de choc | `wert_01` |  |
| `XENON_AL_VERBAUT` | xenon · feux de croisement · installe | `nicht_aktiv` · `aktiv` | *Xenon low beams installed* |
| `XENON_LI` | xenon · gauche | `wert_01` · `wert_02` | *Xenon lights* |
| `XENON_WIEDERHOL_Z_OPT` | xenon · repetition · Z · option | `nicht_aktiv` · `aktiv` | *Automatically re-ignite xenon light if failed to start* |
| `XENON_WIEDER_ZUEND` | xenon · a nouveau · allumage | `nicht_aktiv` · `aktiv` | *Automatically re-ignite xenon light if failed to start* |
| `XENON_WIEDER_Z_AUS_PHASE` | xenon · a nouveau · Z · arret · phase | `wert_01` | *Waiting time between xenon light ignition attempts [time_ms=(valeur*300)+600]* |
| `ZEITK_TRIGGER1` | constante de temps · declencheur 1 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+7)* | *Time constant for steering wheel movement trigger* |
| `ZUWACHS_TASTVERH_FLC_LED` | increment · rapport cyclique · _FLC_ · LED | `wert_01` · `wert_02` | *Maximum brightness pulse duty factor for automatic light control (FLC) LED (added to base width)* |
| `ZUWACHS_TASTVERH_KL_58G` | increment · rapport cyclique · borne · borne 58G | `wert_01` | *Maximum brightness pulse duty factor for instrument lights (added to base width)* |
| `ZUWACHS_TASTVERH_WBL_BEL` | increment · rapport cyclique · clignotant lateral · eclairage | `wert_01` · `wert_02` | *Maximum brightness pulse duty factor for hazard light switch (added to base width)* |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Signification** — texte en clair. En romain : rédigé à la main. *En italique* : description
de la base communautaire NCS Dummy, en anglais, non encore traduite.
Les notations d'unités (`[durée s = valeur / 10]`) sont traduites.
