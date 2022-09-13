#[derive(Default)]
pub struct SearchFilters {
    pub provincia: Option<Provincias>,
    pub localidad: Option<String>,
    pub edad_minima: Option<u8>,
    pub edad_maxima: Option<u8>,
}

#[derive(Copy, Clone)]
pub enum Provincias {
    CapitalFederal = 0,
    BuenosAires = 1,
    Catamarca = 2,
    Chaco = 16,
    Chubut = 17,
    Cordoba = 3,
    Corrientes = 4,
    EntreRios = 5,
    Formosa = 18,
    Jujuy = 6,
    LaPampa = 21,
    LaRioja = 8,
    Mendoza = 7,
    Misiones = 19,
    Neuquen = 20,
    RioNegro = 22,
    Salta = 9,
    SanJuan = 10,
    SanLuis = 11,
    SantaCruz = 23,
    SantaFe = 12,
    SantiagoDelEstero = 13,
    TierraDelFuego = 24,
    Tucuman = 14,
}

#[derive(Copy, Clone)]
pub enum Genero {
    Masculino,
    Femenino,
    Otro,
}

impl SearchFilters {
    pub fn new() -> SearchFilters {
        SearchFilters::default()
    }

    pub fn set_provincia(&mut self, provincia: Provincias) {
        self.provincia = Some(provincia);
    }

    pub fn set_localidad(&mut self, localidad: String) {
        self.localidad = Some(localidad);
    }

    pub fn set_edad_minima(&mut self, edad_min: u8) {
        self.edad_minima = Some(edad_min);
    }

    pub fn set_edad_maxima(&mut self, edad_max: u8) {
        self.edad_maxima = Some(edad_max);
    }

    pub fn to_hashmap(&mut self) -> std::collections::HashMap<&str, String> {
        let mut map = std::collections::HashMap::new();
        self.provincia.as_ref().map_or(false, |p| {
            map.insert("provincia_nombre", (*p as i8).to_string());
            true
        });
        self.localidad.as_ref().map_or(false, |l| {
            map.insert("localidad", l.to_string());
            true
        });
        self.edad_minima.as_ref().map_or(false, |emi| {
            map.insert("edad_desde", emi.to_string());
            true
        });
        self.edad_maxima.as_ref().map_or(false, |ema| {
            map.insert("edad_hasta", ema.to_string());
            true
        });
        map
    }
}
