#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
mod impls {
    use super::*;
    use frame::prelude::*;
    impl<T: Config> Pallet<T> {
        pub fn mint(owner: T::AccountId) -> DispatchResult {
            Self::deposit_event(Event::<T>::Created { owner });
            Ok(())
        }
    }
}
use frame::prelude::*;
pub use pallet::*;
/**The `pallet` module in each FRAME pallet hosts the most important items needed
to construct this pallet.

The main components of this pallet are:
- [`Pallet`], which implements all of the dispatchable extrinsics of the pallet, among
other public functions.
	- The subset of the functions that are dispatchable can be identified either in the
	[`dispatchables`] module or in the [`Call`] enum.
- [`storage_types`], which contains the list of all types that are representing a
storage item. Otherwise, all storage items are listed among [*Type Definitions*](#types).
- [`Config`], which contains the configuration trait of this pallet.
- [`Event`] and [`Error`], which are listed among the [*Enums*](#enums).
		*/
pub mod pallet {
    use super::*;
    /**
				The `Pallet` struct, the main type that implements traits and standalone
				functions within the pallet.
			*/
    pub struct Pallet<T>(core::marker::PhantomData<T>);
    const _: () = {
        #[automatically_derived]
        impl<T> ::core::clone::Clone for Pallet<T> {
            fn clone(&self) -> Self {
                Self(::core::clone::Clone::clone(&self.0))
            }
        }
    };
    const _: () = {
        impl<T> ::core::cmp::Eq for Pallet<T> {}
    };
    const _: () = {
        #[automatically_derived]
        impl<T> ::core::cmp::PartialEq for Pallet<T> {
            fn eq(&self, other: &Self) -> bool {
                true && self.0 == other.0
            }
        }
    };
    const _: () = {
        #[automatically_derived]
        impl<T> ::core::fmt::Debug for Pallet<T> {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                fmt.debug_tuple("Pallet").field(&self.0).finish()
            }
        }
    };
    /**
Configuration trait of this pallet.

The main purpose of this trait is to act as an interface between this pallet and the runtime in
which it is embedded in. A type, function, or constant in this trait is essentially left to be
configured by the runtime that includes this pallet.

Consequently, a runtime that wants to include this pallet must implement this trait.*/
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>>
            + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }
    ///The `Event` enum of this pallet
    #[scale_info(skip_type_params(T), capture_docs = "always")]
    pub enum Event<T: Config> {
        Created { owner: T::AccountId },
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(::core::marker::PhantomData<(T)>, frame::deps::frame_support::Never),
    }
    const _: () = {
        #[automatically_derived]
        impl<T: Config> ::core::clone::Clone for Event<T> {
            fn clone(&self) -> Self {
                match self {
                    Self::Created { ref owner } => {
                        Self::Created {
                            owner: ::core::clone::Clone::clone(owner),
                        }
                    }
                    Self::__Ignore(ref _0, ref _1) => {
                        Self::__Ignore(
                            ::core::clone::Clone::clone(_0),
                            ::core::clone::Clone::clone(_1),
                        )
                    }
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> ::core::cmp::Eq for Event<T> {}
    };
    const _: () = {
        #[automatically_derived]
        impl<T: Config> ::core::cmp::PartialEq for Event<T> {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (Self::Created { owner }, Self::Created { owner: _0 }) => {
                        true && owner == _0
                    }
                    (Self::__Ignore(_0, _1), Self::__Ignore(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::Created { .. }, Self::__Ignore { .. }) => false,
                    (Self::__Ignore { .. }, Self::Created { .. }) => false,
                }
            }
        }
    };
    const _: () = {
        #[automatically_derived]
        impl<T: Config> ::core::fmt::Debug for Event<T> {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self::Created { ref owner } => {
                        fmt.debug_struct("Event::Created")
                            .field("owner", &owner)
                            .finish()
                    }
                    Self::__Ignore(ref _0, ref _1) => {
                        fmt.debug_tuple("Event::__Ignore").field(&_0).field(&_1).finish()
                    }
                }
            }
        }
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl<T: Config> ::codec::Encode for Event<T>
        where
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
        {
            fn size_hint(&self) -> usize {
                1_usize
                    + match *self {
                        Event::Created { ref owner } => {
                            0_usize.saturating_add(::codec::Encode::size_hint(owner))
                        }
                        _ => 0_usize,
                    }
            }
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Event::Created { ref owner } => {
                        __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                        ::codec::Encode::encode_to(owner, __codec_dest_edqy);
                    }
                    _ => {}
                }
            }
        }
        #[automatically_derived]
        impl<T: Config> ::codec::EncodeLike for Event<T>
        where
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
        {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl<T: Config> ::codec::Decode for Event<T>
        where
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
        {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| {
                        e.chain("Could not decode `Event`, failed to read variant byte")
                    })?
                {
                    #[allow(clippy::unnecessary_cast)]
                    __codec_x_edqy if __codec_x_edqy
                        == 0usize as ::core::primitive::u8 => {
                        #[allow(clippy::redundant_closure_call)]
                        return (move || {
                            ::core::result::Result::Ok(Event::<T>::Created {
                                owner: {
                                    let __codec_res_edqy = <T::AccountId as ::codec::Decode>::decode(
                                        __codec_input_edqy,
                                    );
                                    match __codec_res_edqy {
                                        ::core::result::Result::Err(e) => {
                                            return ::core::result::Result::Err(
                                                e.chain("Could not decode `Event::Created::owner`"),
                                            );
                                        }
                                        ::core::result::Result::Ok(__codec_res_edqy) => {
                                            __codec_res_edqy
                                        }
                                    }
                                },
                            })
                        })();
                    }
                    _ => {
                        #[allow(clippy::redundant_closure_call)]
                        return (move || {
                            ::core::result::Result::Err(
                                <_ as ::core::convert::Into<
                                    _,
                                >>::into("Could not decode `Event`, variant doesn't exist"),
                            )
                        })();
                    }
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl<T: Config> ::scale_info::TypeInfo for Event<T>
        where
            T::AccountId: ::scale_info::TypeInfo + 'static,
            ::core::marker::PhantomData<(T)>: ::scale_info::TypeInfo + 'static,
            T: Config + 'static,
        {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(
                        ::scale_info::Path::new_with_replace(
                            "Event",
                            "pallet_kitties::pallet",
                            &[],
                        ),
                    )
                    .type_params(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                ::scale_info::TypeParameter::new(
                                    "T",
                                    ::core::option::Option::None,
                                ),
                            ]),
                        ),
                    )
                    .docs_always(&["The `Event` enum of this pallet"])
                    .variant(
                        ::scale_info::build::Variants::new()
                            .variant(
                                "Created",
                                |v| {
                                    v
                                        .index(0usize as ::core::primitive::u8)
                                        .fields(
                                            ::scale_info::build::Fields::named()
                                                .field(|f| {
                                                    f
                                                        .ty::<T::AccountId>()
                                                        .name("owner")
                                                        .type_name("T::AccountId")
                                                }),
                                        )
                                },
                            ),
                    )
            }
        }
    };
    #[scale_info(skip_type_params(T), capture_docs = "always")]
    ///The `Error` enum of this pallet.
    pub enum Error<T> {
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(core::marker::PhantomData<(T)>, frame::deps::frame_support::Never),
    }
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl<T> ::codec::Encode for Error<T> {}
        #[automatically_derived]
        impl<T> ::codec::EncodeLike for Error<T> {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl<T> ::codec::Decode for Error<T> {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| {
                        e.chain("Could not decode `Error`, failed to read variant byte")
                    })?
                {
                    _ => {
                        #[allow(clippy::redundant_closure_call)]
                        return (move || {
                            ::core::result::Result::Err(
                                <_ as ::core::convert::Into<
                                    _,
                                >>::into("Could not decode `Error`, variant doesn't exist"),
                            )
                        })();
                    }
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl<T> ::scale_info::TypeInfo for Error<T>
        where
            core::marker::PhantomData<(T)>: ::scale_info::TypeInfo + 'static,
            T: 'static,
        {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(
                        ::scale_info::Path::new_with_replace(
                            "Error",
                            "pallet_kitties::pallet",
                            &[],
                        ),
                    )
                    .type_params(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                ::scale_info::TypeParameter::new(
                                    "T",
                                    ::core::option::Option::None,
                                ),
                            ]),
                        ),
                    )
                    .docs_always(&["The `Error` enum of this pallet."])
                    .variant(::scale_info::build::Variants::new())
            }
        }
    };
    const _: () = {
        impl<T> frame::deps::frame_support::traits::PalletError for Error<T> {
            const MAX_ENCODED_SIZE: usize = 1;
        }
    };
    impl<T: Config> Pallet<T> {
        pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
            frame::deps::frame_support::storage::with_storage_layer::<
                (),
                frame::deps::frame_support::pallet_prelude::DispatchError,
                _,
            >(|| {
                let who = ensure_signed(origin)?;
                Self::mint(who)?;
                Ok(())
            })
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn pallet_documentation_metadata() -> frame::deps::frame_support::__private::Vec<
            &'static str,
        > {
            ::alloc::vec::Vec::new()
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn pallet_constants_metadata() -> frame::deps::frame_support::__private::Vec<
            frame::deps::frame_support::__private::metadata_ir::PalletConstantMetadataIR,
        > {
            ::alloc::vec::Vec::new()
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn error_metadata() -> Option<
            frame::deps::frame_support::__private::metadata_ir::PalletErrorMetadataIR,
        > {
            Some(frame::deps::frame_support::__private::metadata_ir::PalletErrorMetadataIR {
                ty: frame::deps::frame_support::__private::scale_info::meta_type::<
                    Error<T>,
                >(),
            })
        }
    }
    /// Type alias to `Pallet`, to be used by `construct_runtime`.
    ///
    /// Generated by `pallet` attribute macro.
    #[deprecated(note = "use `Pallet` instead")]
    #[allow(dead_code)]
    pub type Module<T> = Pallet<T>;
    impl<T: Config> frame::deps::frame_support::traits::GetStorageVersion for Pallet<T> {
        type InCodeStorageVersion = frame::deps::frame_support::traits::NoStorageVersionSet;
        fn in_code_storage_version() -> Self::InCodeStorageVersion {
            core::default::Default::default()
        }
        fn on_chain_storage_version() -> frame::deps::frame_support::traits::StorageVersion {
            frame::deps::frame_support::traits::StorageVersion::get::<Self>()
        }
    }
    impl<T: Config> frame::deps::frame_support::traits::OnGenesis for Pallet<T> {
        fn on_genesis() {
            let storage_version: frame::deps::frame_support::traits::StorageVersion = core::default::Default::default();
            storage_version.put::<Self>();
        }
    }
    impl<T: Config> frame::deps::frame_support::traits::PalletInfoAccess for Pallet<T> {
        fn index() -> usize {
            <<T as frame::deps::frame_system::Config>::PalletInfo as frame::deps::frame_support::traits::PalletInfo>::index::<
                Self,
            >()
                .expect(
                    "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
                )
        }
        fn name() -> &'static str {
            <<T as frame::deps::frame_system::Config>::PalletInfo as frame::deps::frame_support::traits::PalletInfo>::name::<
                Self,
            >()
                .expect(
                    "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
                )
        }
        fn name_hash() -> [u8; 16] {
            <<T as frame::deps::frame_system::Config>::PalletInfo as frame::deps::frame_support::traits::PalletInfo>::name_hash::<
                Self,
            >()
                .expect(
                    "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
                )
        }
        fn module_name() -> &'static str {
            <<T as frame::deps::frame_system::Config>::PalletInfo as frame::deps::frame_support::traits::PalletInfo>::module_name::<
                Self,
            >()
                .expect(
                    "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
                )
        }
        fn crate_version() -> frame::deps::frame_support::traits::CrateVersion {
            frame::deps::frame_support::traits::CrateVersion {
                major: 0u16,
                minor: 1u8,
                patch: 0u8,
            }
        }
    }
    impl<T: Config> frame::deps::frame_support::traits::PalletsInfoAccess for Pallet<T> {
        fn count() -> usize {
            1
        }
        fn infos() -> frame::deps::frame_support::__private::Vec<
            frame::deps::frame_support::traits::PalletInfoData,
        > {
            use frame::deps::frame_support::traits::PalletInfoAccess;
            let item = frame::deps::frame_support::traits::PalletInfoData {
                index: Self::index(),
                name: Self::name(),
                module_name: Self::module_name(),
                crate_version: Self::crate_version(),
            };
            <[_]>::into_vec(::alloc::boxed::box_new([item]))
        }
    }
    impl<T: Config> frame::deps::frame_support::traits::StorageInfoTrait for Pallet<T> {
        fn storage_info() -> frame::deps::frame_support::__private::Vec<
            frame::deps::frame_support::traits::StorageInfo,
        > {
            #[allow(unused_mut)]
            let mut res = ::alloc::vec::Vec::new();
            res
        }
    }
    use frame::deps::frame_support::traits::{
        StorageInfoTrait, TrackedStorageKey, WhitelistedStorageKeys,
    };
    impl<T: Config> WhitelistedStorageKeys for Pallet<T> {
        fn whitelisted_storage_keys() -> frame::deps::frame_support::__private::Vec<
            TrackedStorageKey,
        > {
            use frame::deps::frame_support::__private::vec;
            ::alloc::vec::Vec::new()
        }
    }
    #[doc(hidden)]
    mod warnings {}
    #[allow(unused_imports)]
    #[doc(hidden)]
    pub mod __substrate_call_check {
        #[doc(hidden)]
        pub use __is_call_part_defined_0 as is_call_part_defined;
    }
    ///Contains a variant per dispatchable extrinsic that this pallet has.
    #[codec(encode_bound())]
    #[codec(decode_bound())]
    #[scale_info(skip_type_params(T), capture_docs = "always")]
    #[allow(non_camel_case_types)]
    pub enum Call<T: Config> {
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(::core::marker::PhantomData<(T,)>, frame::deps::frame_support::Never),
        #[codec(index = 0u8)]
        create_kitty {},
    }
    const _: () = {
        #[automatically_derived]
        impl<T: Config> ::core::fmt::Debug for Call<T> {
            fn fmt(&self, fmt: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self::__Ignore(ref _0, ref _1) => {
                        fmt.debug_tuple("Call::__Ignore").field(&_0).field(&_1).finish()
                    }
                    Self::create_kitty {} => {
                        fmt.debug_struct("Call::create_kitty").finish()
                    }
                }
            }
        }
    };
    const _: () = {
        #[automatically_derived]
        impl<T: Config> ::core::clone::Clone for Call<T> {
            fn clone(&self) -> Self {
                match self {
                    Self::__Ignore(ref _0, ref _1) => {
                        Self::__Ignore(
                            ::core::clone::Clone::clone(_0),
                            ::core::clone::Clone::clone(_1),
                        )
                    }
                    Self::create_kitty {} => Self::create_kitty {},
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> ::core::cmp::Eq for Call<T> {}
    };
    const _: () = {
        #[automatically_derived]
        impl<T: Config> ::core::cmp::PartialEq for Call<T> {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (Self::__Ignore(_0, _1), Self::__Ignore(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::create_kitty {}, Self::create_kitty {}) => true,
                    (Self::__Ignore { .. }, Self::create_kitty { .. }) => false,
                    (Self::create_kitty { .. }, Self::__Ignore { .. }) => false,
                }
            }
        }
    };
    #[allow(deprecated)]
    const _: () = {
        #[allow(non_camel_case_types)]
        #[automatically_derived]
        impl<T: Config> ::codec::Encode for Call<T> {
            fn size_hint(&self) -> usize {
                1_usize
                    + match *self {
                        Call::create_kitty {} => 0_usize,
                        _ => 0_usize,
                    }
            }
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Call::create_kitty {} => {
                        __codec_dest_edqy.push_byte(0u8 as ::core::primitive::u8);
                    }
                    _ => {}
                }
            }
        }
        #[automatically_derived]
        impl<T: Config> ::codec::EncodeLike for Call<T> {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[allow(non_camel_case_types)]
        #[automatically_derived]
        impl<T: Config> ::codec::Decode for Call<T> {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| {
                        e.chain("Could not decode `Call`, failed to read variant byte")
                    })?
                {
                    #[allow(clippy::unnecessary_cast)]
                    __codec_x_edqy if __codec_x_edqy == 0u8 as ::core::primitive::u8 => {
                        #[allow(clippy::redundant_closure_call)]
                        return (move || {
                            ::core::result::Result::Ok(Call::<T>::create_kitty {})
                        })();
                    }
                    _ => {
                        #[allow(clippy::redundant_closure_call)]
                        return (move || {
                            ::core::result::Result::Err(
                                <_ as ::core::convert::Into<
                                    _,
                                >>::into("Could not decode `Call`, variant doesn't exist"),
                            )
                        })();
                    }
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl<T: Config> ::scale_info::TypeInfo for Call<T>
        where
            ::core::marker::PhantomData<(T,)>: ::scale_info::TypeInfo + 'static,
            T: Config + 'static,
        {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(
                        ::scale_info::Path::new_with_replace(
                            "Call",
                            "pallet_kitties::pallet",
                            &[],
                        ),
                    )
                    .type_params(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                ::scale_info::TypeParameter::new(
                                    "T",
                                    ::core::option::Option::None,
                                ),
                            ]),
                        ),
                    )
                    .docs_always(
                        &[
                            "Contains a variant per dispatchable extrinsic that this pallet has.",
                        ],
                    )
                    .variant(
                        ::scale_info::build::Variants::new()
                            .variant(
                                "create_kitty",
                                |v| {
                                    v
                                        .index(0u8 as ::core::primitive::u8)
                                        .fields(::scale_info::build::Fields::named())
                                },
                            ),
                    )
            }
        }
    };
    impl<T: Config> Call<T> {
        ///Create a call with the variant `create_kitty`.
        pub fn new_call_variant_create_kitty() -> Self {
            Self::create_kitty {}
        }
    }
    impl<T: Config> frame::deps::frame_support::dispatch::GetDispatchInfo for Call<T> {
        fn get_dispatch_info(
            &self,
        ) -> frame::deps::frame_support::dispatch::DispatchInfo {
            match *self {
                Self::create_kitty {} => {
                    let __pallet_base_weight = 0;
                    let __pallet_weight = <dyn frame::deps::frame_support::dispatch::WeighData<
                        (),
                    >>::weigh_data(&__pallet_base_weight, ());
                    let __pallet_class = <dyn frame::deps::frame_support::dispatch::ClassifyDispatch<
                        (),
                    >>::classify_dispatch(&__pallet_base_weight, ());
                    let __pallet_pays_fee = <dyn frame::deps::frame_support::dispatch::PaysFee<
                        (),
                    >>::pays_fee(&__pallet_base_weight, ());
                    frame::deps::frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("__Ignore cannot be used"),
                        ),
                    );
                }
            }
        }
    }
    impl<T: Config> frame::deps::frame_support::dispatch::CheckIfFeeless for Call<T> {
        type Origin = frame::deps::frame_system::pallet_prelude::OriginFor<T>;
        #[allow(unused_variables)]
        fn is_feeless(&self, origin: &Self::Origin) -> bool {
            match *self {
                Self::create_kitty {} => false,
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("__Ignore cannot be used"),
                        ),
                    );
                }
            }
        }
    }
    impl<T: Config> frame::deps::frame_support::traits::GetCallName for Call<T> {
        fn get_call_name(&self) -> &'static str {
            match *self {
                Self::create_kitty { .. } => "create_kitty",
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("__PhantomItem cannot be used."),
                        ),
                    );
                }
            }
        }
        fn get_call_names() -> &'static [&'static str] {
            &["create_kitty"]
        }
    }
    impl<T: Config> frame::deps::frame_support::traits::GetCallIndex for Call<T> {
        fn get_call_index(&self) -> u8 {
            match *self {
                Self::create_kitty { .. } => 0u8,
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("__PhantomItem cannot be used."),
                        ),
                    );
                }
            }
        }
        fn get_call_indices() -> &'static [u8] {
            &[0u8]
        }
    }
    impl<T: Config> frame::deps::frame_support::traits::UnfilteredDispatchable
    for Call<T> {
        type RuntimeOrigin = frame::deps::frame_system::pallet_prelude::OriginFor<T>;
        fn dispatch_bypass_filter(
            self,
            origin: Self::RuntimeOrigin,
        ) -> frame::deps::frame_support::dispatch::DispatchResultWithPostInfo {
            frame::deps::frame_support::dispatch_context::run_in_context(|| {
                match self {
                    Self::create_kitty {} => {
                        let __within_span__ = {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "create_kitty",
                                        "pallet_kitties::pallet",
                                        ::tracing::Level::TRACE,
                                        ::core::option::Option::Some("src/lib.rs"),
                                        ::core::option::Option::Some(9u32),
                                        ::core::option::Option::Some("pallet_kitties::pallet"),
                                        ::tracing_core::field::FieldSet::new(
                                            &[],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::SPAN,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let mut interest = ::tracing::subscriber::Interest::never();
                            if ::tracing::Level::TRACE
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::TRACE
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    interest = __CALLSITE.interest();
                                    !interest.is_never()
                                }
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                            {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Span::new(
                                    meta,
                                    &{ meta.fields().value_set(&[]) },
                                )
                            } else {
                                let span = ::tracing::__macro_support::__disabled_span(
                                    __CALLSITE.metadata(),
                                );
                                if match ::tracing::Level::TRACE {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            span.record_all(
                                                &{ __CALLSITE.metadata().fields().value_set(&[]) },
                                            );
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
                                span
                            }
                        };
                        let __tracing_guard__ = __within_span__.enter();
                        <Pallet<T>>::create_kitty(origin)
                            .map(Into::into)
                            .map_err(Into::into)
                    }
                    Self::__Ignore(_, _) => {
                        let _ = origin;
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "internal error: entered unreachable code: {0}",
                                    format_args!("__PhantomItem cannot be used."),
                                ),
                            );
                        };
                    }
                }
            })
        }
    }
    impl<T: Config> frame::deps::frame_support::dispatch::Callable<T> for Pallet<T> {
        type RuntimeCall = Call<T>;
    }
    impl<T: Config> Pallet<T> {
        #[allow(dead_code)]
        #[doc(hidden)]
        pub fn call_functions() -> frame::deps::frame_support::__private::metadata_ir::PalletCallMetadataIR {
            frame::deps::frame_support::__private::scale_info::meta_type::<Call<T>>()
                .into()
        }
    }
    impl<T: Config> core::fmt::Debug for Error<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl<T: Config> Error<T> {
        #[doc(hidden)]
        pub fn as_str(&self) -> &'static str {
            match &self {
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("`__Ignore` can never be constructed"),
                        ),
                    );
                }
            }
        }
    }
    impl<T: Config> From<Error<T>> for &'static str {
        fn from(err: Error<T>) -> &'static str {
            err.as_str()
        }
    }
    impl<T: Config> From<Error<T>>
    for frame::deps::frame_support::sp_runtime::DispatchError {
        fn from(err: Error<T>) -> Self {
            use frame::deps::frame_support::__private::codec::Encode;
            let index = <<T as frame::deps::frame_system::Config>::PalletInfo as frame::deps::frame_support::traits::PalletInfo>::index::<
                Pallet<T>,
            >()
                .expect("Every active module has an index in the runtime; qed") as u8;
            let mut encoded = err.encode();
            encoded.resize(frame::deps::frame_support::MAX_MODULE_ERROR_ENCODED_SIZE, 0);
            frame::deps::frame_support::sp_runtime::DispatchError::Module(frame::deps::frame_support::sp_runtime::ModuleError {
                index,
                error: TryInto::try_into(encoded)
                    .expect(
                        "encoded error is resized to be equal to the maximum encoded error size; qed",
                    ),
                message: Some(err.as_str()),
            })
        }
    }
    pub use __tt_error_token_1 as tt_error_token;
    #[doc(hidden)]
    pub mod __substrate_event_check {
        #[doc(hidden)]
        pub use __is_event_part_defined_2 as is_event_part_defined;
    }
    impl<T: Config> Pallet<T> {
        pub(super) fn deposit_event(event: Event<T>) {
            let event = <<T as Config>::RuntimeEvent as From<Event<T>>>::from(event);
            let event = <<T as Config>::RuntimeEvent as Into<
                <T as frame::deps::frame_system::Config>::RuntimeEvent,
            >>::into(event);
            <frame::deps::frame_system::Pallet<T>>::deposit_event(event)
        }
    }
    impl<T: Config> From<Event<T>> for () {
        fn from(_: Event<T>) {}
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn storage_metadata() -> frame::deps::frame_support::__private::metadata_ir::PalletStorageMetadataIR {
            frame::deps::frame_support::__private::metadata_ir::PalletStorageMetadataIR {
                prefix: <<T as frame::deps::frame_system::Config>::PalletInfo as frame::deps::frame_support::traits::PalletInfo>::name::<
                    Pallet<T>,
                >()
                    .expect(
                        "No name found for the pallet in the runtime! This usually means that the pallet wasn't added to `construct_runtime!`.",
                    ),
                entries: {
                    #[allow(unused_mut)]
                    let mut entries = ::alloc::vec::Vec::new();
                    entries
                },
            }
        }
    }
    #[doc(hidden)]
    pub mod __substrate_inherent_check {
        #[doc(hidden)]
        pub use __is_inherent_part_defined_3 as is_inherent_part_defined;
    }
    /// Hidden instance generated to be internally used when module is used without
    /// instance.
    #[doc(hidden)]
    pub type __InherentHiddenInstance = ();
    impl<
        T: Config,
    > frame::deps::frame_support::traits::Hooks<
        frame::deps::frame_system::pallet_prelude::BlockNumberFor<T>,
    > for Pallet<T> {}
    impl<
        T: Config,
    > frame::deps::frame_support::traits::OnFinalize<
        frame::deps::frame_system::pallet_prelude::BlockNumberFor<T>,
    > for Pallet<T> {
        fn on_finalize(n: frame::deps::frame_system::pallet_prelude::BlockNumberFor<T>) {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_finalize",
                            "pallet_kitties::pallet",
                            ::tracing::Level::TRACE,
                            ::core::option::Option::Some("src/lib.rs"),
                            ::core::option::Option::Some(9u32),
                            ::core::option::Option::Some("pallet_kitties::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = __CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(
                        __CALLSITE.metadata(),
                        interest,
                    )
                {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(
                        __CALLSITE.metadata(),
                    );
                    if match ::tracing::Level::TRACE {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    } <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                span.record_all(
                                    &{ __CALLSITE.metadata().fields().value_set(&[]) },
                                );
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            <Self as frame::deps::frame_support::traits::Hooks<
                frame::deps::frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::on_finalize(n)
        }
    }
    impl<
        T: Config,
    > frame::deps::frame_support::traits::OnIdle<
        frame::deps::frame_system::pallet_prelude::BlockNumberFor<T>,
    > for Pallet<T> {
        fn on_idle(
            n: frame::deps::frame_system::pallet_prelude::BlockNumberFor<T>,
            remaining_weight: frame::deps::frame_support::weights::Weight,
        ) -> frame::deps::frame_support::weights::Weight {
            <Self as frame::deps::frame_support::traits::Hooks<
                frame::deps::frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::on_idle(n, remaining_weight)
        }
    }
    impl<
        T: Config,
    > frame::deps::frame_support::traits::OnPoll<
        frame::deps::frame_system::pallet_prelude::BlockNumberFor<T>,
    > for Pallet<T> {
        fn on_poll(
            n: frame::deps::frame_system::pallet_prelude::BlockNumberFor<T>,
            weight: &mut frame::deps::frame_support::weights::WeightMeter,
        ) {
            <Self as frame::deps::frame_support::traits::Hooks<
                frame::deps::frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::on_poll(n, weight);
        }
    }
    impl<
        T: Config,
    > frame::deps::frame_support::traits::OnInitialize<
        frame::deps::frame_system::pallet_prelude::BlockNumberFor<T>,
    > for Pallet<T> {
        fn on_initialize(
            n: frame::deps::frame_system::pallet_prelude::BlockNumberFor<T>,
        ) -> frame::deps::frame_support::weights::Weight {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_initialize",
                            "pallet_kitties::pallet",
                            ::tracing::Level::TRACE,
                            ::core::option::Option::Some("src/lib.rs"),
                            ::core::option::Option::Some(9u32),
                            ::core::option::Option::Some("pallet_kitties::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = __CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(
                        __CALLSITE.metadata(),
                        interest,
                    )
                {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(
                        __CALLSITE.metadata(),
                    );
                    if match ::tracing::Level::TRACE {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    } <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                span.record_all(
                                    &{ __CALLSITE.metadata().fields().value_set(&[]) },
                                );
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            <Self as frame::deps::frame_support::traits::Hooks<
                frame::deps::frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::on_initialize(n)
        }
    }
    impl<T: Config> frame::deps::frame_support::traits::BeforeAllRuntimeMigrations
    for Pallet<T> {
        fn before_all_runtime_migrations() -> frame::deps::frame_support::weights::Weight {
            use frame::deps::frame_support::traits::{Get, PalletInfoAccess};
            use frame::deps::frame_support::__private::hashing::twox_128;
            use frame::deps::frame_support::storage::unhashed::contains_prefixed_key;
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "before_all",
                            "pallet_kitties::pallet",
                            ::tracing::Level::TRACE,
                            ::core::option::Option::Some("src/lib.rs"),
                            ::core::option::Option::Some(9u32),
                            ::core::option::Option::Some("pallet_kitties::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = __CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(
                        __CALLSITE.metadata(),
                        interest,
                    )
                {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(
                        __CALLSITE.metadata(),
                    );
                    if match ::tracing::Level::TRACE {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    } <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                span.record_all(
                                    &{ __CALLSITE.metadata().fields().value_set(&[]) },
                                );
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            let pallet_hashed_prefix = <Self as PalletInfoAccess>::name_hash();
            let exists = contains_prefixed_key(&pallet_hashed_prefix);
            if !exists {
                let default_version = frame::deps::frame_support::traits::StorageVersion::new(
                    0,
                );
                {
                    let lvl = ::log::Level::Info;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            format_args!(
                                "🐥 New pallet {0:?} detected in the runtime. The pallet has no defined storage version, so the on-chain version is being initialized to {1:?}.",
                                <<T as frame::deps::frame_system::Config>::PalletInfo as frame::deps::frame_support::traits::PalletInfo>::name::<
                                    Self,
                                >()
                                    .unwrap_or("<unknown pallet name>"),
                                default_version,
                            ),
                            lvl,
                            &(
                                frame::deps::frame_support::LOG_TARGET,
                                "pallet_kitties::pallet",
                                ::log::__private_api::loc(),
                            ),
                            (),
                        );
                    }
                };
                default_version.put::<Self>();
                <T as frame::deps::frame_system::Config>::DbWeight::get()
                    .reads_writes(1, 1)
            } else {
                <T as frame::deps::frame_system::Config>::DbWeight::get().reads(1)
            }
        }
    }
    impl<T: Config> frame::deps::frame_support::traits::OnRuntimeUpgrade for Pallet<T> {
        fn on_runtime_upgrade() -> frame::deps::frame_support::weights::Weight {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_runtime_update",
                            "pallet_kitties::pallet",
                            ::tracing::Level::TRACE,
                            ::core::option::Option::Some("src/lib.rs"),
                            ::core::option::Option::Some(9u32),
                            ::core::option::Option::Some("pallet_kitties::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = __CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(
                        __CALLSITE.metadata(),
                        interest,
                    )
                {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(
                        __CALLSITE.metadata(),
                    );
                    if match ::tracing::Level::TRACE {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    } <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                span.record_all(
                                    &{ __CALLSITE.metadata().fields().value_set(&[]) },
                                );
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "✅ no migration for {0}",
                            <<T as frame::deps::frame_system::Config>::PalletInfo as frame::deps::frame_support::traits::PalletInfo>::name::<
                                Self,
                            >()
                                .unwrap_or("<unknown pallet name>"),
                        ),
                        lvl,
                        &(
                            frame::deps::frame_support::LOG_TARGET,
                            "pallet_kitties::pallet",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            <Self as frame::deps::frame_support::traits::Hooks<
                frame::deps::frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::on_runtime_upgrade()
        }
    }
    impl<
        T: Config,
    > frame::deps::frame_support::traits::OffchainWorker<
        frame::deps::frame_system::pallet_prelude::BlockNumberFor<T>,
    > for Pallet<T> {
        fn offchain_worker(
            n: frame::deps::frame_system::pallet_prelude::BlockNumberFor<T>,
        ) {
            <Self as frame::deps::frame_support::traits::Hooks<
                frame::deps::frame_system::pallet_prelude::BlockNumberFor<T>,
            >>::offchain_worker(n)
        }
    }
    impl<T: Config> frame::deps::frame_support::traits::IntegrityTest for Pallet<T> {
        fn integrity_test() {
            frame::deps::frame_support::__private::sp_io::TestExternalities::default()
                .execute_with(|| {
                    <Self as frame::deps::frame_support::traits::Hooks<
                        frame::deps::frame_system::pallet_prelude::BlockNumberFor<T>,
                    >>::integrity_test()
                });
        }
    }
    #[doc(hidden)]
    pub mod __substrate_genesis_config_check {
        #[doc(hidden)]
        pub use __is_genesis_config_defined_4 as is_genesis_config_defined;
        #[doc(hidden)]
        pub use __is_std_enabled_for_genesis_4 as is_std_enabled_for_genesis;
    }
    #[doc(hidden)]
    pub mod __substrate_origin_check {
        #[doc(hidden)]
        pub use __is_origin_part_defined_5 as is_origin_part_defined;
    }
    #[doc(hidden)]
    pub mod __substrate_validate_unsigned_check {
        #[doc(hidden)]
        pub use __is_validate_unsigned_part_defined_6 as is_validate_unsigned_part_defined;
    }
    pub use __tt_default_parts_7 as tt_default_parts;
    pub use __tt_extra_parts_7 as tt_extra_parts;
    pub use __tt_default_parts_v2_7 as tt_default_parts_v2;
}
