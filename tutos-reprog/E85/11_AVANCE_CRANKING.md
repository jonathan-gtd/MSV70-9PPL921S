# §9 — Avance à l'allumage au cranking

> Paramètre optionnel. Sur E85 correctement calibré (§5 — `ip_mff_cst_opm_*`), le démarrage doit être inférieur à 3 tours sans toucher l'avance de cranking. N'intervenir ici qu'en dernier recours si le démarrage reste difficile malgré la calibration cranking correcte.

---

<a id="p1"></a>
## ① `c_iga_ini` — Avance initiale cranking — OPTIONNEL

| Champ | Valeur |
|---|---|
| Adresse | 0x44B2A |
| Structure | Constante scalaire |

**Rôle :** Avance à l'allumage appliquée lors du premier cycle d'allumage pendant la phase de cranking (moteur en rotation, première combustion). Stock = 6.0°CRK. L'éthanol a un indice d'octane plus élevé que l'essence (104–108 RON vs 95), ce qui lui permet de supporter plus d'avance sans détonation. Un léger supplément d'avance au cranking peut aider le premier allumage à se propager plus efficacement dans une chambre froide chargée en éthanol peu vaporisé. Modifier uniquement si démarrage > 5 tours malgré §5 correctement calibré — et progressivement.

**Avant / Après :**

| Scénario | ◀ Stock | ✏️ E85 |
|---|---|---|
| Recommandé — inchangé | 6.0 °CRK | 6.0 °CRK |
| Option +1° (démarrage > 5 tours) | 6.0 °CRK | **7.13 °CRK** |
| Option +2° (démarrage > 8 tours) | 6.0 °CRK | **7.88 °CRK** |

> Maximum recommandé : +2° (7.88 °CRK max). Au-delà, risque de détonation au cranking sur moteur froid.

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Démarrage froid (TCO < 10°C) | < 3 tours | > 5 tours → vérifier §5 en priorité avant de toucher `c_iga_ini` |
| Démarrage chaud (TCO > 80°C) | Immédiat (1 tour) | Calage → problème d'injecteurs ou de sonde, pas d'avance |
| Son de cliquetis au cranking | Absent | Son métallique bref → réduire de −1° immédiatement |
