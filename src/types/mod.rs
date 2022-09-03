pub mod frameworks;
pub mod orms;

pub mod install {
    use super::{frameworks::actix::actix, orms::diesel::diesel};

    pub async fn install_orm(orm: String, database: String) -> Result<bool, String> {
        // TODO: implement the orm module

        return match orm.as_str() {
            "diesel" => diesel(database).await,
            _ => Err(format!("{} is not an implemented orm by Servust", orm)),
        };
    }

    pub async fn install_framework(framework: String) -> Result<bool, String> {
        // TODO: implement the framework module

        return match framework.as_str() {
            "actix" => actix().await,
            _ => Err(format!(
                "{} is not an implemented framework by Servust",
                framework
            )),
        };
    }
}
