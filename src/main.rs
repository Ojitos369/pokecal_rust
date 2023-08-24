use std::env;
use mysql::prelude::*;
use mysql::Pool;
use std::collections::HashMap;
use mysql::serde_json::json;

struct Tipo {
    id: i32,
    nombre: String,
}
struct Nombre {
    nombre: String,
}


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

fn get_tipos(conn: &mut mysql::PooledConn, query: String) -> Result<Vec<Tipo>, mysql::Error> {
    let rows: Vec<Tipo> = conn.query_map(query, |(id, nombre)| {
        Tipo { id, nombre }
    })?;
    return Ok(rows);
}

fn get_nombre(conn: &mut mysql::PooledConn, query: String) -> Result<Vec<Nombre>, mysql::Error> {
    let rows: Vec<Nombre> = conn.query_map(query, |nombre| {
        Nombre { nombre }
    })?;
    return Ok(rows);
}

fn get_danio(con: &mut mysql::PooledConn, tipos: Vec<&str>) -> Result<Vec<(String, Vec<(String, f32)>)>, mysql::Error> {
    let mut conn = con;

    let query_tipos = "SELECT * FROM tipos WHERE nombre = '{tipo}'";
    let query_dad = "SELECT (select nombre from tipos where id = d.da_a) nombre FROM da_doble d where tipo = '{tipo}' ";
    let query_recibed = "SELECT (select nombre from tipos where id = d.recibe_de) nombre FROM recibe_doble d where tipo = '{tipo}' ";
    let query_dam = "SELECT (select nombre from tipos where id = d.da_a) nombre FROM da_mitad d where tipo = '{tipo}' ";
    let query_recibem = "SELECT (select nombre from tipos where id = d.recibe_de) nombre FROM recibe_mitad d where tipo = '{tipo}' ";
    let query_da0 = "SELECT (select nombre from tipos where id = d.da_a) nombre FROM da_nada d where tipo = '{tipo}' ";
    let query_recibe0 = "SELECT (select nombre from tipos where id = d.recibe_de) nombre FROM recibe_nada d where tipo = '{tipo}' ";

    let mut data = json!({
        "dam": [],
        "recibed": [],
        "recibem": [],
        "dad": [],
        "da0": [],
        "recibe0": [],
    });

    let mut final_name = String::new();

    let mut dad = json!({"info": []});
    let mut recibed = json!({"info": []});
    let mut dam = json!({"info": []});
    let mut recibem = json!({"info": []});
    let mut da0 = json!({"info": []});
    let mut recibe0 = json!({"info": []});

    for t in tipos {
        let query_t: String = query_tipos.replace("{tipo}", t);
        let rs = get_tipos(&mut conn, query_t)?;
        if rs.len() == 0 {
            panic!("No existe el tipo {t}");
        }

        let tipo = &rs[0];

        final_name.push_str(&tipo.nombre);
        final_name.push_str("-");

        // println!("Para: {}", tipo.nombre);

        // dad
        let query_dad_t = query_dad.replace("{tipo}", &tipo.id.to_string());
        let dad_temp: Vec<Nombre> = get_nombre(&mut conn, query_dad_t)?;
        for d in dad_temp {
            dad["info"].as_array_mut().unwrap().push(json!(d.nombre));
            data["dad"].as_array_mut().unwrap().push(json!(d.nombre));
        }

        // recibed
        // println!("recibed_temp: ");
        let query_recibed_t = query_recibed.replace("{tipo}", &tipo.id.to_string());
        let recibed_temp: Vec<Nombre> = get_nombre(&mut conn, query_recibed_t)?;
        for d in recibed_temp {
            // println!("{}", d.nombre);
            recibed["info"].as_array_mut().unwrap().push(json!(d.nombre));
            data["recibed"].as_array_mut().unwrap().push(json!(d.nombre));
        }

        // dam
        let query_dam_t = query_dam.replace("{tipo}", &tipo.id.to_string());
        let dam_temp: Vec<Nombre> = get_nombre(&mut conn, query_dam_t)?;
        for d in dam_temp {
            dam["info"].as_array_mut().unwrap().push(json!(d.nombre));
            data["dam"].as_array_mut().unwrap().push(json!(d.nombre));
        }

        // recibem
        // println!("recibem_temp: ");
        let query_recibem_t = query_recibem.replace("{tipo}", &tipo.id.to_string());
        let recibem_temp: Vec<Nombre> = get_nombre(&mut conn, query_recibem_t)?;
        for d in recibem_temp {
            // println!("{}", d.nombre);
            recibem["info"].as_array_mut().unwrap().push(json!(d.nombre));
            data["recibem"].as_array_mut().unwrap().push(json!(d.nombre));
        }

        // da0
        let query_da0_t = query_da0.replace("{tipo}", &tipo.id.to_string());
        let da0_temp: Vec<Nombre> = get_nombre(&mut conn, query_da0_t)?;
        for d in da0_temp {
            da0["info"].as_array_mut().unwrap().push(json!(d.nombre));
            data["da0"].as_array_mut().unwrap().push(json!(d.nombre));
        }

        // recibe0
        // println!("recibe0_temp: ");
        let query_recibe0_t = query_recibe0.replace("{tipo}", &tipo.id.to_string());
        let recibe0_temp: Vec<Nombre> = get_nombre(&mut conn, query_recibe0_t)?;
        for d in recibe0_temp {
            // println!("{}", d.nombre);
            recibe0["info"].as_array_mut().unwrap().push(json!(d.nombre));
            data["recibe0"].as_array_mut().unwrap().push(json!(d.nombre));
        }
        
    }

    let mut danio = HashMap::new();

    for f in recibed["info"].as_array().unwrap() {
        let nombre = f.as_str().unwrap();
        let entry = danio.entry(nombre.to_string()).or_insert(0);
        *entry += 2;
    }
    for f in recibem["info"].as_array().unwrap() {
        let nombre = f.as_str().unwrap();
        let entry = danio.entry(nombre.to_string()).or_insert(0);
        *entry -= 2;
    }
    for f in recibe0["info"].as_array().unwrap() {
        let nombre = f.as_str().unwrap();
        let entry = danio.entry(nombre.to_string()).or_insert(0);
        *entry = -100;
    }

    let mut danio_fin = HashMap::new();
    for (k, v) in danio.iter() {
        let v = *v as i32;
        let mut vf: f32 = 0.0;
        if v != 0 {
            if v < 0 && v != -100 {
                vf = -1.0 / v as f32;
            }
            else if v == -100 {
                vf = 0.0;
            }
            else if v > 0 {
                vf = v as f32;
            }
            danio_fin.insert(k.clone(), vf);
        }
    }
    // order danio_fin by value
    let mut danio_fin: Vec<_> = danio_fin.into_iter().collect();
    danio_fin.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    final_name.pop();

    // final_name, danio_fin
    let result: Vec<(String, Vec<(String, f32)>)> = [(final_name, danio_fin)].to_vec();
    return Ok(result);
}

fn main() -> Result<(), mysql::Error> {
    println!("Que tipos quieres ver? ");
    let mut tipos_str = String::new();
    std::io::stdin().read_line(&mut tipos_str).unwrap();
    let tipos_str = tipos_str.trim().replace(" ", "");
    let tipos: Vec<&str> = tipos_str.split(",").collect();

    let mut conn = get_conn()?;

    let danio = get_danio(&mut conn, tipos)?;
    for d in danio {
        println!("{}: ", d.0);
        for f in d.1 {
            println!("{}: {}", f.0, f.1);
        }
    }

    Ok(())
}

// to compile
// cargo build --release
// to run
// cargo run --release
// to run without compile
// 
