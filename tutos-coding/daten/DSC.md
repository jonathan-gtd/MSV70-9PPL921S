# DSC — Controle de stabilite / ABS

🔴 Module `DSC` — voir [04 — Modules](../04_MODULES.md) pour son rôle.

> 🔴 **Module a ne pas coder.** Liste fournie pour comprendre, pas pour modifier.

---

## Identification

| | |
|---|---|
| Variante | **`MK60_87.C0A`** |
| Identifiée par | recoupement avec `DSC.TRC` lu sur le véhicule |
| Fonctions retenues | **86** |
| Écartées (hors blocs du TRC) | 0 |
| Avec valeurs | 86 |
| Avec explication rédigée | 8 |
| Blocs d'adresses du TRC | 4 |

> Seules les fonctions dont l'adresse tombe dans un bloc réellement renvoyé par
> le module sont listées. Ce sont celles que **ta** voiture expose.

---

## Les 86 fonctions codables

| FSW | Traduction | Valeurs | Explication |
|---|---|---|---|
| `C09_C8` | [variante C09] · **C8** | `wert_01` · `wert_02` |  |
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
| `C0B_BB_RAB` | [variante C0B] · **BB** · precharge des plaquettes | `wert_00` · `wert_01` | Precharge des plaquettes au lever brusque de la pedale d'accelerateur. |
| `C0B_FBS` | [variante C0B] · compensation de fading | `wert_00` · `wert_01` | Compensation de l'effet de fading des freins. |
| `C0B_SST` | [variante C0B] · _SST_ | `wert_00` · `wert_01` |  |
| `C0B_TB_BDD` | [variante C0B] · **TB** · **BDD** | `wert_00` · `wert_01` | Sechage periodique des disques de frein par temps humide. |
| `CF_ESM_HB` | **CF** · _ESM_ · **HB** | `wert_01` |  |
| `CF_ESM_LB` | **CF** · _ESM_ · **LB** | `wert_01` |  |
| `CHECKSUM` | somme de controle | `grundcode_var` |  |
| `CR_ESM_HB` | **CR** · _ESM_ · **HB** | `wert_01` |  |
| `CR_ESM_LB` | **CR** · _ESM_ · **LB** | `wert_01` |  |
| `LF_ESM` | **LF** · _ESM_ | `wert_01` |  |
| `LR_ESM` | **LR** · _ESM_ | `wert_01` |  |
| `LWK_X2` | angle de braquage · [axe X2] | `wert_01` |  |
| `LWK_X3` | angle de braquage · [axe X3] | `wert_01` |  |
| `LWK_X4` | angle de braquage · [axe X4] | `wert_01` |  |
| `LWK_Y1` | angle de braquage · [axe Y1] | `wert_01` |  |
| `LWK_Y2` | angle de braquage · [axe Y2] | `wert_01` |  |
| `LWK_Y3` | angle de braquage · [axe Y3] | `wert_01` |  |
| `LWK_Y4` | angle de braquage · [axe Y4] | `wert_01` |  |
| `M_ESM` | M · _ESM_ | `wert_01` |  |
| `SC_X_ESM` | _SC_ · X · _ESM_ | `wert_01` |  |
| `SC_Y_ESM` | _SC_ · Y · _ESM_ | `wert_01` |  |
| `SPURWEITE` | voie | `wert_01` |  |
| `SZL` | **SZL** | `wert_01` · `wert_02` · `wert_04` · `wert_05` · `wert_06` · `wert_07` |  |
| `THETA_ESM` | _THETA_ · _ESM_ | `wert_01` |  |

---

**Traduction** — décomposition du nom allemand, voir [LEXIQUE_FRAGMENTS.md](LEXIQUE_FRAGMENTS.md).
Fragments en **gras** : sans traduction établie. Entre `_underscores_` : ambigus.

**Valeurs** — ce que le module accepte, extrait des DATEN.

**Explication** — rédigée seulement quand l'effet est établi de source sûre.
