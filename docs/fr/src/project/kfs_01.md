# KFS 01 - Grub, boot & screen

## Introduction

Notre premier projet `KFS` ! J'étais moi-même assez nerveux au début en regardant cela. Mon expérience précédente avec les noyaux se limitait à Linux from Scratch, qui effleure à peine la surface comparé à KFS.

Pour ce projet, j'ai choisi Rust comme langage de programmation. Mon expérience est limitée. Je viens de terminer le tutoriel appelé [Rustlings](https://github.com/rust-lang/rustlings/). J'ai quelques années d'expérience en C/C++ & Assembly, ce qui sera définitivement utile.

## Objectifs

- Un noyau qui peut démarrer via Grub
- Une base démarrable en `ASM`
- Une bibliothèque de noyau basique
- Afficher "42" à l'écran
Simple, n'est-ce pas ? (_Ce ne l'était pas_)

## Approche Technique & Implémentation

Mon approche était assez directe pour ce projet. Lire, lire & LIRE ! J'ai principalement commencé par lire [OSDev](https://wiki.osdev.org/Expanded_Main_Page). Il offre de bons conseils sur le développement de noyaux.

J'ai commencé à suivre le tutoriel direct d'OSDev pour le démarrage d'OS en C. Avoir ma propre implémentation de `libc` a rendu cette phase assez fluide. J'ai pu facilement rendre un système démarrable.

Après cela, j'avais quelques connaissances de base sur le démarrage d'un système via GRUB. Le défi était maintenant de le convertir en Rust. Heureusement, le [blog de Philipp Oppermann](https://os.phil-opp.com/) m'a énormément aidé ! Il m'a donné plus d'aperçus sur la configuration d'un environnement Rust. Je devais juste comprendre comment changer cela pour `x86_32`, puisque leur tutoriel est destiné à `x86_64`.

Après cela, j'ai remarqué que M. Oppermann avait un second tutoriel sur VGA ; comment le configurer et y imprimer, ce qui est l'une des exigences. Je l'ai terminé et pratiquement fini `KFS_01`. Je devais juste mettre les points sur les i et barrer les t.

## Défis

Le plus grand défi de ce projet était de comprendre le `nix-shell`, le système de ciblage de Rust & le démarrage du noyau Rust en `x86_32`.

Vous avez peut-être remarqué que j'utilise `nix-shell`. La raison est simplement de faciliter le démarrage d'un développeur dans le bon environnement. Une fois que `nix-shell` est configuré, il garantit que vous êtes toujours sur la bonne version avec les bons programmes. Vous aurez moins de _"Ça marche sur ma machine"_. Le principal défi était de configurer le `nix-shell`, car la documentation de Nix est assez limitée. C'était juste essayer beaucoup de choses jusqu'à ce que ça marche.

Deuxièmement, le système de ciblage de Rust était assez vague pour moi. Le principal défi pour moi était de comprendre qu'il fallait un `target.json` pour vos propres spécifications. Je pense que c'est la meilleure approche, mais j'étais habitué à `gcc`, où vous devez le compiler vous-même. Il m'a fallu un peu de temps pour comprendre qu'il ne faut pas compiler `rustc` depuis zéro, mais lui donner un `target.json` pour donner votre code bare-metal.

Enfin, démarrer en 32 bits était vraiment pénible. À la fin, je ne suis toujours pas sûr de ce qui n'allait pas entièrement. Le code Rust lui-même fonctionnait, mais il y avait quelque chose qui n'allait pas avec mon `boot.asm` & `linker.ld`. Ce n'était pas correctement configuré par le Linker pour faire savoir au BIOS où trouver mon `kernel_main()`. À la fin, j'ai juste dû changer quelque chose dans mon `boot.asm`. Ce qui a fonctionné.

## Conclusion & Leçons Apprises

Au final, cela s'est passé beaucoup plus facilement que prévu. Il y avait beaucoup de tutoriels et de documentation compréhensible pour me guider à travers le premier projet.

La leçon que j'ai apprise était de ne pas supposer la même approche pour chaque compilateur. Chaque langage de programmation a une approche différente. Je suis toujours content de mon choix d'utiliser `nix-shell`. Cela évitera définitivement des maux de tête à l'avenir.

---
_Note : Cette traduction a été réalisée par une intelligence artificielle et peut contenir des erreurs._
