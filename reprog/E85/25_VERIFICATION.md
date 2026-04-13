# §25 — État de Vérification du Tuto

Ce tuto est issu d'un audit du XDF `BMW_Siemens_MSV70_9PPL921S_2560K.xdf` et du bin `VB67774_921S_Full.bin` du dépôt. Voici l'état honnête de chaque section :

| Section | Statut | Niveau de confiance |
|---|---|---|
| §1 Injecteurs (`ip_mff_cor_opm_*`) | ✅ Adresses + valeurs vérifiées — c_fac reste stock, overflow documenté | **Élevé** |
| §2 Cranking (`c_tco_n_mff_cst`, `ip_mff_cst_opm_*`) | ✅ Vérifié | **Élevé** |
| §2.3 `ip_fac_lamb_wup` | ⚠️ Adresse corrigée (0x42764), axes confirmés (MAF×RPM) | **Moyen** — la stratégie d'utilisation reste à valider en pratique |
| §3 Avance (`ip_iga_bas_max_knk__n__maf`) | ⚠️ Table identifiée comme « plafond knock » mais le modèle de couple MSV70 est complexe ; il existe aussi `ip_iga_min_n_maf_opm_*`, `ip_fac_eff_iga_opm_*`, `ip_iga_ofs_max_knk` qui interagissent. | **Moyen** — l'effet réel d'une modif +5° devrait être validé sur banc avant tout test piste |
| §4 Lambda (`ip_lamb_fl__n` comme vraie table WOT) | ✅ Description XDF lue directement, stock vérifié | **Élevé** |
| §5 Film mural (vraies tables `ip_ti_tco_*_*_wf_opm_*`) | ⚠️ Tables identifiées via descriptions XDF ; multiplicateur ×1.20 = recommandation conservative basée sur la physique, pas sur retour d'expérience N52 publié | **Moyen** |
| §6 EVAP | ⚠️ Mentionné mais non vérifié en détail | **Faible** |
| §7 Compléments (deadtime `ip_ti_add_dly`, délai FL) | ✅ Vérifié | **Élevé** |
| §13 Paramètres non couverts | ✅ Adresses + valeurs stock vérifiées au bit près pour 13.1–13.9. Correction apportée : LTFT réel = −8%/+12% (pas ±25%). Recommandations E85 basées sur la physique et les valeurs bin. | **Moyen** — impact réel à valider sur banc |

**Limites de cet audit :**
- Aucun test sur véhicule réel n'a été conduit. Les recommandations sont basées sur la physique de la combustion E85 et la lecture du XDF, pas sur un retour d'expérience validé sur N52 + MSV70 + injecteurs stock.
- Le modèle de couple MSV70 (torque model) est volumineux et imbriqué ; certaines interactions (par ex. comment `ip_fac_eff_iga_opm_1` module l'avance demandée) n'ont pas été tracées en détail.
- Les checksums du bin ne sont pas vérifiés par ce tuto. TunerPro RT les corrige automatiquement avec ce XDF, mais c'est à valider avant flash.
- Aucune version « bin déjà calibré E85 » n'est fournie. Ce tuto donne des valeurs cibles, pas un fichier prêt à flasher.

**Avant tout flash :**
1. Lire l'avertissement flex-fuel en tête de document
2. Avoir 2 backups du bin stock
3. Tester d'abord les modifications de §1 (injecteurs) seules, valider STFT, puis ajouter §2 (cranking), valider, etc. — ne jamais tout modifier d'un coup
4. Avoir un scanner OBD2 capable de lire STFT/LTFT/lambda en temps réel
5. Pour la phase avance §3, avoir idéalement un accès banc dynamométrique ou au minimum un log de cliquetis détaillé

---

