use std::env;
use mysql::prelude::*;
use mysql::Pool;
// use mysql::serde_json::json;


fn get_link_connection() -> String {
    let user = env::var("DB_USER").unwrap_or_default();
    let password = env::var("DB_PASSWORD").unwrap_or_default();
    let host = env::var("DB_HOST").unwrap_or_default();
    let port = env::var("DB_PORT").unwrap_or_default();
    let name = env::var("DB_DATABASE").unwrap_or_default();

    let url_btr = "mysql://".to_string() + &user + ":" + &password + "@" + &host + ":" + &port + "/" + &name;
    return url_btr;
}

fn get_conn() -> Result<mysql::PooledConn, mysql::Error> {
    let url_str = get_link_connection();
    let url = url_str.as_str();
    let pool = Pool::new(url)?;
    let conn = pool.get_conn()?;
    return Ok(conn);
}

struct Tipo {
    id: i32,
    nombre: String,
}

fn main() -> Result<(), mysql::Error> {
    let tipos = [
        "Normal",
        "Pelea",
        "Volador",
        "Veneno",
        "Tierra",
        "Roca",
        "Bicho",
        "Fantasma",
        "Acero",
        "Fuego",
        "Agua",
        "Planta",
        "Electrico",
        "Psiquico",
        "Hielo",
        "Dragon",
        "Siniestro",
        "Hada",
    ];

    let mut conn = get_conn()?;
    let mut query = "INSERT INTO tipos (nombre) values ('{tipo}') ";

    for t in tipos {
        println!("{}", t);
        let query = query.replace("{tipo}", t);
        conn.query_drop(query)?;

    }

    let json_data_path = "media/json/tipos.json";
    let json_data_d = std::fs::read_to_string(json_data_path).expect("No se pudo leer el archivo");
    let data: serde_json::Value = serde_json::from_str(&json_data_d).expect("No se pudo parsear el json");
    // print!("{}", data);

    query = "SELECT * FROM tipos WHERE nombre = '{tipo}'";
    for (k, v) in data.as_object().unwrap().iter() {
        println!("Datos de {}...", k);
        let rows: Vec<Tipo> = conn.query_map(query.replace("{tipo}", k), |(id, nombre)| {
            Tipo { id, nombre }
        })?;
        let tipo = &rows[0];
        println!("Tipo: {}", tipo.nombre);

        // dad - da_doble - da_a
        println!("Da Doble de {}...", k);
        let dad = v.get("dad").unwrap();
        for d in dad.as_array().unwrap() {
            let qrs = query.replace("{tipo}", d.as_str().unwrap());
            let rs = conn.query_map(qrs, |(id, nombre)| {
                Tipo { id, nombre }
            })?;
            let tipo_d = &rs[0];
                        
            let mut qr: String = String::from("INSERT INTO da_doble (tipo, da_a) VALUES ('{t_id}', '{td_id}')");
            qr = qr.replace("{t_id}", &tipo.id.to_string());
            qr = qr.replace("{td_id}", &tipo_d.id.to_string());
            conn.query_drop(qr)?;
        }

        // recibed - recibe_doble - recibe_de
        println!("Recibe Doble de {}...", k);
        let recibed = v.get("recibed").unwrap();
        for d in recibed.as_array().unwrap() {
            let qrs = query.replace("{tipo}", d.as_str().unwrap());
            let rs = conn.query_map(qrs, |(id, nombre)| {
                Tipo { id, nombre }
            })?;
            let tipo_d = &rs[0];
                        
            let mut qr: String = String::from("INSERT INTO recibe_doble (tipo, recibe_de) VALUES ('{t_id}', '{td_id}')");
            qr = qr.replace("{t_id}", &tipo.id.to_string());
            qr = qr.replace("{td_id}", &tipo_d.id.to_string());
            conn.query_drop(qr)?;
        }

        // dam - da_mitad - da_a
        println!("Da Mitad de {}...", k);
        let dam = v.get("dam").unwrap();
        for d in dam.as_array().unwrap() {
            let qrs = query.replace("{tipo}", d.as_str().unwrap());
            let rs = conn.query_map(qrs, |(id, nombre)| {
                Tipo { id, nombre }
            })?;
            let tipo_d = &rs[0];
                        
            let mut qr: String = String::from("INSERT INTO da_mitad (tipo, da_a) VALUES ('{t_id}', '{td_id}')");
            qr = qr.replace("{t_id}", &tipo.id.to_string());
            qr = qr.replace("{td_id}", &tipo_d.id.to_string());
            conn.query_drop(qr)?;
        }

        // recibem - recibe_mitad - recibe_de
        println!("Recibe Mitad de {}...", k);
        let recibem = v.get("recibem").unwrap();
        for d in recibem.as_array().unwrap() {
            let qrs = query.replace("{tipo}", d.as_str().unwrap());
            let rs = conn.query_map(qrs, |(id, nombre)| {
                Tipo { id, nombre }
            })?;
            let tipo_d = &rs[0];
                        
            let mut qr: String = String::from("INSERT INTO recibe_mitad (tipo, recibe_de) VALUES ('{t_id}', '{td_id}')");
            qr = qr.replace("{t_id}", &tipo.id.to_string());
            qr = qr.replace("{td_id}", &tipo_d.id.to_string());
            conn.query_drop(qr)?;
        }

        // da0 - da_nada - da_a
        println!("Da Nada de {}...", k);
        let da0 = v.get("da0").unwrap();
        for d in da0.as_array().unwrap() {
            let qrs = query.replace("{tipo}", d.as_str().unwrap());
            let rs = conn.query_map(qrs, |(id, nombre)| {
                Tipo { id, nombre }
            })?;
            let tipo_d = &rs[0];
                        
            let mut qr: String = String::from("INSERT INTO da_nada (tipo, da_a) VALUES ('{t_id}', '{td_id}')");
            qr = qr.replace("{t_id}", &tipo.id.to_string());
            qr = qr.replace("{td_id}", &tipo_d.id.to_string());
            conn.query_drop(qr)?;
        }

        // recibe0 - recibe_nada - recibe_de
        println!("Recibe Nada de {}...", k);
        let recibe0 = v.get("recibe0").unwrap();
        for d in recibe0.as_array().unwrap() {
            let qrs = query.replace("{tipo}", d.as_str().unwrap());
            let rs = conn.query_map(qrs, |(id, nombre)| {
                Tipo { id, nombre }
            })?;
            let tipo_d = &rs[0];
                        
            let mut qr: String = String::from("INSERT INTO recibe_nada (tipo, recibe_de) VALUES ('{t_id}', '{td_id}')");
            qr = qr.replace("{t_id}", &tipo.id.to_string());
            qr = qr.replace("{td_id}", &tipo_d.id.to_string());
            conn.query_drop(qr)?;
        }

        println!("Agregado correctamente");

    }

    Ok(())
}

// dr run -d -p 3307:3306 --name pkcal -e MYSQL_ROOT_PASSWORD=root -e MYSQL_DATABASE=pkcal mysql