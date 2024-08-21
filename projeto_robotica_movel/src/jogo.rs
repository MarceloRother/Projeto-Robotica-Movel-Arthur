use std::os::windows::process;
use std::process::exit;
use std::{sync::RwLockWriteGuard, process::Command};
use crate::caixa::Caixa;
use crate::carteiro::{Carteiro, Status, Direcao};

pub struct Jogo{
    carteiro: Carteiro,
    caixa: Caixa,
    mapa: Vec<Vec<char>>,
    ult_pos_x: i32,
    ult_pos_y: i32,
    destino_x: i32,
    destino_y: i32
}

impl Jogo {
    // Construtor
    pub fn new(novo_cateiro: Carteiro, novo_caixa: Caixa, mapa: Vec<Vec<char>>, x: i32, y: i32) -> Self {
        Self {carteiro: novo_cateiro, caixa: novo_caixa, mapa: mapa, ult_pos_x: 0, ult_pos_y: 0, destino_x: x, destino_y: y}
    }

    // Funcao para limpar terminal
    pub fn limpa_terminal(&self) {
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", "cls"])
                .status()
                .unwrap();
        } else {
            // Comando para outros sistemas operacionais, como Linux e macOS
            Command::new("clear").status().unwrap();
        }
    }

    // Imprimi e atualiza mapa
    pub fn imprime_mapa(&mut self){
        // Imprime mapa
        for (i, row) in self.mapa.iter().enumerate(){
            for (j, elem) in row.iter().enumerate(){
                print!("{}", elem);
            }
            print!("\n");
        }
        println!("\nPitch: {}\t Roll: {}\n", self.carteiro.get_sensor_pitch(), self.carteiro.get_sensor_roll());
    }

    pub fn update(&mut self){
        // Update dos sensores do carteiro
        self.carteiro.update_sensor();
        if self.carteiro.get_sensor_pitch() > 1.0 || self.carteiro.get_sensor_pitch() < -1.0 ||
         self.carteiro.get_sensor_roll() > 1.0 || self.carteiro.get_sensor_roll() < -1.0 {
            println!("Ops!! O Carteiro caiu e seu jogo acabou!!\n");
            self.imprime_mapa();
            std::process::exit(0);
        }

        // Atualiza local do carteiro e 
        for (i_usize, row) in self.mapa.iter_mut().enumerate() {
            let i = i_usize as i32;
            if i == self.carteiro.get_pos_x() {
                for (j_usize, elem) in  row.iter_mut().enumerate(){
                    let j = j_usize as i32;
                    if j == self.carteiro.get_pos_y() {
                        *elem = '&';
                    }
                }
            }
            else if i == self.ult_pos_x {
                for (j_usize, elem) in  row.iter_mut().enumerate(){
                    let j = j_usize as i32;
                    if j == self.ult_pos_y {
                        *elem = '+';
                    }
                }
            }  
        }



        if self.carteiro.get_pos_x() == self.caixa.get_pos_x() && self.carteiro.get_pos_y() == self.caixa.get_pos_y() {
            self.carteiro.set_status(Status::jogando_com_caixa);
        }

        // Atualiza local da caixa / destino
        match self.carteiro.get_status() {
            Status::jogando_sem_caixa => {
                if self.carteiro.get_pos_x() == self.caixa.get_pos_x() && self.carteiro.get_pos_y() == self.caixa.get_pos_y() {
                    self.carteiro.set_status(Status::jogando_com_caixa);
                }
                else {
                    for (i_usize, row) in self.mapa.iter_mut().enumerate() {
                        let i = i_usize as i32;
                        if i == self.caixa.get_pos_x() {
                            for (j_usize, elem) in  row.iter_mut().enumerate(){
                                let j = j_usize as i32;
                                if j == self.caixa.get_pos_y() {
                                    *elem = '@';
                                }
                            }
                        }  
                    }
                }
            },

            Status::jogando_com_caixa => {
                if self.carteiro.get_pos_x() == self.destino_x && self.carteiro.get_pos_y() == self.destino_y {
                    self.carteiro.set_status(Status::fim);
                }
                else {
                    for (i_usize, row) in self.mapa.iter_mut().enumerate() {
                        let i = i_usize as i32;
                        if i == self.destino_x {
                            for (j_usize, elem) in  row.iter_mut().enumerate(){
                                let j = j_usize as i32;
                                if j == self.destino_y {
                                    *elem = 'X';
                                }
                            }
                        }  
                    } 
                }
            },
            
            Status::fim => {
                self.limpa_terminal();
                self.imprime_mapa();
                println!("\n\tViva!!\n\tVoce terminou!!\n");
                exit(0);
            }
        }
    }

    pub fn joga(&mut self){
        self.limpa_terminal();
        loop {
            self.update();
            self.imprime_mapa();
        }
    }
}