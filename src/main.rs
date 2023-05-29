struct Record<K, T> {
    id: K,
    valor: T,
    peso: i32,
}

struct Records<K, T> {
    datos: Vec<Record<K, T>>,
}

impl<K: std::cmp::PartialEq, T> Records<K, T> {
    fn obtener_peso_por_id(&self, id: K) -> i32 {
        for record in &self.datos {
            if record.id == id {
                return record.peso;
            }
        }
        // Si no encuentra nada, entonces regresamos un -1
        -1
    }
}

fn main() {
    // Record con id i32 y valor String
    let record1 = Record {
        id: 1,
        valor: String::from("Este es un valor String del valor 1"),
        peso: 5,
    };

    println!("Valor Record 1: {}", record1.valor);

    // Record con id String y valor numerico
    let _record2 = Record {
        id: String::from("id1"),
        valor: 1000,
        peso: 1,
    };

    // Record con id y valor como string
    let record3 = Record {
        id: String::from("id2"),
        valor: String::from("valor como string nuevamente"),
        peso: 7,
    };

    // Records simulando ser de Una misma tabla en una base de datos
    // // Record con id y valor como string
    let record4 = Record {
        id: String::from("id3"),
        valor: String::from("valor como string nuevamente"),
        peso: 2,
    };

    let records = Records {
        datos: vec![record3, record4],
    };

    let resultado_busqueda = records.obtener_peso_por_id("id2".to_string());

    println!("peso del resultado busqueda: {}", resultado_busqueda);
}
