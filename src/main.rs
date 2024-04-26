use std::io;

fn main() {
  
    // message d'accueil
    println!("Bienvenue sur votre compte bancaire");

    // Initialiser compte
    let mut balance = 1000.0;

    println!("Votre solde est de : {} €", balance);
    
    // depot sur le compte
    let mut input = String::new();
    println!("Saisissez un montant à ajouter au compte:");
    
    io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de l'entrée");

    let deposit: f64 = input.trim().parse().expect("Veuillez saisir un nombre valide");
    balance += deposit;

    println!("Votre solde est maitenant de : {} €", balance);

    // Retrait sur le compte
    let mut input: String = String::new();

    println!("Saisissez le montant à retirer :");

    io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de l'entrée");

    let withdrawal: f64 = input.trim().parse().expect("Veuillez saisir un nombre valide");

    if balance - withdrawal < 0.0 {
        
    println!("Vous ne pouvez pas retirer plus que votre solde actuel");

    } else if withdrawal < 0.0 {
        println!("Vous avez essayé de hack le système");
    }
    
    else {
         balance -= withdrawal;

    }


    // Affichage du solde par emprunt/reference
    let borrowed_balance = &balance;

    println!("Votre solde avant frais de gestion est de : {} €", borrowed_balance);

    // Application des frais de gestion, affichage via shadowing pour le solde final
    let withdrawal_fees = 10.0;

    let balance = balance - withdrawal_fees;


    println!("Votre solde total est de {} €", balance);

}
