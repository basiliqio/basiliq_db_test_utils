#[tokio::main]
async fn main() {
    println!("Deploying migrations");
    basiliq_db_test_utils::run_migrations_in_current_db().await;
    println!("Connecting to db");
    let mut pool = basiliq_db_test_utils::connect_to_management_pool();
    println!("Populating db");
    basiliq_db_test_utils::init_values(&mut pool).await;
}
