# n-corps-gravitationnel

Sujet de rattrapage CIN2 Semetre à l'international
Simulateur gravitationnel à N corps, en partant d'un cas trivial (deux corps) pour aboutir à un algorithme non trivial de complexité O(N log N) (Barnes-Hut), comparé rigoureusement, par la mesure, à l'algorithme naïf en O(N²).

## Personnalisation (Semaine 1)

Conformément aux consignes de la section 5, mes paramètres ont été dérivés de mon adresse mail institutionnelle :

- **Email :** gwenn....@isen. yncrea.fr

- **Seed :** 3712810258

- **e_perso :** 0.20 (utilisé à l'étape 7.2)

- **theta_perso :** 0.30 (utilisé à l'étape 8.2)

## Journal daté

### 1. Semaine 1

- **Liste des tâches :**
  - [x] Setup Git + dépôt GitHub, journal.md
  - [x] Bibliothèque vectorielle + tests
  - [x] Calcul de e_perso / theta_perso
  - [x] Orbite Terre-Soleil (A1)
  - [ ] Deux corps mobiles + leapfrog + orbite personnalisée (A2)

- **Travail réalisé par jour :**
  - **06/07 :**
    - Création du repo GitHub
    - Choix du langage : Rust pour la partie caluls et simulation & Python pour la partie traitement des données et visualisation graphique
    - Création de journal.md
    - Installation de Rust ('rustup', 'cargo') sur la machine locale
    - Premier code avec "Hello world"

  - **07/07 :**
    - Personnalisation des données (Tentative de compréhension : ce que je n'avais pas compris c'est qu'on devait hasher l'adresse mail)

  - **08/07 :**
    - Personnalisation des données (Compréhension du code et de la consigne) dans perso.rs

  - **09/07 :**
    - Mise en place de la première partie de la semaine 1 (A1) avec la bibliothèque vectorielle et les premiers tests concluants

  - **10/07 :**
    - Orbite Terre-Soleil dans le main.rs avec Euler
    - Création et écriture du temps et la position de la Terre dans trajectoire.csv
    - Mise en place d'un script Python pour utiliser les données dans trajectoire.csv pour construire le graphique de la trajectoire de la Terre autour du Soleil fixe
    - Passerelle avec Python pour lancer le graphique qu'avec Rust
    - Mise à jour du journal.md
    - Commit de tous le travail effectué les derniers jours

  - **11/07 :**
    - Tentative de l'auto-diagnostique A2
      - Comprendre les consignes demandés

### 2. Semaine 2

- **Liste des tâches :**
  - [ ] Deux corps mobiles + leapfrog + orbite personnalisée (A2)
  - [ ] N corps directs
  - [ ] Validation sur au moins 4 planètes (A3)
  - [ ] Nuages synthétiques + Benchmark brute force + Régression
log-log (A4, fin du socle)
  - [ ] Mise à niveau structures récursives
si besoin

- **Travail réalisé par jour :**
  - **13/07 :**
    - Tentative de l'auto-diagnostique A2
      - Comprendre les consignes demandés
      - Ebauche de solution

  - **14/07 :**
    - Tentative de l'auto-diagnostique A2
      - Aération du main en créant des fonction pour calculer l'énergie totale, les intégrations Leapfrog et Euler et les accélérations
      - Solution de l'auto diagnostique A2
    - Sauvegarde des graphique dans le dossier image
    - 

### 3. Semaine 3

- **Liste des tâches :** Quadtree
  - [ ] Construction
  - [ ] Tests unitaires
  - [ ] Cas dégénéré (B1)
  - [ ] Calcul de force Barnes-Hut
  - [ ] Validation de précision à θ=0, θ=0.5 , θ_perso (B2, coeur du sujet)

- **Travail réalisé par jour :**

### 4. Semaine 4

- **Liste des tâches :**
  - [ ] Couplage BH + leapfrog
  - [ ] Campagne de mesures
  - [ ] N de
croisement (B3, fin du coeur algorithmique)
  - [ ] Rapport
  - [ ] Script
de reproductibilité
  - [ ] Préparation soutenance

- **Travail réalisé par jour :**

### 5. Semaine 5

- **Liste des tâches :**
  - [ ]

- **Travail réalisé par jour :**

### 6. Semaine 6

- **Liste des tâches :**
  - [ ]

- **Travail réalisé par jour :**

### 7. Semaine 7

- **Liste des tâches :**
  - [ ]

- **Travail réalisé par jour :**

## Auto-diagnostique

### A1

- **Tests unitaires vectoriels**
  - Test passé avec cargo test ✅
- **Vitesse initiale calculée**
  - 2π ≈ 6,283 UA/an ✅
  - Test passé avec cargo test ✅
- **Trajectoire tracée**
  - Visuellement un cercle ✅
  - **Période mesurée**
  - Tolérance entre 0 et 1 ans = 0.06573445584 = 6.5735% < 10% ✅
  - (1.0060,0,0,1.0078801592466042,0.00041741704146491495
     0     ,0,0,1                 ,0.)
  - 0.6%

### A2

- **Apoastre mesuré proche de a(1+e_perso) , périastre mesuré proche de a(1-e_perso)**
  - Test passé avec cargo test ✅
- **Propriété remarquable à vérifier**
  - Période proche d'un an ✅
- **Dérive d'énergie relative**
  - Pour Euler :
    - Au bout de 50 tours/années : |E(t)-E(0)|/|E(0)| = 0.6145230602688544
    - Croissante ✅
  - Pour Leapfrog :
    - Au bout de 50 tours/années : |E(t)-E(0)|/|E(0)| = Variation de 0 à 1.02933940517e-05 <1%
    - Oscillement ✅
  - **Période mesurée**
  - Tolérance entre 0 et 1 ans = 0.06573445584 = 6.5735% < 10% ✅
