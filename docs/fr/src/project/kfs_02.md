# KFS 02 - GDT & Stack

## Introduction

Passons au second projet `KFS`. Le premier était faisable et je me sentais confiant pour faire le deuxième.

Pour ce projet, nous devions implémenter une GDT (Global Descriptor Table). La GDT sert de structure de données fondamentale dans l'architecture x86, jouant un rôle crucial dans la gestion et la protection de la mémoire. Quand notre ordinateur démarre, il commence en `mode réel`, un mode de fonctionnement simple qui fournit un accès direct à la mémoire et aux périphériques d'E/S. Cependant, nous devons passer en mode protégé, qui introduit la protection mémoire, la mémoire virtuelle et les niveaux de privilège.

Pensez au `mode protégé` comme à l'établissement de différents niveaux d'habilitation de sécurité dans un bâtiment. La GDT agit comme le système de sécurité qui définit qui peut accéder à quoi. Bien que ma comparaison précédente avec `sudo` capture l'idée de base des niveaux de privilège, la réalité est plus sophistiquée. Au lieu de simplement "admin" et "utilisateur", l'architecture x86 fournit quatre anneaux (0-3), où l'anneau 0 est le plus privilégié (espace noyau) et l'anneau 3 est le moins privilégié (espace utilisateur). Chaque anneau a des permissions et des restrictions spécifiques, toutes définies dans notre GDT.

La GDT est essentielle non seulement pour la sécurité, mais aussi pour le fonctionnement de base du `mode protégé`. Sans une GDT correctement configurée, le CPU ne peut pas du tout exécuter de code en `mode protégé`.

## Objectifs

Le projet nécessite la création d'une GDT à 0x00000800 avec des entrées pour l'Espace de Données Noyau, l'Espace de Code Noyau, l'Espace de Données Utilisateur et l'Espace de Code Utilisateur. De plus, nous devons ajouter un support minimal du clavier PS/2 et implémenter un shell basique avec les commandes `reboot` & `gdt`. La commande `gdt` affichera les entrées de la GDT de manière lisible par l'humain.

## Approche Technique & Implémentation

Mon voyage a commencé par l'étude de la documentation [OSDev](https://wiki.osdev.org/Global_Descriptor_Table). Les concepts étaient initialement écrasants - des termes comme descripteurs de segment, niveaux de privilège et drapeaux de descripteur ressemblaient à l'apprentissage d'une nouvelle langue. Après avoir regardé plusieurs tutoriels YouTube ([ici](https://www.youtube.com/watch?v=GvIJYELuaaE&t=5615s) & [ici](https://www.youtube.com/watch?v=Wh5nPn2U_1w&t=429s)) sur l'implémentation de la GDT en Rust, les choses ont commencé à devenir plus claires.

J'ai fait face à un choix : implémenter la GDT en Assembly ou en Rust. Bien que l'Assembly donnerait plus de contrôle direct, j'ai choisi Rust pour ses fonctionnalités de sécurité et ma familiarité croissante avec celui-ci. Voici comment j'ai structuré l'implémentation :

Le processus de démarrage commence dans boot.asm, où nous configurons les drapeaux multiboot et préparons la transition vers le mode protégé. Ensuite, nous appelons `gdt_init`, une fonction Rust qui configure notre GDT :

```rust
#[no_mangle] // Assure que rustc ne modifie pas le nom du symbole pour la liaison externe
pub fn gdt_init() {
    // Crée la structure descripteur GDT
    // size est (total_size - 1) car le champ limit est l'unité maximale adressable
    let gdt_descriptor = GDTDescriptor { 
        size: (size_of::<GdtGates>() - 1) as u16,
        offset: 0x00000800,  // Place la GDT à l'adresse spécifiée
    }; 
    // Appelle la fonction assembly pour charger le registre GDT (GDTR)
    gdt_flush(&gdt_descriptor as *const _);
}
```

Voici comment nos entrées GDT sont structurées :

```rust
pub struct Gate(pub u64);  // Chaque entrée GDT fait 64 bits

#[no_mangle]
#[link_section = ".gdt"]  // Place dans une section GDT spéciale pour la liaison
pub static GDT_ENTRIES: GdtGates = [
    // Descripteur nul - Requis par la spécification CPU
    Gate(0),
    // Segment de Code Noyau : Anneau 0, exécutable, non-conforming
    Gate::new(0, !0, 0b10011010, 0b1100),  
    // Segment de Données Noyau : Anneau 0, inscriptible, grow-up
    Gate::new(0, !0, 0b10010010, 0b1100),  
    // Segment de Code Utilisateur : Anneau 3, exécutable, non-conforming
    Gate::new(0, !0, 0b11111010, 0b1100),  
    // Segment de Données Utilisateur : Anneau 3, inscriptible, grow-up
    Gate::new(0, !0, 0b11110010, 0b1100),  
];
```

Chaque appel Gate::new() prend quatre paramètres :

- base : L'adresse de début du segment (0 pour le modèle de mémoire plat)
- limit : L'unité maximale adressable (!0 signifie utiliser tout l'espace d'adressage)
- access : Définit les privilèges et le type du segment (expliqué en détail dans le tableau ci-dessous)
- flags : Contrôle la granularité et la taille (0b1100 pour le mode protégé 32-bit)

Après avoir configuré la GDT, j'ai implémenté un support basique du clavier. Bien que mon approche actuelle de polling ne soit pas idéale (elle vérifie continuellement les frappes de touches), elle fonctionne pour notre shell basique. Une implémentation appropriée utiliserait des interruptions pour gérer les événements clavier, mais c'est un sujet pour les projets futurs. Le pilote VGA de KFS_01 a été adapté pour créer une interface shell simple, permettant les commandes `reboot` et `gdt`.

Le système subissait encore des triple fautes initialement. La solution se trouvait dans le script de liaison - en utilisant `#[link_section = ".gdt"]`, j'ai assuré que notre GDT était placée à la bonne adresse mémoire. L'ordre est crucial : code de démarrage BIOS, puis GDT, puis le reste de notre noyau.

```
  /* Commence à 2MB */
  . = 2M;


  .gdt 0x800 : ALIGN(0x800)
    {
    gdt_start = .;
    *(.gdt)
    gdt_end = .;
  }

  /* Le reste... */
```

## Défis

Les défis étaient principalement la compréhension de la GDT. J'ai eu du mal à saisir son but et son fonctionnement exact. J'ai dû lire plusieurs articles et regarder plusieurs vidéos pour comprendre finalement ce qu'elle est censée faire.

Je n'avais aussi aucune expérience réelle avec le linker. Trouver la source de la triple faute était particulièrement frustrant, et il m'a fallu un certain temps avant de réaliser que le linker pourrait ne pas placer la GDT à la bonne adresse.

## Conclusion & Leçon Apprise

J'ai découvert que j'avais besoin de relire les documents plusieurs fois pour bien comprendre les concepts. Heureusement, il y avait beaucoup de documentation disponible sur la GDT et son implémentation. Travailler avec la GDT m'a motivé à tout documenter de manière approfondie, comme ces pages. Je fais cela principalement pour m'assurer que je comprends vraiment la fonctionnalité de chaque composant avec lequel je travaille.

---
*Note : Cette traduction a été réalisée par une intelligence artificielle et peut contenir des erreurs.*
