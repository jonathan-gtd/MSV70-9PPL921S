# Purge Canister EVAP

Les vapeurs E85 sont plus riches en éthanol et plus denses que les vapeurs essence — lors des cycles de purge canister, elles enrichissent temporairement le mélange. La boucle fermée corrige automatiquement dans la plupart des cas. Intervenir uniquement si les oscillations STFT lors de la purge sont excessives.

**À ne modifier qu'en présence d'oscillations STFT > ±10% lors de la purge active.** Ne jamais mettre à zéro — la purge canister est nécessaire et son désactivation peut déclencher des DTCs EVAP (P0442/P0455/P0456).

---

<a id="p1"></a>
## ① `ip_flow_max_cps` — Débit maximal de purge canister

| Champ | Valeur |
|---|---|
| Adresse | (voir XDF) |
| Type | Courbe ou map f(conditions) |
| Unité | voir XDF |

**Rôle :** Limite le débit d'air maximum autorisé à travers la vanne de purge canister (CPS). Sur E85, les vapeurs d'éthanol sont plus denses et ont un pouvoir calorifique inférieur — un débit identique à l'essence enrichit davantage le mélange. Réduire de 10–15% si les STFT oscillent de façon prononcée lors des purges.

**Avant / Après :**

| Condition | ◀ Stock VB67774 | ✅ E85 |
|---|---|---|
| STFT stable lors purge (< ±10%) | Valeur d'usine | **Inchangé** |
| STFT oscillant > ±15% lors purge | Valeur d'usine | **Stock − 10 à 15%** |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| STFT pendant purge (~50 km/h, moteur chaud) | Oscillations ±10% max | > ±15% continu → réduire ip_flow_max_cps de −10% |
| STFT après purge (vanne fermée) | Retour à ±5% | Reste décalé → problème LTFT, pas de purge |

---

<a id="p2"></a>
## ② `ip_flow_cps` — Débit nominal de purge canister

| Champ | Valeur |
|---|---|
| Adresse | (voir XDF) |
| Type | Courbe ou map f(conditions) |
| Unité | voir XDF |

**Rôle :** Débit nominal de la purge canister (valeur courante en conditions normales, en dessous du maximum). Agit en tandem avec `ip_flow_max_cps` — réduire les deux si les oscillations STFT persistent après avoir réduit le maximum.

**Avant / Après :**

| Condition | ◀ Stock VB67774 | ✅ E85 |
|---|---|---|
| STFT stable lors purge (< ±10%) | Valeur d'usine | **Inchangé** |
| STFT oscillant > ±10% lors purge | Valeur d'usine | **Stock − 15%** |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| STFT pendant purge active | Oscillations ±10% max | > ±10% systématique → réduire ip_flow_cps de −15% |
| Voyant EVAP | Absent | P0442/P0455/P0456 → problème de canister, pas de ce paramètre |

> **Règle générale :** modifier `ip_flow_max_cps` en premier. Si insuffisant, réduire également `ip_flow_cps`. Ne jamais mettre à zéro.
