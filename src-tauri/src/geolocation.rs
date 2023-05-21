use serde::ser::{SerializeStruct};

pub struct Location {
  city: Option<String>,
  country: Option<String>
}

pub struct Geolocation {
  ip: Option<String>,
  location: Option<Location>,
}

impl serde::Serialize for Location {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    let mut state = serializer.serialize_struct("Location", 2)?;
    state.serialize_field("city", &self.city)?;
    state.serialize_field("country", &self.country)?;
    state.end()
  }
}

impl Geolocation {
  pub fn new() -> Self {
    Self {
      ip: None,
      location: None
    }
  }

  pub async fn get_location(&mut self) -> Option<Location> {
    if let Some(ip) = public_ip::addr().await {
      // Only update if the IP has changed
      if Some(ip.to_string()) != self.ip {
        self.ip = Some(ip.to_string());
        let info = iplocate::lookup(ip).unwrap();
        self.location = Some(Location {
          city: info.geo_ip.city,
          country: info.geo_ip.country
        });
      }
    }

    if self.location.is_none() {
      None
    } else {
      Some(Location { 
        city: self.location.as_ref().unwrap().city.clone(),
        country: self.location.as_ref().unwrap().country.clone()
      })
    }
  }
}
