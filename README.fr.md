# Replacer

`replacer` est un utilitaire en ligne de commande extrêmement rapide écrit en Rust qui permet de remplacer toutes les occurrences d'une chaîne de caractères donnée par une autre chaîne de caractères dans un fichier spécifié.

## Dépendances

Ce projet utilise les dépendances suivantes (telles que définies dans `Cargo.toml`) :

-   `clap` (version `4.5.41`) : Pour l'analyse des arguments de la ligne de commande.
-   `indicatif` (version `0.18.0`) : Pour afficher une barre de progression.
-   `encoding_rs` (version `0.8.35`) : Pour la gestion des encodages de texte.

## Installation

### Prérequis

Assurez-vous d'avoir Rust et Cargo d'installés sur votre système. Vous pouvez les installer en suivant les instructions sur le site officiel de Rust : [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### Compilation pour Linux (depuis Linux/macOS)
1.  Clonez ce dépôt :
    ```sh
    git clone https://github.com/cederig/replacer.git
    cd replacer
    ```
2.  Compilez le projet :
    ```sh
    cargo build --release
    ```
    L'exécutable se trouvera dans `target/release/replacer`.

### Compilation pour Windows (depuis Linux/macOS)

Pour compiler ce projet pour Windows à partir d'un autre système d'exploitation (comme Linux ou macOS), vous pouvez utiliser la compilation croisée. Vous aurez besoin de la cible Rust pour Windows.

1.  Ajoutez la cible Windows à votre installation Rust :
    ```sh
    rustup target add x86_64-pc-windows-gnu
    ```

2.  Compilez le projet pour la cible Windows :
    ```sh
    cargo build --release --target=x86_64-pc-windows-gnu
    ```

L'exécutable pour Windows se trouvera dans `target/x86_64-pc-windows-gnu/release/replacer.exe`.

### Compilation pour macOS (depuis Linux/macOS)

Pour compiler ce projet pour macOS à partir d'un autre système d'exploitation (comme Linux ou macOS), vous pouvez utiliser la compilation croisée. Vous aurez besoin de la cible Rust pour macOS.

1.  Ajoutez la cible macOS à votre installation Rust (choisissez la bonne architecture) :
    *   Pour les Mac Intel (x86_64) :
        ```sh
        rustup target add x86_64-apple-darwin
        ```
    *   Pour les Mac Apple Silicon (aarch64) :
        ```sh
        rustup target add aarch64-apple-darwin
        ```

2.  Compilez le projet pour la cible macOS (choisissez la bonne architecture) :
    *   Pour les Mac Intel :
        ```sh
        cargo build --release --target=x86_64-apple-darwin
        ```
    *   Pour les Mac Apple Silicon :
        ```sh
        cargo build --release --target=aarch64-apple-darwin
        ```

L'exécutable pour macOS se trouvera dans `target/<votre_cible_mac>/release/replacer` (par exemple, `target/x86_64-apple-darwin/release/replacer`).

## Utilisation

La syntaxe de base est la suivante :

```bash
./replacer [OPTIONS] --file <FILE> --old <OLD> --new <NEW>
```

### Options

*   `-f`, `--file <chemin_du_fichier>`: Spécifie le chemin d'accès au fichier à lire. (Obligatoire)
*   `-o`, `--old <ancienne_chaine>`: La chaîne de caractères à rechercher et à remplacer. (Obligatoire)
*   `-n`, `--new <nouvelle_chaine>`: La chaîne de caractères par laquelle remplacer l'ancienne chaîne. (Obligatoire)
*   `--stat`: Affiche des statistiques sur le remplacement, y compris le nombre de remplacements effectués et le temps de traitement. (Optionnel)
*   `-w`, `--output <chemin_du_fichier_sortie>`: Spécifie un fichier de sortie. Si cette option est utilisée, le fichier source ne sera pas modifié et le contenu remplacé sera écrit dans ce nouveau fichier. (Optionnel)
*   `-e`, `--encoding <encodage>`: Spécifie l'encodage du fichier d'entrée (par exemple, `UTF-8`, `Latin-1`, `Shift_JIS`). Si cette option n'est pas spécifiée, le programme tentera de détecter automatiquement l'encodage (priorité à la BOM, puis UTF-8, puis Windows-1252 en dernier recours). (Optionnel)



### Exemples

1.  Remplacer "Bonjour" par "Salut" dans `exemple.txt` (détection automatique de l'encodage) et afficher les statistiques:

    ```bash
    ./replacer -f exemple.txt -o "Bonjour" -n "Salut" --stat
    ```

2.  Remplacer toutes les occurrences de "erreur" par "succès" dans `log.txt` (encodé en Latin-1) et écrire le résultat dans `log_modifie.txt`:

    ```bash
    ./replacer -f log.txt -o "erreur" -n "succès" -w log_modifie.txt -e Latin-1
    ```

3.  Remplacer "pomme" par "orange" dans `fruits.txt` (détection automatique de l'encodage), écrire le résultat dans `nouveaux_fruits.txt` et afficher les statistiques:

    ```bash
    ./replacer -f fruits.txt -o "pomme" -n "orange" -w nouveaux_fruits.txt --stat
    ```

## Performance

`replacer` est conçu pour être extrêmement rapide. Grâce à l'efficacité de Rust, il est capable de traiter de grands fichiers et d'effectuer un nombre important de remplacements en un temps record. Par exemple, il peut remplacer 50 000 occurrences d'une chaîne de caractères en moins de 50 millisecondes sur des configurations matérielles typiques.

## Tests

Ce projet inclut des tests unitaires; pour les exécuter, utilisez la commande suivante à la racine du projet :

```bash
cargo test
```

Cette commande compile le programme en mode test et exécute toutes les fonctions de test.

