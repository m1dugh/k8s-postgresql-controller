use futures::TryStreamExt;
use kube::{ResourceExt, runtime::watcher, Api};
use crate::{crds::Database, manager_controller::ManagersMap};

const ANNOTATION_KEY: &str = "psql.midugh.fr/manager";

async fn on_db_applied(db: Database, managers_map: ManagersMap) -> bool {

    let _namespace = db.namespace().clone().unwrap();

    let annotations = match db.metadata.annotations.clone() {
        Some(v) => v,
        _ => return false,
    };

    let manager_name = match annotations.get(ANNOTATION_KEY) {
        Some(v) => v.to_string(),
        _ => return false,
    };

    if let Some(manager) = managers_map.lock().unwrap().get(&manager_name) {
        if let Err(e) = manager.create_user(&db.spec.username, &db.spec.password).await {
            eprintln!("create user: {e}");
            return false
        }

        if let Err(e) = manager.create_db(&db.spec.name, &db.spec.username).await {
            eprintln!("create db: {e}");
            return false;
        }
    }

    true
}

async fn on_db_removed(_db: Database, _managers_map: ManagersMap) -> bool {
    true
}

pub async fn watch_databases(db_api: Api<Database>, managers_map: ManagersMap) -> Result<bool, watcher::Error> {
    watcher(db_api, watcher::Config::default())
        .try_all(|e| async {
            match e {
                watcher::Event::Applied(d)
                    => on_db_applied(d, managers_map.clone()).await,
                watcher::Event::Deleted(d)
                    => on_db_removed(d, managers_map.clone()).await,
                _ => true,
            }
        }).await
}
