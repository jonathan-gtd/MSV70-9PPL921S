# §18 — Protection EGT : c_teg_max_iga

| Paramètre | Adresse | Valeur stock |
|---|---|---|
| `c_teg_max_iga` | 0x44F54 | **865 °C** |

Sur E85, l'EGT WOT est ~30–50°C inférieure → protection se déclenche beaucoup moins souvent. Cette marge thermique justifie les +2.5° d'avance de la stratégie E60-safe (§3).

| Situation | Essence | E85 |
|---|---|---|
| EGT WOT 6500 rpm | ~800–850°C | ~750–800°C |
| Déclenchement protection | Fréquent haute charge | Rare |

**Verdict : ne pas modifier.** Protège contre pannes sonde lambda ou carburant hors spec. Augmenter ce seuil est dangereux.

---

