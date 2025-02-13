# Global Description Table (GDT)

## Qu'est-ce que c'est

La GDT sert de structure de données fondamentale dans l'architecture x86, jouant un rôle crucial dans la gestion et la protection de la mémoire. Quand notre ordinateur démarre, il commence en `mode réel`, un mode de fonctionnement simple qui fournit un accès direct à la mémoire et aux périphériques d'E/S. Cependant, nous devons passer en `mode protégé`, qui introduit la protection mémoire, la mémoire virtuelle et les niveaux de privilège.

Imaginez le `mode protégé` comme l'établissement de différents niveaux d'habilitation de sécurité dans un bâtiment. La GDT agit comme le système de sécurité qui définit qui peut accéder à quoi. Bien que ma comparaison précédente avec `sudo` capture l'idée de base des niveaux de privilège, la réalité est plus sophistiquée. Au lieu de simplement "admin" et "utilisateur", l'architecture x86 fournit quatre anneaux (0-3), où l'anneau 0 est le plus privilégié (espace noyau) et l'anneau 3 est le moins privilégié (espace utilisateur). Chaque anneau possède des permissions et des restrictions spécifiques, toutes définies dans notre GDT.

La GDT est essentielle non seulement pour la sécurité, mais aussi pour le fonctionnement de base du `mode protégé`. Sans une GDT correctement configurée, le CPU ne peut pas du tout exécuter de code en `mode protégé`.

Pour plus d'informations, consultez [OSDev](https://wiki.osdev.org/Global_Descriptor_Table)

## Mon Approche Technique

Mon approche était la suivante : je commencerais par le `boot.asm` et configurerais le multiboot. Celui-ci appellera ensuite `gdt_init`, qui est une fonction Rust. `gdt_init` configurera les `GDT_Entries` et s'assurera de créer le pointeur de structure correct qui sera passé à `gdt.asm`. `gdt.asm` placera les entrées dans les registres appropriés.

La configuration multiboot est cruciale car elle garantit que notre noyau est chargé correctement par le chargeur de démarrage et respecte la Spécification Multiboot, qui est une manière standardisée pour les chargeurs de démarrage de charger les systèmes d'exploitation.

Voici quelques extraits pour vous donner une meilleure idée :

```nasm
; Les deux fonctions Rust
extern gdt_init
extern kernel_main

_start:
    ; Le chargeur de démarrage nous a chargés en mode protégé 32 bits
    ; mais nous devons configurer notre propre GDT pour une segmentation appropriée
    call   gdt_init
    call   kernel_main
```

```rust
#[no_mangle] // Assure que rustc ne modifie pas le nom du symbole pour la liaison externe
pub fn gdt_init() {
    // Crée la structure descripteur GDT
    // size est (total_size - 1) car le champ limit est l'unité maximale adressable
    let gdt_descriptor = GDTDescriptor { 
        size: (size_of::<GdtGates>() - 1) as u16,  // La taille doit être inférieure d'une unité à la taille réelle
        offset: 0x00000800,  // Place la GDT à l'adresse spécifiée en mémoire
    }; 
    // Appelle la fonction assembly pour charger le registre GDT (GDTR)
    gdt_flush(&gdt_descriptor as *const _);
}
```

Voici comment nos entrées GDT sont structurées :

```rust
// Chaque entrée GDT fait 64 bits (8 octets)
pub struct Gate(pub u64);  

#[no_mangle]
#[link_section = ".gdt"]  // Place dans une section GDT spéciale pour la liaison
pub static GDT_ENTRIES: GdtGates = [
    // Descripteur nul - Requis par la spécification CPU pour la vérification d'erreurs
    Gate(0),
    // Segment de Code Noyau : Anneau 0, exécutable, non-conforming
    // Paramètres : base=0, limit=max, access=0b10011010 (présent, anneau 0, code), flags=0b1100 (32-bit, granularité 4KB)
    Gate::new(0, !0, 0b10011010, 0b1100),  
    // Segment de Données Noyau : Anneau 0, inscriptible, grow-up
    // Paramètres : base=0, limit=max, access=0b10010010 (présent, anneau 0, données), flags=0b1100 (32-bit, granularité 4KB)
    Gate::new(0, !0, 0b10010010, 0b1100),  
    // Segment de Code Utilisateur : Anneau 3, exécutable, non-conforming
    // Paramètres : base=0, limit=max, access=0b11111010 (présent, anneau 3, code), flags=0b1100 (32-bit, granularité 4KB)
    Gate::new(0, !0, 0b11111010, 0b1100),  
    // Segment de Données Utilisateur : Anneau 3, inscriptible, grow-up
    // Paramètres : base=0, limit=max, access=0b11110010 (présent, anneau 3, données), flags=0b1100 (32-bit, granularité 4KB)
    Gate::new(0, !0, 0b11110010, 0b1100),  
];
```

Le code assembly dans `gdt.asm` qui charge effectivement la GDT :

```nasm
gdt_flush:
    ; Charge l'adresse de la structure descripteur GDT depuis la pile
    mov  eax, [esp + 4]
    lgdt [eax]

    ; Active le mode protégé en définissant le premier bit de CR0
    mov eax, cr0
    or  eax, 1
    mov cr0, eax

    ; Configure les registres de segment avec les sélecteurs appropriés
    ; 0x10 pointe vers le segment de données noyau (troisième entrée GDT)
    mov eax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    ; Saut lointain pour vider le pipeline et charger CS avec le sélecteur de code noyau (0x08)
    ; C'est nécessaire pour entrer complètement en mode protégé
    jmp 0x08:.flush

.flush:
    ret
```

Je n'entre pas dans les détails sur pourquoi des choses spécifiques se produisent, mais chaque étape est cruciale pour initialiser correctement le mode protégé.

## Considérations

J'ai envisagé d'utiliser l'assembly en ligne, mais je ne l'ai finalement pas fait car il avait quelques bugs connus dans Rust. Je me sentais beaucoup plus à l'aise en utilisant de vrais fichiers `asm`, plutôt que de le faire en ligne. L'assembly en ligne dans Rust peut être particulièrement problématique lors de la manipulation de fonctionnalités CPU de bas niveau, car les hypothèses du compilateur sur l'utilisation des registres et les conventions d'appel pourraient entrer en conflit avec ce dont nous avons besoin pour la configuration de la GDT.

### Utiliser C ou Assembly au lieu de Rust

Comme C est un programme beaucoup plus ancien et mieux documenté que Rust, j'ai envisagé d'utiliser C pour le `gdt_init`, au lieu de Rust. Je ne l'ai finalement pas fait, car je voulais rester fidèle à mon noyau Rust "pur" et je sentais que cela compliquerait beaucoup plus les choses. L'utilisation de C aurait nécessité une complexité supplémentaire dans le système de build pour gérer plusieurs langages et leur interaction.

## Éditeur de Liens

Force l'Éditeur de Liens à mettre la GDT avant le reste du code. Vous pouvez le faire sans cela, mais vous aurez des problèmes si vous souhaitez la placer à une adresse spécifique. Si ce n'est pas nécessaire pour vous, vous pouvez ignorer cela. Le placement spécifique de la GDT en mémoire peut être important pour certaines conceptions de système, particulièrement lors de la gestion de la mémoire et de la configuration de la mémoire virtuelle. Voici mon approche :

```
  /* Commence à 2MB */
  . = 2M;


  .gdt 0x800 : ALIGN(0x800)
    {
    gdt_start = .;
    *(.gdt)
    gdt_end = .;
  }
```

### Considérations sur la Disposition de la Mémoire

Lors du placement de la GDT à une adresse spécifique, il est important de s'assurer que :

1. L'adresse est accessible pendant la transition vers le mode protégé
2. L'adresse n'entre pas en conflit avec d'autres structures système importantes
3. L'adresse est correctement alignée pour des performances optimales

---
_Note : Cette traduction a été réalisée par une intelligence artificielle et peut contenir des erreurs._
