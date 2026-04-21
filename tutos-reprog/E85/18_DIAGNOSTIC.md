# §21 — Diagnostic Rapide des Problèmes

| Symptôme | Cause la plus probable | Solution |
|---|---|---|
| Pas de démarrage à froid | ip_mff_cst_opm trop pauvre | +20% cranking |
| Démarrage laborieux (10+ tours) | ip_mff_cst_opm insuffisant | +15% cranking + vérifier batterie |
| Cale après démarrage froid | ip_fac_lamb_wup insuffisant | +0.10 à 20–40°C |
| Trou/hésitation à 40–60°C | film mural insuffisant | +10% sur `ip_ti_tco_pos_fast_wf_opm_1` / `ip_ti_tco_pos_fast_wf_opm_2` ligne TCO concernée |
| STFT > +15% en permanence | ip_mff_cor_opm trop faible | +3–5% sur les 4 maps |
| STFT < −15% en permanence | ip_mff_cor_opm trop élevé | −3–5% sur les 4 maps |
| Ralenti instable moteur chaud | EVAP ou STFT oscillant | Vérifier canister purge, STFT en temps réel |
| Cliquetis pleine charge | Avance trop haute | −1° à −2° immédiatement |
| Perte puissance progressive | Filtre bouché ou pompe fatiguée | Changez filtre / testez pompe |
| Fumée noire échappement | ip_fac_lamb_wup trop riche | −0.05 à 0.08 |
| LTFT monte sur autoroute | ip_mff_cor_opm trop faible | +3–5% sur les 4 maps |
| Odeur forte éthanol à l'arrêt | Fuite carburant (joints) | Inspection immédiate obligatoire |

---

