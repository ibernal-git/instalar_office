use reqwest::blocking::Client;
use std::io;
use std::fs::write;
use std::fs::File;
use std::io::{Read, Write};
use std::env;
use std::process::Command;
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; 


#[derive(Debug, EnumIter)]
pub enum Channels {
    PerpetualVL2019,
    O365ProPlusRetail,
}

impl Channels {
    pub fn get_value(&self) -> &'static str {
        match self {
            Channels::PerpetualVL2019 => "PerpetualVL2019",
            Channels::O365ProPlusRetail => "O365ProPlusRetail",
        }
    }
    pub fn get_product_id(&self) -> &'static str {
        match self {
            Channels::PerpetualVL2019 => "ProPlus2019Volume",
            Channels::O365ProPlusRetail => "O365ProPlusRetail",
        }
    }
}

pub const OFFICE_DEPLOYMENT_TOOL_URL: &str = "https://download.microsoft.com/download/2/7/A/27AF1BE6-DD20-4CB4-B154-EBAB8A7D4A7E/officedeploymenttool_15726-20202.exe";
pub const OFFICE_KEY_FILE: &str = "clave.txt";
pub const DEPLOYMENT_FILENAME: &str = "file.exe";
pub const XML_FILENAME: &str = "configuration.xml";

pub fn get_channel() -> Channels {
        loop {
        println!("Selecciona un canal:");

        let mut count: i32 = 1;
        
        for channel in Channels::iter() {
            println!("{} - {:?}", count, channel);
            count += 1;
        }

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Error al leer la entrada");

        let user_option = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match user_option {
            1 =>  return Channels::PerpetualVL2019,
            2 =>  return Channels::O365ProPlusRetail,
            _ => println!("Opción no válida"),
        }

    }
}

pub fn download_file(file: &str, url: &str) {
  
    // Crea un nuevo cliente HTTP
    let client = Client::new();
  
    // Realiza una solicitud HTTP GET al URL del archivo que quieres descargar
    let response = client.get(url)
        .send()
        .expect("Error al realizar la solicitud");
  
    // Obtén el contenido del archivo como una cadena de bytes
    let bytes = response.bytes().expect("Error al obtener el contenido del archivo");
  
    // Escribe el contenido del archivo en disco en un archivo llamado "file.exe"
    write(file, bytes).expect("Error al escribir el archivo en disco");
  
  }

pub fn get_current_directory() -> String {
    
    let current_dir = match env::current_dir() {
        Ok(dir) => dir, // Si current_dir() devuelve un valor correcto, almacenarlo en la variable "dir"
        Err(e) => {
            // Si current_dir() devuelve un error, imprimirlo y salir del programa
            println!("Error al obtener el directorio actual: {}", e);
            std::process::exit(1);
        }
    };
    
    return String::from(current_dir.to_string_lossy());
}

pub fn run_command(exe: &String, directory: &String, args: &Vec<String>, ) {

    let mut command = Command::new(format!("{}\\{}",  directory, exe));

    for arg in args {
        command.arg(arg);
    }

    // Ejecuta el programa y maneja posibles errores
    match command.output() {
        Ok(output) => {
            println!("La ejecución de {:?} se realizo correctamente\n", exe);
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        Err(error) => {
            println!("Error al ejecutar el programa: {}\n", error);
        }
    }

}

pub fn read_file(file_name: &str) -> Result<String, String> {
    let mut file = match File::open(file_name) {
        Ok(file) => file,
        Err(err) => return Err(format!("Error al abrir el archivo: {}\n", err)),
    };

    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Ok(_) => Ok(file_contents),
        Err(err) => Err(format!("Error al leer el archivo: {}\n", err)),
    }
}

pub fn write_file(file_name: &str, content : &String) -> Result<String, String> {

    let mut file = match File::create(file_name) {
        Ok(file) => file,
        Err(err) => panic!("Error al crear el archivo: {}", err),
    };

    match file.write_all(content.as_bytes()) {
        Ok(_) => Ok(format!("XML escrito correctamente en el archivo {}\n", file_name)),
        Err(err) => panic!("Error al escribir en el archivo: {}", err),
    }

}