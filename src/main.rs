use std::fs::File;
use std::io::Write;

use crate::fs::read_json;
use actix_multipart::Multipart;
use actix_web::{put, App, HttpResponse, HttpServer};
use futures::{StreamExt, TryStreamExt};

use crate::get_services::{
    handle_accounts_services, handle_memberships, handle_subdomain, handle_user,
};
use crate::post_services::handle_accounts_scripts;

mod fs;
mod get_services;
mod post_services;
mod put_services;

#[put("/client/v4/accounts/{accounts}/workers/scripts/{scripts}")]
async fn save_file(mut payload: Multipart) -> String {
    // Parcourir les différents champs de la requête multipart
    while let Ok(Some(mut field)) = payload.try_next().await {
        // Récupérer le nom du fichier depuis le champ
        let content_disposition = field.content_disposition();

        let filename = content_disposition.get_filename().unwrap();

        // Chemin de destination pour enregistrer le fichier
        let dest_path = format!("./uploads/{}", filename);
        println!("sgrklsdrhwguosd");
        // Créer un fichier local pour enregistrer le contenu du champ
        if let Ok(mut f) = File::create(&dest_path) {
            println!("je suis passe ici");
            // Copier le contenu du champ vers le fichier local
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f = match f.write_all(&data) {
                    Ok(_) => f,
                    Err(_) => {
                        panic!("cannot write file");
                        return format!("error");
                    }
                };
            }
        } else {
            panic!("cannot create file");
            return format!("error");
        }
    }

    read_json("./src/defaultResponses/put_accounts_scripts.json")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(handle_user)
            .service(handle_memberships)
            .service(handle_subdomain)
            .service(handle_accounts_services)
            .service(handle_accounts_scripts)
            .service(save_file)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
