use std::io::{stdin,stdout,Write};

struct Coffre {
    code: String,
    state: i8,
    try_state: u8
}

impl Coffre {
    fn new() -> Coffre {
        Coffre {
            code: String::from(""),
            state: 0,
            try_state: 0
        }
    }

    fn open(&mut self, code_propose: &str) -> i8 {
        /*
            -1, 0 -> erreur de state (bloqué/déjà ouvert)
            3 -> code invalide
            1 -> ouverture réussie
        */

        if self.state != 1 {
           return self.state
        }

        if code_propose != self.code {
            self.try_state += 1;

            if self.try_state >= 3 {
                self.state = -1;
                return -1;
            }

            return 3;
        }

        self.state = 0;
        self.try_state = 0;

        return 1
    }

    fn close(&mut self, codeVerrou: &str) -> i8 {
        /*
            1, -1 -> Erreur de state
            0 -> Valide
        */
        if self.state != 0 {
            return self.state
        }

        self.state = 1;
        self.code = codeVerrou.parse().unwrap();

        return 0
    }

    fn getNbreEssaies(&mut self) -> u8 {
        return self.try_state;
    }

    fn reset(&mut self) {
        self.code = "".parse().unwrap();
        self.state = 0;
        self.try_state = 0
    }
}

fn askQuestion(question: &str) -> String {
    let mut string_response = String::new();
    print!("{}", question);

    let _ = stdout().flush();
    stdin().read_line(&mut string_response).expect("Did not enter a correct string");

    if let Some('\n')= string_response.chars().next_back() {
        string_response.pop();
    }
    if let Some('\r')= string_response.chars().next_back() {
        string_response.pop();
    }

    return string_response;
}

fn main() {
    let mut new_coffre: Coffre = Coffre::new();

    loop {
        let mut response = askQuestion("Que voulez-vous faire ? (Ouvrir: O, Fermer: F, Reset: R)");

        match response.as_str() {
            "O" => {
                response = askQuestion("Quel est le code pour ouvrir ?");
                let response_open: i8 = new_coffre.open(&response);

                match response_open {
                    -1=>{
                        println!("Erreur, le code est bloqué");
                    }
                    0=> {
                        println!("Erreur, le coffre est déjà ouvert")
                    }
                    3=> {
                        println!("Mauvais code ! ")
                    }
                    _=> {
                        println!("Coffre ouvert !")
                    }
                }
            }
            "F" => {
                response = askQuestion("Quel est le code pour fermer ?");
                let response_close: i8 = new_coffre.close(&response);

                match response_close {
                    -1=>{
                        println!("Erreur, le code est bloqué");
                    }
                    1=> {
                        println!("Erreur, le coffre est déjà fermé!")
                    }
                    _=> {
                        println!("Coffre fermé !")
                    }
                }
            }
            "R" => {
                new_coffre.reset();
                println!("Vous venez de reset le coffre !");
            }
            _=> {
                println!("Je n'ai pas compris votre demande...")
            }
        }
    }
}
