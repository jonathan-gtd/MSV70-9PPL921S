# CAPPL — CCC - applications iDrive

🟠 Module `CAPPL` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Source

| | |
|---|---|
| Variante retenue | `CCC_APP.C07` |
| Fonctions codables | **233** |
| Avec valeurs extraites | 231 (99 %) |
| Avec explication rédigée | 7 |
| Fichiers DATEN analysés | 7 |

Variantes disponibles : `CCC_APP.C06`, `CCC_APP.C07`, `CCC_APP.C08`, `CCC_APP.C09`, `CCC_APP.C0A`, `CCC_APP.C0B`, `CCC_APP.C0C`

> La variante réellement installée est identifiée par NCS Dummy à l'ouverture du `.TRC`.
> Si elle diffère de celle retenue ici, la liste change.

---

## Les 233 fonctions codables

| FSW | Traduction | Valeurs possibles | Explication |
|---|---|---|---|
| `ACC` | regulateur adaptatif | `nicht_aktiv` · `aktiv` |  |
| `ACOUSTIC` | **ACOUSTIC** | `nicht_aktiv` · `aktiv` | Bip sonore au verrouillage et au deverrouillage. |
| `ADRBOOK` | **ADRBOOK** | `nicht_aktiv` · `aktiv` |  |
| `AEREA_CODE_DVD` | **AEREA** · **CODE** · DVD | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `ALLOW_GPS` | **ALLOW** · **GPS** | `nicht_aktiv` · `aktiv` |  |
| `AMBANDS` | **AMBANDS** | `nicht_aktiv` · `aktiv` |  |
| `AUDIO_SYSTEM` | audio · **SYSTEM** | `e60_stereo` · `e60_hifi` · `e60_top_hifi` · `pl2_stereo` · `pl2_hifi` · `pl2_top_hifi` · `e70_stereo` · `e70_hifi` · `e70_top_hifi` |  |
| `AUTO_DRV_LIGHT` | automatique · **DRV** · lumiere | `nicht_aktiv` · `aktiv` |  |
| `AUX_CONFIG` | **AUX** · **CONFIG** | `not_mounted` · `mounted` |  |
| `BACKDOOR` | **BACKDOOR** | `nicht_aktiv` · `aktiv` |  |
| `BAUART` | type de construction | `limousine` · `touring` · `coupe` · `cabrio` · `compact` · `sicherheitsfahrzeug` |  |
| `BAUREIHE` | **BAUREIHE** | `e65` · `rr1` · `e66` · `e67` · `e60` · `e63` · `e64` · `e61` … *(+14)* |  |
| `BA_LAENDER_VARIANTE` | **BA** · **LAENDER** · variante | `wert_01` · `ece` · `japan` · `kanada` · `usa` |  |
| `BLUETOOTH_NODES_CCC` | Bluetooth · **NODES** · systeme multimedia CCC | `nicht_aktiv` · `profil_1` · `profil_2` |  |
| `BMW_ASSIST` | BMW · assistance | `nicht_aktiv` · `aktiv` |  |
| `BMW_DEALER_CCC` | BMW · **DEALER** · systeme multimedia CCC | `nicht_aktiv` · `aktiv` |  |
| `BMW_HOTLINE_CCC` | BMW · **HOTLINE** · systeme multimedia CCC | `nicht_aktiv` · `aktiv` |  |
| `BMW_MOBILE_CCC` | BMW · **MOBILE** · systeme multimedia CCC | `nicht_aktiv` · `aktiv` |  |
| `BMW_N_DEALER` | BMW · N · **DEALER** | `nicht_aktiv` · `aktiv` |  |
| `C7501_TAGFAHRLICHT` | **C7501** · feux de jour | `fkt_frei` · `fkt_gesperrt` |  |
| `C750A_SCHWELLWERT_EMPFIN` | **C750A** · valeur de seuil · **EMPFIN** | `par_frei` · `par_gesperrt` |  |
| `C750A_SCHWELLWERT_NORMAL` | **C750A** · valeur de seuil · **NORMAL** | `par_frei` · `par_gesperrt` |  |
| `C750A_SCHWELLWERT_SEHR_E` | **C750A** · valeur de seuil · **SEHR** · E | `par_frei` · `par_gesperrt` |  |
| `C940A_QUITT_OPT_ENTSCH` | **C940A** · acquittement sonore · option · desarmement | `fkt_frei` · `fkt_gesperrt` |  |
| `C940C_QUITT_OPT_ENTSCH` | **C940C** · acquittement sonore · option · desarmement | `fkt_frei` · `fkt_gesperrt` |  |
| `C940D_QUITT_OPT_SCHAERF` | **C940D** · acquittement sonore · option · armement | `fkt_frei` · `fkt_gesperrt` |  |
| `C9410_QUITT_OPT_SCHAERF` | **C9410** · acquittement sonore · option · armement | `fkt_frei` · `fkt_gesperrt` |  |
| `C9413_QUITT_AKUST_SCHAERF` | **C9413** · acquittement sonore · sonore · armement | `fkt_frei` · `fkt_gesperrt` |  |
| `C9419_QUITT_OPT_KLAPPE` | **C9419** · acquittement sonore · option · trappe | `fkt_frei` · `fkt_gesperrt` |  |
| `C941C_QUITT_AKUST_KLAPPE` | **C941C** · acquittement sonore · sonore · trappe | `fkt_frei` · `fkt_gesperrt` |  |
| `C9C01_HKL_TASTE` | **C9C01** · hayon · **TASTE** | `fkt_frei` · `fkt_gesperrt` |  |
| `CA107_AUT_VERST_DEAKT` | **CA107** · automatique · amplification · **DEAKT** | `par_frei` · `par_gesperrt` |  |
| `CA107_AUT_VERST_ENTR` | **CA107** · automatique · amplification · deverrouillage | `par_frei` · `par_gesperrt` |  |
| `CA107_AUT_VERST_ENT_T` | **CA107** · automatique · amplification · des- · T | `par_frei` · `par_gesperrt` |  |
| `CAR_COLOR` | vehicule · **COLOR** | `nicht_aktiv` · `aktiv` |  |
| `CBS` | entretien selon besoin (CBS) | `cbs_3` · `cbs_4` |  |
| `CDC` | [variante CDC] | `nicht_aktiv` · `aktiv` |  |
| `CID_CCC` | ecran central · systeme multimedia CCC | `cid_high` · `cid_low` |  |
| `CID_COLOUR` | ecran central · **COLOUR** | `table_1` · `table_2` · `table_3` · `table_4` |  |
| `CID_TYPE` | ecran central · type | `static` · `flap` |  |
| `CODIERUNG_ERFOLGT` | codage · effectue | `wert_01` · `codierung_erfolgt` |  |
| `CRUISE_CTRL_CCC` | **CRUISE** · **CTRL** · systeme multimedia CCC | `nicht_aktiv` · `aktiv` |  |
| `DATUM_EINHEIT` | date · unite | `tt_mm_jj` · `mm_tt_jj` |  |
| `DAY_RUN_LIGHT_CCC` | **DAY** · **RUN** · lumiere · systeme multimedia CCC | `nicht_aktiv` · `aktiv` |  |
| `DEF_BALANCE_VOLUME` | defaut · **BALANCE** · volume | `standard` |  |
| `DEF_BASS_VOLUME` | defaut · **BASS** · volume | `standard` |  |
| `DEF_DATUM_EINHEIT` | defaut · date · unite | `tt_mm_jj` · `mm_tt_jj` |  |
| `DEF_DRUCK_EINHEIT` | defaut · pression · unite | `psi` · `bar` · `kp` · `default` |  |
| `DEF_EQUALIZER_VALUE` | defaut · **EQUALIZER** · valeur | `standard` |  |
| `DEF_FADER_VOLUME` | defaut · **FADER** · volume | `standard` |  |
| `DEF_GAL_VALUE` | defaut · _GAL_ · valeur | `anhebung_sehr_stark` · `anhebung_stark` · `anhebung_mittel` · `anhebung_gering` · `anhebung_sehr_gering` · `keine_anhebung` |  |
| `DEF_LAST_SPEECH` | defaut · charge · **SPEECH** | `nicht_aktiv` · `deutsch` · `englisch` · `us` · `spanisch` · `italienisch` · `franzoesisch` · `flaemisch` … *(+8)* |  |
| `DEF_REVERB_VALUE` | defaut · **REVERB** · valeur | `standard` |  |
| `DEF_TEMPERATUR_EINHEIT` | defaut · temperature · unite | `grad_c` · `grad_f` |  |
| `DEF_TREMBLE_VOLUME` | defaut · **TREMBLE** · volume | `standard` |  |
| `DEF_VERBRAUCH` | defaut · consommation | `mpg` · `default` |  |
| `DEF_WEG_EINHEIT` | defaut · course · unite | `km` · `mls` |  |
| `DEF_ZEIT_EINHEIT` | defaut · duree · unite | `standard` · `e60_stereo` · `e60_hifi` · `e60_top_hifi` · `e63_hifi` · `e63_top_hifi` · `e61_stereo` · `e61_hifi` … *(+15)* |  |
| `DIGITAL_AUDIO_SOURCE` | numerique · audio · **SOURCE** | — |  |
| `DRUCK_EINHEIT` | pression · unite | `psi` · `bar` · `kp` · `default` |  |
| `DSC` | controle de stabilite | `nicht_aktiv` · `aktiv` |  |
| `DWS` | **DWS** | `nicht_aktiv` · `aktiv` |  |
| `EINH_MASTER_KOMBI` | unite · **MASTER** · combine | `nicht_aktiv` · `aktiv` |  |
| `ELDRVSEAT` | **ELDRVSEAT** | `nicht_aktiv` · `aktiv` |  |
| `FOND_HIGH` | places arriere · haut | `nicht_aktiv` · `aktiv` |  |
| `FOND_LOW` | places arriere · bas | `nicht_aktiv` · `aktiv` |  |
| `GAL_VALUE` | _GAL_ · valeur | `anhebung_sehr_stark` · `anhebung_stark` · `anhebung_mittel` · `anhebung_gering` · `anhebung_sehr_gering` · `keine_anhebung` |  |
| `HDC` | assistance en descente | `nicht_aktiv` · `aktiv` |  |
| `HELPNET` | **HELPNET** | `nicht_aktiv` · `aktiv` |  |
| `HIP` | **HIP** | `nicht_aktiv` · `aktiv` |  |
| `HUD` | **HUD** | `nicht_aktiv` · `aktiv` |  |
| `HUD_M` | **HUD** · M | `nicht_aktiv` · `aktiv` |  |
| `IBOC_CCC` | **IBOC** · systeme multimedia CCC | `nicht_aktiv` · `aktiv` |  |
| `JAPAN` | **JAPAN** | `nicht_aktiv` · `aktiv` |  |
| `K2407_PDC_MONITORBILD` | [K-Wort K2407] · radars · **MONITORBILD** | `fkt_frei` · `fkt_gesperrt` |  |
| `K410D_EINHEITEN_WEG` | [K-Wort K410D] · **EINHEITEN** · course | `fkt_frei` · `fkt_gesperrt` |  |
| `K4110_VERBR_MPG_UK` | [K-Wort K4110] · **VERBR** · **MPG** · **UK** | `par_frei` · `par_gesperrt` |  |
| `K4110_VERBR_MPG_US` | [K-Wort K4110] · **VERBR** · **MPG** · marche americain | `par_frei` · `par_gesperrt` |  |
| `K4113_ZEIT_STD_DARST` | [K-Wort K4113] · duree · **STD** · **DARST** | `fkt_frei` · `fkt_gesperrt` |  |
| `K4116_EINH_TEMPERATUR` | [K-Wort K4116] · unite · temperature | `fkt_frei` · `fkt_gesperrt` |  |
| `K4119_DRUCK_BAR` | [K-Wort K4119] · pression · **BAR** | `par_frei` · `par_gesperrt` |  |
| `K4119_DRUCK_KPA` | [K-Wort K4119] · pression · **KPA** | `par_frei` · `par_gesperrt` |  |
| `K4119_DRUCK_PSI` | [K-Wort K4119] · pression · **PSI** | `par_frei` · `par_gesperrt` |  |
| `K430C_DATUM_FORMAT` | [K-Wort K430C] · date · **FORMAT** | `fkt_frei` · `fkt_gesperrt` |  |
| `K4310_ANZ_SPRACHE_1` | [K-Wort K4310] · nombre · langue · 1 | `par_frei` · `par_gesperrt` |  |
| `K4310_ANZ_SPRACHE_2` | [K-Wort K4310] · nombre · langue · 2 | `par_frei` · `par_gesperrt` |  |
| `K4310_ANZ_SPRACHE_3` | [K-Wort K4310] · nombre · langue · 3 | `par_frei` · `par_gesperrt` |  |
| `K4328_BLINKEN_TIPPBET` | [K-Wort K4328] · **BLINKEN** · **TIPPBET** | `fkt_frei` · `fkt_gesperrt` |  |
| `K4D03_ANZEIGE_ACC` | [K-Wort K4D03] · affichage · regulateur adaptatif | `par_frei` · `par_gesperrt` |  |
| `K4D03_ANZEIGE_CHECKCONTRL` | [K-Wort K4D03] · affichage · **CHECKCONTRL** | `par_frei` · `par_gesperrt` |  |
| `K4D03_ANZEIGE_GESCHWNDGKT` | [K-Wort K4D03] · affichage · **GESCHWNDGKT** | `par_frei` · `par_gesperrt` |  |
| `K4D03_ANZEIGE_NAVIGATION` | [K-Wort K4D03] · affichage · navigation | `par_frei` · `par_gesperrt` |  |
| `K4D03_ANZEIGE_TEMPOMAT` | [K-Wort K4D03] · affichage · **TEMPOMAT** | `par_frei` · `par_gesperrt` |  |
| `K5303_GAL_GERING` | [K-Wort K5303] · _GAL_ · **GERING** | `par_frei` · `par_gesperrt` |  |
| `K5303_GAL_KEINE_ANHEB` | [K-Wort K5303] · _GAL_ · aucun · **ANHEB** | `par_frei` · `par_gesperrt` |  |
| `K5303_GAL_MITTEL` | [K-Wort K5303] · _GAL_ · moyen | `par_frei` · `par_gesperrt` |  |
| `K5303_GAL_SEHR_GERING` | [K-Wort K5303] · _GAL_ · **SEHR** · **GERING** | `par_frei` · `par_gesperrt` |  |
| `K5303_GAL_SEHR_STARK` | [K-Wort K5303] · _GAL_ · **SEHR** · **STARK** | `par_frei` · `par_gesperrt` |  |
| `K5303_GAL_STARK` | [K-Wort K5303] · _GAL_ · **STARK** | `par_frei` · `par_gesperrt` |  |
| `K5305_KLANG_BALANCE` | [K-Wort K5305] · sonorite · **BALANCE** | `par_frei` · `par_gesperrt` |  |
| `K5305_KLANG_BASS` | [K-Wort K5305] · sonorite · **BASS** | `par_frei` · `par_gesperrt` |  |
| `K5305_KLANG_EQULIZER` | [K-Wort K5305] · sonorite · **EQULIZER** | `par_frei` · `par_gesperrt` |  |
| `K5305_KLANG_FADER` | [K-Wort K5305] · sonorite · **FADER** | `par_frei` · `par_gesperrt` |  |
| `K5305_KLANG_REVERB` | [K-Wort K5305] · sonorite · **REVERB** | `par_frei` · `par_gesperrt` |  |
| `K5305_KLANG_TREBLE` | [K-Wort K5305] · sonorite · **TREBLE** | `par_frei` · `par_gesperrt` |  |
| `K531A_LAUTST_NAV` | [K-Wort K531A] · volume sonore · **NAV** | `par_frei` · `par_gesperrt` |  |
| `K531A_LAUTST_PDC` | [K-Wort K531A] · volume sonore · radars | `par_frei` · `par_gesperrt` |  |
| `K531A_LAUTST_SVS` | [K-Wort K531A] · volume sonore · **SVS** | `par_frei` · `par_gesperrt` |  |
| `K531A_LAUTST_TELEFON` | [K-Wort K531A] · volume sonore · telephone | `par_frei` · `par_gesperrt` |  |
| `K531A_LAUTST_VERKEHR` | [K-Wort K531A] · volume sonore · **VERKEHR** | `par_frei` · `par_gesperrt` |  |
| `K7504_HEIMLEUCHTEN_150S` | [K-Wort K7504] · eclairage d'accompagnement · **150S** | `par_frei` · `par_gesperrt` |  |
| `K7504_HEIMLEUCHTEN_240S` | [K-Wort K7504] · eclairage d'accompagnement · **240S** | `par_frei` · `par_gesperrt` |  |
| `K7504_HEIMLEUCHTEN_40S` | [K-Wort K7504] · eclairage d'accompagnement · **40S** | `par_frei` · `par_gesperrt` |  |
| `K7504_HEIMLEUCHTEN_90S` | [K-Wort K7504] · eclairage d'accompagnement · **90S** | `par_frei` · `par_gesperrt` |  |
| `K7504_HEIMLEUCHTEN_AUS` | [K-Wort K7504] · eclairage d'accompagnement · arret | `par_frei` · `par_gesperrt` |  |
| `K8116_KLAPPENAUTOMATIK` | [K-Wort K8116] · **KLAPPENAUTOMATIK** | `fkt_frei` · `fkt_gesperrt` |  |
| `K8119_PRG_AC` | [K-Wort K8119] · programme · **AC** | `par_frei` · `par_gesperrt` |  |
| `K8119_PRG_AUTOMATIK` | [K-Wort K8119] · programme · **AUTOMATIK** | `par_frei` · `par_gesperrt` |  |
| `K8119_PRG_GEBL_AKTUELL` | [K-Wort K8119] · programme · **GEBL** · **AKTUELL** | `par_frei` · `par_gesperrt` |  |
| `K8119_PRG_LUFTVERTEILUNG` | [K-Wort K8119] · programme · **LUFTVERTEILUNG** | `par_frei` · `par_gesperrt` |  |
| `K8119_PRG_SCHICHTUNG` | [K-Wort K8119] · programme · **SCHICHTUNG** | `par_frei` · `par_gesperrt` |  |
| `K8125_KLAPPENPROGRAMM_BO` | [K-Wort K8125] · **KLAPPENPROGRAMM** · **BO** | `fkt_frei` · `fkt_gesperrt` |  |
| `K8127_KLAPPENPROGRAMM_BO` | [K-Wort K8127] · **KLAPPENPROGRAMM** · **BO** | `fkt_frei` · `fkt_gesperrt` |  |
| `K8129_ZWAKO_AC_EIN_AUTO` | [K-Wort K8129] · **ZWAKO** · **AC** · marche · automatique | `fkt_frei` · `fkt_gesperrt` |  |
| `K8313_GEBLAESE_0` | [K-Wort K8313] · pulseur · 0 | `par_frei` · `par_gesperrt` |  |
| `K8313_GEBLAESE_M10` | [K-Wort K8313] · pulseur · **M10** | `par_frei` · `par_gesperrt` |  |
| `K8313_GEBLAESE_P10` | [K-Wort K8313] · pulseur · **P10** | `par_frei` · `par_gesperrt` |  |
| `K8601_SOLLWERT_0` | [K-Wort K8601] · consigne · 0 | `par_frei` · `par_gesperrt` |  |
| `K8601_SOLLWERT_M1` | [K-Wort K8601] · consigne · **M1** | `par_frei` · `par_gesperrt` |  |
| `K8601_SOLLWERT_M2` | [K-Wort K8601] · consigne · **M2** | `par_frei` · `par_gesperrt` |  |
| `K8601_SOLLWERT_M3` | [K-Wort K8601] · consigne · **M3** | `par_frei` · `par_gesperrt` |  |
| `K8601_SOLLWERT_P1` | [K-Wort K8601] · consigne · **P1** | `par_frei` · `par_gesperrt` |  |
| `K8601_SOLLWERT_P2` | [K-Wort K8601] · consigne · **P2** | `par_frei` · `par_gesperrt` |  |
| `K8601_SOLLWERT_P3` | [K-Wort K8601] · consigne · **P3** | `par_frei` · `par_gesperrt` |  |
| `K930E_ZV_VER_N_XMIN_FB` | [K-Wort K930E] · verrouillage centralise · version · N · **XMIN** · telecommande | `fkt_frei` · `fkt_gesperrt` |  |
| `K931D_SCHEIBENABSENKUNG` | [K-Wort K931D] · **SCHEIBENABSENKUNG** | `par_frei` · `par_gesperrt` |  |
| `K931D_ZV_ALLE_TUEREN` | [K-Wort K931D] · verrouillage centralise · **ALLE** · portes | `par_frei` · `par_gesperrt` |  |
| `K931D_ZV_SELEKTIV` | [K-Wort K931D] · verrouillage centralise · **SELEKTIV** | `par_frei` · `par_gesperrt` |  |
| `K9416_QUITT_AKUST_ENTSCH` | [K-Wort K9416] · acquittement sonore · sonore · desarmement | `fkt_frei` · `fkt_gesperrt` |  |
| `K9901_HUPE` | [K-Wort K9901] · **HUPE** | `par_frei` · `par_gesperrt` |  |
| `K9901_KOMFORT_PARKBREMSE` | [K-Wort K9901] · confort · **PARKBREMSE** | `par_frei` · `par_gesperrt` |  |
| `K9901_LENKRADHEIZUNG` | [K-Wort K9901] · **LENKRADHEIZUNG** | `par_frei` · `par_gesperrt` |  |
| `K9901_MONITOR_EIN` | [K-Wort K9901] · surveillance · marche | `par_frei` · `par_gesperrt` |  |
| `K9901_MUTE_EIN` | [K-Wort K9901] · **MUTE** · marche | `par_frei` · `par_gesperrt` |  |
| `K9901_NAV_LETZER_HINWEIS` | [K-Wort K9901] · **NAV** · **LETZER** · **HINWEIS** | `par_frei` · `par_gesperrt` |  |
| `K9901_SWR` | [K-Wort K9901] · **SWR** | `par_frei` · `par_gesperrt` |  |
| `K9901_UMLUFT_EIN` | [K-Wort K9901] · recyclage d'air · marche | `par_frei` · `par_gesperrt` |  |
| `K9901_WARNBLINKEN` | [K-Wort K9901] · **WARNBLINKEN** | `par_frei` · `par_gesperrt` |  |
| `KANA_WRITER` | **KANA** · **WRITER** | `nicht_aktiv` · `aktiv` |  |
| `KLIMA_SG_ART_CCC` | climatisation · **SG** · type · systeme multimedia CCC | `basis` · `high` · `low_vent` |  |
| `LAENDERVARIANTE_TUNER` | variante pays · **TUNER** | `no_area` · `ece` · `us` · `japan` · `oceanien` |  |
| `LAST_BALANCE_VOLUME` | charge · **BALANCE** · volume | `standard` |  |
| `LAST_BASS_VOLUME` | charge · **BASS** · volume | `standard` |  |
| `LAST_EQUALIZER_VALUE` | charge · **EQUALIZER** · valeur | `standard` |  |
| `LAST_FADER_VOLUME` | charge · **FADER** · volume | `standard` |  |
| `LAST_REVERB_VALUE` | charge · **REVERB** · valeur | `standard` |  |
| `LAST_SPEECH` | charge · **SPEECH** | `nicht_aktiv` · `deutsch` · `englisch` · `us` · `spanisch` · `italienisch` · `franzoesisch` · `flaemisch` … *(+7)* |  |
| `LAST_TREMBLE_VOLUME` | charge · **TREMBLE** · volume | `standard` |  |
| `LEGAL_DISCL` | mention legale · avertissement | `nicht_aktiv` · `aktiv` | Avertissement legal affiche au demarrage. |
| `MAX_PHONE_VOL` | maximum · **PHONE** · **VOL** | `standard` · `mulf` · `japan` · `japan_mulf` · `e60_stereo` · `e60_hifi` · `e60_top_hifi` · `e61_stereo` … *(+27)* |  |
| `MDRIVE` | M Drive | `nicht_aktiv` · `aktiv` |  |
| `MDRIVE_ALBV` | M Drive · **ALBV** | `nicht_aktiv` · `aktiv` |  |
| `MDRIVE_SMG` | M Drive · **SMG** | `nicht_aktiv` · `aktiv` |  |
| `MFL_CLEAN` | volant multifonction · **CLEAN** | `nicht_aktiv` · `aktiv` |  |
| `MFL_HAZARD` | volant multifonction · **HAZARD** | `nicht_aktiv` · `aktiv` |  |
| `MFL_HORN` | volant multifonction · **HORN** | `nicht_aktiv` · `aktiv` |  |
| `MFL_MENU` | volant multifonction · **MENU** | `nicht_aktiv` · `aktiv` |  |
| `MMC` | **MMC** | `nicht_aktiv` · `aktiv` |  |
| `MP3_CCC` | **MP3** · systeme multimedia CCC | `nicht_aktiv` · `intern` · `extern` · `full` · `cd_only` · `dvd_only` · `cd_extern` · `dvd_extern` |  |
| `MULTIPARTY` | **MULTIPARTY** | `nicht_aktiv` · `aktiv` |  |
| `NAVIGATION` | navigation | `nicht_aktiv` · `aktiv` |  |
| `NAVIGATION_TYPE` | navigation · type | `wert_01` · `navi_sv` · `navi_jp` · `navi_exbox` |  |
| `NIVI` | **NIVI** | `nicht_aktiv` · `aktiv` |  |
| `NIVI_DISPLAY_OPTIONS` | **NIVI** · **DISPLAY** · **OPTIONS** | `only_mainwindow` · `only_fullscreen` · `main_or_fullscreen` |  |
| `NOTEPAD` | **NOTEPAD** | `nicht_aktiv` · `aktiv` |  |
| `OIL_LEVEL` | huile · niveau | `nicht_aktiv` · `aktiv` | Affichage du niveau d'huile dans les menus iDrive. |
| `ONLINE` | en ligne | `nicht_aktiv` · `aktiv` |  |
| `ONLINE_CALLSTRATEGY` | en ligne · **CALLSTRATEGY** | `no_csd` · `use_csd` · `use_paketdata` · `use_csd_paketdata` · `use_telematic_sim` |  |
| `ONLINE_DAR_TYPE` | en ligne · **DAR** · type | `dar_ece` · `dar_us` · `dar_uk` · `dar_japan` · `dar_aus` · `dar_bmw` |  |
| `ONLINE_DISABLE_CONTENT` | en ligne · desactivation · **CONTENT** | `content_enable` · `content_disable_vmax` · `content_disabl_eng_brake` · `content_disable_engine` · `content_disable_brake` |  |
| `ONLINE_DISABLE_VMAX` | en ligne · desactivation · **VMAX** | `disable_us` · `disable_aus` · `standard` |  |
| `PDC` | radars | `nicht_aktiv` · `aktiv` |  |
| `PDC_SIGNAL` | radars · **SIGNAL** | `pdc_no` · `pdc_rear` · `pdc_front` · `pdc_front_rear` |  |
| `PDC_TYPE` | radars · type | `rear` · `full` |  |
| `PHONE_VERSION` | **PHONE** · **VERSION** | — |  |
| `PIA` | **PIA** | `ckm` · `pia_low` |  |
| `RAIN_LIGHT_SENSOR` | pluie · lumiere · capteur | `nicht_aktiv` · `aktiv` | Reglage de la sensibilite pluie/lumiere depuis l'iDrive. |
| `RDC_CCC` | pression des pneus · systeme multimedia CCC | `nicht_aktiv` · `aktiv` |  |
| `RDS_AF_FUNKTION_CCC` | **RDS** · **AF** · fonction · systeme multimedia CCC | `rds_aus` · `rds_ein_af_aus` · `rds_ein_af_manuell` · `rds_ein_af_automatisch` |  |
| `REVERSE_GONG` | **REVERSE** · gong | `nicht_aktiv` · `aktiv` |  |
| `REV_INPUT` | revision · entree | `nicht_aktiv` · `aktiv` |  |
| `ROLLER_COVER` | **ROLLER** · **COVER** | `nicht_aktiv` · `aktiv` |  |
| `RVC` | **RVC** | `nicht_aktiv` · `aktiv` |  |
| `RVC_CONNECTED_TO_VM` | **RVC** · **CONNECTED** · **TO** · **VM** | `nicht_aktiv` · `aktiv` |  |
| `RVC_DISPLAY_OPTIONS` | **RVC** · **DISPLAY** · **OPTIONS** | `only_mainwindow` · `only_fullscreen` · `main_or_fullscreen` |  |
| `SDARS` | **SDARS** | `nicht_aktiv` · `aktiv` |  |
| `SIM` | **SIM** | `nicht_aktiv` · `aktiv` |  |
| `SITZHEIZUNG` | siege chauffant | `nicht_aktiv` · `aktiv` |  |
| `SMS` | **SMS** | `nicht_aktiv` · `aktiv` |  |
| `SPRECHER_AKTIVE_SPRACHE` | **SPRECHER** · **AKTIVE** · langue | `sprecher_maennlich` · `sprecher_weiblich` |  |
| `SPRECHER_ALTERNATIVE_2` | **SPRECHER** · **ALTERNATIVE** · 2 | `sprecher_maennlich` · `sprecher_weiblich` |  |
| `SPRECHER_ALTERNATIVE_3` | **SPRECHER** · **ALTERNATIVE** · 3 | `sprecher_maennlich` · `sprecher_weiblich` |  |
| `STANDHEIZUNG` | **STANDHEIZUNG** | `nicht_aktiv` · `aktiv` |  |
| `STANDLUEFTUNG_CCC` | **STANDLUEFTUNG** · systeme multimedia CCC | `nicht_aktiv` · `aktiv` |  |
| `SVS` | **SVS** | `nicht_aktiv` · `aktiv` |  |
| `SVS_MICROPHONE_PATH` | **SVS** · **MICROPHONE** · **PATH** | `most` · `analog` |  |
| `TELEFON_TYPE` | telephone · type | `no_phone` · `ece_phone` · `business_phone` · `prof_phone` · `japan_phone` · `umts_phone` · `mulf_phone` · `va_tpp` |  |
| `TEMPERATUR_EINHEIT` | temperature · unite | `grad_c` · `grad_f` |  |
| `TMC` | info trafic | `nicht_aktiv` · `aktiv` | Reception des informations trafic. |
| `TOWBAR` | **TOWBAR** | `nicht_aktiv` · `aktiv` |  |
| `TP` | **TP** | `nicht_aktiv` · `aktiv` |  |
| `TP_FUNKTION` | **TP** · fonction | `nicht_aktiv` · `tp` |  |
| `TRF` | **TRF** | `nicht_aktiv` · `aktiv` |  |
| `TWO_MICROPHONES` | **TWO** · **MICROPHONES** | `wert_02` · `wert_01` |  |
| `TYP_LENKUNG` | type · **LENKUNG** | `linkslenker` · `rechtslenker` |  |
| `UNIT_FEET` | **UNIT** · **FEET** | `nicht_aktiv` · `aktiv` |  |
| `VA_BPI` | essieu avant · **BPI** | `nicht_aktiv` · `aktiv` |  |
| `VA_FLAP_CC` | essieu avant · **FLAP** · Check Control | `nicht_aktiv` · `aktiv` |  |
| `VA_FLAP_DISPLAY_CONFIG` | essieu avant · **FLAP** · **DISPLAY** · **CONFIG** | `nicht_aktiv` · `aktiv` |  |
| `VA_FLAP_HMISTART` | essieu avant · **FLAP** · **HMISTART** | `nicht_aktiv` · `aktiv` |  |
| `VA_FLAP_NAVI` | essieu avant · **FLAP** · navigation | `nicht_aktiv` · `aktiv` |  |
| `VA_FLAP_PHONE` | essieu avant · **FLAP** · **PHONE** | `nicht_aktiv` · `aktiv` |  |
| `VA_FLAP_SVS` | essieu avant · **FLAP** · **SVS** | `nicht_aktiv` · `aktiv` |  |
| `VERBRAUCH` | consommation | `mpg` · `default` |  |
| `VIDEOMODUL_CCC` | **VIDEOMODUL** · systeme multimedia CCC | `nicht_aktiv` · `ece` · `rgb` · `drive` |  |
| `VIDEO_DENY` | video · **DENY** | `deny_slow_key` · `deny_stop_break` · `deny_at_all_times` · `deny_never` | Conditions de blocage de la video pendant le roulage. |
| `WAEHLBARE_SPRACHE_1` | **WAEHLBARE** · langue · 1 | `nicht_aktiv` · `deutsch` · `englisch` · `us` · `spanisch` · `italienisch` · `franzoesisch` · `flaemisch` … *(+8)* | Langue principale de l'interface. |
| `WAEHLBARE_SPRACHE_2` | **WAEHLBARE** · langue · 2 | `nicht_aktiv` · `deutsch` · `englisch` · `us` · `spanisch` · `italienisch` · `franzoesisch` · `flaemisch` … *(+7)* |  |
| `WAEHLBARE_SPRACHE_3` | **WAEHLBARE** · langue · 3 | `nicht_aktiv` · `deutsch` · `englisch` · `us` · `spanisch` · `italienisch` · `franzoesisch` · `flaemisch` … *(+7)* |  |
| `WEATHERBAND` | **WEATHERBAND** | `nicht_aktiv` · `aktiv` |  |
| `WEG_EINHEIT` | course · unite | `km` · `mls` |  |
| `ZBE` | **ZBE** | `high` · `low` · `low_with_slopes` |  |
| `ZEIT_EINHEIT` | duree · unite | `standard` · `e60_stereo` · `e60_hifi` · `e60_top_hifi` · `e63_hifi` · `e63_top_hifi` · `e61_stereo` · `e61_hifi` … *(+15)* |  |

---

**Traduction** : décomposition du nom allemand — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments en **gras** n'ont pas de traduction établie, ceux entre `_underscores_` sont ambigus.

**Valeurs possibles** : extraites des DATEN. `aktiv`/`nicht_aktiv` = interrupteur,
`wert_xx` = valeur numérique, autres = choix nommés.

**Explication** : rédigée uniquement pour les fonctions identifiées avec certitude.
Une case vide ne veut pas dire que la fonction est sans intérêt — seulement que son effet
n'a pas été établi de source sûre.
