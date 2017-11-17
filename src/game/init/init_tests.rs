#[cfg(test)]
mod tests {
  use game::tcod::Console;
  use game::tcod::console;

  use game::init;
  use game::init::config;

  // Test the root console creation
  #[test]
  fn test_root() {

    let cfg = config::load("config/cfg.yml");
    let root = init::root();

    assert_eq!(root.width(), cfg.width);
    assert_eq!(root.height(), cfg.height);
    assert_eq!(root.is_active(), true);
    assert_eq!(root.is_fullscreen(), cfg.fullscreen);

  }

  // Load test should fail on invalid paths
  #[test]
  #[should_panic]
  fn test_bad_path_load() {
    config::load("bad path");
  }

  // Load test should fail on invalid file types
  #[test]
  #[should_panic]
  fn test_bad_file_load() {
    config::load("src/main.rs");
  }

  // Load test should fail on files with valid yml that do not fit the Config struct types
  #[test]
  #[should_panic]
  fn test_bad_types_load() {
    config::load("tests/bad_types.yml");
  }

  // Load test should fail on files without all the proper fields
  #[test]
  #[should_panic]
  fn test_missing_load() {
    config::load("tests/missing.yml");
  }

  // Load test should fail on files with just invalid fields
  #[test]
  #[should_panic]
  fn test_invalid_breaking_load() {
    config::load("tests/invalid_breaking.yml");
  }

  // ... However config::load test should not fail on files with extra invalid fields
  #[test]
  fn test_invalid_nonbreaking_load() {
    config::load("tests/invalid_nonbreaking.yml");
  }

  // Load passes with no problems on proper yml files
  #[test]
  fn test_good_load() {
    config::load("config/cfg.yml");
  }
  
}

