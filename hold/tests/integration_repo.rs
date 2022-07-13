use lfg_core::repo::IntegrationRepo;
use lfg_hold::repo::PsqlIntegrationRepo;
use lfg_support::fixtures::TestNewIntegration;
use sqlx::postgres::PgPoolOptions;

#[tokio::test]
async fn find_empty() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .connect("postgres://jgayfer:@localhost/lfg")
        .await?;
    let repo = PsqlIntegrationRepo::new(pool);

    let found_integration = repo.find(117).await.unwrap();

    assert!(found_integration.is_none());

    Ok(())
}

#[tokio::test]
async fn find_with_result() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .connect("postgres://jgayfer:@localhost/lfg")
        .await?;
    let mut repo = PsqlIntegrationRepo::new(pool);
    let new_integration = TestNewIntegration::new().build();

    let created_integration = repo.create(new_integration).await.unwrap();

    let found_integration = repo.find(created_integration.id).await.unwrap().unwrap();

    assert_eq!(found_integration.id, created_integration.id);

    Ok(())
}

#[tokio::test]
async fn create() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .connect("postgres://jgayfer:@localhost/lfg")
        .await?;
    let mut repo = PsqlIntegrationRepo::new(pool);
    let new_integration = TestNewIntegration::new().build();

    let integration = repo.create(new_integration).await.unwrap();

    assert_eq!(integration.success_url, "https://test-success.com");
    assert_eq!(integration.webhook_url, "https://test-webhook.com");

    Ok(())
}
