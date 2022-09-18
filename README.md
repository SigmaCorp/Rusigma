# Rusigma
Sigma Search SDK built on Rust

## Usage
```rust
use rusigma::SigmaClient;

#[tokio::main]
async fn main() -> Result<()> {
    let mut sclient = SigmaClient::new();
    sclient.login_with_credentials(username, password).await?;
    
    let results = sclient.test_search_standard_dni("185912951".to_string()).await?;
    println!("{:#?}", results);
    
    Ok(())
}
```

## Methods
```rust
    /// Searches for a person by their DNI.
    /// The argument DNI must be a String, returns an
    /// Struct containing the data.
    ///
    /// This is part of the standard plan or higher.
    ///
    /// ```
    /// let personData = client.search_standard_dni("4211928".to_string()).await?;
    /// ```
    pub async fn search_standard_dni(
        &mut self,
        dni: String,
    ) -> Result<response::DNIStandardResponse, Error>

    /// Searches for all the phone numbers related
    /// to a given DNI. This function returns a vector of
    /// PhoneNumber structs with data from the owner.
    ///
    /// This is part of the standard plan or higher.
    ///
    /// ```
    /// let phoneNumbers = client.search_phones_by_dni("4211928".to_string()).await?;
    /// ```
    pub async fn search_phones_by_dni(
        &mut self,
        dni: String,
    ) -> Result<Vec<response::PhoneNumber>, Error>

    /// Searches for every transaction of a
    /// given vehicle plate containing data about owners
    /// that can be people or organizations.
    ///
    /// The return object is a vector with PlateHistory
    /// structs.
    ///
    /// This is part of the medium plan or higher.
    ///
    /// ```
    /// let history = client.search_plate("plate".to_string()).await?;
    /// ```
    pub async fn search_plate(
        &mut self,
        plate: String,
    ) -> Result<Vec<response::PlateHistory>, Error>

    /// Searches for every transaction of vehicles owned
    /// or related to a given DNI. The return object is a vector
    /// with PlateHistory structs containing data from owners
    /// and the vehicle.
    ///
    /// This is part of the medium plan or higher.
    ///
    /// ```
    /// let history = client.search_plate_by_dni("42185921".to_string()).await?;
    /// ```
    pub async fn search_plate_by_dni(
        &mut self,
        dni: String,
    ) -> Result<Vec<response::PlateHistory>, Error>

    /// Searches for leaked credentials by a given query string.
    /// Returns a vector with BreachCredentials structs containing
    /// email and password.
    ///
    /// This is part of the medium plan or higher.
    ///
    /// ```
    /// let credentials = client.search_leaks("gov".to_string()).await?;
    /// ```
    pub async fn search_leaks(
        &mut self,
        query: String,
    ) -> Result<Vec<response::BreachCredentials>, Error>

    /// Searches for a person by their DNI.
    /// Returns extra data directly from a high quality source.
    ///
    /// It takes two arguments: dni as String and gender as rusigma::utils::Genero
    /// enum. It is mandatory to know the gender of the person you want to search for.
    ///
    /// Those can be:
    ///     Genero::Masculino
    ///     Genero::Femenino
    ///     Genero::Otro
    ///
    /// This is part of the professional plan.
    ///
    /// ```
    /// let person = client.search_profesional_dni("15341".to_string(), Genero::Masculino).await?;
    /// ```
    pub async fn search_profesional_dni(
        &mut self,
        dni: String,
        gender: utils::Genero,
    ) -> Result<response::DNIProfesional, Error>

    /// Searches for people by their name.
    /// This endpoint can fetch a maximum of 10 people. You can
    /// use filters to find by their province, age range or city.
    ///
    /// The first argumento is the name as String, the second is optional
    /// and can be None or rusigma::utils::SearchFilters
    ///
    /// This is part of the professional plan.
    ///
    /// ```
    /// let mut filters = SearchFilters::new();
    ///
    /// filters.set_provincia(Provincias::BuenosAires);
    /// filters.set_localidad("Some localidad".to_string());
    /// filters.set_edad_minima(20);
    /// filters.set_edad_maxima(40);
    ///
    /// let results = client.search_name("Carlos".to_string(), filters).await?;
    /// ```
    pub async fn search_name(
        &mut self,
        name: String,
        filters: Option<utils::SearchFilters>,
    ) -> Result<Vec<response::PersonaNombre>, Error>

    /// Searches for an email from a given phone number carried
    /// by Movistar company.
    ///
    /// This is part of the professional plan.
    ///
    /// ```
    /// let email = client.search_movistar_email("1487528".to_string()).await?;
    /// ```
    pub async fn search_movistar_email(
        &mut self,
        number: String,
    ) -> Result<response::MovistarEmail, Error>

    /// Searches for all the people living on a given address.
    /// This function returns a vector with PersonaDireccion structs
    /// containing names, DNIs, numbers and carriers of every neighbor.
    ///
    /// This is part of the professional plan.
    ///
    /// ```
    /// let neighbors = client.search_by_address("Niceto vega".to_string()).await?;
    /// ```
    pub async fn search_by_address(
        &mut self,
        address: String,
    ) -> Result<Vec<response::PersonaDireccion>, Error>

    /// Searches for a phone number. It returns data from
    /// the people that owned or are owning the number,
    /// therefore it returns a vector.
    ///
    /// This is part of the professional plan.
    ///
    /// ```
    /// let phoneOwners = client.search_phone("185719512".to_string()).await?;
    /// ```
    pub async fn search_phone(
        &mut self,
        number: String,
    ) -> Result<Vec<response::PersonaFromNumero>, Error>

    /// Searches for a phone number at a special endpoint.
    /// If it returns data from the owner, it means the number
    /// is highly used for personal stuff.
    ///
    /// This endpoint also returns PersonFromNumeroMagic struct
    /// containing an email.
    ///
    /// This is part of the professional plan.
    ///
    /// ```
    /// let person = client.search_phone_magic("1579212".to_string()).await?;
    /// ```
    pub async fn search_phone_magic(
        &mut self,
        number: String,
    ) -> Result<response::PersonaFromNumeroMagic, Error>

    /// Searches for the owner of a CBU or alias.
    ///
    /// This is part of the professional plan.
    ///
    /// ```
    /// let owner = client.search_cbu("99852932692620932".to_string()).await?;
    /// ```
    pub async fn search_cbu(
        &mut self,
        cvu_or_alias: String,
    ) -> Result<response::TitularCBU, Error>

    /// Searches for an email address and gets the owner's name.
    ///
    /// This is part of the professional plan.
    ///
    /// ```
    /// let owner = client.search_email("fuasdhfusda@gmail.com".to_string()).await?;
    /// ```
    pub async fn search_email(
        &mut self,
        email: String,
    ) -> Result<response::EmailResultados, Error>
