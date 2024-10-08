use std::time::Instant;
use std::mem::size_of;

// Estructura grande (aproximadamente 1MB)
#[derive(Clone)]
struct GranEstructura {
    datos: [u8; 1_000_000],
    valor: f64,
}

impl GranEstructura {
    fn new(valor: f64) -> Self {
        GranEstructura {
            datos: [0; 1_000_000],
            valor,
        }
    }

    fn procesar_por_valor(self) -> f64 {
        self.valor * 2.0
    }

    fn procesar_por_referencia(&self) -> f64 {
        self.valor * 2.0
    }
}

fn main() {
    println!("Tamaño de GranEstructura: {} bytes", size_of::<GranEstructura>());

    let mut resultados_valor = Vec::new();
    let mut resultados_referencia = Vec::new();

    // Prueba con diferentes números de iteraciones
    for &iteraciones in &[100, 1000, 10000] {
        println!("\nPrueba con {} iteraciones:", iteraciones);

        // Prueba pasando por valor
        let inicio = Instant::now();
        for _ in 0..iteraciones {
            let estructura = GranEstructura::new(42.0);
            let resultado = estructura.procesar_por_valor();
            resultados_valor.push(resultado);
        }
        let duracion_valor = inicio.elapsed();

        // Prueba pasando por referencia
        let inicio = Instant::now();
        for _ in 0..iteraciones {
            let estructura = GranEstructura::new(42.0);
            let resultado = estructura.procesar_por_referencia();
            resultados_referencia.push(resultado);
        }
        let duracion_referencia = inicio.elapsed();

        println!("  Tiempo por valor: {:?}", duracion_valor);
        println!("  Tiempo por referencia: {:?}", duracion_referencia);
        println!("  Diferencia: {:?}", duracion_valor.checked_sub(duracion_referencia).unwrap_or_default());
    }

    // Verificar que los resultados sean iguales
    assert_eq!(resultados_valor, resultados_referencia);
}
