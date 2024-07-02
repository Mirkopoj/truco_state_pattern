use super::{
    envido::Envido, envido_va_primero::EnvidoVaPrimero, falta_envido::FaltaEnvido, nada::Nada,
    r#final::Final, real_envido::RealEnvido, Envidos, Players, TrucoState, Trucos,
};

#[derive(Debug, Clone)]
pub struct Inicial {
    players: Players,
}

impl Inicial {
    pub fn new(players: Vec<String>) -> Self {
        Self {
            players: Players::new(players),
        }
    }
}

impl TrucoState for Inicial {
    fn irse_al_maso(self: Box<Self>) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Ok(Box::new(Final::new(Envidos::Value(0), Trucos::Simple)))
    }

    fn cantar_quiero(self: Box<Self>, _: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Err(self)
    }

    fn cantar_no_quiero(self: Box<Self>, _: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Err(self)
    }

    fn cantar_envido(self: Box<Self>, player: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        if self.players.is_turn(player) {
            Ok(Box::new(Envido::new(self.players)))
        } else {
            Err(self)
        }
    }

    fn cantar_real_envido(self: Box<Self>, player: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        if self.players.is_turn(player) {
            Ok(Box::new(RealEnvido::new(Envidos::Value(0), self.players)))
        } else {
            Err(self)
        }
    }

    fn cantar_falta_envido(self: Box<Self>, player: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        if self.players.is_turn(player) {
            Ok(Box::new(FaltaEnvido::new(Envidos::Value(0), self.players)))
        } else {
            Err(self)
        }
    }

    fn cantar_truco(self: Box<Self>, player: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        if self.players.is_turn(player) {
            Ok(Box::new(EnvidoVaPrimero::new(self.players)))
        } else {
            Err(self)
        }
    }

    fn cantar_re_truco(self: Box<Self>, _: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Err(self)
    }

    fn cantar_vale_cuatro(self: Box<Self>, _: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Err(self)
    }

    fn tirar_carta(mut self: Box<Self>, player: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        if !self.players.is_turn(player) {
            return Err(self);
        }
        if self.players.next_player() {
            Ok(Box::new(Nada::new(Envidos::Value(0), self.players)))
        } else {
            Ok(self)
        }
    }

    fn tantos(&self) -> Result<Envidos, &str> {
        Err("El envido aun no se termina de cantar.")
    }

    fn valor_ronda(&self) -> Result<Trucos, &str> {
        Err("La ronda aun no a terminado.")
    }
}
