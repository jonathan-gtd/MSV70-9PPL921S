# DSC — Controle de stabilite / ABS

🔴 Module `DSC` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🔴 **Module a ne pas coder.** Cette liste est fournie pour comprendre, pas pour modifier.

---

## Source

| | |
|---|---|
| Variante retenue | union de toutes les variantes (variante exacte a confirmer) |
| FSW dans cette variante | **761** |
| FSW toutes variantes confondues | 761 |
| Fichiers DATEN analyses | 26 |

Variantes disponibles : `MK60_87.C03`, `MK60_87.C05`, `MK60_87.C06`, `MK60_87.C07`, `MK60_87.C08`, `MK60_87.C09`, `MK60_87.C0A`, `MK60_87.C0B`, `MK60_87.C0C`, `MK60_87.C0D`, `MK60_87.C0E`, `MK60_87.C0F`, `MK60_87.C20`, `DSC_89.C02`, `DSC_89.C03`, `DSC_84.C01`, `DSC_84.C02`, `DSC_84.C03`, `DSC_84.C04`, `DSC8_RPA.C06`, `DSC8_RPA.C09`, `DSC8_RPA.C0B`, `DSC8_RPA.C0D`, `DSC8_RPA.C0E`, `DSC8_RPA.C0F`, `DSC8_RPA.C10`

> La variante reellement installee sur le vehicule est identifiee par NCS Dummy
> a l'ouverture du `.TRC`. Si elle differe de celle retenue ici, la liste change.

---

## Les 761 fonctions codables

| FSW | Traduction |
|---|---|
| `AFH_DOA` | _AFH_ · **DOA** |
| `AFS` | phares adaptatifs (AFS) |
| `ASL` | _ASL_ |
| `AUSSTATTUNG_RPA` | **AUSSTATTUNG** · indicateur de crevaison |
| `AX_REF` | **AX** · reference |
| `AX_REF_DIFF_LOCK` | **AX** · reference · differentiel · blocage |
| `A_MAX_DSC` | A · maximum · controle de stabilite |
| `A_MIN_DSC` | A · minimum · controle de stabilite |
| `BAUART_MK60` | type de construction · **MK60** |
| `BBV_KORR_HA` | usure des plaquettes · correction · essieu arriere |
| `BBV_KORR_VA` | usure des plaquettes · correction · essieu avant |
| `BB_RAB` | **BB** · precharge des plaquettes |
| `BERGABANSATZ` | amorce de descente |
| `BERGABANSATZ_C0F` | amorce de descente · [index variante C0F] |
| `BERGAB_GERASTET` | descente · crante |
| `BERGAB_GERASTET_C0E` | descente · crante · [index variante C0E] |
| `BERGAB_GERASTET_C0F` | descente · crante · [index variante C0F] |
| `BERGAB_KENNEN` | descente · reconnaissance |
| `BERGAB_KENNEN_C0E` | descente · reconnaissance · [index variante C0E] |
| `BERGAB_KENNEN_C0F` | descente · reconnaissance · [index variante C0F] |
| `BERRGAB_ANSATZ` | **BERRGAB** · **ANSATZ** |
| `BSW` | **BSW** |
| `C02E84_MOTOR_LEISTUNG_MAX` | **C02E84** · moteur · puissance · maximum |
| `C02_E84_BERGAB_GERASTET` | [index variante C02] · **E84** · descente · crante |
| `C02_E84_FZG_MASSE` | [index variante C02] · **E84** · vehicule · masse |
| `C02_E84_SCHUBABSCHALTUNG` | [index variante C02] · **E84** · coupure en deceleration |
| `C03_UNTERSTEUERSCHWELLE_2` | [index variante C03] · seuil de sous-virage · 2 |
| `C04_UEBERSTEUERN_MUE_0_2` | [index variante C04] · survirage · coefficient d'adherence · 0 · 2 |
| `C04_UEBERSTEUERN_MUE_1_2` | [index variante C04] · survirage · coefficient d'adherence · 1 · 2 |
| `C04_UNTERSTEUERSCHWELLE_2` | [index variante C04] · seuil de sous-virage · 2 |
| `C05_BBV` | [index variante C05] · usure des plaquettes |
| `C05_HSA` | [index variante C05] · aide au demarrage en cote |
| `C06_BBV` | [index variante C06] · usure des plaquettes |
| `C07_BAUART_MK60` | [index variante C07] · type de construction · **MK60** |
| `C07_BBV` | [index variante C07] · usure des plaquettes |
| `C07_BBV_KORR_HA` | [index variante C07] · usure des plaquettes · correction · essieu arriere |
| `C07_BBV_KORR_VA` | [index variante C07] · usure des plaquettes · correction · essieu avant |
| `C07_DELTA_PSI_BETA_LIMIT` | [index variante C07] · ecart · **PSI** · _BETA_ · limite |
| `C07_DF_DIR` | [index variante C07] · **DF** · direction |
| `C07_DRUCKMODELL_HINTERACH` | [index variante C07] · modele de pression · **HINTERACH** |
| `C07_DRUCKMODELL_VORDERACH` | [index variante C07] · modele de pression · **VORDERACH** |
| `C07_GMK_ECBA_HA` | [index variante C07] · _GMK_ · _ECBA_ · essieu arriere |
| `C07_GMK_ECBA_VA` | [index variante C07] · _GMK_ · _ECBA_ · essieu avant |
| `C07_HSA` | [index variante C07] · aide au demarrage en cote |
| `C07_MOTOR_ART_DSC_2` | [index variante C07] · moteur · type · controle de stabilite · 2 |
| `C07_UEBERSTEUERN_MUE_0_2` | [index variante C07] · survirage · coefficient d'adherence · 0 · 2 |
| `C07_UEBERSTEUERN_MUE_1_2` | [index variante C07] · survirage · coefficient d'adherence · 1 · 2 |
| `C07_UNTERSTEUERSCHWELLE` | [index variante C07] · seuil de sous-virage |
| `C08_AFH_HSA` | [index variante C08] · _AFH_ · aide au demarrage en cote |
| `C08_AFS` | [index variante C08] · phares adaptatifs (AFS) |
| `C08_ASL` | [index variante C08] · _ASL_ |
| `C08_BAUART` | [index variante C08] · type de construction |
| `C08_BBV` | [index variante C08] · usure des plaquettes |
| `C08_BBV_KORR_HA` | [index variante C08] · usure des plaquettes · correction · essieu arriere |
| `C08_BBV_KORR_VA` | [index variante C08] · usure des plaquettes · correction · essieu avant |
| `C08_BB_RAB` | [index variante C08] · **BB** · precharge des plaquettes |
| `C08_BERGAB_GERASTET` | [index variante C08] · descente · crante |
| `C08_BERGAB_KENNEN` | [index variante C08] · descente · reconnaissance |
| `C08_CBC` | [index variante C08] · controle de freinage en courbe |
| `C08_CSC_DYNAMIC` | [index variante C08] · **CSC** · **DYNAMIC** |
| `C08_DBC_MIN_AUSLOESEDRUCK` | [index variante C08] · _DBC_ · minimum · pression de declenchement |
| `C08_DELTA_PSI_BETA_LIMIT` | [index variante C08] · ecart · **PSI** · _BETA_ · limite |
| `C08_DF_DIR` | [index variante C08] · **DF** · direction |
| `C08_DRUCKMODELL_HA` | [index variante C08] · modele de pression · essieu arriere |
| `C08_DRUCKMODELL_VA` | [index variante C08] · modele de pression · essieu avant |
| `C08_ECBA_DCC` | [index variante C08] · _ECBA_ · _DCC_ |
| `C08_FBS` | [index variante C08] · compensation de fading |
| `C08_FLR` | [index variante C08] · reduction de puissance moteur |
| `C08_FZG_MASSE` | [index variante C08] · vehicule · masse |
| `C08_GETRIEBE` | [index variante C08] · boite de vitesses |
| `C08_GMB` | [index variante C08] · _GMB_ |
| `C08_GMK` | [index variante C08] · _GMK_ |
| `C08_GMK_ECBA_HA` | [index variante C08] · _GMK_ · _ECBA_ · essieu arriere |
| `C08_GMK_ECBA_VA` | [index variante C08] · _GMK_ · _ECBA_ · essieu avant |
| `C08_GMR_MAX_LIMIT_SCHRAEG` | [index variante C08] · regulation du moment de lacet · maximum · limite · derive |
| `C08_GMR_MIN_LIMIT_SCHRAEG` | [index variante C08] · regulation du moment de lacet · minimum · limite · derive |
| `C08_HSA` | [index variante C08] · aide au demarrage en cote |
| `C08_IPM_MAX_P_RES` | [index variante C08] · _IPM_ · maximum · P · remise a zero |
| `C08_LENKWINKELKENNLINIE` | [index variante C08] · courbe d'angle de braquage |
| `C08_LUEFTER` | [index variante C08] · ventilateur |
| `C08_MIN_DRUCKAUFBAU_GRAD` | [index variante C08] · minimum · montee en pression · degre |
| `C08_MOTOR_ART` | [index variante C08] · moteur · type |
| `C08_MOTOR_LEISTUNG_MAX` | [index variante C08] · moteur · puissance · maximum |
| `C08_MOTOR_SCHLEPP_P` | [index variante C08] · moteur · couple de frein moteur · P |
| `C08_PRE_MSR` | [index variante C08] · pre- · regulation du couple de frein moteur |
| `C08_PRE_USC` | [index variante C08] · pre- · _USC_ |
| `C08_RPA` | [index variante C08] · indicateur de crevaison |
| `C08_RPA_SCHWELLE` | [index variante C08] · indicateur de crevaison · seuil |
| `C08_SCHUBABSCHALTUNG` | [index variante C08] · coupure en deceleration |
| `C08_SDR_CTC` | [index variante C08] · _SDR_ · _CTC_ |
| `C08_SETZ_V_MAX` | [index variante C08] · positionnement · _V_ · maximum |
| `C08_SST` | [index variante C08] · _SST_ |
| `C08_TB_BDD` | [index variante C08] · **TB** · **BDD** |
| `C08_UEBERSTEUERN_MUE_0` | [index variante C08] · survirage · coefficient d'adherence · 0 |
| `C08_UEBERSTEUERN_MUE_1_2` | [index variante C08] · survirage · coefficient d'adherence · 1 · 2 |
| `C08_UNTERSTEUERN_MUE_0` | [index variante C08] · sous-virage · coefficient d'adherence · 0 |
| `C08_UNTERSTEUERN_MUE_1_2` | [index variante C08] · sous-virage · coefficient d'adherence · 1 · 2 |
| `C08_UNTERSTEUERSCHWELLE` | [index variante C08] · seuil de sous-virage |
| `C08_VERZOEG_NEG_FEIN` | [index variante C08] · temporisation · negatif · fin |
| `C08_VERZOEG_NEG_GROB` | [index variante C08] · temporisation · negatif · grossier |
| `C08_VERZOEG_POS_FEIN` | [index variante C08] · temporisation · position · fin |
| `C08_VERZOEG_POS_GROB` | [index variante C08] · temporisation · position · grossier |
| `C08_V_EINHEIT` | [index variante C08] · _V_ · unite |
| `C09_ASL` | [index variante C09] · _ASL_ |
| `C09_BAUART` | [index variante C09] · type de construction |
| `C09_BBV` | [index variante C09] · usure des plaquettes |
| `C09_BBV_KORR_HA` | [index variante C09] · usure des plaquettes · correction · essieu arriere |
| `C09_BBV_KORR_VA` | [index variante C09] · usure des plaquettes · correction · essieu avant |
| `C09_BTM` | [index variante C09] · _BTM_ |
| `C09_C8` | [index variante C09] · **C8** |
| `C09_CBC` | [index variante C09] · controle de freinage en courbe |
| `C09_CF_ESM_HB` | [index variante C09] · **CF** · _ESM_ · **HB** |
| `C09_CF_ESM_LB` | [index variante C09] · **CF** · _ESM_ · **LB** |
| `C09_CR_ESM_HB` | [index variante C09] · **CR** · _ESM_ · **HB** |
| `C09_CR_ESM_LB` | [index variante C09] · **CR** · _ESM_ · **LB** |
| `C09_DBC_MIN_AUSLOESEDRUCK` | [index variante C09] · _DBC_ · minimum · pression de declenchement |
| `C09_DELTA_PSI_BETA_LIMIT` | [index variante C09] · ecart · **PSI** · _BETA_ · limite |
| `C09_DIFF_LOCK` | [index variante C09] · differentiel · blocage |
| `C09_DRUCKMODELL_HA` | [index variante C09] · modele de pression · essieu arriere |
| `C09_DRUCKMODELL_VA` | [index variante C09] · modele de pression · essieu avant |
| `C09_FLR` | [index variante C09] · reduction de puissance moteur |
| `C09_GETRIEBE` | [index variante C09] · boite de vitesses |
| `C09_GMB` | [index variante C09] · _GMB_ |
| `C09_GMR_MAX_LIMIT_SCHRAEG` | [index variante C09] · regulation du moment de lacet · maximum · limite · derive |
| `C09_GMR_MIN_LIMIT_SCHRAEG` | [index variante C09] · regulation du moment de lacet · minimum · limite · derive |
| `C09_KUEHLSCHACHT` | [index variante C09] · conduit de refroidissement |
| `C09_LCL` | [index variante C09] · _LCL_ |
| `C09_LENKWINKELKENNLINIE` | [index variante C09] · courbe d'angle de braquage |
| `C09_LF_ESM` | [index variante C09] · **LF** · _ESM_ |
| `C09_LR_ESM` | [index variante C09] · **LR** · _ESM_ |
| `C09_LUEFTER` | [index variante C09] · ventilateur |
| `C09_LWK_X2` | [index variante C09] · angle de braquage · [point d'axe X2] |
| `C09_LWK_X3` | [index variante C09] · angle de braquage · [point d'axe X3] |
| `C09_LWK_X4` | [index variante C09] · angle de braquage · [point d'axe X4] |
| `C09_LWK_Y1` | [index variante C09] · angle de braquage · [point d'axe Y1] |
| `C09_LWK_Y2` | [index variante C09] · angle de braquage · [point d'axe Y2] |
| `C09_LWK_Y3` | [index variante C09] · angle de braquage · [point d'axe Y3] |
| `C09_LWK_Y4` | [index variante C09] · angle de braquage · [point d'axe Y4] |
| `C09_MIN_DRUCKAUFBAU_GRAD` | [index variante C09] · minimum · montee en pression · degre |
| `C09_MOTOR` | [index variante C09] · moteur |
| `C09_M_ESM` | [index variante C09] · M · _ESM_ |
| `C09_PRE_MSR` | [index variante C09] · pre- · regulation du couple de frein moteur |
| `C09_PRE_USC` | [index variante C09] · pre- · _USC_ |
| `C09_RPA` | [index variante C09] · indicateur de crevaison |
| `C09_RPA_SCHWELLE` | [index variante C09] · indicateur de crevaison · seuil |
| `C09_SC_X_ESM` | [index variante C09] · _SC_ · X · _ESM_ |
| `C09_SC_Y_ESM` | [index variante C09] · _SC_ · Y · _ESM_ |
| `C09_SDR_CTC` | [index variante C09] · _SDR_ · _CTC_ |
| `C09_SPURWEITE` | [index variante C09] · voie |
| `C09_THETA_ESM` | [index variante C09] · _THETA_ · _ESM_ |
| `C09_UEBERSTEUERN_MUE_0` | [index variante C09] · survirage · coefficient d'adherence · 0 |
| `C09_UEBERSTEUERN_MUE_1_2` | [index variante C09] · survirage · coefficient d'adherence · 1 · 2 |
| `C09_UNTERSTEUERN_MUE_0` | [index variante C09] · sous-virage · coefficient d'adherence · 0 |
| `C09_UNTERSTEUERN_MUE_1_2` | [index variante C09] · sous-virage · coefficient d'adherence · 1 · 2 |
| `C09_UNTERSTEUERSCHWELLE` | [index variante C09] · seuil de sous-virage |
| `C09_WLC` | [index variante C09] · **WLC** |
| `C0A_AFS` | [index variante C0A] · phares adaptatifs (AFS) |
| `C0A_ASC_AMR_STB` | [index variante C0A] · antipatinage · **AMR** · _STB_ |
| `C0A_ASC_BMR_STB` | [index variante C0A] · antipatinage · **BMR** · _STB_ |
| `C0A_ASL` | [index variante C0A] · _ASL_ |
| `C0A_BAUART` | [index variante C0A] · type de construction |
| `C0A_BBV` | [index variante C0A] · usure des plaquettes |
| `C0A_BBV_KORR_HA` | [index variante C0A] · usure des plaquettes · correction · essieu arriere |
| `C0A_BBV_KORR_VA` | [index variante C0A] · usure des plaquettes · correction · essieu avant |
| `C0A_BERGABANSATZ` | [index variante C0A] · amorce de descente |
| `C0A_BERGAB_GERASTET` | [index variante C0A] · descente · crante |
| `C0A_BERGAB_KENNEN` | [index variante C0A] · descente · reconnaissance |
| `C0A_BTM` | [index variante C0A] · _BTM_ |
| `C0A_CBC` | [index variante C0A] · controle de freinage en courbe |
| `C0A_DBC_MIN_AUSLOESEDRUCK` | [index variante C0A] · _DBC_ · minimum · pression de declenchement |
| `C0A_DCC_C_AAT` | [index variante C0A] · _DCC_ · C · temperature exterieure |
| `C0A_DCC_C_ATT_SCHUB_AUS` | [index variante C0A] · _DCC_ · C · _ATT_ · pied leve / deceleration · arret |
| `C0A_DCC_C_ATT_SCHUB_EIN` | [index variante C0A] · _DCC_ · C · _ATT_ · pied leve / deceleration · marche |
| `C0A_DELTA_PSI_BETA_LIMIT` | [index variante C0A] · ecart · **PSI** · _BETA_ · limite |
| `C0A_DF_DIR` | [index variante C0A] · **DF** · direction |
| `C0A_DIFF_LOCK` | [index variante C0A] · differentiel · blocage |
| `C0A_DRUCKMODELL_HA` | [index variante C0A] · modele de pression · essieu arriere |
| `C0A_DRUCKMODELL_VA` | [index variante C0A] · modele de pression · essieu avant |
| `C0A_ECBA_DCC` | [index variante C0A] · _ECBA_ · _DCC_ |
| `C0A_FLR` | [index variante C0A] · reduction de puissance moteur |
| `C0A_GETRIEBE` | [index variante C0A] · boite de vitesses |
| `C0A_GMB` | [index variante C0A] · _GMB_ |
| `C0A_GMK` | [index variante C0A] · _GMK_ |
| `C0A_GMK_ECBA_HA` | [index variante C0A] · _GMK_ · _ECBA_ · essieu arriere |
| `C0A_GMK_ECBA_VA` | [index variante C0A] · _GMK_ · _ECBA_ · essieu avant |
| `C0A_GMR_MAX_LIMIT_SCHRAEG` | [index variante C0A] · regulation du moment de lacet · maximum · limite · derive |
| `C0A_GMR_MIN_LIMIT_SCHRAEG` | [index variante C0A] · regulation du moment de lacet · minimum · limite · derive |
| `C0A_HSA` | [index variante C0A] · aide au demarrage en cote |
| `C0A_HSA_AFH_AFU` | [index variante C0A] · aide au demarrage en cote · _AFH_ · **AFU** |
| `C0A_IPM_MAX_P_RES` | [index variante C0A] · _IPM_ · maximum · P · remise a zero |
| `C0A_KUEHLSCHACHT` | [index variante C0A] · conduit de refroidissement |
| `C0A_LCL` | [index variante C0A] · _LCL_ |
| `C0A_LENKWINKELKENNLINIE` | [index variante C0A] · courbe d'angle de braquage |
| `C0A_LUEFTER` | [index variante C0A] · ventilateur |
| `C0A_MIN_DRUCKAUFBAU_GRAD` | [index variante C0A] · minimum · montee en pression · degre |
| `C0A_MOTOR` | [index variante C0A] · moteur |
| `C0A_MOTOR_LEISTUNG_MAX` | [index variante C0A] · moteur · puissance · maximum |
| `C0A_MOTOR_SCHLEPP_P` | [index variante C0A] · moteur · couple de frein moteur · P |
| `C0A_PRE_MSR` | [index variante C0A] · pre- · regulation du couple de frein moteur |
| `C0A_PRE_USC` | [index variante C0A] · pre- · _USC_ |
| `C0A_RPA` | [index variante C0A] · indicateur de crevaison |
| `C0A_RPA_SCHWELLE` | [index variante C0A] · indicateur de crevaison · seuil |
| `C0A_SCHUBABSCHALTUNG` | [index variante C0A] · coupure en deceleration |
| `C0A_SDR_CTC` | [index variante C0A] · _SDR_ · _CTC_ |
| `C0A_SETZ_V_MAX` | [index variante C0A] · positionnement · _V_ · maximum |
| `C0A_UEBERSTEUERN_MUE_0` | [index variante C0A] · survirage · coefficient d'adherence · 0 |
| `C0A_UEBERSTEUERN_MUE_1_2` | [index variante C0A] · survirage · coefficient d'adherence · 1 · 2 |
| `C0A_UNTERSTEUERN_MUE_0` | [index variante C0A] · sous-virage · coefficient d'adherence · 0 |
| `C0A_UNTERSTEUERN_MUE_1_2` | [index variante C0A] · sous-virage · coefficient d'adherence · 1 · 2 |
| `C0A_UNTERSTEUERSCHWELLE` | [index variante C0A] · seuil de sous-virage |
| `C0A_VERZOEG_NEG_FEIN` | [index variante C0A] · temporisation · negatif · fin |
| `C0A_VERZOEG_NEG_GROB` | [index variante C0A] · temporisation · negatif · grossier |
| `C0A_VERZOEG_POS_FEIN` | [index variante C0A] · temporisation · position · fin |
| `C0A_VERZOEG_POS_GROB` | [index variante C0A] · temporisation · position · grossier |
| `C0A_V_EINHEIT` | [index variante C0A] · _V_ · unite |
| `C0A_WLC` | [index variante C0A] · **WLC** |
| `C0B_AFH_HSA` | [index variante C0B] · _AFH_ · aide au demarrage en cote |
| `C0B_AFS` | [index variante C0B] · phares adaptatifs (AFS) |
| `C0B_ASC_AMR_STB` | [index variante C0B] · antipatinage · **AMR** · _STB_ |
| `C0B_ASC_BMR_STB` | [index variante C0B] · antipatinage · **BMR** · _STB_ |
| `C0B_ASL` | [index variante C0B] · _ASL_ |
| `C0B_BAUART` | [index variante C0B] · type de construction |
| `C0B_BBV` | [index variante C0B] · usure des plaquettes |
| `C0B_BBV_KORR_HA` | [index variante C0B] · usure des plaquettes · correction · essieu arriere |
| `C0B_BBV_KORR_VA` | [index variante C0B] · usure des plaquettes · correction · essieu avant |
| `C0B_BB_RAB` | [index variante C0B] · **BB** · precharge des plaquettes |
| `C0B_BERGABANSATZ` | [index variante C0B] · amorce de descente |
| `C0B_BERGAB_GERASTET` | [index variante C0B] · descente · crante |
| `C0B_BERGAB_KENNEN` | [index variante C0B] · descente · reconnaissance |
| `C0B_BTM` | [index variante C0B] · _BTM_ |
| `C0B_CBC` | [index variante C0B] · controle de freinage en courbe |
| `C0B_CF_ESM_HB` | [index variante C0B] · **CF** · _ESM_ · **HB** |
| `C0B_CF_ESM_LB` | [index variante C0B] · **CF** · _ESM_ · **LB** |
| `C0B_CR_ESM_HB` | [index variante C0B] · **CR** · _ESM_ · **HB** |
| `C0B_CR_ESM_LB` | [index variante C0B] · **CR** · _ESM_ · **LB** |
| `C0B_DBC_MIN_AUSLOESEDRUCK` | [index variante C0B] · _DBC_ · minimum · pression de declenchement |
| `C0B_DCC_C_AAT` | [index variante C0B] · _DCC_ · C · temperature exterieure |
| `C0B_DCC_C_ATT_SCHUB_AUS` | [index variante C0B] · _DCC_ · C · _ATT_ · pied leve / deceleration · arret |
| `C0B_DCC_C_ATT_SCHUB_EIN` | [index variante C0B] · _DCC_ · C · _ATT_ · pied leve / deceleration · marche |
| `C0B_DCC_DTC` | [index variante C0B] · _DCC_ · controle de traction dynamique |
| `C0B_DELTA_PSI_BETA_LIMIT` | [index variante C0B] · ecart · **PSI** · _BETA_ · limite |
| `C0B_DF_DIR` | [index variante C0B] · **DF** · direction |
| `C0B_DIFF_LOCK` | [index variante C0B] · differentiel · blocage |
| `C0B_DRUCKMODELL_HA` | [index variante C0B] · modele de pression · essieu arriere |
| `C0B_DRUCKMODELL_VA` | [index variante C0B] · modele de pression · essieu avant |
| `C0B_ECBA_DCC` | [index variante C0B] · _ECBA_ · _DCC_ |
| `C0B_FBS` | [index variante C0B] · compensation de fading |
| `C0B_FLR` | [index variante C0B] · reduction de puissance moteur |
| `C0B_GETRIEBE` | [index variante C0B] · boite de vitesses |
| `C0B_GMB` | [index variante C0B] · _GMB_ |
| `C0B_GMK_ECBA_HA` | [index variante C0B] · _GMK_ · _ECBA_ · essieu arriere |
| `C0B_GMK_ECBA_VA` | [index variante C0B] · _GMK_ · _ECBA_ · essieu avant |
| `C0B_GMR_MAX_LIMIT_SCHRAEG` | [index variante C0B] · regulation du moment de lacet · maximum · limite · derive |
| `C0B_GMR_MIN_LIMIT_SCHRAEG` | [index variante C0B] · regulation du moment de lacet · minimum · limite · derive |
| `C0B_HSA` | [index variante C0B] · aide au demarrage en cote |
| `C0B_HSA_AFH_AFU` | [index variante C0B] · aide au demarrage en cote · _AFH_ · **AFU** |
| `C0B_IPM_MAX_P_RES` | [index variante C0B] · _IPM_ · maximum · P · remise a zero |
| `C0B_KUEHLSCHACHT` | [index variante C0B] · conduit de refroidissement |
| `C0B_LCL` | [index variante C0B] · _LCL_ |
| `C0B_LENKWINKELKENNLINIE` | [index variante C0B] · courbe d'angle de braquage |
| `C0B_LF_ESM` | [index variante C0B] · **LF** · _ESM_ |
| `C0B_LR_ESM` | [index variante C0B] · **LR** · _ESM_ |
| `C0B_LUEFTER` | [index variante C0B] · ventilateur |
| `C0B_LWK_X2` | [index variante C0B] · angle de braquage · [point d'axe X2] |
| `C0B_LWK_X3` | [index variante C0B] · angle de braquage · [point d'axe X3] |
| `C0B_LWK_X4` | [index variante C0B] · angle de braquage · [point d'axe X4] |
| `C0B_LWK_Y1` | [index variante C0B] · angle de braquage · [point d'axe Y1] |
| `C0B_LWK_Y2` | [index variante C0B] · angle de braquage · [point d'axe Y2] |
| `C0B_LWK_Y3` | [index variante C0B] · angle de braquage · [point d'axe Y3] |
| `C0B_LWK_Y4` | [index variante C0B] · angle de braquage · [point d'axe Y4] |
| `C0B_MIN_DRUCKAUFBAU_GRAD` | [index variante C0B] · minimum · montee en pression · degre |
| `C0B_MOTOR` | [index variante C0B] · moteur |
| `C0B_MOTOR_LEISTUNG_MAX` | [index variante C0B] · moteur · puissance · maximum |
| `C0B_MOTOR_SCHLEPP_P` | [index variante C0B] · moteur · couple de frein moteur · P |
| `C0B_M_ESM` | [index variante C0B] · M · _ESM_ |
| `C0B_PRE_MSR` | [index variante C0B] · pre- · regulation du couple de frein moteur |
| `C0B_PRE_USC` | [index variante C0B] · pre- · _USC_ |
| `C0B_RPA` | [index variante C0B] · indicateur de crevaison |
| `C0B_RPA_SCHWELLE` | [index variante C0B] · indicateur de crevaison · seuil |
| `C0B_SCHUBABSCHALTUNG` | [index variante C0B] · coupure en deceleration |
| `C0B_SC_X_ESM` | [index variante C0B] · _SC_ · X · _ESM_ |
| `C0B_SC_Y_ESM` | [index variante C0B] · _SC_ · Y · _ESM_ |
| `C0B_SDR_CTC` | [index variante C0B] · _SDR_ · _CTC_ |
| `C0B_SETZ_V_MAX` | [index variante C0B] · positionnement · _V_ · maximum |
| `C0B_SPURWEITE` | [index variante C0B] · voie |
| `C0B_SST` | [index variante C0B] · _SST_ |
| `C0B_TB_BDD` | [index variante C0B] · **TB** · **BDD** |
| `C0B_THETA_ESM` | [index variante C0B] · _THETA_ · _ESM_ |
| `C0B_UEBERSTEUERN_MUE_0` | [index variante C0B] · survirage · coefficient d'adherence · 0 |
| `C0B_UEBERSTEUERN_MUE_1_2` | [index variante C0B] · survirage · coefficient d'adherence · 1 · 2 |
| `C0B_UNTERSTEUERN_MUE_0` | [index variante C0B] · sous-virage · coefficient d'adherence · 0 |
| `C0B_UNTERSTEUERN_MUE_1_2` | [index variante C0B] · sous-virage · coefficient d'adherence · 1 · 2 |
| `C0B_UNTERSTEUERSCHWELLE` | [index variante C0B] · seuil de sous-virage |
| `C0B_VERZOEG_NEG_FEIN` | [index variante C0B] · temporisation · negatif · fin |
| `C0B_VERZOEG_NEG_GROB` | [index variante C0B] · temporisation · negatif · grossier |
| `C0B_VERZOEG_POS_FEIN` | [index variante C0B] · temporisation · position · fin |
| `C0B_VERZOEG_POS_GROB` | [index variante C0B] · temporisation · position · grossier |
| `C0B_V_EINHEIT` | [index variante C0B] · _V_ · unite |
| `C0C_AFH_HSA` | [index variante C0C] · _AFH_ · aide au demarrage en cote |
| `C0C_AFS` | [index variante C0C] · phares adaptatifs (AFS) |
| `C0C_ASC_AMR_STB` | [index variante C0C] · antipatinage · **AMR** · _STB_ |
| `C0C_ASC_BMR_STB` | [index variante C0C] · antipatinage · **BMR** · _STB_ |
| `C0C_ASL` | [index variante C0C] · _ASL_ |
| `C0C_BAUART` | [index variante C0C] · type de construction |
| `C0C_BBV` | [index variante C0C] · usure des plaquettes |
| `C0C_BBV_KORR_HA` | [index variante C0C] · usure des plaquettes · correction · essieu arriere |
| `C0C_BBV_KORR_VA` | [index variante C0C] · usure des plaquettes · correction · essieu avant |
| `C0C_BB_RAB` | [index variante C0C] · **BB** · precharge des plaquettes |
| `C0C_BERGABANSATZ` | [index variante C0C] · amorce de descente |
| `C0C_BERGAB_GERASTET` | [index variante C0C] · descente · crante |
| `C0C_BERGAB_KENNEN` | [index variante C0C] · descente · reconnaissance |
| `C0C_BTM` | [index variante C0C] · _BTM_ |
| `C0C_CBC` | [index variante C0C] · controle de freinage en courbe |
| `C0C_CF_ESM_HB` | [index variante C0C] · **CF** · _ESM_ · **HB** |
| `C0C_CF_ESM_LB` | [index variante C0C] · **CF** · _ESM_ · **LB** |
| `C0C_CR_ESM_HB` | [index variante C0C] · **CR** · _ESM_ · **HB** |
| `C0C_CR_ESM_LB` | [index variante C0C] · **CR** · _ESM_ · **LB** |
| `C0C_DBC_MIN_AUSLOESEDRUCK` | [index variante C0C] · _DBC_ · minimum · pression de declenchement |
| `C0C_DCC_C_AAT` | [index variante C0C] · _DCC_ · C · temperature exterieure |
| `C0C_DCC_C_ATT_SCHUB_AUS` | [index variante C0C] · _DCC_ · C · _ATT_ · pied leve / deceleration · arret |
| `C0C_DCC_C_ATT_SCHUB_EIN` | [index variante C0C] · _DCC_ · C · _ATT_ · pied leve / deceleration · marche |
| `C0C_DCC_DTC` | [index variante C0C] · _DCC_ · controle de traction dynamique |
| `C0C_DELTA_PSI_BETA_LIMIT` | [index variante C0C] · ecart · **PSI** · _BETA_ · limite |
| `C0C_DF_DIR` | [index variante C0C] · **DF** · direction |
| `C0C_DIFF_LOCK` | [index variante C0C] · differentiel · blocage |
| `C0C_DRUCKMODELL_HA` | [index variante C0C] · modele de pression · essieu arriere |
| `C0C_DRUCKMODELL_VA` | [index variante C0C] · modele de pression · essieu avant |
| `C0C_ECBA_DCC` | [index variante C0C] · _ECBA_ · _DCC_ |
| `C0C_FBS` | [index variante C0C] · compensation de fading |
| `C0C_FLR` | [index variante C0C] · reduction de puissance moteur |
| `C0C_GETRIEBE` | [index variante C0C] · boite de vitesses |
| `C0C_GMB` | [index variante C0C] · _GMB_ |
| `C0C_GMK_ECBA_HA` | [index variante C0C] · _GMK_ · _ECBA_ · essieu arriere |
| `C0C_GMK_ECBA_VA` | [index variante C0C] · _GMK_ · _ECBA_ · essieu avant |
| `C0C_GMR_MAX_LIMIT_SCHRAEG` | [index variante C0C] · regulation du moment de lacet · maximum · limite · derive |
| `C0C_GMR_MIN_LIMIT_SCHRAEG` | [index variante C0C] · regulation du moment de lacet · minimum · limite · derive |
| `C0C_HSA` | [index variante C0C] · aide au demarrage en cote |
| `C0C_HSA_AFH_AFU` | [index variante C0C] · aide au demarrage en cote · _AFH_ · **AFU** |
| `C0C_IPM_MAX_P_RES` | [index variante C0C] · _IPM_ · maximum · P · remise a zero |
| `C0C_KUEHLSCHACHT` | [index variante C0C] · conduit de refroidissement |
| `C0C_LCL` | [index variante C0C] · _LCL_ |
| `C0C_LENKWINKELKENNLINIE` | [index variante C0C] · courbe d'angle de braquage |
| `C0C_LF_ESM` | [index variante C0C] · **LF** · _ESM_ |
| `C0C_LR_ESM` | [index variante C0C] · **LR** · _ESM_ |
| `C0C_LUEFTER` | [index variante C0C] · ventilateur |
| `C0C_LWK_X2` | [index variante C0C] · angle de braquage · [point d'axe X2] |
| `C0C_LWK_X3` | [index variante C0C] · angle de braquage · [point d'axe X3] |
| `C0C_LWK_X4` | [index variante C0C] · angle de braquage · [point d'axe X4] |
| `C0C_LWK_Y1` | [index variante C0C] · angle de braquage · [point d'axe Y1] |
| `C0C_LWK_Y2` | [index variante C0C] · angle de braquage · [point d'axe Y2] |
| `C0C_LWK_Y3` | [index variante C0C] · angle de braquage · [point d'axe Y3] |
| `C0C_LWK_Y4` | [index variante C0C] · angle de braquage · [point d'axe Y4] |
| `C0C_MIN_DRUCKAUFBAU_GRAD` | [index variante C0C] · minimum · montee en pression · degre |
| `C0C_MOTOR` | [index variante C0C] · moteur |
| `C0C_MOTOR_LEISTUNG_MAX` | [index variante C0C] · moteur · puissance · maximum |
| `C0C_MOTOR_SCHLEPP_P` | [index variante C0C] · moteur · couple de frein moteur · P |
| `C0C_M_ESM` | [index variante C0C] · M · _ESM_ |
| `C0C_PRE_MSR` | [index variante C0C] · pre- · regulation du couple de frein moteur |
| `C0C_PRE_USC` | [index variante C0C] · pre- · _USC_ |
| `C0C_RPA` | [index variante C0C] · indicateur de crevaison |
| `C0C_RPA_SCHWELLE` | [index variante C0C] · indicateur de crevaison · seuil |
| `C0C_SCHUBABSCHALTUNG` | [index variante C0C] · coupure en deceleration |
| `C0C_SC_X_ESM` | [index variante C0C] · _SC_ · X · _ESM_ |
| `C0C_SC_Y_ESM` | [index variante C0C] · _SC_ · Y · _ESM_ |
| `C0C_SDR_CTC` | [index variante C0C] · _SDR_ · _CTC_ |
| `C0C_SETZ_V_MAX` | [index variante C0C] · positionnement · _V_ · maximum |
| `C0C_SPURWEITE` | [index variante C0C] · voie |
| `C0C_SST` | [index variante C0C] · _SST_ |
| `C0C_TB_BDD` | [index variante C0C] · **TB** · **BDD** |
| `C0C_THETA_ESM` | [index variante C0C] · _THETA_ · _ESM_ |
| `C0C_UEBERSTEUERN_MUE_0` | [index variante C0C] · survirage · coefficient d'adherence · 0 |
| `C0C_UEBERSTEUERN_MUE_1_2` | [index variante C0C] · survirage · coefficient d'adherence · 1 · 2 |
| `C0C_UNTERSTEUERN_MUE_0` | [index variante C0C] · sous-virage · coefficient d'adherence · 0 |
| `C0C_UNTERSTEUERN_MUE_1_2` | [index variante C0C] · sous-virage · coefficient d'adherence · 1 · 2 |
| `C0C_UNTERSTEUERSCHWELLE` | [index variante C0C] · seuil de sous-virage |
| `C0C_VERZOEG_NEG_FEIN` | [index variante C0C] · temporisation · negatif · fin |
| `C0C_VERZOEG_NEG_GROB` | [index variante C0C] · temporisation · negatif · grossier |
| `C0C_VERZOEG_POS_FEIN` | [index variante C0C] · temporisation · position · fin |
| `C0C_VERZOEG_POS_GROB` | [index variante C0C] · temporisation · position · grossier |
| `C0C_V_EINHEIT` | [index variante C0C] · _V_ · unite |
| `C0D_ASL` | [index variante C0D] · _ASL_ |
| `C0D_BAUART` | [index variante C0D] · type de construction |
| `C0D_BBV` | [index variante C0D] · usure des plaquettes |
| `C0D_BBV_KORR_HA` | [index variante C0D] · usure des plaquettes · correction · essieu arriere |
| `C0D_BBV_KORR_VA` | [index variante C0D] · usure des plaquettes · correction · essieu avant |
| `C0D_BTM` | [index variante C0D] · _BTM_ |
| `C0D_CBC` | [index variante C0D] · controle de freinage en courbe |
| `C0D_CF_ESM_HB` | [index variante C0D] · **CF** · _ESM_ · **HB** |
| `C0D_CF_ESM_LB` | [index variante C0D] · **CF** · _ESM_ · **LB** |
| `C0D_CR_ESM_HB` | [index variante C0D] · **CR** · _ESM_ · **HB** |
| `C0D_CR_ESM_LB` | [index variante C0D] · **CR** · _ESM_ · **LB** |
| `C0D_DBC_MIN_AUSLOESEDRUCK` | [index variante C0D] · _DBC_ · minimum · pression de declenchement |
| `C0D_DELTA_PSI_BETA_LIMIT` | [index variante C0D] · ecart · **PSI** · _BETA_ · limite |
| `C0D_DIFF_LOCK` | [index variante C0D] · differentiel · blocage |
| `C0D_DRUCKMODELL_HA` | [index variante C0D] · modele de pression · essieu arriere |
| `C0D_DRUCKMODELL_VA` | [index variante C0D] · modele de pression · essieu avant |
| `C0D_FLR` | [index variante C0D] · reduction de puissance moteur |
| `C0D_GETRIEBE` | [index variante C0D] · boite de vitesses |
| `C0D_GMB` | [index variante C0D] · _GMB_ |
| `C0D_GMR_MAX_LIMIT_SCHRAEG` | [index variante C0D] · regulation du moment de lacet · maximum · limite · derive |
| `C0D_GMR_MIN_LIMIT_SCHRAEG` | [index variante C0D] · regulation du moment de lacet · minimum · limite · derive |
| `C0D_KUEHLSCHACHT` | [index variante C0D] · conduit de refroidissement |
| `C0D_LCL` | [index variante C0D] · _LCL_ |
| `C0D_LENKWINKELKENNLINIE` | [index variante C0D] · courbe d'angle de braquage |
| `C0D_LF_ESM` | [index variante C0D] · **LF** · _ESM_ |
| `C0D_LR_ESM` | [index variante C0D] · **LR** · _ESM_ |
| `C0D_LUEFTER` | [index variante C0D] · ventilateur |
| `C0D_LWK_X2` | [index variante C0D] · angle de braquage · [point d'axe X2] |
| `C0D_LWK_X3` | [index variante C0D] · angle de braquage · [point d'axe X3] |
| `C0D_LWK_X4` | [index variante C0D] · angle de braquage · [point d'axe X4] |
| `C0D_LWK_Y1` | [index variante C0D] · angle de braquage · [point d'axe Y1] |
| `C0D_LWK_Y2` | [index variante C0D] · angle de braquage · [point d'axe Y2] |
| `C0D_LWK_Y3` | [index variante C0D] · angle de braquage · [point d'axe Y3] |
| `C0D_LWK_Y4` | [index variante C0D] · angle de braquage · [point d'axe Y4] |
| `C0D_MIN_DRUCKAUFBAU_GRAD` | [index variante C0D] · minimum · montee en pression · degre |
| `C0D_MOTOR` | [index variante C0D] · moteur |
| `C0D_M_ESM` | [index variante C0D] · M · _ESM_ |
| `C0D_PRE_MSR` | [index variante C0D] · pre- · regulation du couple de frein moteur |
| `C0D_PRE_USC` | [index variante C0D] · pre- · _USC_ |
| `C0D_RPA` | [index variante C0D] · indicateur de crevaison |
| `C0D_RPA_SCHWELLE` | [index variante C0D] · indicateur de crevaison · seuil |
| `C0D_SC_X_ESM` | [index variante C0D] · _SC_ · X · _ESM_ |
| `C0D_SC_Y_ESM` | [index variante C0D] · _SC_ · Y · _ESM_ |
| `C0D_SDR_CTC` | [index variante C0D] · _SDR_ · _CTC_ |
| `C0D_SPURWEITE` | [index variante C0D] · voie |
| `C0D_THETA_ESM` | [index variante C0D] · _THETA_ · _ESM_ |
| `C0D_UEBERSTEUERN_MUE_0` | [index variante C0D] · survirage · coefficient d'adherence · 0 |
| `C0D_UEBERSTEUERN_MUE_1_2` | [index variante C0D] · survirage · coefficient d'adherence · 1 · 2 |
| `C0D_UNTERSTEUERN_MUE_0` | [index variante C0D] · sous-virage · coefficient d'adherence · 0 |
| `C0D_UNTERSTEUERN_MUE_1_2` | [index variante C0D] · sous-virage · coefficient d'adherence · 1 · 2 |
| `C0D_UNTERSTEUERSCHWELLE` | [index variante C0D] · seuil de sous-virage |
| `C0D_WLC` | [index variante C0D] · **WLC** |
| `C0E_ASL` | [index variante C0E] · _ASL_ |
| `C0E_BAUART` | [index variante C0E] · type de construction |
| `C0E_BBV` | [index variante C0E] · usure des plaquettes |
| `C0E_BBV_KORR_HA` | [index variante C0E] · usure des plaquettes · correction · essieu arriere |
| `C0E_BBV_KORR_VA` | [index variante C0E] · usure des plaquettes · correction · essieu avant |
| `C0E_BTM` | [index variante C0E] · _BTM_ |
| `C0E_CBC` | [index variante C0E] · controle de freinage en courbe |
| `C0E_CF_ESM_HB` | [index variante C0E] · **CF** · _ESM_ · **HB** |
| `C0E_CF_ESM_LB` | [index variante C0E] · **CF** · _ESM_ · **LB** |
| `C0E_CR_ESM_HB` | [index variante C0E] · **CR** · _ESM_ · **HB** |
| `C0E_CR_ESM_LB` | [index variante C0E] · **CR** · _ESM_ · **LB** |
| `C0E_DBC_MIN_AUSLOESEDRUCK` | [index variante C0E] · _DBC_ · minimum · pression de declenchement |
| `C0E_DELTA_PSI_BETA_LIMIT` | [index variante C0E] · ecart · **PSI** · _BETA_ · limite |
| `C0E_DIFF_LOCK` | [index variante C0E] · differentiel · blocage |
| `C0E_DRUCKMODELL_HA` | [index variante C0E] · modele de pression · essieu arriere |
| `C0E_DRUCKMODELL_VA` | [index variante C0E] · modele de pression · essieu avant |
| `C0E_FLR` | [index variante C0E] · reduction de puissance moteur |
| `C0E_GETRIEBE` | [index variante C0E] · boite de vitesses |
| `C0E_GMB` | [index variante C0E] · _GMB_ |
| `C0E_GMR_MAX_LIMIT_SCHRAEG` | [index variante C0E] · regulation du moment de lacet · maximum · limite · derive |
| `C0E_GMR_MIN_LIMIT_SCHRAEG` | [index variante C0E] · regulation du moment de lacet · minimum · limite · derive |
| `C0E_KUEHLSCHACHT` | [index variante C0E] · conduit de refroidissement |
| `C0E_LCL` | [index variante C0E] · _LCL_ |
| `C0E_LENKWINKELKENNLINIE` | [index variante C0E] · courbe d'angle de braquage |
| `C0E_LF_ESM` | [index variante C0E] · **LF** · _ESM_ |
| `C0E_LR_ESM` | [index variante C0E] · **LR** · _ESM_ |
| `C0E_LUEFTER` | [index variante C0E] · ventilateur |
| `C0E_LWK_X2` | [index variante C0E] · angle de braquage · [point d'axe X2] |
| `C0E_LWK_X3` | [index variante C0E] · angle de braquage · [point d'axe X3] |
| `C0E_LWK_X4` | [index variante C0E] · angle de braquage · [point d'axe X4] |
| `C0E_LWK_Y1` | [index variante C0E] · angle de braquage · [point d'axe Y1] |
| `C0E_LWK_Y2` | [index variante C0E] · angle de braquage · [point d'axe Y2] |
| `C0E_LWK_Y3` | [index variante C0E] · angle de braquage · [point d'axe Y3] |
| `C0E_LWK_Y4` | [index variante C0E] · angle de braquage · [point d'axe Y4] |
| `C0E_MIN_DRUCKAUFBAU_GRAD` | [index variante C0E] · minimum · montee en pression · degre |
| `C0E_MOTOR` | [index variante C0E] · moteur |
| `C0E_M_ESM` | [index variante C0E] · M · _ESM_ |
| `C0E_PRE_MSR` | [index variante C0E] · pre- · regulation du couple de frein moteur |
| `C0E_PRE_USC` | [index variante C0E] · pre- · _USC_ |
| `C0E_RPA` | [index variante C0E] · indicateur de crevaison |
| `C0E_RPA_SCHWELLE` | [index variante C0E] · indicateur de crevaison · seuil |
| `C0E_SC_X_ESM` | [index variante C0E] · _SC_ · X · _ESM_ |
| `C0E_SC_Y_ESM` | [index variante C0E] · _SC_ · Y · _ESM_ |
| `C0E_SDR_CTC` | [index variante C0E] · _SDR_ · _CTC_ |
| `C0E_SPURWEITE` | [index variante C0E] · voie |
| `C0E_THETA_ESM` | [index variante C0E] · _THETA_ · _ESM_ |
| `C0E_UEBERSTEUERN_MUE_0` | [index variante C0E] · survirage · coefficient d'adherence · 0 |
| `C0E_UEBERSTEUERN_MUE_1_2` | [index variante C0E] · survirage · coefficient d'adherence · 1 · 2 |
| `C0E_UNTERSTEUERN_MUE_0` | [index variante C0E] · sous-virage · coefficient d'adherence · 0 |
| `C0E_UNTERSTEUERN_MUE_1_2` | [index variante C0E] · sous-virage · coefficient d'adherence · 1 · 2 |
| `C0E_UNTERSTEUERSCHWELLE` | [index variante C0E] · seuil de sous-virage |
| `C0E_WLC` | [index variante C0E] · **WLC** |
| `C0F_AFH_HSA` | [index variante C0F] · _AFH_ · aide au demarrage en cote |
| `C0F_AFS` | [index variante C0F] · phares adaptatifs (AFS) |
| `C0F_ASC_AMR_STB` | [index variante C0F] · antipatinage · **AMR** · _STB_ |
| `C0F_ASC_BMR_STB` | [index variante C0F] · antipatinage · **BMR** · _STB_ |
| `C0F_ASL` | [index variante C0F] · _ASL_ |
| `C0F_BAUART` | [index variante C0F] · type de construction |
| `C0F_BBV` | [index variante C0F] · usure des plaquettes |
| `C0F_BBV_KORR_HA` | [index variante C0F] · usure des plaquettes · correction · essieu arriere |
| `C0F_BBV_KORR_VA` | [index variante C0F] · usure des plaquettes · correction · essieu avant |
| `C0F_BB_RAB` | [index variante C0F] · **BB** · precharge des plaquettes |
| `C0F_BERGABANSATZ` | [index variante C0F] · amorce de descente |
| `C0F_BERGAB_GERASTET` | [index variante C0F] · descente · crante |
| `C0F_BERGAB_KENNEN` | [index variante C0F] · descente · reconnaissance |
| `C0F_BTM` | [index variante C0F] · _BTM_ |
| `C0F_CBC` | [index variante C0F] · controle de freinage en courbe |
| `C0F_CF_ESM_HB` | [index variante C0F] · **CF** · _ESM_ · **HB** |
| `C0F_CF_ESM_LB` | [index variante C0F] · **CF** · _ESM_ · **LB** |
| `C0F_CR_ESM_HB` | [index variante C0F] · **CR** · _ESM_ · **HB** |
| `C0F_CR_ESM_LB` | [index variante C0F] · **CR** · _ESM_ · **LB** |
| `C0F_DBC_MIN_AUSLOESEDRUCK` | [index variante C0F] · _DBC_ · minimum · pression de declenchement |
| `C0F_DCC_C_AAT` | [index variante C0F] · _DCC_ · C · temperature exterieure |
| `C0F_DCC_C_ATT_SCHUB_AUS` | [index variante C0F] · _DCC_ · C · _ATT_ · pied leve / deceleration · arret |
| `C0F_DCC_C_ATT_SCHUB_EIN` | [index variante C0F] · _DCC_ · C · _ATT_ · pied leve / deceleration · marche |
| `C0F_DCC_DTC` | [index variante C0F] · _DCC_ · controle de traction dynamique |
| `C0F_DELTA_PSI_BETA_LIMIT` | [index variante C0F] · ecart · **PSI** · _BETA_ · limite |
| `C0F_DF_DIR` | [index variante C0F] · **DF** · direction |
| `C0F_DIFF_LOCK` | [index variante C0F] · differentiel · blocage |
| `C0F_DRUCKMODELL_HA` | [index variante C0F] · modele de pression · essieu arriere |
| `C0F_DRUCKMODELL_VA` | [index variante C0F] · modele de pression · essieu avant |
| `C0F_ECBA_DCC` | [index variante C0F] · _ECBA_ · _DCC_ |
| `C0F_FBS` | [index variante C0F] · compensation de fading |
| `C0F_FLR` | [index variante C0F] · reduction de puissance moteur |
| `C0F_GETRIEBE` | [index variante C0F] · boite de vitesses |
| `C0F_GMB` | [index variante C0F] · _GMB_ |
| `C0F_GMK_ECBA_HA` | [index variante C0F] · _GMK_ · _ECBA_ · essieu arriere |
| `C0F_GMK_ECBA_VA` | [index variante C0F] · _GMK_ · _ECBA_ · essieu avant |
| `C0F_GMR_MAX_LIMIT_SCHRAEG` | [index variante C0F] · regulation du moment de lacet · maximum · limite · derive |
| `C0F_GMR_MIN_LIMIT_SCHRAEG` | [index variante C0F] · regulation du moment de lacet · minimum · limite · derive |
| `C0F_HSA` | [index variante C0F] · aide au demarrage en cote |
| `C0F_HSA_AFH_AFU` | [index variante C0F] · aide au demarrage en cote · _AFH_ · **AFU** |
| `C0F_IPM_MAX_P_RES` | [index variante C0F] · _IPM_ · maximum · P · remise a zero |
| `C0F_KUEHLSCHACHT` | [index variante C0F] · conduit de refroidissement |
| `C0F_LCL` | [index variante C0F] · _LCL_ |
| `C0F_LENKWINKELKENNLINIE` | [index variante C0F] · courbe d'angle de braquage |
| `C0F_LF_ESM` | [index variante C0F] · **LF** · _ESM_ |
| `C0F_LR_ESM` | [index variante C0F] · **LR** · _ESM_ |
| `C0F_LUEFTER` | [index variante C0F] · ventilateur |
| `C0F_LWK_X2` | [index variante C0F] · angle de braquage · [point d'axe X2] |
| `C0F_LWK_X3` | [index variante C0F] · angle de braquage · [point d'axe X3] |
| `C0F_LWK_X4` | [index variante C0F] · angle de braquage · [point d'axe X4] |
| `C0F_LWK_Y1` | [index variante C0F] · angle de braquage · [point d'axe Y1] |
| `C0F_LWK_Y2` | [index variante C0F] · angle de braquage · [point d'axe Y2] |
| `C0F_LWK_Y3` | [index variante C0F] · angle de braquage · [point d'axe Y3] |
| `C0F_LWK_Y4` | [index variante C0F] · angle de braquage · [point d'axe Y4] |
| `C0F_MIN_DRUCKAUFBAU_GRAD` | [index variante C0F] · minimum · montee en pression · degre |
| `C0F_MOTOR` | [index variante C0F] · moteur |
| `C0F_MOTOR_LEISTUNG_MAX` | [index variante C0F] · moteur · puissance · maximum |
| `C0F_MOTOR_SCHLEPP_P` | [index variante C0F] · moteur · couple de frein moteur · P |
| `C0F_M_ESM` | [index variante C0F] · M · _ESM_ |
| `C0F_PRE_MSR` | [index variante C0F] · pre- · regulation du couple de frein moteur |
| `C0F_PRE_USC` | [index variante C0F] · pre- · _USC_ |
| `C0F_RPA` | [index variante C0F] · indicateur de crevaison |
| `C0F_RPA_SCHWELLE` | [index variante C0F] · indicateur de crevaison · seuil |
| `C0F_SCHUBABSCHALTUNG` | [index variante C0F] · coupure en deceleration |
| `C0F_SC_X_ESM` | [index variante C0F] · _SC_ · X · _ESM_ |
| `C0F_SC_Y_ESM` | [index variante C0F] · _SC_ · Y · _ESM_ |
| `C0F_SDR_CTC` | [index variante C0F] · _SDR_ · _CTC_ |
| `C0F_SETZ_V_MAX` | [index variante C0F] · positionnement · _V_ · maximum |
| `C0F_SPURWEITE` | [index variante C0F] · voie |
| `C0F_SST` | [index variante C0F] · _SST_ |
| `C0F_TB_BDD` | [index variante C0F] · **TB** · **BDD** |
| `C0F_THETA_ESM` | [index variante C0F] · _THETA_ · _ESM_ |
| `C0F_UEBERSTEUERN_MUE_0` | [index variante C0F] · survirage · coefficient d'adherence · 0 |
| `C0F_UEBERSTEUERN_MUE_1_2` | [index variante C0F] · survirage · coefficient d'adherence · 1 · 2 |
| `C0F_UNTERSTEUERN_MUE_0` | [index variante C0F] · sous-virage · coefficient d'adherence · 0 |
| `C0F_UNTERSTEUERN_MUE_1_2` | [index variante C0F] · sous-virage · coefficient d'adherence · 1 · 2 |
| `C0F_UNTERSTEUERSCHWELLE` | [index variante C0F] · seuil de sous-virage |
| `C0F_VERZOEG_NEG_FEIN` | [index variante C0F] · temporisation · negatif · fin |
| `C0F_VERZOEG_NEG_GROB` | [index variante C0F] · temporisation · negatif · grossier |
| `C0F_VERZOEG_POS_FEIN` | [index variante C0F] · temporisation · position · fin |
| `C0F_VERZOEG_POS_GROB` | [index variante C0F] · temporisation · position · grossier |
| `C0F_V_EINHEIT` | [index variante C0F] · _V_ · unite |
| `C0X_C8` | **C0X** · **C8** |
| `C10C_AAT_SCHUB_AUS_OFFSET` | **C10C** · temperature exterieure · pied leve / deceleration · arret · decalage |
| `C10C_AAT_SCHUB_EIN_KORR` | **C10C** · temperature exterieure · pied leve / deceleration · marche · correction |
| `C10C_AAT_SCHUB_EIN_OFFSET` | **C10C** · temperature exterieure · pied leve / deceleration · marche · decalage |
| `C10_A_MAX` | [index variante C10] · A · maximum |
| `C10_A_MIN` | [index variante C10] · A · minimum |
| `C10_BERGABANSATZ` | [index variante C10] · amorce de descente |
| `C10_BERGAB_GERASTET` | [index variante C10] · descente · crante |
| `C10_BERGAB_KENNEN` | [index variante C10] · descente · reconnaissance |
| `C10_DTC_DCC` | [index variante C10] · controle de traction dynamique · _DCC_ |
| `C10_ECBA_DCC` | [index variante C10] · _ECBA_ · _DCC_ |
| `C10_FZG_MASSE` | [index variante C10] · vehicule · masse |
| `C10_IPM_MAX_P_RES` | [index variante C10] · _IPM_ · maximum · P · remise a zero |
| `C10_MOTOR_LEISTUNG_MAX` | [index variante C10] · moteur · puissance · maximum |
| `C10_P_AUSROLL_ANTRIEB` | [index variante C10] · P · roue libre · transmission |
| `C10_R_RAD_DYN_DSC` | [index variante C10] · _R_ · roue · dynamique · controle de stabilite |
| `C10_SCHUBABSCHALTUNG` | [index variante C10] · coupure en deceleration |
| `C10_SETZ_V_MAX_CC` | [index variante C10] · positionnement · _V_ · maximum · Check Control |
| `C10_SETZ_V_MIN` | [index variante C10] · positionnement · _V_ · minimum |
| `C10_VERZOEG_NEG_FEIN` | [index variante C10] · temporisation · negatif · fin |
| `C10_VERZOEG_NEG_GROB` | [index variante C10] · temporisation · negatif · grossier |
| `C10_VERZOEG_POS_FEIN` | [index variante C10] · temporisation · position · fin |
| `C10_VERZOEG_POS_GROB` | [index variante C10] · temporisation · position · grossier |
| `C10_V_ABSCHALT` | [index variante C10] · _V_ · coupure |
| `C10_V_EINHEIT` | [index variante C10] · _V_ · unite |
| `C8` | **C8** |
| `C8_BDSC` | **C8** · **BDSC** |
| `CF_ESM_HB` | **CF** · _ESM_ · **HB** |
| `CF_ESM_LB` | **CF** · _ESM_ · **LB** |
| `CHECKSUM` | somme de controle |
| `CR_ESM_HB` | **CR** · _ESM_ · **HB** |
| `CR_ESM_LB` | **CR** · _ESM_ · **LB** |
| `C_AAT_SCHUB_AUS_OFFSET` | C · temperature exterieure · pied leve / deceleration · arret · decalage |
| `C_AAT_SCHUB_EIN_KORR_05` | C · temperature exterieure · pied leve / deceleration · marche · correction · 05 |
| `C_AAT_SCHUB_EIN_KORR_C0F` | C · temperature exterieure · pied leve / deceleration · marche · correction · [index variante C0F] |
| `C_AAT_SCHUB_EIN_KORR_SKAL` | C · temperature exterieure · pied leve / deceleration · marche · correction · **SKAL** |
| `C_AAT_SCHUB_EIN_OFFSET` | C · temperature exterieure · pied leve / deceleration · marche · decalage |
| `DBC_MIN_AUFBAUGRADIENT` | _DBC_ · minimum · **AUFBAUGRADIENT** |
| `DBC_MIN_AUSLOESEDRUCK` | _DBC_ · minimum · pression de declenchement |
| `DCC_DTC` | _DCC_ · controle de traction dynamique |
| `DELTA_PSI_BETA_LIMITKORR2` | ecart · **PSI** · _BETA_ · **LIMITKORR2** |
| `DF_DIR` | **DF** · direction |
| `DIFF_LOCK` | differentiel · blocage |
| `DRUCKMODELL_HINTERACHSE` | modele de pression · **HINTERACHSE** |
| `DRUCKMODELL_VORDERACHSE` | modele de pression · **VORDERACHSE** |
| `DSC8_SST` | **DSC8** · _SST_ |
| `DTC_DCC` | controle de traction dynamique · _DCC_ |
| `DXC_CBS` | _DXC_ · entretien selon besoin (CBS) |
| `E84C_AAT_SCHUB_AUS_OFFSET` | **E84C** · temperature exterieure · pied leve / deceleration · arret · decalage |
| `E84C_AAT_SCHUB_EIN_KORR` | **E84C** · temperature exterieure · pied leve / deceleration · marche · correction |
| `E84C_AAT_SCHUB_EIN_OFFSET` | **E84C** · temperature exterieure · pied leve / deceleration · marche · decalage |
| `E84_AAT_SCHUB_AUS_OFFSET` | **E84** · temperature exterieure · pied leve / deceleration · arret · decalage |
| `E84_AAT_SCHUB_EIN_KORR` | **E84** · temperature exterieure · pied leve / deceleration · marche · correction |
| `E84_AAT_SCHUB_EIN_OFFSET` | **E84** · temperature exterieure · pied leve / deceleration · marche · decalage |
| `E84_ASL` | **E84** · _ASL_ |
| `E84_AX_REF_DIFF_LOCK` | **E84** · **AX** · reference · differentiel · blocage |
| `E84_A_MAX` | **E84** · A · maximum |
| `E84_A_MAX_DSC` | **E84** · A · maximum · controle de stabilite |
| `E84_A_MIN` | **E84** · A · minimum |
| `E84_A_MIN_DSC` | **E84** · A · minimum · controle de stabilite |
| `E84_BERGABANSATZ` | **E84** · amorce de descente |
| `E84_BERGAB_GERASTET` | **E84** · descente · crante |
| `E84_BERGAB_KENNEN` | **E84** · descente · reconnaissance |
| `E84_BSW` | **E84** · **BSW** |
| `E84_BSW_HO` | **E84** · **BSW** · **HO** |
| `E84_C8_BDSC` | **E84** · **C8** · **BDSC** |
| `E84_CBS` | **E84** · entretien selon besoin (CBS) |
| `E84_DCDC` | **E84** · **DCDC** |
| `E84_DTC_DCC` | **E84** · controle de traction dynamique · _DCC_ |
| `E84_ECBA_DCC` | **E84** · _ECBA_ · _DCC_ |
| `E84_EVB` | **E84** · **EVB** |
| `E84_FDB` | **E84** · **FDB** |
| `E84_FLR` | **E84** · reduction de puissance moteur |
| `E84_FZG_MASSE` | **E84** · vehicule · masse |
| `E84_FZG_VARIANTEN` | **E84** · vehicule · **VARIANTEN** |
| `E84_HBA` | **E84** · assistance de freinage |
| `E84_HHC` | **E84** · **HHC** |
| `E84_HILLDESC` | **E84** · assistance en descente |
| `E84_HILLDESC_DXC_8` | **E84** · assistance en descente · _DXC_ · 8 |
| `E84_HPS` | **E84** · **HPS** |
| `E84_HVV` | **E84** · **HVV** |
| `E84_IPM_MAX_P_RES` | **E84** · _IPM_ · maximum · P · remise a zero |
| `E84_MOTOR_LEISTUNG_MAX` | **E84** · moteur · puissance · maximum |
| `E84_P_AUSROLL_ANTRIEB` | **E84** · P · roue libre · transmission |
| `E84_R_RAD_DYN_DSC` | **E84** · _R_ · roue · dynamique · controle de stabilite |
| `E84_SCHUBABSCHALTUNG` | **E84** · coupure en deceleration |
| `E84_SETZ_V_MAX_CC` | **E84** · positionnement · _V_ · maximum · Check Control |
| `E84_SETZ_V_MIN` | **E84** · positionnement · _V_ · minimum |
| `E84_SST` | **E84** · _SST_ |
| `E84_VCH` | **E84** · **VCH** |
| `E84_VERZOEG_NEG_FEIN` | **E84** · temporisation · negatif · fin |
| `E84_VERZOEG_NEG_GROB` | **E84** · temporisation · negatif · grossier |
| `E84_VERZOEG_POS_FEIN` | **E84** · temporisation · position · fin |
| `E84_VERZOEG_POS_GROB` | **E84** · temporisation · position · grossier |
| `E84_V_ABSCHALT` | **E84** · _V_ · coupure |
| `E84_V_EINHEIT` | **E84** · _V_ · unite |
| `E89_ASL` | **E89** · _ASL_ |
| `E89_BERGAB_GERASTET` | **E89** · descente · crante |
| `E89_BERGAB_KENNEN` | **E89** · descente · reconnaissance |
| `E89_BSW` | **E89** · **BSW** |
| `E89_C8` | **E89** · **C8** |
| `E89_ECBA_DCC` | **E89** · _ECBA_ · _DCC_ |
| `E89_EVB` | **E89** · **EVB** |
| `E89_FLR` | **E89** · reduction de puissance moteur |
| `E89_HHC` | **E89** · **HHC** |
| `E89_HILLDESC_DXC_8` | **E89** · assistance en descente · _DXC_ · 8 |
| `E89_HPS` | **E89** · **HPS** |
| `E89_MOTOR_LEISTUNG_MAX` | **E89** · moteur · puissance · maximum |
| `E89_P_AUSROLL_ANTRIEB` | **E89** · P · roue libre · transmission |
| `E89_R_RAD_DYN_DSC` | **E89** · _R_ · roue · dynamique · controle de stabilite |
| `E89_SCHUBABSCHALTUNG` | **E89** · coupure en deceleration |
| `E89_SST` | **E89** · _SST_ |
| `E89_SZL` | **E89** · **SZL** |
| `E90_FZG_VARIANTEN` | **E90** · vehicule · **VARIANTEN** |
| `ECBA` | _ECBA_ |
| `ECBA_DCC` | _ECBA_ · _DCC_ |
| `ECBA_DCC_C0F` | _ECBA_ · _DCC_ · [index variante C0F] |
| `ECD` | **ECD** |
| `EVB` | **EVB** |
| `FBS` | compensation de fading |
| `FDB` | **FDB** |
| `FDB_0F` | **FDB** · **0F** |
| `FLR` | reduction de puissance moteur |
| `FLR_C0D` | reduction de puissance moteur · [index variante C0D] |
| `FLR_C0E` | reduction de puissance moteur · [index variante C0E] |
| `FLR_C0F` | reduction de puissance moteur · [index variante C0F] |
| `FZG_MASSE` | vehicule · masse |
| `FZG_MASSE_C03` | vehicule · masse · [index variante C03] |
| `FZG_VARIANTEN` | vehicule · **VARIANTEN** |
| `GEARMOMENTREGLER_MAX_HA_2` | **GEARMOMENTREGLER** · maximum · essieu arriere · 2 |
| `GEARMOMENTREGLER_MIN_HA_2` | **GEARMOMENTREGLER** · minimum · essieu arriere · 2 |
| `GETRIEBE` | boite de vitesses |
| `GMB` | _GMB_ |
| `GMK` | _GMK_ |
| `GMK_ECBA_HA` | _GMK_ · _ECBA_ · essieu arriere |
| `GMK_ECBA_VA` | _GMK_ · _ECBA_ · essieu avant |
| `HBA_DXC_8` | assistance de freinage · _DXC_ · 8 |
| `HHC` | **HHC** |
| `HILLDESC_DXC_8` | assistance en descente · _DXC_ · 8 |
| `HILLDESC_DXC_8_C0F` | assistance en descente · _DXC_ · 8 · [index variante C0F] |
| `HPS` | **HPS** |
| `HVV` | **HVV** |
| `IPM_MAX_P_RES` | _IPM_ · maximum · P · remise a zero |
| `LENKWINKELKENNLINIE_2` | courbe d'angle de braquage · 2 |
| `LF_ESM` | **LF** · _ESM_ |
| `LR_ESM` | **LR** · _ESM_ |
| `LUEFTER` | ventilateur |
| `LWK_X2` | angle de braquage · [point d'axe X2] |
| `LWK_X3` | angle de braquage · [point d'axe X3] |
| `LWK_X4` | angle de braquage · [point d'axe X4] |
| `LWK_Y1` | angle de braquage · [point d'axe Y1] |
| `LWK_Y2` | angle de braquage · [point d'axe Y2] |
| `LWK_Y3` | angle de braquage · [point d'axe Y3] |
| `LWK_Y4` | angle de braquage · [point d'axe Y4] |
| `MOTOR_ART_DSC_2` | moteur · type · controle de stabilite · 2 |
| `MOTOR_LEISTUNG_MAX` | moteur · puissance · maximum |
| `MOTOR_LEISTUNG_MAX_C0F` | moteur · puissance · maximum · [index variante C0F] |
| `M_ESM` | M · _ESM_ |
| `PRE_MSR` | pre- · regulation du couple de frein moteur |
| `P_AUSROLL_ANTRIEB` | P · roue libre · transmission |
| `P_AUSROLL_ANTRIEB_C0E` | P · roue libre · transmission · [index variante C0E] |
| `P_AUSROLL_ANTRIEB_C0F` | P · roue libre · transmission · [index variante C0F] |
| `RPA` | indicateur de crevaison |
| `RPA_SCHWELLE` | indicateur de crevaison · seuil |
| `R_RAD_DYN_DSC` | _R_ · roue · dynamique · controle de stabilite |
| `R_RAD_DYN_DSC_C0F` | _R_ · roue · dynamique · controle de stabilite · [index variante C0F] |
| `SCHUBABSCHALTUNG` | coupure en deceleration |
| `SCHUBABSCHALTUNG_C0E` | coupure en deceleration · [index variante C0E] |
| `SCHUBABSCHALTUNG_C0F` | coupure en deceleration · [index variante C0F] |
| `SC_X_ESM` | _SC_ · X · _ESM_ |
| `SC_Y_ESM` | _SC_ · Y · _ESM_ |
| `SETZ_V_MAX_CC` | positionnement · _V_ · maximum · Check Control |
| `SETZ_V_MIN` | positionnement · _V_ · minimum |
| `SPURWEITE` | voie |
| `SST` | _SST_ |
| `SZL` | **SZL** |
| `SZL_BDSC` | **SZL** · **BDSC** |
| `TB_BDD` | **TB** · **BDD** |
| `THETA_ESM` | _THETA_ · _ESM_ |
| `UEBERSTEUERN_MUE_0_2` | survirage · coefficient d'adherence · 0 · 2 |
| `UEBERSTEUERN_MUE_1_2` | survirage · coefficient d'adherence · 1 · 2 |
| `UNTERSTEUERN_MUE_0_2` | sous-virage · coefficient d'adherence · 0 · 2 |
| `UNTERSTEUERN_MUE_1_2` | sous-virage · coefficient d'adherence · 1 · 2 |
| `US_VARIANTE` | marche americain · variante |
| `VCH` | **VCH** |
| `VCH_C0E` | **VCH** · [index variante C0E] |
| `VCH_C0F` | **VCH** · [index variante C0F] |
| `VERZOEG_NEG_FEIN` | temporisation · negatif · fin |
| `VERZOEG_NEG_GROB` | temporisation · negatif · grossier |
| `VERZOEG_POS_FEIN` | temporisation · position · fin |
| `VERZOEG_POS_GROB` | temporisation · position · grossier |
| `VOLUMENDURCHFLUSSPARA_2` | **VOLUMENDURCHFLUSSPARA** · 2 |
| `V_ABSCHALT` | _V_ · coupure |
| `V_EINHEIT` | _V_ · unite |
| `V_EINHEIT_C0F` | _V_ · unite · [index variante C0F] |

---

Les fragments en **gras** n'ont pas de traduction etablie — voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Les fragments entre `_underscores_` sont ambigus : leur sens depend du module.
