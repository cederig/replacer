# Replacer

`replacer` est un utilitaire en ligne de commande extrêmement rapide écrit en Rust qui permet de remplacer toutes les occurrences d'une chaîne de caractères donnée par une autre chaîne de caractères dans un fichier spécifié.

## Utilisation

Pour le développement et les tests rapides, vous pouvez utiliser `cargo run`:

```bash
cargo run -- -f <chemin_du_fichier> -o <ancienne_chaine> -n <nouvelle_chaine> [options]
```

Pour une utilisation en production, il est recommandé de compiler le programme et d'exécuter le binaire directement. Après avoir compilé avec `cargo build --release`, l'exécutable se trouvera dans `target/release/replacer` (ou `target/release/replacer.exe` sur Windows).

Exemple d'utilisation du binaire compilé (depuis le répertoire racine du projet):

```bash
./target/release/replacer -f <chemin_du_fichier> -o <ancienne_chaine> -n <nouvelle_chaine> [options]
```

### Options

*   `-f`, `--file <chemin_du_fichier>`: Spécifie le chemin d'accès au fichier à lire. (Obligatoire)
*   `-o`, `--old <ancienne_chaine>`: La chaîne de caractères à rechercher et à remplacer. (Obligatoire)
*   `-n`, `--new <nouvelle_chaine>`: La chaîne de caractères par laquelle remplacer l'ancienne chaîne. (Obligatoire)
*   `--stat`: Affiche des statistiques sur le remplacement, y compris le nombre de remplacements effectués et le temps de traitement. (Optionnel)
*   `-w`, `--output <chemin_du_fichier_sortie>`: Spécifie un fichier de sortie. Si cette option est utilisée, le fichier source ne sera pas modifié et le contenu remplacé sera écrit dans ce nouveau fichier. (Optionnel)
*   `-e`, `--encoding <encodage>`: Spécifie l'encodage du fichier d'entrée (par exemple, `UTF-8`, `Latin-1`, `Shift_JIS`). Si cette option n'est pas spécifiée, le programme tentera de détecter automatiquement l'encodage (priorité à la BOM, puis UTF-8, puis Windows-1252 en dernier recours). (Optionnel)

## Dépendances

Ce projet utilise les bibliothèques `clap` pour l'analyse des arguments de la ligne de commande et `encoding_rs` pour la gestion des encodages de caractères.

*   `clap = { version = "4.0", features = ["derive"] }`
*   `encoding_rs = "0.8"`

## Exemples

1.  **Remplacer "Bonjour" par "Salut" dans `exemple.txt` (détection automatique de l'encodage) et afficher les statistiques:**

    ```bash
    ./target/release/replacer -f exemple.txt -o "Bonjour" -n "Salut" --stat
    ```

2.  **Remplacer toutes les occurrences de "erreur" par "succès" dans `log.txt` (encodé en Latin-1) et écrire le résultat dans `log_modifie.txt`:**

    ```bash
    ./target/release/replacer -f log.txt -o "erreur" -n "succès" -w log_modifie.txt -e Latin-1
    ```

3.  **Remplacer "pomme" par "orange" dans `fruits.txt` (détection automatique de l'encodage), écrire le résultat dans `nouveaux_fruits.txt` et afficher les statistiques:**

    ```bash
    ./target/release/replacer -f fruits.txt -o "pomme" -n "orange" -w nouveaux_fruits.txt --stat
    ```

## Performance

`replacer` est conçu pour être extrêmement rapide. Grâce à l'efficacité de Rust, il est capable de traiter de grands fichiers et d'effectuer un nombre important de remplacements en un temps record. Par exemple, il peut remplacer 50 000 occurrences d'une chaîne de caractères en moins de 50 millisecondes sur des configurations matérielles typiques.

## Compilation et Exécution

Pour compiler et exécuter le projet, assurez-vous d'avoir Rust et Cargo installés. Ensuite, naviguez jusqu'au répertoire du projet et utilisez la commande `cargo run`:

```bash
cd replacer
cargo run -- -f <chemin_du_fichier> -o <ancienne_chaine> -n <nouvelle_chaine> [options]
```

Pour compiler une version optimisée et obtenir un exécutable autonome pour votre système d'exploitation actuel, utilisez:

```bash
cargo build --release
```

L'exécutable se trouvera dans `target/release/replacer` (ou `target/release/replacer.exe` sur Windows).

### Compilation pour Windows (depuis Linux/macOS)

Si vous êtes sur Linux ou macOS et que vous souhaitez compiler `replacer` pour Windows, vous devez d'abord ajouter la cible de compilation Windows:

```bash
rustup target add x86_64-pc-windows-gnu
# ou pour MSVC (si vous avez Visual Studio installé sur Windows)
# rustup target add x86_64-pc-windows-msvc
```

Ensuite, vous pouvez compiler le projet en spécifiant la cible:

```bash
cargo build --release --target x86_64-pc-windows-gnu
```

L'exécutable Windows se trouvera dans `target/x86_64-pc-windows-gnu/release/replacer.exe`.
