use crate::sensor::Sensor;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direcao{
    norte,
    sul,
    leste,
    oeste
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Status{
    jogando_sem_caixa,
    jogando_com_caixa,
    fim
}

pub struct Carteiro{
    pos_x: i32,
    pos_y: i32,
    status: Status,
    sensor: Sensor,
    direcao: Direcao

}

impl Carteiro{
    // Construtor
    pub fn new(x: i32, y: i32) -> Self {
        Self { pos_x: x, pos_y: y, status: Status::jogando_sem_caixa, sensor: Sensor::new(), direcao: Direcao::norte}
    }

    // Metodos de get para os atributos
    pub fn get_pos_x(&self) -> i32{
        self.pos_x
    }

    pub fn get_pos_y(&self) -> i32{
        self.pos_y
    }

    pub fn get_sensor_pitch(&self) -> f64 {
        self.sensor.get_pitch()
    }

    pub fn get_sensor_roll(&self) -> f64 {
        self.sensor.get_roll()
    }

    pub fn get_status(&self) -> Status {
        self.status
    }

    pub fn get_direcao(&self) -> Direcao {
        self.direcao
    }

    pub fn set_status(&mut self, novo_status: Status) {
        self.status = novo_status;
    }

    pub fn muda_direcao(&mut self, nova_direcao: Direcao) {
        self.direcao = nova_direcao;
    }

    // Uptdate OBRIGATORIO do sensor
    pub fn update_sensor(&mut self) {
        self.sensor.update();
    }

    // Verifica a possibilidade do robo se locomover para tal direcao
    pub fn verifica_andar(&self) -> bool {
        match self.get_direcao() {
            Direcao::norte => {
                if self.get_pos_y() > 19 {
                    false
                }
                else {
                    true
                }
            }
            Direcao::sul => {
                if self.get_pos_y() < 0 {
                    false
                }
                else {
                    true
                }
            }
            Direcao::leste => {
                if self.get_pos_x() > 19 {
                    false
                }
                else {
                    true
                }
            }
            Direcao::oeste => {
                if self.get_pos_x() < 0 {
                    false
                }
                else {
                    true
                }
            }
        }
    }

    // Utiliza a funcao verifica_andar() para saber a possibilidade de locomocao do robo.
    // Se verdadeiro, entao move o robo sem retorno. 
    // Se falso, retorna falso sem mover o robo
    pub fn andar(&mut self) -> Option<bool>{
        match self.get_direcao() {
            Direcao::norte => { 
                if self.get_pos_y() > 19 {
                    Some(false)
                }
                else {
                    self.pos_x += 1;
                    None
                }
            },
            Direcao::sul => { 
                if self.get_pos_y() < 0 {
                    Some(false)
                }
                else {
                    self.pos_x -= 1;
                    None
                }
            },
            Direcao::leste => { 
                if self.get_pos_x() > 19 {
                    Some(false)
                }
                else {
                    self.pos_y += 1;
                    None
                }
            },
            Direcao::oeste => { 
                if self.get_pos_x() < 0 {
                    Some(false)
                }
                else {
                    self.pos_y -= 1;
                    None
                }
            }
        }
    }
}