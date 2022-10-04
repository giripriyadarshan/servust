pub mod frameworks;
pub mod orms;

pub mod install {
    use super::{
        frameworks::{actix::actix, axum::axum, salvo::salvo, tonic::tonic},
        orms::{diesel::diesel, sea_orm::sea_orm},
    };

    pub async fn install_orm(
        orm: String,
        database: String,
        framework: &str,
    ) -> Result<bool, String> {
        // TODO: implement the orm module

        match orm.as_str() {
            "diesel" => diesel(database).await,
            "sea-orm" => sea_orm(database, framework).await,
            _ => Err(format!("{} is not an implemented orm by Servust", orm)),
        }
    }

    pub async fn install_framework(framework: &str) -> Result<bool, String> {
        // TODO: implement the framework module

        match framework {
            "actix" => actix().await,
            "tonic" => tonic().await,
            "salvo" => salvo().await,
            "axum" => axum().await,
            _ => Err(format!(
                "{} is not an implemented framework by Servust",
                framework
            )),
        }
    }
}
