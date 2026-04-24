# §7 — Lambda, richesse WOT et limites correction

> Sur E85 correctement calibré, le moteur fonctionne en boucle fermée autour de lambda=1. Ces paramètres servent à (1) élargir les plages de correction pendant la phase d'apprentissage initiale pour éviter les DTC voyant moteur — **obligatoire avant le premier flash**, (2) protéger le moteur en pleine charge avec un léger enrichissement WOT — optionnel.

---

<a id="p1"></a>
## ① `ip_fac_lamb_max_fsd_1` — Plafond correction WRAF instantanée, mode 1 (anti-DTC)

| Champ | Valeur |
|---|---|
| Adresse | (voir XDF) |
| Structure | Table |

**Rôle :** Limite haute de la correction STFT instantanée autorisée par le système WRAF (Wide Range Air Fuel). Si le STFT monte au-delà de ce plafond, l'ECU considère la sonde hors de sa plage opérationnelle et peut lever un DTC → voyant moteur. Sur E85 en break-in (0–500 km), le STFT peut atteindre +18 à +22% pendant la convergence — au-delà du plafond stock de 1.15. Élargir pendant le rodage, resserrer une fois stabilisé.

**Avant / Après :**

| Phase | ◀ Avant — Stock | ✅ Après — Break-in (0–500 km) | ✅ Après — Stabilisé (> 500 km) |
|---|---|---|---|
| `ip_fac_lamb_max_fsd_1` | ~1.15 (±15%) | **1.25–1.30** (±25–30%) | **1.20** (±20%) |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Voyant moteur dans les 500 premiers km | Absent | Allumé → FSD trop serré, élargir à 1.30 |
| STFT boucle fermée | ±15% max | > ±20% continu → calibration injecteurs à revoir |

---

<a id="p2"></a>
## ② `ip_fac_lamb_max_fsd_2` — Plafond correction WRAF instantanée, mode 2 (anti-DTC)

| Champ | Valeur |
|---|---|
| Adresse | (voir XDF) |
| Structure | Table |

**Rôle :** Copie du plafond WRAF pour le mode 2. Même logique qu'ip_fac_lamb_max_fsd_1. Les deux modes doivent être cohérents — si mode 1 est élargi et mode 2 reste stock, le voyant peut s'allumer lors des commutations de mode de régulation lambda.

**Avant / Après :**

| Phase | ◀ Avant — Stock | ✅ Après — Break-in (0–500 km) | ✅ Après — Stabilisé (> 500 km) |
|---|---|---|---|
| `ip_fac_lamb_max_fsd_2` | ~1.15 (±15%) | **1.25–1.30** (±25–30%) | **1.20** (±20%) |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| Voyant moteur intermittent | Absent | Intermittent → mode 2 non modifié, commutation déclenche DTC |

---

<a id="p3"></a>
## ③ `c_lamb_delta_i_max_lam_adj` — Plafond LTFT intégral (anti-DTC)

| Champ | Valeur |
|---|---|
| Adresse | (voir XDF) |
| Structure | Constante scalaire |

**Rôle :** Valeur maximale d'accumulation de l'intégrateur LTFT (long term fuel trim). C'est la limite absolue du long terme — si l'adaptation atteint ce plafond, l'ECU ne peut plus compenser et déclare "adaptation at limit" → DTC voyant moteur. Sur E85 en break-in, le LTFT peut vouloir s'accumuler jusqu'à +25% pendant les premiers 200 km pendant que le calculateur apprend le nouveau carburant. Resserrer après stabilisation.

**Avant / Après :**

| Phase | ◀ Avant — Stock | ✅ Après — Break-in (0–500 km) | ✅ Après — Stabilisé (> 500 km) |
|---|---|---|---|
| `c_lamb_delta_i_max_lam_adj` | ~0.15–0.20 λ (~15–20%) | **0.25–0.30 λ** (25–30%) | **0.20 λ** (20%) |

**Vérification :**

| Condition | ✅ Cible | ⚠️ Action |
|---|---|---|
| LTFT après 500 km | ±5% | > ±10% → calibration injecteurs à affiner |
| Voyant "adaptation" | Absent | Allumé → LTFT max trop serré, élargir |

---

## Note — Procédure complète prévention DTC break-in

```
Avant premier flash E85 :
  1. Monter ip_fac_lamb_max_fsd_1/2 à 1.25–1.30
  2. Monter c_lamb_delta_i_max_lam_adj à 0.25–0.30 λ

Après 300–500 km :
  3. Lire LTFT avec ISTA ou scanner OBD → cible ±5%
  4. Si stable → resserrer : FSD à 1.20, LTFT max à 0.20 λ
  5. Reflasher — valeurs serrées évitent les faux DTC futurs

Voyant MIL allumé pendant break-in :
  → DTC "fuel trim" → FSD/LTFT trop serrés → élargir et reflasher
  → NE PAS effacer le DTC sans reflasher : il revient immédiatement
```

---

<a id="p4"></a>
## ④ `ip_lamb_fl__n` — Lambda cible pleine charge f(RPM) — OPTIONNEL

| Champ | Valeur |
|---|---|
| Adresse | 0x436A2 |
| Structure | Courbe 1×12 |
| Axe | X = RPM (608–6496 tr/min) |

**Rôle :** Consigne de richesse ciblée par le calculateur en mode pleine charge (WOT). Stock N52B30 : déjà enrichi à λ 0.920 en WOT (descend à 0.871 à 6500 RPM). Sur E85, cette richesse protège le moteur — rien d'obligatoire à modifier. Option B : dé-enrichir légèrement (λ 0.940–0.950) pour gagner un peu de puissance en exploitant la chaleur de vaporisation de l'éthanol (qui refroidit naturellement la chambre).

**◀ Avant — Stock (λ)**

| RPM (tr/min) | 608 | 992 | 1216 | 1600 | 2016 | 2496 | 3008 | 3520 | 4128 | 4800 | 5504 | 6496 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| λ | 0.920 | 0.920 | 0.913 | 0.920 | 0.920 | 0.920 | 0.920 | 0.920 | 0.920 | 0.920 | 0.901 | 0.871 |

Option A (recommandée) : **laisser stock** — λ 0.920 WOT déjà présent, E85 ne nécessite pas d'enrichissement supplémentaire.

**✅ Après — Option B : dé-enrichissement léger (gain puissance)**

| RPM (tr/min) | 608 | 992 | 1216 | 1600 | 2016 | 2496 | 3008 | 3520 | 4128 | 4800 | 5504 | 6496 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| λ | **0.950** | **0.950** | **0.945** | **0.950** | **0.950** | **0.945** | **0.945** | **0.940** | **0.935** | **0.930** | **0.920** | **0.900** |

> Conserver les cellules 5504 et 6496 rpm à λ bas (0.900–0.920) : protection thermique soupapes à très haut régime.

**Vérification :**

| Signal | ✅ Cible | ⚠️ Action |
|---|---|---|
| Lambda WOT (sonde large bande) | 0.90–0.95 | Hors plage → ajuster ip_lamb_fl__n |
| LTFT roulage | ±5% | > +10% → ip_mff_cor_opm_* trop petit / < −10% → trop grand |
