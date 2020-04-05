use super::*;
use std::collections::HashMap;
use std::sync::Arc;

/// Registry of all the login providers that we can use
#[derive(Clone)]
pub struct ProviderRegistry {
  providers: HashMap<ProviderName, Arc<dyn Provider>>,
}

impl ProviderRegistry {
  /// Construct the Provider Registry
  pub fn new(providers: HashMap<ProviderName, Arc<dyn Provider>>) -> Self {
    ProviderRegistry { providers }
  }

  /// Get the list of providers from the registry
  pub fn list_providers(&self) -> Vec<&ProviderName> {
    self.providers.keys().collect()
  }

  /// Find the Provider that has the given name
  pub fn find_provider(&self, provider_name: &ProviderName) -> Option<Arc<dyn Provider>> {
    self
      .providers
      .get(provider_name)
      .map(|provider| provider.clone())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use async_trait::async_trait;
  use galvanic_assert::{assert_that, matchers::collection::*};

  struct MockProvider {}

  #[async_trait]
  impl Provider for MockProvider {
    fn start(&self) -> StartAuthentication {
      todo!()
    }

    async fn complete(&self, _: HashMap<String, String>) -> Result<String, AuthenticationError> {
      todo!()
    }
  }

  #[test]
  fn test_list_no_providers() {
    let sut = ProviderRegistry::new(HashMap::new());

    let list = sut.list_providers();

    assert_that!(&list, contains_in_any_order(vec![]));
  }

  #[test]
  fn test_list_some_providers() {
    let mock_name: ProviderName = "testing".parse().unwrap();
    let mock = MockProvider {};
    let mut providers = HashMap::new();
    providers.insert(mock_name.clone(), Arc::new(mock) as Arc<dyn Provider>);

    let sut = ProviderRegistry::new(providers);

    let list = sut.list_providers();

    assert_that!(&list, contains_in_any_order(vec![&mock_name]));
  }

  #[test]
  fn test_find_no_provider() {
    let provider_name: ProviderName = "testing".parse().unwrap();
    let sut = ProviderRegistry::new(HashMap::new());

    let result = sut.find_provider(&provider_name);

    assert_eq!(result.is_none(), true);
  }

  #[test]
  fn test_find_provider() {
    let mock_name: ProviderName = "testing".parse().unwrap();
    let mock = MockProvider {};
    let provider: Arc<dyn Provider> = Arc::new(mock);
    let mut providers = HashMap::new();
    providers.insert(mock_name.clone(), provider);

    let sut = ProviderRegistry::new(providers);

    let result = sut.find_provider(&mock_name);
    assert_eq!(result.is_some(), true);
  }
}
