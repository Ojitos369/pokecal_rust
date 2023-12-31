use std::env;
use std::collections::HashMap;

use mysql::prelude::*;
use mysql::Pool;
use mysql::serde_json::json;

use gtk4 as gtk;
use gdk::Display;
use gtk::prelude::*;
// Button, Entry, Label, ComboBoxText, CssProvider
use gtk::{gdk, glib, Application, ApplicationWindow, Orientation, Label, ComboBoxText, CssProvider };
use glib::clone;

const APP_ID: &str = "com.ojitos369.pkcal";

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

fn get_all_tipos() -> Result<Vec<Tipo>, mysql::Error> {
    let mut conn = get_conn()?;
    let query = "SELECT * FROM tipos ORDER BY nombre ASC";
    let rows: Vec<Tipo> = conn.query_map(query, |(id, nombre)| {
        Tipo { id, nombre }
    })?;
    return Ok(rows);
}

fn get_danio(con: &mut mysql::PooledConn, tipos: Vec<&str>) -> Result<Vec<(String, Vec<(String, f32)>)>, mysql::Error> {
    let mut conn = con;

    let query_tipos = "SELECT * FROM tipos WHERE nombre = '{tipo}'";
    let query_dad = "SELECT (select nombre from tipos where id = d.da_a) nombre FROM da_doble d where tipo = '{tipo}' order by nombre asc";
    let query_recibed = "SELECT (select nombre from tipos where id = d.recibe_de) nombre FROM recibe_doble d where tipo = '{tipo}' order by nombre asc";
    let query_dam = "SELECT (select nombre from tipos where id = d.da_a) nombre FROM da_mitad d where tipo = '{tipo}' order by nombre asc";
    let query_recibem = "SELECT (select nombre from tipos where id = d.recibe_de) nombre FROM recibe_mitad d where tipo = '{tipo}' order by nombre asc";
    let query_da0 = "SELECT (select nombre from tipos where id = d.da_a) nombre FROM da_nada d where tipo = '{tipo}' order by nombre asc";
    let query_recibe0 = "SELECT (select nombre from tipos where id = d.recibe_de) nombre FROM recibe_nada d where tipo = '{tipo}' order by nombre asc";

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
            // panic!("No existe el tipo {t}");
            // wrap error
            let text = format!("No existe el tipo {}", t);
            let result: Vec<(String, Vec<(String, f32)>)> = [(text, vec![])].to_vec();
            return Ok(result);
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

fn set_label_text(op1: &ComboBoxText, op2: &ComboBoxText, vbox: &gtk::Box) {
    // val or ""
    let val1 = op1.active_id().unwrap_or_default();
    let val2 = op2.active_id().unwrap_or_default();

    let mut tipos: Vec<&str> = Vec::new();

    if val1 != "" {
        if !tipos.contains(&val1.as_str()) {
            tipos.push(val1.as_str());
        }
    }
    if val2 != "" {
        if !tipos.contains(&val2.as_str()) {
            tipos.push(val2.as_str());
        }
    }

    let last_child_t = vbox.last_child();
    let last_child = last_child_t.as_ref().unwrap();
    let fist_child_temp = last_child.last_child();
    // validate if is label
    if fist_child_temp.is_some() {
        let fist_child = fist_child_temp.unwrap();
        // validate if is label
        if fist_child.is::<Label>() {
            // remove last child
            vbox.remove(last_child);
        }
    }
    if tipos.len() == 0 {
        return;
    }

    let mut conn = get_conn().unwrap();

    let new_child = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    // try and catch message error
    let mut danio: Vec<(String, Vec<(String, f32)>)> = Vec::new();
    let exec_it = || -> Result<(), mysql::Error> {
        danio = get_danio(&mut conn, tipos).unwrap();
        Ok(())
    };

    if let Err(e) = exec_it() {
        let text = format!("Error: {}", e);
        let title_label = Label::builder()
            .label(text.as_str())
            .build();
        new_child.append(&title_label);
        vbox.append(&new_child);
        return;
    }

    for d in danio {
        let mut danio_4 = 0;
        let mut danio_2 = 0;
        let mut danio_50 = 0;
        let mut danio_25 = 0;
        let mut danio_0 = 0;

        // let mut text2 = "".to_string();
        let mut to_add: Vec<Label> = Vec::new();
        for f in d.1 {
            if f.1 == 4.0 {
                danio_4 += 1;
            } else if {
                f.1 == 2.0
            } {
                danio_2 += 1;
            } else if {
                f.1 == 0.5
            } {
                danio_50 += 1;
            } else if {
                f.1 == 0.25
            } {
                danio_25 += 1;
            } else if {
                f.1 == 0.0
            } {
                danio_0 += 1;
            }

            let text_damage = format!("{}: {}\n", f.0, f.1);
            let damage_label = Label::builder()
                .label(text_damage.as_str())
                .margin_start(0)
                .margin_end(0)
                .margin_bottom(0)
                .margin_top(0)
                .build();
            to_add.push(damage_label);
        }

        // Se agrega el tipo elegido 
        let text = format!("Tipo: {}\t{} - {} - {} - {} - {} -> {}, {}, {}", d.0, danio_4, danio_2, danio_50, danio_25, danio_0, danio_4+danio_2, danio_50+danio_25, danio_0);
        let title_label = Label::builder()
            .label(text.as_str())
            .margin_bottom(15)
            .build();
        new_child.append(&title_label);

        for a in to_add {
            new_child.append(&a);
        }
    }

    vbox.append(&new_child);
}



fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    app.run()
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("style.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    // One Button
    let opciones: Vec<Tipo> = get_all_tipos().unwrap();

    let opciones1 = ComboBoxText::builder()
        .css_classes(vec!["opciones_list", "opciones1"])
        .halign(gtk::Align::Center)
        .margin_end(40)
        .build();
    let opciones2 = ComboBoxText::builder()
        .css_classes(vec!["opciones_list", "opciones2"])
        .halign(gtk::Align::Center)
        .margin_start(40)
        .build();

    opciones1.append(Some(""), "-------------------");
    opciones2.append(Some(""), "-------------------");
    for o in opciones {
        opciones1.append(Some(&o.nombre.to_string()), &o.nombre);
        opciones2.append(Some(&o.nombre.to_string()), &o.nombre);
    }
    opciones1.set_active(Some(0));
    opciones2.set_active(Some(0));

    let vbox = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    opciones1.connect_changed(clone!(@weak opciones1, @weak opciones2, @weak vbox => move |_| {
        set_label_text(&opciones1, &opciones2, &vbox);
    }));
    opciones2.connect_changed(clone!(@weak opciones1, @weak opciones2, @weak vbox => move |_| {
        set_label_text(&opciones1, &opciones2, &vbox);
    }));

    // opt className "opciones"
    let opt = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .css_classes(vec!["opciones"])
        .build();

    opt.append(&opciones1);
    opt.append(&opciones2);
    
    vbox.append(&opt);

    // overflow scroll
    let scroll = gtk::ScrolledWindow::builder()
        .child(&vbox)
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .build();

    // Create the main window
    // width, height 500 px
    let window = ApplicationWindow::builder()
        .application(app)
        .title("pkCal")
        .child(&scroll)
        .default_width(500)
        .default_height(500)
        .build();

    // Present window
    window.present();
}

// export DB_USER='root'
// export DB_PASSWORD='root'
// export DB_HOST='localhost'
// export DB_PORT='3307'
// export DB_DATABASE='pkcal'
