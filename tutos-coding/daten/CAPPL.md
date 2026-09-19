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
| Signification (EN) | 289 |
| Explication (FR) | 289 |
| Blocs d'adresses du TRC | 13 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 305 fonctions codables

| FSW | Décomposition | Valeurs | Signification (EN) | Explication (FR) |
|---|---|---|---|---|
| `ACC` | regulateur adaptatif | `nicht_aktiv` · `aktiv` | Active cruise control (ACC) | Regulateur de vitesse actif (ACC) |
| `ACOUSTIC` | sonore | `nicht_aktiv` · `aktiv` | Acoustic acknowledge when locking/unlocking with remote control (linked to [C941C_QUITT_AKUST_KLAPPE], [K9416_QUITT_AKUST_ENTSCH] and [C9413_QUITT_AKUST_SCHAERF]) | Bip sonore au verrouillage et au deverrouillage. |
| `ADRBOOK` | repertoire | `nicht_aktiv` · `aktiv` | Address book | Repertoire |
| `AEREA_CODE_DVD` | zone · code · DVD | `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` | Dvd region code | Code region DVD |
| `ALBV_TOGGLE_APP` | reglage de site · bascule · application | `nicht_aktiv` · `aktiv` | Toggle active seat backrest width adjustment | Bascule du reglage actif de largeur du dossier |
| `ALLOW_GPS` | autoriser · GPS | `nicht_aktiv` · `aktiv` | Display GPS data | Affichage des donnees GPS |
| `AMBANDS` | bandes AM | `nicht_aktiv` · `aktiv` | AM radio band in source selection | Bande AM proposee dans le choix des sources |
| `AUDIO_SYSTEM` | audio · systeme | `e60_stereo` · `e60_hifi` · `e60_top_hifi` · `pl2_stereo` · `pl2_hifi` · `pl2_top_hifi` · `e70_stereo` · `e70_hifi` … *(+3)* | Audio system | Systeme audio |
| `AUDIO_VOLUME_STEPS` | audio · volume · pas | `e60_stereo` · `e60_hifi` · `e60_top_hifi` · `e63_hifi` · `e63_top_hifi` · `e61_stereo` · `e61_hifi` · `e61_top_hifi` … *(+38)* | Audio volume steps | Pas de reglage du volume audio |
| `AUTO_DRV_LIGHT` | automatique · conducteur · lumiere | `nicht_aktiv` · `aktiv` | Automatic light control (FLC) | Allumage automatique des feux (FLC) |
| `AUX_CONFIG` | entree auxiliaire · configuration | `not_mounted` · `mounted` | Auxiliary input | Entree auxiliaire |
| `BACKDOOR` | hayon | `nicht_aktiv` · `aktiv` | Automatic trunk lid | Coffre motorise |
| `BAUART` | type de construction | `limousine` · `touring` · `coupe` · `cabrio` · `compact` · `sicherheitsfahrzeug` | Model variant | Variante de modele |
| `BAUREIHE` | serie | `e65` · `rr1` · `e66` · `e67` · `e60` · `e63` · `e64` · `e61` … *(+16)* | Chassis | Serie du vehicule |
| `BA_LAENDER_VARIANTE` | mode de fonctionnement (?) · pays · variante | `wert_01` · `ece` · `japan` · `kanada` · `usa` | Country variant BMW Assist | Variante pays de BMW Assist |
| `BGSCREEN_JUMP` | image de fond · saut | `nicht_aktiv` · `aktiv` | — | — |
| `BLUETOOTH_NODES_CCC` | Bluetooth · noeuds · systeme multimedia CCC | `nicht_aktiv` · `profil_1` · `profil_2` | Bluetooth nodes | Noeuds Bluetooth |
| `BMW_ASSIST` | BMW · assistance | `nicht_aktiv` · `aktiv` | BMW Assist | BMW Assist |
| `BMW_DEALER_CCC` | BMW · concessionnaire · systeme multimedia CCC | `nicht_aktiv` · `aktiv` | BMW dealer | Concessionnaire BMW |
| `BMW_HOTLINE_CCC` | BMW · assistance · systeme multimedia CCC | `nicht_aktiv` · `aktiv` | BMW hotline | Assistance telephonique BMW |
| `BMW_MOBILE_CCC` | BMW · mobile · systeme multimedia CCC | `nicht_aktiv` · `aktiv` | BMW mobile | BMW Mobile |
| `BMW_N_DEALER` | BMW · N · concessionnaire | `nicht_aktiv` · `aktiv` | BMW nearest dealer | Concessionnaire BMW le plus proche |
| `C7501_TAGFAHRLICHT` | [identifiant BMW C7501] · feux de jour | `fkt_frei` · `fkt_gesperrt` | European daytime running lights | Feux de jour, norme europeenne |
| `C750A_SCHWELLWERT_EMPFIN` | [identifiant BMW C750A] · valeur de seuil · sensibilite | `par_frei` · `par_gesperrt` | Normal sensitivity for automatic light control (FLC) | Sensibilite normale de l'allumage automatique des feux (FLC) |
| `C750A_SCHWELLWERT_NORMAL` | [identifiant BMW C750A] · valeur de seuil · normal | `par_frei` · `par_gesperrt` | High sensitivity for automatic light control (FLC) | Sensibilite elevee de l'allumage automatique des feux (FLC) |
| `C750A_SCHWELLWERT_SEHR_E` | [identifiant BMW C750A] · valeur de seuil · tres · E | `par_frei` · `par_gesperrt` | Very high sensitivity for automatic light control (FLC) | Sensibilite tres elevee de l'allumage automatique des feux (FLC) |
| `C940A_QUITT_OPT_ENTSCH` | [identifiant BMW C940A] · acquittement sonore · option · desarmement | `fkt_frei` · `fkt_gesperrt` | Visual unlocking acknowledge (two turn signal flashes) | Acquittement visuel au deverrouillage (deux flashs de clignotants) |
| `C940C_QUITT_OPT_ENTSCH` | [identifiant BMW C940C] · acquittement sonore · option · desarmement | `fkt_frei` · `fkt_gesperrt` | Visual unlocking acknowledge without anti-theft alarm system (two turn signal flashes) | Acquittement visuel au deverrouillage sans alarme antivol (deux flashs de clignotants) |
| `C940D_QUITT_OPT_SCHAERF` | [identifiant BMW C940D] · acquittement sonore · option · armement | `fkt_frei` · `fkt_gesperrt` | Visual locking acknowledge without anti-theft alarm system (single turn signal flash) | Acquittement visuel au verrouillage sans alarme antivol (un flash de clignotants) |
| `C9410_QUITT_OPT_SCHAERF` | [identifiant BMW C9410] · acquittement sonore · option · armement | `fkt_frei` · `fkt_gesperrt` | Visual locking acknowledge (single turn signal flash) | Acquittement visuel au verrouillage (un flash de clignotants) |
| `C9413_QUITT_AKUST_SCHAERF` | [identifiant BMW C9413] · acquittement sonore · sonore · armement | `fkt_frei` · `fkt_gesperrt` | Acoustic locking acknowledge (single alarm beep) | Acquittement sonore au verrouillage (un bip d'alarme) |
| `C9419_QUITT_OPT_KLAPPE` | [identifiant BMW C9419] · acquittement sonore · option · trappe | `fkt_frei` · `fkt_gesperrt` | Visual locking acknowledge only when all doors and lids closed | Acquittement visuel au verrouillage uniquement si toutes les portes et trappes sont fermees |
| `C941C_QUITT_AKUST_KLAPPE` | [identifiant BMW C941C] · acquittement sonore · sonore · trappe | `fkt_frei` · `fkt_gesperrt` | Acoustic locking acknowledge only when all doors and lids closed | Acquittement sonore au verrouillage uniquement si toutes les portes et trappes sont fermees |
| `C9C01_HKL_TASTE` | [identifiant BMW C9C01] · hayon · touche | `fkt_frei` · `fkt_gesperrt` | Remote control trunk lid button function | Fonction du bouton de coffre sur la telecommande |
| `CA107_AUT_VERST_DEAKT` | identifiant interne (?) · automatique · amplification · desactivation | `par_frei` · `par_gesperrt` | Automatic seat adjustment disabled | Reglage automatique du siege desactive |
| `CA107_AUT_VERST_ENTR` | identifiant interne (?) · automatique · amplification · deverrouillage | `par_frei` · `par_gesperrt` | Automatic seat adjustment after unlocking with remote control | Reglage automatique du siege apres deverrouillage par la telecommande |
| `CA107_AUT_VERST_ENT_T` | identifiant interne (?) · automatique · amplification · des- · T | `par_frei` · `par_gesperrt` | Automatic seat adjustment after unlocking and opening door | Reglage automatique du siege apres deverrouillage et ouverture de porte |
| `CAR_COLOR_CHAMP` | vehicule · couleur · champ | `nicht_aktiv` · `aktiv` | Display car color | Affichage de la couleur du vehicule |
| `CA_TMC` | acces confort (?) · info trafic | `nicht_aktiv` · `aktiv` | Conditional access for traffic message channel (TMC) | Informations trafic — canal complementaire. |
| `CBS` | entretien selon besoin (CBS) | `cbs_3` · `cbs_4` | Condition based service (CBS) version | Version de l'entretien selon besoin (CBS) |
| `CBS_STATUS` | entretien selon besoin (CBS) · etat | `nicht_aktiv` · `aktiv` | Condition based service (CBS) status | Etat de l'entretien selon besoin (CBS) |
| `CCC_E71` | systeme multimedia CCC · serie E71 | `nicht_aktiv` · `aktiv` | iDrive Professional Navigation (CCC) in E71 (X6) | Navigation Professional iDrive (CCC) sur E71 (X6) |
| `CC_LIST` | Check Control · liste | `nicht_aktiv` · `aktiv` | Check control notifications list | Liste des messages Check Control |
| `CDC` | [variante CDC] | `nicht_aktiv` · `aktiv` | CD changer | Changeur de CD |
| `CID_CCC` | ecran central · systeme multimedia CCC | `cid_high` · `cid_low` | Central information display (CID) | Ecran central (CID) |
| `CID_COLOUR` | ecran central · couleur | `table_1` · `table_2` · `table_3` · `table_4` | Central information display (CID) color | Couleur de l'ecran central (CID) |
| `CID_TYPE` | ecran central · type | `static` · `flap` | Central information display (CID) type | Type d'ecran central (CID) |
| `CLIMATE_WO_PREVIEW` | climatisation · ou · apercu | `nicht_aktiv` · `aktiv` | — | — |
| `CODIERUNG_ERFOLGT` | codage · effectue | `wert_01` · `codierung_erfolgt` | Coding completed | Codage termine |
| `CRUISE_CTRL_CCC` | regulateur de vitesse · commande · systeme multimedia CCC | `nicht_aktiv` · `aktiv` | Cruise Control | Regulateur de vitesse |
| `DAB` | radio numerique DAB | `nicht_aktiv` · `aktiv` | Digital audio broadcast (DAB) | Radio numerique DAB |
| `DAB_BAND_SETTINGS` | radio numerique DAB · bande · reglages | `nicht_aktiv` · `aktiv` | Digital audio broadcast (DAB) band selection | Choix de bande de la radio numerique DAB |
| `DAB_FM_SWITCH` | radio numerique DAB · FM · commutateur | `nicht_aktiv` · `aktiv` | Switch to FM radio with poor digital audio broadcast (DAB) reception quality | Bascule vers la FM quand la reception DAB se degrade |
| `DAB_FROM_FM` | radio numerique DAB · depuis · FM | `wert_01` · `iso` · `shifted_jis` · `china` · `default` | Switch back from FM radio to digital audio broadcast (DAB) when reception quality restores [time_s=data/4] | Retour de la FM vers le DAB quand la reception redevient bonne [duree s = valeur / 4] |
| `DAB_TO_FM` | radio numerique DAB · toit ouvrant · FM | `wert_01` | Switch to FM radio with poor digital audio broadcast (DAB) reception quality [time_s=data/4] | Bascule vers la FM quand la reception DAB se degrade [duree s = valeur / 4] |
| `DATUM_EINHEIT` | date · unite | `tt_mm_jj` · `mm_tt_jj` | Date format | Format de date |
| `DAY_RUN_LIGHT_CCC` | jour · marche · lumiere · systeme multimedia CCC | `nicht_aktiv` · `aktiv` | Daytime running lights | Feux de jour |
| `DEF_BALANCE_VOLUME` | defaut · balance · volume | `standard` | Default car/key memory for balance setting | Valeur par defaut de la memoire par cle pour la balance |
| `DEF_BASS_VOLUME` | defaut · graves · volume | `standard` | Default car/key memory for bass setting | Valeur par defaut de la memoire par cle pour les graves |
| `DEF_DATUM_EINHEIT` | defaut · date · unite | `tt_mm_jj` · `mm_tt_jj` | Default car/key memory for date format | Valeur par defaut de la memoire par cle pour le format de date |
| `DEF_DRUCK_EINHEIT` | defaut · pression · unite | `psi` · `bar` · `kp` · `default` | Default car/key memory for pressure unit | Valeur par defaut de la memoire par cle pour l'unite de pression |
| `DEF_EQUALIZER_VALUE` | defaut · egaliseur · valeur | `standard` | Default car/key memory for equalizer setting | Valeur par defaut de la memoire par cle pour l'egaliseur |
| `DEF_FADER_VOLUME` | defaut · balance avant-arriere · volume | `standard` | Default car/key memory for fader setting | Valeur par defaut de la memoire par cle pour la balance avant-arriere |
| `DEF_GAL_VALUE` | defaut · _GAL_ · valeur | `anhebung_sehr_stark` · `anhebung_stark` · `anhebung_mittel` · `anhebung_gering` · `anhebung_sehr_gering` · `keine_anhebung` | Default car/key memory for speed-dependent volume control (GAL) increase | Valeur par defaut de la memoire par cle pour l'adaptation du volume a la vitesse (GAL) |
| `DEF_LAST_SPEECH` | defaut · charge · reconnaissance vocale | `nicht_aktiv` · `deutsch` · `englisch` · `us` · `spanisch` · `italienisch` · `franzoesisch` · `flaemisch` … *(+8)* | Default car/key memory for language | Valeur par defaut de la memoire par cle pour la langue |
| `DEF_REVERB_VALUE` | defaut · reverberation · valeur | `standard` | Default car/key memory for reverb setting | Valeur par defaut de la memoire par cle pour la reverberation |
| `DEF_TEMPERATUR_EINHEIT` | defaut · temperature · unite | `grad_c` · `grad_f` | Default car/key memory for temperature unit | Valeur par defaut de la memoire par cle pour l'unite de temperature |
| `DEF_TREMBLE_VOLUME` | defaut · aigus · volume | `standard` | Default car/key memory for treble setting | Valeur par defaut de la memoire par cle pour les aigus |
| `DEF_VERBRAUCH` | defaut · consommation | `mpg` · `default` | Default car/key memory for fuel consumption unit | Valeur par defaut de la memoire par cle pour l'unite de consommation |
| `DEF_WEG_EINHEIT` | defaut · course · unite | `km` · `mls` | Default car/key memory for distance unit | Valeur par defaut de la memoire par cle pour l'unite de distance |
| `DEF_ZEIT_EINHEIT` | defaut · duree · unite | `standard` · `e60_stereo` · `e60_hifi` · `e60_top_hifi` · `e63_hifi` · `e63_top_hifi` · `e61_stereo` · `e61_hifi` … *(+17)* | Default car/key memory for time format | Valeur par defaut de la memoire par cle pour le format d'heure |
| `DIGITAL_AUDIO_SOURCE` | numerique · audio · source | — | Digital audio source | Source audio numerique |
| `DKG` | boite a double embrayage | `nicht_aktiv` · `aktiv` | Double clutch gearbox (DKG) | Boite a double embrayage (DKG) |
| `DRUCK_EINHEIT` | pression · unite | `psi` · `bar` · `kp` · `default` | Pressure unit | Unite de pression |
| `DSC` | controle de stabilite | `nicht_aktiv` · `aktiv` | Dynamic stability control (DSC) | Controle dynamique de stabilite (DSC) |
| `DVD_EJECT_LOCKED` | DVD · ejection · verrouille | `free` · `locked` | DVD ejecting locked | Ejection du DVD verrouillee |
| `DWS` | alarme antivol (?) | `nicht_aktiv` · `aktiv` | Antitheft alarm system (DWA) | Alarme antivol (DWA) |
| `EDC` | amortissement pilote | `nicht_aktiv` · `aktiv` | Electronic damper control (EDC) | Amortissement pilote (EDC) |
| `EINH_MASTER_KOMBI` | unite · cle maitre · combine | `nicht_aktiv` · `aktiv` | Units master in instrument cluster | Unites pilotees par le combine |
| `ELDRVSEAT` | siege conducteur electrique | `nicht_aktiv` · `aktiv` | Seat memory | Memoire de siege |
| `FOND_HIGH` | places arriere · haut | `nicht_aktiv` · `aktiv` | Rear monitor | Ecran arriere |
| `FOND_LOW` | places arriere · bas | `nicht_aktiv` · `aktiv` | Rear headphones | Casques arriere |
| `FOND_MID` | places arriere · milieu | `nicht_aktiv` · `aktiv` | Rear seat entertainment (RSE) with monitor | Divertissement aux places arriere (RSE) avec ecran |
| `FULLWORD_RECOG` | mot complet · reconnaissance | `nicht_aktiv` · `aktiv` | Full-word recognition | Reconnaissance de mots entiers |
| `GAL_VALUE` | _GAL_ · valeur | `anhebung_sehr_stark` · `anhebung_stark` · `anhebung_mittel` · `anhebung_gering` · `anhebung_sehr_gering` · `keine_anhebung` | Speed-dependent volume control (GAL) increase | Adaptation du volume a la vitesse (GAL) |
| `HDC` | assistance en descente | `nicht_aktiv` · `aktiv` | Hill descent control (HDC) | Assistance en descente (HDC) |
| `HELPNET` | appel d'urgence | `nicht_aktiv` · `aktiv` | — | — |
| `HIP` | interface (?) | `nicht_aktiv` · `aktiv` | — | — |
| `HUD` | affichage tete haute | `nicht_aktiv` · `aktiv` | Head-up display (HUD) | Affichage tete haute (HUD) |
| `HUD_M` | affichage tete haute · M | `nicht_aktiv` · `aktiv` | BMW M head-up display (HUD) shows tachometer (RPM) and current gear (linked to [M_SPORT_HUD] and [MDRIVE]) | L'affichage tete haute M montre le compte-tours et le rapport engage (lie a M_SPORT_HUD et MDRIVE) |
| `IBOC_CCC` | radio numerique HD · systeme multimedia CCC | `nicht_aktiv` · `aktiv` | In-band on-channel (IBOC) digital radio (HD radio) | Radio numerique IBOC (HD Radio) |
| `JAPAN` | Japon | `nicht_aktiv` · `aktiv` | Japan | Japon |
| `JAP_SIDEVIEW_CAM` | Japon · vue laterale · camera | `nicht_aktiv` · `aktiv` | Japanese side-view camera | Camera de vision laterale, marche japonais |
| `K2407_PDC_MONITORBILD` | [identifiant BMW K2407] · radars · image ecran | `fkt_frei` · `fkt_gesperrt` | Visual park distance control (PDC) | Affichage visuel des radars de stationnement (PDC) |
| `K410D_EINHEITEN_WEG` | [identifiant BMW K410D] · unites · course | `fkt_frei` · `fkt_gesperrt` | Distance unit | Unite de distance |
| `K4110_VERBR_MPG_UK` | [identifiant BMW K4110] · consommation · miles par gallon · Royaume-Uni | `par_frei` · `par_gesperrt` | Fuel consumption in miles per gallon (United Kingdom) | Consommation en miles par gallon (Royaume-Uni) |
| `K4110_VERBR_MPG_US` | [identifiant BMW K4110] · consommation · miles par gallon · marche americain | `par_frei` · `par_gesperrt` | Fuel consumption in miles per gallon (USA) | Consommation en miles par gallon (Etats-Unis) |
| `K4113_ZEIT_STD_DARST` | [identifiant BMW K4113] · duree · standard · representation | `fkt_frei` · `fkt_gesperrt` | Time format | Format d'heure |
| `K4116_EINH_TEMPERATUR` | [identifiant BMW K4116] · unite · temperature | `fkt_frei` · `fkt_gesperrt` | Temperature unit | Unite de temperature |
| `K4119_DRUCK_BAR` | [identifiant BMW K4119] · pression · bar | `par_frei` · `par_gesperrt` | Pressure in bar | Pression en bars |
| `K4119_DRUCK_KPA` | [identifiant BMW K4119] · pression · kPa | `par_frei` · `par_gesperrt` | Pressure in kilopascal | Pression en kilopascals |
| `K4119_DRUCK_PSI` | [identifiant BMW K4119] · pression · psi (?) | `par_frei` · `par_gesperrt` | Pressure in pounds per square inch | Pression en psi |
| `K430C_DATUM_FORMAT` | [identifiant BMW K430C] · date · format | `fkt_frei` · `fkt_gesperrt` | Date format | Format de date |
| `K4310_ANZ_SPRACHE_1` | [identifiant BMW K4310] · nombre · langue · 1 | `par_frei` · `par_gesperrt` | Display language selection 1 | Choix de langue d'affichage 1 |
| `K4310_ANZ_SPRACHE_2` | [identifiant BMW K4310] · nombre · langue · 2 | `par_frei` · `par_gesperrt` | Display language selection 2 | Choix de langue d'affichage 2 |
| `K4310_ANZ_SPRACHE_3` | [identifiant BMW K4310] · nombre · langue · 3 | `par_frei` · `par_gesperrt` | Display language selection 3 | Choix de langue d'affichage 3 |
| `K4328_BLINKEN_TIPPBET` | [identifiant BMW K4328] · clignotement · actionnement par impulsion | `fkt_frei` · `fkt_gesperrt` | One-touch turn signal functionality | Clignotants a impulsion unique (mode confort) |
| `K4D03_ANZEIGE_ACC` | [identifiant BMW K4D03] · affichage · regulateur adaptatif | `par_frei` · `par_gesperrt` | Display active cruise control (ACC) | Affichage du regulateur actif (ACC) |
| `K4D03_ANZEIGE_CHECKCONTRL` | [identifiant BMW K4D03] · affichage · Check Control | `par_frei` · `par_gesperrt` | Display check control notifications | Affichage des messages Check Control |
| `K4D03_ANZEIGE_GESCHWNDGKT` | [identifiant BMW K4D03] · affichage · vitesse | `par_frei` · `par_gesperrt` | Display speed | Affichage de la vitesse |
| `K4D03_ANZEIGE_NAVIGATION` | [identifiant BMW K4D03] · affichage · navigation | `par_frei` · `par_gesperrt` | Display navigation | Affichage de la navigation |
| `K4D03_ANZEIGE_TEMPOMAT` | [identifiant BMW K4D03] · affichage · regulateur de vitesse | `par_frei` · `par_gesperrt` | Display cruise control | Affichage du regulateur de vitesse |
| `K5303_GAL_GERING` | [identifiant BMW K5303] · _GAL_ · faible | `par_frei` · `par_gesperrt` | Low increase for speed-dependent volume control (GAL) | Adaptation faible du volume a la vitesse (GAL) |
| `K5303_GAL_KEINE_ANHEB` | [identifiant BMW K5303] · _GAL_ · aucun · levee | `par_frei` · `par_gesperrt` | No increase for speed-dependent volume control (GAL) | Aucune adaptation du volume a la vitesse (GAL) |
| `K5303_GAL_MITTEL` | [identifiant BMW K5303] · _GAL_ · moyen | `par_frei` · `par_gesperrt` | Medium increase for speed-dependent volume control (GAL) | Adaptation moyenne du volume a la vitesse (GAL) |
| `K5303_GAL_SEHR_GERING` | [identifiant BMW K5303] · _GAL_ · tres · faible | `par_frei` · `par_gesperrt` | Very low increase for speed-dependent volume control (GAL) | Adaptation tres faible du volume a la vitesse (GAL) |
| `K5303_GAL_SEHR_STARK` | [identifiant BMW K5303] · _GAL_ · tres · fort | `par_frei` · `par_gesperrt` | Very strong increase for speed-dependent volume control (GAL) | Adaptation tres forte du volume a la vitesse (GAL) |
| `K5303_GAL_STARK` | [identifiant BMW K5303] · _GAL_ · fort | `par_frei` · `par_gesperrt` | Strong increase for speed-dependent volume control (GAL) | Adaptation forte du volume a la vitesse (GAL) |
| `K5305_KLANG_BALANCE` | [identifiant BMW K5305] · sonorite · balance | `par_frei` · `par_gesperrt` | Balance setting | Reglage de balance |
| `K5305_KLANG_BASS` | [identifiant BMW K5305] · sonorite · graves | `par_frei` · `par_gesperrt` | Bass setting | Reglage des graves |
| `K5305_KLANG_EQULIZER` | [identifiant BMW K5305] · sonorite · egaliseur | `par_frei` · `par_gesperrt` | Equalizer setting | Reglage de l'egaliseur |
| `K5305_KLANG_FADER` | [identifiant BMW K5305] · sonorite · balance avant-arriere | `par_frei` · `par_gesperrt` | Fader setting | Reglage de la balance avant-arriere |
| `K5305_KLANG_REVERB` | [identifiant BMW K5305] · sonorite · reverberation | `par_frei` · `par_gesperrt` | Reverb setting | Reglage de la reverberation |
| `K5305_KLANG_TREBLE` | [identifiant BMW K5305] · sonorite · aigus | `par_frei` · `par_gesperrt` | Treble setting | Reglage des aigus |
| `K531A_LAUTST_NAV` | [identifiant BMW K531A] · volume sonore · navigation (?) | `par_frei` · `par_gesperrt` | Volume for navigation announcement | Volume des annonces de navigation |
| `K531A_LAUTST_PDC` | [identifiant BMW K531A] · volume sonore · radars | `par_frei` · `par_gesperrt` | Volume for acoustic park distance control (PDC) signal | Volume du signal sonore des radars de stationnement |
| `K531A_LAUTST_SVS` | [identifiant BMW K531A] · volume sonore · surveillance (?) | `par_frei` · `par_gesperrt` | Volume for speech announcement | Volume des annonces vocales |
| `K531A_LAUTST_TELEFON` | [identifiant BMW K531A] · volume sonore · telephone | `par_frei` · `par_gesperrt` | Volume for telephone call | Volume des appels telephoniques |
| `K531A_LAUTST_VERKEHR` | [identifiant BMW K531A] · volume sonore · circulation | `par_frei` · `par_gesperrt` | Volume for traffic announcement | Volume des annonces trafic |
| `K7504_HEIMLEUCHTEN_150S` | [identifiant BMW K7504] · eclairage d'accompagnement · 150 s | `par_frei` · `par_gesperrt` | Follow-me-home lights 150 seconds | Eclairage d'accompagnement 150 secondes |
| `K7504_HEIMLEUCHTEN_240S` | [identifiant BMW K7504] · eclairage d'accompagnement · 240 s | `par_frei` · `par_gesperrt` | Follow-me-home lights 240 seconds | Eclairage d'accompagnement 240 secondes |
| `K7504_HEIMLEUCHTEN_40S` | [identifiant BMW K7504] · eclairage d'accompagnement · 40 s | `par_frei` · `par_gesperrt` | Follow-me-home lights 40 seconds | Eclairage d'accompagnement 40 secondes |
| `K7504_HEIMLEUCHTEN_90S` | [identifiant BMW K7504] · eclairage d'accompagnement · 90 s | `par_frei` · `par_gesperrt` | Follow-me-home lights 90 seconds | Eclairage d'accompagnement 90 secondes |
| `K7504_HEIMLEUCHTEN_AUS` | [identifiant BMW K7504] · eclairage d'accompagnement · arret | `par_frei` · `par_gesperrt` | Follow-me-home lights off | Eclairage d'accompagnement desactive |
| `K8116_KLAPPENAUTOMATIK` | [identifiant BMW K8116] · automatisme de volets | `fkt_frei` · `fkt_gesperrt` | Climate control automatic blower speed and air distribution control after switching on ignition | Gestion automatique du pulseur et de la repartition d'air a la mise du contact |
| `K8119_PRG_AC` | [identifiant BMW K8119] · programme · climatisation (?) | `par_frei` · `par_gesperrt` | Climate control air conditioning compressor setting | Reglage du compresseur de climatisation |
| `K8119_PRG_AUTOMATIK` | [identifiant BMW K8119] · programme · automatique | `par_frei` · `par_gesperrt` | Climate control automatic mode setting | Reglage du mode automatique de la climatisation |
| `K8119_PRG_GEBL_AKTUELL` | [identifiant BMW K8119] · programme · pulseur · actuel | `par_frei` · `par_gesperrt` | Climate control blower speed setting | Reglage de la vitesse du pulseur |
| `K8119_PRG_LUFTVERTEILUNG` | [identifiant BMW K8119] · programme · repartition d'air | `par_frei` · `par_gesperrt` | Climate control air distribution setting | Reglage de la repartition d'air |
| `K8119_PRG_SCHICHTUNG` | [identifiant BMW K8119] · programme · stratification | `par_frei` · `par_gesperrt` | Climate control stratification (hot/cold mixing flap) setting | Reglage de la stratification (volet de melange chaud/froid) |
| `K8125_KLAPPENPROGRAMM_BO` | [identifiant BMW K8125] · programme de volets · carrosserie (?) | `fkt_frei` · `fkt_gesperrt` | Climate control ventilation program | Programme de ventilation de la climatisation |
| `K8127_KLAPPENPROGRAMM_BO` | [identifiant BMW K8127] · programme de volets · carrosserie (?) | `fkt_frei` · `fkt_gesperrt` | Climate control defrost and footwell flaps closed in maximum cooling mode | Volets de degivrage et de plancher fermes en refroidissement maximal |
| `K8129_ZWAKO_AC_EIN_AUTO` | [identifiant BMW K8129] · composant additionnel (?) · climatisation (?) · marche · automatique | `fkt_frei` · `fkt_gesperrt` | Climate control air conditioning compressor on in automatic mode | Compresseur de climatisation actif en mode automatique |
| `K8313_GEBLAESE_0` | [identifiant BMW K8313] · pulseur · 0 | `par_frei` · `par_gesperrt` | Climate control blower speed offset 0% | Decalage de vitesse du pulseur : 0 % |
| `K8313_GEBLAESE_M10` | [identifiant BMW K8313] · pulseur · parametre M10 (?) | `par_frei` · `par_gesperrt` | Climate control blower speed offset -10% | Decalage de vitesse du pulseur : -10 % |
| `K8313_GEBLAESE_P10` | [identifiant BMW K8313] · pulseur · parametre 10 | `par_frei` · `par_gesperrt` | Climate control blower speed offset +10% | Decalage de vitesse du pulseur : +10 % |
| `K8601_SOLLWERT_0` | [identifiant BMW K8601] · consigne · 0 | `par_frei` · `par_gesperrt` | Climate control set temperature offset 0 �C | Decalage de temperature de consigne : 0 C |
| `K8601_SOLLWERT_M1` | [identifiant BMW K8601] · consigne · parametre M1 | `par_frei` · `par_gesperrt` | Climate control set temperature offset -1 �C | Decalage de temperature de consigne : -1 C |
| `K8601_SOLLWERT_M2` | [identifiant BMW K8601] · consigne · parametre M2 | `par_frei` · `par_gesperrt` | Climate control set temperature offset -2 �C | Decalage de temperature de consigne : -2 C |
| `K8601_SOLLWERT_M3` | [identifiant BMW K8601] · consigne · parametre M3 | `par_frei` · `par_gesperrt` | Climate control set temperature offset -3 �C | Decalage de temperature de consigne : -3 C |
| `K8601_SOLLWERT_P1` | [identifiant BMW K8601] · consigne · parametre 1 | `par_frei` · `par_gesperrt` | Climate control set temperature offset +1 �C | Decalage de temperature de consigne : +1 C |
| `K8601_SOLLWERT_P2` | [identifiant BMW K8601] · consigne · parametre 2 | `par_frei` · `par_gesperrt` | Climate control set temperature offset +2 �C | Decalage de temperature de consigne : +2 C |
| `K8601_SOLLWERT_P3` | [identifiant BMW K8601] · consigne · parametre 3 | `par_frei` · `par_gesperrt` | Climate control set temperature offset +3 �C | Decalage de temperature de consigne : +3 C |
| `K930E_ZV_VER_N_XMIN_FB` | [identifiant BMW K930E] · verrouillage centralise · version · N · minimum · telecommande | `fkt_frei` · `fkt_gesperrt` | Automatically re-lock after being unlocked for a certain time without opening a door | Reverrouillage automatique apres un certain temps sans ouverture de porte |
| `K931D_SCHEIBENABSENKUNG` | [identifiant BMW K931D] · abaissement de vitre | `par_frei` · `par_gesperrt` | Pressing remote control unlock button once unlocks all doors, twice opens driver power window (easy entry for frameless coup� door in narrow spaces) | Une pression ouvre toutes les portes, deux pressions descendent la vitre conducteur (acces facilite pour portes sans encadrement en espace etroit) |
| `K931D_ZV_ALLE_TUEREN` | [identifiant BMW K931D] · verrouillage centralise · tous · portes | `par_frei` · `par_gesperrt` | Pressing remote control unlock button once unlocks all doors | Une seule pression sur le bouton de deverrouillage ouvre toutes les portes |
| `K931D_ZV_SELEKTIV` | [identifiant BMW K931D] · verrouillage centralise · selectif | `par_frei` · `par_gesperrt` | Pressing remote control unlock button once unlocks only driver door, twice unlocks all doors | Une pression sur le bouton de deverrouillage n'ouvre que la porte conducteur, deux pressions ouvrent toutes les portes |
| `K9416_QUITT_AKUST_ENTSCH` | [identifiant BMW K9416] · acquittement sonore · sonore · desarmement | `fkt_frei` · `fkt_gesperrt` | Acoustic unlocking acknowledge (two alarm beeps) | Acquittement sonore au deverrouillage (deux bips d'alarme) |
| `K9901_HUPE` | [identifiant BMW K9901] · avertisseur sonore | `par_frei` · `par_gesperrt` | Programmable button for horn | Bouton programmable : avertisseur sonore |
| `K9901_KOMFORT_PARKBREMSE` | [identifiant BMW K9901] · confort · frein de stationnement | `par_frei` · `par_gesperrt` | Programmable button for automatic parking brake | Bouton programmable : frein de stationnement automatique |
| `K9901_LENKRADHEIZUNG` | [identifiant BMW K9901] · volant chauffant | `par_frei` · `par_gesperrt` | Programmable button for steering wheel heating | Bouton programmable : volant chauffant |
| `K9901_MONITOR_EIN` | [identifiant BMW K9901] · surveillance · marche | `par_frei` · `par_gesperrt` | Programmable button for display on | Bouton programmable : allumage de l'ecran |
| `K9901_MUTE_EIN` | [identifiant BMW K9901] · coupure du son · marche | `par_frei` · `par_gesperrt` | Programmable button for audio muting | Bouton programmable : coupure du son |
| `K9901_NAV_LETZER_HINWEIS` | [identifiant BMW K9901] · navigation (?) · dernier · indication | `par_frei` · `par_gesperrt` | Programmable button for recall last navigation destination | Bouton programmable : rappel de la derniere destination |
| `K9901_SWR` | [identifiant BMW K9901] · correcteur de site (?) | `par_frei` · `par_gesperrt` | Programmable button for headlight cleaning | Bouton programmable : lave-phares |
| `K9901_UMLUFT_EIN` | [identifiant BMW K9901] · recyclage d'air · marche | `par_frei` · `par_gesperrt` | Programmable button for recirculated air | Bouton programmable : recyclage d'air |
| `K9901_WARNBLINKEN` | [identifiant BMW K9901] · feux de detresse | `par_frei` · `par_gesperrt` | Programmable button for hazard lights | Bouton programmable : feux de detresse |
| `KANA_WRITER` | kana · saisie | `nicht_aktiv` · `aktiv` | Support for Kana (Japanese) writing system | Prise en charge de l'ecriture kana (japonais) |
| `KLIMA_SG_ART_CCC` | climatisation · calculateur · type · systeme multimedia CCC | `basis` · `high` · `low_temp` · `low_vent` · `nicht_aktiv` | Climate control type | Type de climatisation |
| `LAENDERVARIANTE_TUNER` | variante pays · tuner | `ece` · `us` · `japan` · `oceanien` · `no_area` | Country variant tuner | Variante pays du tuner |
| `LAST_BALANCE_VOLUME` | charge · balance · volume | `standard` | Last setting for balance | Derniere valeur de balance |
| `LAST_BASS_VOLUME` | charge · graves · volume | `standard` | Last setting for bass | Derniere valeur des graves |
| `LAST_EQUALIZER_VALUE` | charge · egaliseur · valeur | `standard` | Last setting for equalizer | Derniere valeur de l'egaliseur |
| `LAST_FADER_VOLUME` | charge · balance avant-arriere · volume | `standard` | Last setting for fader | Derniere valeur de la balance avant-arriere |
| `LAST_REVERB_VALUE` | charge · reverberation · valeur | `standard` | Last setting for reverb | Derniere valeur de la reverberation |
| `LAST_SPEECH` | charge · reconnaissance vocale | `nicht_aktiv` · `deutsch` · `englisch` · `us` · `spanisch` · `italienisch` · `franzoesisch` · `flaemisch` … *(+7)* | Last language | Derniere langue utilisee |
| `LAST_TREMBLE_VOLUME` | charge · aigus · volume | `standard` | Last setting for treble | Derniere valeur des aigus |
| `LEGAL_DISCL` | mention legale · avertissement | `nicht_aktiv` · `aktiv` | Display legal disclaimer | Avertissement legal affiche au demarrage. |
| `LEGAL_DISCL_TIMOUT` | mention legale · avertissement · delai d'attente | `standard` · `wert_00` · `aus` | Legal disclaimer display timeout | Duree d'affichage de l'avertissement legal |
| `MAX_PHONE_VOL` | maximum · telephone · volume | `standard` · `mulf` · `japan` · `japan_mulf` · `e60_stereo` · `e60_hifi` · `e60_top_hifi` · `e61_stereo` … *(+31)* | Maximum volume for telephone | Volume maximal du telephone |
| `MDRIVE` | M Drive | `nicht_aktiv` · `aktiv` | BMW M Drive (linked to [HUD_M] and [M_SPORT_HUD]) | BMW M Drive (lie a HUD_M et M_SPORT_HUD) |
| `MDRIVE_ALBV` | M Drive · reglage de site | `nicht_aktiv` · `aktiv` | BMW M Drive active seat backrest width adjustment | M Drive : reglage actif de largeur du dossier |
| `MDRIVE_LOW_CCC` | M Drive · bas · systeme multimedia CCC | `nicht_aktiv` · `aktiv` | BMW M Drive low | M Drive : niveau bas |
| `MDRIVE_MENU` | M Drive · menu | `nicht_aktiv` · `aktiv` | — | — |
| `MDRIVE_SMG` | M Drive · boite sequentielle | `nicht_aktiv` · `aktiv` | BMW M Drive sequential manual gearbox (SMG) | M Drive : boite sequentielle (SMG) |
| `MFL_CLEAN` | volant multifonction · propre | `nicht_aktiv` · `aktiv` | Headlight cleaning with multifunction steering wheel button | Lave-phares via une touche du volant multifonction |
| `MFL_HAZARD` | volant multifonction · feux de detresse | `nicht_aktiv` · `aktiv` | Hazard lights with multifunction steering wheel button | Feux de detresse via une touche du volant multifonction |
| `MFL_HORN` | volant multifonction · avertisseur sonore | `nicht_aktiv` · `aktiv` | Horn with multifunction steering wheel button | Avertisseur sonore via une touche du volant multifonction |
| `MFL_MENU` | volant multifonction · menu | `nicht_aktiv` · `aktiv` | Disable diamond button customization in multifunction steering wheel menu (BMW M button replaces diamond button) | Desactive la personnalisation de la touche losange au volant (la touche M la remplace) |
| `MMC` | carte memoire | `nicht_aktiv` · `aktiv` | Multimedia changer (MMC) | Changeur multimedia (MMC) |
| `MM_DEVICE` | module (?) · peripherique | `nicht_aktiv` · `aktiv` | External multimedia device | Peripherique multimedia externe |
| `MM_DEVICE2` | module (?) · peripherique 2 (?) | `nicht_aktiv` · `aktiv` | External multimedia device 2 | Peripherique multimedia externe 2 |
| `MP3_CCC` | MP3 · systeme multimedia CCC | `nicht_aktiv` · `intern` · `extern` · `full` · `cd_only` · `dvd_only` · `cd_extern` · `dvd_extern` | MP3 support | Prise en charge du MP3 |
| `MULTIPARTY` | conference | `nicht_aktiv` · `aktiv` | Conference calls | Conferences telephoniques |
| `NAVIGATION` | navigation | `nicht_aktiv` · `aktiv` | Navigation | Navigation |
| `NAVIGATION_TYPE` | navigation · type | `wert_01` · `navi_sv` · `navi_jp` · `navi_exbox` | Navigation type | Type de navigation |
| `NAVI_PFEIL` | navigation · fleche | `nicht_aktiv` · `aktiv` | Navigation with arrows | Navigation par fleches |
| `NEXT_SOURCE` | suivant · source | `nicht_aktiv` · `aktiv` | Select next source functionality | Fonction de passage a la source suivante |
| `NIVI` | correcteur d'assiette | `nicht_aktiv` · `aktiv` | Night vision | Vision nocturne |
| `NIVI_DISPLAY_OPTIONS` | correcteur d'assiette · affichage · options | `only_mainwindow` · `only_fullscreen` · `main_or_fullscreen` | Night vision display options | Options d'affichage de la vision nocturne |
| `NOTEPAD` | bloc-notes | `nicht_aktiv` · `aktiv` | Notepad entry in iDrive telephone menu | Entree bloc-notes dans le menu telephone de l'iDrive |
| `NO_DETOUR` | non · deviation | `nicht_aktiv` · `aktiv` | Hide navigation detour function | Masquer la fonction de deviation de la navigation |
| `OIL_LEVEL` | huile · niveau | `nicht_aktiv` · `aktiv` | Engine oil level | Affichage du niveau d'huile dans les menus iDrive. |
| `OLD_TELEMAT_STRUCT` | ancien · telematique · structure | `nicht_aktiv` · `aktiv` | Old telematics structure | Ancienne architecture telematique |
| `ONLINE` | en ligne | `nicht_aktiv` · `aktiv` | BMW online platform | Plateforme BMW Online |
| `ONLINE_CALLSTRATEGY` | en ligne · strategie d'appel | `no_csd` · `use_csd` · `use_paketdata` · `use_csd_paketdata` · `use_telematic_sim` | Call strategy for BMW online platform | Strategie d'appel de la plateforme BMW Online |
| `ONLINE_DAR_TYPE_B` | en ligne · representation · type · B | `dar_us_1` · `dar_us` · `dar_us_2` · `dar_aus` · `dar_aus_gprs` · `dar_ece` · `dar_ece_gprs` · `dar_ece_1` · `dar_ece_2` | — | — |
| `ONLINE_DISABLE_CONTENT` | en ligne · desactivation · contenu | `content_enable` · `content_disable_vmax` · `content_disabl_eng_brake` · `content_disable_engine` · `content_disable_brake` | Disabling conditions for BMW online platform (linked to [ONLINE_DISABLE_VMAX]) | Conditions de desactivation de la plateforme BMW Online (lie a ONLINE_DISABLE_VMAX) |
| `ONLINE_DISABLE_VMAX` | en ligne · desactivation · vitesse maximale | `disable_us` · `disable_aus` · `standard` | Maximum speed for BMW online platform [speed_m/s=data] (linked to [ONLINE_DISABLE_CONTENT]) | Vitesse maximale d'utilisation de BMW Online [vitesse m/s = valeur] (lie a ONLINE_DISABLE_CONTENT) |
| `PDC` | radars | `nicht_aktiv` · `aktiv` | Park distance control (PDC) | Radars de stationnement (PDC) |
| `PDC_SIGNAL` | radars · signal | `pdc_no` · `pdc_rear` · `pdc_front` · `pdc_front_rear` | Park distance control (PDC) acoustic signal generation | Generation du signal sonore des radars de stationnement |
| `PDC_TYPE` | radars · type | `rear` · `full` | Park distance control (PDC) type | Type de radars de stationnement (PDC) |
| `PHONE_VERSION` | telephone · version | — | Telephone type | Type de telephone |
| `PIA` | interface d'affichage (?) | `ckm` · `pia_low` | Personal profile manager | Gestionnaire de profils personnels |
| `RAIN_LIGHT_SENSOR` | pluie · lumiere · capteur | `nicht_aktiv` · `aktiv` | Control light and rain sensor in iDrive (additional changes required to [C750A_SCHWELLWERT_EMPFIN], [C750A_SCHWELLWERT_NORMAL], and [C750A_SCHWELLWERT_SEHR_E] from [par_gesperrt] to [par_frei]) | Reglage de la sensibilite pluie/lumiere depuis l'iDrive. |
| `RDC_CCC` | pression des pneus · systeme multimedia CCC | `nicht_aktiv` · `aktiv` | Tire pressure control (RDC) | Controle de pression des pneus (RDC) |
| `RDS_AF_FUNKTION_CCC` | RDS · frequences alternatives · fonction · systeme multimedia CCC | `rds_aus` · `rds_ein_af_aus` · `rds_ein_af_manuell` · `rds_ein_af_automatisch` | Radio data system (RDS) alternative frequencies (AF) (switch to alternative frequency for the same station when the signal becomes to weak) | Frequences alternatives (AF) du RDS : bascule sur une autre frequence de la meme station quand le signal faiblit |
| `REAR_HEAD_UNIT` | arriere · tete · unite | `nicht_aktiv` · `aktiv` | — | — |
| `REVERSE_GONG` | inversion · gong | `nicht_aktiv` · `aktiv` | Reverse gear warning | Alerte de marche arriere engagee |
| `REV_INPUT` | revision · entree | `nicht_aktiv` · `aktiv` | Reversed navigation input [nicht_aktiv: first place then street, aktiv: first street then place] | Ordre de saisie de la navigation [nicht_aktiv : ville puis rue, aktiv : rue puis ville] |
| `ROLLER_COVER` | store · couvercle | `nicht_aktiv` · `aktiv` | Automatic cargo bay roll-up cover | Cache-bagages enroulable automatique |
| `RSEMIDPLUS_AVAILABLE` | divertissement arriere (?) · disponible | `nicht_aktiv` · `aktiv` | — | — |
| `RVC` | camera de recul | `nicht_aktiv` · `aktiv` | Rear-view camera | Camera de recul |
| `RVC_CONNECTED_TO_VM` | camera de recul · connecte · toit ouvrant · gestion du vehicule (?) | `nicht_aktiv` · `aktiv` | Rear-view camera connected to video/television module | Camera de recul raccordee au module video/television |
| `RVC_DISPLAY_OPTIONS` | camera de recul · affichage · options | `only_mainwindow` · `only_fullscreen` · `main_or_fullscreen` | Rear-view camera display options | Options d'affichage de la camera de recul |
| `SDARS` | radio satellite | `nicht_aktiv` · `aktiv` | Satellite digital audio radio service (SDARS) | Radio numerique par satellite (SDARS) |
| `SIM` | carte SIM | `nicht_aktiv` · `aktiv` | — | — |
| `SITZHEIZUNG_APP_9` | siege chauffant · application · 9 | `nicht_aktiv` · `aktiv` | Seat heating | Sieges chauffants |
| `SMS` | SMS | `nicht_aktiv` · `aktiv` | SMS messaging | Messagerie SMS |
| `SPRECHER_AKTIVE_SPRACHE` | haut-parleur · actif · langue | `sprecher_maennlich` · `sprecher_weiblich` | Voice for language 1 | Voix de la langue 1 |
| `SPRECHER_ALTERNATIVE_2` | haut-parleur · alternative · 2 | `sprecher_maennlich` · `sprecher_weiblich` | Voice for language 2 | Voix de la langue 2 |
| `SPRECHER_ALTERNATIVE_3` | haut-parleur · alternative · 3 | `sprecher_maennlich` · `sprecher_weiblich` | Voice for language 3 | Voix de la langue 3 |
| `STANDHEIZUNG` | chauffage stationnaire | `nicht_aktiv` · `aktiv` | Independent park heating (note: aktiv/nicht_aktiv inverted for chassis E31) | Chauffage stationnaire (attention : aktiv/nicht_aktiv sont inverses sur E31) |
| `STANDLUEFTUNG_CCC` | ventilation a l'arret · systeme multimedia CCC | `nicht_aktiv` · `aktiv` | Independent park ventilation | Ventilation stationnaire |
| `SVR` | surveillance (?) | `nicht_aktiv` · `aktiv` | Stolen vehicle recovery (SVR) | Recuperation de vehicule vole (SVR) |
| `SVS` | surveillance (?) | `nicht_aktiv` · `aktiv` | Voice recognition | Reconnaissance vocale |
| `SVS_MICROPHONE_PATH` | surveillance (?) · microphone · chemin | `most` · `analog` | Microphone connection for voice recognition | Raccordement du microphone pour la reconnaissance vocale |
| `TDA_ASC_STATUS` | _TDA_ · antipatinage · etat | `nicht_aktiv` · `aktiv` | TeleServices automatic service call | TeleServices : appel d'entretien automatique |
| `TDA_BDA_STATUS` | _TDA_ · notice d'utilisation · etat | `nicht_aktiv` · `aktiv` | TeleServices BMW Assist | TeleServices : BMW Assist |
| `TDA_CFG_AUTO_TRIG` | _TDA_ · configuration · automatique · declenchement | `nicht_aktiv` · `aktiv` | — | — |
| `TDA_CFG_CUST_MOBILE_USE` | _TDA_ · configuration · personnalise · mobile · utilisation | `nicht_aktiv` · `aktiv` | — | — |
| `TDA_CFG_HLC_DATA` | _TDA_ · configuration · commande d'eclairage (?) · donnees | `nicht_aktiv` · `aktiv` | — | — |
| `TDA_CFG_IND_PDM_GR_NBR` | _TDA_ · configuration · indicateur · module de puissance (?) · groupe (?) · nombre | `file0` · `file1` · `file2` · `file3` · `file4` · `file5` · `file6` · `file7` | — | — |
| `TDA_CFG_SER_ERR_SINK` | _TDA_ · configuration · serie (?) · erreur · abaissement | `nicht_aktiv` · `aktiv` | — | — |
| `TDA_CFG_VEHICLE_SER` | _TDA_ · configuration · vehicule · serie (?) | `nicht_aktiv` · `aktiv` | — | — |
| `TDA_DEACTIVATE_STATUS` | _TDA_ · desactivation · etat | `nicht_aktiv` · `aktiv` | Option to disable TeleServices in iDrive | Possibilite de desactiver les TeleServices depuis l'iDrive |
| `TDA_HLC_STATUS` | _TDA_ · commande d'eclairage (?) · etat | `nicht_aktiv` · `aktiv` | TeleServices hotline call | TeleServices : appel de l'assistance |
| `TDA_MSC_STATUS` | _TDA_ · commande moteur (?) · etat | `nicht_aktiv` · `aktiv` | TeleServices manual service call | TeleServices : appel d'entretien manuel |
| `TDA_SERVICE_STATUS` | _TDA_ · entretien · etat | `nicht_aktiv` · `aktiv` | TeleServices | TeleServices |
| `TDA_TDC_STATUS` | _TDA_ · point mort haut (?) · etat | `nicht_aktiv` · `aktiv` | TeleServices remote diagnose | TeleServices : diagnostic a distance |
| `TELEFON_TYPE_C08` | telephone · type · [variante C08] | `no_phone` · `ece_phone` · `business_phone` · `prof_phone` · `japan_phone` · `umts_phone` · `mulf_phone` · `va_tpp` · `japan_mulf` | Telephone type | Type de telephone |
| `TELESERVICE` | teleservice | `nicht_aktiv` · `aktiv` | BMW TeleServices | BMW TeleServices |
| `TEMPERATUR_EINHEIT` | temperature · unite | `grad_c` · `grad_f` | Temperature unit | Unite de temperature |
| `TLC` | commande d'eclairage (?) | `nicht_aktiv` · `aktiv` | Lane departure warning (TLC) | Alerte de franchissement de ligne (TLC) |
| `TMC` | info trafic | `nicht_aktiv` · `aktiv` | Radio data system (RDS) traffic message channel (TMC) | Reception des informations trafic. |
| `TMC_REGIONAL` | info trafic · regional | `nicht_aktiv` · `aktiv` | Get traffic message channel (TMC) information only from stations with message geographic scope (MGS) "Urban" or "Regional" | N'accepter les infos trafic (TMC) que des stations de portee geographique urbaine ou regionale |
| `TOP_AMPL_TYPE` | haut · amplitude · type | `logic_7` · `prologic_2` · `nicht_aktiv` | Top hifi amplifier type | Type d'amplificateur HiFi Professional |
| `TOWBAR` | attelage | `nicht_aktiv` · `aktiv` | Focus rear-view camera on trailer tow hitch | Cadrage de la camera de recul sur la boule d'attelage |
| `TP` | point de test (?) | `nicht_aktiv` · `aktiv` | Radio data system (RDS) traffic programme (TP) | Programme trafic (TP) du RDS |
| `TP_FUNKTION` | point de test (?) · fonction | `nicht_aktiv` · `tp` | Radio data system (RDS) traffic programme (TP) | Programme trafic (TP) du RDS |
| `TRF` | trafic | `nicht_aktiv` · `aktiv` | Traffic channel in source selection | Canal trafic propose dans le choix des sources |
| `TV_FRONT_ENABLED` | television · avant · active | `nicht_aktiv` · `aktiv` | — | — |
| `TWO_MICROPHONES` | deux · microphones | `wert_02` · `wert_01` | Two microphones [wert_01: 1 microphone, wert_02: 2 microphones] | Deux microphones [wert_01 : 1 micro, wert_02 : 2 micros] |
| `TYP_LENKUNG` | type · direction | `linkslenker` · `rechtslenker` | Steering type | Type de direction |
| `UNIT_FEET` | unite · pieds | `nicht_aktiv` · `aktiv` | Display short distances in feet | Affichage des courtes distances en pieds |
| `VA_ADAPTIVEHEADLIGHT` | essieu avant · phare adaptatif | `nicht_aktiv` · `aktiv` | Adaptive headlights (AHL) | Phares adaptatifs (AHL) |
| `VA_AIRVENT_SHORTCUT` | essieu avant · aerateur · raccourci | `nicht_aktiv` · `aktiv` | Air distribution menu briefly appears in iDrive when changing air distribution mode | Le menu de repartition d'air apparait brievement dans l'iDrive lors d'un changement |
| `VA_AUTOMATIC_SHORTCUT` | essieu avant · automatique · raccourci | `nicht_aktiv` · `aktiv` | Climate control menu briefly appears in iDrive when changing automatic mode | Le menu de climatisation apparait brievement dans l'iDrive lors d'un changement de mode automatique |
| `VA_BPI` | essieu avant · interface (?) | `nicht_aktiv` · `aktiv` | Bluetooth portable integration | Integration Bluetooth des appareils portables |
| `VA_CORNERINGLIGHT` | essieu avant · eclairage d'intersection | `nicht_aktiv` · `aktiv` | Cornering lights | Eclairage d'intersection |
| `VA_FB` | essieu avant · telecommande | `nicht_aktiv` | Programmable buttons | Boutons programmables |
| `VA_FB_COMMUNICATION` | essieu avant · telecommande · communication | `nicht_aktiv` · `aktiv` | Programmable buttons for communication | Boutons programmables : communication |
| `VA_FB_ENTERTAINMENT` | essieu avant · telecommande · divertissement | `nicht_aktiv` · `aktiv` | Programmable buttons for entertainment | Boutons programmables : divertissement |
| `VA_FB_NAVIGATION` | essieu avant · telecommande · navigation | `nicht_aktiv` · `aktiv` | Programmable buttons for navigation | Boutons programmables : navigation |
| `VA_FLAP_CC` | essieu avant · volet · Check Control | `nicht_aktiv` · `aktiv` | Check control notification opens flip-up display | Un message Check Control fait sortir l'ecran escamotable |
| `VA_FLAP_DISPLAY_CONFIG` | essieu avant · volet · affichage · configuration | `nicht_aktiv` · `aktiv` | Flip-up display opening configuration | Configuration d'ouverture de l'ecran escamotable |
| `VA_FLAP_HMISTART` | essieu avant · volet · demarrage de l'interface | `nicht_aktiv` · `aktiv` | Flip-up display opens when starting | L'ecran escamotable sort au demarrage |
| `VA_FLAP_NAVI` | essieu avant · volet · navigation | `nicht_aktiv` · `aktiv` | Navigation announcement opens flip-up display | Une annonce de navigation fait sortir l'ecran escamotable |
| `VA_FLAP_PHONE` | essieu avant · volet · telephone | `nicht_aktiv` · `aktiv` | Telephone call opens flip-up display | Un appel telephonique fait sortir l'ecran escamotable |
| `VA_FLAP_SVS` | essieu avant · volet · surveillance (?) | `nicht_aktiv` · `aktiv` | Speech announcement opens flip-up display | Une annonce vocale fait sortir l'ecran escamotable |
| `VA_FM_AM_TOGGLE` | essieu avant · FM · AM · bascule | `nicht_aktiv` · `aktiv` | Reserve programmable button for dedicated FM/AM radio band toggle button | Reserver un bouton programmable a la bascule de bande FM/AM |
| `VA_FOND_CLIMATE` | essieu avant · places arriere · climatisation | `nicht_aktiv` · `aktiv` | Rear climate control | Climatisation arriere |
| `VA_HANGUL_TYPEWRITER` | essieu avant · hangul · saisie clavier | `nicht_aktiv` · `aktiv` | Support for Hangul (Korean) writing system | Prise en charge de l'ecriture hangul (coreen) |
| `VA_KANJI_TYPEWRITER` | essieu avant · kanji · saisie clavier | `nicht_aktiv` · `aktiv` | Support for Kanji (Japanese) writing system | Prise en charge de l'ecriture kanji (japonais) |
| `VA_LONGDISTANCELIGHT` | essieu avant · feux longue portee | `nicht_aktiv` · `aktiv` | High beam assist (FLA) | Assistant de feux de route (FLA) |
| `VA_MANUAL_SHIFT` | essieu avant · manuel · passage de rapport | `nicht_aktiv` · `aktiv` | BMW M manual transmission | Boite mecanique BMW M |
| `VA_MODE_TOGGLE` | essieu avant · mode · bascule | `nicht_aktiv` · `aktiv` | Reserve programmable button for dedicated audio source toggle button | Reserver un bouton programmable a la bascule de source audio |
| `VA_MP3_DISCLAIMER` | essieu avant · MP3 · avertissement | `nicht_aktiv` · `aktiv` | Legal disclaimer for MP3 | Avertissement legal pour le MP3 |
| `VA_NOAIRVENT_FINETUNING` | essieu avant · sans aerateur · reglage fin | `nicht_aktiv` · `aktiv` | No air vent fine tuning | Pas de reglage fin des aerateurs |
| `VA_PDC_LOW` | essieu avant · radars · bas | `nicht_aktiv` · `aktiv` | Park distance control (PDC) without on-screen visualization | Radars de stationnement sans affichage a l'ecran |
| `VA_SEATCLIMA_SHORTCUT` | essieu avant · climatisation de siege · raccourci | `nicht_aktiv` · `aktiv` | Seat heating menu briefly appears in iDrive when pressing seat heating switch | Le menu des sieges chauffants apparait brievement dans l'iDrive a l'appui sur la touche |
| `VA_SEATHEAT_SLIDER` | essieu avant · siege chauffant · curseur | `nicht_aktiv` · `aktiv` | Seat heating intensity slider in iDrive | Curseur d'intensite des sieges chauffants dans l'iDrive |
| `VA_SEATVENTILATION` | essieu avant · ventilation des sieges | `nicht_aktiv` · `aktiv` | Seat ventilation | Ventilation des sieges |
| `VA_WELCOMELIGHT` | essieu avant · eclairage d'accueil | `nicht_aktiv` · `aktiv` | Welcome lights (linked to [WELCOME_LIGHT] in LMA) | Eclairage d'accueil (lie a WELCOME_LIGHT dans le LMA) |
| `VEHICLE_FINDER` | vehicule · recherche | `nicht_aktiv` · `aktiv` | Vehicle tracking | Suivi du vehicule |
| `VERBRAUCH` | consommation | `mpg` · `default` | Fuel consumption unit | Unite de consommation |
| `VIDEOMODUL_CCC_B` | module video · systeme multimedia CCC · B | `nicht_aktiv` · `ece` · `rgb` · `drive` | Video/television | Video / television |
| `VIDEO_DENY_C0B` | video · refus · [variante C0B] | `deny_slow_key` · `deny_stop_break` · `deny_at_all_times` · `deny_never` | Disable video/television while driving (linked to [SPEEDLOCK_FREISCHALTMENU] in CIC) | Blocage de la video/television en roulant (lie a SPEEDLOCK_FREISCHALTMENU dans le CIC) |
| `WAEHLBARE_SPRACHE_1` | selectionnable · langue · 1 | `nicht_aktiv` · `deutsch` · `englisch` · `us` · `spanisch` · `italienisch` · `franzoesisch` · `flaemisch` … *(+8)* | Language 1 for language selection | Langue principale de l'interface. |
| `WAEHLBARE_SPRACHE_2` | selectionnable · langue · 2 | `nicht_aktiv` · `deutsch` · `englisch` · `us` · `spanisch` · `italienisch` · `franzoesisch` · `flaemisch` … *(+7)* | Language 2 for language selection | Langue 2 du selecteur de langue |
| `WAEHLBARE_SPRACHE_3` | selectionnable · langue · 3 | `nicht_aktiv` · `deutsch` · `englisch` · `us` · `spanisch` · `italienisch` · `franzoesisch` · `flaemisch` … *(+7)* | Language 3 for language selection | Langue 3 du selecteur de langue |
| `WEATHERBAND` | bande meteo | `nicht_aktiv` · `aktiv` | Weather band in source selection | Bande meteo proposee dans le choix des sources |
| `WEG_EINHEIT` | course · unite | `km` · `mls` | Distance unit | Unite de distance |
| `ZBE` | molette iDrive | `high` · `low` · `low_with_slopes` | iDrive controller (ZBE) type | Type de molette iDrive (ZBE) |
| `ZEIT_EINHEIT` | duree · unite | `standard` · `e60_stereo` · `e60_hifi` · `e60_top_hifi` · `e63_hifi` · `e63_top_hifi` · `e61_stereo` · `e61_hifi` … *(+17)* | Time format | Format d'heure |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Signification (EN)** — description issue de la base communautaire NCS Dummy, verbatim.

**Explication (FR)** — traduction française. Un tiret signifie que la ligne n'est pas
encore traduite. Les notations d'unités sont converties : `[durée s = valeur / 10]`.
