# §14 — Purge Canister EVAP

> Les vapeurs E85 sont plus riches en éthanol et plus denses que les vapeurs essence — lors des cycles de purge canister, elles enrichissent temporairement le mélange. La boucle fermée corrige automatiquement. Deux paramètres peuvent être ajustés si les oscillations STFT lors de la purge sont excessives.

---

<a id="p1"></a>
## ① `ip_flow_max_cps` — Débit maximal de purge canister

| Champ | Valeur |
|---|---|
| Adresse | (voir XDF) |
| Structure | Courbe ou map f(conditions) |
| Équation | (voir XDF) |

**Rôle :** Limite le débit d'air maximum autorisé à travers la vanne de purge canister (CPS). Sur essence, ce débit est calibré pour une densité de vapeurs d'hydrocarbures standard. Sur E85, les vapeurs d'éthanol sont plus denses et ont un pouvoir calorifique inférieur — un débit identique à l'essence enrichit davantage le mélange et peut créer des oscillations STFT importantes lors des purges. Réduire de 10–15% si les STFT oscillent de façon prononcée lors des purges.

**Avant / Après :**

| | ◀ Valeur stock | ▶ Valeur E85 |
|---|---|---|
| `ip_flow_max_cps` | Valeur d'usine | **Stock − 10 à 15%** si STFT oscillent > ±15% lors purge |
| `ip_flow_max_cps` | Valeur d'usine | **Inchangé** si STFT stable lors purge |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| STFT pendant purge (~50 km/h, moteur chaud) | Oscillations ±10% max | > ±15% continu → réduire ip_flow_max_cps −10% |
| STFT après purge (vanne fermée) | Retour à ±5% | Reste décalé → problème LTFT, pas de purge |

---

<a id="p2"></a>
## ② `ip_flow_cps` — Débit nominal de purge canister

| Champ | Valeur |
|---|---|
| Adresse | (voir XDF) |
| Structure | Courbe ou map f(conditions) |
| Équation | (voir XDF) |

**Rôle :** Débit nominal de la purge canister (valeur courante utilisée en conditions normales, en dessous du maximum). Agit en tandem avec ip_flow_max_cps — réduire les deux si les oscillations STFT persistent. Sur E85, réduire de 15% si le STFT oscille de plus de ±10% de façon systématique lors des purges.

**Avant / Après :**

| | ◀ Valeur stock | ▶ Valeur E85 |
|---|---|---|
| `ip_flow_cps` | Valeur d'usine | **Stock − 15%** si STFT > ±10% lors purge |
| `ip_flow_cps` | Valeur d'usine | **Inchangé** si STFT stable lors purge |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| STFT pendant purge active | Oscillations ±10% max | > ±10% systématique → réduire ip_flow_cps −15% |
| Comportement EVAP général | Aucun voyant EVAP | P0442/P0455/P0456 → problème de canister, pas de ce paramètre |

> **Règle générale :** modifier ip_flow_max_cps en premier. Si insuffisant, réduire également ip_flow_cps. Ne jamais mettre à zéro — la purge canister est nécessaire pour le bon fonctionnement du système EVAP et peut déclencher des DTCs si désactivée.
