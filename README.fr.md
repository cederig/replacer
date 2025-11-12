# Replacer

`replacer` est un outil en ligne de commande ultra-rapide écrit en Rust qui permet de remplacer toutes les occurrences d'une chaîne de caractères donnée par une autre chaîne de caractères dans des fichiers spécifiés. Avec des optimisations avancées, un traitement parallèle et une sélection d'algorithme intelligente.

## Fonctionnalités

`replacer` est conçu pour être extrêmement rapide avec de multiples optimisations de performance :

- **40-50% plus rapide que les implémentations naïves** grâce au traitement en une seule passe
- **Traitement parallèle** pour les gros fichiers (>1Mo par défaut) utilisant Rayon
- **Optimisation ASCII** pour le contenu ASCII uniquement (2-3x plus rapide)
- **Mise en cache intelligente** pour les opérations répétées
- **Streaming économe en mémoire** pour les très gros fichiers
- **Support multi-patterns** utilisant l'algorithme Aho-Corasick
- **Détection automatique d'encodage** avec support BOM
- **Traitement conscient UTF-8** avec gestion sécurisée des limites

### Benchmarks de Performance

| Cas de test | Optimisé | Original | Amélioration |
|-------------|-----------|----------|--------------|
| 100k remplacements | 2.5ms | 4.2ms | **+40%** |
| Aucune correspondance | 389µs | 775µs | **+50%** |
| Pattern large | 880µs | 1.4ms | **+37%** |
| Optimisation ASCII | 1.9ms | N/A | **Nouvelle fonctionnalité** |

## Dépendances

Ce projet utilise les dépendances suivantes (telles que définies dans `Cargo.toml`) :

- `clap` (version `4.5.51`) : Pour l'analyse des arguments de la ligne de commande
- `indicatif` (version `0.18.2`) : Pour afficher une barre de progression
- `encoding_rs` (version `0.8.35`) : Pour la gestion des encodages de texte
- `rayon` (version `1.11.0`) : Pour le traitement parallèle
- `aho-corasick` (version `1.1.4`) : Pour le remplacement multi-patterns
- `once_cell` (version `1.21.3`) : Pour le mécanisme de mise en cache
- `tempfile` (version `3.14.0`) : Pour les tests

## Installation

### Prérequis

Assurez-vous d'avoir Rust et Cargo d'installés sur votre système. Vous pouvez les installer en suivant les instructions sur le site officiel de Rust : [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### Compilation pour Linux (depuis Linux)
1. Clonez ce dépôt :
    ```sh
    git clone https://github.com/cederig/replacer.git
    cd replacer
    ```
2. Compilez le projet :
    ```sh
    cargo build --release
    ```
    L'exécutable se trouvera dans `target/release/replacer`.

### Compilation pour Windows (depuis Linux/macOS)

Pour compiler ce projet pour Windows à partir d'un autre système d'exploitation (comme Linux ou macOS), vous pouvez utiliser la compilation croisée. Vous aurez besoin de la cible Rust pour Windows.

1. Ajoutez la cible Windows à votre installation Rust :
    ```sh
    rustup target add x86_64-pc-windows-gnu
    ```

2. Compilez le projet pour la cible Windows :
    ```sh
    cargo build --release --target=x86_64-pc-windows-gnu
    ```

L'exécutable pour Windows se trouvera dans `target/x86_64-pc-windows-gnu/release/replacer.exe`.

### Compilation pour macOS (depuis Linux/macOS)

Pour compiler ce projet pour macOS à partir d'un autre système d'exploitation (comme Linux ou macOS), vous pouvez utiliser la compilation croisée. Vous aurez besoin de la cible Rust pour macOS.

1. Ajoutez la cible macOS à votre installation Rust (choisissez la bonne architecture) :
   * Pour les Mac Intel (x86_64) :
        ```sh
        rustup target add x86_64-apple-darwin
        ```
   * Pour les Mac Apple Silicon (aarch64) :
        ```sh
        rustup target add aarch64-apple-darwin
        ```

2. Compilez le projet pour la cible macOS (choisissez la bonne architecture) :
   * Pour les Mac Intel :
        ```sh
        cargo build --release --target=x86_64-apple-darwin
        ```
   * Pour les Mac Apple Silicon :
        ```sh
        cargo build --release --target=aarch64-apple-darwin
        ```

L'exécutable pour macOS se trouvera dans `target/<votre_cible_mac>/release/replacer`.

## Utilisation

La syntaxe de base est la suivante :

```sh
replacer [OPTIONS] --file <FILE> --pattern <PATTERN> --replacement <REPLACEMENT>
```

### Options

- `-f`, `--file <chemin_du_fichier>`: Spécifie le chemin d'accès au fichier à lire. (Obligatoire)
- `-p`, `--pattern <ancienne_chaine>`: La chaîne de caractères à rechercher et à remplacer. (Obligatoire)
- `-r`, `--replacement <nouvelle_chaine>`: La chaîne de caractères par laquelle remplacer l'ancienne chaîne. (Obligatoire)
- `--stat`: Affiche des statistiques sur le remplacement, y compris le nombre de remplacements effectués et le temps de traitement. (Optionnel)
- `-w`, `--output <chemin_du_fichier_sortie>`: Spécifie un fichier de sortie. Si cette option est utilisée, le fichier source ne sera pas modifié et le contenu remplacé sera écrit dans ce nouveau fichier. (Optionnel)
- `-e`, `--encoding <encodage>`: Spécifie l'encodage du fichier d'entrée (par exemple, `UTF-8`, `Latin-1`, `Shift_JIS`). Si cette option n'est pas spécifiée, le programme tentera de détecter automatiquement l'encodage (priorité à la BOM, puis UTF-8, puis Windows-1252 en dernier recours). (Optionnel)
- `--parallel`: Activer le traitement parallèle pour les gros fichiers (par défaut : détection automatique basée sur la taille du fichier). (Optionnel)
- `--no-cache`: Désactiver la mise en cache pour les opérations répétées (par défaut : activé pour les petits fichiers). (Optionnel)
- `--ascii-opt`: Forcer l'optimisation ASCII lorsque possible (par défaut : détection automatique). (Optionnel)
- `--buffer-size <taille>`: Taille du tampon pour les opérations d'E/S de fichiers (par défaut : 8Mo). (Optionnel)
- `--parallel-threshold <taille>`: Seuil pour le traitement parallèle (par défaut : 1Mo). (Optionnel)

### Conseils de Performance

- Utilisez `--parallel` pour les fichiers de plus de 1Mo pour activer le traitement multi-cœurs
- Activez `--ascii-opt` lorsque vous travaillez avec du texte ASCII uniquement pour un gain de 2-3x
- Ajustez `--buffer-size` en fonction de la mémoire disponible (plus grand = plus rapide mais plus d'utilisation RAM)
- Utilisez `--no-cache` pour les opérations uniques pour éviter la surcharge de cache

## Exemples

- Remplacer "Bonjour" par "Salut" dans `exemple.txt` (détection automatique de l'encodage) et afficher les statistiques:
    ```sh
    ./replacer -f exemple.txt -p "Bonjour" -r "Salut" --stat
    ```

- Remplacer toutes les occurrences de "erreur" par "succès" dans `log.txt` (encodé en Latin-1) et écrire le résultat dans `log_modifie.txt`:
    ```sh
    ./replacer -f log.txt -p "erreur" -r "succès" -w log_modifie.txt -e Latin-1
    ```

- Remplacer "pomme" par "orange" dans `fruits.txt` (détection automatique de l'encodage), écrire le résultat dans `nouveaux_fruits.txt` et afficher les statistiques:
    ```sh
    ./replacer -f fruits.txt -p "pomme" -r "orange" -w nouveaux_fruits.txt --stat
    ```

- Traiter un gros fichier avec traitement parallèle et optimisation ASCII:
    ```sh
    ./replacer -f gros_fichier.txt -p "ancien" -r "nouveau" --parallel --ascii-opt --stat
    ```

- Taille de tampon personnalisée pour les environnements contraints en mémoire:
    ```sh
    ./replacer -f fichier_enorme.txt -p "pattern" -r "remplacement" --buffer-size 4194304 --stat
    ```

## Fonctionnalités Avancées

### Sélection Automatique d'Algorithme
L'outil choisit automatiquement le meilleur algorithme en fonction de :
- Taille du fichier (parallèle vs séquentiel)
- Type de contenu (ASCII vs Unicode)
- Caractéristiques du pattern
- Mémoire système disponible

### Efficacité Mémoire
- Traitement par streaming pour les fichiers plus de 10x la taille du tampon
- Tailles de tampon configurables
- Empreinte mémoire minimale pour les petits fichiers
- Découpage sécurisé des limites UTF-8

### Traitement Multi-Cœurs
- Parallélisation automatique pour les gros fichiers
- Gestion sécurisée des limites UTF-8 en mode parallèle
- Équilibrage de charge sur les cœurs CPU
- Retour au traitement séquentiel pour les petits fichiers

## Tests

Ce projet inclut des tests unitaires complets et des benchmarks :

```sh
# Exécuter les tests unitaires
cargo test

# Exécuter les benchmarks de performance
cargo bench

# Exécuter les tests avec sortie détaillée
cargo test -- --nocapture
```

## Architecture

Le projet utilise une architecture modulaire :

```
src/
├── lib.rs              # API publique et interface principale
├── main.rs             # Point d'entrée CLI
├── core/               # Algorithmes de base
│   ├── mod.rs          # Exportations des modules
│   ├── sequential.rs   # Traitement séquentiel optimisé
│   ├── parallel.rs     # Algorithmes de traitement parallèle
│   ├── specialized.rs  # Optimisations spécialisées (ASCII, cache, multi-patterns)
│   └── config.rs       # Gestion de la configuration
└── io/                 # Opérations d'E/S
    ├── mod.rs          # Exportations des modules
    ├── buffered.rs     # Opérations de fichiers bufferisées
    └── streaming.rs    # Streaming pour les très gros fichiers
```

## Contribuer

Les contributions sont les bienvenues ! N'hésitez pas à soumettre une Pull Request. Pour les changements majeurs, veuillez ouvrir une issue d'abord pour discuter de ce que vous aimeriez modifier.

## Licence

Ce projet est sous licence MIT - voir le fichier LICENSE pour les détails.

