macro_rules! enum_is {
  ($enum_name:ident, $field:ident) => {
    |t: &$enum_name| {
      match t {
        $enum_name::$field => true,
        _ => false
      }
    }
  };
}

/// tuple enum is field
macro_rules! tuple_enum_is {
  ($enum_name:ident, $field:ident) => {
    |t: &$enum_name| {
      match t {
        $enum_name::$field(_) => true,
        _ => false
      }
    }
  };
//  ($e:ident, $t:ident, $namespace:ident) => {
//    Box::new(|t: &$e| {
//      match t {
//        $namespace::$e::$t(_) => true,
//        _ => false
//      }
//    })
//  };
}

macro_rules! tuple_enum_on {
  ($enum_name:ident, $field:ident, $fnc:expr) => {
    |t: &$enum_name| {
      match t {
        $enum_name::$field(t) => $fnc(t),
        _ => {}
      };
    }
  };
}

macro_rules! enum_on {
  ($enum_name:ident, $field:ident, $fnc:expr) => {
    |t: &$enum_name| {
      match t {
        $enum_name::$field => $fnc(),
        _ => {}
      };
    }
  };
}


macro_rules! tuple_rtd_type_mapping {
  ($rtd_trait:ident, $tg_type:ident, $rtd_type:ident, $(($td_type_field:ident, $tg_type_field:ident, $tg_clz:ident));*;) => (
    |td: Box<$rtd_trait>| {
      match td_types::$rtd_type::of(td.td_name()) {
      $(
        Some(td_types::$rtd_type::$td_type_field) => $tg_type::$tg_type_field($tg_clz::from_json(td.to_json()).expect(&errors::data_fail_with_json(td.to_json())[..])),
      )*
        None => panic!(errors::data_fail_with_json(td.to_json()))
      }
    }
  );
}

macro_rules! rtd_type_mapping {
  ($rtd_trait:ident, $tg_type:ident, $rtd_type:ident, $(($td_type_field:ident, $tg_type_field:ident));*;) => (
    |td: Box<$rtd_trait>| {
      match td_types::$rtd_type::of(td.td_name()) {
      $(
        Some(td_types::$rtd_type::$td_type_field) => $tg_type::$tg_type_field,
      )*
        None => panic!(errors::data_fail_with_json(td.to_json()))
      }
    }
  );
}
