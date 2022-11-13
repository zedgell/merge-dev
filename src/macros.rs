#[macro_export]
macro_rules! add_fields {
     ($b: expr, $n: expr, $s: tt, [$( $x:ident ),*]) => {
         {
             let service = $b;
             let model = $n;
             let mut url = format!("https://api.merge.dev/api/{}/v1/{}", service, model);
             let mut has_params = false;
             $(
                if $s.$x.is_some() {
                    if has_params {
                        url = format!("{}&{}={}", url, stringify!($x), $s.clone().$x.unwrap());
                    } else {
                       url = format!("{}?{}={}", url, stringify!($x), $s.clone().$x.unwrap());
                       has_params = true;
                    }
                }
             )*
             url
         }
     }
}
