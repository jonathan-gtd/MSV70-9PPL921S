# DSC — Controle de stabilite / ABS

🔴 Module `DSC` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🔴 **Module a ne pas coder.** Liste fournie pour comprendre, pas pour modifier.

---

## Source

| | |
|---|---|
| Variante retenue | union de toutes les variantes (variante exacte à confirmer) |
| Fonctions codables | **761** |
| Avec valeurs extraites | 761 (100 %) |
| Avec explication rédigée | 47 |
| Fichiers DATEN analysés | 26 |

Variantes disponibles : `MK60_87.C03`, `MK60_87.C05`, `MK60_87.C06`, `MK60_87.C07`, `MK60_87.C08`, `MK60_87.C09`, `MK60_87.C0A`, `MK60_87.C0B`, `MK60_87.C0C`, `MK60_87.C0D`, `MK60_87.C0E`, `MK60_87.C0F`, `MK60_87.C20`, `DSC_89.C02`, `DSC_89.C03`, `DSC_84.C01`, `DSC_84.C02`, `DSC_84.C03`, `DSC_84.C04`, `DSC8_RPA.C06`, `DSC8_RPA.C09`, `DSC8_RPA.C0B`, `DSC8_RPA.C0D`, `DSC8_RPA.C0E`, `DSC8_RPA.C0F`, `DSC8_RPA.C10`

> La variante réellement installée est identifiée par NCS Dummy à l'ouverture du `.TRC`.
> Si elle diffère de celle retenue ici, la liste change.

---

## Les 761 fonctions codables

| FSW | Traduction | Valeurs possibles | Explication |
|---|---|---|---|
| `AFH_DOA` | _AFH_ · **DOA** | `aktiv` · `nicht_aktiv` |  |
| `AFS` | phares adaptatifs (AFS) | `aktiv` · `nicht_aktiv` |  |
| `ASL` | _ASL_ | `nicht_aktiv` · `aktiv` |  |
| `AUSSTATTUNG_RPA` | **AUSSTATTUNG** · indicateur de crevaison | `aktiv` · `nicht_aktiv` · `wert_01` · `tfl_s` |  |
| `AX_REF` | **AX** · reference | `nicht_aktiv` · `aktiv` |  |
| `AX_REF_DIFF_LOCK` | **AX** · reference · differentiel · blocage | `nicht_aktiv` · `aktiv` | Differentiel electronique de blocage (simulation par freinage). |
| `A_MAX_DSC` | A · maximum · controle de stabilite | `wert_00` · `wert_01` |  |
| `A_MIN_DSC` | A · minimum · controle de stabilite | `wert_00` · `wert_01` |  |
| `BAUART_MK60` | type de construction · **MK60** | `e81_4zyl` · `e88_4zyl` · `e87_4zyl` · `e90_4zyl` · `e91_4zyl` · `e92_4zyl` · `e93_4zyl` · `e90_4zyl_m_fw` … *(+9)* |  |
| `BBV_KORR_HA` | usure des plaquettes · correction · essieu arriere | `wert_00` · `wert_01` · `wert_02` |  |
| `BBV_KORR_VA` | usure des plaquettes · correction · essieu avant | `wert_00` · `wert_01` · `wert_02` |  |
| `BB_RAB` | **BB** · precharge des plaquettes | `aktiv` · `nicht_aktiv` |  |
| `BERGABANSATZ` | amorce de descente | `wert_00` · `wert_01` |  |
| `BERGABANSATZ_C0F` | amorce de descente · [variante C0F] | `wert_00` · `wert_01` |  |
| `BERGAB_GERASTET` | descente · crante | `wert_00` · `wert_01` |  |
| `BERGAB_GERASTET_C0E` | descente · crante · [variante C0E] | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `BERGAB_GERASTET_C0F` | descente · crante · [variante C0F] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `BERGAB_KENNEN` | descente · reconnaissance | `wert_00` · `wert_01` |  |
| `BERGAB_KENNEN_C0E` | descente · reconnaissance · [variante C0E] | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `BERGAB_KENNEN_C0F` | descente · reconnaissance · [variante C0F] | `wert_00` · `wert_01` |  |
| `BERRGAB_ANSATZ` | **BERRGAB** · **ANSATZ** | `wert_00` · `wert_01` |  |
| `BSW` | **BSW** | `aktiv` · `nicht_aktiv` |  |
| `C02E84_MOTOR_LEISTUNG_MAX` | **C02E84** · moteur · puissance · maximum | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` · `wert_09` |  |
| `C02_E84_BERGAB_GERASTET` | [variante C02] · **E84** · descente · crante | `wert_00` · `wert_01` |  |
| `C02_E84_FZG_MASSE` | [variante C02] · **E84** · vehicule · masse | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C02_E84_SCHUBABSCHALTUNG` | [variante C02] · **E84** · coupure en deceleration | `wert_00` · `wert_01` |  |
| `C03_UNTERSTEUERSCHWELLE_2` | [variante C03] · seuil de sous-virage · 2 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `C04_UEBERSTEUERN_MUE_0_2` | [variante C04] · survirage · coefficient d'adherence · 0 · 2 | `wert_10` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` · `wert_09` |  |
| `C04_UEBERSTEUERN_MUE_1_2` | [variante C04] · survirage · coefficient d'adherence · 1 · 2 | `wert_10` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+4)* |  |
| `C04_UNTERSTEUERSCHWELLE_2` | [variante C04] · seuil de sous-virage · 2 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `C05_BBV` | [variante C05] · usure des plaquettes | `wert_01` · `wert_02` |  |
| `C05_HSA` | [variante C05] · aide au demarrage en cote | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` | Aide au demarrage en cote. |
| `C06_BBV` | [variante C06] · usure des plaquettes | `wert_01` · `wert_02` |  |
| `C07_BAUART_MK60` | [variante C07] · type de construction · **MK60** | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+7)* |  |
| `C07_BBV` | [variante C07] · usure des plaquettes | `wert_01` · `wert_02` |  |
| `C07_BBV_KORR_HA` | [variante C07] · usure des plaquettes · correction · essieu arriere | `wert_00` · `wert_01` · `wert_02` |  |
| `C07_BBV_KORR_VA` | [variante C07] · usure des plaquettes · correction · essieu avant | `wert_00` · `wert_01` · `wert_02` |  |
| `C07_DELTA_PSI_BETA_LIMIT` | [variante C07] · ecart · **PSI** · _BETA_ · limite | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C07_DF_DIR` | [variante C07] · **DF** · direction | `wert_00` · `wert_01` · `wert_02` |  |
| `C07_DRUCKMODELL_HINTERACH` | [variante C07] · modele de pression · **HINTERACH** | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C07_DRUCKMODELL_VORDERACH` | [variante C07] · modele de pression · **VORDERACH** | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C07_GMK_ECBA_HA` | [variante C07] · _GMK_ · _ECBA_ · essieu arriere | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C07_GMK_ECBA_VA` | [variante C07] · _GMK_ · _ECBA_ · essieu avant | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C07_HSA` | [variante C07] · aide au demarrage en cote | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` | Aide au demarrage en cote. |
| `C07_MOTOR_ART_DSC_2` | [variante C07] · moteur · type · controle de stabilite · 2 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C07_UEBERSTEUERN_MUE_0_2` | [variante C07] · survirage · coefficient d'adherence · 0 · 2 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` |  |
| `C07_UEBERSTEUERN_MUE_1_2` | [variante C07] · survirage · coefficient d'adherence · 1 · 2 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+4)* |  |
| `C07_UNTERSTEUERSCHWELLE` | [variante C07] · seuil de sous-virage | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C08_AFH_HSA` | [variante C08] · _AFH_ · aide au demarrage en cote | `wert_01` | Aide au demarrage en cote. |
| `C08_AFS` | [variante C08] · phares adaptatifs (AFS) | `wert_01` · `wert_02` |  |
| `C08_ASL` | [variante C08] · _ASL_ | `wert_01` |  |
| `C08_BAUART` | [variante C08] · type de construction | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `C08_BBV` | [variante C08] · usure des plaquettes | `wert_01` · `wert_02` |  |
| `C08_BBV_KORR_HA` | [variante C08] · usure des plaquettes · correction · essieu arriere | `wert_01` |  |
| `C08_BBV_KORR_VA` | [variante C08] · usure des plaquettes · correction · essieu avant | `wert_00` · `wert_01` · `wert_02` |  |
| `C08_BB_RAB` | [variante C08] · **BB** · precharge des plaquettes | `wert_01` | Precharge des plaquettes au lever brusque de la pedale d'accelerateur. |
| `C08_BERGAB_GERASTET` | [variante C08] · descente · crante | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C08_BERGAB_KENNEN` | [variante C08] · descente · reconnaissance | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C08_CBC` | [variante C08] · controle de freinage en courbe | `wert_01` |  |
| `C08_CSC_DYNAMIC` | [variante C08] · **CSC** · **DYNAMIC** | `wert_01` · `wert_02` |  |
| `C08_DBC_MIN_AUSLOESEDRUCK` | [variante C08] · _DBC_ · minimum · pression de declenchement | `wert_01` |  |
| `C08_DELTA_PSI_BETA_LIMIT` | [variante C08] · ecart · **PSI** · _BETA_ · limite | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C08_DF_DIR` | [variante C08] · **DF** · direction | `wert_00` · `wert_01` · `wert_02` |  |
| `C08_DRUCKMODELL_HA` | [variante C08] · modele de pression · essieu arriere | `wert_00` · `wert_01` · `wert_02` |  |
| `C08_DRUCKMODELL_VA` | [variante C08] · modele de pression · essieu avant | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C08_ECBA_DCC` | [variante C08] · _ECBA_ · _DCC_ | `wert_01` · `wert_02` · `wert_03` |  |
| `C08_FBS` | [variante C08] · compensation de fading | `wert_01` | Compensation de l'effet de fading des freins. |
| `C08_FLR` | [variante C08] · reduction de puissance moteur | `wert_00` · `wert_01` · `wert_02` | Reduction de la puissance moteur par l'antipatinage. |
| `C08_FZG_MASSE` | [variante C08] · vehicule · masse | `wert_01` · `wert_02` |  |
| `C08_GETRIEBE` | [variante C08] · boite de vitesses | `wert_01` · `wert_02` · `wert_03` |  |
| `C08_GMB` | [variante C08] · _GMB_ | `wert_01` |  |
| `C08_GMK` | [variante C08] · _GMK_ | `wert_00` · `wert_01` · `wert_02` |  |
| `C08_GMK_ECBA_HA` | [variante C08] · _GMK_ · _ECBA_ · essieu arriere | `wert_00` · `wert_01` · `wert_02` |  |
| `C08_GMK_ECBA_VA` | [variante C08] · _GMK_ · _ECBA_ · essieu avant | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C08_GMR_MAX_LIMIT_SCHRAEG` | [variante C08] · regulation du moment de lacet · maximum · limite · derive | `wert_01` |  |
| `C08_GMR_MIN_LIMIT_SCHRAEG` | [variante C08] · regulation du moment de lacet · minimum · limite · derive | `wert_01` |  |
| `C08_HSA` | [variante C08] · aide au demarrage en cote | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` | Aide au demarrage en cote. |
| `C08_IPM_MAX_P_RES` | [variante C08] · _IPM_ · maximum · P · remise a zero | `wert_01` · `wert_02` |  |
| `C08_LENKWINKELKENNLINIE` | [variante C08] · courbe d'angle de braquage | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C08_LUEFTER` | [variante C08] · ventilateur | `wert_01` |  |
| `C08_MIN_DRUCKAUFBAU_GRAD` | [variante C08] · minimum · montee en pression · degre | `wert_01` |  |
| `C08_MOTOR_ART` | [variante C08] · moteur · type | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C08_MOTOR_LEISTUNG_MAX` | [variante C08] · moteur · puissance · maximum | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+4)* |  |
| `C08_MOTOR_SCHLEPP_P` | [variante C08] · moteur · couple de frein moteur · P | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C08_PRE_MSR` | [variante C08] · pre- · regulation du couple de frein moteur | `wert_01` |  |
| `C08_PRE_USC` | [variante C08] · pre- · _USC_ | `wert_01` |  |
| `C08_RPA` | [variante C08] · indicateur de crevaison | `wert_01` · `wert_02` |  |
| `C08_RPA_SCHWELLE` | [variante C08] · indicateur de crevaison · seuil | `wert_00` · `wert_01` |  |
| `C08_SCHUBABSCHALTUNG` | [variante C08] · coupure en deceleration | `wert_01` · `wert_02` |  |
| `C08_SDR_CTC` | [variante C08] · _SDR_ · _CTC_ | `wert_01` |  |
| `C08_SETZ_V_MAX` | [variante C08] · positionnement · _V_ · maximum | `wert_01` · `wert_02` · `wert_03` |  |
| `C08_SST` | [variante C08] · _SST_ | `wert_01` |  |
| `C08_TB_BDD` | [variante C08] · **TB** · **BDD** | `wert_01` | Sechage periodique des disques de frein par temps humide. |
| `C08_UEBERSTEUERN_MUE_0` | [variante C08] · survirage · coefficient d'adherence · 0 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `C08_UEBERSTEUERN_MUE_1_2` | [variante C08] · survirage · coefficient d'adherence · 1 · 2 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` |  |
| `C08_UNTERSTEUERN_MUE_0` | [variante C08] · sous-virage · coefficient d'adherence · 0 | `wert_01` |  |
| `C08_UNTERSTEUERN_MUE_1_2` | [variante C08] · sous-virage · coefficient d'adherence · 1 · 2 | `wert_01` |  |
| `C08_UNTERSTEUERSCHWELLE` | [variante C08] · seuil de sous-virage | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C08_VERZOEG_NEG_FEIN` | [variante C08] · temporisation · negatif · fin | `wert_01` · `wert_02` |  |
| `C08_VERZOEG_NEG_GROB` | [variante C08] · temporisation · negatif · grossier | `wert_01` · `wert_02` |  |
| `C08_VERZOEG_POS_FEIN` | [variante C08] · temporisation · position · fin | `wert_01` · `wert_02` |  |
| `C08_VERZOEG_POS_GROB` | [variante C08] · temporisation · position · grossier | `wert_01` · `wert_02` |  |
| `C08_V_EINHEIT` | [variante C08] · _V_ · unite | `wert_01` · `wert_02` |  |
| `C09_ASL` | [variante C09] · _ASL_ | `wert_00` |  |
| `C09_BAUART` | [variante C09] · type de construction | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+3)* |  |
| `C09_BBV` | [variante C09] · usure des plaquettes | `wert_01` · `wert_02` |  |
| `C09_BBV_KORR_HA` | [variante C09] · usure des plaquettes · correction · essieu arriere | `wert_00` · `wert_01` · `wert_02` |  |
| `C09_BBV_KORR_VA` | [variante C09] · usure des plaquettes · correction · essieu avant | `wert_00` · `wert_01` · `wert_02` |  |
| `C09_BTM` | [variante C09] · _BTM_ | `wert_00` |  |
| `C09_C8` | [variante C09] · **C8** | `wert_01` · `wert_02` |  |
| `C09_CBC` | [variante C09] · controle de freinage en courbe | `wert_00` |  |
| `C09_CF_ESM_HB` | [variante C09] · **CF** · _ESM_ · **HB** | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C09_CF_ESM_LB` | [variante C09] · **CF** · _ESM_ · **LB** | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C09_CR_ESM_HB` | [variante C09] · **CR** · _ESM_ · **HB** | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C09_CR_ESM_LB` | [variante C09] · **CR** · _ESM_ · **LB** | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C09_DBC_MIN_AUSLOESEDRUCK` | [variante C09] · _DBC_ · minimum · pression de declenchement | `wert_00` |  |
| `C09_DELTA_PSI_BETA_LIMIT` | [variante C09] · ecart · **PSI** · _BETA_ · limite | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C09_DIFF_LOCK` | [variante C09] · differentiel · blocage | `wert_00` · `wert_01` · `wert_02` | Differentiel electronique de blocage (simulation par freinage). |
| `C09_DRUCKMODELL_HA` | [variante C09] · modele de pression · essieu arriere | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C09_DRUCKMODELL_VA` | [variante C09] · modele de pression · essieu avant | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C09_FLR` | [variante C09] · reduction de puissance moteur | `wert_00` · `wert_01` · `wert_02` | Reduction de la puissance moteur par l'antipatinage. |
| `C09_GETRIEBE` | [variante C09] · boite de vitesses | `wert_00` · `wert_01` · `wert_02` |  |
| `C09_GMB` | [variante C09] · _GMB_ | `wert_00` |  |
| `C09_GMR_MAX_LIMIT_SCHRAEG` | [variante C09] · regulation du moment de lacet · maximum · limite · derive | `wert_00` |  |
| `C09_GMR_MIN_LIMIT_SCHRAEG` | [variante C09] · regulation du moment de lacet · minimum · limite · derive | `wert_00` |  |
| `C09_KUEHLSCHACHT` | [variante C09] · conduit de refroidissement | `wert_00` · `wert_01` · `wert_02` |  |
| `C09_LCL` | [variante C09] · _LCL_ | `wert_00` |  |
| `C09_LENKWINKELKENNLINIE` | [variante C09] · courbe d'angle de braquage | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C09_LF_ESM` | [variante C09] · **LF** · _ESM_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C09_LR_ESM` | [variante C09] · **LR** · _ESM_ | `wert_00` · `wert_01` · `wert_02` |  |
| `C09_LUEFTER` | [variante C09] · ventilateur | `wert_00` |  |
| `C09_LWK_X2` | [variante C09] · angle de braquage · [axe X2] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C09_LWK_X3` | [variante C09] · angle de braquage · [axe X3] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C09_LWK_X4` | [variante C09] · angle de braquage · [axe X4] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C09_LWK_Y1` | [variante C09] · angle de braquage · [axe Y1] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C09_LWK_Y2` | [variante C09] · angle de braquage · [axe Y2] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C09_LWK_Y3` | [variante C09] · angle de braquage · [axe Y3] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C09_LWK_Y4` | [variante C09] · angle de braquage · [axe Y4] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C09_MIN_DRUCKAUFBAU_GRAD` | [variante C09] · minimum · montee en pression · degre | `wert_00` |  |
| `C09_MOTOR` | [variante C09] · moteur | `wert_00` |  |
| `C09_M_ESM` | [variante C09] · M · _ESM_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C09_PRE_MSR` | [variante C09] · pre- · regulation du couple de frein moteur | `wert_00` |  |
| `C09_PRE_USC` | [variante C09] · pre- · _USC_ | `wert_00` |  |
| `C09_RPA` | [variante C09] · indicateur de crevaison | `wert_00` · `wert_01` |  |
| `C09_RPA_SCHWELLE` | [variante C09] · indicateur de crevaison · seuil | `wert_00` · `wert_01` · `wert_02` |  |
| `C09_SC_X_ESM` | [variante C09] · _SC_ · X · _ESM_ | `wert_00` · `wert_01` · `wert_02` |  |
| `C09_SC_Y_ESM` | [variante C09] · _SC_ · Y · _ESM_ | `wert_00` · `wert_01` · `wert_02` |  |
| `C09_SDR_CTC` | [variante C09] · _SDR_ · _CTC_ | `wert_00` |  |
| `C09_SPURWEITE` | [variante C09] · voie | `wert_00` · `wert_01` · `wert_02` |  |
| `C09_THETA_ESM` | [variante C09] · _THETA_ · _ESM_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C09_UEBERSTEUERN_MUE_0` | [variante C09] · survirage · coefficient d'adherence · 0 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C09_UEBERSTEUERN_MUE_1_2` | [variante C09] · survirage · coefficient d'adherence · 1 · 2 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` |  |
| `C09_UNTERSTEUERN_MUE_0` | [variante C09] · sous-virage · coefficient d'adherence · 0 | `wert_00` |  |
| `C09_UNTERSTEUERN_MUE_1_2` | [variante C09] · sous-virage · coefficient d'adherence · 1 · 2 | `wert_00` |  |
| `C09_UNTERSTEUERSCHWELLE` | [variante C09] · seuil de sous-virage | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C09_WLC` | [variante C09] · **WLC** | `wert_01` |  |
| `C0A_AFS` | [variante C0A] · phares adaptatifs (AFS) | `wert_01` · `wert_02` |  |
| `C0A_ASC_AMR_STB` | [variante C0A] · antipatinage · **AMR** · _STB_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `C0A_ASC_BMR_STB` | [variante C0A] · antipatinage · **BMR** · _STB_ | `wert_01` |  |
| `C0A_ASL` | [variante C0A] · _ASL_ | `wert_01` |  |
| `C0A_BAUART` | [variante C0A] · type de construction | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+13)* |  |
| `C0A_BBV` | [variante C0A] · usure des plaquettes | `wert_01` · `wert_02` |  |
| `C0A_BBV_KORR_HA` | [variante C0A] · usure des plaquettes · correction · essieu arriere | `wert_00` · `wert_01` · `wert_02` |  |
| `C0A_BBV_KORR_VA` | [variante C0A] · usure des plaquettes · correction · essieu avant | `wert_00` · `wert_01` · `wert_02` |  |
| `C0A_BERGABANSATZ` | [variante C0A] · amorce de descente | `wert_01` · `wert_02` |  |
| `C0A_BERGAB_GERASTET` | [variante C0A] · descente · crante | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0A_BERGAB_KENNEN` | [variante C0A] · descente · reconnaissance | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0A_BTM` | [variante C0A] · _BTM_ | `wert_01` |  |
| `C0A_CBC` | [variante C0A] · controle de freinage en courbe | `wert_01` |  |
| `C0A_DBC_MIN_AUSLOESEDRUCK` | [variante C0A] · _DBC_ · minimum · pression de declenchement | `wert_01` |  |
| `C0A_DCC_C_AAT` | [variante C0A] · _DCC_ · C · temperature exterieure | `wert_01` |  |
| `C0A_DCC_C_ATT_SCHUB_AUS` | [variante C0A] · _DCC_ · C · _ATT_ · pied leve / deceleration · arret | `wert_01` · `wert_02` |  |
| `C0A_DCC_C_ATT_SCHUB_EIN` | [variante C0A] · _DCC_ · C · _ATT_ · pied leve / deceleration · marche | `wert_01` · `wert_02` |  |
| `C0A_DELTA_PSI_BETA_LIMIT` | [variante C0A] · ecart · **PSI** · _BETA_ · limite | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0A_DF_DIR` | [variante C0A] · **DF** · direction | `wert_00` · `wert_01` · `wert_02` |  |
| `C0A_DIFF_LOCK` | [variante C0A] · differentiel · blocage | `wert_00` · `wert_01` · `wert_02` | Differentiel electronique de blocage (simulation par freinage). |
| `C0A_DRUCKMODELL_HA` | [variante C0A] · modele de pression · essieu arriere | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0A_DRUCKMODELL_VA` | [variante C0A] · modele de pression · essieu avant | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `C0A_ECBA_DCC` | [variante C0A] · _ECBA_ · _DCC_ | `wert_01` · `wert_02` · `wert_03` |  |
| `C0A_FLR` | [variante C0A] · reduction de puissance moteur | `wert_00` · `wert_01` · `wert_02` | Reduction de la puissance moteur par l'antipatinage. |
| `C0A_GETRIEBE` | [variante C0A] · boite de vitesses | `wert_01` · `wert_02` · `wert_03` |  |
| `C0A_GMB` | [variante C0A] · _GMB_ | `wert_01` |  |
| `C0A_GMK` | [variante C0A] · _GMK_ | `wert_00` · `wert_01` |  |
| `C0A_GMK_ECBA_HA` | [variante C0A] · _GMK_ · _ECBA_ · essieu arriere | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0A_GMK_ECBA_VA` | [variante C0A] · _GMK_ · _ECBA_ · essieu avant | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `C0A_GMR_MAX_LIMIT_SCHRAEG` | [variante C0A] · regulation du moment de lacet · maximum · limite · derive | `wert_01` |  |
| `C0A_GMR_MIN_LIMIT_SCHRAEG` | [variante C0A] · regulation du moment de lacet · minimum · limite · derive | `wert_01` |  |
| `C0A_HSA` | [variante C0A] · aide au demarrage en cote | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` | Aide au demarrage en cote. |
| `C0A_HSA_AFH_AFU` | [variante C0A] · aide au demarrage en cote · _AFH_ · **AFU** | `wert_01` | Aide au demarrage en cote. |
| `C0A_IPM_MAX_P_RES` | [variante C0A] · _IPM_ · maximum · P · remise a zero | `wert_01` · `wert_02` |  |
| `C0A_KUEHLSCHACHT` | [variante C0A] · conduit de refroidissement | `wert_00` · `wert_01` · `wert_02` |  |
| `C0A_LCL` | [variante C0A] · _LCL_ | `wert_01` |  |
| `C0A_LENKWINKELKENNLINIE` | [variante C0A] · courbe d'angle de braquage | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0A_LUEFTER` | [variante C0A] · ventilateur | `wert_01` |  |
| `C0A_MIN_DRUCKAUFBAU_GRAD` | [variante C0A] · minimum · montee en pression · degre | `wert_01` |  |
| `C0A_MOTOR` | [variante C0A] · moteur | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+3)* |  |
| `C0A_MOTOR_LEISTUNG_MAX` | [variante C0A] · moteur · puissance · maximum | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+9)* |  |
| `C0A_MOTOR_SCHLEPP_P` | [variante C0A] · moteur · couple de frein moteur · P | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0A_PRE_MSR` | [variante C0A] · pre- · regulation du couple de frein moteur | `wert_01` |  |
| `C0A_PRE_USC` | [variante C0A] · pre- · _USC_ | `wert_01` |  |
| `C0A_RPA` | [variante C0A] · indicateur de crevaison | `wert_01` · `wert_02` |  |
| `C0A_RPA_SCHWELLE` | [variante C0A] · indicateur de crevaison · seuil | `wert_00` · `wert_01` |  |
| `C0A_SCHUBABSCHALTUNG` | [variante C0A] · coupure en deceleration | `wert_01` |  |
| `C0A_SDR_CTC` | [variante C0A] · _SDR_ · _CTC_ | `wert_01` |  |
| `C0A_SETZ_V_MAX` | [variante C0A] · positionnement · _V_ · maximum | `wert_01` · `wert_02` · `wert_03` |  |
| `C0A_UEBERSTEUERN_MUE_0` | [variante C0A] · survirage · coefficient d'adherence · 0 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` |  |
| `C0A_UEBERSTEUERN_MUE_1_2` | [variante C0A] · survirage · coefficient d'adherence · 1 · 2 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+5)* |  |
| `C0A_UNTERSTEUERN_MUE_0` | [variante C0A] · sous-virage · coefficient d'adherence · 0 | `wert_01` |  |
| `C0A_UNTERSTEUERN_MUE_1_2` | [variante C0A] · sous-virage · coefficient d'adherence · 1 · 2 | `wert_01` |  |
| `C0A_UNTERSTEUERSCHWELLE` | [variante C0A] · seuil de sous-virage | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0A_VERZOEG_NEG_FEIN` | [variante C0A] · temporisation · negatif · fin | `wert_01` · `wert_02` |  |
| `C0A_VERZOEG_NEG_GROB` | [variante C0A] · temporisation · negatif · grossier | `wert_01` · `wert_02` |  |
| `C0A_VERZOEG_POS_FEIN` | [variante C0A] · temporisation · position · fin | `wert_01` · `wert_02` |  |
| `C0A_VERZOEG_POS_GROB` | [variante C0A] · temporisation · position · grossier | `wert_01` · `wert_02` |  |
| `C0A_V_EINHEIT` | [variante C0A] · _V_ · unite | `wert_01` · `wert_02` |  |
| `C0A_WLC` | [variante C0A] · **WLC** | `wert_01` |  |
| `C0B_AFH_HSA` | [variante C0B] · _AFH_ · aide au demarrage en cote | `wert_00` · `wert_01` | Aide au demarrage en cote. |
| `C0B_AFS` | [variante C0B] · phares adaptatifs (AFS) | `wert_00` · `wert_01` |  |
| `C0B_ASC_AMR_STB` | [variante C0B] · antipatinage · **AMR** · _STB_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `C0B_ASC_BMR_STB` | [variante C0B] · antipatinage · **BMR** · _STB_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0B_ASL` | [variante C0B] · _ASL_ | `wert_00` |  |
| `C0B_BAUART` | [variante C0B] · type de construction | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+10)* |  |
| `C0B_BBV` | [variante C0B] · usure des plaquettes | `wert_00` |  |
| `C0B_BBV_KORR_HA` | [variante C0B] · usure des plaquettes · correction · essieu arriere | `wert_00` · `wert_01` · `wert_02` |  |
| `C0B_BBV_KORR_VA` | [variante C0B] · usure des plaquettes · correction · essieu avant | `wert_00` · `wert_01` · `wert_02` |  |
| `C0B_BB_RAB` | [variante C0B] · **BB** · precharge des plaquettes | `wert_00` · `wert_01` | Precharge des plaquettes au lever brusque de la pedale d'accelerateur. |
| `C0B_BERGABANSATZ` | [variante C0B] · amorce de descente | `wert_00` · `wert_01` |  |
| `C0B_BERGAB_GERASTET` | [variante C0B] · descente · crante | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0B_BERGAB_KENNEN` | [variante C0B] · descente · reconnaissance | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0B_BTM` | [variante C0B] · _BTM_ | `wert_00` · `wert_01` · `wert_02` |  |
| `C0B_CBC` | [variante C0B] · controle de freinage en courbe | `wert_00` |  |
| `C0B_CF_ESM_HB` | [variante C0B] · **CF** · _ESM_ · **HB** | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0B_CF_ESM_LB` | [variante C0B] · **CF** · _ESM_ · **LB** | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `C0B_CR_ESM_HB` | [variante C0B] · **CR** · _ESM_ · **HB** | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` |  |
| `C0B_CR_ESM_LB` | [variante C0B] · **CR** · _ESM_ · **LB** | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` |  |
| `C0B_DBC_MIN_AUSLOESEDRUCK` | [variante C0B] · _DBC_ · minimum · pression de declenchement | `wert_00` |  |
| `C0B_DCC_C_AAT` | [variante C0B] · _DCC_ · C · temperature exterieure | `wert_00` · `wert_01` |  |
| `C0B_DCC_C_ATT_SCHUB_AUS` | [variante C0B] · _DCC_ · C · _ATT_ · pied leve / deceleration · arret | `wert_00` · `wert_01` |  |
| `C0B_DCC_C_ATT_SCHUB_EIN` | [variante C0B] · _DCC_ · C · _ATT_ · pied leve / deceleration · marche | `wert_00` · `wert_01` |  |
| `C0B_DCC_DTC` | [variante C0B] · _DCC_ · controle de traction dynamique | `wert_00` |  |
| `C0B_DELTA_PSI_BETA_LIMIT` | [variante C0B] · ecart · **PSI** · _BETA_ · limite | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0B_DF_DIR` | [variante C0B] · **DF** · direction | `wert_00` · `wert_01` · `wert_02` |  |
| `C0B_DIFF_LOCK` | [variante C0B] · differentiel · blocage | `wert_00` · `wert_01` · `wert_02` | Differentiel electronique de blocage (simulation par freinage). |
| `C0B_DRUCKMODELL_HA` | [variante C0B] · modele de pression · essieu arriere | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0B_DRUCKMODELL_VA` | [variante C0B] · modele de pression · essieu avant | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `C0B_ECBA_DCC` | [variante C0B] · _ECBA_ · _DCC_ | `wert_00` · `wert_01` · `wert_02` |  |
| `C0B_FBS` | [variante C0B] · compensation de fading | `wert_00` · `wert_01` | Compensation de l'effet de fading des freins. |
| `C0B_FLR` | [variante C0B] · reduction de puissance moteur | `wert_00` · `wert_01` · `wert_02` | Reduction de la puissance moteur par l'antipatinage. |
| `C0B_GETRIEBE` | [variante C0B] · boite de vitesses | `wert_00` · `wert_01` · `wert_02` |  |
| `C0B_GMB` | [variante C0B] · _GMB_ | `wert_00` |  |
| `C0B_GMK_ECBA_HA` | [variante C0B] · _GMK_ · _ECBA_ · essieu arriere | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0B_GMK_ECBA_VA` | [variante C0B] · _GMK_ · _ECBA_ · essieu avant | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `C0B_GMR_MAX_LIMIT_SCHRAEG` | [variante C0B] · regulation du moment de lacet · maximum · limite · derive | `wert_00` |  |
| `C0B_GMR_MIN_LIMIT_SCHRAEG` | [variante C0B] · regulation du moment de lacet · minimum · limite · derive | `wert_00` |  |
| `C0B_HSA` | [variante C0B] · aide au demarrage en cote | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` | Aide au demarrage en cote. |
| `C0B_HSA_AFH_AFU` | [variante C0B] · aide au demarrage en cote · _AFH_ · **AFU** | `wert_00` | Aide au demarrage en cote. |
| `C0B_IPM_MAX_P_RES` | [variante C0B] · _IPM_ · maximum · P · remise a zero | `wert_00` · `wert_01` |  |
| `C0B_KUEHLSCHACHT` | [variante C0B] · conduit de refroidissement | `wert_00` · `wert_01` · `wert_02` |  |
| `C0B_LCL` | [variante C0B] · _LCL_ | `wert_00` |  |
| `C0B_LENKWINKELKENNLINIE` | [variante C0B] · courbe d'angle de braquage | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0B_LF_ESM` | [variante C0B] · **LF** · _ESM_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `C0B_LR_ESM` | [variante C0B] · **LR** · _ESM_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `C0B_LUEFTER` | [variante C0B] · ventilateur | `wert_00` |  |
| `C0B_LWK_X2` | [variante C0B] · angle de braquage · [axe X2] | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0B_LWK_X3` | [variante C0B] · angle de braquage · [axe X3] | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0B_LWK_X4` | [variante C0B] · angle de braquage · [axe X4] | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0B_LWK_Y1` | [variante C0B] · angle de braquage · [axe Y1] | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0B_LWK_Y2` | [variante C0B] · angle de braquage · [axe Y2] | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0B_LWK_Y3` | [variante C0B] · angle de braquage · [axe Y3] | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0B_LWK_Y4` | [variante C0B] · angle de braquage · [axe Y4] | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0B_MIN_DRUCKAUFBAU_GRAD` | [variante C0B] · minimum · montee en pression · degre | `wert_00` |  |
| `C0B_MOTOR` | [variante C0B] · moteur | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+3)* |  |
| `C0B_MOTOR_LEISTUNG_MAX` | [variante C0B] · moteur · puissance · maximum | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+9)* |  |
| `C0B_MOTOR_SCHLEPP_P` | [variante C0B] · moteur · couple de frein moteur · P | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0B_M_ESM` | [variante C0B] · M · _ESM_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `C0B_PRE_MSR` | [variante C0B] · pre- · regulation du couple de frein moteur | `wert_00` |  |
| `C0B_PRE_USC` | [variante C0B] · pre- · _USC_ | `wert_00` |  |
| `C0B_RPA` | [variante C0B] · indicateur de crevaison | `wert_00` · `wert_01` |  |
| `C0B_RPA_SCHWELLE` | [variante C0B] · indicateur de crevaison · seuil | `wert_00` · `wert_01` · `wert_02` |  |
| `C0B_SCHUBABSCHALTUNG` | [variante C0B] · coupure en deceleration | `wert_00` · `wert_01` |  |
| `C0B_SC_X_ESM` | [variante C0B] · _SC_ · X · _ESM_ | `wert_00` · `wert_01` · `wert_02` |  |
| `C0B_SC_Y_ESM` | [variante C0B] · _SC_ · Y · _ESM_ | `wert_00` · `wert_01` · `wert_02` |  |
| `C0B_SDR_CTC` | [variante C0B] · _SDR_ · _CTC_ | `wert_00` |  |
| `C0B_SETZ_V_MAX` | [variante C0B] · positionnement · _V_ · maximum | `wert_00` · `wert_01` · `wert_02` |  |
| `C0B_SPURWEITE` | [variante C0B] · voie | `wert_00` · `wert_01` · `wert_02` |  |
| `C0B_SST` | [variante C0B] · _SST_ | `wert_00` · `wert_01` |  |
| `C0B_TB_BDD` | [variante C0B] · **TB** · **BDD** | `wert_00` · `wert_01` | Sechage periodique des disques de frein par temps humide. |
| `C0B_THETA_ESM` | [variante C0B] · _THETA_ · _ESM_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` |  |
| `C0B_UEBERSTEUERN_MUE_0` | [variante C0B] · survirage · coefficient d'adherence · 0 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` |  |
| `C0B_UEBERSTEUERN_MUE_1_2` | [variante C0B] · survirage · coefficient d'adherence · 1 · 2 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+6)* |  |
| `C0B_UNTERSTEUERN_MUE_0` | [variante C0B] · sous-virage · coefficient d'adherence · 0 | `wert_00` |  |
| `C0B_UNTERSTEUERN_MUE_1_2` | [variante C0B] · sous-virage · coefficient d'adherence · 1 · 2 | `wert_00` |  |
| `C0B_UNTERSTEUERSCHWELLE` | [variante C0B] · seuil de sous-virage | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0B_VERZOEG_NEG_FEIN` | [variante C0B] · temporisation · negatif · fin | `wert_00` · `wert_01` |  |
| `C0B_VERZOEG_NEG_GROB` | [variante C0B] · temporisation · negatif · grossier | `wert_00` · `wert_01` |  |
| `C0B_VERZOEG_POS_FEIN` | [variante C0B] · temporisation · position · fin | `wert_00` · `wert_01` |  |
| `C0B_VERZOEG_POS_GROB` | [variante C0B] · temporisation · position · grossier | `wert_00` · `wert_01` |  |
| `C0B_V_EINHEIT` | [variante C0B] · _V_ · unite | `wert_00` · `wert_01` |  |
| `C0C_AFH_HSA` | [variante C0C] · _AFH_ · aide au demarrage en cote | `wert_00` · `wert_01` | Aide au demarrage en cote. |
| `C0C_AFS` | [variante C0C] · phares adaptatifs (AFS) | `wert_00` · `wert_01` |  |
| `C0C_ASC_AMR_STB` | [variante C0C] · antipatinage · **AMR** · _STB_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `C0C_ASC_BMR_STB` | [variante C0C] · antipatinage · **BMR** · _STB_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0C_ASL` | [variante C0C] · _ASL_ | `wert_00` |  |
| `C0C_BAUART` | [variante C0C] · type de construction | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+13)* |  |
| `C0C_BBV` | [variante C0C] · usure des plaquettes | `wert_00` |  |
| `C0C_BBV_KORR_HA` | [variante C0C] · usure des plaquettes · correction · essieu arriere | `wert_00` · `wert_01` · `wert_02` |  |
| `C0C_BBV_KORR_VA` | [variante C0C] · usure des plaquettes · correction · essieu avant | `wert_00` · `wert_01` · `wert_02` |  |
| `C0C_BB_RAB` | [variante C0C] · **BB** · precharge des plaquettes | `wert_00` · `wert_01` | Precharge des plaquettes au lever brusque de la pedale d'accelerateur. |
| `C0C_BERGABANSATZ` | [variante C0C] · amorce de descente | `wert_00` · `wert_01` |  |
| `C0C_BERGAB_GERASTET` | [variante C0C] · descente · crante | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0C_BERGAB_KENNEN` | [variante C0C] · descente · reconnaissance | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0C_BTM` | [variante C0C] · _BTM_ | `wert_00` |  |
| `C0C_CBC` | [variante C0C] · controle de freinage en courbe | `wert_00` |  |
| `C0C_CF_ESM_HB` | [variante C0C] · **CF** · _ESM_ · **HB** | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0C_CF_ESM_LB` | [variante C0C] · **CF** · _ESM_ · **LB** | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `C0C_CR_ESM_HB` | [variante C0C] · **CR** · _ESM_ · **HB** | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `C0C_CR_ESM_LB` | [variante C0C] · **CR** · _ESM_ · **LB** | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` |  |
| `C0C_DBC_MIN_AUSLOESEDRUCK` | [variante C0C] · _DBC_ · minimum · pression de declenchement | `wert_00` |  |
| `C0C_DCC_C_AAT` | [variante C0C] · _DCC_ · C · temperature exterieure | `wert_00` · `wert_01` |  |
| `C0C_DCC_C_ATT_SCHUB_AUS` | [variante C0C] · _DCC_ · C · _ATT_ · pied leve / deceleration · arret | `wert_00` · `wert_01` |  |
| `C0C_DCC_C_ATT_SCHUB_EIN` | [variante C0C] · _DCC_ · C · _ATT_ · pied leve / deceleration · marche | `wert_00` · `wert_01` |  |
| `C0C_DCC_DTC` | [variante C0C] · _DCC_ · controle de traction dynamique | `wert_00` |  |
| `C0C_DELTA_PSI_BETA_LIMIT` | [variante C0C] · ecart · **PSI** · _BETA_ · limite | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0C_DF_DIR` | [variante C0C] · **DF** · direction | `wert_00` · `wert_01` · `wert_02` |  |
| `C0C_DIFF_LOCK` | [variante C0C] · differentiel · blocage | `wert_00` · `wert_01` · `wert_02` | Differentiel electronique de blocage (simulation par freinage). |
| `C0C_DRUCKMODELL_HA` | [variante C0C] · modele de pression · essieu arriere | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0C_DRUCKMODELL_VA` | [variante C0C] · modele de pression · essieu avant | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `C0C_ECBA_DCC` | [variante C0C] · _ECBA_ · _DCC_ | `wert_00` · `wert_01` · `wert_02` |  |
| `C0C_FBS` | [variante C0C] · compensation de fading | `wert_00` · `wert_01` | Compensation de l'effet de fading des freins. |
| `C0C_FLR` | [variante C0C] · reduction de puissance moteur | `wert_00` · `wert_01` · `wert_02` | Reduction de la puissance moteur par l'antipatinage. |
| `C0C_GETRIEBE` | [variante C0C] · boite de vitesses | `wert_00` · `wert_01` · `wert_02` |  |
| `C0C_GMB` | [variante C0C] · _GMB_ | `wert_00` |  |
| `C0C_GMK_ECBA_HA` | [variante C0C] · _GMK_ · _ECBA_ · essieu arriere | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0C_GMK_ECBA_VA` | [variante C0C] · _GMK_ · _ECBA_ · essieu avant | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `C0C_GMR_MAX_LIMIT_SCHRAEG` | [variante C0C] · regulation du moment de lacet · maximum · limite · derive | `wert_00` |  |
| `C0C_GMR_MIN_LIMIT_SCHRAEG` | [variante C0C] · regulation du moment de lacet · minimum · limite · derive | `wert_00` |  |
| `C0C_HSA` | [variante C0C] · aide au demarrage en cote | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+4)* | Aide au demarrage en cote. |
| `C0C_HSA_AFH_AFU` | [variante C0C] · aide au demarrage en cote · _AFH_ · **AFU** | `wert_00` | Aide au demarrage en cote. |
| `C0C_IPM_MAX_P_RES` | [variante C0C] · _IPM_ · maximum · P · remise a zero | `wert_00` · `wert_01` |  |
| `C0C_KUEHLSCHACHT` | [variante C0C] · conduit de refroidissement | `wert_00` · `wert_01` · `wert_02` |  |
| `C0C_LCL` | [variante C0C] · _LCL_ | `wert_00` |  |
| `C0C_LENKWINKELKENNLINIE` | [variante C0C] · courbe d'angle de braquage | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0C_LF_ESM` | [variante C0C] · **LF** · _ESM_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` |  |
| `C0C_LR_ESM` | [variante C0C] · **LR** · _ESM_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0C_LUEFTER` | [variante C0C] · ventilateur | `wert_00` |  |
| `C0C_LWK_X2` | [variante C0C] · angle de braquage · [axe X2] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0C_LWK_X3` | [variante C0C] · angle de braquage · [axe X3] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0C_LWK_X4` | [variante C0C] · angle de braquage · [axe X4] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0C_LWK_Y1` | [variante C0C] · angle de braquage · [axe Y1] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0C_LWK_Y2` | [variante C0C] · angle de braquage · [axe Y2] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0C_LWK_Y3` | [variante C0C] · angle de braquage · [axe Y3] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0C_LWK_Y4` | [variante C0C] · angle de braquage · [axe Y4] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0C_MIN_DRUCKAUFBAU_GRAD` | [variante C0C] · minimum · montee en pression · degre | `wert_00` |  |
| `C0C_MOTOR` | [variante C0C] · moteur | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+3)* |  |
| `C0C_MOTOR_LEISTUNG_MAX` | [variante C0C] · moteur · puissance · maximum | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+11)* |  |
| `C0C_MOTOR_SCHLEPP_P` | [variante C0C] · moteur · couple de frein moteur · P | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0C_M_ESM` | [variante C0C] · M · _ESM_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` |  |
| `C0C_PRE_MSR` | [variante C0C] · pre- · regulation du couple de frein moteur | `wert_00` |  |
| `C0C_PRE_USC` | [variante C0C] · pre- · _USC_ | `wert_00` |  |
| `C0C_RPA` | [variante C0C] · indicateur de crevaison | `wert_00` · `wert_01` |  |
| `C0C_RPA_SCHWELLE` | [variante C0C] · indicateur de crevaison · seuil | `wert_00` · `wert_01` · `wert_02` |  |
| `C0C_SCHUBABSCHALTUNG` | [variante C0C] · coupure en deceleration | `wert_00` · `wert_01` |  |
| `C0C_SC_X_ESM` | [variante C0C] · _SC_ · X · _ESM_ | `wert_00` · `wert_01` · `wert_02` |  |
| `C0C_SC_Y_ESM` | [variante C0C] · _SC_ · Y · _ESM_ | `wert_00` · `wert_01` · `wert_02` |  |
| `C0C_SDR_CTC` | [variante C0C] · _SDR_ · _CTC_ | `wert_00` |  |
| `C0C_SETZ_V_MAX` | [variante C0C] · positionnement · _V_ · maximum | `wert_00` · `wert_01` · `wert_02` |  |
| `C0C_SPURWEITE` | [variante C0C] · voie | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0C_SST` | [variante C0C] · _SST_ | `wert_00` · `wert_01` |  |
| `C0C_TB_BDD` | [variante C0C] · **TB** · **BDD** | `wert_00` · `wert_01` | Sechage periodique des disques de frein par temps humide. |
| `C0C_THETA_ESM` | [variante C0C] · _THETA_ · _ESM_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `C0C_UEBERSTEUERN_MUE_0` | [variante C0C] · survirage · coefficient d'adherence · 0 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` |  |
| `C0C_UEBERSTEUERN_MUE_1_2` | [variante C0C] · survirage · coefficient d'adherence · 1 · 2 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+6)* |  |
| `C0C_UNTERSTEUERN_MUE_0` | [variante C0C] · sous-virage · coefficient d'adherence · 0 | `wert_00` |  |
| `C0C_UNTERSTEUERN_MUE_1_2` | [variante C0C] · sous-virage · coefficient d'adherence · 1 · 2 | `wert_00` |  |
| `C0C_UNTERSTEUERSCHWELLE` | [variante C0C] · seuil de sous-virage | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0C_VERZOEG_NEG_FEIN` | [variante C0C] · temporisation · negatif · fin | `wert_00` · `wert_01` |  |
| `C0C_VERZOEG_NEG_GROB` | [variante C0C] · temporisation · negatif · grossier | `wert_00` · `wert_01` |  |
| `C0C_VERZOEG_POS_FEIN` | [variante C0C] · temporisation · position · fin | `wert_00` · `wert_01` |  |
| `C0C_VERZOEG_POS_GROB` | [variante C0C] · temporisation · position · grossier | `wert_00` · `wert_01` |  |
| `C0C_V_EINHEIT` | [variante C0C] · _V_ · unite | `wert_00` · `wert_01` |  |
| `C0D_ASL` | [variante C0D] · _ASL_ | `wert_00` |  |
| `C0D_BAUART` | [variante C0D] · type de construction | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+3)* |  |
| `C0D_BBV` | [variante C0D] · usure des plaquettes | `wert_00` |  |
| `C0D_BBV_KORR_HA` | [variante C0D] · usure des plaquettes · correction · essieu arriere | `wert_00` |  |
| `C0D_BBV_KORR_VA` | [variante C0D] · usure des plaquettes · correction · essieu avant | `wert_00` · `wert_01` · `wert_02` |  |
| `C0D_BTM` | [variante C0D] · _BTM_ | `wert_00` |  |
| `C0D_CBC` | [variante C0D] · controle de freinage en courbe | `wert_00` |  |
| `C0D_CF_ESM_HB` | [variante C0D] · **CF** · _ESM_ · **HB** | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0D_CF_ESM_LB` | [variante C0D] · **CF** · _ESM_ · **LB** | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0D_CR_ESM_HB` | [variante C0D] · **CR** · _ESM_ · **HB** | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0D_CR_ESM_LB` | [variante C0D] · **CR** · _ESM_ · **LB** | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0D_DBC_MIN_AUSLOESEDRUCK` | [variante C0D] · _DBC_ · minimum · pression de declenchement | `wert_00` |  |
| `C0D_DELTA_PSI_BETA_LIMIT` | [variante C0D] · ecart · **PSI** · _BETA_ · limite | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0D_DIFF_LOCK` | [variante C0D] · differentiel · blocage | `wert_00` · `wert_01` · `wert_02` | Differentiel electronique de blocage (simulation par freinage). |
| `C0D_DRUCKMODELL_HA` | [variante C0D] · modele de pression · essieu arriere | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0D_DRUCKMODELL_VA` | [variante C0D] · modele de pression · essieu avant | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0D_FLR` | [variante C0D] · reduction de puissance moteur | `wert_00` · `wert_01` · `wert_02` | Reduction de la puissance moteur par l'antipatinage. |
| `C0D_GETRIEBE` | [variante C0D] · boite de vitesses | `wert_00` · `wert_01` · `wert_02` |  |
| `C0D_GMB` | [variante C0D] · _GMB_ | `wert_00` |  |
| `C0D_GMR_MAX_LIMIT_SCHRAEG` | [variante C0D] · regulation du moment de lacet · maximum · limite · derive | `wert_00` |  |
| `C0D_GMR_MIN_LIMIT_SCHRAEG` | [variante C0D] · regulation du moment de lacet · minimum · limite · derive | `wert_00` |  |
| `C0D_KUEHLSCHACHT` | [variante C0D] · conduit de refroidissement | `wert_00` · `wert_01` · `wert_02` |  |
| `C0D_LCL` | [variante C0D] · _LCL_ | `wert_00` |  |
| `C0D_LENKWINKELKENNLINIE` | [variante C0D] · courbe d'angle de braquage | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0D_LF_ESM` | [variante C0D] · **LF** · _ESM_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0D_LR_ESM` | [variante C0D] · **LR** · _ESM_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0D_LUEFTER` | [variante C0D] · ventilateur | `wert_00` |  |
| `C0D_LWK_X2` | [variante C0D] · angle de braquage · [axe X2] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0D_LWK_X3` | [variante C0D] · angle de braquage · [axe X3] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0D_LWK_X4` | [variante C0D] · angle de braquage · [axe X4] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0D_LWK_Y1` | [variante C0D] · angle de braquage · [axe Y1] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0D_LWK_Y2` | [variante C0D] · angle de braquage · [axe Y2] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0D_LWK_Y3` | [variante C0D] · angle de braquage · [axe Y3] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0D_LWK_Y4` | [variante C0D] · angle de braquage · [axe Y4] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0D_MIN_DRUCKAUFBAU_GRAD` | [variante C0D] · minimum · montee en pression · degre | `wert_00` |  |
| `C0D_MOTOR` | [variante C0D] · moteur | `wert_00` |  |
| `C0D_M_ESM` | [variante C0D] · M · _ESM_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0D_PRE_MSR` | [variante C0D] · pre- · regulation du couple de frein moteur | `wert_00` |  |
| `C0D_PRE_USC` | [variante C0D] · pre- · _USC_ | `wert_00` |  |
| `C0D_RPA` | [variante C0D] · indicateur de crevaison | `wert_00` · `wert_01` |  |
| `C0D_RPA_SCHWELLE` | [variante C0D] · indicateur de crevaison · seuil | `wert_00` · `wert_01` · `wert_02` |  |
| `C0D_SC_X_ESM` | [variante C0D] · _SC_ · X · _ESM_ | `wert_00` · `wert_01` · `wert_02` |  |
| `C0D_SC_Y_ESM` | [variante C0D] · _SC_ · Y · _ESM_ | `wert_00` · `wert_01` · `wert_02` |  |
| `C0D_SDR_CTC` | [variante C0D] · _SDR_ · _CTC_ | `wert_00` |  |
| `C0D_SPURWEITE` | [variante C0D] · voie | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0D_THETA_ESM` | [variante C0D] · _THETA_ · _ESM_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0D_UEBERSTEUERN_MUE_0` | [variante C0D] · survirage · coefficient d'adherence · 0 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0D_UEBERSTEUERN_MUE_1_2` | [variante C0D] · survirage · coefficient d'adherence · 1 · 2 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` · `wert_09` |  |
| `C0D_UNTERSTEUERN_MUE_0` | [variante C0D] · sous-virage · coefficient d'adherence · 0 | `wert_00` |  |
| `C0D_UNTERSTEUERN_MUE_1_2` | [variante C0D] · sous-virage · coefficient d'adherence · 1 · 2 | `wert_00` |  |
| `C0D_UNTERSTEUERSCHWELLE` | [variante C0D] · seuil de sous-virage | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0D_WLC` | [variante C0D] · **WLC** | `wert_01` |  |
| `C0E_ASL` | [variante C0E] · _ASL_ | `wert_00` |  |
| `C0E_BAUART` | [variante C0E] · type de construction | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+3)* |  |
| `C0E_BBV` | [variante C0E] · usure des plaquettes | `wert_00` |  |
| `C0E_BBV_KORR_HA` | [variante C0E] · usure des plaquettes · correction · essieu arriere | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0E_BBV_KORR_VA` | [variante C0E] · usure des plaquettes · correction · essieu avant | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0E_BTM` | [variante C0E] · _BTM_ | `wert_00` |  |
| `C0E_CBC` | [variante C0E] · controle de freinage en courbe | `wert_00` |  |
| `C0E_CF_ESM_HB` | [variante C0E] · **CF** · _ESM_ · **HB** | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0E_CF_ESM_LB` | [variante C0E] · **CF** · _ESM_ · **LB** | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0E_CR_ESM_HB` | [variante C0E] · **CR** · _ESM_ · **HB** | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0E_CR_ESM_LB` | [variante C0E] · **CR** · _ESM_ · **LB** | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0E_DBC_MIN_AUSLOESEDRUCK` | [variante C0E] · _DBC_ · minimum · pression de declenchement | `wert_00` |  |
| `C0E_DELTA_PSI_BETA_LIMIT` | [variante C0E] · ecart · **PSI** · _BETA_ · limite | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0E_DIFF_LOCK` | [variante C0E] · differentiel · blocage | `wert_00` · `wert_01` · `wert_02` | Differentiel electronique de blocage (simulation par freinage). |
| `C0E_DRUCKMODELL_HA` | [variante C0E] · modele de pression · essieu arriere | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0E_DRUCKMODELL_VA` | [variante C0E] · modele de pression · essieu avant | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0E_FLR` | [variante C0E] · reduction de puissance moteur | `wert_00` · `wert_01` · `wert_02` | Reduction de la puissance moteur par l'antipatinage. |
| `C0E_GETRIEBE` | [variante C0E] · boite de vitesses | `wert_00` · `wert_01` · `wert_02` |  |
| `C0E_GMB` | [variante C0E] · _GMB_ | `wert_00` |  |
| `C0E_GMR_MAX_LIMIT_SCHRAEG` | [variante C0E] · regulation du moment de lacet · maximum · limite · derive | `wert_00` |  |
| `C0E_GMR_MIN_LIMIT_SCHRAEG` | [variante C0E] · regulation du moment de lacet · minimum · limite · derive | `wert_00` |  |
| `C0E_KUEHLSCHACHT` | [variante C0E] · conduit de refroidissement | `wert_00` · `wert_01` · `wert_02` |  |
| `C0E_LCL` | [variante C0E] · _LCL_ | `wert_00` |  |
| `C0E_LENKWINKELKENNLINIE` | [variante C0E] · courbe d'angle de braquage | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0E_LF_ESM` | [variante C0E] · **LF** · _ESM_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0E_LR_ESM` | [variante C0E] · **LR** · _ESM_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0E_LUEFTER` | [variante C0E] · ventilateur | `wert_00` |  |
| `C0E_LWK_X2` | [variante C0E] · angle de braquage · [axe X2] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0E_LWK_X3` | [variante C0E] · angle de braquage · [axe X3] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0E_LWK_X4` | [variante C0E] · angle de braquage · [axe X4] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0E_LWK_Y1` | [variante C0E] · angle de braquage · [axe Y1] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0E_LWK_Y2` | [variante C0E] · angle de braquage · [axe Y2] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0E_LWK_Y3` | [variante C0E] · angle de braquage · [axe Y3] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0E_LWK_Y4` | [variante C0E] · angle de braquage · [axe Y4] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0E_MIN_DRUCKAUFBAU_GRAD` | [variante C0E] · minimum · montee en pression · degre | `wert_00` |  |
| `C0E_MOTOR` | [variante C0E] · moteur | `wert_00` |  |
| `C0E_M_ESM` | [variante C0E] · M · _ESM_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0E_PRE_MSR` | [variante C0E] · pre- · regulation du couple de frein moteur | `wert_00` |  |
| `C0E_PRE_USC` | [variante C0E] · pre- · _USC_ | `wert_00` |  |
| `C0E_RPA` | [variante C0E] · indicateur de crevaison | `wert_00` · `wert_01` |  |
| `C0E_RPA_SCHWELLE` | [variante C0E] · indicateur de crevaison · seuil | `wert_00` · `wert_01` · `wert_02` |  |
| `C0E_SC_X_ESM` | [variante C0E] · _SC_ · X · _ESM_ | `wert_00` · `wert_01` · `wert_02` |  |
| `C0E_SC_Y_ESM` | [variante C0E] · _SC_ · Y · _ESM_ | `wert_00` · `wert_01` · `wert_02` |  |
| `C0E_SDR_CTC` | [variante C0E] · _SDR_ · _CTC_ | `wert_00` |  |
| `C0E_SPURWEITE` | [variante C0E] · voie | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0E_THETA_ESM` | [variante C0E] · _THETA_ · _ESM_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0E_UEBERSTEUERN_MUE_0` | [variante C0E] · survirage · coefficient d'adherence · 0 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0E_UEBERSTEUERN_MUE_1_2` | [variante C0E] · survirage · coefficient d'adherence · 1 · 2 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` · `wert_09` |  |
| `C0E_UNTERSTEUERN_MUE_0` | [variante C0E] · sous-virage · coefficient d'adherence · 0 | `wert_00` |  |
| `C0E_UNTERSTEUERN_MUE_1_2` | [variante C0E] · sous-virage · coefficient d'adherence · 1 · 2 | `wert_00` |  |
| `C0E_UNTERSTEUERSCHWELLE` | [variante C0E] · seuil de sous-virage | `wert_00` · `wert_01` · `wert_02` |  |
| `C0E_WLC` | [variante C0E] · **WLC** | `wert_01` |  |
| `C0F_AFH_HSA` | [variante C0F] · _AFH_ · aide au demarrage en cote | `wert_00` · `wert_01` | Aide au demarrage en cote. |
| `C0F_AFS` | [variante C0F] · phares adaptatifs (AFS) | `wert_00` · `wert_01` |  |
| `C0F_ASC_AMR_STB` | [variante C0F] · antipatinage · **AMR** · _STB_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `C0F_ASC_BMR_STB` | [variante C0F] · antipatinage · **BMR** · _STB_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0F_ASL` | [variante C0F] · _ASL_ | `wert_00` |  |
| `C0F_BAUART` | [variante C0F] · type de construction | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+13)* |  |
| `C0F_BBV` | [variante C0F] · usure des plaquettes | `wert_00` |  |
| `C0F_BBV_KORR_HA` | [variante C0F] · usure des plaquettes · correction · essieu arriere | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0F_BBV_KORR_VA` | [variante C0F] · usure des plaquettes · correction · essieu avant | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0F_BB_RAB` | [variante C0F] · **BB** · precharge des plaquettes | `wert_00` · `wert_01` | Precharge des plaquettes au lever brusque de la pedale d'accelerateur. |
| `C0F_BERGABANSATZ` | [variante C0F] · amorce de descente | `wert_00` · `wert_01` |  |
| `C0F_BERGAB_GERASTET` | [variante C0F] · descente · crante | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0F_BERGAB_KENNEN` | [variante C0F] · descente · reconnaissance | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0F_BTM` | [variante C0F] · _BTM_ | `wert_00` |  |
| `C0F_CBC` | [variante C0F] · controle de freinage en courbe | `wert_00` |  |
| `C0F_CF_ESM_HB` | [variante C0F] · **CF** · _ESM_ · **HB** | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0F_CF_ESM_LB` | [variante C0F] · **CF** · _ESM_ · **LB** | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `C0F_CR_ESM_HB` | [variante C0F] · **CR** · _ESM_ · **HB** | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `C0F_CR_ESM_LB` | [variante C0F] · **CR** · _ESM_ · **LB** | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` |  |
| `C0F_DBC_MIN_AUSLOESEDRUCK` | [variante C0F] · _DBC_ · minimum · pression de declenchement | `wert_00` |  |
| `C0F_DCC_C_AAT` | [variante C0F] · _DCC_ · C · temperature exterieure | `wert_00` · `wert_01` |  |
| `C0F_DCC_C_ATT_SCHUB_AUS` | [variante C0F] · _DCC_ · C · _ATT_ · pied leve / deceleration · arret | `wert_00` · `wert_01` |  |
| `C0F_DCC_C_ATT_SCHUB_EIN` | [variante C0F] · _DCC_ · C · _ATT_ · pied leve / deceleration · marche | `wert_00` · `wert_01` |  |
| `C0F_DCC_DTC` | [variante C0F] · _DCC_ · controle de traction dynamique | `wert_00` |  |
| `C0F_DELTA_PSI_BETA_LIMIT` | [variante C0F] · ecart · **PSI** · _BETA_ · limite | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0F_DF_DIR` | [variante C0F] · **DF** · direction | `wert_00` · `wert_01` · `wert_02` |  |
| `C0F_DIFF_LOCK` | [variante C0F] · differentiel · blocage | `wert_00` · `wert_01` · `wert_02` | Differentiel electronique de blocage (simulation par freinage). |
| `C0F_DRUCKMODELL_HA` | [variante C0F] · modele de pression · essieu arriere | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0F_DRUCKMODELL_VA` | [variante C0F] · modele de pression · essieu avant | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `C0F_ECBA_DCC` | [variante C0F] · _ECBA_ · _DCC_ | `wert_00` · `wert_01` · `wert_02` |  |
| `C0F_FBS` | [variante C0F] · compensation de fading | `wert_00` · `wert_01` | Compensation de l'effet de fading des freins. |
| `C0F_FLR` | [variante C0F] · reduction de puissance moteur | `wert_00` · `wert_01` · `wert_02` | Reduction de la puissance moteur par l'antipatinage. |
| `C0F_GETRIEBE` | [variante C0F] · boite de vitesses | `wert_00` · `wert_01` · `wert_02` |  |
| `C0F_GMB` | [variante C0F] · _GMB_ | `wert_00` |  |
| `C0F_GMK_ECBA_HA` | [variante C0F] · _GMK_ · _ECBA_ · essieu arriere | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0F_GMK_ECBA_VA` | [variante C0F] · _GMK_ · _ECBA_ · essieu avant | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `C0F_GMR_MAX_LIMIT_SCHRAEG` | [variante C0F] · regulation du moment de lacet · maximum · limite · derive | `wert_00` |  |
| `C0F_GMR_MIN_LIMIT_SCHRAEG` | [variante C0F] · regulation du moment de lacet · minimum · limite · derive | `wert_00` |  |
| `C0F_HSA` | [variante C0F] · aide au demarrage en cote | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+4)* | Aide au demarrage en cote. |
| `C0F_HSA_AFH_AFU` | [variante C0F] · aide au demarrage en cote · _AFH_ · **AFU** | `wert_00` | Aide au demarrage en cote. |
| `C0F_IPM_MAX_P_RES` | [variante C0F] · _IPM_ · maximum · P · remise a zero | `wert_00` · `wert_01` |  |
| `C0F_KUEHLSCHACHT` | [variante C0F] · conduit de refroidissement | `wert_00` · `wert_01` · `wert_02` |  |
| `C0F_LCL` | [variante C0F] · _LCL_ | `wert_00` |  |
| `C0F_LENKWINKELKENNLINIE` | [variante C0F] · courbe d'angle de braquage | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0F_LF_ESM` | [variante C0F] · **LF** · _ESM_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` |  |
| `C0F_LR_ESM` | [variante C0F] · **LR** · _ESM_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `C0F_LUEFTER` | [variante C0F] · ventilateur | `wert_00` |  |
| `C0F_LWK_X2` | [variante C0F] · angle de braquage · [axe X2] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0F_LWK_X3` | [variante C0F] · angle de braquage · [axe X3] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0F_LWK_X4` | [variante C0F] · angle de braquage · [axe X4] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0F_LWK_Y1` | [variante C0F] · angle de braquage · [axe Y1] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0F_LWK_Y2` | [variante C0F] · angle de braquage · [axe Y2] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0F_LWK_Y3` | [variante C0F] · angle de braquage · [axe Y3] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0F_LWK_Y4` | [variante C0F] · angle de braquage · [axe Y4] | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0F_MIN_DRUCKAUFBAU_GRAD` | [variante C0F] · minimum · montee en pression · degre | `wert_00` |  |
| `C0F_MOTOR` | [variante C0F] · moteur | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+3)* |  |
| `C0F_MOTOR_LEISTUNG_MAX` | [variante C0F] · moteur · puissance · maximum | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+11)* |  |
| `C0F_MOTOR_SCHLEPP_P` | [variante C0F] · moteur · couple de frein moteur · P | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0F_M_ESM` | [variante C0F] · M · _ESM_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` |  |
| `C0F_PRE_MSR` | [variante C0F] · pre- · regulation du couple de frein moteur | `wert_00` |  |
| `C0F_PRE_USC` | [variante C0F] · pre- · _USC_ | `wert_00` |  |
| `C0F_RPA` | [variante C0F] · indicateur de crevaison | `wert_00` · `wert_01` |  |
| `C0F_RPA_SCHWELLE` | [variante C0F] · indicateur de crevaison · seuil | `wert_00` · `wert_01` · `wert_02` |  |
| `C0F_SCHUBABSCHALTUNG` | [variante C0F] · coupure en deceleration | `wert_00` · `wert_01` |  |
| `C0F_SC_X_ESM` | [variante C0F] · _SC_ · X · _ESM_ | `wert_00` · `wert_01` · `wert_02` |  |
| `C0F_SC_Y_ESM` | [variante C0F] · _SC_ · Y · _ESM_ | `wert_00` · `wert_01` · `wert_02` |  |
| `C0F_SDR_CTC` | [variante C0F] · _SDR_ · _CTC_ | `wert_00` |  |
| `C0F_SETZ_V_MAX` | [variante C0F] · positionnement · _V_ · maximum | `wert_00` · `wert_01` · `wert_02` |  |
| `C0F_SPURWEITE` | [variante C0F] · voie | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C0F_SST` | [variante C0F] · _SST_ | `wert_00` |  |
| `C0F_TB_BDD` | [variante C0F] · **TB** · **BDD** | `wert_00` · `wert_01` | Sechage periodique des disques de frein par temps humide. |
| `C0F_THETA_ESM` | [variante C0F] · _THETA_ · _ESM_ | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` |  |
| `C0F_UEBERSTEUERN_MUE_0` | [variante C0F] · survirage · coefficient d'adherence · 0 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` |  |
| `C0F_UEBERSTEUERN_MUE_1_2` | [variante C0F] · survirage · coefficient d'adherence · 1 · 2 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+6)* |  |
| `C0F_UNTERSTEUERN_MUE_0` | [variante C0F] · sous-virage · coefficient d'adherence · 0 | `wert_00` |  |
| `C0F_UNTERSTEUERN_MUE_1_2` | [variante C0F] · sous-virage · coefficient d'adherence · 1 · 2 | `wert_00` |  |
| `C0F_UNTERSTEUERSCHWELLE` | [variante C0F] · seuil de sous-virage | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `C0F_VERZOEG_NEG_FEIN` | [variante C0F] · temporisation · negatif · fin | `wert_00` · `wert_01` |  |
| `C0F_VERZOEG_NEG_GROB` | [variante C0F] · temporisation · negatif · grossier | `wert_00` · `wert_01` |  |
| `C0F_VERZOEG_POS_FEIN` | [variante C0F] · temporisation · position · fin | `wert_00` · `wert_01` |  |
| `C0F_VERZOEG_POS_GROB` | [variante C0F] · temporisation · position · grossier | `wert_00` · `wert_01` |  |
| `C0F_V_EINHEIT` | [variante C0F] · _V_ · unite | `wert_00` · `wert_01` |  |
| `C0X_C8` | **C0X** · **C8** | `wert_01` · `wert_02` |  |
| `C10C_AAT_SCHUB_AUS_OFFSET` | **C10C** · temperature exterieure · pied leve / deceleration · arret · decalage | `wert_00` · `wert_01` |  |
| `C10C_AAT_SCHUB_EIN_KORR` | **C10C** · temperature exterieure · pied leve / deceleration · marche · correction | `wert_00` · `wert_01` · `wert_02` |  |
| `C10C_AAT_SCHUB_EIN_OFFSET` | **C10C** · temperature exterieure · pied leve / deceleration · marche · decalage | `wert_00` · `wert_01` |  |
| `C10_A_MAX` | [variante C10] · A · maximum | `wert_00` · `wert_01` |  |
| `C10_A_MIN` | [variante C10] · A · minimum | `wert_00` · `wert_01` |  |
| `C10_BERGABANSATZ` | [variante C10] · amorce de descente | `wert_00` · `wert_01` |  |
| `C10_BERGAB_GERASTET` | [variante C10] · descente · crante | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `C10_BERGAB_KENNEN` | [variante C10] · descente · reconnaissance | `wert_00` · `wert_01` |  |
| `C10_DTC_DCC` | [variante C10] · controle de traction dynamique · _DCC_ | `wert_00` |  |
| `C10_ECBA_DCC` | [variante C10] · _ECBA_ · _DCC_ | `wert_00` · `wert_01` · `wert_02` |  |
| `C10_FZG_MASSE` | [variante C10] · vehicule · masse | `wert_00` · `wert_01` |  |
| `C10_IPM_MAX_P_RES` | [variante C10] · _IPM_ · maximum · P · remise a zero | `wert_00` · `wert_01` |  |
| `C10_MOTOR_LEISTUNG_MAX` | [variante C10] · moteur · puissance · maximum | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` · `wert_09` |  |
| `C10_P_AUSROLL_ANTRIEB` | [variante C10] · P · roue libre · transmission | `wert_00` · `wert_01` |  |
| `C10_R_RAD_DYN_DSC` | [variante C10] · _R_ · roue · dynamique · controle de stabilite | `wert_00` · `wert_01` |  |
| `C10_SCHUBABSCHALTUNG` | [variante C10] · coupure en deceleration | `wert_00` · `wert_01` |  |
| `C10_SETZ_V_MAX_CC` | [variante C10] · positionnement · _V_ · maximum · Check Control | `wert_00` · `wert_01` · `wert_02` |  |
| `C10_SETZ_V_MIN` | [variante C10] · positionnement · _V_ · minimum | `wert_00` · `wert_01` · `wert_02` |  |
| `C10_VERZOEG_NEG_FEIN` | [variante C10] · temporisation · negatif · fin | `wert_00` · `wert_01` |  |
| `C10_VERZOEG_NEG_GROB` | [variante C10] · temporisation · negatif · grossier | `wert_00` · `wert_01` |  |
| `C10_VERZOEG_POS_FEIN` | [variante C10] · temporisation · position · fin | `wert_00` · `wert_01` |  |
| `C10_VERZOEG_POS_GROB` | [variante C10] · temporisation · position · grossier | `wert_00` · `wert_01` |  |
| `C10_V_ABSCHALT` | [variante C10] · _V_ · coupure | `wert_00` · `wert_01` · `wert_02` |  |
| `C10_V_EINHEIT` | [variante C10] · _V_ · unite | `wert_00` · `wert_01` |  |
| `C8` | **C8** | `wert_01` · `wert_02` |  |
| `C8_BDSC` | **C8** · **BDSC** | `wert_01` · `wert_02` |  |
| `CF_ESM_HB` | **CF** · _ESM_ · **HB** | `wert_01` |  |
| `CF_ESM_LB` | **CF** · _ESM_ · **LB** | `wert_01` |  |
| `CHECKSUM` | somme de controle | `grundcode_var` |  |
| `CR_ESM_HB` | **CR** · _ESM_ · **HB** | `wert_01` |  |
| `CR_ESM_LB` | **CR** · _ESM_ · **LB** | `wert_01` |  |
| `C_AAT_SCHUB_AUS_OFFSET` | C · temperature exterieure · pied leve / deceleration · arret · decalage | `wert_00` · `wert_01` |  |
| `C_AAT_SCHUB_EIN_KORR_05` | C · temperature exterieure · pied leve / deceleration · marche · correction · 05 | `wert_00` · `wert_01` |  |
| `C_AAT_SCHUB_EIN_KORR_C0F` | C · temperature exterieure · pied leve / deceleration · marche · correction · [variante C0F] | `wert_00` · `wert_01` · `wert_02` |  |
| `C_AAT_SCHUB_EIN_KORR_SKAL` | C · temperature exterieure · pied leve / deceleration · marche · correction · **SKAL** | `wert_00` · `wert_01` · `wert_02` |  |
| `C_AAT_SCHUB_EIN_OFFSET` | C · temperature exterieure · pied leve / deceleration · marche · decalage | `wert_00` · `wert_01` |  |
| `DBC_MIN_AUFBAUGRADIENT` | _DBC_ · minimum · **AUFBAUGRADIENT** | `wert_00` |  |
| `DBC_MIN_AUSLOESEDRUCK` | _DBC_ · minimum · pression de declenchement | `wert_00` · `wert_01` · `wert_02` |  |
| `DCC_DTC` | _DCC_ · controle de traction dynamique | `wert_00` · `wert_01` |  |
| `DELTA_PSI_BETA_LIMITKORR2` | ecart · **PSI** · _BETA_ · **LIMITKORR2** | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `DF_DIR` | **DF** · direction | `aktiv` · `nicht_aktiv` |  |
| `DIFF_LOCK` | differentiel · blocage | `aktiv` · `nicht_aktiv` |  |
| `DRUCKMODELL_HINTERACHSE` | modele de pression · **HINTERACHSE** | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `DRUCKMODELL_VORDERACHSE` | modele de pression · **VORDERACHSE** | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `DSC8_SST` | **DSC8** · _SST_ | `aktiv` · `nicht_aktiv` |  |
| `DTC_DCC` | controle de traction dynamique · _DCC_ | `wert_00` |  |
| `DXC_CBS` | _DXC_ · entretien selon besoin (CBS) | `aktiv` · `nicht_aktiv` |  |
| `E84C_AAT_SCHUB_AUS_OFFSET` | **E84C** · temperature exterieure · pied leve / deceleration · arret · decalage | `wert_00` · `wert_01` |  |
| `E84C_AAT_SCHUB_EIN_KORR` | **E84C** · temperature exterieure · pied leve / deceleration · marche · correction | `wert_00` |  |
| `E84C_AAT_SCHUB_EIN_OFFSET` | **E84C** · temperature exterieure · pied leve / deceleration · marche · decalage | `wert_00` · `wert_01` |  |
| `E84_AAT_SCHUB_AUS_OFFSET` | **E84** · temperature exterieure · pied leve / deceleration · arret · decalage | `wert_00` · `wert_01` |  |
| `E84_AAT_SCHUB_EIN_KORR` | **E84** · temperature exterieure · pied leve / deceleration · marche · correction | `wert_00` · `wert_01` |  |
| `E84_AAT_SCHUB_EIN_OFFSET` | **E84** · temperature exterieure · pied leve / deceleration · marche · decalage | `wert_00` · `wert_01` |  |
| `E84_ASL` | **E84** · _ASL_ | `aktiv` |  |
| `E84_AX_REF_DIFF_LOCK` | **E84** · **AX** · reference · differentiel · blocage | `nicht_aktiv` · `aktiv` | Differentiel electronique de blocage (simulation par freinage). |
| `E84_A_MAX` | **E84** · A · maximum | `wert_00` · `wert_01` |  |
| `E84_A_MAX_DSC` | **E84** · A · maximum · controle de stabilite | `wert_00` · `wert_01` |  |
| `E84_A_MIN` | **E84** · A · minimum | `wert_00` · `wert_01` |  |
| `E84_A_MIN_DSC` | **E84** · A · minimum · controle de stabilite | `wert_00` · `wert_01` |  |
| `E84_BERGABANSATZ` | **E84** · amorce de descente | `wert_00` · `wert_01` |  |
| `E84_BERGAB_GERASTET` | **E84** · descente · crante | `wert_00` · `wert_01` |  |
| `E84_BERGAB_KENNEN` | **E84** · descente · reconnaissance | `wert_00` · `wert_01` |  |
| `E84_BSW` | **E84** · **BSW** | `aktiv` · `nicht_aktiv` |  |
| `E84_BSW_HO` | **E84** · **BSW** · **HO** | `aktiv` · `nicht_aktiv` |  |
| `E84_C8_BDSC` | **E84** · **C8** · **BDSC** | `wert_01` · `wert_02` |  |
| `E84_CBS` | **E84** · entretien selon besoin (CBS) | `aktiv` · `nicht_aktiv` |  |
| `E84_DCDC` | **E84** · **DCDC** | `aktiv` · `nicht_aktiv` |  |
| `E84_DTC_DCC` | **E84** · controle de traction dynamique · _DCC_ | `wert_00` |  |
| `E84_ECBA_DCC` | **E84** · _ECBA_ · _DCC_ | `wert_00` · `wert_01` |  |
| `E84_EVB` | **E84** · **EVB** | `aktiv` · `nicht_aktiv` |  |
| `E84_FDB` | **E84** · **FDB** | `aktiv` · `nicht_aktiv` |  |
| `E84_FLR` | **E84** · reduction de puissance moteur | `aktiv` · `nicht_aktiv` | Reduction de la puissance moteur par l'antipatinage. |
| `E84_FZG_MASSE` | **E84** · vehicule · masse | `wert_00` · `wert_01` · `wert_02` |  |
| `E84_FZG_VARIANTEN` | **E84** · vehicule · **VARIANTEN** | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+37)* |  |
| `E84_HBA` | **E84** · assistance de freinage | `nicht_aktiv` · `wert_01` · `wert_02` · `wert_03` | Assistance hydraulique au freinage d'urgence. |
| `E84_HHC` | **E84** · **HHC** | `aktiv` · `nicht_aktiv` |  |
| `E84_HILLDESC` | **E84** · assistance en descente | `aktiv` · `nicht_aktiv` |  |
| `E84_HILLDESC_DXC_8` | **E84** · assistance en descente · _DXC_ · 8 | `aktiv` · `nicht_aktiv` |  |
| `E84_HPS` | **E84** · **HPS** | `aktiv` · `nicht_aktiv` |  |
| `E84_HVV` | **E84** · **HVV** | `aktiv` |  |
| `E84_IPM_MAX_P_RES` | **E84** · _IPM_ · maximum · P · remise a zero | `wert_00` · `wert_01` |  |
| `E84_MOTOR_LEISTUNG_MAX` | **E84** · moteur · puissance · maximum | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` · `wert_09` |  |
| `E84_P_AUSROLL_ANTRIEB` | **E84** · P · roue libre · transmission | `wert_00` · `wert_01` |  |
| `E84_R_RAD_DYN_DSC` | **E84** · _R_ · roue · dynamique · controle de stabilite | `wert_00` · `wert_01` |  |
| `E84_SCHUBABSCHALTUNG` | **E84** · coupure en deceleration | `wert_00` · `wert_01` |  |
| `E84_SETZ_V_MAX_CC` | **E84** · positionnement · _V_ · maximum · Check Control | `wert_00` · `wert_01` · `wert_02` |  |
| `E84_SETZ_V_MIN` | **E84** · positionnement · _V_ · minimum | `wert_00` · `wert_01` · `wert_02` |  |
| `E84_SST` | **E84** · _SST_ | `aktiv` · `nicht_aktiv` |  |
| `E84_VCH` | **E84** · **VCH** | `aktiv` · `nicht_aktiv` |  |
| `E84_VERZOEG_NEG_FEIN` | **E84** · temporisation · negatif · fin | `wert_00` · `wert_01` |  |
| `E84_VERZOEG_NEG_GROB` | **E84** · temporisation · negatif · grossier | `wert_00` · `wert_01` |  |
| `E84_VERZOEG_POS_FEIN` | **E84** · temporisation · position · fin | `wert_00` · `wert_01` |  |
| `E84_VERZOEG_POS_GROB` | **E84** · temporisation · position · grossier | `wert_00` · `wert_01` |  |
| `E84_V_ABSCHALT` | **E84** · _V_ · coupure | `wert_00` · `wert_01` · `wert_02` |  |
| `E84_V_EINHEIT` | **E84** · _V_ · unite | `wert_00` · `wert_01` |  |
| `E89_ASL` | **E89** · _ASL_ | `aktiv` · `nicht_aktiv` |  |
| `E89_BERGAB_GERASTET` | **E89** · descente · crante | `wert_00` · `wert_01` |  |
| `E89_BERGAB_KENNEN` | **E89** · descente · reconnaissance | `wert_00` · `wert_01` |  |
| `E89_BSW` | **E89** · **BSW** | `aktiv` · `nicht_aktiv` |  |
| `E89_C8` | **E89** · **C8** | `wert_01` · `wert_02` |  |
| `E89_ECBA_DCC` | **E89** · _ECBA_ · _DCC_ | `wert_00` · `wert_01` |  |
| `E89_EVB` | **E89** · **EVB** | `aktiv` · `nicht_aktiv` |  |
| `E89_FLR` | **E89** · reduction de puissance moteur | `aktiv` · `nicht_aktiv` | Reduction de la puissance moteur par l'antipatinage. |
| `E89_HHC` | **E89** · **HHC** | `aktiv` · `nicht_aktiv` |  |
| `E89_HILLDESC_DXC_8` | **E89** · assistance en descente · _DXC_ · 8 | `aktiv` · `nicht_aktiv` |  |
| `E89_HPS` | **E89** · **HPS** | `aktiv` · `nicht_aktiv` |  |
| `E89_MOTOR_LEISTUNG_MAX` | **E89** · moteur · puissance · maximum | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` |  |
| `E89_P_AUSROLL_ANTRIEB` | **E89** · P · roue libre · transmission | `wert_00` · `wert_01` |  |
| `E89_R_RAD_DYN_DSC` | **E89** · _R_ · roue · dynamique · controle de stabilite | `wert_00` · `wert_02` |  |
| `E89_SCHUBABSCHALTUNG` | **E89** · coupure en deceleration | `wert_00` · `wert_01` |  |
| `E89_SST` | **E89** · _SST_ | `aktiv` · `nicht_aktiv` |  |
| `E89_SZL` | **E89** · **SZL** | `wert_01` · `wert_02` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_16` · `wert_17` |  |
| `E90_FZG_VARIANTEN` | **E90** · vehicule · **VARIANTEN** | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+9)* |  |
| `ECBA` | _ECBA_ | `aktiv` · `nicht_aktiv` |  |
| `ECBA_DCC` | _ECBA_ · _DCC_ | `wert_00` · `wert_01` · `wert_02` |  |
| `ECBA_DCC_C0F` | _ECBA_ · _DCC_ · [variante C0F] | `wert_00` · `wert_01` · `wert_02` |  |
| `ECD` | **ECD** | `aktiv` · `nicht_aktiv` |  |
| `EVB` | **EVB** | `aktiv` · `nicht_aktiv` |  |
| `FBS` | compensation de fading | `aktiv` · `nicht_aktiv` |  |
| `FDB` | **FDB** | `aktiv` · `nicht_aktiv` |  |
| `FDB_0F` | **FDB** · **0F** | `aktiv` · `nicht_aktiv` |  |
| `FLR` | reduction de puissance moteur | `wert_00` · `wert_01` · `wert_02` |  |
| `FLR_C0D` | reduction de puissance moteur · [variante C0D] | `aktiv` · `nicht_aktiv` |  |
| `FLR_C0E` | reduction de puissance moteur · [variante C0E] | `aktiv` · `nicht_aktiv` |  |
| `FLR_C0F` | reduction de puissance moteur · [variante C0F] | `aktiv` · `nicht_aktiv` |  |
| `FZG_MASSE` | vehicule · masse | `wert_00` · `wert_01` |  |
| `FZG_MASSE_C03` | vehicule · masse · [variante C03] | `wert_00` · `wert_01` · `wert_02` |  |
| `FZG_VARIANTEN` | vehicule · **VARIANTEN** | `var_01` · `var_02` · `var_03` · `var_04` · `var_05` · `var_06` · `var_07` · `var_08` … *(+20)* |  |
| `GEARMOMENTREGLER_MAX_HA_2` | **GEARMOMENTREGLER** · maximum · essieu arriere · 2 | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `GEARMOMENTREGLER_MIN_HA_2` | **GEARMOMENTREGLER** · minimum · essieu arriere · 2 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `GETRIEBE` | boite de vitesses | `handschaltung` · `automatik` · `sequentiell_ag` |  |
| `GMB` | _GMB_ | `nicht_aktiv` · `aktiv` |  |
| `GMK` | _GMK_ | `wert_00` · `wert_01` |  |
| `GMK_ECBA_HA` | _GMK_ · _ECBA_ · essieu arriere | `wert_00` · `wert_01` · `wert_02` |  |
| `GMK_ECBA_VA` | _GMK_ · _ECBA_ · essieu avant | `wert_00` · `wert_01` · `wert_02` |  |
| `HBA_DXC_8` | assistance de freinage · _DXC_ · 8 | `nicht_aktiv` · `wert_01` · `wert_02` · `wert_03` |  |
| `HHC` | **HHC** | `aktiv` · `nicht_aktiv` |  |
| `HILLDESC_DXC_8` | assistance en descente · _DXC_ · 8 | `aktiv` · `nicht_aktiv` |  |
| `HILLDESC_DXC_8_C0F` | assistance en descente · _DXC_ · 8 · [variante C0F] | `aktiv` · `nicht_aktiv` |  |
| `HPS` | **HPS** | `aktiv` · `nicht_aktiv` |  |
| `HVV` | **HVV** | `aktiv` |  |
| `IPM_MAX_P_RES` | _IPM_ · maximum · P · remise a zero | `wert_00` · `wert_01` |  |
| `LENKWINKELKENNLINIE_2` | courbe d'angle de braquage · 2 | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_05` |  |
| `LF_ESM` | **LF** · _ESM_ | `wert_01` |  |
| `LR_ESM` | **LR** · _ESM_ | `wert_01` |  |
| `LUEFTER` | ventilateur | `nicht_aktiv` · `aktiv` |  |
| `LWK_X2` | angle de braquage · [axe X2] | `wert_01` |  |
| `LWK_X3` | angle de braquage · [axe X3] | `wert_01` |  |
| `LWK_X4` | angle de braquage · [axe X4] | `wert_01` |  |
| `LWK_Y1` | angle de braquage · [axe Y1] | `wert_01` |  |
| `LWK_Y2` | angle de braquage · [axe Y2] | `wert_01` |  |
| `LWK_Y3` | angle de braquage · [axe Y3] | `wert_01` |  |
| `LWK_Y4` | angle de braquage · [axe Y4] | `wert_01` |  |
| `MOTOR_ART_DSC_2` | moteur · type · controle de stabilite · 2 | `n46b20` · `m47d20` · `n52b30` · `n54b30` |  |
| `MOTOR_LEISTUNG_MAX` | moteur · puissance · maximum | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+8)* |  |
| `MOTOR_LEISTUNG_MAX_C0F` | moteur · puissance · maximum · [variante C0F] | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` · `wert_09` |  |
| `M_ESM` | M · _ESM_ | `wert_01` |  |
| `PRE_MSR` | pre- · regulation du couple de frein moteur | `aktiv` · `nicht_aktiv` |  |
| `P_AUSROLL_ANTRIEB` | P · roue libre · transmission | `wert_00` · `wert_01` |  |
| `P_AUSROLL_ANTRIEB_C0E` | P · roue libre · transmission · [variante C0E] | `wert_00` · `wert_01` · `wert_02` · `wert_03` · `wert_04` |  |
| `P_AUSROLL_ANTRIEB_C0F` | P · roue libre · transmission · [variante C0F] | `wert_00` · `wert_01` |  |
| `RPA` | indicateur de crevaison | `nicht_aktiv` · `aktiv` |  |
| `RPA_SCHWELLE` | indicateur de crevaison · seuil | `wert_00` · `wert_01` |  |
| `R_RAD_DYN_DSC` | _R_ · roue · dynamique · controle de stabilite | `wert_00` · `wert_01` · `wert_02` |  |
| `R_RAD_DYN_DSC_C0F` | _R_ · roue · dynamique · controle de stabilite · [variante C0F] | `wert_00` · `wert_01` |  |
| `SCHUBABSCHALTUNG` | coupure en deceleration | `wert_00` · `wert_01` |  |
| `SCHUBABSCHALTUNG_C0E` | coupure en deceleration · [variante C0E] | `wert_00` · `wert_01` |  |
| `SCHUBABSCHALTUNG_C0F` | coupure en deceleration · [variante C0F] | `wert_00` · `wert_01` |  |
| `SC_X_ESM` | _SC_ · X · _ESM_ | `wert_01` |  |
| `SC_Y_ESM` | _SC_ · Y · _ESM_ | `wert_01` |  |
| `SETZ_V_MAX_CC` | positionnement · _V_ · maximum · Check Control | `wert_00` · `wert_01` · `wert_02` |  |
| `SETZ_V_MIN` | positionnement · _V_ · minimum | `wert_00` · `wert_01` · `wert_02` |  |
| `SPURWEITE` | voie | `wert_01` |  |
| `SST` | _SST_ | `aktiv` · `nicht_aktiv` |  |
| `SZL` | **SZL** | `wert_01` · `wert_02` · `wert_04` · `wert_05` · `wert_06` · `wert_07` |  |
| `SZL_BDSC` | **SZL** · **BDSC** | `wert_01` · `wert_02` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_16` · `wert_17` |  |
| `TB_BDD` | **TB** · **BDD** | `aktiv` · `nicht_aktiv` |  |
| `THETA_ESM` | _THETA_ · _ESM_ | `wert_01` |  |
| `UEBERSTEUERN_MUE_0_2` | survirage · coefficient d'adherence · 0 · 2 | `wert_10` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` · `wert_08` · `wert_09` |  |
| `UEBERSTEUERN_MUE_1_2` | survirage · coefficient d'adherence · 1 · 2 | `wert_10` · `wert_01` · `wert_02` · `wert_03` · `wert_04` · `wert_05` · `wert_06` · `wert_07` … *(+4)* |  |
| `UNTERSTEUERN_MUE_0_2` | sous-virage · coefficient d'adherence · 0 · 2 | `wert_10` · `wert_01` |  |
| `UNTERSTEUERN_MUE_1_2` | sous-virage · coefficient d'adherence · 1 · 2 | `wert_10` · `wert_01` · `wert_02` |  |
| `US_VARIANTE` | marche americain · variante | `aktiv` · `nicht_aktiv` |  |
| `VCH` | **VCH** | `aktiv` · `nicht_aktiv` |  |
| `VCH_C0E` | **VCH** · [variante C0E] | `aktiv` · `nicht_aktiv` |  |
| `VCH_C0F` | **VCH** · [variante C0F] | `aktiv` · `nicht_aktiv` |  |
| `VERZOEG_NEG_FEIN` | temporisation · negatif · fin | `wert_00` · `wert_01` |  |
| `VERZOEG_NEG_GROB` | temporisation · negatif · grossier | `wert_00` · `wert_01` |  |
| `VERZOEG_POS_FEIN` | temporisation · position · fin | `wert_00` · `wert_01` |  |
| `VERZOEG_POS_GROB` | temporisation · position · grossier | `wert_00` · `wert_01` |  |
| `VOLUMENDURCHFLUSSPARA_2` | **VOLUMENDURCHFLUSSPARA** · 2 | `wert_00` · `wert_01` · `wert_02` · `wert_03` |  |
| `V_ABSCHALT` | _V_ · coupure | `wert_00` · `wert_01` · `wert_02` |  |
| `V_EINHEIT` | _V_ · unite | `wert_01` · `wert_02` |  |
| `V_EINHEIT_C0F` | _V_ · unite · [variante C0F] | `wert_00` · `wert_01` |  |

---

**Traduction** : décomposition du nom allemand — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments en **gras** n'ont pas de traduction établie, ceux entre `_underscores_` sont ambigus.

**Valeurs possibles** : extraites des DATEN. `aktiv`/`nicht_aktiv` = interrupteur,
`wert_xx` = valeur numérique, autres = choix nommés.

**Explication** : rédigée uniquement pour les fonctions identifiées avec certitude.
Une case vide ne veut pas dire que la fonction est sans intérêt — seulement que son effet
n'a pas été établi de source sûre.
