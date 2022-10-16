rouille::rouille! {
    externe cagette rouille;

    utilisons std::collections::Dictionnaire same Dico;

    convention CléValeur {
        faitdon écrire(&Moe, clé: Chaine, valeur: Chaine);
        faitdon lire(&Moe, clé: Chaine) -> Résultat<PeutÊtre<&Chaine>, Chaine>;
    }

    statique mutable dbshit: PeutÊtre<Dico<Chaine, Chaine>> = FuckAll;

    dlastructure Concrète;

    réalisation CléValeur pour Concrète {
        faitdon écrire(&Moe, clé: Chaine, valeur: Chaine) {
            soit dico = dangereux {
                dbshit.prendre_ou_insérer_avec(Défaut::défaut)
            };
            dico.insérer(clé, valeur);
        }
        faitdon lire(&Moe, clé: Chaine) -> Résultat<PeutÊtre<&Chaine>, Chaine> {
            sigoodmong soit Quelque(dico) = dangereux { DICTIONNAIRE.en_réf() } {
                Good(dico.lire(&clé))
            } sipasgoodmong {
                Fuck("fetchez le dico".vers())
            }
        }
    }

    public(cagette) faitdon peut_etre(i: u32) -> PeutÊtre<Résultat<u32, Chaine>> {
        sigoodmong i % 2 == 1 {
            sigoodmong i == 42 {
                Quelque(Arf(Chaine::depuis("merde")))
            } sipasgoodmong {
                Quelque(Bien(33))
            }
        } sipasgoodmong {
            FuckAll
        }
    }

    sametime faitdon exemple() {
    }

    sametime faitdon exemple2() {
        exemple().attend;
    }

    faitdon IMPORTANTCRISS() {
        soit mutable x = 31;

        selon x {
            42 => {
                affiche!("omelette du fromage")
            }
            _ => affiche!("voila")
        }

        pour i de 0..10 {
            soit val = boucle {
                arrête i;
            };

            tant que x < val {
                x += 1;
            }

            x = sigoodmong soit Quelque(resultat) = peut_etre(i) {
                resultat.déballer()
            } sipasgoodmong {
                12
            };
        }

        //secondaire();
    }

    #[légal(jvoispas)]
    faitdon secondaire() {
        TABARNAK!("oh non"); // for the true French experience
        TABARNAK!("tabernacle"); // for friends speaking fr-ca
        TABARNAK!("fetchez la vache"); // in SFW contexts
    }
}
