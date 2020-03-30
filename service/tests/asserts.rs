use assert_json_diff::assert_json_eq_no_panic;
use galvanic_assert::{MatchResultBuilder, Matcher};
use serde_json::Value;
use spacegame_lib::infrastructure::server::test::TestResponse;

pub fn has_status_code<'a>(
  expected: actix_web::http::StatusCode,
) -> Box<dyn Matcher<'a, TestResponse> + 'a> {
  Box::new(move |actual: &TestResponse| {
    let result = MatchResultBuilder::for_("status");
    if actual.status == expected {
      result.matched()
    } else {
      result.failed_comparison(&actual.status, &expected)
    }
  })
}

pub fn has_header<'a, N, V>(
  header_name: N,
  expected_value: V,
) -> Box<dyn Matcher<'a, TestResponse> + 'a>
where
  N: Into<String>,
  V: Into<String>,
{
  let header_name = header_name.into();
  let expected_value = expected_value.into();

  Box::new(move |actual: &TestResponse| {
    let header_name = header_name.clone();
    let expected_value = expected_value.clone();

    let result = MatchResultBuilder::for_(&format!("Header {}", header_name));
    let header = actual.headers.get(header_name);

    match header.map(|v| v.to_str().map(|v| v.to_owned())) {
      None => result.failed_because("Header not present"),
      Some(Err(e)) => result.failed_because(&format!("Header not parseable: {}", e)),
      Some(Ok(v)) if v != expected_value => result.failed_comparison(&v, &expected_value),
      _ => result.matched(),
    }
  })
}

pub fn has_json_body<'a>(expected: Value) -> Box<dyn Matcher<'a, TestResponse> + 'a> {
  Box::new(move |actual: &TestResponse| {
    let result = MatchResultBuilder::for_("body");
    let actual = actual.to_json().unwrap();
    let match_result = assert_json_eq_no_panic(&actual, &expected);

    match match_result {
      Ok(_) => result.matched(),
      Err(err) => result.failed_because(&err),
    }
  })
}
