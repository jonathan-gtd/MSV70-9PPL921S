# CAPPL — CCC - applications iDrive

🟠 Module `CAPPL` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Identification

| | |
|---|---|
| Variante | **`CCC_APP.C0B`** |
| Identifiée par | recoupement avec `CAPPL.TRC` lu sur le véhicule |
| Fonctions retenues | **216** |
| Écartées (hors blocs du TRC) | 89 |
| Avec valeurs | 214 |
| Avec explication rédigée | 7 |
| Blocs d'adresses du TRC | 23 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 216 fonctions codables

| FSW | Traduction | Valeurs | Explication |
|---|---|---|---|
| `ACC` | regulateur adaptatif | `nicht_aktiv` · `aktiv` |  |
| `ACOUSTIC` | **ACOUSTIC** | `nicht_aktiv` · `aktiv` | Bip sonore au verrouillage et au deverrouillage. |
| `ADRBOOK` | **ADRBOOK** | `nicht_aktiv` · `aktiv` |  |
| `AEREA_CODE_DVD` | zone · code · DVD | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `ALBV_TOGGLE_APP` | reglage de site · bascule · **APP** | `nicht_aktiv` · `aktiv` |  |
| `ALLOW_GPS` | **ALLOW** · GPS | `nicht_aktiv` · `aktiv` |  |
| `AMBANDS` | bandes AM | `nicht_aktiv` · `aktiv` |  |
| `AUDIO_SYSTEM` | audio · **SYSTEM** | `e60_stereo` · `e60_hifi` · `e60_top_hifi` · `pl2_stereo` · `pl2_hifi` · `pl2_top_hifi` · `e70_stereo` · `e70_hifi` … *(+3)* |  |
| `AUDIO_VOLUME_STEPS` | audio · volume · **STEPS** | `e60_stereo` · `e60_hifi` · `e60_top_hifi` · `e63_hifi` · `e63_top_hifi` · `e61_stereo` · `e61_hifi` · `e61_top_hifi` … *(+38)* |  |
| `AUTO_DRV_LIGHT` | automatique · **DRV** · lumiere | `nicht_aktiv` · `aktiv` |  |
| `AUX_CONFIG` | **AUX** · configuration | `not_mounted` · `mounted` |  |
| `BACKDOOR` | **BACKDOOR** | `nicht_aktiv` · `aktiv` |  |
| `BAUART` | type de construction | `limousine` · `touring` · `coupe` · `cabrio` · `compact` · `sicherheitsfahrzeug` |  |
| `BAUREIHE` | serie | `e65` · `rr1` · `e66` · `e67` · `e60` · `e63` · `e64` · `e61` … *(+16)* |  |
| `BA_LAENDER_VARIANTE` | **BA** · **LAENDER** · variante | `wert_01` · `ece` · `japan` · `kanada` · `usa` |  |
| `BGSCREEN_JUMP` | **BGSCREEN** · **JUMP** | `nicht_aktiv` · `aktiv` |  |
| `BLUETOOTH_NODES_CCC` | Bluetooth · **NODES** · systeme multimedia CCC | `nicht_aktiv` · `profil_1` · `profil_2` |  |
| `BMW_ASSIST` | BMW · assistance | `nicht_aktiv` · `aktiv` |  |
| `BMW_DEALER_CCC` | BMW · concessionnaire · systeme multimedia CCC | `nicht_aktiv` · `aktiv` |  |
| `BMW_HOTLINE_CCC` | BMW · **HOTLINE** · systeme multimedia CCC | `nicht_aktiv` · `aktiv` |  |
| `BMW_MOBILE_CCC` | BMW · mobile · systeme multimedia CCC | `nicht_aktiv` · `aktiv` |  |
| `BMW_N_DEALER` | BMW · N · concessionnaire | `nicht_aktiv` · `aktiv` |  |
| `CAR_COLOR_CHAMP` | vehicule · **COLOR** · **CHAMP** | `nicht_aktiv` · `aktiv` |  |
| `CA_TMC` | **CA** · info trafic | `nicht_aktiv` · `aktiv` | Informations trafic — canal complementaire. |
| `CBS` | entretien selon besoin (CBS) | `cbs_3` · `cbs_4` |  |
| `CBS_STATUS` | entretien selon besoin (CBS) · etat | `nicht_aktiv` · `aktiv` |  |
| `CCC_E71` | systeme multimedia CCC · **E71** | `nicht_aktiv` · `aktiv` |  |
| `CC_LIST` | Check Control · liste | `nicht_aktiv` · `aktiv` |  |
| `CDC` | [variante CDC] | `nicht_aktiv` · `aktiv` |  |
| `CID_CCC` | ecran central · systeme multimedia CCC | `cid_high` · `cid_low` |  |
| `CID_COLOUR` | ecran central · **COLOUR** | `table_1` · `table_2` · `table_3` · `table_4` |  |
| `CID_TYPE` | ecran central · type | `static` · `flap` |  |
| `CLIMATE_WO_PREVIEW` | climatisation · **WO** · **PREVIEW** | `nicht_aktiv` · `aktiv` |  |
| `CODIERUNG_ERFOLGT` | codage · effectue | `wert_01` · `codierung_erfolgt` |  |
| `CRUISE_CTRL_CCC` | **CRUISE** · **CTRL** · systeme multimedia CCC | `nicht_aktiv` · `aktiv` |  |
| `DAB` | radio numerique DAB | `nicht_aktiv` · `aktiv` |  |
| `DAB_BAND_SETTINGS` | radio numerique DAB · **BAND** · **SETTINGS** | `nicht_aktiv` · `aktiv` |  |
| `DAB_FM_SWITCH` | radio numerique DAB · FM · commutateur | `nicht_aktiv` · `aktiv` |  |
| `DAB_FROM_FM` | radio numerique DAB · **FROM** · FM | `wert_01` · `iso` · `shifted_jis` · `china` · `default` |  |
| `DAB_TO_FM` | radio numerique DAB · toit ouvrant · FM | `wert_01` |  |
| `DATUM_EINHEIT` | date · unite | `tt_mm_jj` · `mm_tt_jj` |  |
| `DAY_RUN_LIGHT_CCC` | **DAY** · marche · lumiere · systeme multimedia CCC | `nicht_aktiv` · `aktiv` |  |
| `DEF_BALANCE_VOLUME` | defaut · balance · volume | `standard` |  |
| `DEF_BASS_VOLUME` | defaut · graves · volume | `standard` |  |
| `DEF_DATUM_EINHEIT` | defaut · date · unite | `tt_mm_jj` · `mm_tt_jj` |  |
| `DEF_DRUCK_EINHEIT` | defaut · pression · unite | `psi` · `bar` · `kp` · `default` |  |
| `DEF_EQUALIZER_VALUE` | defaut · egaliseur · valeur | `standard` |  |
| `DEF_FADER_VOLUME` | defaut · balance avant-arriere · volume | `standard` |  |
| `DEF_GAL_VALUE` | defaut · _GAL_ · valeur | `anhebung_sehr_stark` · `anhebung_stark` · `anhebung_mittel` · `anhebung_gering` · `anhebung_sehr_gering` · `keine_anhebung` |  |
| `DEF_LAST_SPEECH` | defaut · charge · reconnaissance vocale | `nicht_aktiv` · `deutsch` · `englisch` · `us` · `spanisch` · `italienisch` · `franzoesisch` · `flaemisch` … *(+8)* |  |
| `DEF_REVERB_VALUE` | defaut · reverberation · valeur | `standard` |  |
| `DEF_TEMPERATUR_EINHEIT` | defaut · temperature · unite | `grad_c` · `grad_f` |  |
| `DEF_TREMBLE_VOLUME` | defaut · aigus · volume | `standard` |  |
| `DEF_VERBRAUCH` | defaut · consommation | `mpg` · `default` |  |
| `DEF_WEG_EINHEIT` | defaut · course · unite | `km` · `mls` |  |
| `DEF_ZEIT_EINHEIT` | defaut · duree · unite | `standard` · `e60_stereo` · `e60_hifi` · `e60_top_hifi` · `e63_hifi` · `e63_top_hifi` · `e61_stereo` · `e61_hifi` … *(+17)* |  |
| `DIGITAL_AUDIO_SOURCE` | numerique · audio · source | — |  |
| `DKG` | **DKG** | `nicht_aktiv` · `aktiv` |  |
| `DRUCK_EINHEIT` | pression · unite | `psi` · `bar` · `kp` · `default` |  |
| `DSC` | controle de stabilite | `nicht_aktiv` · `aktiv` |  |
| `DVD_EJECT_LOCKED` | DVD · ejection · **LOCKED** | `free` · `locked` |  |
| `DWS` | **DWS** | `nicht_aktiv` · `aktiv` |  |
| `EDC` | amortissement pilote | `nicht_aktiv` · `aktiv` |  |
| `EINH_MASTER_KOMBI` | unite · cle maitre · combine | `nicht_aktiv` · `aktiv` |  |
| `ELDRVSEAT` | **ELDRVSEAT** | `nicht_aktiv` · `aktiv` |  |
| `FOND_HIGH` | places arriere · haut | `nicht_aktiv` · `aktiv` |  |
| `FOND_LOW` | places arriere · bas | `nicht_aktiv` · `aktiv` |  |
| `FOND_MID` | places arriere · **MID** | `nicht_aktiv` · `aktiv` |  |
| `FULLWORD_RECOG` | **FULLWORD** · **RECOG** | `nicht_aktiv` · `aktiv` |  |
| `GAL_VALUE` | _GAL_ · valeur | `anhebung_sehr_stark` · `anhebung_stark` · `anhebung_mittel` · `anhebung_gering` · `anhebung_sehr_gering` · `keine_anhebung` |  |
| `HDC` | assistance en descente | `nicht_aktiv` · `aktiv` |  |
| `HELPNET` | **HELPNET** | `nicht_aktiv` · `aktiv` |  |
| `HIP` | **HIP** | `nicht_aktiv` · `aktiv` |  |
| `HUD` | affichage tete haute | `nicht_aktiv` · `aktiv` |  |
| `HUD_M` | affichage tete haute · M | `nicht_aktiv` · `aktiv` |  |
| `IBOC_CCC` | **IBOC** · systeme multimedia CCC | `nicht_aktiv` · `aktiv` |  |
| `JAPAN` | **JAPAN** | `nicht_aktiv` · `aktiv` |  |
| `JAP_SIDEVIEW_CAM` | **JAP** · **SIDEVIEW** · **CAM** | `nicht_aktiv` · `aktiv` |  |
| `KANA_WRITER` | **KANA** · **WRITER** | `nicht_aktiv` · `aktiv` |  |
| `KLIMA_SG_ART_CCC` | climatisation · calculateur · type · systeme multimedia CCC | `basis` · `high` · `low_temp` · `low_vent` · `nicht_aktiv` |  |
| `LAENDERVARIANTE_TUNER` | variante pays · tuner | `ece` · `us` · `japan` · `oceanien` · `no_area` |  |
| `LAST_BALANCE_VOLUME` | charge · balance · volume | `standard` |  |
| `LAST_BASS_VOLUME` | charge · graves · volume | `standard` |  |
| `LAST_EQUALIZER_VALUE` | charge · egaliseur · valeur | `standard` |  |
| `LAST_FADER_VOLUME` | charge · balance avant-arriere · volume | `standard` |  |
| `LAST_REVERB_VALUE` | charge · reverberation · valeur | `standard` |  |
| `LAST_SPEECH` | charge · reconnaissance vocale | `nicht_aktiv` · `deutsch` · `englisch` · `us` · `spanisch` · `italienisch` · `franzoesisch` · `flaemisch` … *(+7)* |  |
| `LAST_TREMBLE_VOLUME` | charge · aigus · volume | `standard` |  |
| `LEGAL_DISCL` | mention legale · avertissement | `nicht_aktiv` · `aktiv` | Avertissement legal affiche au demarrage. |
| `LEGAL_DISCL_TIMOUT` | mention legale · avertissement · delai d'attente | `standard` · `wert_00` · `aus` |  |
| `MAX_PHONE_VOL` | maximum · telephone · **VOL** | `standard` · `mulf` · `japan` · `japan_mulf` · `e60_stereo` · `e60_hifi` · `e60_top_hifi` · `e61_stereo` … *(+31)* |  |
| `MDRIVE` | M Drive | `nicht_aktiv` · `aktiv` |  |
| `MDRIVE_ALBV` | M Drive · reglage de site | `nicht_aktiv` · `aktiv` |  |
| `MDRIVE_LOW_CCC` | M Drive · bas · systeme multimedia CCC | `nicht_aktiv` · `aktiv` |  |
| `MDRIVE_MENU` | M Drive · menu | `nicht_aktiv` · `aktiv` |  |
| `MDRIVE_SMG` | M Drive · **SMG** | `nicht_aktiv` · `aktiv` |  |
| `MFL_CLEAN` | volant multifonction · **CLEAN** | `nicht_aktiv` · `aktiv` |  |
| `MFL_HAZARD` | volant multifonction · **HAZARD** | `nicht_aktiv` · `aktiv` |  |
| `MFL_HORN` | volant multifonction · **HORN** | `nicht_aktiv` · `aktiv` |  |
| `MFL_MENU` | volant multifonction · menu | `nicht_aktiv` · `aktiv` |  |
| `MMC` | **MMC** | `nicht_aktiv` · `aktiv` |  |
| `MM_DEVICE` | **MM** · **DEVICE** | `nicht_aktiv` · `aktiv` |  |
| `MM_DEVICE2` | **MM** · **DEVICE2** | `nicht_aktiv` · `aktiv` |  |
| `MP3_CCC` | MP3 · systeme multimedia CCC | `nicht_aktiv` · `intern` · `extern` · `full` · `cd_only` · `dvd_only` · `cd_extern` · `dvd_extern` |  |
| `MULTIPARTY` | **MULTIPARTY** | `nicht_aktiv` · `aktiv` |  |
| `NAVIGATION` | navigation | `nicht_aktiv` · `aktiv` |  |
| `NAVIGATION_TYPE` | navigation · type | `wert_01` · `navi_sv` · `navi_jp` · `navi_exbox` |  |
| `NAVI_PFEIL` | navigation · **PFEIL** | `nicht_aktiv` · `aktiv` |  |
| `NEXT_SOURCE` | **NEXT** · source | `nicht_aktiv` · `aktiv` |  |
| `NIVI` | correcteur d'assiette | `nicht_aktiv` · `aktiv` |  |
| `NIVI_DISPLAY_OPTIONS` | correcteur d'assiette · affichage · options | `only_mainwindow` · `only_fullscreen` · `main_or_fullscreen` |  |
| `NOTEPAD` | **NOTEPAD** | `nicht_aktiv` · `aktiv` |  |
| `NO_DETOUR` | non · **DETOUR** | `nicht_aktiv` · `aktiv` |  |
| `OIL_LEVEL` | huile · niveau | `nicht_aktiv` · `aktiv` | Affichage du niveau d'huile dans les menus iDrive. |
| `OLD_TELEMAT_STRUCT` | **OLD** · **TELEMAT** · **STRUCT** | `nicht_aktiv` · `aktiv` |  |
| `ONLINE` | en ligne | `nicht_aktiv` · `aktiv` |  |
| `ONLINE_CALLSTRATEGY` | en ligne · **CALLSTRATEGY** | `no_csd` · `use_csd` · `use_paketdata` · `use_csd_paketdata` · `use_telematic_sim` |  |
| `ONLINE_DAR_TYPE_B` | en ligne · **DAR** · type · B | `dar_us_1` · `dar_us` · `dar_us_2` · `dar_aus` · `dar_aus_gprs` · `dar_ece` · `dar_ece_gprs` · `dar_ece_1` · `dar_ece_2` |  |
| `ONLINE_DISABLE_CONTENT` | en ligne · desactivation · contenu | `content_enable` · `content_disable_vmax` · `content_disabl_eng_brake` · `content_disable_engine` · `content_disable_brake` |  |
| `ONLINE_DISABLE_VMAX` | en ligne · desactivation · vitesse maximale | `disable_us` · `disable_aus` · `standard` |  |
| `PDC` | radars | `nicht_aktiv` · `aktiv` |  |
| `PDC_SIGNAL` | radars · signal | `pdc_no` · `pdc_rear` · `pdc_front` · `pdc_front_rear` |  |
| `PDC_TYPE` | radars · type | `rear` · `full` |  |
| `PHONE_VERSION` | telephone · version | — |  |
| `PIA` | **PIA** | `ckm` · `pia_low` |  |
| `RAIN_LIGHT_SENSOR` | pluie · lumiere · capteur | `nicht_aktiv` · `aktiv` | Reglage de la sensibilite pluie/lumiere depuis l'iDrive. |
| `RDC_CCC` | pression des pneus · systeme multimedia CCC | `nicht_aktiv` · `aktiv` |  |
| `RDS_AF_FUNKTION_CCC` | **RDS** · **AF** · fonction · systeme multimedia CCC | `rds_aus` · `rds_ein_af_aus` · `rds_ein_af_manuell` · `rds_ein_af_automatisch` |  |
| `REAR_HEAD_UNIT` | arriere · **HEAD** · unite | `nicht_aktiv` · `aktiv` |  |
| `REVERSE_GONG` | inversion · gong | `nicht_aktiv` · `aktiv` |  |
| `REV_INPUT` | revision · entree | `nicht_aktiv` · `aktiv` |  |
| `ROLLER_COVER` | **ROLLER** · **COVER** | `nicht_aktiv` · `aktiv` |  |
| `RSEMIDPLUS_AVAILABLE` | **RSEMIDPLUS** · **AVAILABLE** | `nicht_aktiv` · `aktiv` |  |
| `RVC` | camera de recul | `nicht_aktiv` · `aktiv` |  |
| `RVC_CONNECTED_TO_VM` | camera de recul · **CONNECTED** · toit ouvrant · **VM** | `nicht_aktiv` · `aktiv` |  |
| `RVC_DISPLAY_OPTIONS` | camera de recul · affichage · options | `only_mainwindow` · `only_fullscreen` · `main_or_fullscreen` |  |
| `SDARS` | **SDARS** | `nicht_aktiv` · `aktiv` |  |
| `SIM` | **SIM** | `nicht_aktiv` · `aktiv` |  |
| `SITZHEIZUNG_APP_9` | siege chauffant · **APP** · 9 | `nicht_aktiv` · `aktiv` |  |
| `SMS` | **SMS** | `nicht_aktiv` · `aktiv` |  |
| `SPRECHER_AKTIVE_SPRACHE` | haut-parleur · **AKTIVE** · langue | `sprecher_maennlich` · `sprecher_weiblich` |  |
| `SPRECHER_ALTERNATIVE_2` | haut-parleur · alternative · 2 | `sprecher_maennlich` · `sprecher_weiblich` |  |
| `SPRECHER_ALTERNATIVE_3` | haut-parleur · alternative · 3 | `sprecher_maennlich` · `sprecher_weiblich` |  |
| `STANDHEIZUNG` | chauffage stationnaire | `nicht_aktiv` · `aktiv` |  |
| `STANDLUEFTUNG_CCC` | ventilation a l'arret · systeme multimedia CCC | `nicht_aktiv` · `aktiv` |  |
| `SVR` | **SVR** | `nicht_aktiv` · `aktiv` |  |
| `SVS` | **SVS** | `nicht_aktiv` · `aktiv` |  |
| `SVS_MICROPHONE_PATH` | **SVS** · **MICROPHONE** · **PATH** | `most` · `analog` |  |
| `TDA_ASC_STATUS` | _TDA_ · antipatinage · etat | `nicht_aktiv` · `aktiv` |  |
| `TDA_BDA_STATUS` | _TDA_ · **BDA** · etat | `nicht_aktiv` · `aktiv` |  |
| `TDA_CFG_AUTO_TRIG` | _TDA_ · configuration · automatique · **TRIG** | `nicht_aktiv` · `aktiv` |  |
| `TDA_CFG_CUST_MOBILE_USE` | _TDA_ · configuration · **CUST** · mobile · utilisation | `nicht_aktiv` · `aktiv` |  |
| `TDA_CFG_HLC_DATA` | _TDA_ · configuration · **HLC** · donnees | `nicht_aktiv` · `aktiv` |  |
| `TDA_CFG_IND_PDM_GR_NBR` | _TDA_ · configuration · **IND** · **PDM** · **GR** · **NBR** | `file0` · `file1` · `file2` · `file3` · `file4` · `file5` · `file6` · `file7` |  |
| `TDA_CFG_SER_ERR_SINK` | _TDA_ · configuration · **SER** · erreur · **SINK** | `nicht_aktiv` · `aktiv` |  |
| `TDA_CFG_VEHICLE_SER` | _TDA_ · configuration · **VEHICLE** · **SER** | `nicht_aktiv` · `aktiv` |  |
| `TDA_DEACTIVATE_STATUS` | _TDA_ · **DEACTIVATE** · etat | `nicht_aktiv` · `aktiv` |  |
| `TDA_HLC_STATUS` | _TDA_ · **HLC** · etat | `nicht_aktiv` · `aktiv` |  |
| `TDA_MSC_STATUS` | _TDA_ · **MSC** · etat | `nicht_aktiv` · `aktiv` |  |
| `TDA_SERVICE_STATUS` | _TDA_ · entretien · etat | `nicht_aktiv` · `aktiv` |  |
| `TDA_TDC_STATUS` | _TDA_ · **TDC** · etat | `nicht_aktiv` · `aktiv` |  |
| `TELEFON_TYPE_C08` | telephone · type · [variante C08] | `no_phone` · `ece_phone` · `business_phone` · `prof_phone` · `japan_phone` · `umts_phone` · `mulf_phone` · `va_tpp` · `japan_mulf` |  |
| `TELESERVICE` | **TELESERVICE** | `nicht_aktiv` · `aktiv` |  |
| `TEMPERATUR_EINHEIT` | temperature · unite | `grad_c` · `grad_f` |  |
| `TLC` | **TLC** | `nicht_aktiv` · `aktiv` |  |
| `TMC` | info trafic | `nicht_aktiv` · `aktiv` | Reception des informations trafic. |
| `TMC_REGIONAL` | info trafic · regional | `nicht_aktiv` · `aktiv` |  |
| `TOP_AMPL_TYPE` | **TOP** · **AMPL** · type | `logic_7` · `prologic_2` · `nicht_aktiv` |  |
| `TOWBAR` | attelage | `nicht_aktiv` · `aktiv` |  |
| `TP` | **TP** | `nicht_aktiv` · `aktiv` |  |
| `TP_FUNKTION` | **TP** · fonction | `nicht_aktiv` · `tp` |  |
| `TRF` | **TRF** | `nicht_aktiv` · `aktiv` |  |
| `TV_FRONT_ENABLED` | **TV** · avant · **ENABLED** | `nicht_aktiv` · `aktiv` |  |
| `TWO_MICROPHONES` | **TWO** · **MICROPHONES** | `wert_02` · `wert_01` |  |
| `TYP_LENKUNG` | type · **LENKUNG** | `linkslenker` · `rechtslenker` |  |
| `UNIT_FEET` | unite · **FEET** | `nicht_aktiv` · `aktiv` |  |
| `VA_ADAPTIVEHEADLIGHT` | essieu avant · **ADAPTIVEHEADLIGHT** | `nicht_aktiv` · `aktiv` |  |
| `VA_AIRVENT_SHORTCUT` | essieu avant · **AIRVENT** · raccourci | `nicht_aktiv` · `aktiv` |  |
| `VA_AUTOMATIC_SHORTCUT` | essieu avant · **AUTOMATIC** · raccourci | `nicht_aktiv` · `aktiv` |  |
| `VA_BPI` | essieu avant · **BPI** | `nicht_aktiv` · `aktiv` |  |
| `VA_CORNERINGLIGHT` | essieu avant · **CORNERINGLIGHT** | `nicht_aktiv` · `aktiv` |  |
| `VA_FB` | essieu avant · telecommande | `nicht_aktiv` |  |
| `VA_FB_COMMUNICATION` | essieu avant · telecommande · **COMMUNICATION** | `nicht_aktiv` · `aktiv` |  |
| `VA_FB_ENTERTAINMENT` | essieu avant · telecommande · divertissement | `nicht_aktiv` · `aktiv` |  |
| `VA_FB_NAVIGATION` | essieu avant · telecommande · navigation | `nicht_aktiv` · `aktiv` |  |
| `VA_FLAP_CC` | essieu avant · volet · Check Control | `nicht_aktiv` · `aktiv` |  |
| `VA_FLAP_DISPLAY_CONFIG` | essieu avant · volet · affichage · configuration | `nicht_aktiv` · `aktiv` |  |
| `VA_FLAP_HMISTART` | essieu avant · volet · **HMISTART** | `nicht_aktiv` · `aktiv` |  |
| `VA_FLAP_NAVI` | essieu avant · volet · navigation | `nicht_aktiv` · `aktiv` |  |
| `VA_FLAP_PHONE` | essieu avant · volet · telephone | `nicht_aktiv` · `aktiv` |  |
| `VA_FLAP_SVS` | essieu avant · volet · **SVS** | `nicht_aktiv` · `aktiv` |  |
| `VA_FM_AM_TOGGLE` | essieu avant · FM · AM · bascule | `nicht_aktiv` · `aktiv` |  |
| `VA_FOND_CLIMATE` | essieu avant · places arriere · climatisation | `nicht_aktiv` · `aktiv` |  |
| `VA_HANGUL_TYPEWRITER` | essieu avant · **HANGUL** · **TYPEWRITER** | `nicht_aktiv` · `aktiv` |  |
| `VA_KANJI_TYPEWRITER` | essieu avant · **KANJI** · **TYPEWRITER** | `nicht_aktiv` · `aktiv` |  |
| `VA_LONGDISTANCELIGHT` | essieu avant · **LONGDISTANCELIGHT** | `nicht_aktiv` · `aktiv` |  |
| `VA_MANUAL_SHIFT` | essieu avant · **MANUAL** · **SHIFT** | `nicht_aktiv` · `aktiv` |  |
| `VA_MODE_TOGGLE` | essieu avant · mode · bascule | `nicht_aktiv` · `aktiv` |  |
| `VA_MP3_DISCLAIMER` | essieu avant · MP3 · avertissement | `nicht_aktiv` · `aktiv` |  |
| `VA_NOAIRVENT_FINETUNING` | essieu avant · **NOAIRVENT** · **FINETUNING** | `nicht_aktiv` · `aktiv` |  |
| `VA_PDC_LOW` | essieu avant · radars · bas | `nicht_aktiv` · `aktiv` |  |
| `VA_SEATCLIMA_SHORTCUT` | essieu avant · **SEATCLIMA** · raccourci | `nicht_aktiv` · `aktiv` |  |
| `VA_SEATHEAT_SLIDER` | essieu avant · **SEATHEAT** · **SLIDER** | `nicht_aktiv` · `aktiv` |  |
| `VA_SEATVENTILATION` | essieu avant · **SEATVENTILATION** | `nicht_aktiv` · `aktiv` |  |
| `VA_WELCOMELIGHT` | essieu avant · eclairage d'accueil | `nicht_aktiv` · `aktiv` |  |
| `VEHICLE_FINDER` | **VEHICLE** · **FINDER** | `nicht_aktiv` · `aktiv` |  |
| `VERBRAUCH` | consommation | `mpg` · `default` |  |
| `VIDEOMODUL_CCC_B` | **VIDEOMODUL** · systeme multimedia CCC · B | `nicht_aktiv` · `ece` · `rgb` · `drive` |  |
| `VIDEO_DENY_C0B` | video · **DENY** · [variante C0B] | `deny_slow_key` · `deny_stop_break` · `deny_at_all_times` · `deny_never` |  |
| `WAEHLBARE_SPRACHE_1` | **WAEHLBARE** · langue · 1 | `nicht_aktiv` · `deutsch` · `englisch` · `us` · `spanisch` · `italienisch` · `franzoesisch` · `flaemisch` … *(+8)* | Langue principale de l'interface. |
| `WAEHLBARE_SPRACHE_2` | **WAEHLBARE** · langue · 2 | `nicht_aktiv` · `deutsch` · `englisch` · `us` · `spanisch` · `italienisch` · `franzoesisch` · `flaemisch` … *(+7)* |  |
| `WAEHLBARE_SPRACHE_3` | **WAEHLBARE** · langue · 3 | `nicht_aktiv` · `deutsch` · `englisch` · `us` · `spanisch` · `italienisch` · `franzoesisch` · `flaemisch` … *(+7)* |  |
| `WEATHERBAND` | bande meteo | `nicht_aktiv` · `aktiv` |  |
| `WEG_EINHEIT` | course · unite | `km` · `mls` |  |
| `ZBE` | **ZBE** | `high` · `low` · `low_with_slopes` |  |
| `ZEIT_EINHEIT` | duree · unite | `standard` · `e60_stereo` · `e60_hifi` · `e60_top_hifi` · `e63_hifi` · `e63_top_hifi` · `e61_stereo` · `e61_hifi` … *(+17)* |  |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Explication** — rédigée seulement quand l'effet est établi de source sûre.
