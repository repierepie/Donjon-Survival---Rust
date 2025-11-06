use std::io::{self, Write};
use rand::Rng;

#[derive(Debug, Clone)]
struct Joueur {
    nom: String,
    vie: i32,
    vie_max: i32,
    attaque: i32,
    defense: i32,
    potions: i32,
    or: i32,
    niveau: i32,
    experience: i32,
}

#[derive(Debug, Clone)]
struct Monstre {
    nom: String,
    vie: i32,
    attaque: i32,
    recompense_or: i32,
    recompense_xp: i32,
}

impl Joueur {
    fn new(nom: String) -> Self {
        Joueur {
            nom,
            vie: 100,
            vie_max: 100,
            attaque: 15,
            defense: 5,
            potions: 3,
            or: 50,
            niveau: 1,
            experience: 0,
        }
    }

    fn attaquer(&self, monstre: &mut Monstre) -> i32 {
        let mut rng = rand::thread_rng();
        let degats = self.attaque + rng.gen_range(0..10);
        monstre.vie -= degats;
        degats
    }

    fn utiliser_potion(&mut self) -> bool {
        if self.potions > 0 {
            self.potions -= 1;
            let soin = 40;
            self.vie = (self.vie + soin).min(self.vie_max);
            println!("💚 Vous utilisez une potion et récupérez {} PV !", soin);
            println!("❤️  Vie actuelle : {}/{}", self.vie, self.vie_max);
            true
        } else {
            println!("❌ Vous n'avez plus de potions !");
            false
        }
    }

    fn gagner_experience(&mut self, xp: i32) {
        self.experience += xp;
        println!("✨ +{} XP !", xp);
        
        let xp_requis = self.niveau * 100;
        if self.experience >= xp_requis {
            self.monter_niveau();
        }
    }

    fn monter_niveau(&mut self) {
        self.niveau += 1;
        self.experience = 0;
        self.vie_max += 20;
        self.vie = self.vie_max;
        self.attaque += 5;
        self.defense += 2;
        
        println!("\n🎉 NIVEAU SUPÉRIEUR ! Vous êtes maintenant niveau {} !", self.niveau);
        println!("📊 Statistiques améliorées :");
        println!("   ❤️  Vie max : {}", self.vie_max);
        println!("   ⚔️  Attaque : {}", self.attaque);
        println!("   🛡️  Défense : {}", self.defense);
    }

    fn afficher_stats(&self) {
        println!("\n╔══════════════════════════════════╗");
        println!("║  🧙 {} - Niveau {}", self.nom, self.niveau);
        println!("╠══════════════════════════════════╣");
        println!("║  ❤️  Vie : {}/{}", self.vie, self.vie_max);
        println!("║  ⚔️  Attaque : {}", self.attaque);
        println!("║  🛡️  Défense : {}", self.defense);
        println!("║  🧪 Potions : {}", self.potions);
        println!("║  💰 Or : {}", self.or);
        println!("║  ✨ XP : {}/{}", self.experience, self.niveau * 100);
        println!("╚══════════════════════════════════╝");
    }
}

impl Monstre {
    fn attaquer(&self, joueur: &mut Joueur) -> i32 {
        let mut rng = rand::thread_rng();
        let degats_bruts = self.attaque + rng.gen_range(0..8);
        let degats = (degats_bruts - joueur.defense).max(1);
        joueur.vie -= degats;
        degats
    }

    fn creer_monstre(profondeur: i32) -> Self {
        let mut rng = rand::thread_rng();
        let type_monstre = rng.gen_range(0..4);
        
        let (nom, vie_base, attaque_base) = match type_monstre {
            0 => ("Gobelin", 30, 8),
            1 => ("Squelette", 40, 10),
            2 => ("Orc", 50, 12),
            _ => ("Dragon", 80, 15),
        };

        let multiplicateur = 1.0 + (profondeur as f32 * 0.2);
        
        Monstre {
            nom: nom.to_string(),
            vie: (vie_base as f32 * multiplicateur) as i32,
            attaque: (attaque_base as f32 * multiplicateur) as i32,
            recompense_or: 20 + (profondeur * 10),
            recompense_xp: 30 + (profondeur * 15),
        }
    }
}

fn combat(joueur: &mut Joueur, monstre: &mut Monstre) -> bool {
    println!("\n⚔️  ═══════════════════════════════════");
    println!("   🐉 Un {} sauvage apparaît !", monstre.nom);
    println!("   ❤️  Vie : {} | ⚔️  Attaque : {}", monstre.vie, monstre.attaque);
    println!("   ═══════════════════════════════════\n");

    while joueur.vie > 0 && monstre.vie > 0 {
        println!("┌─────────────────────────────────┐");
        println!("│ Que voulez-vous faire ?         │");
        println!("│ 1. ⚔️  Attaquer                  │");
        println!("│ 2. 🧪 Utiliser une potion       │");
        println!("│ 3. 🏃 Fuir                       │");
        println!("└─────────────────────────────────┘");

        let choix = lire_choix();

        match choix {
            1 => {
                let degats = joueur.attaquer(monstre);
                println!("⚔️  Vous infligez {} dégâts au {} !", degats, monstre.nom);
                
                if monstre.vie > 0 {
                    println!("   {} vie restante : {}", monstre.nom, monstre.vie);
                } else {
                    println!("🎉 Vous avez vaincu le {} !", monstre.nom);
                    joueur.or += monstre.recompense_or;
                    println!("💰 +{} or", monstre.recompense_or);
                    joueur.gagner_experience(monstre.recompense_xp);
                    return true;
                }
            }
            2 => {
                joueur.utiliser_potion();
            }
            3 => {
                println!("🏃 Vous fuyez le combat !");
                return false;
            }
            _ => {
                println!("❌ Choix invalide !");
                continue;
            }
        }

        if monstre.vie > 0 {
            let degats = monstre.attaquer(joueur);
            println!("💥 Le {} vous inflige {} dégâts !", monstre.nom, degats);
            println!("❤️  Votre vie : {}/{}\n", joueur.vie, joueur.vie_max);
        }

        if joueur.vie <= 0 {
            println!("💀 Vous êtes mort...");
            return false;
        }
    }

    true
}

fn boutique(joueur: &mut Joueur) {
    loop {
        println!("\n🏪 ═══════════════════════════════════");
        println!("   Bienvenue à la boutique !");
        println!("   💰 Votre or : {}", joueur.or);
        println!("   ═══════════════════════════════════");
        println!("\n1. 🧪 Potion de vie (30 or)");
        println!("2. ⚔️  Améliorer attaque (50 or)");
        println!("3. 🛡️  Améliorer défense (50 or)");
        println!("4. 🚪 Quitter la boutique");

        let choix = lire_choix();

        match choix {
            1 => {
                if joueur.or >= 30 {
                    joueur.or -= 30;
                    joueur.potions += 1;
                    println!("✅ Vous avez acheté une potion !");
                } else {
                    println!("❌ Or insuffisant !");
                }
            }
            2 => {
                if joueur.or >= 50 {
                    joueur.or -= 50;
                    joueur.attaque += 3;
                    println!("✅ Attaque améliorée ! Nouvelle attaque : {}", joueur.attaque);
                } else {
                    println!("❌ Or insuffisant !");
                }
            }
            3 => {
                if joueur.or >= 50 {
                    joueur.or -= 50;
                    joueur.defense += 2;
                    println!("✅ Défense améliorée ! Nouvelle défense : {}", joueur.defense);
                } else {
                    println!("❌ Or insuffisant !");
                }
            }
            4 => break,
            _ => println!("❌ Choix invalide !"),
        }
    }
}

fn lire_choix() -> i32 {
    print!("\n➤ Votre choix : ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap_or(0)
}

fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║   🗡️  DONJON SURVIVAL 🗡️             ║");
    println!("║   Survivez aux profondeurs !           ║");
    println!("╚════════════════════════════════════════╝\n");

    print!("Entrez votre nom : ");
    io::stdout().flush().unwrap();
    
    let mut nom = String::new();
    io::stdin().read_line(&mut nom).unwrap();
    let nom = nom.trim().to_string();

    let mut joueur = Joueur::new(nom);
    let mut profondeur = 1;

    println!("\n🎮 Bienvenue {} ! Votre aventure commence...\n", joueur.nom);

    loop {
        joueur.afficher_stats();
        
        println!("\n🏰 Profondeur du donjon : Niveau {}", profondeur);
        println!("\n┌─────────────────────────────────┐");
        println!("│ 1. ⚔️  Explorer plus profond     │");
        println!("│ 2. 🏪 Visiter la boutique       │");
        println!("│ 3. 🚪 Quitter le donjon          │");
        println!("└─────────────────────────────────┘");

        let choix = lire_choix();

        match choix {
            1 => {
                let mut monstre = Monstre::creer_monstre(profondeur);
                if combat(&mut joueur, &mut monstre) {
                    profondeur += 1;
                    println!("\n✨ Vous descendez au niveau {} du donjon...", profondeur);
                }
                
                if joueur.vie <= 0 {
                    println!("\n💀 ═══════════════════════════════════");
                    println!("   GAME OVER");
                    println!("   Profondeur atteinte : Niveau {}", profondeur);
                    println!("   ═══════════════════════════════════");
                    break;
                }
            }
            2 => boutique(&mut joueur),
            3 => {
                println!("\n👋 Vous quittez le donjon. À bientôt !");
                break;
            }
            _ => println!("❌ Choix invalide !"),
        }
    }
}
