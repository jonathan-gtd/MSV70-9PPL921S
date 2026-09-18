# CAPPL — CCC - applications iDrive

🟠 Module `CAPPL` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Source

| | |
|---|---|
| Variante retenue | `CCC_APP.C07` |
| FSW dans cette variante | **233** |
| FSW toutes variantes confondues | 320 |
| Fichiers DATEN analyses | 7 |

Variantes disponibles : `CCC_APP.C06`, `CCC_APP.C07`, `CCC_APP.C08`, `CCC_APP.C09`, `CCC_APP.C0A`, `CCC_APP.C0B`, `CCC_APP.C0C`

> La variante reellement installee sur le vehicule est identifiee par NCS Dummy
> a l'ouverture du `.TRC`. Si elle differe de celle retenue ici, la liste change.

---

## Les 233 fonctions codables

| FSW | Traduction |
|---|---|
| `ACC` | regulateur adaptatif |
| `ACOUSTIC` | **ACOUSTIC** |
| `ADRBOOK` | **ADRBOOK** |
| `AEREA_CODE_DVD` | **AEREA** · **CODE** · DVD |
| `ALLOW_GPS` | **ALLOW** · **GPS** |
| `AMBANDS` | **AMBANDS** |
| `AUDIO_SYSTEM` | audio · **SYSTEM** |
| `AUTO_DRV_LIGHT` | automatique · **DRV** · lumiere |
| `AUX_CONFIG` | **AUX** · **CONFIG** |
| `BACKDOOR` | **BACKDOOR** |
| `BAUART` | type de construction |
| `BAUREIHE` | **BAUREIHE** |
| `BA_LAENDER_VARIANTE` | **BA** · **LAENDER** · variante |
| `BLUETOOTH_NODES_CCC` | Bluetooth · **NODES** · systeme multimedia CCC |
| `BMW_ASSIST` | BMW · assistance |
| `BMW_DEALER_CCC` | BMW · **DEALER** · systeme multimedia CCC |
| `BMW_HOTLINE_CCC` | BMW · **HOTLINE** · systeme multimedia CCC |
| `BMW_MOBILE_CCC` | BMW · **MOBILE** · systeme multimedia CCC |
| `BMW_N_DEALER` | BMW · N · **DEALER** |
| `C7501_TAGFAHRLICHT` | **C7501** · feux de jour |
| `C750A_SCHWELLWERT_EMPFIN` | **C750A** · valeur de seuil · **EMPFIN** |
| `C750A_SCHWELLWERT_NORMAL` | **C750A** · valeur de seuil · **NORMAL** |
| `C750A_SCHWELLWERT_SEHR_E` | **C750A** · valeur de seuil · **SEHR** · E |
| `C940A_QUITT_OPT_ENTSCH` | **C940A** · acquittement sonore · option · desarmement |
| `C940C_QUITT_OPT_ENTSCH` | **C940C** · acquittement sonore · option · desarmement |
| `C940D_QUITT_OPT_SCHAERF` | **C940D** · acquittement sonore · option · armement |
| `C9410_QUITT_OPT_SCHAERF` | **C9410** · acquittement sonore · option · armement |
| `C9413_QUITT_AKUST_SCHAERF` | **C9413** · acquittement sonore · sonore · armement |
| `C9419_QUITT_OPT_KLAPPE` | **C9419** · acquittement sonore · option · trappe |
| `C941C_QUITT_AKUST_KLAPPE` | **C941C** · acquittement sonore · sonore · trappe |
| `C9C01_HKL_TASTE` | **C9C01** · hayon · **TASTE** |
| `CA107_AUT_VERST_DEAKT` | **CA107** · automatique · amplification · **DEAKT** |
| `CA107_AUT_VERST_ENTR` | **CA107** · automatique · amplification · deverrouillage |
| `CA107_AUT_VERST_ENT_T` | **CA107** · automatique · amplification · des- · T |
| `CAR_COLOR` | vehicule · **COLOR** |
| `CBS` | entretien selon besoin (CBS) |
| `CDC` | [index variante CDC] |
| `CID_CCC` | ecran central · systeme multimedia CCC |
| `CID_COLOUR` | ecran central · **COLOUR** |
| `CID_TYPE` | ecran central · type |
| `CODIERUNG_ERFOLGT` | codage · effectue |
| `CRUISE_CTRL_CCC` | **CRUISE** · **CTRL** · systeme multimedia CCC |
| `DATUM_EINHEIT` | date · unite |
| `DAY_RUN_LIGHT_CCC` | **DAY** · **RUN** · lumiere · systeme multimedia CCC |
| `DEF_BALANCE_VOLUME` | defaut · **BALANCE** · volume |
| `DEF_BASS_VOLUME` | defaut · **BASS** · volume |
| `DEF_DATUM_EINHEIT` | defaut · date · unite |
| `DEF_DRUCK_EINHEIT` | defaut · pression · unite |
| `DEF_EQUALIZER_VALUE` | defaut · **EQUALIZER** · valeur |
| `DEF_FADER_VOLUME` | defaut · **FADER** · volume |
| `DEF_GAL_VALUE` | defaut · _GAL_ · valeur |
| `DEF_LAST_SPEECH` | defaut · charge · **SPEECH** |
| `DEF_REVERB_VALUE` | defaut · **REVERB** · valeur |
| `DEF_TEMPERATUR_EINHEIT` | defaut · temperature · unite |
| `DEF_TREMBLE_VOLUME` | defaut · **TREMBLE** · volume |
| `DEF_VERBRAUCH` | defaut · consommation |
| `DEF_WEG_EINHEIT` | defaut · course · unite |
| `DEF_ZEIT_EINHEIT` | defaut · duree · unite |
| `DIGITAL_AUDIO_SOURCE` | numerique · audio · **SOURCE** |
| `DRUCK_EINHEIT` | pression · unite |
| `DSC` | controle de stabilite |
| `DWS` | **DWS** |
| `EINH_MASTER_KOMBI` | unite · **MASTER** · combine |
| `ELDRVSEAT` | **ELDRVSEAT** |
| `FOND_HIGH` | places arriere · haut |
| `FOND_LOW` | places arriere · bas |
| `GAL_VALUE` | _GAL_ · valeur |
| `HDC` | assistance en descente |
| `HELPNET` | **HELPNET** |
| `HIP` | **HIP** |
| `HUD` | **HUD** |
| `HUD_M` | **HUD** · M |
| `IBOC_CCC` | **IBOC** · systeme multimedia CCC |
| `JAPAN` | **JAPAN** |
| `K2407_PDC_MONITORBILD` | [K-Wort K2407] · radars · **MONITORBILD** |
| `K410D_EINHEITEN_WEG` | [K-Wort K410D] · **EINHEITEN** · course |
| `K4110_VERBR_MPG_UK` | [K-Wort K4110] · **VERBR** · **MPG** · **UK** |
| `K4110_VERBR_MPG_US` | [K-Wort K4110] · **VERBR** · **MPG** · marche americain |
| `K4113_ZEIT_STD_DARST` | [K-Wort K4113] · duree · **STD** · **DARST** |
| `K4116_EINH_TEMPERATUR` | [K-Wort K4116] · unite · temperature |
| `K4119_DRUCK_BAR` | [K-Wort K4119] · pression · **BAR** |
| `K4119_DRUCK_KPA` | [K-Wort K4119] · pression · **KPA** |
| `K4119_DRUCK_PSI` | [K-Wort K4119] · pression · **PSI** |
| `K430C_DATUM_FORMAT` | [K-Wort K430C] · date · **FORMAT** |
| `K4310_ANZ_SPRACHE_1` | [K-Wort K4310] · nombre · langue · 1 |
| `K4310_ANZ_SPRACHE_2` | [K-Wort K4310] · nombre · langue · 2 |
| `K4310_ANZ_SPRACHE_3` | [K-Wort K4310] · nombre · langue · 3 |
| `K4328_BLINKEN_TIPPBET` | [K-Wort K4328] · **BLINKEN** · **TIPPBET** |
| `K4D03_ANZEIGE_ACC` | [K-Wort K4D03] · affichage · regulateur adaptatif |
| `K4D03_ANZEIGE_CHECKCONTRL` | [K-Wort K4D03] · affichage · **CHECKCONTRL** |
| `K4D03_ANZEIGE_GESCHWNDGKT` | [K-Wort K4D03] · affichage · **GESCHWNDGKT** |
| `K4D03_ANZEIGE_NAVIGATION` | [K-Wort K4D03] · affichage · navigation |
| `K4D03_ANZEIGE_TEMPOMAT` | [K-Wort K4D03] · affichage · **TEMPOMAT** |
| `K5303_GAL_GERING` | [K-Wort K5303] · _GAL_ · **GERING** |
| `K5303_GAL_KEINE_ANHEB` | [K-Wort K5303] · _GAL_ · aucun · **ANHEB** |
| `K5303_GAL_MITTEL` | [K-Wort K5303] · _GAL_ · moyen |
| `K5303_GAL_SEHR_GERING` | [K-Wort K5303] · _GAL_ · **SEHR** · **GERING** |
| `K5303_GAL_SEHR_STARK` | [K-Wort K5303] · _GAL_ · **SEHR** · **STARK** |
| `K5303_GAL_STARK` | [K-Wort K5303] · _GAL_ · **STARK** |
| `K5305_KLANG_BALANCE` | [K-Wort K5305] · sonorite · **BALANCE** |
| `K5305_KLANG_BASS` | [K-Wort K5305] · sonorite · **BASS** |
| `K5305_KLANG_EQULIZER` | [K-Wort K5305] · sonorite · **EQULIZER** |
| `K5305_KLANG_FADER` | [K-Wort K5305] · sonorite · **FADER** |
| `K5305_KLANG_REVERB` | [K-Wort K5305] · sonorite · **REVERB** |
| `K5305_KLANG_TREBLE` | [K-Wort K5305] · sonorite · **TREBLE** |
| `K531A_LAUTST_NAV` | [K-Wort K531A] · volume sonore · **NAV** |
| `K531A_LAUTST_PDC` | [K-Wort K531A] · volume sonore · radars |
| `K531A_LAUTST_SVS` | [K-Wort K531A] · volume sonore · **SVS** |
| `K531A_LAUTST_TELEFON` | [K-Wort K531A] · volume sonore · telephone |
| `K531A_LAUTST_VERKEHR` | [K-Wort K531A] · volume sonore · **VERKEHR** |
| `K7504_HEIMLEUCHTEN_150S` | [K-Wort K7504] · eclairage d'accompagnement · **150S** |
| `K7504_HEIMLEUCHTEN_240S` | [K-Wort K7504] · eclairage d'accompagnement · **240S** |
| `K7504_HEIMLEUCHTEN_40S` | [K-Wort K7504] · eclairage d'accompagnement · **40S** |
| `K7504_HEIMLEUCHTEN_90S` | [K-Wort K7504] · eclairage d'accompagnement · **90S** |
| `K7504_HEIMLEUCHTEN_AUS` | [K-Wort K7504] · eclairage d'accompagnement · arret |
| `K8116_KLAPPENAUTOMATIK` | [K-Wort K8116] · **KLAPPENAUTOMATIK** |
| `K8119_PRG_AC` | [K-Wort K8119] · programme · **AC** |
| `K8119_PRG_AUTOMATIK` | [K-Wort K8119] · programme · **AUTOMATIK** |
| `K8119_PRG_GEBL_AKTUELL` | [K-Wort K8119] · programme · **GEBL** · **AKTUELL** |
| `K8119_PRG_LUFTVERTEILUNG` | [K-Wort K8119] · programme · **LUFTVERTEILUNG** |
| `K8119_PRG_SCHICHTUNG` | [K-Wort K8119] · programme · **SCHICHTUNG** |
| `K8125_KLAPPENPROGRAMM_BO` | [K-Wort K8125] · **KLAPPENPROGRAMM** · **BO** |
| `K8127_KLAPPENPROGRAMM_BO` | [K-Wort K8127] · **KLAPPENPROGRAMM** · **BO** |
| `K8129_ZWAKO_AC_EIN_AUTO` | [K-Wort K8129] · **ZWAKO** · **AC** · marche · automatique |
| `K8313_GEBLAESE_0` | [K-Wort K8313] · pulseur · 0 |
| `K8313_GEBLAESE_M10` | [K-Wort K8313] · pulseur · **M10** |
| `K8313_GEBLAESE_P10` | [K-Wort K8313] · pulseur · **P10** |
| `K8601_SOLLWERT_0` | [K-Wort K8601] · consigne · 0 |
| `K8601_SOLLWERT_M1` | [K-Wort K8601] · consigne · **M1** |
| `K8601_SOLLWERT_M2` | [K-Wort K8601] · consigne · **M2** |
| `K8601_SOLLWERT_M3` | [K-Wort K8601] · consigne · **M3** |
| `K8601_SOLLWERT_P1` | [K-Wort K8601] · consigne · **P1** |
| `K8601_SOLLWERT_P2` | [K-Wort K8601] · consigne · **P2** |
| `K8601_SOLLWERT_P3` | [K-Wort K8601] · consigne · **P3** |
| `K930E_ZV_VER_N_XMIN_FB` | [K-Wort K930E] · verrouillage centralise · version · N · **XMIN** · telecommande |
| `K931D_SCHEIBENABSENKUNG` | [K-Wort K931D] · **SCHEIBENABSENKUNG** |
| `K931D_ZV_ALLE_TUEREN` | [K-Wort K931D] · verrouillage centralise · **ALLE** · portes |
| `K931D_ZV_SELEKTIV` | [K-Wort K931D] · verrouillage centralise · **SELEKTIV** |
| `K9416_QUITT_AKUST_ENTSCH` | [K-Wort K9416] · acquittement sonore · sonore · desarmement |
| `K9901_HUPE` | [K-Wort K9901] · **HUPE** |
| `K9901_KOMFORT_PARKBREMSE` | [K-Wort K9901] · confort · **PARKBREMSE** |
| `K9901_LENKRADHEIZUNG` | [K-Wort K9901] · **LENKRADHEIZUNG** |
| `K9901_MONITOR_EIN` | [K-Wort K9901] · surveillance · marche |
| `K9901_MUTE_EIN` | [K-Wort K9901] · **MUTE** · marche |
| `K9901_NAV_LETZER_HINWEIS` | [K-Wort K9901] · **NAV** · **LETZER** · **HINWEIS** |
| `K9901_SWR` | [K-Wort K9901] · **SWR** |
| `K9901_UMLUFT_EIN` | [K-Wort K9901] · recyclage d'air · marche |
| `K9901_WARNBLINKEN` | [K-Wort K9901] · **WARNBLINKEN** |
| `KANA_WRITER` | **KANA** · **WRITER** |
| `KLIMA_SG_ART_CCC` | climatisation · **SG** · type · systeme multimedia CCC |
| `LAENDERVARIANTE_TUNER` | variante pays · **TUNER** |
| `LAST_BALANCE_VOLUME` | charge · **BALANCE** · volume |
| `LAST_BASS_VOLUME` | charge · **BASS** · volume |
| `LAST_EQUALIZER_VALUE` | charge · **EQUALIZER** · valeur |
| `LAST_FADER_VOLUME` | charge · **FADER** · volume |
| `LAST_REVERB_VALUE` | charge · **REVERB** · valeur |
| `LAST_SPEECH` | charge · **SPEECH** |
| `LAST_TREMBLE_VOLUME` | charge · **TREMBLE** · volume |
| `LEGAL_DISCL` | mention legale · avertissement |
| `MAX_PHONE_VOL` | maximum · **PHONE** · **VOL** |
| `MDRIVE` | M Drive |
| `MDRIVE_ALBV` | M Drive · **ALBV** |
| `MDRIVE_SMG` | M Drive · **SMG** |
| `MFL_CLEAN` | volant multifonction · **CLEAN** |
| `MFL_HAZARD` | volant multifonction · **HAZARD** |
| `MFL_HORN` | volant multifonction · **HORN** |
| `MFL_MENU` | volant multifonction · **MENU** |
| `MMC` | **MMC** |
| `MP3_CCC` | **MP3** · systeme multimedia CCC |
| `MULTIPARTY` | **MULTIPARTY** |
| `NAVIGATION` | navigation |
| `NAVIGATION_TYPE` | navigation · type |
| `NIVI` | **NIVI** |
| `NIVI_DISPLAY_OPTIONS` | **NIVI** · **DISPLAY** · **OPTIONS** |
| `NOTEPAD` | **NOTEPAD** |
| `OIL_LEVEL` | huile · niveau |
| `ONLINE` | en ligne |
| `ONLINE_CALLSTRATEGY` | en ligne · **CALLSTRATEGY** |
| `ONLINE_DAR_TYPE` | en ligne · **DAR** · type |
| `ONLINE_DISABLE_CONTENT` | en ligne · desactivation · **CONTENT** |
| `ONLINE_DISABLE_VMAX` | en ligne · desactivation · **VMAX** |
| `PDC` | radars |
| `PDC_SIGNAL` | radars · **SIGNAL** |
| `PDC_TYPE` | radars · type |
| `PHONE_VERSION` | **PHONE** · **VERSION** |
| `PIA` | **PIA** |
| `RAIN_LIGHT_SENSOR` | pluie · lumiere · capteur |
| `RDC_CCC` | pression des pneus · systeme multimedia CCC |
| `RDS_AF_FUNKTION_CCC` | **RDS** · **AF** · fonction · systeme multimedia CCC |
| `REVERSE_GONG` | **REVERSE** · gong |
| `REV_INPUT` | revision · entree |
| `ROLLER_COVER` | **ROLLER** · **COVER** |
| `RVC` | **RVC** |
| `RVC_CONNECTED_TO_VM` | **RVC** · **CONNECTED** · **TO** · **VM** |
| `RVC_DISPLAY_OPTIONS` | **RVC** · **DISPLAY** · **OPTIONS** |
| `SDARS` | **SDARS** |
| `SIM` | **SIM** |
| `SITZHEIZUNG` | siege chauffant |
| `SMS` | **SMS** |
| `SPRECHER_AKTIVE_SPRACHE` | **SPRECHER** · **AKTIVE** · langue |
| `SPRECHER_ALTERNATIVE_2` | **SPRECHER** · **ALTERNATIVE** · 2 |
| `SPRECHER_ALTERNATIVE_3` | **SPRECHER** · **ALTERNATIVE** · 3 |
| `STANDHEIZUNG` | **STANDHEIZUNG** |
| `STANDLUEFTUNG_CCC` | **STANDLUEFTUNG** · systeme multimedia CCC |
| `SVS` | **SVS** |
| `SVS_MICROPHONE_PATH` | **SVS** · **MICROPHONE** · **PATH** |
| `TELEFON_TYPE` | telephone · type |
| `TEMPERATUR_EINHEIT` | temperature · unite |
| `TMC` | info trafic |
| `TOWBAR` | **TOWBAR** |
| `TP` | **TP** |
| `TP_FUNKTION` | **TP** · fonction |
| `TRF` | **TRF** |
| `TWO_MICROPHONES` | **TWO** · **MICROPHONES** |
| `TYP_LENKUNG` | type · **LENKUNG** |
| `UNIT_FEET` | **UNIT** · **FEET** |
| `VA_BPI` | essieu avant · **BPI** |
| `VA_FLAP_CC` | essieu avant · **FLAP** · Check Control |
| `VA_FLAP_DISPLAY_CONFIG` | essieu avant · **FLAP** · **DISPLAY** · **CONFIG** |
| `VA_FLAP_HMISTART` | essieu avant · **FLAP** · **HMISTART** |
| `VA_FLAP_NAVI` | essieu avant · **FLAP** · navigation |
| `VA_FLAP_PHONE` | essieu avant · **FLAP** · **PHONE** |
| `VA_FLAP_SVS` | essieu avant · **FLAP** · **SVS** |
| `VERBRAUCH` | consommation |
| `VIDEOMODUL_CCC` | **VIDEOMODUL** · systeme multimedia CCC |
| `VIDEO_DENY` | video · **DENY** |
| `WAEHLBARE_SPRACHE_1` | **WAEHLBARE** · langue · 1 |
| `WAEHLBARE_SPRACHE_2` | **WAEHLBARE** · langue · 2 |
| `WAEHLBARE_SPRACHE_3` | **WAEHLBARE** · langue · 3 |
| `WEATHERBAND` | **WEATHERBAND** |
| `WEG_EINHEIT` | course · unite |
| `ZBE` | **ZBE** |
| `ZEIT_EINHEIT` | duree · unite |

---

Les fragments en **gras** n'ont pas de traduction etablie — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments entre `_underscores_` sont ambigus : leur sens depend du module.
