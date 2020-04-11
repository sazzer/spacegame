use crate::authentication::{Provider, ProviderName};
use std::boxed::Box;
use std::collections::HashMap;

/// Registry of all known providers
pub struct ProviderRegistry {
  providers: HashMap<ProviderName, Box<dyn Provider>>,
}

impl ProviderRegistry {
  /// Construct the provider registry
  pub fn new(providers: HashMap<ProviderName, Box<dyn Provider>>) -> Self {
    Self {
      providers: providers,
    }
  }

  /// Get the names of all the providers registered with the registry
  pub fn provider_names(&self) -> impl Iterator<Item = &ProviderName> {
    self.providers.keys()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use galvanic_assert::{assert_that, matchers::collection::*};

  struct FakeProvider {}
  impl Provider for FakeProvider {}

  #[test]
  fn test_list_names_empty_registry() {
    let sut = ProviderRegistry::new(HashMap::new());
    let names = sut.provider_names().collect::<Vec<&ProviderName>>();

    assert_that!(&names, contains_in_any_order(vec![]));
  }

  #[test]
  fn test_list_names_populated_registry() {
    let google: ProviderName = "google".parse().unwrap();
    let twitter: ProviderName = "twitter".parse().unwrap();

    let mut providers = HashMap::new();
    providers.insert(
      google.clone(),
      Box::new(FakeProvider {}) as Box<dyn Provider>,
    );
    providers.insert(
      twitter.clone(),
      Box::new(FakeProvider {}) as Box<dyn Provider>,
    );
    let sut = ProviderRegistry::new(providers);
    let names = sut.provider_names().collect::<Vec<&ProviderName>>();

    assert_that!(&names, contains_in_any_order(vec![&google, &twitter]));
  }
}
