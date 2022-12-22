use std::io;

use instalar_office::OFFICE_DEPLOYMENT_TOOL_URL;
use instalar_office::OFFICE_KEY_FILE;
use instalar_office::XML_FILENAME;
use instalar_office::DEPLOYMENT_FILENAME;
use instalar_office::download_file;
use instalar_office::get_current_directory;
use instalar_office::run_command;
use instalar_office::read_file;
use instalar_office::write_file;
use instalar_office::get_channel;
use instalar_office::Channels;



fn main() {

    let channel = get_channel();

    let deployment_filename = String::from(DEPLOYMENT_FILENAME);

    download_file(&deployment_filename, OFFICE_DEPLOYMENT_TOOL_URL);

    let current_dir = get_current_directory();

    let deployment_args = vec![format!("/extract:{}", current_dir),"/passive".to_owned()];
    run_command(&deployment_filename,  &current_dir, &deployment_args);


    let mut office_key= String::new();


    if channel.get_value() != Channels::O365ProPlusRetail.get_value() {
        let office_key_file = OFFICE_KEY_FILE;
        
        match read_file(office_key_file) {
            Ok(data) => {
                office_key = format!("PIDKEY=\"{}\"", data);
            },
            Err(err) => {
                println!("Error: {}", err);
                input_office_key();
            },
            // Err(err) => panic!("{}", err),
        };

        fn input_office_key() -> String {
            println!("Introduce la clave de producto:");
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("Error al leer la entrada");
            return user_input;
        }
    }

    let xml_string = format!("<Configuration>
    <Add OfficeClientEdition=\"64\" Channel=\"{}\">
        <Product ID=\"{}\" {}>
        <Language ID=\"es-es\" />
        </Product>
        <Product ID=\"ProofingTools\">
        <Language ID=\"es-es\" />
        </Product>
    </Add>
    <RemoveMSI All=\"True\" />
    </Configuration>", channel.get_value(), channel.get_product_id(), office_key);

    match write_file(XML_FILENAME, &xml_string) {
        Ok(result) => println!("{}", result),
        Err(err) => panic!("{}", err),
    }

    let office_args = vec!["/configure".to_owned(), format!("{}", XML_FILENAME)];

    run_command(&"setup.exe".to_owned(),  &current_dir, &office_args);

}

