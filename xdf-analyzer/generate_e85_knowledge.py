#!/usr/bin/env python3
"""
generate_e85_knowledge.py
Génère knowledge/e85_knowledge.json — documentation E85 pour MSV70/N52B30 (VB67774/9PPL921S).
Format : subcategory → { xdf_category, label, role, parameters: { nom: "explication" } }
"""
import json, os

KNOWLEDGE = {
  "id": "E85_MSV70_VB67774",
  "label": "E85 — MSV70 N52B30 (VB67774 / 9PPL921S)",
  "subcategories": [

    {
      "xdf_category": "GEN_Dataset_identification",
      "label": "Facteur carburant principal — ip_mff_cor_opm",
      "role": "Facteur multiplicatif sur la MFF f(RPM, charge). Stock VB67774 : 1.016 partout (neutre). E85 : ×1.473 sur toutes les cellules. ⚠️ DC injecteur ~90% à 6000 RPM WOT avec injecteurs stock 237 cc/min — saturation au-delà de ~4900 RPM. Injecteurs ≥380 cc/min requis pour WOT sans limite RPM.",
      "parameters": {
        "ip_mff_cor_opm_1_1": "Facteur MFF mode 1 banque 1 f(RPM, charge) [3D 12×16]. Stock raw=32768 → 1.016. E85 : raw=47516 → 1.473 sur toutes les 192 cellules. Vérifier TI en log avant WOT >4900 RPM.",
        "ip_mff_cor_opm_1_2": "Facteur MFF mode 1 banque 2. Même structure que _1_1. Mettre à jour identiquement.",
        "ip_mff_cor_opm_2_1": "Facteur MFF mode 2 banque 1 [3D 10×12]. S'active dans certaines conditions (chauffe cat, etc.). Mettre à jour avec le mode 1.",
        "ip_mff_cor_opm_2_2": "Facteur MFF mode 2 banque 2. Mettre à jour avec _2_1.",
        "id_idx_opm_mff_cor": "Indice de sélection du mode opération pour ip_mff_cor_opm. Signal interne."
      }
    },

    {
      "xdf_category": "INJ_General_Calculation_of_Fuel_Inj_Time",
      "label": "Masse cranking et seuil TCO — ip_mff_cst_opm / c_tco_n_mff_cst",
      "role": "ip_mff_cst_opm : masse injectée pendant la rotation démarreur (avant le 1er allumage) f(TCO, RPM). Pour E85 : ×1.35 à ×2.20 selon la température. c_tco_n_mff_cst : seuil TCO d'activation — stock 17.25°C, E80 → 26°C.",
      "parameters": {
        "ip_mff_cst_opm_1": "Masse cranking mode 1 f(TCO, RPM démarreur) [mg/stk]. E85 : ×2.20 à ≤−20°C, ×2.00 à −10°C, ×1.80 à 0°C, ×1.60 à 10°C, ×1.35 à ≥30°C.",
        "ip_mff_cst_opm_2": "Masse cranking mode 2. Mettre à jour identiquement à mode 1.",
        "c_tco_n_mff_cst": "Seuil TCO activation cranking enrichi. Équation 0.75×raw−48. Stock raw=87 → 17.25°C. E80 : raw=99 → 26°C. E100 : raw=107 → 32°C.",
        "ip_mff_deac_cst_opm_1": "Masse cranking désactivation cylindre mode 1. Non pertinent sur N52 atmosphérique.",
        "ip_mff_deac_cst_opm_2": "Masse cranking désactivation cylindre mode 2. Non pertinent sur N52 atmosphérique."
      }
    },

    {
      "xdf_category": "AUX_Application_incidences_for_Electrical_fuel_pump_Interface",
      "label": "Enrichissement after-start — ip_mff_lgrd_ast",
      "role": "Masse injectée pendant les ~2–5 premières secondes post-allumage (montée 0→800 RPM). L'éthanol s'allume difficilement froid — enrichissement progressif nécessaire.",
      "parameters": {
        "ip_mff_lgrd_ast": "Masse after-start f(TCO) [mg/stk]. E85 : ×1.65@−30°C, ×1.60@−20°C, ×1.55@−10°C, ×1.45@0°C, ×1.35@10°C, ×1.20@30°C, ×1.10@60°C, ×1.00@85°C."
      }
    },

    {
      "xdf_category": "AUX_Engine_speed_limitation_controller",
      "label": "Facteur et masse injection warm-up — ip_fac_ti_tco_wup / ip_ti_tco_wup / ip_ti_wup",
      "role": "Trois groupes complémentaires pour l'injection pendant la chauffe. ip_fac_ti_tco_wup : facteur multiplicateur sur TI f(TCO), stock ~1.000. ip_ti_tco_wup : masse absolue f(TCO, RPM). ip_ti_wup : TI absolu f(TCO). Tous à créer pour E85 (rien dans le bin stock sur essence).",
      "parameters": {
        "ip_fac_ti_tco_wup_opm_1": "Facteur TI warm-up f(TCO). Stock ~1.000. E85 : 1.40@−30°C, 1.35@−10°C, 1.25@0°C, 1.15@20°C, 1.08@40°C, 1.02@60°C, 1.00@80°C+.",
        "ip_fac_ti_tco_wup_opm_2": "Facteur TI warm-up mode 2. Mettre à jour identiquement à opm_1.",
        "ip_ti_tco_wup_opm_1": "Masse absolue warm-up f(TCO, RPM) [mg/stk]. E85 : ×1.15 à ×1.35 pour TCO < 50°C.",
        "ip_ti_tco_wup_opm_2": "Masse absolue warm-up mode 2. Même modification que opm_1.",
        "ip_ti_wup_opm_1": "TI warm-up absolu f(TCO) [ms]. E85 : ×1.20 à ×1.35 pour TCO < 50°C.",
        "ip_ti_wup_opm_2": "TI warm-up absolu mode 2. Même modification que opm_1.",
        "ip_fac_ti_vo_wup": "Facteur TI warm-up selon tension batterie. Paramètre constructeur, ne pas modifier."
      }
    },

    {
      "xdf_category": "INJ_Catalyst_enrichment_function",
      "label": "Lambda warm-up + consigne lambda WOT — ip_fac_lamb_wup / ip_lamb_bas",
      "role": "ip_fac_lamb_wup : enrichissement warm-up sur consigne lambda f(TCO, RPM). Stock VB67774 : 1.0001 partout (aucun enrichissement prévu sur essence). ip_lamb_bas_4 : consigne lambda WOT — stock 0.997λ (64 cellules confirmées). ip_lamb_bas_1/2/3 : ne pas modifier, la boucle fermée gère automatiquement.",
      "parameters": {
        "ip_fac_lamb_wup": "Facteur warm-up lambda f(TCO, RPM). Stock raw≈47 → 1.0001. E85 : 1.45@<−10°C, 1.35@0°C, 1.25@20°C, 1.15@40°C, 1.08@60°C, 1.02@70°C, 1.00@80°C+.",
        "ip_fac_lamb_wup_is": "Facteur warm-up lambda mode IS (bas régimes). Légèrement plus fort : +5%@704rpm/65mg, +3%@704rpm/100mg, +3%@1216rpm/65mg, +2%@1216rpm/100mg.",
        "ip_lamb_bas_4": "Consigne lambda WOT mode 4 f(charge, RPM). Stock raw=16351 → 0.997λ partout. E85 : réduire à 0.92–0.95 en zone >200 mg/stk et >3000 RPM. Valider sonde large bande.",
        "ip_lamb_bas_1": "Consigne lambda base mode 1. Stock 0.997λ. Ne pas modifier — la boucle fermée gère sur E85.",
        "ip_lamb_bas_2": "Consigne lambda base mode 2. Stock 0.997λ. Ne pas modifier.",
        "ip_lamb_bas_3": "Consigne lambda base mode 3. Stock 0.997λ. Ne pas modifier."
      }
    },

    {
      "xdf_category": "EGCP_ACQ_compensation_WRAF_ls",
      "label": "Limites correction STFT / FSD — ip_fac_lamb_max_fsd",
      "role": "Plafond des corrections STFT autorisées. Stock ~1.15 (±15%). Sur E85 break-in, le STFT doit pouvoir monter à +25–30% sans déclencher de DTC. Monter à 1.25–1.30 pour les 200 premiers km. ip_fac_lamb_min_fsd (plancher) : ne pas modifier.",
      "parameters": {
        "ip_fac_lamb_max_fsd_1": "Plafond STFT mode 1. Stock ~1.15. E85 break-in : 1.25–1.30. Resserrer à 1.20 après convergence LTFT.",
        "ip_fac_lamb_max_fsd_2": "Plafond STFT mode 2. Mettre à jour conjointement avec mode 1.",
        "ip_fac_lamb_min_fsd_1": "Plancher STFT mode 1. Stock ~0.84. Ne pas modifier.",
        "ip_fac_lamb_min_fsd_2": "Plancher STFT mode 2. Ne pas modifier.",
        "c_amp_min_fsd": "Amplitude minimale FSD. Ne pas modifier."
      }
    },

    {
      "xdf_category": "BGLLGEN",
      "label": "Limite LTFT — c_lamb_delta_i_max_lam_adj",
      "role": "Plafond d'accumulation de l'intégrateur LTFT. Stock VB67774 : 0.050λ (±5%). Sur E85 break-in : élargir à 0.25–0.30λ pour que l'apprentissage converge sans DTC 'adaptation at limit'. ⚠️ Après reflash ou déconnexion batterie, les LTFT repartent de 0 — 50–100 km de convergence normaux.",
      "parameters": {
        "c_lamb_delta_i_max_lam_adj": "Plafond intégrateur LTFT [λ]. Stock VB67774 : 0.050λ. E85 break-in : 0.25–0.30λ. Resserrer à 0.20λ après 500 km si LTFT stable.",
        "c_lamb_delta_i_min_lam_adj": "Plancher intégrateur LTFT. Ne pas modifier."
      }
    },

    {
      "xdf_category": "AUX_Electrical_fuel_pump_control",
      "label": "Film mural positif f(TCO, RPM) — ip_ti_tco_pos_fast/slow_wf_opm",
      "role": "Masses supplémentaires pour reconstituer le film mural lors de montées en charge. L'éthanol est plus visqueux — film ~25–30% plus épais à froid. Augmenter +15–25% entre 20°C et 70°C. Au-delà de 70°C, la différence E85/essence est quasi nulle. Les tables neg (décélérations) et tous les autres params de ce module ne sont pas à modifier.",
      "parameters": {
        "ip_ti_tco_pos_fast_wf_opm_1": "Film mural montée rapide mode 1 f(TCO, RPM) [3D 8×8]. E85 : ×1.25 uniforme sous 70°C. Première approximation — affiner si à-coups observés.",
        "ip_ti_tco_pos_fast_wf_opm_2": "Film mural montée rapide mode 2. Mettre à jour avec mode 1.",
        "ip_ti_tco_pos_slow_wf_opm_1": "Film mural montée lente mode 1 f(TCO, RPM). Même approche que fast : ×1.25 sous 70°C.",
        "ip_ti_tco_pos_slow_wf_opm_2": "Film mural montée lente mode 2. Mettre à jour avec mode 1.",
        "ip_ti_tco_neg_fast_wf_opm_1": "Film mural décélération rapide mode 1. Ne pas modifier.",
        "ip_ti_tco_neg_fast_wf_opm_2": "Film mural décélération rapide mode 2. Ne pas modifier.",
        "ip_ti_tco_neg_slow_wf_opm_1": "Film mural décélération lente mode 1. Ne pas modifier.",
        "ip_ti_tco_neg_slow_wf_opm_2": "Film mural décélération lente mode 2. Ne pas modifier."
      }
    },

    {
      "xdf_category": "INJR_Injection_phase_1st_pulse",
      "label": "Correction TI f(TCO) — ip_fac_ti_temp_cor",
      "role": "Facteur multiplicateur sur TI en fonction de la température moteur (12 points, −20°C à +100°C). Compense l'adhésion aux parois froides — l'éthanol est plus visqueux et colle davantage. Symptôme si insuffisant : à-coups sur accélérations légères moteur tiède.",
      "parameters": {
        "ip_fac_ti_temp_cor": "Facteur TI f(TCO). E85 : +20%@−10°C, +18%@0°C, +15%@20°C, +10%@40°C, +5%@60°C, 1.00@80°C+."
      }
    },

    {
      "xdf_category": "ATMPH",
      "label": "Modèle dynamique film mural — id_fac_mff_tco / id_mff_wf / ip_fac_mff_map_wf",
      "role": "Modèle dynamique film mural pour les transitions de charge. id_fac_mff_tco_pos/neg_wf : facteurs MFF accélérations/décélérations f(TCO). id_mff_inc/dec_wf : incrément/décrément lors des transitions. ip_fac_mff_map_wf : correction principale f(charge, TCO). Les K_ATMNN_*, K_ATM_*, etc. sont des paramètres internes du modèle thermique.",
      "parameters": {
        "id_fac_mff_tco_pos_wf": "Facteur MFF accélérations f(TCO). E85 : +20–25% entre 20°C et 70°C.",
        "id_fac_mff_tco_neg_wf": "Facteur MFF décélérations f(TCO). E85 : +15% sous 60°C (éthanol s'évapore plus lentement).",
        "id_mff_inc_wf": "Incrément MFF pour reconstituer le film en accélération [mg/stk]. E85 : +20% entre 20°C et 60°C.",
        "id_mff_dec_wf": "Décrément MFF récupéré du film en décélération [mg/stk]. E85 : +20% entre 20°C et 60°C.",
        "ip_fac_mff_map_wf": "Facteur film mural principal f(charge, TCO). E85 : +15% en zone TCO < 60°C."
      }
    },

    {
      "xdf_category": "EOS_Basic_operating_states__LV_ST__LV_IS__LV_REQ_ISC__LV_PL__LV_PU___LV_PUC",
      "label": "Délai enrichissement pleine charge — c_t_ti_dly_fl",
      "role": "Délai entre détection pleine charge et application de l'enrichissement WOT. Stock : 200 ms (raw=20). Sur E85, réduire à 0 ms pour que l'enrichissement soit immédiat. 4 copies à modifier selon la boîte (MT/AT).",
      "parameters": {
        "c_t_ti_dly_fl_1": "Délai WOT MT [s]. Équation 0.010×raw. Stock raw=20 → 200ms. E85 : raw=0 → 0ms.",
        "c_t_ti_dly_fl_2": "Délai WOT MT copie 2. Même modification que _1 : raw=0.",
        "c_t_ti_dly_fl_at_1": "Délai WOT AT. raw=0 si boîte automatique.",
        "c_t_ti_dly_fl_at_2": "Délai WOT AT copie 2. raw=0 si AT."
      }
    },

    {
      "xdf_category": "AGGR_VVTI_adaption",
      "label": "Lambda cible WOT f(RPM) — ip_lamb_fl__n",
      "role": "Lambda cible en mode WOT f(RPM). Stock VB67774 (lu directement dans le bin) : 0.920λ (608–4800 rpm), 0.901λ (5504 rpm), 0.871λ (6496 rpm). Ce bin a déjà un enrichissement WOT baked-in — aucune modification nécessaire. À vérifier à la sonde large bande.",
      "parameters": {
        "ip_lamb_fl__n": "Lambda cible WOT f(RPM). VB67774 stock : 0.920λ@608-4800rpm, 0.901λ@5504rpm, 0.871λ@6496rpm. Déjà enrichi — ne pas modifier sur ce bin."
      }
    },

    {
      "xdf_category": "IGA_Minimum_IGA_limitation_for_exhaust_gas_temperature_protection",
      "label": "Plafond anti-cliquetis — ip_iga_bas_max_knk__n__maf",
      "role": "Plafond de l'avance à l'allumage f(charge, RPM). ⚠️ PLAFOND ≠ AVANCE EFFECTIVE : sur MSV70, avance = min(modèle couple, plafond). Si le modèle couple demande déjà moins, lever le plafond n'a aucun effet. ip_iga_bas_knk (avance de base) est ABSENT du XDF 9PPL921S. Les autres params du module (id_fil_frq_knk_*, ip_gain_knk_*, etc.) sont des paramètres internes du contrôleur knock.",
      "parameters": {
        "ip_iga_bas_max_knk__n__maf": "Plafond avance anti-cliquetis f(charge, RPM) [°CRK, 3D 8×8]. Équation 0.375×raw−35.625. Optionnel — uniquement si cliquetis avéré par logging. Augmenter +2° à +4° en haute charge si le modèle couple était limité par ce plafond."
      }
    },

    {
      "xdf_category": "IGA_Dwell_time_control_open_loop",
      "label": "Avance de base f(charge, RPM) — ip_iga_st_bas_opm",
      "role": "Tables d'avance à l'allumage f(charge, RPM). Sur E85 (RON 105–108), +2° à +5° en pleine charge possible. Modifier uniquement après validation complète de l'injection (ip_mff_cor, warm-up, film mural).",
      "parameters": {
        "ip_iga_st_bas_opm_1": "Avance de base mode 1 f(charge, RPM) [°CRK]. Équation 0.75×raw−48. E85 : +2° à +6° en zone >200 mg/stk et >3000 RPM. Commencer par +2° uniforme, valider 50 km.",
        "ip_iga_st_bas_opm_2": "Avance de base mode 2. Modifier en même temps que opm_1."
      }
    },

    {
      "xdf_category": "IGA_Ignition_with_multiple_spark",
      "label": "Avance cranking — c_iga_ini",
      "role": "Avance à l'allumage lors des premiers cycles cranking. Stock raw=111 → 6.0°CRK. Optionnel — uniquement si démarrage difficile malgré ip_mff_cst_opm correctement calibré.",
      "parameters": {
        "c_iga_ini": "Avance allumage cranking [°CRK]. Équation 0.375×raw−35.625. Stock raw=111 → 6.0°CRK. E85 : +1° (raw=114→7°) à +2° (raw=116→8°). Ne pas dépasser +2°."
      }
    },

    {
      "xdf_category": "INJ_Coordination_of_the_Injection_Time_Correction_Factors_for_Cylinder_Balancing",
      "label": "c_fac_mff_ti_stnd_1 — Facteur MFF→TI copie principale (si changement injecteurs)",
      "role": "Conversion MFF→TI copie principale (cyl. 1–3). Équation 0.000006×raw. Stock injecteurs 13537531634 : raw=56567 → 0.3394 ms/mg → ~237 cc/min chaud. NE PAS MODIFIER si injecteurs stock conservés. 5 copies à mettre à jour simultanément si remplacement.",
      "parameters": {
        "c_fac_mff_ti_stnd_1": "Facteur MFF→TI cyl. 1–3. Adresse 0x044AC0. Équation 0.000006×raw. Stock raw=56567 → 0.3394 ms/mg. Modifier uniquement si injecteurs remplacés — recalculer depuis débit réel."
      }
    },

    {
      "xdf_category": "ATMNN",
      "label": "c_fac_mff_ti_stnd_2 — Facteur MFF→TI copie ATMNN (si changement injecteurs)",
      "role": "2e copie MFF→TI, module ATMNN (cyl. 4–6). Adresse 0x044AC2. Divergence avec _1 → injection asymétrique entre banques.",
      "parameters": {
        "c_fac_mff_ti_stnd_2": "Facteur MFF→TI cyl. 4–6. Adresse 0x044AC2. Équation 0.000006×raw. Stock raw=56567. Modifier avec les 5 autres copies si injecteurs remplacés.",
        "c_cam_ini_ex": "Angle initial came échappement VANOS. Paramètre constructeur.",
        "c_cam_ini_in": "Angle initial came admission VANOS. Paramètre constructeur."
      }
    },

    {
      "xdf_category": "INJR_Fuel_temperature_correction",
      "label": "c_fac_mff_ti_stnd[0]/[1] — Copies SOI/EOI (si changement injecteurs)",
      "role": "3e et 4e copies MFF→TI, module phasage SOI/EOI. Adresses 0x045AAC et 0x045AAE. Équation DIFFÉRENTE : 0.000012×raw — le raw est la moitié de _1/_2. Stock raw=28284 → 0.3394 ms/mg.",
      "parameters": {
        "c_fac_mff_ti_stnd[0]": "Copie SOI/EOI index 0. Adresse 0x045AAC. Équation 0.000012×raw. Stock raw=28284. Raw cible = moitié du raw des copies _1/_2.",
        "c_fac_mff_ti_stnd[1]": "Copie SOI/EOI index 1. Adresse 0x045AAE. Même équation et valeur que [0]."
      }
    },

    {
      "xdf_category": "DIA_Immobilizer_signal_diagnosis",
      "label": "c_fac_mff_ti_stnd_mon — 5e copie monitoring (si changement injecteurs)",
      "role": "5e copie MFF→TI, canal de monitoring indépendant MSV70. Adresse 0x04958C. Équation 0.000006×raw. Si cette copie diverge des copies principales → DTC cohérence injection immédiat.",
      "parameters": {
        "c_fac_mff_ti_stnd_mon": "5e copie MFF→TI canal monitoring. Adresse 0x04958C. Équation 0.000006×raw. Stock raw=56567. Oublier cette copie = DTC cohérence injection."
      }
    },

    {
      "xdf_category": "INJR_Battery_Voltage_Compensation",
      "label": "Dead time injecteur — ip_ti_min (si changement injecteurs)",
      "role": "Dead time électromécanique injecteur f(tension batterie). Stock injecteurs 13537531634 : calibré d'usine — NE PAS MODIFIER. Recalibrer obligatoirement si injecteurs remplacés.",
      "parameters": {
        "ip_ti_min": "Dead time injecteur f(Ubatt) [ms]. Stock 13537531634 : ~0.55ms@12V, ~1.00ms@8V. NE PAS MODIFIER avec injecteurs stock. Recalibrer si injecteurs remplacés — oublier = richesse incorrecte non corrigeable par les fuel trims."
      }
    }

  ]
}

OUTPUT_DIR  = os.path.join(os.path.dirname(os.path.abspath(__file__)), "knowledge")
OUTPUT_FILE = os.path.join(OUTPUT_DIR, "e85_knowledge.json")

os.makedirs(OUTPUT_DIR, exist_ok=True)
with open(OUTPUT_FILE, "w", encoding="utf-8") as f:
    json.dump(KNOWLEDGE, f, ensure_ascii=False, indent=2)

n_sub = len(KNOWLEDGE["subcategories"])
n_par = sum(len(s["parameters"]) for s in KNOWLEDGE["subcategories"])
print(f"Generated {OUTPUT_FILE}  ({n_sub} subcategories, {n_par} params)")
