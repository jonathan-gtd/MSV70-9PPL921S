# Lexique — Codage véhicule

> Terminologie NCS Expert / NCS Dummy.  
> Pour le vocabulaire de calibration moteur, voir [tutos-reprog/LEXIQUE.md](../tutos-reprog/LEXIQUE.md).

---

## 1. Les termes centraux

| Terme | Allemand | Ce que c'est |
|---|---|---|
| **FSW** | *Funktionsschlüsselwort* | La **fonction** codable : « que fait-on ? ». Ex. la remontée des vitres à la clé |
| **PSW** | *Parameterschlüsselwort* | La **valeur** associée : « comment ? ». Ex. `aktiv` / `nicht_aktiv` |
| **FA** | *Fahrzeugauftrag* | Le **bon de commande usine**. Type + liste des options. La référence pour tout recoder d'origine |
| **ZCS** | *Zentrale Codier-Schlüssel* | Ancêtre du FA (E36/E46). Inutilisé sur E9x |
| **Salapa** | *Sonderausstattung-Paket* | Les codes d'option du FA : `$205`, `$494`, `$8SP`… |
| **SG** | *Steuergerät* | Un **calculateur / module** (FRM, KOMBI, CAS…) |
| **DATEN** | — | La base de données de codage BMW. Tout ce que les outils savent, ils le lisent là |
| **Baureihe** | — | La **série** de véhicules. Ici **E89**, qui couvre les E8x/E9x |

---

## 2. Les fichiers

| Extension | Rôle | Emplacement |
|---|---|---|
| `.TRC` | **Trace** — ce qui a été **lu** dans un module | `C:\NCSEXPER\WORK\FSW_PSW.TRC` |
| `.MAN` | **Manipulation** — ce qui sera **écrit** dans un module | `C:\NCSEXPER\WORK\FSW_PSW.MAN` |
| `.PFL` | Profil NCS Expert (droits d'écriture) | `C:\NCSEXPER\PFL\` |
| `.C0x` | Fichier DATEN d'un module, par variante matérielle | `C:\NCSEXPER\DATEN\E89\` |
| `.PRG` | Description EDIABAS d'un module (jobs de diagnostic) | `C:\EDIABAS\ECU\` |

**Moyen mnémotechnique :** `TRC` = **T**race = lecture. `MAN` = **MAN**ipulation = écriture.

---

## 3. Les jobs NCS Expert

| Job | Effet |
|---|---|
| `CODIERDATEN_LESEN` | **Lit** le codage du module → produit `FSW_PSW.TRC`. Sans risque |
| `SG_CODIEREN` | **Écrit** dans le module. Source selon le profil : le `.MAN` (profil NCS Dummy) ou le FA (profil Default) |
| `FA_LESEN` | Lit le bon de commande (job EDIABAS, utilisable dans Tool32 sur `CAS.PRG`) |

---

## 4. Les deux méthodes de codage

| | **Codage manuel (FSW/PSW)** | **VO coding (depuis le FA)** |
|---|---|---|
| Profil | NCS Dummy profile | Default Profil |
| Source | `FSW_PSW.MAN` | Le bon de commande |
| Portée | Les seules lignes du `.MAN` | **Le module entier** |
| Usage | Ajustements ciblés | Rétrofits d'options usine, retour à l'état d'origine |
| Effet de bord | Aucun | **Écrase tous les codages manuels du module** |

> 📌 Règle : faire les VO codings **d'abord**, les ajustements manuels **ensuite**. L'inverse efface le travail.

---

## 5. Les outils

| Outil | Rôle | Chemin |
|---|---|---|
| **NCS Expert** | Lit et écrit dans les modules | `C:\NCSEXPER\BIN\NCSEXPER.exe` |
| **NCS Dummy** | Traduit les DATEN en clair, génère les `.MAN` | `C:\NCS Dummy\NcsDummy.exe` |
| **Tool32** | Exécute un job EDIABAS isolé — utile en diagnostic | `C:\EDIABAS\Bin\Tool32.exe` |
| **INPA** | Diagnostic constructeur | `C:\EC-APPS\INPA\BIN\INPA.exe` |
| **WinKFP** | **Reprogrammation** (flash), pas codage | — |
| **BMW Coding Tool** | Installe/met à jour les DATEN — déjà fait, **ne pas relancer** | — |
| **EDIABAS** | La couche de communication sous tous les outils ci-dessus | `C:\EDIABAS\` |

---

## 6. Codage vs reprogrammation

| | Codage | Reprogrammation |
|---|---|---|
| Allemand | *Codieren* | *Programmieren* |
| Ce qui change | Des **paramètres** dans un module | Le **firmware** d'un module |
| Outil | NCS Expert | WinKFP / flasher |
| Réversible | ✅ oui, depuis le FA | 🔶 seulement avec le bin d'origine |
| Risque | Module mal configuré | Module briqué |

**Sur cette voiture :** la DME MSV70 relève de la reprogrammation ([tutos-reprog/](../tutos-reprog/README.md)) et **ne doit jamais être touchée par NCS Expert**.

---

## 7. Abréviations des libellés allemands

| Fragment | Sens |
|---|---|
| `aktiv` / `nicht_aktiv` | activé / désactivé |
| `mit` / `ohne` | avec / sans |
| `TUER` | porte |
| `SPIEGEL` | rétroviseur |
| `LICHT` | lumière |
| `SCHEIBE` | vitre |
| `WARN` | alerte |
| `ZEIT` | durée |
| `WERT` | valeur |
| `KL_R` / `KL15` / `KL30` | bornes électriques (contact, permanent) |
| `US` / `ECE` / `EU` / `JP` | marché destinataire |
| `NICHT_BELEGT` | non utilisé / non affecté |
