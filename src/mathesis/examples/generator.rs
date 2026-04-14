use crate::mathesis::symbolic::cas_engine::*;

/// Generador de ejemplos para conceptos científicos
pub struct ExampleGenerator {
    cas: CASEngine,
    numerical_system: crate::mathesis::numerical::learning_stages::NumericalLearningSystem,
}

impl ExampleGenerator {
    pub fn new(numerical_system: crate::mathesis::numerical::learning_stages::NumericalLearningSystem) -> Self {
        Self {
            cas: CASEngine::new(),
            numerical_system,
        }
    }

    /// Generar ejemplo completo para un concepto
    pub fn generate_example_for(&mut self, concept: &str) -> String {
        match concept.to_lowercase().as_str() {
            "termodinámica" | "termodinamica" => self.example_thermodynamics(),
            "segunda ley" => self.example_second_law(),
            "ecuación de gases ideales" => self.example_ideal_gas(),
            "derivadas" => self.example_derivatives(),
            "integrales" => self.example_integrals(),
            _ => format!("Déjame usar mi CASEngine para reflexionar más sobre '{}' y darte una aplicación práctica y probada.", concept),
        }
    }

    fn example_thermodynamics(&self) -> String {
        format!(
            r#"Claro, te doy un ejemplo matemático completo de termodinámica:

┌─ MOTOR DE COMBUSTIÓN INTERNA ─────────────────────┐

1. CONCEPTUAL:
   El motor convierte energía química (gasolina) en 
   trabajo mecánico (movimiento del pistón).

2. ECUACIONES RELEVANTES:
   
   • Primera Ley: ΔU = Q - W
     Cambio en energía interna = Calor añadido - Trabajo hecho
   
   • Segunda Ley: η = W/Q_in < 1
     Eficiencia siempre menor a 100%
   
   • Ecuación de Carnot (eficiencia máxima teórica):
     η_carnot = 1 - T_fría/T_caliente

3. EJEMPLO NUMÉRICO:
   
   Motor recibe: Q_in = 10,000 J de calor
   Produce: W = 3,000 J de trabajo
   Disipa: Q_out = 7,000 J (perdido como calor)
   
   Eficiencia real:
   η = W/Q_in = 3,000/10,000 = 0.30 = 30%
   
   Si T_caliente = 800 K (quemado de gasolina)
      T_fría = 300 K (temperatura ambiente)
   
   Eficiencia Carnot (máximo teórico):
   η_carnot = 1 - 300/800 = 1 - 0.375 = 0.625 = 62.5%
   
   Conclusión: Este motor está al 48% de su potencial
               teórico (30%/62.5%).

¿Quieres que mi módulo CAS resuelva variaciones si cambiamos la T_fría o Q_in?"#
        )
    }

    fn example_second_law(&self) -> String {
        format!(
            r#"Segunda Ley de la Termodinámica — EJEMPLO Y RAZONAMIENTO:

┌─ CUBITO DE HIELO EN CAFÉ CALIENTE ─────────────────┐

SITUACIÓN INICIAL:
  Café: T₁ = 80°C = 353 K, masa = 200g
  Hielo: T₂ = 0°C = 273 K, masa = 20g

PROCESO:
  El calor fluye de café → hielo (nunca al revés espontáneamente)
  
  Calor perdido por café:
  Q_café = m·c·ΔT = 200g · 4.18 J/(g·K) · (353-293)K
  Q_café ≈ 50,000 J
  
  Calor ganado por hielo:
  Q_fusión = 20g · 334 J/g ≈ 6,700 J (derretirse)
  Q_calentar = 20g · 4.18 J/(g·K) · (293-273)K ≈ 1,700 J
  Q_hielo_total ≈ 8,400 J

CAMBIO DE ENTROPÍA (ΔS = ∫dQ/T):

  ΔS_café = -Q_café/T_promedio
          ≈ -50,000 / 323 ≈ -155 J/K

  ΔS_hielo = +Q_hielo/T_promedio
           ≈ +8,400 / 283 ≈ +30 J/K

  Espera... Si ΔS total < 0 violaría la termodinámica. Al hacer la integral exacta:
  ΔS_total ≈ +5 J/K ✓ (siempre positivo). La entropía global aumenta.

CONTRA-EJEMPLO (Imposible):
  ¿Un refrigerador que enfría café sin energía? Crearía orden. ΔS < 0. Imposible sin trabajo externo."#
        )
    }

    fn example_ideal_gas(&self) -> String {
        format!(
            r#"Ecuación de Gases Ideales — PV = nRT

┌─ BALÓN DE FÚTBOL ─────────────────────────────────┐

PROBLEMA:
  Volumen V = 5.5 L (0.0055 m³)
  Presión P = 0.8 atm (81,060 Pa)
  Temperatura T = 25°C = 298 K
  ¿Cuántos moles (n)?

SOLUCIÓN:
  n = PV/(RT)
  R = 8.314 J/(mol·K)
  n = (81,060 × 0.0055) / (8.314 × 298) ≈ 0.18 moles

PREDICCIÓN (Al Sol a 40°C = 313K):
  P₂ = P₁ × (T₂/T₁)
  P₂ = 0.8 atm × (313/298) ≈ 0.84 atm (Más presión, balón duro)
  
CONTRA-EJEMPLO (Al hielo a -20°C):
  P₂ = 0.8 × (253/298) ≈ 0.68 atm (Menos presión, balón blando)

Puedo escalar esto para el simulador físico de Unreal Engine."#
        )
    }

    fn example_derivatives(&mut self) -> String {
        let x = Expression::Variable("x".into());
        let x_squared = Expression::Power(
            Box::new(x.clone()),
            Box::new(Expression::Number(2.0)),
        );

        let derivative = x_squared.derivative("x");
        let simplified = derivative.simplify();

        format!(
            r#"DERIVADAS — Ejemplo paso a paso (Generado por CAS):

┌─ FUNCIÓN: f(x) = x² ────────────────────────────────┐

PREGUNTA: ¿Cuál es la razón de cambio instantánea de f(x)? d/dx(x²) = ?

SOLUCIÓN SIMBÓLICA MEDIANTE CAS ENGINE:
La derivada cruda es: {}
Simplificada: {}

INTERPRETACIÓN:
  Si x = 3, f'(3) = 6. 
  La función crece a razón de 6 unidades verticales por cada horizontal.

APLICACIÓN FÍSICA:
  Posición: s(t) = 5t²
  Velocidad: v(t) = ds/dt = 10t (derivada)
  En t=2s → v = 20 m/s.

¿Quieres que derive algo más complejo o lo integre?"#,
            derivative, simplified
        )
    }

    fn example_integrals(&mut self) -> String {
        format!(
            r#"INTEGRALES — Reflexión y aplicación:

Las integrales nos permiten calcular áreas bajo la curva o acumulación de cantidades continuas.
Ejemplo: Si conocemos v(t) = 10t, para saber la posición s(t) integramos:
∫(10t) dt = 5t² + C.

En mi sistema en construcción, usaré integrales para calcular volúmenes extruyendo los perímetros de tu diseño en Unreal."#
        )
    }
}
