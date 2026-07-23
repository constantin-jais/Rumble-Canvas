[English](README.md) · **Français**

> [!NOTE]
> **Réservé · futur foyer de Spec Studio** — reconstruit dans le dépôt de base canonique [`libre-ai/libre-ai`](https://github.com/libre-ai/libre-ai) ([topologie multi-dépôts, ADR-0008](https://github.com/libre-ai/libre-ai/blob/main/docs/adr/0008-multi-repo-target-topology-and-brand.md)).
> Ce dépôt rouvrira comme dépôt produit réel lorsque le propriétaire l'activera, consommant la base comme dépendance versionnée. Les fondations décrites ci-dessous sont **en cours de construction** — avec des liens vers le code qui existe déjà.

# Spec Studio

**Espace de travail produit piloté par les contrats pour transformer des conversations en décisions, spécifications et remises d'exécution.** Créez une intention ; ajoutez des exigences, des décisions et des contrats sourcés ; figez un **SpecPackage** immuable et adressé par contenu avec approbations et preuves ; émettez une remise réservée au planification que les planificateurs aval consomment sans capacité d'exécution. Jamais silencieux, toujours traçable — chaque paquet est verrouillé par règle, jamais muté.

Le cas canonique auquel il répond : _« on en parle, on décide, on l'écrit, on le remet »_ — un espace de travail borné, autonome qui rend chaque étape explicite et le rôle de chaque participant distinct (auteur, relecteur, approbateur). Le raisonnement interne et la remise externe sont tous deux des produits de premier plan, indépendamment versionnables.

## Ce qui le distingue

- **Piloté par les contrats, pas par la Markdown.** Une spécification n'est pas un document — c'est un SpecPackage structuré contenant le problème, les acteurs, les exigences (chacune avec priorité), les contrats sourcés, les fiches de décision, les atténuations de risque et les critères d'acceptation avec preuve observable. La Markdown vit dans le champ de preuve, jamais comme paquet lui-même.
- **État accepté immuable.** Un paquet accepté est adressé par contenu par son digest SHA-256 et **jamais** muté. L'approbation crée un lien de lignage nommé ; la supersession démarre une nouvelle version, jamais un patch.
- **Refus par défaut sur complétude.** Problème manquant, décisions ouvertes, contrats non mappés à travers les frontières, critères non vérifiables ou approbations inadéquates (pas de séparation) bloquent l'acceptation. La validation retourne des identifiants de règles stables ; elle ne remplit jamais les blancs automatiquement.
- **La remise est une capacité distincte.** La remise réservée au planification porte la référence au paquet accepté et les preuves, n'accorde zéro droit d'exécution et comprend une attestation Biscuit vérifiant le digest et l'audience. Le planificateur lit ; le planificateur n'exécute jamais.
- **Collaboration en temps réel, figée à la soumission (planifiée).** Le design ([#198](https://github.com/libre-ai/libre-ai/pull/198), signé par le propriétaire) prévoit que auteurs et relecteurs co-éditent l'espace de travail DRAFT en temps réel via MLS chiffré de bout en bout — la brique de collaboration souveraine de Libre AI, encore à implémenter — puis fige le CRDT en fils de commentaires append-only une fois soumis pour relecture.

## État — spécifié publiquement, fondations en construction

Spec Studio est reconstruit à partir de contrats verrouillés. Il **n'est pas encore publié** ; le domaine et la logique d'acceptation viennent d'abord, et une bonne partie existe déjà et est prouvée dans le dépôt de base :

| Fondation                                                                           | État                 | Preuve                                                                                                                                                                                                                                                                                              |
| ----------------------------------------------------------------------------------- | -------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **`spec-package.v1`** — schéma immuable adressé par contenu                         | ✅ construit, validé | Contrat et vecteurs golden ([specs.v1.schema.json](https://github.com/libre-ai/libre-ai/blob/main/contracts/schemas/spec-package.v1.schema.json))                                                                                                                                                   |
| **`agent-handoff.v1`**, **`evidence-report.v1`** — exports                          | ✅ construit, validé | Schémas de remise réservée au planification et de preuve ([agent-handoff.v1](https://github.com/libre-ai/libre-ai/blob/main/contracts/schemas/agent-handoff.v1.schema.json), [evidence-report.v1](https://github.com/libre-ai/libre-ai/blob/main/contracts/schemas/evidence-report.v1.schema.json)) |
| **Agrégat du cycle de vie de l'espace de travail** — cadre, soumission, acceptation | ✅ construit, testé  | Logique de domaine, protections RLS, événements append-only (#172)                                                                                                                                                                                                                                  |
| **Validateur de spec-package accepté** — règles de complétude                       | ✅ construit, testé  | Validateur refus-par-défaut correspondant au schéma SpecPackage v1 (#166)                                                                                                                                                                                                                           |
| **Persistance d'espace de travail spec** — PostgreSQL, instantanés + événements     | ✅ construit, testé  | Tenant-scoped, RLS, acceptation immuable, migration 0001 (#176)                                                                                                                                                                                                                                     |
| **Service de commandes** — orchestration du chemin d'écriture                       | ✅ construit, testé  | Compose la logique de domaine et la persistance pour toutes les opérations du cycle de vie (#177)                                                                                                                                                                                                   |
| **Couture d'acceptation** — porte de décision + validation                          | ✅ construit, testé  | `decideAcceptance()` pur + transition persistée, gestion des refus (#178)                                                                                                                                                                                                                           |
| **Magasin de spec-packages adressé par contenu** — persistance dédiée               | ✅ construit, testé  | Stockage idempotent, conflits de digest refusés, append-only immuable (#201)                                                                                                                                                                                                                        |
| **Cockpit en lecture seule** — vue rendue côté serveur                              | ✅ construit, testé  | Liste et détails de l'espace de travail ; l'interface utilisateur d'édition arrive ensuite (#180)                                                                                                                                                                                                   |
| Interface utilisateur d'édition (cadre/exigence/contrat/relecture)                  | ⏳ suite             | Créer et éditer les espaces de travail DRAFT, ajouter des exigences, enregistrer les décisions et contrats                                                                                                                                                                                          |
| Autorisation de surface de commande (Biscuit, isolation de tenant)                  | ⏳ suite             | Opérations auteur/relecture/approbation/export, atténuation de capacité de remise                                                                                                                                                                                                                   |
| Conformité du consommateur réservé au planification                                 | ⏳ avancé            | Intégration Missions/orchestrateur, vérification de remise                                                                                                                                                                                                                                          |
| Qualification de concurrence et de rollback                                         | ⏳ avancé            | Résolution de conflits, timetravel sûr, garantie d'immuabilité des preuves                                                                                                                                                                                                                          |

Ce dépôt est une réserve publique ; l'implémentation legacy qu'il porte encore est gelée pour référence, et la reconstruction se passe dans le dépôt de base jusqu'à l'activation (vague 4). **Cible de référence :** outillage de gouvernance de flux de travail et de spécification (p. ex. Notion, Figma design specs) — atteinte par une structure explicitement pilotée par les contrats plutôt que par des documents libres.

## Comment ça fonctionne

1. **Cadre** — l'auteur crée un espace de travail avec énoncé du problème, acteurs, contraintes et hypothèses initiales. L'espace de travail s'ouvre en mode DRAFT ; tous les champs sont mutables. La validation expose immédiatement le problème manquant, les acteurs ou les décisions requises.
2. **Spécifier et relire** — l'auteur ajoute des exigences (avec priorité), des contrats sourcés pour les dépendances cross-boundary, des atténuations de risque et des critères d'acceptation avec preuve observable. Les relecteurs lisent et commentent ; quand prêt, l'auteur soumet le paquet pour relecture. Dans le design signé, quand `collab_enabled`, éditeur et relecteurs partagent un espace de travail chiffré en temps réel (clés MLS par-epoch) et le CRDT se fige en commentaires append-only à la soumission — la brique de collaboration qui porte cela est planifiée, pas encore construite.
3. **Accepter** — l'approbateur invoque la couture d'acceptation : la validation de complétude s'exécute (problème, acteurs, exigences, contrats, risques, critères, séparation d'approbation) et en cas de succès, l'espace de travail se fige dans un **SpecPackage** immuable, adressé par contenu avec digest et signatures d'approbation. La supersession lie la nouvelle version à l'ancienne ; les versions passées ne sont jamais réécrites.
4. **Remise** — l'utilisateur autorisé émet une remise réservée au planification : un message portant le hash du paquet accepté, les références de preuves et l'attestation Biscuit. Le planificateur aval charge la remise, vérifie le digest et consomme la spécification pour le planification — jamais pour l'exécution ou la mutation.

## Architecture — assemblé à partir de briques interopérables

Spec Studio est un produit assemblé à partir de briques versionnées indépendamment ; chacune est utilisable et testable seule, et le produit est leur composition (la cible multi-dépôts de l'[ADR-0008](https://github.com/libre-ai/libre-ai/blob/main/docs/adr/0008-multi-repo-target-topology-and-brand.md)).

| Brique                                       | Rôle                                   | Interface exposée / consommée                                                                                     |
| -------------------------------------------- | -------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| **`spec-package.v1`** (JSON Schema + golden) | La structure de spécification immuable | Adressée par contenu par digest SHA-256 ; validateur retourne les identifiants de règles ; pas d'API de mutation  |
| **`@libre-ai/web-platform`**                 | Fondation SSR / BFF Bun                | Gestionnaire de requêtes, cockpit rendu côté serveur, markup d'accessibilité-first                                |
| **`@libre-ai/data`**                         | Couche de persistance PostgreSQL       | Magasin du cycle de vie de l'espace de travail, événements append-only, spec-packages adressés par contenu, RLS   |
| **Contrats**                                 | Surface d'interopérabilité verrouillée | Schémas `spec-package.v1`, `agent-handoff.v1`, `evidence-report.v1`, OpenAPI `specifications.v1`, vecteurs golden |

L'hôte qui autorise passe au validateur les octets de spécification canoniques ; le validateur retourne une preuve règle par règle. Tout consommateur qui parle les mêmes contrats peut lire et vérifier le paquet.

## Où se déroule le travail

Tout le développement actif est dans le dépôt de base, sous :

- `apps/specifications` — l'hôte produit (cockpit SSR, service de commandes, persistance)
- `contracts/schemas/spec-package.v1.schema.json` — la définition immuable SpecPackage
- `contracts/schemas/agent-handoff.v1.schema.json`, `evidence-report.v1.schema.json` — contrats de remise et de preuve
- `contracts/openapi/specifications.v1.yaml` — la surface API
- [`docs/apps/specifications.md`](https://github.com/libre-ai/libre-ai/blob/main/docs/apps/specifications.md) — le cahier des charges produit complet

Pour suivre l'avancement ou contribuer, ouvrez issues et pull requests dans [`libre-ai/libre-ai`](https://github.com/libre-ai/libre-ai). Ce dépôt reste réservé jusqu'à son activation.

## Licence

EUPL-1.2.
