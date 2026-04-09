# Lexique Technique — Siemens MSV70 / BMW N52

> Référence terminologique pour la calibration ECU MSV70.  
> Bin de référence : `VB67774_921S_Full.bin` — BMW 330i E9x N52B30, SW 9PPL921S

---

## 1. Préfixes de Nomenclature

Chaque paramètre commence par un préfixe qui définit sa nature technique (convention Siemens/Bosch A2L/ASAP2) :

| Préfixe | Type | Description |
|---|---|---|
| `c_` | **Constant** | Valeur scalaire unique (seuil, constante physique). Ex : `c_tco_min_st` |
| `ip_` | **Interpolated Parameter** | Cartographie (Map) ou courbe — valeur interpolée entre les points. Ex : `ip_iga_st_bas` |
| `id_` | **Individual Data** | Sélections, tableaux fixes, identifiants, commutateurs complexes |
| `lc_` | **Logical Constant** | Interrupteur binaire (0/1, On/Off). Ex : `lc_iga_st_ena` |
| `idx_` | **Index** | Points d'axe (X ou Y) pour une cartographie |
| `in_` / `va_` | **Input / Variable** | Variables internes de l'ECU — lecture seule, non modifiables |

---

## 2. Abréviations Courantes

| Sigle | Signification | Unité / Contexte |
|---|---|---|
| **AFR** | Air/Fuel Ratio — rapport air/carburant massique | 14.7:1 (essence) ; ~9.8:1 (E85) |
| **IGA** | Ignition Angle — avance à l'allumage | °CRK (degrés vilebrequin avant PMH) |
| **MFF** | Mass Fuel Flow — masse carburant calculée par l'ECU | mg/stk |
| **TI** | Teil-Einspritzzeit — durée d'ouverture injecteur | ms |
| **MAF** | Mass Air Flow — masse d'air admis (débitmètre HFM) | mg/stk ou kg/h |
| **STK** | Stroke/Arbeitsspiel — un cycle moteur complet | base des unités mg/stk |
| **TCO** | Temperature Coolant — température liquide de refroidissement | °C |
| **TIA** | Temperature Intake Air — température d'air à l'admission | °C |
| **TEG** | Temperatur Abgas — température gaz d'échappement | °C |
| **RPM** | Revolutions Per Minute — régime moteur | tr/min |
| **STFT** | Short Term Fuel Trim — correction richesse court terme (temps réel) | % (cible ±5%) |
| **LTFT** | Long Term Fuel Trim — correction richesse long terme (mémorisée) | % (cible ±5%) |
| **WOT / VL** | Wide Open Throttle / Volllast — pleine charge pédale au plancher | — |
| **MBT** | Minimum advance for Best Torque — avance optimale pour couple max | °CRK |
| **KNK** | Knock — cliquetis/détonation, corrigé par recul d'avance | — |
| **SOI / EOI** | Start/End Of Injection — angle de début/fin d'injection | °CRK |
| **OPM** | Operating Mode — mode opération (_opm_1 = Valvetronic actif, _opm_2 = papillonné) | — |
| **WF** | Wall Film — film carburant sur parois collecteur admission | — |
| **WUP** | Warm-Up — phase de montée en température post-démarrage | — |
| **CRK / CST** | Cranking / Cold Start — phase démarrage (démarreur en action) | — |
| **AST** | After-Start — stabilisation régime juste après allumage (~5 sec) | — |
| **DTC** | Diagnostic Trouble Code — code défaut OBD | ex : P0130 |
| **MIL** | Malfunction Indicator Light — voyant moteur (Check Engine) | — |
| **RON** | Research Octane Number — indice d'octane | SP95 = 95 ; E85 ≈ 105 |
| **PCI** | Pouvoir Calorifique Inférieur — énergie/kg carburant | Essence 44 MJ/kg ; E85 ~26 MJ/kg |
| **LSU / UEGO** | Lambda Sensor Universal / Universal Exhaust Gas Oxygen — sonde large bande Bosch | — |
| **VANOS** | Variable Nockenwellen Steuerung — calage variable arbres à cames BMW | °CRK |
| **EWP** | Elektrische Wasserpumpe — pompe à eau électrique (spécificité N52) | — |
| **EKP** | Elektrische Kraftstoffpumpe — pompe à carburant électrique | — |
| **DISA** | Differenzierte Sauganlage — volets de résonance collecteur admission variable (N52) | — |

---

## 3. Lexique Technique A-Z

| Terme | Source | Définition |
|:---|:---|:---|
| **A2L / ASAP2** | Format | Format standard de description de paramètres ECU (AUTOSAR). Base de tout fichier XDF. |
| **AGGR** | XDF | **Aggregates** : Interaction moteur avec les accessoires (Clim, Alternateur, Servo). |
| **BGLL / BGLLGEN** | XDF | **Background Lambda Learning** : Adaptation lambda long terme — calcul du LTFT. |
| **BGO** | XDF | **Background (Non-Volatile)** : Données d'adaptation sauvegardées en mémoire non-volatile. |
| **BIN** | Format | Fichier binaire image de la mémoire flash ECU. Contient calibration + code exécutable. |
| **BIO** | XDF | **Biofuel** : Paramètres liés à l'éthanol (E85) et à la flex-fuel. |
| **BSFC** | Sigle | **Brake Specific Fuel Consumption** : Consommation spécifique [g/kWh]. |
| **BTDC** | Sigle | **Before Top Dead Center** : Avant le PMH. Unité de position vilebrequin [°]. |
| **CCA** | Sigle | **Cold Cranking Amps** : Capacité de démarrage à froid batterie [A] (standard SAE). |
| **CRLC** | Sigle | **Cylinder-individual Lambda Control** : Régulation richesse individuelle par cylindre. |
| **DCC** | Sigle | **Drive Cycle Conditioning** : Cycle de conduite OBD pour valider les moniteurs de diagnostic. |
| **DIA** | XDF | **Diagnosis** : Autodiagnostic OBD et surveillance capteurs/actionneurs. |
| **DME** | Sigle | **Digital Motor Electronics** : Nom BMW pour l'ECU moteur (= ECM/ECU). |
| **DMTL** | XDF | **Diagnose Modul Tank Leckage** : Module de détection de fuite réservoir (OBD EVAP). |
| **DROF** | Sigle | **Diagnostic Run On Function** : Diagnostic avec moteur en marche (ex : test catalyseur). |
| **DYN** | Sigle | **Dynamic** : Correction en mode transitoire (accélération/décélération). |
| **EAVANOS** | XDF | **VANOS Adaptation** : Adaptation du calage VANOS et correction positions cames. |
| **EGCP** | XDF | **Exhaust Gas Control & Probe** : Gestion sonde lambda et régulation richesse (WRAF). |
| **EGTR** | XDF | **Exhaust Gas Treatment** : Traitement gaz d'échappement — modèle catalyseur, O2 storage. |
| **EISY** | XDF | **Einspritz-Synchronisation** : Synchronisation et boucle fermée lambda. |
| **ENRD** | XDF | **Engine Roughness Detection** : Détection des ratés d'allumage et irrégularité cyclique. |
| **EOS** | XDF | **Engine Operating States** : États moteur (démarrage, ralenti, charge partielle, WOT). |
| **ERRM** | XDF | **Error Management** : Gestion des codes défauts, MIL et stratégies de repli. |
| **ETC / DK** | XDF | **Electronic Throttle Control / Drosselklappe** : Pilotage du papillon motorisé. |
| **EVAC / EVAP** | XDF | **Evaporative Emission Control** : Contrôle émissions EVAP — purge canister. |
| **FAC** | Sigle | **Factor / Faktor** : Multiplicateur de correction sans unité. Ex : `ip_fac_ti_temp_cor`. |
| **FL** | Sigle | **Full Load / Volllast** : Pleine charge (WOT). Mode open-loop enrichissement sur MSV70. |
| **FUP / PRS** | Sigle | **Kraftstoffdruck / Fuel Pressure** : Pression carburant dans le rail [bar ou hPa]. |
| **GEN** | XDF | **Generator** : Gestion de l'alternateur (charge, rendement thermique, protection). |
| **HOM** | Sigle | **Homogeneous** : Mode combustion homogène — mélange uniforme (standard N52). |
| **HUB / VVL** | XDF | **Hub / Valvetronic** : Levée variable des soupapes — contrôle charge sans papillon N52. |
| **KF** | Sigle | **Kennfeld** : Cartographie 3D — valeur f(axe X, axe Y). |
| **KL** | Sigle | **Kennlinie** : Courbe 2D — valeur f(axe X). |
| **Lambda (λ)** | Physique | Rapport AFR réel / AFR stœchiométrique. λ=1.00 = stoïchio ; λ<1 = riche ; λ>1 = pauvre. |
| **LL** | Sigle | **Leerlauf** : Ralenti. Mode régime minimal (600–800 rpm N52). |
| **MMV** | Sigle | **Mischungs-Momentenverteilung** : Distribution couple par richesse (modèle interne MSV70). |
| **NTC** | Sigle | **Negative Temperature Coefficient** : Type de capteur température (TCO, TIA). |
| **NST** | Sigle | **Neuer Start** : Transition cranking → stabilisation ralenti. |
| **O2L** | Sigle | **Oxygen Load** : Charge en oxygène du catalyseur (modèle diagnostic cat). |
| **PID** | Sigle | **Proportional-Integral-Derivative** : Type de régulateur (boucle fermée lambda, ralenti). |
| **PMH / TDC** | Sigle | **Point Mort Haut / Top Dead Center** : Pistons en haut = 0°CRK référence. |
| **SAP** | XDF | **Secondary Air Pump** : Pompe à air secondaire (injection post-démarrage pour chauffe cat). |
| **TECU** | Sigle | **Temperature ECU** : Température interne du calculateur (surveillance thermique). |
| **TOIL** | Sigle | **Öltemperatur** : Température d'huile moteur [°C]. |
| **TWC** | Sigle | **Three-Way Catalyst** : Catalyseur trois voies (CO, HC, NOx) — fonctionne à λ=1.00 ±0.02. |
| **VFZG** | Sigle | **Fahrzeuggeschwindigkeit** : Vitesse du véhicule [km/h]. |
| **VIN** | Sigle | **Vehicle Identification Number** : Numéro d'identification véhicule (17 caractères). |
| **VVTI** | XDF | **Variable Valve Timing Integration** : Adaptation intégrée VANOS + Valvetronic. |
| **WRAF** | XDF | **Wide Range Air/Fuel** : Système régulation lambda par sonde large bande (LSU/UEGO). |
| **XDF** | Format | **eXtended Definition File** : Fichier de définition TunerPro décrivant les paramètres d'un bin ECU. |
| **ZOT** | Sigle | **Zündung Oberer Totpunkt** : Allumage au PMH — référence avance à 0°. |

---

## 4. Structures de Données XDF

| Type | Dimensions | Description |
|---|---|---|
| **Scalaire (Constant)** | 1×1 | Valeur fixe unique — seuils, constantes physiques. Ex : `c_tco_min_st` = 17°C |
| **Courbe 2D (Curve)** | 1×N | Valeur f(un axe X). Ex : enrichissement démarrage = f(TCO). Visualisation : graphe lignes. |
| **Cartographie 3D (Map)** | M×N | Valeur f(axe X, axe Y). Ex : avance = f(MAF, RPM). Visualisation : tableau double entrée. |

## 5. Conventions d'Adressage

- **XDF Address** (`mmedaddress`) : adresse relative dans le bloc de données du XDF
- **BIN Address** : adresse réelle dans le fichier `.bin` = XDF Address + `BASEOFFSET` (0x40000 pour MSV70 9PPL921S)
- **Raw** : valeur binaire stockée (uint8 / uint16 big-endian sur MSV70)
- **Physical** : valeur réelle après application de l'équation XDF — ex : `0.75 × raw − 48` → °C

> Exemple : `c_tco_n_mff_cst` → raw=87 → `0.75 × 87 − 48 = 17.25°C`
