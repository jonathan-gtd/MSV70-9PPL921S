# §19 — Résumé des Paramètres & Ordre d'application

> **Véhicule ciblé :** BMW E90/E91/E92/E93 330i — Moteur N52B30 — Calculateur Siemens MSV70  
> **Injecteurs :** BMW 13 53 7531634 (Bosch EV14, port injection, pression nominale rail 5.0 bar — `c_fup_nom` stock = 5000 hPa)  
> **Fichier de base :** VB67774_921S_Full.bin  
> **Stratégie de calibration :**
> - **Facteur injecteur → E85 (×1.45 via `ip_mff_cor_opm`)** : boucle ouverte (WOT) toujours riche, quelle que soit la saison. Note : `c_fac_mff_ti_stnd` reste au stock — l'enrichissement passe par les 4 maps `ip_mff_cor_opm_*` (raw 47 407, phys 1.473)
> - **Cranking → E70** : exception — trop riche au démarrage = noyage moteur
> - **Avance → E60** : pire octane légal français = zéro risque de cliquetis en toutes conditions
> - **Film mural → E85 (×1.25)** : tip-in riche = safe
> - **Carburant réel moyen attendu :** E70 (70% éthanol) — moyenne annuelle pondérée France  
> **Version :** 3.7 — 2026-04-11

---

## 📌 Résumé des Paramètres Impactés par la Conversion E85

Tous les paramètres du bin MSV70 concernés par la conversion — à modifier, surveiller, ou laisser en l'état.

| Statut | Paramètre(s) | Adresse XDF | Fichier | Action / Raison |
|---|---|---|---|---|
| **✅ MODIFIER** | `ip_mff_cor_opm_1_1` | 0x4E3D4 | 04_INJECTEURS | 32 770 → **47 407** (phys 1.016 → 1.473, ×1.45 E85). Flat, toutes cellules. |
| **✅ MODIFIER** | `ip_mff_cor_opm_1_2` | 0x4E554 | 04_INJECTEURS | Idem — banc 2, mode Valvetronic. |
| **✅ MODIFIER** | `ip_mff_cor_opm_2_1` | 0x4E6D4 | 04_INJECTEURS | Idem — banc 1, mode papillonné (fallback panne). |
| **✅ MODIFIER** | `ip_mff_cor_opm_2_2` | 0x4E7C4 | 04_INJECTEURS | Idem — banc 2, mode papillonné. |
| **✅ MODIFIER** | `c_tco_n_mff_cst` | 0x44F2F | 05_DEMARRAGE_FROID | 87 → **97** (17.25 °C → 25.00 °C). Seuil activation cranking. |
| **✅ MODIFIER** | `ip_mff_cst_opm_1` | 0x437DC | 05_DEMARRAGE_FROID | Table 3×8 : ×1.35–2.00 selon TCO. Cranking E70 — exception noyage. |
| **✅ MODIFIER** | `ip_mff_cst_opm_2` | 0x4380C | 05_DEMARRAGE_FROID | Idem — mode papillonné. Mêmes facteurs. |
| **✅ MODIFIER** | `ip_fac_lamb_wup` | 0x42764 | 05_DEMARRAGE_FROID | Table 6×6 (MAF × RPM) : 1.000 → **1.03–1.08** basses charges. Warm-up post-démarrage. |
| **✅ MODIFIER** | `ip_iga_bas_max_knk__n__maf` | 0x4323A | 06_AVANCE | Table 8×8 : +0 à +2.5° selon zone MAF/RPM. Plafond knock E60-safe. |
| **✅ MODIFIER** | `ip_ti_tco_pos_slow_wf_opm_1` | 0x4CBFC | 08_FILM_MURAL | Table 8×8 : stock ×**1.25**. Film lent tip-in — Valvetronic. |
| **✅ MODIFIER** | `ip_ti_tco_pos_slow_wf_opm_2` | 0x4CC7C | 08_FILM_MURAL | Table 8×8 : stock ×**1.25**. Film lent — mode papillonné. |
| **✅ MODIFIER** | `ip_ti_tco_pos_fast_wf_opm_1` | 0x443FC | 08_FILM_MURAL | Table 8×8 : stock ×**1.25**. Film rapide tip-in — Valvetronic. |
| **✅ MODIFIER** | `ip_ti_tco_pos_fast_wf_opm_2` | 0x4443C | 08_FILM_MURAL | Table 8×8 : stock ×**1.25**. Film rapide — mode papillonné. |
| **✅ MODIFIER** | `c_t_ti_dly_fl_1` | — | 11_DELAI_WOT | → **0 ms**. Enrichissement WOT sans délai. |
| **✅ MODIFIER** | `c_t_ti_dly_fl_2` | — | 11_DELAI_WOT | → **0 ms**. Idem. |
| ⬜ OPTIONNEL | `ip_lamb_fl__n` | 0x436A2 | 07_LAMBDA | Courbe 1×12 : stock λ 0.920 suffisant. Dé-enrichir à 0.940–0.950 pour +puissance. |
| ⬜ OPTIONNEL | `c_iga_ini` | — | 12_AVANCE_CRANKING | Stock +1° à +2° seulement si démarrage > 5 tours malgré démarrage froid calibré. |
| ⬜ OPTIONNEL | `ip_fac_lamb_wup_is` | 0x42788 | 05a_WARMUP_LAMBDA | Table 3×4 (MAF × RPM ralenti) : 1.000 → 1.02–1.05 si ralenti instable après cranking OK. |
| ⬜ OPTIONNEL | `KF_FTRANSVL` | 0x5C5EE | 08a_TRANSITOIRE | Table 8×8 : +10–20% zone 0.393–0.786 si trou exclusivement en kickdown brutal (après film mural validé). |
| ⬜ OPTIONNEL | `ip_fac_ti_maf_sp_wf_pos_opm_1` | 0x42C5A | 08b_FILM_MURAL_VLT | 1×8 f(TCO) : +15–25% à TCO < 15°C si couple instable en modulation douce (après film mural validé). |
| ⛔ NE PAS MODIFIER | `c_fac_mff_ti_stnd` (×5 copies) | — | 02_PRINCIPES | Overflow XDF : max encodable 0.393 ms/mg < cible 0.491. Utiliser `ip_mff_cor_opm_*` à la place. |
| ⛔ NE PAS MODIFIER | `ip_fac_eff_iga_ch_cold_opm_1/2` | 0x4A444 / 0x4A4A8 | 17a_CHAUFFE_CAT | Retard chauffe catalyseur. EGT E85 légèrement plus basse → DTC P0420 temporaires normaux. |
| ⛔ NE PAS MODIFIER | `c_teg_max_iga` | 0x44F54 | 17b_EGT | Seuil EGT à 865 °C. E85 produit moins d'EGT → protection rarement déclenchée, seuil OK. |
| ⛔ NE PAS MODIFIER | `id_maf_n_min_fcut_fast` | 0x41E1C | supprimé | Cutoff décélération indépendant du carburant. |
| ⛔ NE PAS MODIFIER | `ip_eoi_1_bas` | 0x4E914 | 04a_EOI | Phasage EOI : +45% TI E85 reste dans fenêtre acceptable sur injecteurs stock. |
| ⛔ NE PAS MODIFIER | `ip_fup_cor` | 0x4AF44 | 04b_PRESSION_RAIL | Correction pression rail : diagnostic uniquement. Problème → pompe (mécanique). |
| 👁 SURVEILLER | `c_fac_max/min_*_rng_lam_ad` | 0x47F4C–52 | 12_LTFT | Limites LTFT stock : **−8% / +12%**. Plafond −8% permanent → affiner `ip_mff_cor_opm_*`. |
| 👁 SURVEILLER | `c_fup_nom` | 0x44B0C | 04b_PRESSION_RAIL | Pression rail nominale 5000 hPa. Si DTC P0087 ou perte couple WOT : tester pompe (≥ 2 L/30 s). |
| 👁 SURVEILLER | `ip_ti_add_dly` (deadtime) | — | 10_DEADTIME_INJECTEUR | Temps mort injecteur f(tension). Inchangé si injecteurs stock. Recalculer si remplacement. |
| 👁 SURVEILLER | EVAP canister (`ip_crlc_mff_buf_cp`) | — | 14_EVAP | Vapeurs éthanol plus riches → STFT négatif pendant purge. Intervenir si STFT < −8% en continu (dépasse plafond LTFT). |

---

> **Limites de correction moteur (stock MSV70) :** le moteur supporte par défaut **+12 % / −8 %** de correction long terme (LTFT). L'asymétrie est intentionnelle — BMW tolère mieux un léger enrichissement qu'un appauvrissement. Si le LTFT plafonne à −8% en permanence après conversion E85, la calibration `ip_mff_cor_opm_*` est trop riche → l'affiner vers E70 (raw 44 581).

---

## Ordre d'application recommandé

| Étape | Fichier | Paramètre(s) clé(s) | Priorité |
|---|---|---|---|
| **1** | `04_INJECTEURS` | `ip_mff_cor_opm_*` (×4) | **CRITIQUE — base de tout** |
| **2** | `05_DEMARRAGE_FROID` | `ip_mff_cst_opm_1/2` + `ip_fac_lamb_wup` + `c_tco_n_mff_cst` | **CRITIQUE — démarrage froid** |
| **3** | `08_FILM_MURAL` | `ip_ti_tco_pos_slow/fast_wf_opm_1/2` (×4) | Important — tip-in sans trou |
| **4** | `06_AVANCE` | `ip_iga_bas_max_knk__n__maf` | Important — gain puissance |
| **5** | `11_DELAI_WOT` | `c_t_ti_dly_fl_1/2` | Important — WOT sans délai |
| ⬜ | `07_LAMBDA` | `ip_lamb_fl__n` | Optionnel — dé-enrichissement WOT |
| ⬜ | `05a_WARMUP_LAMBDA` | `ip_fac_lamb_wup_is` | Optionnel — si ralenti instable |
| ⬜ | `08a_TRANSITOIRE` | `KF_FTRANSVL` | Optionnel — si trou kickdown |
| ⬜ | `08b_FILM_MURAL_VLT` | `ip_fac_ti_maf_sp_wf_pos_opm_1` | Optionnel — si couple instable modulation douce |
| ⬜ | `12_AVANCE_CRANKING` | `c_iga_ini` | Optionnel — si démarrage > 5 tours |
| 👁 | `07a_LTFT` | `c_fac_max/min_*_rng_lam_ad` | Surveiller — limites LTFT −8% / +12% |
| 👁 | `04b_PRESSION_RAIL` | `c_fup_nom` / `ip_fup_cor` | Surveiller — pression rail WOT |
| 👁 | `09_EVAP` | `ip_crlc_mff_buf_cp` | Surveiller — purge canister |
| 👁 | `10_DEADTIME_INJECTEUR` | `ip_ti_add_dly` | Si remplacement injecteurs uniquement |
| ⛔ | `04a_EOI` | `ip_eoi_1_bas` | Ne pas modifier (injecteurs stock) |
| ⛔ | `supprimé` | `id_maf_n_min_fcut_fast` | Ne pas modifier |
| ⛔ | `17a_CHAUFFE_CAT` | `ip_fac_eff_iga_ch_cold_opm_*` | Ne pas modifier |
| ⛔ | `17b_EGT` | `c_teg_max_iga` | Ne pas modifier |
