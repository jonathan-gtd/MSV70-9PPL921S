# CAPPL — CCC - applications iDrive

🟠 Module `CAPPL` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🟠 **Module sensible.** Sauvegarde du `.TRC` et FA en securite avant toute ecriture.

---

## Identification

| | |
|---|---|
| Variante | **`CCC_APP.C0B`** |
| Identifiée par | **lecture NCS Expert sur le véhicule** |
| Fonctions retenues | **305** |
| Vérification | 100 % des fonctions dans les blocs du `.TRC` |
| Avec valeurs | 303 |
| Avec explication rédigée | 7 |
| Blocs d'adresses du TRC | 13 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 305 fonctions codables

| FSW | Traduction | Valeurs | Explication |
|---|---|---|---|
| `ACC` | regulateur adaptatif | `nicht_aktiv` · `aktiv` |  |
| `ACOUSTIC` | sonore | `nicht_aktiv` · `aktiv` | Bip sonore au verrouillage et au deverrouillage. |
| `ADRBOOK` | repertoire | `nicht_aktiv` · `aktiv` |  |
| `AEREA_CODE_DVD` | zone · code · DVD | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `ALBV_TOGGLE_APP` | reglage de site · bascule · application | `nicht_aktiv` · `aktiv` |  |
| `ALLOW_GPS` | autoriser · GPS | `nicht_aktiv` · `aktiv` |  |
| `AMBANDS` | bandes AM | `nicht_aktiv` · `aktiv` |  |
| `AUDIO_SYSTEM` | audio · systeme | `e60_stereo` · `e60_hifi` · `e60_top_hifi` · `pl2_stereo` · `pl2_hifi` · `pl2_top_hifi` · `e70_stereo` · `e70_hifi` … *(+3)* |  |
| `AUDIO_VOLUME_STEPS` | audio · volume · pas | `e60_stereo` · `e60_hifi` · `e60_top_hifi` · `e63_hifi` · `e63_top_hifi` · `e61_stereo` · `e61_hifi` · `e61_top_hifi` … *(+38)* |  |
| `AUTO_DRV_LIGHT` | automatique · conducteur · lumiere | `nicht_aktiv` · `aktiv` |  |
| `AUX_CONFIG` | entree auxiliaire · configuration | `not_mounted` · `mounted` |  |
| `BACKDOOR` | hayon | `nicht_aktiv` · `aktiv` |  |
| `BAUART` | type de construction | `limousine` · `touring` · `coupe` · `cabrio` · `compact` · `sicherheitsfahrzeug` |  |
| `BAUREIHE` | serie | `e65` · `rr1` · `e66` · `e67` · `e60` · `e63` · `e64` · `e61` … *(+16)* |  |
| `BA_LAENDER_VARIANTE` | mode de fonctionnement (?) · pays · variante | `wert_01` · `ece` · `japan` · `kanada` · `usa` |  |
| `BGSCREEN_JUMP` | image de fond · saut | `nicht_aktiv` · `aktiv` |  |
| `BLUETOOTH_NODES_CCC` | Bluetooth · noeuds · systeme multimedia CCC | `nicht_aktiv` · `profil_1` · `profil_2` |  |
| `BMW_ASSIST` | BMW · assistance | `nicht_aktiv` · `aktiv` |  |
| `BMW_DEALER_CCC` | BMW · concessionnaire · systeme multimedia CCC | `nicht_aktiv` · `aktiv` |  |
| `BMW_HOTLINE_CCC` | BMW · assistance · systeme multimedia CCC | `nicht_aktiv` · `aktiv` |  |
| `BMW_MOBILE_CCC` | BMW · mobile · systeme multimedia CCC | `nicht_aktiv` · `aktiv` |  |
| `BMW_N_DEALER` | BMW · N · concessionnaire | `nicht_aktiv` · `aktiv` |  |
| `C7501_TAGFAHRLICHT` | [identifiant BMW C7501] · feux de jour | `fkt_frei` · `fkt_gesperrt` |  |
| `C750A_SCHWELLWERT_EMPFIN` | [identifiant BMW C750A] · valeur de seuil · sensibilite | `par_frei` · `par_gesperrt` |  |
| `C750A_SCHWELLWERT_NORMAL` | [identifiant BMW C750A] · valeur de seuil · normal | `par_frei` · `par_gesperrt` |  |
| `C750A_SCHWELLWERT_SEHR_E` | [identifiant BMW C750A] · valeur de seuil · tres · E | `par_frei` · `par_gesperrt` |  |
| `C940A_QUITT_OPT_ENTSCH` | [identifiant BMW C940A] · acquittement sonore · option · desarmement | `fkt_frei` · `fkt_gesperrt` |  |
| `C940C_QUITT_OPT_ENTSCH` | [identifiant BMW C940C] · acquittement sonore · option · desarmement | `fkt_frei` · `fkt_gesperrt` |  |
| `C940D_QUITT_OPT_SCHAERF` | [identifiant BMW C940D] · acquittement sonore · option · armement | `fkt_frei` · `fkt_gesperrt` |  |
| `C9410_QUITT_OPT_SCHAERF` | [identifiant BMW C9410] · acquittement sonore · option · armement | `fkt_frei` · `fkt_gesperrt` |  |
| `C9413_QUITT_AKUST_SCHAERF` | [identifiant BMW C9413] · acquittement sonore · sonore · armement | `fkt_frei` · `fkt_gesperrt` |  |
| `C9419_QUITT_OPT_KLAPPE` | [identifiant BMW C9419] · acquittement sonore · option · trappe | `fkt_frei` · `fkt_gesperrt` |  |
| `C941C_QUITT_AKUST_KLAPPE` | [identifiant BMW C941C] · acquittement sonore · sonore · trappe | `fkt_frei` · `fkt_gesperrt` |  |
| `C9C01_HKL_TASTE` | [identifiant BMW C9C01] · hayon · touche | `fkt_frei` · `fkt_gesperrt` |  |
| `CA107_AUT_VERST_DEAKT` | identifiant interne (?) · automatique · amplification · desactivation | `par_frei` · `par_gesperrt` |  |
| `CA107_AUT_VERST_ENTR` | identifiant interne (?) · automatique · amplification · deverrouillage | `par_frei` · `par_gesperrt` |  |
| `CA107_AUT_VERST_ENT_T` | identifiant interne (?) · automatique · amplification · des- · T | `par_frei` · `par_gesperrt` |  |
| `CAR_COLOR_CHAMP` | vehicule · couleur · champ | `nicht_aktiv` · `aktiv` |  |
| `CA_TMC` | acces confort (?) · info trafic | `nicht_aktiv` · `aktiv` | Informations trafic — canal complementaire. |
| `CBS` | entretien selon besoin (CBS) | `cbs_3` · `cbs_4` |  |
| `CBS_STATUS` | entretien selon besoin (CBS) · etat | `nicht_aktiv` · `aktiv` |  |
| `CCC_E71` | systeme multimedia CCC · serie E71 | `nicht_aktiv` · `aktiv` |  |
| `CC_LIST` | Check Control · liste | `nicht_aktiv` · `aktiv` |  |
| `CDC` | [variante CDC] | `nicht_aktiv` · `aktiv` |  |
| `CID_CCC` | ecran central · systeme multimedia CCC | `cid_high` · `cid_low` |  |
| `CID_COLOUR` | ecran central · couleur | `table_1` · `table_2` · `table_3` · `table_4` |  |
| `CID_TYPE` | ecran central · type | `static` · `flap` |  |
| `CLIMATE_WO_PREVIEW` | climatisation · ou · apercu | `nicht_aktiv` · `aktiv` |  |
| `CODIERUNG_ERFOLGT` | codage · effectue | `wert_01` · `codierung_erfolgt` |  |
| `CRUISE_CTRL_CCC` | regulateur de vitesse · commande · systeme multimedia CCC | `nicht_aktiv` · `aktiv` |  |
| `DAB` | radio numerique DAB | `nicht_aktiv` · `aktiv` |  |
| `DAB_BAND_SETTINGS` | radio numerique DAB · bande · reglages | `nicht_aktiv` · `aktiv` |  |
| `DAB_FM_SWITCH` | radio numerique DAB · FM · commutateur | `nicht_aktiv` · `aktiv` |  |
| `DAB_FROM_FM` | radio numerique DAB · depuis · FM | `wert_01` · `iso` · `shifted_jis` · `china` · `default` |  |
| `DAB_TO_FM` | radio numerique DAB · toit ouvrant · FM | `wert_01` |  |
| `DATUM_EINHEIT` | date · unite | `tt_mm_jj` · `mm_tt_jj` |  |
| `DAY_RUN_LIGHT_CCC` | jour · marche · lumiere · systeme multimedia CCC | `nicht_aktiv` · `aktiv` |  |
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
| `DKG` | boite a double embrayage | `nicht_aktiv` · `aktiv` |  |
| `DRUCK_EINHEIT` | pression · unite | `psi` · `bar` · `kp` · `default` |  |
| `DSC` | controle de stabilite | `nicht_aktiv` · `aktiv` |  |
| `DVD_EJECT_LOCKED` | DVD · ejection · verrouille | `free` · `locked` |  |
| `DWS` | alarme antivol (?) | `nicht_aktiv` · `aktiv` |  |
| `EDC` | amortissement pilote | `nicht_aktiv` · `aktiv` |  |
| `EINH_MASTER_KOMBI` | unite · cle maitre · combine | `nicht_aktiv` · `aktiv` |  |
| `ELDRVSEAT` | siege conducteur electrique | `nicht_aktiv` · `aktiv` |  |
| `FOND_HIGH` | places arriere · haut | `nicht_aktiv` · `aktiv` |  |
| `FOND_LOW` | places arriere · bas | `nicht_aktiv` · `aktiv` |  |
| `FOND_MID` | places arriere · milieu | `nicht_aktiv` · `aktiv` |  |
| `FULLWORD_RECOG` | mot complet · reconnaissance | `nicht_aktiv` · `aktiv` |  |
| `GAL_VALUE` | _GAL_ · valeur | `anhebung_sehr_stark` · `anhebung_stark` · `anhebung_mittel` · `anhebung_gering` · `anhebung_sehr_gering` · `keine_anhebung` |  |
| `HDC` | assistance en descente | `nicht_aktiv` · `aktiv` |  |
| `HELPNET` | appel d'urgence | `nicht_aktiv` · `aktiv` |  |
| `HIP` | interface (?) | `nicht_aktiv` · `aktiv` |  |
| `HUD` | affichage tete haute | `nicht_aktiv` · `aktiv` |  |
| `HUD_M` | affichage tete haute · M | `nicht_aktiv` · `aktiv` |  |
| `IBOC_CCC` | radio numerique HD · systeme multimedia CCC | `nicht_aktiv` · `aktiv` |  |
| `JAPAN` | Japon | `nicht_aktiv` · `aktiv` |  |
| `JAP_SIDEVIEW_CAM` | Japon · vue laterale · camera | `nicht_aktiv` · `aktiv` |  |
| `K2407_PDC_MONITORBILD` | [identifiant BMW K2407] · radars · image ecran | `fkt_frei` · `fkt_gesperrt` |  |
| `K410D_EINHEITEN_WEG` | [identifiant BMW K410D] · unites · course | `fkt_frei` · `fkt_gesperrt` |  |
| `K4110_VERBR_MPG_UK` | [identifiant BMW K4110] · consommation · miles par gallon · Royaume-Uni | `par_frei` · `par_gesperrt` |  |
| `K4110_VERBR_MPG_US` | [identifiant BMW K4110] · consommation · miles par gallon · marche americain | `par_frei` · `par_gesperrt` |  |
| `K4113_ZEIT_STD_DARST` | [identifiant BMW K4113] · duree · standard · representation | `fkt_frei` · `fkt_gesperrt` |  |
| `K4116_EINH_TEMPERATUR` | [identifiant BMW K4116] · unite · temperature | `fkt_frei` · `fkt_gesperrt` |  |
| `K4119_DRUCK_BAR` | [identifiant BMW K4119] · pression · bar | `par_frei` · `par_gesperrt` |  |
| `K4119_DRUCK_KPA` | [identifiant BMW K4119] · pression · kPa | `par_frei` · `par_gesperrt` |  |
| `K4119_DRUCK_PSI` | [identifiant BMW K4119] · pression · psi (?) | `par_frei` · `par_gesperrt` |  |
| `K430C_DATUM_FORMAT` | [identifiant BMW K430C] · date · format | `fkt_frei` · `fkt_gesperrt` |  |
| `K4310_ANZ_SPRACHE_1` | [identifiant BMW K4310] · nombre · langue · 1 | `par_frei` · `par_gesperrt` |  |
| `K4310_ANZ_SPRACHE_2` | [identifiant BMW K4310] · nombre · langue · 2 | `par_frei` · `par_gesperrt` |  |
| `K4310_ANZ_SPRACHE_3` | [identifiant BMW K4310] · nombre · langue · 3 | `par_frei` · `par_gesperrt` |  |
| `K4328_BLINKEN_TIPPBET` | [identifiant BMW K4328] · clignotement · actionnement par impulsion | `fkt_frei` · `fkt_gesperrt` |  |
| `K4D03_ANZEIGE_ACC` | [identifiant BMW K4D03] · affichage · regulateur adaptatif | `par_frei` · `par_gesperrt` |  |
| `K4D03_ANZEIGE_CHECKCONTRL` | [identifiant BMW K4D03] · affichage · Check Control | `par_frei` · `par_gesperrt` |  |
| `K4D03_ANZEIGE_GESCHWNDGKT` | [identifiant BMW K4D03] · affichage · vitesse | `par_frei` · `par_gesperrt` |  |
| `K4D03_ANZEIGE_NAVIGATION` | [identifiant BMW K4D03] · affichage · navigation | `par_frei` · `par_gesperrt` |  |
| `K4D03_ANZEIGE_TEMPOMAT` | [identifiant BMW K4D03] · affichage · regulateur de vitesse | `par_frei` · `par_gesperrt` |  |
| `K5303_GAL_GERING` | [identifiant BMW K5303] · _GAL_ · faible | `par_frei` · `par_gesperrt` |  |
| `K5303_GAL_KEINE_ANHEB` | [identifiant BMW K5303] · _GAL_ · aucun · levee | `par_frei` · `par_gesperrt` |  |
| `K5303_GAL_MITTEL` | [identifiant BMW K5303] · _GAL_ · moyen | `par_frei` · `par_gesperrt` |  |
| `K5303_GAL_SEHR_GERING` | [identifiant BMW K5303] · _GAL_ · tres · faible | `par_frei` · `par_gesperrt` |  |
| `K5303_GAL_SEHR_STARK` | [identifiant BMW K5303] · _GAL_ · tres · fort | `par_frei` · `par_gesperrt` |  |
| `K5303_GAL_STARK` | [identifiant BMW K5303] · _GAL_ · fort | `par_frei` · `par_gesperrt` |  |
| `K5305_KLANG_BALANCE` | [identifiant BMW K5305] · sonorite · balance | `par_frei` · `par_gesperrt` |  |
| `K5305_KLANG_BASS` | [identifiant BMW K5305] · sonorite · graves | `par_frei` · `par_gesperrt` |  |
| `K5305_KLANG_EQULIZER` | [identifiant BMW K5305] · sonorite · egaliseur | `par_frei` · `par_gesperrt` |  |
| `K5305_KLANG_FADER` | [identifiant BMW K5305] · sonorite · balance avant-arriere | `par_frei` · `par_gesperrt` |  |
| `K5305_KLANG_REVERB` | [identifiant BMW K5305] · sonorite · reverberation | `par_frei` · `par_gesperrt` |  |
| `K5305_KLANG_TREBLE` | [identifiant BMW K5305] · sonorite · aigus | `par_frei` · `par_gesperrt` |  |
| `K531A_LAUTST_NAV` | [identifiant BMW K531A] · volume sonore · navigation (?) | `par_frei` · `par_gesperrt` |  |
| `K531A_LAUTST_PDC` | [identifiant BMW K531A] · volume sonore · radars | `par_frei` · `par_gesperrt` |  |
| `K531A_LAUTST_SVS` | [identifiant BMW K531A] · volume sonore · surveillance (?) | `par_frei` · `par_gesperrt` |  |
| `K531A_LAUTST_TELEFON` | [identifiant BMW K531A] · volume sonore · telephone | `par_frei` · `par_gesperrt` |  |
| `K531A_LAUTST_VERKEHR` | [identifiant BMW K531A] · volume sonore · circulation | `par_frei` · `par_gesperrt` |  |
| `K7504_HEIMLEUCHTEN_150S` | [identifiant BMW K7504] · eclairage d'accompagnement · 150 s | `par_frei` · `par_gesperrt` |  |
| `K7504_HEIMLEUCHTEN_240S` | [identifiant BMW K7504] · eclairage d'accompagnement · 240 s | `par_frei` · `par_gesperrt` |  |
| `K7504_HEIMLEUCHTEN_40S` | [identifiant BMW K7504] · eclairage d'accompagnement · 40 s | `par_frei` · `par_gesperrt` |  |
| `K7504_HEIMLEUCHTEN_90S` | [identifiant BMW K7504] · eclairage d'accompagnement · 90 s | `par_frei` · `par_gesperrt` |  |
| `K7504_HEIMLEUCHTEN_AUS` | [identifiant BMW K7504] · eclairage d'accompagnement · arret | `par_frei` · `par_gesperrt` |  |
| `K8116_KLAPPENAUTOMATIK` | [identifiant BMW K8116] · automatisme de volets | `fkt_frei` · `fkt_gesperrt` |  |
| `K8119_PRG_AC` | [identifiant BMW K8119] · programme · climatisation (?) | `par_frei` · `par_gesperrt` |  |
| `K8119_PRG_AUTOMATIK` | [identifiant BMW K8119] · programme · automatique | `par_frei` · `par_gesperrt` |  |
| `K8119_PRG_GEBL_AKTUELL` | [identifiant BMW K8119] · programme · pulseur · actuel | `par_frei` · `par_gesperrt` |  |
| `K8119_PRG_LUFTVERTEILUNG` | [identifiant BMW K8119] · programme · repartition d'air | `par_frei` · `par_gesperrt` |  |
| `K8119_PRG_SCHICHTUNG` | [identifiant BMW K8119] · programme · stratification | `par_frei` · `par_gesperrt` |  |
| `K8125_KLAPPENPROGRAMM_BO` | [identifiant BMW K8125] · programme de volets · carrosserie (?) | `fkt_frei` · `fkt_gesperrt` |  |
| `K8127_KLAPPENPROGRAMM_BO` | [identifiant BMW K8127] · programme de volets · carrosserie (?) | `fkt_frei` · `fkt_gesperrt` |  |
| `K8129_ZWAKO_AC_EIN_AUTO` | [identifiant BMW K8129] · composant additionnel (?) · climatisation (?) · marche · automatique | `fkt_frei` · `fkt_gesperrt` |  |
| `K8313_GEBLAESE_0` | [identifiant BMW K8313] · pulseur · 0 | `par_frei` · `par_gesperrt` |  |
| `K8313_GEBLAESE_M10` | [identifiant BMW K8313] · pulseur · parametre M10 (?) | `par_frei` · `par_gesperrt` |  |
| `K8313_GEBLAESE_P10` | [identifiant BMW K8313] · pulseur · parametre 10 | `par_frei` · `par_gesperrt` |  |
| `K8601_SOLLWERT_0` | [identifiant BMW K8601] · consigne · 0 | `par_frei` · `par_gesperrt` |  |
| `K8601_SOLLWERT_M1` | [identifiant BMW K8601] · consigne · parametre M1 | `par_frei` · `par_gesperrt` |  |
| `K8601_SOLLWERT_M2` | [identifiant BMW K8601] · consigne · parametre M2 | `par_frei` · `par_gesperrt` |  |
| `K8601_SOLLWERT_M3` | [identifiant BMW K8601] · consigne · parametre M3 | `par_frei` · `par_gesperrt` |  |
| `K8601_SOLLWERT_P1` | [identifiant BMW K8601] · consigne · parametre 1 | `par_frei` · `par_gesperrt` |  |
| `K8601_SOLLWERT_P2` | [identifiant BMW K8601] · consigne · parametre 2 | `par_frei` · `par_gesperrt` |  |
| `K8601_SOLLWERT_P3` | [identifiant BMW K8601] · consigne · parametre 3 | `par_frei` · `par_gesperrt` |  |
| `K930E_ZV_VER_N_XMIN_FB` | [identifiant BMW K930E] · verrouillage centralise · version · N · minimum · telecommande | `fkt_frei` · `fkt_gesperrt` |  |
| `K931D_SCHEIBENABSENKUNG` | [identifiant BMW K931D] · abaissement de vitre | `par_frei` · `par_gesperrt` |  |
| `K931D_ZV_ALLE_TUEREN` | [identifiant BMW K931D] · verrouillage centralise · tous · portes | `par_frei` · `par_gesperrt` |  |
| `K931D_ZV_SELEKTIV` | [identifiant BMW K931D] · verrouillage centralise · selectif | `par_frei` · `par_gesperrt` |  |
| `K9416_QUITT_AKUST_ENTSCH` | [identifiant BMW K9416] · acquittement sonore · sonore · desarmement | `fkt_frei` · `fkt_gesperrt` |  |
| `K9901_HUPE` | [identifiant BMW K9901] · avertisseur sonore | `par_frei` · `par_gesperrt` |  |
| `K9901_KOMFORT_PARKBREMSE` | [identifiant BMW K9901] · confort · frein de stationnement | `par_frei` · `par_gesperrt` |  |
| `K9901_LENKRADHEIZUNG` | [identifiant BMW K9901] · volant chauffant | `par_frei` · `par_gesperrt` |  |
| `K9901_MONITOR_EIN` | [identifiant BMW K9901] · surveillance · marche | `par_frei` · `par_gesperrt` |  |
| `K9901_MUTE_EIN` | [identifiant BMW K9901] · coupure du son · marche | `par_frei` · `par_gesperrt` |  |
| `K9901_NAV_LETZER_HINWEIS` | [identifiant BMW K9901] · navigation (?) · dernier · indication | `par_frei` · `par_gesperrt` |  |
| `K9901_SWR` | [identifiant BMW K9901] · correcteur de site (?) | `par_frei` · `par_gesperrt` |  |
| `K9901_UMLUFT_EIN` | [identifiant BMW K9901] · recyclage d'air · marche | `par_frei` · `par_gesperrt` |  |
| `K9901_WARNBLINKEN` | [identifiant BMW K9901] · feux de detresse | `par_frei` · `par_gesperrt` |  |
| `KANA_WRITER` | kana · saisie | `nicht_aktiv` · `aktiv` |  |
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
| `MAX_PHONE_VOL` | maximum · telephone · volume | `standard` · `mulf` · `japan` · `japan_mulf` · `e60_stereo` · `e60_hifi` · `e60_top_hifi` · `e61_stereo` … *(+31)* |  |
| `MDRIVE` | M Drive | `nicht_aktiv` · `aktiv` |  |
| `MDRIVE_ALBV` | M Drive · reglage de site | `nicht_aktiv` · `aktiv` |  |
| `MDRIVE_LOW_CCC` | M Drive · bas · systeme multimedia CCC | `nicht_aktiv` · `aktiv` |  |
| `MDRIVE_MENU` | M Drive · menu | `nicht_aktiv` · `aktiv` |  |
| `MDRIVE_SMG` | M Drive · boite sequentielle | `nicht_aktiv` · `aktiv` |  |
| `MFL_CLEAN` | volant multifonction · propre | `nicht_aktiv` · `aktiv` |  |
| `MFL_HAZARD` | volant multifonction · feux de detresse | `nicht_aktiv` · `aktiv` |  |
| `MFL_HORN` | volant multifonction · avertisseur sonore | `nicht_aktiv` · `aktiv` |  |
| `MFL_MENU` | volant multifonction · menu | `nicht_aktiv` · `aktiv` |  |
| `MMC` | carte memoire | `nicht_aktiv` · `aktiv` |  |
| `MM_DEVICE` | module (?) · peripherique | `nicht_aktiv` · `aktiv` |  |
| `MM_DEVICE2` | module (?) · peripherique 2 (?) | `nicht_aktiv` · `aktiv` |  |
| `MP3_CCC` | MP3 · systeme multimedia CCC | `nicht_aktiv` · `intern` · `extern` · `full` · `cd_only` · `dvd_only` · `cd_extern` · `dvd_extern` |  |
| `MULTIPARTY` | conference | `nicht_aktiv` · `aktiv` |  |
| `NAVIGATION` | navigation | `nicht_aktiv` · `aktiv` |  |
| `NAVIGATION_TYPE` | navigation · type | `wert_01` · `navi_sv` · `navi_jp` · `navi_exbox` |  |
| `NAVI_PFEIL` | navigation · fleche | `nicht_aktiv` · `aktiv` |  |
| `NEXT_SOURCE` | suivant · source | `nicht_aktiv` · `aktiv` |  |
| `NIVI` | correcteur d'assiette | `nicht_aktiv` · `aktiv` |  |
| `NIVI_DISPLAY_OPTIONS` | correcteur d'assiette · affichage · options | `only_mainwindow` · `only_fullscreen` · `main_or_fullscreen` |  |
| `NOTEPAD` | bloc-notes | `nicht_aktiv` · `aktiv` |  |
| `NO_DETOUR` | non · deviation | `nicht_aktiv` · `aktiv` |  |
| `OIL_LEVEL` | huile · niveau | `nicht_aktiv` · `aktiv` | Affichage du niveau d'huile dans les menus iDrive. |
| `OLD_TELEMAT_STRUCT` | ancien · telematique · structure | `nicht_aktiv` · `aktiv` |  |
| `ONLINE` | en ligne | `nicht_aktiv` · `aktiv` |  |
| `ONLINE_CALLSTRATEGY` | en ligne · strategie d'appel | `no_csd` · `use_csd` · `use_paketdata` · `use_csd_paketdata` · `use_telematic_sim` |  |
| `ONLINE_DAR_TYPE_B` | en ligne · representation · type · B | `dar_us_1` · `dar_us` · `dar_us_2` · `dar_aus` · `dar_aus_gprs` · `dar_ece` · `dar_ece_gprs` · `dar_ece_1` · `dar_ece_2` |  |
| `ONLINE_DISABLE_CONTENT` | en ligne · desactivation · contenu | `content_enable` · `content_disable_vmax` · `content_disabl_eng_brake` · `content_disable_engine` · `content_disable_brake` |  |
| `ONLINE_DISABLE_VMAX` | en ligne · desactivation · vitesse maximale | `disable_us` · `disable_aus` · `standard` |  |
| `PDC` | radars | `nicht_aktiv` · `aktiv` |  |
| `PDC_SIGNAL` | radars · signal | `pdc_no` · `pdc_rear` · `pdc_front` · `pdc_front_rear` |  |
| `PDC_TYPE` | radars · type | `rear` · `full` |  |
| `PHONE_VERSION` | telephone · version | — |  |
| `PIA` | interface d'affichage (?) | `ckm` · `pia_low` |  |
| `RAIN_LIGHT_SENSOR` | pluie · lumiere · capteur | `nicht_aktiv` · `aktiv` | Reglage de la sensibilite pluie/lumiere depuis l'iDrive. |
| `RDC_CCC` | pression des pneus · systeme multimedia CCC | `nicht_aktiv` · `aktiv` |  |
| `RDS_AF_FUNKTION_CCC` | RDS · frequences alternatives · fonction · systeme multimedia CCC | `rds_aus` · `rds_ein_af_aus` · `rds_ein_af_manuell` · `rds_ein_af_automatisch` |  |
| `REAR_HEAD_UNIT` | arriere · tete · unite | `nicht_aktiv` · `aktiv` |  |
| `REVERSE_GONG` | inversion · gong | `nicht_aktiv` · `aktiv` |  |
| `REV_INPUT` | revision · entree | `nicht_aktiv` · `aktiv` |  |
| `ROLLER_COVER` | store · couvercle | `nicht_aktiv` · `aktiv` |  |
| `RSEMIDPLUS_AVAILABLE` | divertissement arriere (?) · disponible | `nicht_aktiv` · `aktiv` |  |
| `RVC` | camera de recul | `nicht_aktiv` · `aktiv` |  |
| `RVC_CONNECTED_TO_VM` | camera de recul · connecte · toit ouvrant · gestion du vehicule (?) | `nicht_aktiv` · `aktiv` |  |
| `RVC_DISPLAY_OPTIONS` | camera de recul · affichage · options | `only_mainwindow` · `only_fullscreen` · `main_or_fullscreen` |  |
| `SDARS` | radio satellite | `nicht_aktiv` · `aktiv` |  |
| `SIM` | carte SIM | `nicht_aktiv` · `aktiv` |  |
| `SITZHEIZUNG_APP_9` | siege chauffant · application · 9 | `nicht_aktiv` · `aktiv` |  |
| `SMS` | SMS | `nicht_aktiv` · `aktiv` |  |
| `SPRECHER_AKTIVE_SPRACHE` | haut-parleur · actif · langue | `sprecher_maennlich` · `sprecher_weiblich` |  |
| `SPRECHER_ALTERNATIVE_2` | haut-parleur · alternative · 2 | `sprecher_maennlich` · `sprecher_weiblich` |  |
| `SPRECHER_ALTERNATIVE_3` | haut-parleur · alternative · 3 | `sprecher_maennlich` · `sprecher_weiblich` |  |
| `STANDHEIZUNG` | chauffage stationnaire | `nicht_aktiv` · `aktiv` |  |
| `STANDLUEFTUNG_CCC` | ventilation a l'arret · systeme multimedia CCC | `nicht_aktiv` · `aktiv` |  |
| `SVR` | surveillance (?) | `nicht_aktiv` · `aktiv` |  |
| `SVS` | surveillance (?) | `nicht_aktiv` · `aktiv` |  |
| `SVS_MICROPHONE_PATH` | surveillance (?) · microphone · chemin | `most` · `analog` |  |
| `TDA_ASC_STATUS` | _TDA_ · antipatinage · etat | `nicht_aktiv` · `aktiv` |  |
| `TDA_BDA_STATUS` | _TDA_ · notice d'utilisation · etat | `nicht_aktiv` · `aktiv` |  |
| `TDA_CFG_AUTO_TRIG` | _TDA_ · configuration · automatique · declenchement | `nicht_aktiv` · `aktiv` |  |
| `TDA_CFG_CUST_MOBILE_USE` | _TDA_ · configuration · personnalise · mobile · utilisation | `nicht_aktiv` · `aktiv` |  |
| `TDA_CFG_HLC_DATA` | _TDA_ · configuration · commande d'eclairage (?) · donnees | `nicht_aktiv` · `aktiv` |  |
| `TDA_CFG_IND_PDM_GR_NBR` | _TDA_ · configuration · indicateur · module de puissance (?) · groupe (?) · nombre | `file0` · `file1` · `file2` · `file3` · `file4` · `file5` · `file6` · `file7` |  |
| `TDA_CFG_SER_ERR_SINK` | _TDA_ · configuration · serie (?) · erreur · abaissement | `nicht_aktiv` · `aktiv` |  |
| `TDA_CFG_VEHICLE_SER` | _TDA_ · configuration · vehicule · serie (?) | `nicht_aktiv` · `aktiv` |  |
| `TDA_DEACTIVATE_STATUS` | _TDA_ · desactivation · etat | `nicht_aktiv` · `aktiv` |  |
| `TDA_HLC_STATUS` | _TDA_ · commande d'eclairage (?) · etat | `nicht_aktiv` · `aktiv` |  |
| `TDA_MSC_STATUS` | _TDA_ · commande moteur (?) · etat | `nicht_aktiv` · `aktiv` |  |
| `TDA_SERVICE_STATUS` | _TDA_ · entretien · etat | `nicht_aktiv` · `aktiv` |  |
| `TDA_TDC_STATUS` | _TDA_ · point mort haut (?) · etat | `nicht_aktiv` · `aktiv` |  |
| `TELEFON_TYPE_C08` | telephone · type · [variante C08] | `no_phone` · `ece_phone` · `business_phone` · `prof_phone` · `japan_phone` · `umts_phone` · `mulf_phone` · `va_tpp` · `japan_mulf` |  |
| `TELESERVICE` | teleservice | `nicht_aktiv` · `aktiv` |  |
| `TEMPERATUR_EINHEIT` | temperature · unite | `grad_c` · `grad_f` |  |
| `TLC` | commande d'eclairage (?) | `nicht_aktiv` · `aktiv` |  |
| `TMC` | info trafic | `nicht_aktiv` · `aktiv` | Reception des informations trafic. |
| `TMC_REGIONAL` | info trafic · regional | `nicht_aktiv` · `aktiv` |  |
| `TOP_AMPL_TYPE` | haut · amplitude · type | `logic_7` · `prologic_2` · `nicht_aktiv` |  |
| `TOWBAR` | attelage | `nicht_aktiv` · `aktiv` |  |
| `TP` | point de test (?) | `nicht_aktiv` · `aktiv` |  |
| `TP_FUNKTION` | point de test (?) · fonction | `nicht_aktiv` · `tp` |  |
| `TRF` | trafic | `nicht_aktiv` · `aktiv` |  |
| `TV_FRONT_ENABLED` | television · avant · active | `nicht_aktiv` · `aktiv` |  |
| `TWO_MICROPHONES` | deux · microphones | `wert_02` · `wert_01` |  |
| `TYP_LENKUNG` | type · direction | `linkslenker` · `rechtslenker` |  |
| `UNIT_FEET` | unite · pieds | `nicht_aktiv` · `aktiv` |  |
| `VA_ADAPTIVEHEADLIGHT` | essieu avant · phare adaptatif | `nicht_aktiv` · `aktiv` |  |
| `VA_AIRVENT_SHORTCUT` | essieu avant · aerateur · raccourci | `nicht_aktiv` · `aktiv` |  |
| `VA_AUTOMATIC_SHORTCUT` | essieu avant · automatique · raccourci | `nicht_aktiv` · `aktiv` |  |
| `VA_BPI` | essieu avant · interface (?) | `nicht_aktiv` · `aktiv` |  |
| `VA_CORNERINGLIGHT` | essieu avant · eclairage d'intersection | `nicht_aktiv` · `aktiv` |  |
| `VA_FB` | essieu avant · telecommande | `nicht_aktiv` |  |
| `VA_FB_COMMUNICATION` | essieu avant · telecommande · communication | `nicht_aktiv` · `aktiv` |  |
| `VA_FB_ENTERTAINMENT` | essieu avant · telecommande · divertissement | `nicht_aktiv` · `aktiv` |  |
| `VA_FB_NAVIGATION` | essieu avant · telecommande · navigation | `nicht_aktiv` · `aktiv` |  |
| `VA_FLAP_CC` | essieu avant · volet · Check Control | `nicht_aktiv` · `aktiv` |  |
| `VA_FLAP_DISPLAY_CONFIG` | essieu avant · volet · affichage · configuration | `nicht_aktiv` · `aktiv` |  |
| `VA_FLAP_HMISTART` | essieu avant · volet · demarrage de l'interface | `nicht_aktiv` · `aktiv` |  |
| `VA_FLAP_NAVI` | essieu avant · volet · navigation | `nicht_aktiv` · `aktiv` |  |
| `VA_FLAP_PHONE` | essieu avant · volet · telephone | `nicht_aktiv` · `aktiv` |  |
| `VA_FLAP_SVS` | essieu avant · volet · surveillance (?) | `nicht_aktiv` · `aktiv` |  |
| `VA_FM_AM_TOGGLE` | essieu avant · FM · AM · bascule | `nicht_aktiv` · `aktiv` |  |
| `VA_FOND_CLIMATE` | essieu avant · places arriere · climatisation | `nicht_aktiv` · `aktiv` |  |
| `VA_HANGUL_TYPEWRITER` | essieu avant · hangul · saisie clavier | `nicht_aktiv` · `aktiv` |  |
| `VA_KANJI_TYPEWRITER` | essieu avant · kanji · saisie clavier | `nicht_aktiv` · `aktiv` |  |
| `VA_LONGDISTANCELIGHT` | essieu avant · feux longue portee | `nicht_aktiv` · `aktiv` |  |
| `VA_MANUAL_SHIFT` | essieu avant · manuel · passage de rapport | `nicht_aktiv` · `aktiv` |  |
| `VA_MODE_TOGGLE` | essieu avant · mode · bascule | `nicht_aktiv` · `aktiv` |  |
| `VA_MP3_DISCLAIMER` | essieu avant · MP3 · avertissement | `nicht_aktiv` · `aktiv` |  |
| `VA_NOAIRVENT_FINETUNING` | essieu avant · sans aerateur · reglage fin | `nicht_aktiv` · `aktiv` |  |
| `VA_PDC_LOW` | essieu avant · radars · bas | `nicht_aktiv` · `aktiv` |  |
| `VA_SEATCLIMA_SHORTCUT` | essieu avant · climatisation de siege · raccourci | `nicht_aktiv` · `aktiv` |  |
| `VA_SEATHEAT_SLIDER` | essieu avant · siege chauffant · curseur | `nicht_aktiv` · `aktiv` |  |
| `VA_SEATVENTILATION` | essieu avant · ventilation des sieges | `nicht_aktiv` · `aktiv` |  |
| `VA_WELCOMELIGHT` | essieu avant · eclairage d'accueil | `nicht_aktiv` · `aktiv` |  |
| `VEHICLE_FINDER` | vehicule · recherche | `nicht_aktiv` · `aktiv` |  |
| `VERBRAUCH` | consommation | `mpg` · `default` |  |
| `VIDEOMODUL_CCC_B` | module video · systeme multimedia CCC · B | `nicht_aktiv` · `ece` · `rgb` · `drive` |  |
| `VIDEO_DENY_C0B` | video · refus · [variante C0B] | `deny_slow_key` · `deny_stop_break` · `deny_at_all_times` · `deny_never` |  |
| `WAEHLBARE_SPRACHE_1` | selectionnable · langue · 1 | `nicht_aktiv` · `deutsch` · `englisch` · `us` · `spanisch` · `italienisch` · `franzoesisch` · `flaemisch` … *(+8)* | Langue principale de l'interface. |
| `WAEHLBARE_SPRACHE_2` | selectionnable · langue · 2 | `nicht_aktiv` · `deutsch` · `englisch` · `us` · `spanisch` · `italienisch` · `franzoesisch` · `flaemisch` … *(+7)* |  |
| `WAEHLBARE_SPRACHE_3` | selectionnable · langue · 3 | `nicht_aktiv` · `deutsch` · `englisch` · `us` · `spanisch` · `italienisch` · `franzoesisch` · `flaemisch` … *(+7)* |  |
| `WEATHERBAND` | bande meteo | `nicht_aktiv` · `aktiv` |  |
| `WEG_EINHEIT` | course · unite | `km` · `mls` |  |
| `ZBE` | molette iDrive | `high` · `low` · `low_with_slopes` |  |
| `ZEIT_EINHEIT` | duree · unite | `standard` · `e60_stereo` · `e60_hifi` · `e60_top_hifi` · `e63_hifi` · `e63_top_hifi` · `e61_stereo` · `e61_hifi` … *(+17)* |  |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Explication** — rédigée seulement quand l'effet est établi de source sûre.
