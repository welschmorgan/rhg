use std::{
  fmt::Display,
  ops::{Deref, DerefMut},
  str::FromStr,
};

use rhg_engine_core::{err, ErrorKind};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FilterPart {
  Star,
  Wildcard,
  Exact(String),
  Exclude(String),
}

impl FilterPart {
  pub fn capture<'a>(
    &self,
    expr: &str,
    offs: &mut usize,
    parts: &mut core::iter::Peekable<std::slice::Iter<'a, FilterPart>>,
  ) -> Option<String> {
    match self {
      FilterPart::Wildcard => {
        let ret = expr.chars().nth(*offs).map(|c| c.to_string());
        if *offs < expr.len() {
          *offs += 1
        }
        ret
      }
      FilterPart::Exclude(value) => {
        let mut accu = String::new();
        while *offs < expr.len() {
          let ch = expr.chars().nth(*offs).unwrap();
          let next_part = parts.peek().map(|p| *p);
          let prev_offs = *offs;
          let next_capture = next_part.and_then(|np| (*np).capture(expr, offs, parts));
          *offs = prev_offs;
          if next_part.is_some() && next_capture.is_some() {
            break;
          }
          accu.push(ch);
          *offs += 1;
        }
        if value.eq(&accu) {
          return None;
        }
        Some(accu)
      }
      FilterPart::Star => {
        let mut accu = String::new();
        while *offs < expr.len() {
          let ch = expr.chars().nth(*offs).unwrap();
          let next_part = parts.peek().map(|p| *p);
          let prev_offs = *offs;
          let next_capture = next_part.and_then(|np| (*np).capture(expr, offs, parts));
          *offs = prev_offs;
          if next_part.is_some() && next_capture.is_some() {
            break;
          }
          accu.push(ch);
          *offs += 1;
        }
        Some(accu)
      }
      FilterPart::Exact(q) => {
        if q.len() > (expr.len() - *offs) {
          return None;
        }
        let sub_expr = &expr[*offs..*offs + q.len()];
        if sub_expr.eq(q) {
          *offs += q.len();
          Some(sub_expr.to_string())
        } else {
          None
        }
      }
    }
  }
}

impl Display for FilterPart {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Star => "*".to_string(),
        Self::Exclude(value) => format!("!({})", value),
        Self::Wildcard => "?".to_string(),
        Self::Exact(q) => q.clone(),
      }
    )
  }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Filter(Vec<FilterPart>);

impl Display for Filter {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      self
        .0
        .iter()
        .map(|part| format!("{}", part))
        .collect::<Vec<_>>()
        .join("")
    )
  }
}

impl Filter {
  pub fn matches(&self, s: &str) -> bool {
    self.capture(s).is_some()
  }

  pub fn capture(&self, expr: &str) -> Option<Vec<(FilterPart, String)>> {
    let mut ret = vec![];
    let mut part_it = self.0.iter().peekable();
    let mut offs = 0;
    // println!("capture '{}' on '{}'", self, expr);
    while let Some(part) = part_it.next() {
      // println!("  capture '{}' on '{}'", part, &expr[offs..]);
      match part.capture(expr, &mut offs, &mut part_it) {
        Some(extracted) => {
          //   println!("    -> captured '{}'", extracted);
          ret.push((part.clone(), extracted));
        }
        None => return None,
      };
    }
    if offs < expr.len() {
      return None;
    }
    return Some(ret);
  }
}

impl Deref for Filter {
  type Target = Vec<FilterPart>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Filter {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl AsRef<Vec<FilterPart>> for Filter {
  fn as_ref(&self) -> &Vec<FilterPart> {
    &self.0
  }
}

impl FromStr for Filter {
  type Err = rhg_engine_core::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut f = Filter::default();
    let mut ch_it = s.chars();
    while let Some(mut ch) = ch_it.next() {
      let part = match ch {
        '*' => FilterPart::Star,
        '?' => FilterPart::Wildcard,
        '!' => {
          ch = match ch_it.next() {
            Some('(') => '(',
            Some(ch) => {
              return err!(
                ErrorKind::IO,
                format!(
                  "invalid filter exclusion pattern, expected '!(...)' but got '!{}'",
                  ch
                )
              )
            }
            None => {
              return err!(
                ErrorKind::IO,
                format!("invalid filter exclusion pattern, expected '!(...)' but got '!'")
              )
            }
          };
          let mut accu = String::new();
          let mut found_close_paren = false;
          while let Some(ch) = ch_it.next() {
            if ch == ')' {
              found_close_paren = true;
              break;
            }
            accu.push(ch);
          }
          if !found_close_paren {
            return err!(
              ErrorKind::IO,
              format!(
                "invalid filter exclusion pattern, expected '!(...)' but got '!({}'",
                accu
              )
            );
          }
          FilterPart::Exclude(accu)
        }
        ch => FilterPart::Exact(ch.to_string()),
      };
      if let (Some(FilterPart::Exact(ref mut prev)), FilterPart::Exact(next)) =
        (f.last_mut(), &part)
      {
        prev.push_str(next.as_str());
      } else {
        f.push(part);
      }
    }
    Ok(f)
  }
}

pub fn parse_filter(value: &str) -> std::result::Result<Filter, std::io::Error> {
  value
    .parse::<Filter>()
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))
}

#[cfg(test)]
mod test {
  use crate::FilterPart;

  use super::Filter;

  #[test]
  fn parse() {
    let f = "*".parse::<Filter>().expect("parse");
    assert_eq!(f, Filter(vec![FilterPart::Star]));

    let f = "?".parse::<Filter>().expect("parse");
    assert_eq!(f, Filter(vec![FilterPart::Wildcard]));

    let f = "!(bc)".parse::<Filter>().expect("parse");
    assert_eq!(f, Filter(vec![FilterPart::Exclude("bc".to_string())]));

    let f = "test".parse::<Filter>().expect("parse");
    assert_eq!(f, Filter(vec![FilterPart::Exact("test".to_string())]));
  }

  macro_rules! filter_match {
    ($filter:expr, $against:expr, $expected:expr) => {{
      let f = $filter.parse::<Filter>().expect("failed to parse");
      let cap = f.capture($against);
      assert_eq!(
        cap, $expected,
        "filter does not match, expected '{:?}' to be '{:?}'",
        cap, $expected
      );
      println!(
        "\x1b[0;32m✔\x1b[0m filter {} {} '{}' -> {:?}",
        f,
        match (cap == $expected, $expected.is_some()) {
          (_, true) => format!("matches"),
          (_, false) => format!("does not match"),
        },
        $against,
        cap
      );
    }};
  }

  #[test]
  fn star() {
    filter_match!(
      "abc*",
      "abc",
      Some(vec![
        (FilterPart::Exact(String::from("abc")), String::from("abc")),
        (FilterPart::Star, String::from("")),
      ])
    );
    filter_match!("abc*", "ab", None as Option<Vec<(FilterPart, String)>>);
    filter_match!(
      "abc*",
      "abcd",
      Some(vec![
        (FilterPart::Exact(String::from("abc")), String::from("abc")),
        (FilterPart::Star, String::from("d")),
      ])
    );
    filter_match!(
      "*abc",
      "dabc",
      Some(vec![
        (FilterPart::Star, String::from("d")),
        (FilterPart::Exact(String::from("abc")), String::from("abc")),
      ])
    );
    filter_match!(
      "abc*",
      "abc3",
      Some(vec![
        (FilterPart::Exact(String::from("abc")), String::from("abc")),
        (FilterPart::Star, String::from("3")),
      ])
    );

    filter_match!("abc*abc", "abc3", None as Option<Vec<(FilterPart, String)>>);
    filter_match!(
      "abc*abc",
      "abc3abc",
      Some(vec![
        (FilterPart::Exact(String::from("abc")), String::from("abc")),
        (FilterPart::Star, String::from("3")),
        (FilterPart::Exact(String::from("abc")), String::from("abc")),
      ])
    );
  }

  #[test]
  fn exclude() {
    filter_match!("a!(bc)d", "abcd", None as Option<Vec<(FilterPart, String)>>);
    filter_match!(
      "a!(bc)d",
      "a3d",
      Some(vec![
        (FilterPart::Exact(String::from("a")), String::from("a")),
        (FilterPart::Exclude(String::from("bc")), String::from("3")),
        (FilterPart::Exact(String::from("d")), String::from("d")),
      ])
    );
  }

  #[test]
  fn wildcard() {
    filter_match!("a?c", "ac", None as Option<Vec<(FilterPart, String)>>);
    filter_match!(
      "a?c",
      "abc",
      Some(vec![
        (FilterPart::Exact(String::from("a")), String::from("a")),
        (FilterPart::Wildcard, String::from("b")),
        (FilterPart::Exact(String::from("c")), String::from("c")),
      ])
    );
  }

  #[test]
  fn exact() {
    filter_match!("abc", "ac", None as Option<Vec<(FilterPart, String)>>);
    filter_match!("t", "test.txt", None as Option<Vec<(FilterPart, String)>>);
    filter_match!(
      "abc",
      "abc",
      Some(vec![(
        FilterPart::Exact(String::from("abc")),
        String::from("abc")
      ),])
    );
  }
}
