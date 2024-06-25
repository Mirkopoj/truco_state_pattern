use crate::{
    envido_envido::EnvidoEnvido, falta_envido::FaltaEnvido, nada::Nada, r#final::Final,
    real_envido::RealEnvido, Envidos, TrucoState, Trucos,
};

#[derive(Debug, Clone, Copy)]
pub struct Envido;

const VALOR_QUERIDO: u8 = 2;
const VALOR_NO_QUERIDO: u8 = 1;

impl TrucoState for Envido {
    fn irse_al_maso(&self) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(Final::new(
            Envidos::Value(VALOR_NO_QUERIDO),
            Trucos::Simple,
        )))
    }

    fn cantar_quiero(&self) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(Nada::new(Envidos::Value(VALOR_QUERIDO))))
    }

    fn cantar_no_quiero(&self) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(Nada::new(Envidos::Value(VALOR_NO_QUERIDO))))
    }

    fn cantar_envido(&self) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(EnvidoEnvido))
    }

    fn cantar_real_envido(&self) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(RealEnvido::new(Envidos::Value(VALOR_QUERIDO))))
    }

    fn cantar_falta_envido(&self) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(FaltaEnvido::new(Envidos::Value(VALOR_QUERIDO))))
    }

    fn cantar_truco(&self) -> Result<Box<dyn TrucoState>, ()> {
        Err(())
    }

    fn cantar_re_truco(&self) -> Result<Box<dyn TrucoState>, ()> {
        Err(())
    }

    fn cantar_vale_cuatro(&self) -> Result<Box<dyn TrucoState>, ()> {
        Err(())
    }

    fn tirar_carta(&mut self) -> Result<Box<dyn TrucoState>, ()> {
        Err(())
    }

    fn tantos(&self) -> std::result::Result<Envidos, &str> {
        Err("El envido aun no se termina de cantar.")
    }

    fn valor_ronda(&self) -> std::result::Result<u8, &str> {
        Err("La ronda aun no a terminado.")
    }
}
