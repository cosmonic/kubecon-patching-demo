#![feature(prelude_import)]
//! xkcd-provider capability provider
//!
//!
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use ::xkcd_interface::XkcdReceiver;
use wasmbus_rpc::provider::prelude::*;
use xkcd_interface::Xkcd;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    provider_main(XkcdProviderProvider::default())?;
    ::std::io::_eprint(::core::fmt::Arguments::new_v1(
        &["xkcd-provider provider exiting\n"],
        &[],
    ));
    Ok(())
}
/// xkcd-provider capability provider implementation
#[services(Xckd)]
struct XkcdProviderProvider {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::default::Default for XkcdProviderProvider {
    #[inline]
    fn default() -> XkcdProviderProvider {
        XkcdProviderProvider {}
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for XkcdProviderProvider {
    #[inline]
    fn clone(&self) -> XkcdProviderProvider {
        match *self {
            XkcdProviderProvider {} => XkcdProviderProvider {},
        }
    }
}
impl MessageDispatch for XkcdProviderProvider {
    #[allow(
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn dispatch<'disp__, 'ctx__, 'msg__, 'async_trait>(
        &'disp__ self,
        ctx: &'ctx__ Context,
        message: Message<'msg__>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = std::result::Result<Message<'msg__>, RpcError>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'disp__: 'async_trait,
        'ctx__: 'async_trait,
        'msg__: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) =
                ::core::option::Option::None::<std::result::Result<Message<'msg__>, RpcError>>
            {
                return __ret;
            }
            let __self = self;
            let ctx = ctx;
            let message = message;
            let __ret: std::result::Result<Message<'msg__>, RpcError> = {
                let (trait_name, trait_method) = message
                    .method
                    .rsplit_once('.')
                    .unwrap_or(("_", message.method));
                let message = Message {
                    method: trait_method,
                    arg: message.arg,
                };
                match trait_name {
                    "Xckd" => XckdReceiver::dispatch(__self, ctx, &message).await,
                    _ => Err(RpcError::MethodNotHandled({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", ".", " - unknown method"],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&trait_name),
                                ::core::fmt::ArgumentV1::new_display(&message.method),
                            ],
                        ));
                        res
                    })),
                }
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
impl XckdReceiver for XkcdProviderProvider {}
/// use default implementations of provider message handlers
impl ProviderDispatch for XkcdProviderProvider {}
impl ProviderHandler for XkcdProviderProvider {}
impl Xkcd for XkcdProviderProvider {
    #[allow(
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn get_comic<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        ctx: &'life1 Context,
        arg: &'life2 u32,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = RpcResult<Vec<u8>>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) =
                ::core::option::Option::None::<RpcResult<Vec<u8>>>
            {
                return __ret;
            }
            let __self = self;
            let ctx = ctx;
            let arg = arg;
            let __ret: RpcResult<Vec<u8>> = {
                let url = {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["https://xkcd.com/", "/info.0.json"],
                        &[::core::fmt::ArgumentV1::new_display(&arg)],
                    ));
                    res
                };
                let resp = reqwest::get(url).await.map_err(|e| {
                    RpcError::Other({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Unable to perform get request: "],
                            &[::core::fmt::ArgumentV1::new_display(&e)],
                        ));
                        res
                    })
                });
                if !resp.status().is_success() {
                    return Err(RpcError::InvalidParameter({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Invalid response received. Got status code "],
                            &[::core::fmt::ArgumentV1::new_display(&resp.status())],
                        ));
                        res
                    }));
                }
                let info: XkcdMetadata = resp.json().await.map_err(|e| {
                    RpcError::Other({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Unable to deserialize response: "],
                            &[::core::fmt::ArgumentV1::new_display(&e)],
                        ));
                        res
                    })
                })?;
                let html = {
                    let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["<!DOCTYPE html>\n        <html>\n        <head>\n            <title>Your XKCD comic</title>\n        </head>\n        <body>\n            <h1>" , "</h1>\n            <img src=\"" , "\"/>\n        </body>\n        </html>\n        "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& & info . title) , :: core :: fmt :: ArgumentV1 :: new_display (& & info . img)])) ;
                    res
                };
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
/// Metadata returned as json
/// (this is a subset of the full metadata, but we only need two fields)
struct XkcdMetadata {
    title: String,
    img: String,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for XkcdMetadata {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "title" => _serde::__private::Ok(__Field::__field0),
                        "img" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"title" => _serde::__private::Ok(__Field::__field0),
                        b"img" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<XkcdMetadata>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = XkcdMetadata;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct XkcdMetadata")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 =
                        match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct XkcdMetadata with 2 elements",
                                ));
                            }
                        };
                    let __field1 =
                        match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct XkcdMetadata with 2 elements",
                                ));
                            }
                        };
                    _serde::__private::Ok(XkcdMetadata {
                        title: __field0,
                        img: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                    while let _serde::__private::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        }
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("title"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<String>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("img"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<String>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {
                                let _ = match _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)
                                {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("title") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("img") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(XkcdMetadata {
                        title: __field0,
                        img: __field1,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["title", "img"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "XkcdMetadata",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<XkcdMetadata>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
