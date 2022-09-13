mod common;

use anyhow::Result;
use common::make_client;
use rusigma::utils::{Genero, Provincias, SearchFilters};

#[tokio::test]
async fn test_client_login() -> Result<()> {
    let mut _sclient = make_client().await?;
    println!("Account logged in");
    Ok(())
}

#[tokio::test]
async fn test_search_standard_dni() -> Result<()> {
    let mut sclient = make_client().await?;
    let dni_results = sclient.search_standard_dni("45938102".to_string()).await?;
    println!("{:#?}", dni_results);
    Ok(())
}

#[tokio::test]
async fn test_search_numbers_by_dni() -> Result<()> {
    let mut sclient = make_client().await?;
    let numbers = sclient.search_phones_by_dni("41042191".to_string()).await?;
    println!("{:#?}", numbers);
    Ok(())
}

#[tokio::test]
async fn test_search_plate() -> Result<()> {
    let mut sclient = make_client().await?;
    let plates = sclient.search_plate("gay001".to_string()).await?;
    println!("{:#?}", plates);
    Ok(())
}

#[tokio::test]
async fn test_search_plate_by_dni() -> Result<()> {
    let mut sclient = make_client().await?;
    let plates = sclient.search_plate_by_dni("24546048".to_string()).await?;
    println!("{:#?}", plates);
    Ok(())
}

#[tokio::test]
async fn test_search_leaks() -> Result<()> {
    let mut client = make_client().await?;
    let leaks = client.search_leaks("cronica".to_string()).await?;
    println!("{:#?}", leaks);
    Ok(())
}

#[tokio::test]
async fn test_search_profesional_dni() -> Result<()> {
    let mut sclient = make_client().await?;
    let dni_results = sclient
        .search_profesional_dni("45938102".to_string(), Genero::Masculino)
        .await?;
    println!("{:#?}", dni_results);
    Ok(())
}

#[tokio::test]
async fn test_search_nombre() -> Result<()> {
    let mut sclient = make_client().await?;

    let mut filters = SearchFilters::new();
    filters.set_provincia(Provincias::Chaco);

    let personas = sclient
        .search_name("Carlos Perez".to_string(), Some(filters))
        .await?;
    println!("{:#?}", personas);
    Ok(())
}

#[tokio::test]
async fn test_search_movistar() -> Result<()> {
    let mut sclient = make_client().await?;
    let movistar = sclient
        .search_movistar_email("2645559925".to_string())
        .await?;
    println!("{:#?}", movistar);
    Ok(())
}

#[tokio::test]
async fn test_search_by_address() -> Result<()> {
    let mut sclient = make_client().await?;
    let personas = sclient.search_by_address("niceto vega".to_string()).await?;
    println!("{:#?}", personas);
    Ok(())
}

#[tokio::test]
async fn test_search_phone() -> Result<()> {
    let mut sclient = make_client().await?;
    let handlers = sclient.search_phone("2645559925".to_string()).await?;
    println!("{:#?}", handlers);
    Ok(())
}

#[tokio::test]
async fn test_search_phone_magic() -> Result<()> {
    let mut sclient = make_client().await?;
    let handler = sclient.search_phone_magic("1158490291".to_string()).await?;
    println!("{:#?}", handler);
    Ok(())
}

#[tokio::test]
async fn test_search_cbu() -> Result<()> {
    let mut sclient = make_client().await?;
    let cbu_results = sclient.search_cbu("interpol".to_string()).await?;
    println!("{:#?}", cbu_results);
    Ok(())
}

#[tokio::test]
async fn test_search_email() -> Result<()> {
    let mut sclient = make_client().await?;
    let email_owner = sclient
        .search_email("frankitox98@gmail.com".to_string())
        .await?;
    println!("{:#?}", email_owner);
    Ok(())
}
