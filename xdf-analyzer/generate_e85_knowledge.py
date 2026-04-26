#!/usr/bin/env python3
"""
generate_e85_knowledge.py
Génère knowledge/e85_knowledge.json — documentation E85 pour MSV70/N52B30 (VB67774/9PPL921S).
Couvre uniquement les paramètres référencés dans tutos-reprog/README.md.
"""
import json
import os

KNOWLEDGE = {
  "id": "E85_MSV70_VB67774",
  "label": "E85 — MSV70 N52B30 — Paramètres documentés (VB67774 / 9PPL921S)",
  "description": (
    "Base de connaissances E85 pour Siemens MSV70, N52B30 3L, dump VB67774, SW 9PPL921S. "
    "Couvre tous les paramètres référencés dans tutos-reprog/README.md. "
    "Pour chaque catégorie XDF documentée ici, les paramètres non listés reçoivent "
    "modification_e85=NON via undocumented_params_default."
  ),
  "subcategories": [

    # ─────────────────────────────────────────────────────────────────────
    # 1 — Facteur carburant principal (ip_mff_cor_opm)  🔴 CRITIQUE
    # ─────────────────────────────────────────────────────────────────────
    {
      "xdf_category": "GEN_Dataset_identification",
      "label": "Facteur carburant principal — ip_mff_cor_opm (CRITIQUE)",
      "role": (
        "Facteur correctif multiplicatif sur la MFF (mg/stk). Axe X : 704–6048 rpm (16 pts). "
        "Axe Y : 19–650 mg/stk (12 pts). Stock VB67774 : 32768 raw → 1.016 partout (neutre). "
        "FinalV1 : 47516 → 1.473 flat sur les 192 cellules. Équation Z : 0.000031×raw. "
        "⚠️ DC injecteur : TI(E85) = TI(E0)×1.473. Injecteurs stock ~237 cc/min chaud "
        "(c_fac_mff_ti_stnd=0.3394 ms/mg). DC safe ≤85%. Saturation >~4900 RPM WOT. "
        "Pour WOT sans limite RPM : injecteurs ≥380 cc/min requis."
      ),
      "tuning_impact": "critical",
      "flex_fuel_sensitive": True,
      "tags": ["e85", "safety", "fueling"],
      "undocumented_params_default": "NON",
      "parameters": {
        "ip_mff_cor_opm_1_1": {
          "doc": "Facteur correctif MFF mode 1 banque 1 f(RPM, charge). 3D Map 12×16. Équation Z : 0.000031×raw. Stock VB67774 : raw=32768 → 1.016 (neutre). FinalV1 : raw=47516 → 1.473 flat.",
          "type_doc": "CARTOGRAPHIE 3D",
          "modification_e85": "OUI",
          "e85_action": "×1.473 uniformément sur les 192 cellules (raw=47516). Graduation possible : ×1.35 sous 100 mg/stk, ×1.45 charge partielle, ×1.473 pleine charge. Valider sonde large bande avant WOT.",
          "stock_raw": 32768,
          "stock_physical": "1.016 (neutre)",
          "e85_factor": "×1.473 (E83)",
          "warning": True,
          "warning_detail": "DC injecteur ~90% à 6000 RPM WOT sur E85 avec stock 237cc/min. Saturation >4900 RPM. Vérifier TI en log avant WOT.",
          "dependencies": ["ip_mff_cor_opm_1_2", "ip_mff_cor_opm_2_1", "ip_mff_cor_opm_2_2"]
        },
        "ip_mff_cor_opm_1_2": {
          "doc": "Facteur correctif MFF mode 1 banque 2. Même structure que _1_1. Mettre à jour identiquement.",
          "type_doc": "CARTOGRAPHIE 3D",
          "modification_e85": "OUI",
          "e85_action": "Même modification que ip_mff_cor_opm_1_1.",
          "stock_raw": 32768,
          "stock_physical": "1.016 (neutre)",
          "e85_factor": "×1.473",
          "warning": False,
          "dependencies": ["ip_mff_cor_opm_1_1", "ip_mff_cor_opm_2_1", "ip_mff_cor_opm_2_2"]
        },
        "ip_mff_cor_opm_2_1": {
          "doc": "Facteur correctif MFF mode 2 banque 1. 3D Map 10×12. S'active dans des conditions spécifiques (chauffe cat, etc.).",
          "type_doc": "CARTOGRAPHIE 3D",
          "modification_e85": "OUI",
          "e85_action": "×1.473 comme le mode 1. Sans mise à jour, injection momentanément incorrecte lors du passage en mode 2.",
          "stock_raw": 32768,
          "stock_physical": "1.016 (neutre)",
          "e85_factor": "×1.473",
          "warning": False,
          "dependencies": ["ip_mff_cor_opm_1_1", "ip_mff_cor_opm_1_2", "ip_mff_cor_opm_2_2"]
        },
        "ip_mff_cor_opm_2_2": {
          "doc": "Facteur correctif MFF mode 2 banque 2. Mettre à jour avec _2_1.",
          "type_doc": "CARTOGRAPHIE 3D",
          "modification_e85": "OUI",
          "e85_action": "Même modification que ip_mff_cor_opm_2_1.",
          "stock_raw": 32768,
          "stock_physical": "1.016 (neutre)",
          "e85_factor": "×1.473",
          "warning": False,
          "dependencies": ["ip_mff_cor_opm_1_1", "ip_mff_cor_opm_1_2", "ip_mff_cor_opm_2_1"]
        },
        "id_idx_opm_mff_cor": {
          "doc": "Indice de sélection du mode opération pour ip_mff_cor_opm. Signal interne.",
          "type_doc": "COURBE 2D",
          "modification_e85": "NON"
        }
      }
    },

    # ─────────────────────────────────────────────────────────────────────
    # 2 — Cranking froid  🔴 CRITIQUE
    # ─────────────────────────────────────────────────────────────────────
    {
      "xdf_category": "INJ_General_Calculation_of_Fuel_Inj_Time",
      "label": "Masse cranking et seuil TCO — ip_mff_cst_opm / c_tco_n_mff_cst (CRITIQUE)",
      "role": (
        "ip_mff_cst_opm_1/2 : tables 3D f(TCO, RPM démarreur). Actives UNIQUEMENT pendant "
        "rotation démarreur. Pour E85 : ×1.35 (>30°C) à ×2.20 (<-20°C). "
        "c_tco_n_mff_cst : seuil TCO pour activation. Stock raw=87 → 17.25°C. "
        "E80 → raw=99 → 26°C."
      ),
      "tuning_impact": "critical",
      "flex_fuel_sensitive": True,
      "tags": ["e85", "cold_start"],
      "undocumented_params_default": "NON",
      "parameters": {
        "ip_mff_cst_opm_1": {
          "doc": "Masse carburant cranking mode 1 f(TCO, RPM démarreur) [mg/stk]. Axe X : −30°C à +90°C. Stock max froid : ~448 mg/stk.",
          "type_doc": "TABLE 3D",
          "modification_e85": "OUI",
          "e85_action": "Multiplier par facteur TCO gradué : ×2.20 à ≤−20°C, ×2.00 à −10°C, ×1.80 à 0°C, ×1.60 à 10°C, ×1.35 à ≥30°C.",
          "e85_factor": "×1.35 à ×2.20",
          "warning": False,
          "dependencies": ["ip_mff_cst_opm_2"]
        },
        "ip_mff_cst_opm_2": {
          "doc": "Masse cranking mode 2 f(TCO, RPM démarreur) [mg/stk]. Mettre à jour identiquement à mode 1.",
          "type_doc": "TABLE 3D",
          "modification_e85": "OUI",
          "e85_action": "Même modification que ip_mff_cst_opm_1.",
          "e85_factor": "×1.35 à ×2.20",
          "warning": False,
          "dependencies": ["ip_mff_cst_opm_1"]
        },
        "c_tco_n_mff_cst": {
          "doc": "Seuil TCO activation cranking enrichi [°C]. Équation : 0.75×raw−48. Stock raw=87 → 17.25°C.",
          "type_doc": "CONSTANTE",
          "modification_e85": "OUI",
          "e85_action": "Augmenter : E60 raw=91 → 20°C, E80 raw=99 → 26°C, E100 raw=107 → 32°C.",
          "stock_raw": 87,
          "stock_physical": "17.25 °C",
          "e60_raw": 91,
          "e80_raw": 99,
          "e100_raw": 107,
          "warning": False
        },
        "ip_mff_deac_cst_opm_1": {
          "doc": "Masse cranking désactivation cylindre mode 1. Non pertinent sur N52 atmosphérique.",
          "type_doc": "TABLE 3D",
          "modification_e85": "NON"
        },
        "ip_mff_deac_cst_opm_2": {
          "doc": "Masse cranking désactivation cylindre mode 2. Non pertinent sur N52 atmosphérique.",
          "type_doc": "TABLE 3D",
          "modification_e85": "NON"
        }
      }
    },

    # ─────────────────────────────────────────────────────────────────────
    # 3 — After-start  🔴 CRITIQUE
    # ─────────────────────────────────────────────────────────────────────
    {
      "xdf_category": "AUX_Application_incidences_for_Electrical_fuel_pump_Interface",
      "label": "Enrichissement after-start — ip_mff_lgrd_ast (CRITIQUE)",
      "role": (
        "ip_mff_lgrd_ast : masse after-start f(TCO) pendant les ~2–5 premières secondes "
        "post-allumage (montée 0→800 RPM). K_ATM_* et K_EWP_* dans ce module = modèle "
        "thermique ATMNN + pompe EFP — non liés à l'éthanol."
      ),
      "tuning_impact": "high",
      "flex_fuel_sensitive": True,
      "tags": ["e85", "cold_start"],
      "undocumented_params_default": "NON",
      "parameters": {
        "ip_mff_lgrd_ast": {
          "doc": "Masse carburant after-start f(TCO) [mg/stk]. Active ~2–5s post-allumage, montée 0→800 RPM.",
          "type_doc": "COURBE 2D",
          "modification_e85": "OUI",
          "e85_action": "Facteur progressif selon TCO : ×1.65@−30°C, ×1.60@−20°C, ×1.55@−10°C, ×1.45@0°C, ×1.35@10°C, ×1.20@30°C, ×1.10@60°C, ×1.00@85°C.",
          "e85_factor": "×1.00 à ×1.65",
          "warning": False
        }
      }
    },

    # ─────────────────────────────────────────────────────────────────────
    # 4 — Warm-up TI  🔴 CRITIQUE
    # ─────────────────────────────────────────────────────────────────────
    {
      "xdf_category": "AUX_Engine_speed_limitation_controller",
      "label": "Facteur et masse injection warm-up — ip_fac_ti_tco_wup / ip_ti_tco_wup / ip_ti_wup (CRITIQUE)",
      "role": (
        "ip_fac_ti_tco_wup_opm_1/2 : facteur multiplicateur sur TI f(TCO). Stock VB67774 : ~1.000. "
        "ip_ti_tco_wup_opm_1/2 : masse absolue warm-up f(TCO, RPM). "
        "ip_ti_wup_opm_1/2 : TI warm-up absolu f(TCO). "
        "ip_vff_* et c_vb_efp_* dans ce module = pompe EFP — non liés à l'éthanol."
      ),
      "tuning_impact": "critical",
      "flex_fuel_sensitive": True,
      "tags": ["e85", "warm_up"],
      "undocumented_params_default": "NON",
      "parameters": {
        "ip_fac_ti_tco_wup_opm_1": {
          "doc": "Facteur multiplicateur TI warm-up f(TCO) [sans unité]. Stock VB67774 : ~1.000 partout.",
          "type_doc": "COURBE 2D",
          "modification_e85": "OUI",
          "e85_action": "Créer profil : 1.40@−30°C, 1.35@−10°C, 1.25@0°C, 1.15@20°C, 1.08@40°C, 1.02@60°C, 1.00@80°C+.",
          "e85_factor": "×1.00 à ×1.40",
          "warning": False,
          "dependencies": ["ip_fac_ti_tco_wup_opm_2"]
        },
        "ip_fac_ti_tco_wup_opm_2": {
          "doc": "Facteur TI warm-up mode 2 f(TCO). Mettre à jour identiquement à opm_1.",
          "type_doc": "COURBE 2D",
          "modification_e85": "OUI",
          "e85_action": "Même profil que ip_fac_ti_tco_wup_opm_1.",
          "e85_factor": "×1.00 à ×1.40",
          "warning": False,
          "dependencies": ["ip_fac_ti_tco_wup_opm_1"]
        },
        "ip_ti_tco_wup_opm_1": {
          "doc": "Masse absolue warm-up mode 1 f(TCO, RPM) [mg/stk]. Complémente ip_fac_ti_tco_wup_opm_1.",
          "type_doc": "TABLE 3D",
          "modification_e85": "OUI",
          "e85_action": "Multiplier zones TCO < 50°C : ×1.15 à ×1.35.",
          "e85_factor": "×1.15 à ×1.35",
          "warning": False,
          "dependencies": ["ip_ti_tco_wup_opm_2"]
        },
        "ip_ti_tco_wup_opm_2": {
          "doc": "Masse absolue warm-up mode 2 f(TCO, RPM) [mg/stk]. Mettre à jour avec opm_1.",
          "type_doc": "TABLE 3D",
          "modification_e85": "OUI",
          "e85_action": "Même modification que ip_ti_tco_wup_opm_1.",
          "e85_factor": "×1.15 à ×1.35",
          "warning": False,
          "dependencies": ["ip_ti_tco_wup_opm_1"]
        },
        "ip_ti_wup_opm_1": {
          "doc": "TI warm-up absolu mode 1 f(TCO) [ms]. Complément direct du facteur.",
          "type_doc": "COURBE 2D",
          "modification_e85": "OUI",
          "e85_action": "×1.20 à ×1.35 pour TCO < 50°C.",
          "e85_factor": "×1.20 à ×1.35",
          "warning": False,
          "dependencies": ["ip_ti_wup_opm_2"]
        },
        "ip_ti_wup_opm_2": {
          "doc": "TI warm-up absolu mode 2 f(TCO) [ms]. Mettre à jour avec opm_1.",
          "type_doc": "COURBE 2D",
          "modification_e85": "OUI",
          "e85_action": "Même modification que ip_ti_wup_opm_1.",
          "e85_factor": "×1.20 à ×1.35",
          "warning": False,
          "dependencies": ["ip_ti_wup_opm_1"]
        },
        "ip_fac_ti_vo_wup": {
          "doc": "Facteur TI warm-up selon tension batterie. Paramètre constructeur.",
          "type_doc": "COURBE 2D",
          "modification_e85": "NON"
        }
      }
    },

    # ─────────────────────────────────────────────────────────────────────
    # 5 — Lambda warm-up + ip_lamb_bas_4  🔴/🟠
    # ─────────────────────────────────────────────────────────────────────
    {
      "xdf_category": "INJ_Catalyst_enrichment_function",
      "label": "Lambda warm-up + consigne lambda WOT — ip_fac_lamb_wup / ip_lamb_bas (CRITIQUE/RECOMMANDÉ)",
      "role": (
        "ip_fac_lamb_wup : facteur d'enrichissement warm-up sur consigne lambda f(TCO, RPM). "
        "Stock VB67774 : raw≈47 → 1.0001 partout. "
        "ip_fac_lamb_wup_is : même logique en mode IS (idle/start). "
        "ip_lamb_bas_4 : consigne lambda WOT f(charge, RPM). "
        "Stock VB67774 : 16351 raw → 0.997λ (ALL 64 cellules confirmées par lecture bin). "
        "ip_lamb_bas_1/2/3 : consignes lambda base — NE PAS MODIFIER. "
        "La boucle fermée gère automatiquement sur E85 si c_fac_mff_ti_stnd est correct."
      ),
      "tuning_impact": "critical",
      "flex_fuel_sensitive": True,
      "tags": ["e85", "warm_up"],
      "undocumented_params_default": "NON",
      "parameters": {
        "ip_fac_lamb_wup": {
          "doc": "Facteur warm-up lambda f(TCO, RPM) [sans unité]. 3D map. Stock VB67774 : raw≈47 → 1.0001.",
          "type_doc": "CARTOGRAPHIE 3D",
          "modification_e85": "OUI",
          "e85_action": "Créer profil : 1.45@TCO<−10°C, 1.35@0°C, 1.25@20°C, 1.15@40°C, 1.08@60°C, 1.02@70°C, 1.00@80°C+. Appliquer sur tous les RPM.",
          "stock_raw": 47,
          "stock_physical": "1.0001",
          "e85_factor": "×1.02 à ×1.45",
          "warning": False,
          "dependencies": ["ip_fac_lamb_wup_is"]
        },
        "ip_fac_lamb_wup_is": {
          "doc": "Facteur warm-up lambda mode IS f(TCO, RPM). Complément pour bas régimes.",
          "type_doc": "CARTOGRAPHIE 3D",
          "modification_e85": "OUI",
          "e85_action": "+5%@704rpm/65mg, +3%@704rpm/100mg, +3%@1216rpm/65mg, +2%@1216rpm/100mg.",
          "e85_factor": "×1.02 à ×1.45",
          "warning": False,
          "dependencies": ["ip_fac_lamb_wup"]
        },
        "ip_lamb_bas_4": {
          "doc": "Consigne lambda WOT mode 4 f(charge, RPM) [λ]. 3D map. Stock VB67774 : raw=16351 → 0.997λ partout (64 cellules, confirmé lecture bin).",
          "type_doc": "CARTOGRAPHIE 3D",
          "modification_e85": "OUI",
          "e85_action": "Réduire à 0.92–0.95 en zone >200 mg/stk et >3000 RPM. Laisser 0.997 en charge partielle. Valider sonde large bande avant WOT.",
          "stock_raw": 16351,
          "stock_physical": "0.997 λ",
          "warning": False
        },
        "ip_lamb_bas_1": {
          "doc": "Consigne lambda base mode 1 f(charge, RPM) [λ]. Stock : 0.997λ. NE PAS MODIFIER.",
          "type_doc": "CARTOGRAPHIE 3D",
          "modification_e85": "NON",
          "stock_raw": 47,
          "stock_physical": "0.997 λ"
        },
        "ip_lamb_bas_2": {
          "doc": "Consigne lambda base mode 2 f(charge, RPM) [λ]. Stock : 0.997λ. NE PAS MODIFIER.",
          "type_doc": "CARTOGRAPHIE 3D",
          "modification_e85": "NON",
          "stock_raw": 47,
          "stock_physical": "0.997 λ"
        },
        "ip_lamb_bas_3": {
          "doc": "Consigne lambda base mode 3 f(charge, RPM) [λ]. Stock : 0.997λ.",
          "type_doc": "CARTOGRAPHIE 3D",
          "modification_e85": "NON",
          "stock_raw": 47,
          "stock_physical": "0.997 λ"
        }
      }
    },

    # ─────────────────────────────────────────────────────────────────────
    # 6 — FSD limits  🔴 CRITIQUE
    # ─────────────────────────────────────────────────────────────────────
    {
      "xdf_category": "EGCP_ACQ_compensation_WRAF_ls",
      "label": "Limites correction STFT / FSD — ip_fac_lamb_max_fsd (CRITIQUE)",
      "role": (
        "Limites haute et basse des corrections STFT autorisées (FSD = Full Scale Deflection). "
        "Stock ip_fac_lamb_max_fsd : ~1.15 (±15%). Sur E85 break-in (0–200 km), le STFT doit "
        "pouvoir atteindre +25–30% sans DTC. Monter à 1.25–1.30. "
        "ip_fac_lamb_min_fsd (plancher) : NE PAS MODIFIER."
      ),
      "tuning_impact": "high",
      "flex_fuel_sensitive": True,
      "tags": ["e85", "dtc", "lambda"],
      "undocumented_params_default": "NON",
      "parameters": {
        "ip_fac_lamb_max_fsd_1": {
          "doc": "Limite haute STFT mode 1 [λ]. Plafond correction instantanée. Stock ~1.15. Si dépassé → DTC adaptation.",
          "type_doc": "TABLE",
          "modification_e85": "OUI",
          "e85_action": "1.25–1.30 pour les 0–200 premiers km. Resserrer à 1.20 après convergence LTFT stable.",
          "stock_physical": "~1.15 λ",
          "warning": False,
          "dependencies": ["ip_fac_lamb_max_fsd_2", "c_lamb_delta_i_max_lam_adj"]
        },
        "ip_fac_lamb_max_fsd_2": {
          "doc": "Limite haute STFT mode 2 [λ]. Même logique que mode 1. Mettre à jour conjointement.",
          "type_doc": "TABLE",
          "modification_e85": "OUI",
          "e85_action": "Même modification que ip_fac_lamb_max_fsd_1.",
          "stock_physical": "~1.15 λ",
          "warning": False,
          "dependencies": ["ip_fac_lamb_max_fsd_1", "c_lamb_delta_i_max_lam_adj"]
        },
        "ip_fac_lamb_min_fsd_1": {
          "doc": "Limite basse STFT mode 1 [λ]. Stock ~0.84. NE PAS MODIFIER.",
          "type_doc": "TABLE",
          "modification_e85": "NON",
          "stock_physical": "~0.84 λ"
        },
        "ip_fac_lamb_min_fsd_2": {
          "doc": "Limite basse STFT mode 2 [λ]. NE PAS MODIFIER.",
          "type_doc": "TABLE",
          "modification_e85": "NON",
          "stock_physical": "~0.84 λ"
        },
        "c_amp_min_fsd": {
          "doc": "Amplitude minimale FSD [λ]. Largeur minimale fenêtre correction.",
          "type_doc": "CONSTANTE",
          "modification_e85": "NON"
        }
      }
    },

    # ─────────────────────────────────────────────────────────────────────
    # 7 — Limite LTFT  🔴 CRITIQUE
    # ─────────────────────────────────────────────────────────────────────
    {
      "xdf_category": "BGLLGEN",
      "label": "Limite LTFT intégral — c_lamb_delta_i_max_lam_adj (CRITIQUE)",
      "role": (
        "c_lamb_delta_i_max_lam_adj : plafond d'accumulation de l'intégrateur LTFT [λ]. "
        "Stock VB67774 : 0.050λ (±5%). Sur E85 break-in : élargir à 0.25–0.30λ. "
        "⚠️ Reset LTFT : après reflash ou déconnexion batterie, les LTFT repartent de 0. "
        "Surplus ~35% via ip_mff_cor pendant 50–100 km — normal. "
        "Ne pas juger la calibration avant convergence complète."
      ),
      "tuning_impact": "high",
      "flex_fuel_sensitive": True,
      "tags": ["e85", "ltft", "dtc"],
      "undocumented_params_default": "NON",
      "parameters": {
        "c_lamb_delta_i_max_lam_adj": {
          "doc": "Plafond intégrateur LTFT [λ]. Stock VB67774 : 0.050λ (±5%). Si atteint → DTC 'adaptation at limit'.",
          "type_doc": "CONSTANTE",
          "modification_e85": "OUI",
          "e85_action": "0.25–0.30λ pour 0–200 premiers km. Resserrer à 0.20λ après 500 km si LTFT stable.",
          "stock_physical": "0.050 λ (±5%)",
          "warning": False,
          "dependencies": ["ip_fac_lamb_max_fsd_1", "ip_fac_lamb_max_fsd_2"]
        },
        "c_lamb_delta_i_min_lam_adj": {
          "doc": "Plancher intégrateur LTFT [λ]. Protège contre correction négative. NE PAS MODIFIER.",
          "type_doc": "CONSTANTE",
          "modification_e85": "NON"
        }
      }
    },

    # ─────────────────────────────────────────────────────────────────────
    # 8 — Wall film TCO pos (ip_ti_tco_pos_fast/slow_wf_opm)  🟠 RECOMMANDÉ
    # ─────────────────────────────────────────────────────────────────────
    {
      "xdf_category": "AUX_Electrical_fuel_pump_control",
      "label": "Film mural positif f(TCO, RPM) — ip_ti_tco_pos_fast/slow_wf_opm (RECOMMANDÉ)",
      "role": (
        "Masses d'injection supplémentaires pour reconstituer le film mural lors de montées "
        "en charge f(TCO, RPM). fast = transitions rapides, slow = progressives. "
        "Sur E85 : l'éthanol est plus visqueux — film ~25–30% plus épais à froid. "
        "Augmenter +15–25% entre 20°C et 70°C. Au-dessus 70°C : différence quasi nulle. "
        "ip_ti_tco_neg_* (décélérations) : NE PAS MODIFIER. "
        "Tous les autres params de ce module (ip_crlc_*, ip_fac_ti_*, c_fac_wf_*, "
        "c_n_*, c_t_*, c_ti_crlc_*, etc.) : paramètres internes film mural — non E85."
      ),
      "tuning_impact": "high",
      "flex_fuel_sensitive": True,
      "tags": ["e85", "warm_up", "wall_film"],
      "undocumented_params_default": "NON",
      "parameters": {
        "ip_ti_tco_pos_fast_wf_opm_1": {
          "doc": "Masse film mural montée rapide mode 1 f(TCO, RPM). 3D Map 8×8.",
          "type_doc": "CARTOGRAPHIE 3D",
          "modification_e85": "OUI",
          "e85_action": "×1.25 uniforme sous 70°C. Affiner par zone TCO si oscillations observées.",
          "e85_factor": "×1.15 à ×1.25 sous 70°C",
          "warning": False,
          "dependencies": ["ip_ti_tco_pos_fast_wf_opm_2", "ip_ti_tco_pos_slow_wf_opm_1"]
        },
        "ip_ti_tco_pos_fast_wf_opm_2": {
          "doc": "Masse film mural montée rapide mode 2 f(TCO, RPM). Mettre à jour avec mode 1.",
          "type_doc": "CARTOGRAPHIE 3D",
          "modification_e85": "OUI",
          "e85_action": "Même modification que ip_ti_tco_pos_fast_wf_opm_1.",
          "e85_factor": "×1.15 à ×1.25 sous 70°C",
          "warning": False,
          "dependencies": ["ip_ti_tco_pos_fast_wf_opm_1"]
        },
        "ip_ti_tco_pos_slow_wf_opm_1": {
          "doc": "Masse film mural montée lente mode 1 f(TCO, RPM). Même logique que fast.",
          "type_doc": "CARTOGRAPHIE 3D",
          "modification_e85": "OUI",
          "e85_action": "×1.25 sous 70°C. Même approche que fast.",
          "e85_factor": "×1.15 à ×1.25 sous 70°C",
          "warning": False,
          "dependencies": ["ip_ti_tco_pos_slow_wf_opm_2", "ip_ti_tco_pos_fast_wf_opm_1"]
        },
        "ip_ti_tco_pos_slow_wf_opm_2": {
          "doc": "Masse film mural montée lente mode 2 f(TCO, RPM). Mettre à jour avec mode 1.",
          "type_doc": "CARTOGRAPHIE 3D",
          "modification_e85": "OUI",
          "e85_action": "Même modification que ip_ti_tco_pos_slow_wf_opm_1.",
          "e85_factor": "×1.15 à ×1.25 sous 70°C",
          "warning": False,
          "dependencies": ["ip_ti_tco_pos_slow_wf_opm_1"]
        },
        "ip_ti_tco_neg_fast_wf_opm_1": { "doc": "Film mural décélération rapide mode 1. NE PAS MODIFIER.", "type_doc": "CARTOGRAPHIE 3D", "modification_e85": "NON" },
        "ip_ti_tco_neg_fast_wf_opm_2": { "doc": "Film mural décélération rapide mode 2.", "type_doc": "CARTOGRAPHIE 3D", "modification_e85": "NON" },
        "ip_ti_tco_neg_slow_wf_opm_1": { "doc": "Film mural décélération lente mode 1.", "type_doc": "CARTOGRAPHIE 3D", "modification_e85": "NON" },
        "ip_ti_tco_neg_slow_wf_opm_2": { "doc": "Film mural décélération lente mode 2.", "type_doc": "CARTOGRAPHIE 3D", "modification_e85": "NON" }
      }
    },

    # ─────────────────────────────────────────────────────────────────────
    # 9 — ip_fac_ti_temp_cor  🟠 RECOMMANDÉ
    # ─────────────────────────────────────────────────────────────────────
    {
      "xdf_category": "INJR_Injection_phase_1st_pulse",
      "label": "Correction TI f(TCO) — ip_fac_ti_temp_cor (RECOMMANDÉ)",
      "role": (
        "Facteur multiplicateur sur TI f(TCO). 12 points, −20°C à +100°C. "
        "Compense l'adhésion aux parois froides (éthanol viscosité ~×1.5 à 20°C vs essence). "
        "Symptôme si insuffisant : à-coups accélérations légères moteur tiède (30–70°C)."
      ),
      "tuning_impact": "high",
      "flex_fuel_sensitive": True,
      "tags": ["e85", "warm_up", "wall_film"],
      "undocumented_params_default": "NON",
      "parameters": {
        "ip_fac_ti_temp_cor": {
          "doc": "Facteur TI f(TCO) [sans unité]. 12 points : −20°C à +100°C.",
          "type_doc": "COURBE 2D",
          "modification_e85": "OUI",
          "e85_action": "+20%@−10°C, +18%@0°C, +15%@20°C, +10%@40°C, +5%@60°C, 1.00@80°C+.",
          "e85_factor": "×1.05 à ×1.20",
          "warning": False
        }
      }
    },

    # ─────────────────────────────────────────────────────────────────────
    # 10 — Film mural dynamique (ATMPH)  🟠 RECOMMANDÉ
    # ─────────────────────────────────────────────────────────────────────
    {
      "xdf_category": "ATMPH",
      "label": "Modèle dynamique film mural — id_fac_mff_tco / id_mff_wf / ip_fac_mff_map_wf (RECOMMANDÉ)",
      "role": (
        "Paramètres du modèle dynamique film mural pour transitions de charge. "
        "id_fac_mff_tco_pos_wf : facteur MFF accélérations f(TCO). "
        "id_fac_mff_tco_neg_wf : facteur décélérations. "
        "id_mff_inc_wf / id_mff_dec_wf : incrément/décrément lors des transitions. "
        "ip_fac_mff_map_wf : correction principale f(charge, TCO). "
        "K_ATMNN_*, K_ATM_*, K_BS*, K_DT*, S_ATM* = modèle thermique interne — non E85."
      ),
      "tuning_impact": "high",
      "flex_fuel_sensitive": True,
      "tags": ["e85", "warm_up", "wall_film"],
      "undocumented_params_default": "NON",
      "parameters": {
        "id_fac_mff_tco_pos_wf": {
          "doc": "Facteur MFF accélérations f(TCO) [sans unité]. Carburant suppl. pour alimenter le film.",
          "type_doc": "COURBE 2D",
          "modification_e85": "OUI",
          "e85_action": "+20–25% entre 20°C et 70°C.",
          "e85_factor": "×1.20 à ×1.25",
          "warning": False,
          "dependencies": ["id_fac_mff_tco_neg_wf", "id_mff_inc_wf"]
        },
        "id_fac_mff_tco_neg_wf": {
          "doc": "Facteur MFF décélérations f(TCO). Réduction injection lors des décélérations.",
          "type_doc": "COURBE 2D",
          "modification_e85": "OUI",
          "e85_action": "+15% sous 60°C (éthanol s'évapore plus lentement — plus de film résiduel).",
          "e85_factor": "×1.15 sous 60°C",
          "warning": False
        },
        "id_mff_inc_wf": {
          "doc": "Incrément MFF pour reconstituer film mural en accélération [mg/stk]. 3D map.",
          "type_doc": "CARTOGRAPHIE 3D",
          "modification_e85": "OUI",
          "e85_action": "+20% entre 20°C et 60°C.",
          "e85_factor": "×1.20",
          "warning": False,
          "dependencies": ["id_mff_dec_wf"]
        },
        "id_mff_dec_wf": {
          "doc": "Décrément MFF récupéré du film en décélération [mg/stk]. 3D map.",
          "type_doc": "CARTOGRAPHIE 3D",
          "modification_e85": "OUI",
          "e85_action": "+20% entre 20°C et 60°C (film E85 plus épais → plus récupérable).",
          "e85_factor": "×1.20",
          "warning": False,
          "dependencies": ["id_mff_inc_wf"]
        },
        "ip_fac_mff_map_wf": {
          "doc": "Facteur correction MFF wall film principal f(charge, TCO). Correction cartographiée principale.",
          "type_doc": "CARTOGRAPHIE 3D",
          "modification_e85": "OUI",
          "e85_action": "+15% en zone TCO < 60°C.",
          "e85_factor": "×1.15 sous 60°C",
          "warning": False
        }
      }
    },

    # ─────────────────────────────────────────────────────────────────────
    # 11 — Délai enrichissement WOT  🟠 RECOMMANDÉ
    # ─────────────────────────────────────────────────────────────────────
    {
      "xdf_category": "EOS_Basic_operating_states__LV_ST__LV_IS__LV_REQ_ISC__LV_PL__LV_PU___LV_PUC",
      "label": "Délai enrichissement pleine charge — c_t_ti_dly_fl (RECOMMANDÉ)",
      "role": (
        "Délai entre détection pleine charge et application enrichissement WOT. "
        "Stock : 200 ms (raw=20). Sur E85, réduire à 0 ms pour application immédiate. "
        "Modifier les 4 copies MT/AT selon la boîte."
      ),
      "tuning_impact": "medium",
      "flex_fuel_sensitive": True,
      "tags": ["e85", "performance"],
      "undocumented_params_default": "NON",
      "parameters": {
        "c_t_ti_dly_fl_1": {
          "doc": "Délai enrichissement WOT — MT [s]. Équation 0.010×raw. Stock raw=20 → 200ms.",
          "type_doc": "CONSTANTE",
          "modification_e85": "OUI",
          "e85_action": "raw=0 → 0 ms.",
          "stock_raw": 20,
          "stock_physical": "0.200 s (200 ms)",
          "warning": False,
          "dependencies": ["c_t_ti_dly_fl_2"]
        },
        "c_t_ti_dly_fl_2": {
          "doc": "Délai enrichissement WOT — MT copie 2 [s].",
          "type_doc": "CONSTANTE",
          "modification_e85": "OUI",
          "e85_action": "raw=0.",
          "stock_raw": 20,
          "stock_physical": "0.200 s (200 ms)",
          "warning": False,
          "dependencies": ["c_t_ti_dly_fl_1"]
        },
        "c_t_ti_dly_fl_at_1": {
          "doc": "Délai enrichissement WOT — AT [s].",
          "type_doc": "CONSTANTE",
          "modification_e85": "OUI",
          "e85_action": "raw=0 si AT.",
          "stock_raw": 20,
          "stock_physical": "0.200 s (200 ms)",
          "warning": False
        },
        "c_t_ti_dly_fl_at_2": {
          "doc": "Délai enrichissement WOT — AT copie 2 [s].",
          "type_doc": "CONSTANTE",
          "modification_e85": "OUI",
          "e85_action": "raw=0 si AT.",
          "stock_raw": 20,
          "stock_physical": "0.200 s (200 ms)",
          "warning": False
        }
      }
    },

    # ─────────────────────────────────────────────────────────────────────
    # 12 — ip_lamb_fl__n  🟠 NOTE (VB67774 déjà enrichi)
    # ─────────────────────────────────────────────────────────────────────
    {
      "xdf_category": "AGGR_VVTI_adaption",
      "label": "Lambda cible WOT f(RPM) — ip_lamb_fl__n (NOTE — VB67774 déjà enrichi)",
      "role": (
        "ip_lamb_fl__n : lambda cible WOT f(RPM). "
        "Stock VB67774 (lecture bin) : 0.920λ (608–4800rpm), 0.901λ (5504rpm), 0.871λ (6496rpm). "
        "⚠️ Ce bin possède déjà un enrichissement WOT baked-in. AUCUNE MODIFICATION nécessaire "
        "sur VB67774. À vérifier sonde large bande sur le véhicule."
      ),
      "tuning_impact": "high",
      "flex_fuel_sensitive": True,
      "tags": ["e85", "performance"],
      "undocumented_params_default": "NON",
      "parameters": {
        "ip_lamb_fl__n": {
          "doc": "Lambda cible WOT f(RPM) [λ]. VB67774 stock : 0.920λ (608–4800rpm), 0.901λ (5504rpm), 0.871λ (6496rpm). Déjà enrichi — pas de modification sur ce bin.",
          "type_doc": "COURBE 2D",
          "modification_e85": "NON",
          "stock_physical": "0.920λ@608-4800rpm / 0.901λ@5504rpm / 0.871λ@6496rpm (VB67774)",
          "warning": False
        }
      }
    },

    # ─────────────────────────────────────────────────────────────────────
    # 13 — Plafond anti-cliquetis  ⚪ OPTIONNEL
    # ─────────────────────────────────────────────────────────────────────
    {
      "xdf_category": "IGA_Minimum_IGA_limitation_for_exhaust_gas_temperature_protection",
      "label": "Plafond anti-cliquetis — ip_iga_bas_max_knk__n__maf (OPTIONNEL)",
      "role": (
        "ip_iga_bas_max_knk__n__maf : plafond (ceiling) d'avance f(charge, RPM). "
        "⚠️ PLAFOND ≠ AVANCE EFFECTIVE. Sur MSV70 : avance = min(modèle couple, plafond). "
        "Si le modèle couple demande déjà moins que le plafond, lever le plafond = aucun effet. "
        "ip_iga_bas_knk : ABSENT du XDF 9PPL921S — non modifiable via TunerPro. "
        "Tous les autres params (id_fil_frq_knk_*, id_itc_knk_*, ip_gain_knk_*, "
        "ip_fac_thd_knk_*, ip_knkwb_*, c_knk_*, etc.) = contrôleur knock interne — non E85."
      ),
      "tuning_impact": "medium",
      "flex_fuel_sensitive": True,
      "tags": ["e85", "ignition"],
      "undocumented_params_default": "NON",
      "parameters": {
        "ip_iga_bas_max_knk__n__maf": {
          "doc": "Plafond avance anti-cliquetis f(charge, RPM) [°CRK]. 3D Map 8×8. Équation 0.375×raw−35.625. C'est un plafond, pas l'avance effective.",
          "type_doc": "CARTOGRAPHIE 3D",
          "modification_e85": "OUI",
          "e85_action": "OPTIONNEL — seulement si cliquetis avéré par logging sur E85. Augmenter le plafond +2°à+4° en haute charge si le modèle de couple était limité par ce plafond. Valider par logging knock avant/après.",
          "warning": True,
          "warning_detail": "ip_iga_bas_knk (avance de base) absent du XDF. Si le modèle couple demande déjà moins que ce plafond, cette modification n'a aucun effet moteur."
        }
      }
    },

    # ─────────────────────────────────────────────────────────────────────
    # 14 — Avance de base  ⚪ OPTIONNEL
    # ─────────────────────────────────────────────────────────────────────
    {
      "xdf_category": "IGA_Dwell_time_control_open_loop",
      "label": "Avance de base f(charge, RPM) — ip_iga_st_bas_opm (OPTIONNEL)",
      "role": (
        "Tables d'avance à l'allumage f(charge, RPM). Sur E85 (RON 105–108 vs RON 95), "
        "+2° à +5° en pleine charge possible. Modifier UNIQUEMENT après validation injection complète."
      ),
      "tuning_impact": "high",
      "flex_fuel_sensitive": True,
      "tags": ["e85", "ignition", "performance"],
      "undocumented_params_default": "NON",
      "parameters": {
        "ip_iga_st_bas_opm_1": {
          "doc": "Avance de base mode 1 f(charge, RPM) [°CRK]. Équation 0.75×raw−48.",
          "type_doc": "CARTOGRAPHIE 3D",
          "modification_e85": "OUI",
          "e85_action": "+2° à +6° en haute charge (>200 mg/stk) et haut régime (>3000 RPM). Commencer par +2° uniforme, valider 50 km, affiner. UNIQUEMENT après validation injection.",
          "warning": False,
          "dependencies": ["ip_iga_st_bas_opm_2"]
        },
        "ip_iga_st_bas_opm_2": {
          "doc": "Avance de base mode 2 f(charge, RPM) [°CRK]. Modifier en même temps que opm_1.",
          "type_doc": "CARTOGRAPHIE 3D",
          "modification_e85": "OUI",
          "e85_action": "Même modification que ip_iga_st_bas_opm_1.",
          "warning": False,
          "dependencies": ["ip_iga_st_bas_opm_1"]
        }
      }
    },

    # ─────────────────────────────────────────────────────────────────────
    # 15 — Avance cranking  ⚪ OPTIONNEL
    # ─────────────────────────────────────────────────────────────────────
    {
      "xdf_category": "IGA_Ignition_with_multiple_spark",
      "label": "Avance cranking — c_iga_ini (OPTIONNEL)",
      "role": (
        "c_iga_ini : avance allumage lors des premiers cycles cranking. "
        "Stock VB67774 : raw=111 → 6.0°CRK. Équation : 0.375×raw−35.625."
      ),
      "tuning_impact": "low",
      "flex_fuel_sensitive": True,
      "tags": ["e85", "cold_start", "ignition"],
      "undocumented_params_default": "NON",
      "parameters": {
        "c_iga_ini": {
          "doc": "Avance allumage initiale cranking [°CRK]. raw=111 → 6.0°CRK.",
          "type_doc": "CONSTANTE",
          "modification_e85": "OUI",
          "e85_action": "OPTIONNEL — uniquement si démarrage difficile après calibration ip_mff_cst_opm. +1° (raw=114→7°CRK) à +2° (raw=116→8°CRK). Ne pas dépasser +2°.",
          "stock_raw": 111,
          "stock_physical": "6.0 °CRK",
          "warning": False
        }
      }
    },

    # ─────────────────────────────────────────────────────────────────────
    # 16–19 — c_fac_mff_ti_stnd (5 copies) — si changement injecteurs
    # ─────────────────────────────────────────────────────────────────────
    {
      "xdf_category": "INJ_Coordination_of_the_Injection_Time_Correction_Factors_for_Cylinder_Balancing",
      "label": "c_fac_mff_ti_stnd_1 — Facteur MFF→TI copie principale (si changement injecteurs)",
      "role": (
        "Facteur de conversion MFF→TI, copie principale (cyl. 1–3). Adresse 0x044AC0. "
        "Équation : 0.000006×raw. Stock 13537531634 : raw=56567 → 0.3394 ms/mg → ~237cc/min chaud. "
        "NE PAS MODIFIER si injecteurs stock. 5 copies à mettre à jour simultanément."
      ),
      "tuning_impact": "critical",
      "flex_fuel_sensitive": True,
      "tags": ["e85", "injector"],
      "undocumented_params_default": "NON",
      "parameters": {
        "c_fac_mff_ti_stnd_1": {
          "doc": "Facteur MFF→TI cyl. 1–3. 0x044AC0. Équation 0.000006×raw. Stock raw=56567 → 0.3394 ms/mg.",
          "type_doc": "CONSTANTE",
          "modification_e85": "OUI — si changement injecteurs",
          "e85_action": "UNIQUEMENT si injecteurs remplacés. Recalculer : raw_new = valeur_physique / 0.000006. Mettre à jour les 5 copies en même temps.",
          "stock_raw": 56567,
          "stock_physical": "0.3394 ms/mg (~237 cc/min chaud)",
          "warning": False,
          "dependencies": ["c_fac_mff_ti_stnd_2", "c_fac_mff_ti_stnd[0]", "c_fac_mff_ti_stnd[1]", "c_fac_mff_ti_stnd_mon"]
        }
      }
    },
    {
      "xdf_category": "ATMNN",
      "label": "c_fac_mff_ti_stnd_2 — Facteur MFF→TI copie ATMNN (si changement injecteurs)",
      "role": (
        "2e copie MFF→TI, module ATMNN (cyl. 4–6). Adresse 0x044AC2. "
        "Même équation et valeur que _1. Divergence → injection asymétrique."
      ),
      "tuning_impact": "critical",
      "flex_fuel_sensitive": True,
      "tags": ["e85", "injector"],
      "undocumented_params_default": "NON",
      "parameters": {
        "c_fac_mff_ti_stnd_2": {
          "doc": "Facteur MFF→TI cyl. 4–6. 0x044AC2. Même valeur et équation que _1.",
          "type_doc": "CONSTANTE",
          "modification_e85": "OUI — si changement injecteurs",
          "stock_raw": 56567,
          "stock_physical": "0.3394 ms/mg",
          "warning": False,
          "dependencies": ["c_fac_mff_ti_stnd_1", "c_fac_mff_ti_stnd[0]", "c_fac_mff_ti_stnd[1]", "c_fac_mff_ti_stnd_mon"]
        },
        "c_cam_ini_ex": { "doc": "Angle initial came échappement VANOS. Non E85.", "type_doc": "CONSTANTE", "modification_e85": "NON" },
        "c_cam_ini_in": { "doc": "Angle initial came admission VANOS. Non E85.", "type_doc": "CONSTANTE", "modification_e85": "NON" }
      }
    },
    {
      "xdf_category": "INJR_Fuel_temperature_correction",
      "label": "c_fac_mff_ti_stnd[0]/[1] — Copies SOI/EOI (si changement injecteurs)",
      "role": (
        "3e et 4e copies MFF→TI, module phasage SOI/EOI. "
        "Adresses 0x045AAC et 0x045AAE. "
        "Équation DIFFÉRENTE : 0.000012×raw → raw = moitié de _1/_2. "
        "Stock : raw=28284 → 0.3394 ms/mg."
      ),
      "tuning_impact": "critical",
      "flex_fuel_sensitive": True,
      "tags": ["e85", "injector"],
      "undocumented_params_default": "NON",
      "parameters": {
        "c_fac_mff_ti_stnd[0]": {
          "doc": "Copie SOI/EOI index 0. 0x045AAC. Équation 0.000012×raw. Stock raw=28284 → 0.3394 ms/mg.",
          "type_doc": "CONSTANTE",
          "modification_e85": "OUI — si changement injecteurs",
          "stock_raw": 28284,
          "stock_physical": "0.3394 ms/mg",
          "warning": False,
          "dependencies": ["c_fac_mff_ti_stnd[1]", "c_fac_mff_ti_stnd_1", "c_fac_mff_ti_stnd_2", "c_fac_mff_ti_stnd_mon"]
        },
        "c_fac_mff_ti_stnd[1]": {
          "doc": "Copie SOI/EOI index 1. 0x045AAE. Même équation que [0].",
          "type_doc": "CONSTANTE",
          "modification_e85": "OUI — si changement injecteurs",
          "stock_raw": 28284,
          "stock_physical": "0.3394 ms/mg",
          "warning": False,
          "dependencies": ["c_fac_mff_ti_stnd[0]", "c_fac_mff_ti_stnd_1", "c_fac_mff_ti_stnd_2", "c_fac_mff_ti_stnd_mon"]
        }
      }
    },
    {
      "xdf_category": "DIA_Immobilizer_signal_diagnosis",
      "label": "c_fac_mff_ti_stnd_mon — 5e copie monitoring (si changement injecteurs)",
      "role": (
        "5e copie MFF→TI, canal monitoring indépendant MSV70. Adresse 0x04958C. "
        "Équation 0.000006×raw. Divergence avec _1/_2 → DTC cohérence injection immédiat."
      ),
      "tuning_impact": "critical",
      "flex_fuel_sensitive": True,
      "tags": ["e85", "injector"],
      "undocumented_params_default": "NON",
      "parameters": {
        "c_fac_mff_ti_stnd_mon": {
          "doc": "5e copie MFF→TI canal monitoring. 0x04958C. Équation 0.000006×raw.",
          "type_doc": "CONSTANTE",
          "modification_e85": "OUI — si changement injecteurs",
          "stock_raw": 56567,
          "stock_physical": "0.3394 ms/mg",
          "warning": True,
          "warning_detail": "Oublier cette copie = DTC cohérence injection immédiat.",
          "dependencies": ["c_fac_mff_ti_stnd_1", "c_fac_mff_ti_stnd_2", "c_fac_mff_ti_stnd[0]", "c_fac_mff_ti_stnd[1]"]
        }
      }
    },

    # ─────────────────────────────────────────────────────────────────────
    # 20 — Dead time — si changement injecteurs
    # ─────────────────────────────────────────────────────────────────────
    {
      "xdf_category": "INJR_Battery_Voltage_Compensation",
      "label": "Dead time injecteur — ip_ti_min (si changement injecteurs)",
      "role": (
        "ip_ti_min : dead time électromécanique f(tension batterie). "
        "Stock injecteurs 13537531634 : calibré d'usine. "
        "NE PAS MODIFIER avec injecteurs stock. "
        "Recalibrer OBLIGATOIREMENT si injecteurs remplacés."
      ),
      "tuning_impact": "critical",
      "flex_fuel_sensitive": False,
      "tags": ["injector", "dead_time"],
      "undocumented_params_default": "NON",
      "parameters": {
        "ip_ti_min": {
          "doc": "Dead time injecteur f(Ubatt) [ms]. Stock 13537531634 : ~0.55ms@12V, ~1.00ms@8V. NE PAS MODIFIER avec injecteurs stock.",
          "type_doc": "COURBE 2D",
          "modification_e85": "NON — injecteurs stock 13537531634",
          "e85_action": "Ne pas modifier avec injecteurs stock. Recalibrer si injecteurs remplacés.",
          "warning": True,
          "warning_detail": "Oublier cette table lors d'un changement d'injecteurs = richesse incorrecte non corrigeable par les fuel trims."
        }
      }
    }

  ]
}

# ─────────────────────────────────────────────────────────────────────────
OUTPUT_DIR = os.path.join(os.path.dirname(os.path.abspath(__file__)), "knowledge")
OUTPUT_FILE = os.path.join(OUTPUT_DIR, "e85_knowledge.json")

os.makedirs(OUTPUT_DIR, exist_ok=True)
with open(OUTPUT_FILE, "w", encoding="utf-8") as f:
    json.dump(KNOWLEDGE, f, ensure_ascii=False, indent=2)

n_sub = len(KNOWLEDGE["subcategories"])
n_params = sum(len(s.get("parameters", {})) for s in KNOWLEDGE["subcategories"])
print(f"Generated {OUTPUT_FILE}")
print(f"  {n_sub} subcategories, {n_params} params documented")
