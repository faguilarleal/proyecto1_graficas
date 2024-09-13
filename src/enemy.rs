
use nalgebra_glm::{Vec2};

pub struct Enemy {
    pub pos: Vec2,
    pub collected: bool, // Nuevo campo para verificar si el enemigo ha sido recolectado
}

impl Enemy {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            collected: false,
        }
    }
    // Función para cambiar el estado de collected a true
    pub fn collect(&mut self) {
        self.collected = true;
    }
}

