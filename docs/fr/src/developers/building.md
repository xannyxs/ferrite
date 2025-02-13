# Construction depuis les Sources

Vous aurez besoin de :

- `nix-shell` pour un environnement de développement isolé
- QEMU pour les tests

```bash
# Cloner le dépôt
git clone https://github.com/xannyxs/fungul
cd fungul
# Lancer nix-shell
nix-shell shell.nix --command "zsh"
# Construire le noyau
make
# Exécuter dans QEMU
make run
```

---
_Note : Cette traduction a été réalisée par une intelligence artificielle et peut contenir des erreurs._
