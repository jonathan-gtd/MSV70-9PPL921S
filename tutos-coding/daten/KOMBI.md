# KOMBI — Combine

🟢 Module `KOMBI` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟢 Module sans fonction de securite. Sauvegarde du `.TRC` quand meme.

---

## Identification

| | |
|---|---|
| Variante | **`KMBI_PL2.C08`** |
| Identifiée par | **lecture NCS Expert sur le véhicule** |
| Fonctions retenues | **295** |
| Vérification | 100 % des fonctions dans les blocs du `.TRC` |
| Avec valeurs | 295 |
| Signification (EN) | 184 |
| Explication (FR) | 21 |
| Blocs d'adresses du TRC | 16 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 295 fonctions codables

| FSW | Décomposition | Valeurs | Signification (EN) | Explication (FR) |
|---|---|---|---|---|
| `ACC_ALIVE_ZAEHLER` | regulateur adaptatif · signal de presence · compteur | `nicht_aktiv` · `aktiv` | Alive counter for active cruise control (ACC) | — |
| `ACC_AUSSTATTUNG` | regulateur adaptatif · equipement | `wert_01` · `wert_02` · `wert_03` · `wert_04` | Cruise control installed | — |
| `ACC_CSUM_MONITOR` | regulateur adaptatif · somme de controle · surveillance | `nicht_aktiv` · `aktiv` | Checksum monitoring for active cruise control (ACC) | — |
| `ACC_FILTER_FALLEND` | regulateur adaptatif · filtre · decroissant | `wert_01` | Cruise control damping when decreasing | — |
| `ACC_FILTER_STEIGEND` | regulateur adaptatif · filtre · croissant | `wert_01` | Cruise control damping when increasing | — |
| `ACC_ID_MONITOR` | regulateur adaptatif · identifiant · surveillance | `nicht_aktiv` · `aktiv` | Identification timeout for active cruise control (ACC) | — |
| `ACC_ZEIGER_AUF_NULL` | regulateur adaptatif · aiguille · ouverture · nul | `nicht_aktiv` · `aktiv` | Cruise control steering column stalk always returns to center position | — |
| `AFS_ALIVE_ZAEHLER` | phares adaptatifs (AFS) · signal de presence · compteur | `nicht_aktiv` · `aktiv` | Alive counter for active front steering (AFS) | — |
| `AFS_ID_MONITOR` | phares adaptatifs (AFS) · identifiant · surveillance | `nicht_aktiv` · `aktiv` | Identification timeout for active front steering (AFS) | — |
| `AHM_ALIVE` | module d'attelage · signal de presence | `nicht_aktiv` · `aktiv` | Alive monitoring for trailer module | — |
| `ARS_ALIVE_ZAEHLER` | stabilisation active · signal de presence · compteur | `nicht_aktiv` · `aktiv` | Alive counter for active roll stabilization (ARS) | — |
| `ARS_ID_MONITOR` | stabilisation active · identifiant · surveillance | `nicht_aktiv` · `aktiv` | Identification timeout for active roll stabilization (ARS) | — |
| `AUSSENTEMP_ANZ_FEHLER_1` | temperature exterieure · nombre · erreur · 1 | `wert_01` · `wert_02` · `wert_03` | Outside temperature display fault 1 (lower limit) | — |
| `AUSSENTEMP_ANZ_FEHLER_2` | temperature exterieure · nombre · erreur · 2 | `wert_01` | Outside temperature display fault 2 (upper limit) | — |
| `AUSSENTEMP_DELTA_K1` | temperature exterieure · ecart · canal 1 (?) | `wert_01` | Outside temperature delta K1 | — |
| `AUSSENTEMP_DELTA_K2` | temperature exterieure · ecart · canal 2 (?) | `wert_01` | Outside temperature delta K2 | — |
| `AUSSENTEMP_SPERR_FAKTOR` | temperature exterieure · blocage · facteur | `wert_01` | Outside temperature lock time factor | — |
| `AUSSENTEMP_SPERR_OFFSET` | temperature exterieure · blocage · decalage | `wert_01` | Outside temperature lock time offset | — |
| `BC_2_ENABLE` | ordinateur de bord · 2 · activation | `nicht_aktiv` · `aktiv` | On-board computer (OBC) 2 enabled | — |
| `BC_DIGITAL_V` | ordinateur de bord · numerique · _V_ | `nicht_aktiv` · `aktiv` | On-board computer (OBC) digital speed display | Affichage de la vitesse en chiffres sur l'ordinateur de bord. |
| `BC_DIGITAL_V_KORREKTUR` | ordinateur de bord · numerique · _V_ · correction | `nicht_aktiv` · `aktiv` | On-board computer (OBC) digital speed display correction [aktiv: corrected speed, nicht_aktiv: true speed] | Correction optimiste du compteur. Desactiver pour afficher la vitesse reelle. |
| `BC_DIG_GESCHW_BEI_KL_R` | ordinateur de bord · numerique · vitesse · lors de · borne · _R_ | `nicht_aktiv` · `aktiv` | — | — |
| `BC_LEERZEILE` | ordinateur de bord · ligne vide | `nicht_aktiv` · `aktiv` | On-board computer (OBC) include blank information display when cycling through the information displays with BC switch on turn signal stalk | — |
| `BC_LIMIT_ENABLE` | ordinateur de bord · limite · activation | `nicht_aktiv` · `aktiv` | On-board computer (OBC) speed limit enabled | — |
| `BC_V_KORREKTUR` | ordinateur de bord · _V_ · correction | `nicht_aktiv` · `aktiv` | On-board computer (OBC) speed correction [aktiv: corrected speed, nicht_aktiv: true speed] | — |
| `C08_TANK_SCHWELLE_RESERVE` | [variante C08] · reservoir · seuil · reserve | `wert_01` · `wert_02` · `wert_03` · `wert_04` | Fuel tank reserve threshold | — |
| `CAS_ALIVE` | systeme d'acces · signal de presence | `nicht_aktiv` · `aktiv` | Alive monitoring for car access system (CAS) | — |
| `CBS_AVKM_START` | entretien selon besoin (CBS) · module AV (?) · demarrage | `wert_01` | Condition based service (CBS) starting value for average number of kilometers per week [km/week=data*40] | — |
| `CBS_EINH_01H` | entretien selon besoin (CBS) · unite · [0x01] | `prozent` · `wegbas` · `zeitbas` · `wegzeit` | Condition based service (CBS) unit for engine oil | — |
| `CBS_EINH_02H` | entretien selon besoin (CBS) · unite · [0x02] | `prozent` · `wegbas` · `zeitbas` · `wegzeit` | Condition based service (CBS) unit for front brake pad wear | — |
| `CBS_EINH_03H` | entretien selon besoin (CBS) · unite · [0x03] | `prozent` · `wegbas` · `zeitbas` | Condition based service (CBS) unit for brake fluid | — |
| `CBS_EINH_04H` | entretien selon besoin (CBS) · unite · [0x04] | `prozent` · `wegbas` · `zeitbas` · `wegzeit` | Condition based service (CBS) unit for microfilter | — |
| `CBS_EINH_06H` | entretien selon besoin (CBS) · unite · [0x06] | `prozent` · `wegbas` · `zeitbas` | Condition based service (CBS) unit for rear brake pad wear | — |
| `CBS_EINH_07H` | entretien selon besoin (CBS) · unite · [0x07] | `prozent` · `wegbas` · `zeitbas` | Condition based service (CBS) unit for diesel particle filter | — |
| `CBS_EINH_0AH` | entretien selon besoin (CBS) · unite · [0x0A] | `prozent` · `wegbas` · `zeitbas` · `wegzeit` | — | — |
| `CBS_EINH_10H` | entretien selon besoin (CBS) · unite · [0x10] | `prozent` · `wegbas` · `zeitbas` | Condition based service (CBS) unit for spark plugs | — |
| `CBS_EINH_11H` | entretien selon besoin (CBS) · unite · [0x11] | `prozent` · `wegbas` · `zeitbas` · `wegzeit` | Condition based service (CBS) unit for vehicle inspection | — |
| `CBS_EINH_12H` | entretien selon besoin (CBS) · unite · [0x12] | `prozent` · `wegbas` · `zeitbas` | — | — |
| `CBS_EINH_13H` | entretien selon besoin (CBS) · unite · [0x13] | `prozent` · `wegbas` · `zeitbas` · `wegzeit` | — | — |
| `CBS_EINH_14H` | entretien selon besoin (CBS) · unite · [0x14] | `prozent` · `wegbas` · `zeitbas` · `wegzeit` | Condition based service (CBS) unit for pre-delivery inspection | — |
| `CBS_EINH_20H` | entretien selon besoin (CBS) · unite · [0x20] | `prozent` · `wegbas` · `zeitbas` | Condition based service (CBS) unit for statutory vehicle inspection (MOT) | — |
| `CBS_EINH_21H` | entretien selon besoin (CBS) · unite · [0x21] | `prozent` · `wegbas` · `zeitbas` | Condition based service (CBS) unit for emission inspection (smog test) | — |
| `CBS_GELB` | entretien selon besoin (CBS) · jaune | `wert_01` · `wert_02` | Condition based service (CBS) display time for yellow warning [weeks=data] | — |
| `CBS_GELB_HUAU` | entretien selon besoin (CBS) · jaune · sortie (?) | `wert_01` | Condition based service (CBS) display time for statutory vehicle inspection (MOT) / emission inspection (smog test) [months=data] | — |
| `CBS_GELB_KM_MIN` | entretien selon besoin (CBS) · jaune · kilometres · minimum | `wert_01` | Condition based service (CBS) display distance for yellow warning [distance_km=data*100] | — |
| `CBS_INTD_03H` | entretien selon besoin (CBS) · intervalle d'essuyage · [0x03] | `wert_01` | Condition based service (CBS) interval time for brake fluid [months=data] | — |
| `CBS_INTD_11H` | entretien selon besoin (CBS) · intervalle d'essuyage · [0x11] | `wert_01` · `wert_02` · `wert_03` | — | — |
| `CBS_INTD_13H` | entretien selon besoin (CBS) · intervalle d'essuyage · [0x13] | `wert_01` | — | — |
| `CBS_INTD_14H` | entretien selon besoin (CBS) · intervalle d'essuyage · [0x14] | `wert_01` | Condition based service (CBS) interval time for pre-delivery inspection [months=data] | — |
| `CBS_INTW_10H` | entretien selon besoin (CBS) · intervalle d'essuyage · [0x10] | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` | Condition based service (CBS) interval distance for spark plugs [distance_km=data*1000] | — |
| `CBS_INTW_11H` | entretien selon besoin (CBS) · intervalle d'essuyage · [0x11] | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` | Condition based service (CBS) interval distance for vehicle check [distance_km=data*1000] | — |
| `CBS_INTW_12H` | entretien selon besoin (CBS) · intervalle d'essuyage · [0x12] | `wert_01` | — | — |
| `CBS_RES_SPERR` | entretien selon besoin (CBS) · remise a zero · blocage | `wert_01` | Condition based service (CBS) reset lock [percentage_%=data] | — |
| `CBS_SCEN_01H` | entretien selon besoin (CBS) · scenario · [0x01] | `aktiv` · `nicht_aktiv` | Condition based service (CBS) telematics service call for engine oil | — |
| `CBS_SCEN_02H` | entretien selon besoin (CBS) · scenario · [0x02] | `aktiv` · `nicht_aktiv` | Condition based service (CBS) telematics service call for front brake pad wear | — |
| `CBS_SCEN_03H` | entretien selon besoin (CBS) · scenario · [0x03] | `aktiv` · `nicht_aktiv` | Condition based service (CBS) telematics service call for brake fluid | — |
| `CBS_SCEN_04H` | entretien selon besoin (CBS) · scenario · [0x04] | `aktiv` · `nicht_aktiv` | Condition based service (CBS) telematics service call for microfilter | — |
| `CBS_SCEN_06H` | entretien selon besoin (CBS) · scenario · [0x06] | `aktiv` · `nicht_aktiv` | Condition based service (CBS) telematics service call for rear brake pad wear | — |
| `CBS_SCEN_07H` | entretien selon besoin (CBS) · scenario · [0x07] | `aktiv` · `nicht_aktiv` | Condition based service (CBS) telematics service call for diesel particle filter | — |
| `CBS_SCEN_0AH` | entretien selon besoin (CBS) · scenario · [0x0A] | `aktiv` · `nicht_aktiv` | — | — |
| `CBS_SCEN_10H` | entretien selon besoin (CBS) · scenario · [0x10] | `aktiv` · `nicht_aktiv` | Condition based service (CBS) telematics service call for spark plugs | — |
| `CBS_SCEN_11H` | entretien selon besoin (CBS) · scenario · [0x11] | `aktiv` · `nicht_aktiv` | Condition based service (CBS) telematics service call for vehicle check | — |
| `CBS_SCEN_12H` | entretien selon besoin (CBS) · scenario · [0x12] | `aktiv` · `nicht_aktiv` | — | — |
| `CBS_SCEN_13H` | entretien selon besoin (CBS) · scenario · [0x13] | `aktiv` · `nicht_aktiv` | — | — |
| `CBS_SCEN_14H` | entretien selon besoin (CBS) · scenario · [0x14] | `aktiv` · `nicht_aktiv` | Condition based service (CBS) telematics service call for pre-delivery inspection | — |
| `CBS_SCEN_20H` | entretien selon besoin (CBS) · scenario · [0x20] | `aktiv` · `nicht_aktiv` | Condition based service (CBS) telematics service call for statutory vehicle inspection (MOT) | — |
| `CBS_SCEN_21H` | entretien selon besoin (CBS) · scenario · [0x21] | `aktiv` · `nicht_aktiv` | Condition based service (CBS) telematics service call for emission inspection (smog test) | — |
| `CBS_STAT_01H` | entretien selon besoin (CBS) · etat · [0x01] | `anzeige` · `sperre` · `erprobung` | Condition based service (CBS) status for engine oil | — |
| `CBS_STAT_02H` | entretien selon besoin (CBS) · etat · [0x02] | `anzeige` · `sperre` · `erprobung` | Condition based service (CBS) status for front brake pad wear | — |
| `CBS_STAT_03H` | entretien selon besoin (CBS) · etat · [0x03] | `anzeige` · `sperre` · `erprobung` | Condition based service (CBS) status for brake fluid | — |
| `CBS_STAT_04H` | entretien selon besoin (CBS) · etat · [0x04] | `anzeige` · `sperre` · `erprobung` | Condition based service (CBS) status for microfilter | — |
| `CBS_STAT_06H` | entretien selon besoin (CBS) · etat · [0x06] | `anzeige` · `sperre` · `erprobung` | Condition based service (CBS) status for rear brake pad wear | — |
| `CBS_STAT_07H` | entretien selon besoin (CBS) · etat · [0x07] | `anzeige` · `sperre` · `erprobung` | Condition based service (CBS) status for diesel particle filter | — |
| `CBS_STAT_0AH` | entretien selon besoin (CBS) · etat · [0x0A] | `anzeige` · `sperre` · `erprobung` | — | — |
| `CBS_STAT_10H` | entretien selon besoin (CBS) · etat · [0x10] | `anzeige` · `sperre` · `erprobung` | Condition based service (CBS) status for spark plugs | — |
| `CBS_STAT_11H` | entretien selon besoin (CBS) · etat · [0x11] | `anzeige` · `sperre` · `erprobung` | Condition based service (CBS) status for vehicle check | — |
| `CBS_STAT_12H` | entretien selon besoin (CBS) · etat · [0x12] | `anzeige` · `sperre` · `erprobung` | — | — |
| `CBS_STAT_13H` | entretien selon besoin (CBS) · etat · [0x13] | `anzeige` · `sperre` · `erprobung` | — | — |
| `CBS_STAT_14H` | entretien selon besoin (CBS) · etat · [0x14] | `anzeige` · `sperre` · `erprobung` | Condition based service (CBS) status for pre-delivery inspection | — |
| `CBS_STAT_20H` | entretien selon besoin (CBS) · etat · [0x20] | `anzeige` · `sperre` · `erprobung` | Condition based service (CBS) status for statutory vehicle inspection (MOT) | — |
| `CBS_STAT_21H` | entretien selon besoin (CBS) · etat · [0x21] | `anzeige` · `sperre` · `erprobung` | Condition based service (CBS) status for emission inspection (smog test) | — |
| `CBS_TZ_SCHALTER` | entretien selon besoin (CBS) · duree (?) · interrupteur | `tagz` · `borddatum` | Condition based service (CBS) source for days counter | — |
| `CC_15_AUS` | Check Control · 15 · arret | `wert_01` | Check control synchronization phase after switching off ignition [time_ms=data*10] | — |
| `CC_15_EIN` | Check Control · 15 · marche | `benzin` · `diesel` | Check control synchronization phase after switching on ignition [time_ms=data*10] | — |
| `CC_BLINKFREQUENZ_LANGSAM` | Check Control · frequence de clignotement · lent | `wert_01` | Check control slow flashing frequency [frequency_Hz=10/data] | — |
| `CC_BLINKFREQUENZ_SCHNELL` | Check Control · frequence de clignotement · rapide | `wert_01` | Check control fast flashing frequency [frequency_Hz=10/data] | — |
| `CC_BREMSWARNL_NUR_ROT` | Check Control · temoin de frein · uniquement · rouge | `nicht_aktiv` · `aktiv` | Check control notiofication for brakes only when red (high priority) | — |
| `CC_CSUM` | Check Control · somme de controle | `grundcode_var` · `wert_01` · `wert_02` · `wert_03` · `wert_10` · `wert_04` | Checksum check control | — |
| `CC_ML` | Check Control · module (?) | `wert_01` | Check control synchronization phase after staring engine [time_ms=data*10] | — |
| `CC_NR_ROLLE` | Check Control · numero · rouleau | `nicht_aktiv` · `aktiv` | — | — |
| `CC_R_AUS` | Check Control · _R_ · arret | `wert_01` | Check control synchronization phase after switching off accessories (ignition key position 1) [time_ms=data*10] | — |
| `CC_R_EIN` | Check Control · _R_ · marche | `wert_01` | Check control synchronization phase after switching on accessories (ignition key position 1) [time_ms=data*10] | — |
| `CC_TEMPERATURWARNUNG` | Check Control · alerte de temperature | `nicht_aktiv` · `aktiv` | Check control notification for temperature warning | — |
| `CC_ZUENDSCHLUESSEL` | Check Control · cle de contact | `nicht_aktiv` · `aktiv` | Check control notification for key left in ignition warning when opening door | Alerte de portiere ouverte lorsque la cle est en contact. |
| `CODING_VALID_BMW` | codage · valide · BMW | `wert_01` | Coding valid BMW | — |
| `COMBOX_ALIVE_MONITOR` | module telematique · signal de presence · surveillance | `nicht_aktiv` · `aktiv` | — | — |
| `COMBOX_ID_MONITOR` | module telematique · identifiant · surveillance | `nicht_aktiv` · `aktiv` | — | — |
| `CRASH_TH_DIMMUNG` | choc · seuil (?) · gradation | `wert_01` | — | — |
| `CSUM_ACC` | somme de controle · regulateur adaptatif | `grundcode_var` | Checksum active cruise control (ACC) | — |
| `CSUM_AKUSTIK_KLANG` | somme de controle · acoustique · sonorite | `grundcode_var` · `wert_01` · `wert_02` | Checksum acoustic sound | — |
| `CSUM_ATEMP` | somme de controle · temperature exterieure | `grundcode_var` | Checksum outside temperature | — |
| `CSUM_AUSSTATTUNG_1` | somme de controle · equipement · 1 | `grundcode_var` | Checksum equipment 1 | — |
| `CSUM_AUSSTATTUNG_PL2` | somme de controle · equipement · plateforme 2 | `grundcode_var` | Checksum equipment | — |
| `CSUM_CBS_01` | somme de controle · entretien selon besoin (CBS) · 01 | `grundcode_var` | Checksum condition based service (CBS) 01 | — |
| `CSUM_CC_ALIVE` | somme de controle · Check Control · signal de presence | `grundcode_var` | Checksum check control alive monitoring | — |
| `CSUM_KODIERUNG_VALID` | somme de controle · codage · valide | `grundcode_var` | Checksum coding valid | — |
| `CSUM_ST_ANZEIGE_1` | somme de controle · commande (?) · affichage · 1 | `grundcode_var` | Checksum control display 1 | — |
| `CSUM_TANK_INPUT` | somme de controle · reservoir · entree | `grundcode_var` | Checksum fuel tank input | — |
| `CSUM_TANK_PROZESS` | somme de controle · reservoir · processus | `grundcode_var` | Checksum fuel tank processes | — |
| `CSUM_WEG` | somme de controle · course | `grundcode_var` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` | Checksum distance | — |
| `CVS_ALIVE_ZAEHLER` | systeme de commande (?) · signal de presence · compteur | `nicht_aktiv` · `aktiv` | Alive counter for convertible top | — |
| `CVS_ID_MONITOR` | systeme de commande (?) · identifiant · surveillance | `nicht_aktiv` · `aktiv` | Identification timeout for convertible top | — |
| `DIGITAL_KVA` | numerique · valeur de consigne (?) | `nicht_aktiv` · `aktiv` | On-board computer (OBC) momentary fuel consumption | — |
| `DIMM_MN_MAX` | gradation · principal (?) · maximum | `wert_01` | — | — |
| `DIMM_MN_MIN` | gradation · principal (?) · minimum | `wert_01` | — | — |
| `DISP_ENGDAT_ALIVE_ZAEHLER` | affichage · donnees moteur · signal de presence · compteur | `nicht_aktiv` · `aktiv` | Alive counter for engine data display | — |
| `DISP_ENGDAT_ID_MONITOR` | affichage · donnees moteur · identifiant · surveillance | `nicht_aktiv` · `aktiv` | Identification timeout for engine data display | — |
| `DME_ALIVE_ZAEHLER` | calculateur moteur · signal de presence · compteur | `nicht_aktiv` · `aktiv` | Alive counter for digital motor electronics (DME) | — |
| `DME_ID_MONITOR` | calculateur moteur · identifiant · surveillance | `nicht_aktiv` · `aktiv` | Identification timeout for digital motor electronics (DME) | — |
| `DSC_ALIVE_ZAEHLER` | controle de stabilite · signal de presence · compteur | `nicht_aktiv` · `aktiv` | Alive counter for dynamic stability control (DSC) | — |
| `DSC_ALLRAD_CC_WAHL` | controle de stabilite · transmission integrale · Check Control · selection | `nicht_aktiv` · `aktiv` | Check control for dynamic stability control (DSC) xDrive [nicht_aktiv: rear-wheel drive, aktiv: 4-wheel drive] | — |
| `DSC_ID_MONITOR` | controle de stabilite · identifiant · surveillance | `nicht_aktiv` · `aktiv` | Identification timeout for dynamic stability control (DSC) | — |
| `DSC_MIT_RPA_CC` | controle de stabilite · avec · indicateur de crevaison · Check Control | `nicht_aktiv` · `aktiv` | — | — |
| `DSC_VARIANTE` | controle de stabilite · variante | `wert_01` · `wert_02` | — | — |
| `DWA_VERBAUT` | alarme antivol · installe | `nicht_aktiv` · `aktiv` | Antitheft alarm system (DWA) installed | — |
| `DZM_DISP_RPM` | compte-tours · affichage · regime moteur | `wert_01` · `wert_02` | Tachometer (RPM) signal source | — |
| `DZM_UMRECHNUNG` | compte-tours · conversion | `wert_01` · `wert_02` · `wert_03` | — | — |
| `E89_FDS_COMFORT` | serie E89 · _FDS_ · confort | `wert_01` · `wert_02` | — | — |
| `E92_E93_FZG` | serie E92 · serie E93 · vehicule | `nicht_aktiv` · `aktiv` | — | — |
| `EDCK_ALIVE_ZAEHLER` | amortissement pilote (?) · signal de presence · compteur | `nicht_aktiv` · `aktiv` | Alive counter for electronic damper control (EDCK) | — |
| `EDCK_ID_MONITOR` | amortissement pilote (?) · identifiant · surveillance | `nicht_aktiv` · `aktiv` | Identification timeout for electronic damper control (EDCK) | — |
| `EGS_ALIVE_ZAEHLER` | calculateur de boite · signal de presence · compteur | `nicht_aktiv` · `aktiv` | Alive counter for electronic transmission control (EGS) | — |
| `EGS_CSUM_MONITOR` | calculateur de boite · somme de controle · surveillance | `nicht_aktiv` · `aktiv` | Checksum monitoring for electronic transmission control (EGS) | — |
| `EGS_ID_MONITOR` | calculateur de boite · identifiant · surveillance | `nicht_aktiv` · `aktiv` | Identification timeout for electronic transmission control (EGS) | — |
| `EHC_ALIVE` | regulation de hauteur · signal de presence | `nicht_aktiv` · `aktiv` | Alive monitoring for electronic height control (EHC) | — |
| `EINST_HILFE` | reglage · aide | `nicht_aktiv` · `aktiv` | Easy entry | — |
| `EKP_ALIVE_ZAEHLER` | pompe a carburant · signal de presence · compteur | `nicht_aktiv` · `aktiv` | Alive counter for electronically controlled fuel pump (EKP) | — |
| `EKP_ID_MONITOR` | pompe a carburant · identifiant · surveillance | `nicht_aktiv` · `aktiv` | Identification timeout for electronically controlled fuel pump (EKP) | — |
| `EMF_ALIVE_ZAEHLER` | frein de stationnement electromecanique · signal de presence · compteur | `nicht_aktiv` · `aktiv` | Alive counter for electronic parking brake | — |
| `EMF_ID_MONITOR` | frein de stationnement electromecanique · identifiant · surveillance | `nicht_aktiv` · `aktiv` | Identification timeout for electronic parking brake | — |
| `EPS_ALIVE_MONITOR` | direction assistee · signal de presence · surveillance | `nicht_aktiv` · `aktiv` | — | — |
| `EPS_ID_MONITOR` | direction assistee · identifiant · surveillance | `nicht_aktiv` · `aktiv` | — | — |
| `FB_ALLE_TUEREN` | telecommande · tous · portes | `nicht_aktiv` · `aktiv` | Pressing remote control unlock button once unlocks all doors | Une seule pression sur le bouton de deverrouillage ouvre toutes les portes |
| `FB_FA_TUER` | telecommande · conducteur · porte | `nicht_aktiv` · `aktiv` | Pressing remote control unlock button once unlocks only driver door, twice unlocks all doors | Une pression sur le bouton de deverrouillage n'ouvre que la porte conducteur, deux pressions ouvrent toutes les portes |
| `FDC_ID_MONITOR` | commande de conduite (?) · identifiant · surveillance | `nicht_aktiv` · `aktiv` | Identification timeout for dynamic driving control (FDC) | — |
| `FDS_ALIVE_ZAEHLER` | _FDS_ · signal de presence · compteur | `nicht_aktiv` · `aktiv` | — | — |
| `FDS_CSUM_MONITOR` | _FDS_ · somme de controle · surveillance | `nicht_aktiv` · `aktiv` | — | — |
| `FDS_ENABLE` | _FDS_ · activation | `nicht_aktiv` · `aktiv` | — | — |
| `FDS_ID_MONITOR` | _FDS_ · identifiant · surveillance | `nicht_aktiv` · `aktiv` | — | — |
| `FDS_TIME_OUT` | _FDS_ · duree · sortie | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+3)* | — | — |
| `FLA_VERBAUT` | assistant feux de route · installe | `nicht_aktiv` · `aktiv` | — | — |
| `GETRIEBE_ART` | boite de vitesses · type | `automatik` · `ssg` · `handschalter` | Transmission type | Type de boite de vitesses |
| `GPS_UHR` | GPS · heure | `nicht_aktiv` · `aktiv` | Use time from GPS | — |
| `HDC_ID_MONITOR` | assistance en descente · identifiant · surveillance | `nicht_aktiv` · `aktiv` | Identification timeout for hill descent control (HDC) | — |
| `HDC_VERBAUT` | assistance en descente · installe | `nicht_aktiv` · `aktiv` | Hill descent control (HDC) installed | — |
| `HEIMLEUCHTEN_10S` | eclairage d'accompagnement · 10 s | `nicht_aktiv` · `aktiv` | Follow-me-home lights 10 seconds | — |
| `HEIMLEUCHTEN_150S` | eclairage d'accompagnement · 150 s | `nicht_aktiv` · `aktiv` | Follow-me-home lights 150 seconds | Eclairage d'accompagnement 150 secondes |
| `HEIMLEUCHTEN_240S` | eclairage d'accompagnement · 240 s | `nicht_aktiv` · `aktiv` | Follow-me-home lights 240 seconds | Eclairage d'accompagnement 240 secondes |
| `HEIMLEUCHTEN_40S` | eclairage d'accompagnement · 40 s | `nicht_aktiv` · `aktiv` | Follow-me-home lights 40 seconds | Eclairage d'accompagnement 40 secondes |
| `HEIMLEUCHTEN_90S` | eclairage d'accompagnement · 90 s | `nicht_aktiv` · `aktiv` | Follow-me-home lights 90 seconds | Eclairage d'accompagnement 90 secondes |
| `HEIMLEUCHTEN_AUS` | eclairage d'accompagnement · arret | `nicht_aktiv` · `aktiv` | Follow-me-home lights off | Eclairage d'accompagnement desactive |
| `HU_VERBAUT_C08` | controle technique · installe · [variante C08] | `radiostufe_1` · `radiostufe_2` · `radiostufe_3_4` | — | — |
| `K410D_C_WEG_1` | [identifiant BMW K410D] · C · course · 1 | `wert_01` · `wert_02` | — | — |
| `K410D_C_WEG_2` | [identifiant BMW K410D] · C · course · 2 | `wert_01` · `wert_02` | — | — |
| `K410D_C_WEG_3` | [identifiant BMW K410D] · C · course · 3 | `wert_01` · `wert_02` | — | — |
| `K410D_C_WEG_4` | [identifiant BMW K410D] · C · course · 4 | `wert_01` · `wert_02` | — | — |
| `K410D_D_WEG` | [identifiant BMW K410D] · D · course | `wert_01` · `wert_02` | — | — |
| `K4110_C_VERBRAUCH_1` | [identifiant BMW K4110] · C · consommation · 1 | `wert_01` · `wert_02` · `wert_03` · `wert_04` | — | — |
| `K4110_C_VERBRAUCH_2` | [identifiant BMW K4110] · C · consommation · 2 | `wert_01` · `wert_02` · `wert_03` · `wert_04` | — | — |
| `K4110_C_VERBRAUCH_3` | [identifiant BMW K4110] · C · consommation · 3 | `wert_01` · `wert_02` · `wert_03` · `wert_04` | — | — |
| `K4110_C_VERBRAUCH_4` | [identifiant BMW K4110] · C · consommation · 4 | `wert_01` · `wert_02` · `wert_03` · `wert_04` | — | — |
| `K4110_D_VERBRAUCH` | [identifiant BMW K4110] · D · consommation | `wert_01` · `wert_02` · `wert_03` · `wert_04` | — | — |
| `K4113_C_ZEIT_1` | [identifiant BMW K4113] · C · duree · 1 | `wert_01` · `wert_02` | — | — |
| `K4113_C_ZEIT_2` | [identifiant BMW K4113] · C · duree · 2 | `wert_01` · `wert_02` | — | — |
| `K4113_C_ZEIT_3` | [identifiant BMW K4113] · C · duree · 3 | `wert_01` · `wert_02` | — | — |
| `K4113_C_ZEIT_4` | [identifiant BMW K4113] · C · duree · 4 | `wert_01` · `wert_02` | — | — |
| `K4113_D_ZEIT` | [identifiant BMW K4113] · D · duree | `wert_01` · `wert_02` | — | — |
| `K4116_C_TEMPERATUR_1` | [identifiant BMW K4116] · C · temperature · 1 | `wert_01` · `wert_02` | — | — |
| `K4116_C_TEMPERATUR_2` | [identifiant BMW K4116] · C · temperature · 2 | `wert_01` · `wert_02` | — | — |
| `K4116_C_TEMPERATUR_3` | [identifiant BMW K4116] · C · temperature · 3 | `wert_01` · `wert_02` | — | — |
| `K4116_C_TEMPERATUR_4` | [identifiant BMW K4116] · C · temperature · 4 | `wert_01` · `wert_02` | — | — |
| `K4116_D_TEMPERATUR` | [identifiant BMW K4116] · D · temperature | `wert_01` · `wert_02` | — | — |
| `K4119_C_DRUCK_1` | [identifiant BMW K4119] · C · pression · 1 | `wert_01` · `wert_02` · `wert_03` | — | — |
| `K4119_C_DRUCK_2` | [identifiant BMW K4119] · C · pression · 2 | `wert_01` · `wert_02` · `wert_03` | — | — |
| `K4119_C_DRUCK_3` | [identifiant BMW K4119] · C · pression · 3 | `wert_01` · `wert_02` · `wert_03` | — | — |
| `K4119_C_DRUCK_4` | [identifiant BMW K4119] · C · pression · 4 | `wert_01` · `wert_02` · `wert_03` | — | — |
| `K4119_D_DRUCK` | [identifiant BMW K4119] · D · pression | `wert_01` · `wert_02` · `wert_03` | — | — |
| `K430C_C_DATUM_1` | [identifiant BMW K430C] · C · date · 1 | `wert_01` · `wert_02` | — | — |
| `K430C_C_DATUM_2` | [identifiant BMW K430C] · C · date · 2 | `wert_01` · `wert_02` | — | — |
| `K430C_C_DATUM_3` | [identifiant BMW K430C] · C · date · 3 | `wert_01` · `wert_02` | — | — |
| `K430C_C_DATUM_4` | [identifiant BMW K430C] · C · date · 4 | `wert_01` · `wert_02` | — | — |
| `K430C_D_DATUM` | [identifiant BMW K430C] · D · date | `wert_01` · `wert_02` | — | — |
| `K4310_C_SPRACHE_1` | [identifiant BMW K4310] · C · langue · 1 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+15)* | — | — |
| `K4310_C_SPRACHE_2` | [identifiant BMW K4310] · C · langue · 2 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+15)* | — | — |
| `K4310_C_SPRACHE_3` | [identifiant BMW K4310] · C · langue · 3 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+15)* | — | — |
| `K4310_C_SPRACHE_4` | [identifiant BMW K4310] · C · langue · 4 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+15)* | — | — |
| `K4310_D_SPRACHE` | [identifiant BMW K4310] · D · langue | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` … *(+15)* | — | — |
| `KL_PREDRIVE_AUSWAHL_1` | borne · avant roulage · selection · 1 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` | — | — |
| `KL_PREDRIVE_AUSWAHL_2` | borne · avant roulage · selection · 2 | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` | — | — |
| `KUEHL_MTL_AKTIV` | refroidissement · mensuel (?) · actif | `aktiv_low` · `aktiv_high` | Coolant level sensor logic [aktiv_low: switch closed with low level, aktiv_high: switch open with low level] | — |
| `LEHNE_BF_ALIVE_ZAEHLER` | dossier · passager · signal de presence · compteur | `nicht_aktiv` · `aktiv` | Alive counter for passenger flip-forward seat backrest lock | — |
| `LEHNE_BF_ID_MONITOR` | dossier · passager · identifiant · surveillance | `nicht_aktiv` · `aktiv` | Identification timeout for passenger flip-forward seat backrest lock | — |
| `LEHNE_FA_ALIVE_ZAEHLER` | dossier · conducteur · signal de presence · compteur | `nicht_aktiv` · `aktiv` | Alive counter for driver flip-forward seat backrest lock | — |
| `LEHNE_FA_ID_MONITOR` | dossier · conducteur · identifiant · surveillance | `nicht_aktiv` · `aktiv` | Identification timeout for driver flip-forward seat backrest lock | — |
| `LICHT_AUT_WERT_1` | lumiere · automatique · valeur · 1 | `nicht_aktiv` · `aktiv` | Control auto headlight sensitivity from OBC button (linked to [LICHT_AUT_WERT_2], [LICHT_AUT_WERT_3] all to aktiv) | Seuil de declenchement de l'allumage automatique des feux. Les trois valeurs se modifient ensemble. |
| `LICHT_AUT_WERT_2` | lumiere · automatique · valeur · 2 | `nicht_aktiv` · `aktiv` | Control auto headlight sensitivity from OBC button (linked to [LICHT_AUT_WERT_1], [LICHT_AUT_WERT_3] all to aktiv) | Seuil de declenchement de l'allumage automatique des feux. Les trois valeurs se modifient ensemble. |
| `LICHT_AUT_WERT_3` | lumiere · automatique · valeur · 3 | `nicht_aktiv` · `aktiv` | Control auto headlight sensitivity from OBC button (linked to [LICHT_AUT_WERT_1], [LICHT_AUT_WERT_2] all to aktiv) | Seuil de declenchement de l'allumage automatique des feux. Les trois valeurs se modifient ensemble. |
| `LL_RL_FZG` | ralenti (?) · feu arriere · vehicule | `nicht_aktiv` · `aktiv` | Left-hand drive / right-hand drive vehicle | — |
| `LM_ALIVE` | module d'eclairage (?) · signal de presence | `nicht_aktiv` · `aktiv` | Alive monitoring for light module (LM) | — |
| `MMI_ENA_PIA` | interface homme-machine · activation · interface d'affichage (?) | `nicht_aktiv` · `aktiv` | — | — |
| `MMI_ENA_RPA_RDC` | interface homme-machine · activation · indicateur de crevaison · pression des pneus | `nicht_aktiv` · `aktiv` | — | — |
| `MMI_ENA_SHIFTLIGHTS` | interface homme-machine · activation · temoins de passage de rapport | `nicht_aktiv` · `aktiv` | — | — |
| `MMI_ENA_UHR_DATUM` | interface homme-machine · activation · heure · date | `nicht_aktiv` · `aktiv` | — | — |
| `MOTORKONSTANTE` | constante moteur | `wert_01` | Engine constant | — |
| `MOTORSPORT` | sport automobile | `wert_01` · `wert_02` | BMW M (Motorsport) [wert_01: BMW M, wert_02: BMW AG] | — |
| `MSA_ALIVE_MONITOR` | Start & Stop · signal de presence · surveillance | `nicht_aktiv` · `aktiv` | — | — |
| `MSA_CSUM_MONITOR` | Start & Stop · somme de controle · surveillance | `nicht_aktiv` · `aktiv` | — | — |
| `MSA_ENABLE` | Start & Stop · activation | `wert_01` | Automatic start/stop (MSA) enabled | — |
| `MSA_ID_MONITOR` | Start & Stop · identifiant · surveillance | `nicht_aktiv` · `aktiv` | Identification timeout for automatic start/stop (MSA) | — |
| `MSA_N_VERFUEGBAR` | Start & Stop · N · disponible | `nicht_aktiv` · `aktiv` | — | — |
| `MSA_SWITCH_ON_OFF_ENABLE` | Start & Stop · commutateur · marche · arret · activation | `nicht_aktiv` · `aktiv` | Automatic start/stop (MSA) button enabled | — |
| `M_TAG_SKALENBELEUCHTUNG` | M · jour · eclairage des cadrans | `nicht_aktiv` · `aktiv` | Instrument lights on with daytime running lights | — |
| `OHNE_QUITT_ENT` | sans · acquittement sonore · des- | `nicht_aktiv` · `aktiv` | — | — |
| `OHNE_QUITT_VER` | sans · acquittement sonore · version | `nicht_aktiv` · `aktiv` | — | — |
| `PREDRIVE_ABB_MOT` | avant roulage · attenuation (?) · moteur | `nicht_aktiv` · `aktiv` | Pre-drive checks cancelled when starting engine | — |
| `PREDRIVE_PERIODENDAUER` | avant roulage · duree de periode | `wert_01` | Pre-drive check duration [time_ms=data*100] | — |
| `PREDRIVE_START_MIT_OBD` | avant roulage · demarrage · avec · diagnostic OBD | `nicht_aktiv` · `aktiv` | — | — |
| `QUITT_AKUST_ENT` | acquittement sonore · sonore · des- | `nicht_aktiv` · `aktiv` | Acoustic unlocking acknowledge (two alarm beeps) | Acquittement sonore au deverrouillage (deux bips d'alarme) |
| `QUITT_AKUST_VER` | acquittement sonore · sonore · version | `nicht_aktiv` · `aktiv` | Acoustic locking acknowledge (single alarm beep) | Acquittement sonore au verrouillage (un bip d'alarme) |
| `QUITT_OPT_AKUST_ENT` | acquittement sonore · option · sonore · des- | `nicht_aktiv` · `aktiv` | Acoustic and visual unlocking acknowledge (two alarm beeps and turn signal flashes) | — |
| `QUITT_OPT_AKUST_VER` | acquittement sonore · option · sonore · version | `nicht_aktiv` · `aktiv` | Acoustic and visual locking acknowledge (single alarm beep and turn signal flash) | — |
| `QUITT_OPT_ENT` | acquittement sonore · option · des- | `nicht_aktiv` · `aktiv` | Visual unlocking acknowledge (two turn signal flashes) | Acquittement visuel au deverrouillage (deux flashs de clignotants) |
| `QUITT_OPT_VER` | acquittement sonore · option · version | `nicht_aktiv` · `aktiv` | Visual locking acknowledge (single turn signal flash) | Acquittement visuel au verrouillage (un flash de clignotants) |
| `RDC_ALIVE` | pression des pneus · signal de presence | `nicht_aktiv` · `aktiv` | Alive monitoring for tire pressure control (RDC) | — |
| `RDC_DISP_SWITCH` | pression des pneus · affichage · commutateur | `aktiv` · `nicht_aktiv` | — | — |
| `RDC_VERBAUT` | pression des pneus · installe | `nicht_aktiv` · `aktiv` | Tire pressure control (RDC) installed | — |
| `SCHALT_HINW_ALIVE_MONITOR` | commutation · indication · signal de presence · surveillance | `nicht_aktiv` · `aktiv` | Alive counter for gear shift indicator (SPA) | — |
| `SCHALT_HINW_CSUM_MONITOR` | commutation · indication · somme de controle · surveillance | `nicht_aktiv` · `aktiv` | Checksum monitoring for gear shift indicator (SPA) | — |
| `SCHALT_HINW_ID_MONITOR` | commutation · indication · identifiant · surveillance | `nicht_aktiv` · `aktiv` | Identification timeout for gear shift indicator (SPA) | — |
| `SCHALT_PKT_BC` | commutation · point · ordinateur de bord | `nicht_aktiv` · `aktiv` | Gear shift indicator (SPA) in on-board computer (OBC) | Indicateur de rapport affiche sur l'ordinateur de bord. |
| `SCHALT_PKT_DAUERHAFT` | commutation · point · permanent | `nicht_aktiv` · `aktiv` | Permanent gear shift indicator (SPA) in on-board computer (OBC) | Indicateur de rapport affiche en permanence. |
| `SFY_ALIVE_ZAEHLER` | securite (?) · signal de presence · compteur | `nicht_aktiv` · `aktiv` | Alive counter for airbag | — |
| `SFY_ID_MONITOR` | securite (?) · identifiant · surveillance | `nicht_aktiv` · `aktiv` | Identification timeout for airbag | — |
| `SHIFTLIGHTS` | temoins de passage de rapport | `wert_01` | — | — |
| `SHIFTLIGHTS_1` | temoins de passage de rapport · 1 | `wert_01` | — | — |
| `SHIFTLIGHTS_2` | temoins de passage de rapport · 2 | `wert_01` | — | — |
| `SHIFTLIGHTS_3` | temoins de passage de rapport · 3 | `wert_01` | — | — |
| `SHIFTLIGHTS_4` | temoins de passage de rapport · 4 | `wert_01` | — | — |
| `SHOW_CC_MSG_NR` | affichage · Check Control · message · numero | `nicht_aktiv` · `aktiv` | — | — |
| `SHOW_OEL_STAND` | affichage · huile · etat | `nicht_aktiv` · `aktiv` | — | — |
| `SITZ_AUT_VERST_AUS` | siege · automatique · amplification · arret | `nicht_aktiv` · `aktiv` | — | — |
| `SITZ_AUT_VERST_ENT` | siege · automatique · amplification · des- | `nicht_aktiv` · `aktiv` | — | — |
| `SITZ_AUT_VERST_ENT_T` | siege · automatique · amplification · des- · T | `nicht_aktiv` · `aktiv` | — | — |
| `SITZ_L_UEBERWACHUNG` | siege · _L_ · surveillance | `nicht_aktiv` · `aktiv` | Display flip-forward seat backrest lock warning | — |
| `SPA_IST_GANG_ENABLE` | assistant de stationnement (?) · reel · rapport · activation | `wert_01` · `wert_02` | — | — |
| `SPORT_ANZEIGE` | sport · affichage | `nicht_aktiv` · `aktiv` | Sport display | — |
| `SP_ANZEIGE` | point de passage de rapport · affichage | `nicht_aktiv` · `aktiv` | Gear shift indicator (SPA) | Indicateur de rapport conseille. |
| `SP_ANZEIGE_1` | point de passage de rapport · affichage · 1 | `nicht_aktiv` · `aktiv` | Car/key memory 1 for gear shift indicator (SPA) (linked to [SP_ANZEIGE]) | — |
| `SP_ANZEIGE_2` | point de passage de rapport · affichage · 2 | `nicht_aktiv` · `aktiv` | Car/key memory 2 for gear shift indicator (SPA) (linked to [SP_ANZEIGE]) | — |
| `SP_ANZEIGE_3` | point de passage de rapport · affichage · 3 | `nicht_aktiv` · `aktiv` | Car/key memory 3 for gear shift indicator (SPA) (linked to [SP_ANZEIGE]) | — |
| `SP_ANZEIGE_4` | point de passage de rapport · affichage · 4 | `nicht_aktiv` · `aktiv` | Car/key memory 4 for gear shift indicator (SPA) (linked to [SP_ANZEIGE]) | — |
| `ST_MDRV_ALIVE_MONITOR` | commande (?) · M Drive · signal de presence · surveillance | `nicht_aktiv` · `aktiv` | — | — |
| `ST_MDRV_CSUM_MONITOR` | commande (?) · M Drive · somme de controle · surveillance | `nicht_aktiv` · `aktiv` | — | — |
| `ST_MDRV_ID_MONITOR` | commande (?) · M Drive · identifiant · surveillance | `nicht_aktiv` · `aktiv` | — | — |
| `SZ_ANSTEUERUNG` | centre de commande (?) · commande | `nicht_aktiv` · `aktiv` | — | — |
| `SZ_LED_VERBAUT` | centre de commande (?) · LED · installe | `verbaut` · `nicht_verbaut` | — | — |
| `TAG_LICHT_AN` | jour · lumiere · marche | `nicht_aktiv` · `aktiv` | Day light on | — |
| `TAG_LICHT_AUS` | jour · lumiere · arret | `nicht_aktiv` · `aktiv` | Day light off | — |
| `TANK_BERUHIGUNGSZEIT` | reservoir · duree de stabilisation | `wert_01` | Fuel tank settling time | — |
| `TANK_FILTER_FALLEND` | reservoir · filtre · decroissant | `wert_01` · `wert_02` · `wert_03` | Fuel tank damping when decreasing | — |
| `TANK_FILTER_STEIGEND` | reservoir · filtre · croissant | `wert_01` · `wert_02` · `wert_03` | Fuel tank damping when increasing | — |
| `TANK_MITTLUNGSZEIT` | reservoir · duree de moyennage | `wert_01` | Fuel tank averaging period | — |
| `TANK_OHM_FEHLER_OG` | reservoir · ohm · erreur · superieur (?) | `wert_01` · `wert_02` | — | — |
| `TANK_OHM_FEHLER_UG` | reservoir · ohm · erreur · limite inferieure (?) | `wert_01` · `wert_02` | — | — |
| `TANK_RED_REICHWEITE` | reservoir · reduction · autonomie | `wert_01` · `anlieferung_01` · `anlieferung_02` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` | Fuel tank range reduction factor [factor=data/255] | — |
| `TANK_RW_WARNUNG` | reservoir · conduite a droite (?) · alerte | `wert_01` | Fuel tank low range warning threshold [distance_km=data] | — |
| `TANK_SCHWELLE_AUF_AB` | reservoir · seuil · ouverture · a partir de | `wert_01` · `wert_02` | — | — |
| `TANK_SICHERHEITSFAKTOR` | reservoir · facteur de securite | `wert_01` | Fuel tank safety factor | — |
| `TANK_VOLL_ERKENNUNG` | reservoir · plein · detection | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `anlieferung_01` · `anlieferung_02` | Fuel tank refilled detection | — |
| `TCM_ALIVE_MONITOR` | module de boite · signal de presence · surveillance | `nicht_aktiv` · `aktiv` | — | — |
| `TCM_ID_MONITOR` | module de boite · identifiant · surveillance | `nicht_aktiv` · `aktiv` | — | — |
| `TELEX_STATUS_SC` | telex · etat · _SC_ | `wert_01` · `wert_02` | — | — |
| `TEMPOMAT_SETZ_ANZ_DAUER` | regulateur de vitesse · positionnement · nombre · duree | `wert_01` · `wert_02` | Display duration of cruise control target speed [time_s=data/2] | — |
| `TIPP_BLINKEN_1X` | impulsion · clignotement · une fois | `nicht_aktiv` · `aktiv` | — | — |
| `TIPP_BLINKEN_3X` | impulsion · clignotement · trois fois | `nicht_aktiv` · `aktiv` | — | — |
| `TORQUE_3_ID_MONITOR` | couple · 3 · identifiant · surveillance | `nicht_aktiv` · `aktiv` | — | — |
| `VERDECKVARIANTE` | variante de capote | `wert_00` · `wert_01` · `wert_02` | Convertible top variant | — |
| `WARNFELD_INFO_DME` | zone d'alerte · information · calculateur moteur | `nicht_aktiv` · `aktiv` | — | — |
| `WARNUNG_GESCHW_LIMIT` | alerte · vitesse · limite | `nicht_aktiv` | Speed limit warning [CCM,EKM,KMB_E32,LCM ? speed_km/h=data+50 : speed_km/h=data] | — |
| `ZGM_ALIVE_ZAEHLER` | module passerelle central · signal de presence · compteur | `nicht_aktiv` · `aktiv` | Alive counter for central gateway module (ZGM) | — |
| `ZGM_ID_MONITOR` | module passerelle central · identifiant · surveillance | `nicht_aktiv` · `aktiv` | Identification timeout for central gateway module (ZGM) | — |
| `ZV_VER_AUS` | verrouillage centralise · version · arret | `nicht_aktiv` · `aktiv` | — | — |
| `ZV_VER_FAHRT` | verrouillage centralise · version · roulage | `nicht_aktiv` · `aktiv` | — | — |
| `ZV_VER_FAHRT_ZEIT` | verrouillage centralise · version · roulage · duree | `nicht_aktiv` · `aktiv` | — | — |
| `ZV_VER_ZEIT` | verrouillage centralise · version · duree | `nicht_aktiv` · `aktiv` | — | — |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Signification (EN)** — description issue de la base communautaire NCS Dummy, verbatim.

**Explication (FR)** — traduction française. Un tiret signifie que la ligne n'est pas
encore traduite. Les notations d'unités sont converties : `[durée s = valeur / 10]`.
