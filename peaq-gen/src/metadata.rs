#[allow(dead_code, unused_imports, non_camel_case_types)]
#[allow(clippy::all)]
#[allow(rustdoc::broken_intra_doc_links)]
pub mod api {
    #[allow(unused_imports)]
    mod root_mod {
        pub use super::*;
    }
    pub static PALLETS: [&str; 4usize] = ["System", "Balances", "PeaqDid", "PeaqRbac"];
    pub static RUNTIME_APIS: [&str; 0usize] = [];
    #[doc = r" The error type returned when there is a runtime issue."]
    pub type DispatchError = runtime_types::sp_runtime::DispatchError;
    #[doc = r" The outer event enum."]
    pub type Event = runtime_types::peaq_dev_runtime::RuntimeEvent;
    #[doc = r" The outer extrinsic enum."]
    pub type Call = runtime_types::peaq_dev_runtime::RuntimeCall;
    #[doc = r" The outer error enum representing the DispatchError's Module variant."]
    pub type Error = runtime_types::peaq_dev_runtime::RuntimeError;
    pub fn constants() -> ConstantsApi {
        ConstantsApi
    }
    pub fn storage() -> StorageApi {
        StorageApi
    }
    pub fn tx() -> TransactionApi {
        TransactionApi
    }
    pub fn apis() -> runtime_apis::RuntimeApi {
        runtime_apis::RuntimeApi
    }
    pub mod runtime_apis {
        use super::root_mod;
        use super::runtime_types;
        use ::subxt::ext::codec::Encode;
        pub struct RuntimeApi;
        impl RuntimeApi {}
    }
    pub fn custom() -> CustomValuesApi {
        CustomValuesApi
    }
    pub struct CustomValuesApi;
    impl CustomValuesApi {}
    pub struct ConstantsApi;
    impl ConstantsApi {
        pub fn system(&self) -> system::constants::ConstantsApi {
            system::constants::ConstantsApi
        }
        pub fn balances(&self) -> balances::constants::ConstantsApi {
            balances::constants::ConstantsApi
        }
    }
    pub struct StorageApi;
    impl StorageApi {
        pub fn system(&self) -> system::storage::StorageApi {
            system::storage::StorageApi
        }
        pub fn balances(&self) -> balances::storage::StorageApi {
            balances::storage::StorageApi
        }
        pub fn peaq_did(&self) -> peaq_did::storage::StorageApi {
            peaq_did::storage::StorageApi
        }
        pub fn peaq_rbac(&self) -> peaq_rbac::storage::StorageApi {
            peaq_rbac::storage::StorageApi
        }
    }
    pub struct TransactionApi;
    impl TransactionApi {
        pub fn system(&self) -> system::calls::TransactionApi {
            system::calls::TransactionApi
        }
        pub fn balances(&self) -> balances::calls::TransactionApi {
            balances::calls::TransactionApi
        }
        pub fn peaq_did(&self) -> peaq_did::calls::TransactionApi {
            peaq_did::calls::TransactionApi
        }
        pub fn peaq_rbac(&self) -> peaq_rbac::calls::TransactionApi {
            peaq_rbac::calls::TransactionApi
        }
    }
    #[doc = r" check whether the metadata provided is aligned with this statically generated code."]
    pub fn is_codegen_valid_for(metadata: &::subxt::Metadata) -> bool {
        let runtime_metadata_hash = metadata
            .hasher()
            .only_these_pallets(&PALLETS)
            .only_these_runtime_apis(&RUNTIME_APIS)
            .hash();
        runtime_metadata_hash
            == [
                33u8, 30u8, 145u8, 226u8, 104u8, 165u8, 13u8, 145u8, 43u8, 57u8, 96u8, 8u8, 90u8,
                73u8, 31u8, 221u8, 128u8, 20u8, 223u8, 176u8, 4u8, 213u8, 71u8, 252u8, 194u8,
                166u8, 105u8, 90u8, 51u8, 74u8, 84u8, 217u8,
            ]
    }
    pub mod system {
        use super::root_mod;
        use super::runtime_types;
        pub type Error = runtime_types::frame_system::pallet::Error;
        pub type Call = runtime_types::frame_system::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Remark {
                    pub remark: remark::Remark,
                }
                pub mod remark {
                    use super::runtime_types;
                    pub type Remark = ::std::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::blocks::StaticExtrinsic for Remark {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "remark";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetHeapPages {
                    pub pages: set_heap_pages::Pages,
                }
                pub mod set_heap_pages {
                    use super::runtime_types;
                    pub type Pages = ::core::primitive::u64;
                }
                impl ::subxt::blocks::StaticExtrinsic for SetHeapPages {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_heap_pages";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetCode {
                    pub code: set_code::Code,
                }
                pub mod set_code {
                    use super::runtime_types;
                    pub type Code = ::std::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::blocks::StaticExtrinsic for SetCode {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_code";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetCodeWithoutChecks {
                    pub code: set_code_without_checks::Code,
                }
                pub mod set_code_without_checks {
                    use super::runtime_types;
                    pub type Code = ::std::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::blocks::StaticExtrinsic for SetCodeWithoutChecks {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_code_without_checks";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetStorage {
                    pub items: set_storage::Items,
                }
                pub mod set_storage {
                    use super::runtime_types;
                    pub type Items = ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>;
                }
                impl ::subxt::blocks::StaticExtrinsic for SetStorage {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_storage";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct KillStorage {
                    pub keys: kill_storage::Keys,
                }
                pub mod kill_storage {
                    use super::runtime_types;
                    pub type Keys = ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>;
                }
                impl ::subxt::blocks::StaticExtrinsic for KillStorage {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "kill_storage";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct KillPrefix {
                    pub prefix: kill_prefix::Prefix,
                    pub subkeys: kill_prefix::Subkeys,
                }
                pub mod kill_prefix {
                    use super::runtime_types;
                    pub type Prefix = ::std::vec::Vec<::core::primitive::u8>;
                    pub type Subkeys = ::core::primitive::u32;
                }
                impl ::subxt::blocks::StaticExtrinsic for KillPrefix {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "kill_prefix";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct RemarkWithEvent {
                    pub remark: remark_with_event::Remark,
                }
                pub mod remark_with_event {
                    use super::runtime_types;
                    pub type Remark = ::std::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::blocks::StaticExtrinsic for RemarkWithEvent {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "remark_with_event";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn remark(
                    &self,
                    remark: types::remark::Remark,
                ) -> ::subxt::tx::Payload<types::Remark> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "remark",
                        types::Remark { remark },
                        [
                            43u8, 126u8, 180u8, 174u8, 141u8, 48u8, 52u8, 125u8, 166u8, 212u8,
                            216u8, 98u8, 100u8, 24u8, 132u8, 71u8, 101u8, 64u8, 246u8, 169u8, 33u8,
                            250u8, 147u8, 208u8, 2u8, 40u8, 129u8, 209u8, 232u8, 207u8, 207u8,
                            13u8,
                        ],
                    )
                }
                pub fn set_heap_pages(
                    &self,
                    pages: types::set_heap_pages::Pages,
                ) -> ::subxt::tx::Payload<types::SetHeapPages> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "set_heap_pages",
                        types::SetHeapPages { pages },
                        [
                            188u8, 191u8, 99u8, 216u8, 219u8, 109u8, 141u8, 50u8, 78u8, 235u8,
                            215u8, 242u8, 195u8, 24u8, 111u8, 76u8, 229u8, 64u8, 99u8, 225u8,
                            134u8, 121u8, 81u8, 209u8, 127u8, 223u8, 98u8, 215u8, 150u8, 70u8,
                            57u8, 147u8,
                        ],
                    )
                }
                pub fn set_code(
                    &self,
                    code: types::set_code::Code,
                ) -> ::subxt::tx::Payload<types::SetCode> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "set_code",
                        types::SetCode { code },
                        [
                            233u8, 248u8, 88u8, 245u8, 28u8, 65u8, 25u8, 169u8, 35u8, 237u8, 19u8,
                            203u8, 136u8, 160u8, 18u8, 3u8, 20u8, 197u8, 81u8, 169u8, 244u8, 188u8,
                            27u8, 147u8, 147u8, 236u8, 65u8, 25u8, 3u8, 143u8, 182u8, 22u8,
                        ],
                    )
                }
                pub fn set_code_without_checks(
                    &self,
                    code: types::set_code_without_checks::Code,
                ) -> ::subxt::tx::Payload<types::SetCodeWithoutChecks> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "set_code_without_checks",
                        types::SetCodeWithoutChecks { code },
                        [
                            82u8, 212u8, 157u8, 44u8, 70u8, 0u8, 143u8, 15u8, 109u8, 109u8, 107u8,
                            157u8, 141u8, 42u8, 169u8, 11u8, 15u8, 186u8, 252u8, 138u8, 10u8,
                            147u8, 15u8, 178u8, 247u8, 229u8, 213u8, 98u8, 207u8, 231u8, 119u8,
                            115u8,
                        ],
                    )
                }
                pub fn set_storage(
                    &self,
                    items: types::set_storage::Items,
                ) -> ::subxt::tx::Payload<types::SetStorage> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "set_storage",
                        types::SetStorage { items },
                        [
                            141u8, 216u8, 52u8, 222u8, 223u8, 136u8, 123u8, 181u8, 19u8, 75u8,
                            163u8, 102u8, 229u8, 189u8, 158u8, 142u8, 95u8, 235u8, 240u8, 49u8,
                            150u8, 76u8, 78u8, 137u8, 126u8, 88u8, 183u8, 88u8, 231u8, 146u8,
                            234u8, 43u8,
                        ],
                    )
                }
                pub fn kill_storage(
                    &self,
                    keys: types::kill_storage::Keys,
                ) -> ::subxt::tx::Payload<types::KillStorage> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "kill_storage",
                        types::KillStorage { keys },
                        [
                            73u8, 63u8, 196u8, 36u8, 144u8, 114u8, 34u8, 213u8, 108u8, 93u8, 209u8,
                            234u8, 153u8, 185u8, 33u8, 91u8, 187u8, 195u8, 223u8, 130u8, 58u8,
                            156u8, 63u8, 47u8, 228u8, 249u8, 216u8, 139u8, 143u8, 177u8, 41u8,
                            35u8,
                        ],
                    )
                }
                pub fn kill_prefix(
                    &self,
                    prefix: types::kill_prefix::Prefix,
                    subkeys: types::kill_prefix::Subkeys,
                ) -> ::subxt::tx::Payload<types::KillPrefix> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "kill_prefix",
                        types::KillPrefix { prefix, subkeys },
                        [
                            184u8, 57u8, 139u8, 24u8, 208u8, 87u8, 108u8, 215u8, 198u8, 189u8,
                            175u8, 242u8, 167u8, 215u8, 97u8, 63u8, 110u8, 166u8, 238u8, 98u8,
                            67u8, 236u8, 111u8, 110u8, 234u8, 81u8, 102u8, 5u8, 182u8, 5u8, 214u8,
                            85u8,
                        ],
                    )
                }
                pub fn remark_with_event(
                    &self,
                    remark: types::remark_with_event::Remark,
                ) -> ::subxt::tx::Payload<types::RemarkWithEvent> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "remark_with_event",
                        types::RemarkWithEvent { remark },
                        [
                            120u8, 120u8, 153u8, 92u8, 184u8, 85u8, 34u8, 2u8, 174u8, 206u8, 105u8,
                            228u8, 233u8, 130u8, 80u8, 246u8, 228u8, 59u8, 234u8, 240u8, 4u8, 49u8,
                            147u8, 170u8, 115u8, 91u8, 149u8, 200u8, 228u8, 181u8, 8u8, 154u8,
                        ],
                    )
                }
            }
        }
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ExtrinsicSuccess {
                pub dispatch_info: extrinsic_success::DispatchInfo,
            }
            pub mod extrinsic_success {
                use super::runtime_types;
                pub type DispatchInfo = runtime_types::frame_support::dispatch::DispatchInfo;
            }
            impl ::subxt::events::StaticEvent for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ExtrinsicFailed {
                pub dispatch_error: extrinsic_failed::DispatchError,
                pub dispatch_info: extrinsic_failed::DispatchInfo,
            }
            pub mod extrinsic_failed {
                use super::runtime_types;
                pub type DispatchError = runtime_types::sp_runtime::DispatchError;
                pub type DispatchInfo = runtime_types::frame_support::dispatch::DispatchInfo;
            }
            impl ::subxt::events::StaticEvent for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct CodeUpdated;
            impl ::subxt::events::StaticEvent for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct NewAccount {
                pub account: new_account::Account,
            }
            pub mod new_account {
                use super::runtime_types;
                pub type Account = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct KilledAccount {
                pub account: killed_account::Account,
            }
            pub mod killed_account {
                use super::runtime_types;
                pub type Account = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Remarked {
                pub sender: remarked::Sender,
                pub hash: remarked::Hash,
            }
            pub mod remarked {
                use super::runtime_types;
                pub type Sender = ::subxt::utils::AccountId32;
                pub type Hash = ::subxt::utils::H256;
            }
            impl ::subxt::events::StaticEvent for Remarked {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "Remarked";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod account {
                    use super::runtime_types;
                    pub type Account = runtime_types::frame_system::AccountInfo<
                        ::core::primitive::u32,
                        runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
                    >;
                    pub type Param0 = ::subxt::utils::AccountId32;
                }
                pub mod extrinsic_count {
                    use super::runtime_types;
                    pub type ExtrinsicCount = ::core::primitive::u32;
                }
                pub mod block_weight {
                    use super::runtime_types;
                    pub type BlockWeight = runtime_types::frame_support::dispatch::PerDispatchClass<
                        runtime_types::sp_weights::weight_v2::Weight,
                    >;
                }
                pub mod all_extrinsics_len {
                    use super::runtime_types;
                    pub type AllExtrinsicsLen = ::core::primitive::u32;
                }
                pub mod block_hash {
                    use super::runtime_types;
                    pub type BlockHash = ::subxt::utils::H256;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod extrinsic_data {
                    use super::runtime_types;
                    pub type ExtrinsicData = ::std::vec::Vec<::core::primitive::u8>;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod number {
                    use super::runtime_types;
                    pub type Number = ::core::primitive::u32;
                }
                pub mod parent_hash {
                    use super::runtime_types;
                    pub type ParentHash = ::subxt::utils::H256;
                }
                pub mod digest {
                    use super::runtime_types;
                    pub type Digest = runtime_types::sp_runtime::generic::digest::Digest;
                }
                pub mod events {
                    use super::runtime_types;
                    pub type Events = ::std::vec::Vec<
                        runtime_types::frame_system::EventRecord<
                            runtime_types::peaq_dev_runtime::RuntimeEvent,
                            ::subxt::utils::H256,
                        >,
                    >;
                }
                pub mod event_count {
                    use super::runtime_types;
                    pub type EventCount = ::core::primitive::u32;
                }
                pub mod event_topics {
                    use super::runtime_types;
                    pub type EventTopics =
                        ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>;
                    pub type Param0 = ::subxt::utils::H256;
                }
                pub mod last_runtime_upgrade {
                    use super::runtime_types;
                    pub type LastRuntimeUpgrade =
                        runtime_types::frame_system::LastRuntimeUpgradeInfo;
                }
                pub mod upgraded_to_u32_ref_count {
                    use super::runtime_types;
                    pub type UpgradedToU32RefCount = ::core::primitive::bool;
                }
                pub mod upgraded_to_triple_ref_count {
                    use super::runtime_types;
                    pub type UpgradedToTripleRefCount = ::core::primitive::bool;
                }
                pub mod execution_phase {
                    use super::runtime_types;
                    pub type ExecutionPhase = runtime_types::frame_system::Phase;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn account_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::account::Account,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Account",
                        vec![],
                        [
                            14u8, 233u8, 115u8, 214u8, 0u8, 109u8, 222u8, 121u8, 162u8, 65u8, 60u8,
                            175u8, 209u8, 79u8, 222u8, 124u8, 22u8, 235u8, 138u8, 176u8, 133u8,
                            124u8, 90u8, 158u8, 85u8, 45u8, 37u8, 174u8, 47u8, 79u8, 47u8, 166u8,
                        ],
                    )
                }
                pub fn account(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::account::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::account::Account,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Account",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            14u8, 233u8, 115u8, 214u8, 0u8, 109u8, 222u8, 121u8, 162u8, 65u8, 60u8,
                            175u8, 209u8, 79u8, 222u8, 124u8, 22u8, 235u8, 138u8, 176u8, 133u8,
                            124u8, 90u8, 158u8, 85u8, 45u8, 37u8, 174u8, 47u8, 79u8, 47u8, 166u8,
                        ],
                    )
                }
                pub fn extrinsic_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::extrinsic_count::ExtrinsicCount,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ExtrinsicCount",
                        vec![],
                        [
                            102u8, 76u8, 236u8, 42u8, 40u8, 231u8, 33u8, 222u8, 123u8, 147u8,
                            153u8, 148u8, 234u8, 203u8, 181u8, 119u8, 6u8, 187u8, 177u8, 199u8,
                            120u8, 47u8, 137u8, 254u8, 96u8, 100u8, 165u8, 182u8, 249u8, 230u8,
                            159u8, 79u8,
                        ],
                    )
                }
                pub fn block_weight(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::block_weight::BlockWeight,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "BlockWeight",
                        vec![],
                        [
                            158u8, 46u8, 228u8, 89u8, 210u8, 214u8, 84u8, 154u8, 50u8, 68u8, 63u8,
                            62u8, 43u8, 42u8, 99u8, 27u8, 54u8, 42u8, 146u8, 44u8, 241u8, 216u8,
                            229u8, 30u8, 216u8, 255u8, 165u8, 238u8, 181u8, 130u8, 36u8, 102u8,
                        ],
                    )
                }
                pub fn all_extrinsics_len(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::all_extrinsics_len::AllExtrinsicsLen,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "AllExtrinsicsLen",
                        vec![],
                        [
                            117u8, 86u8, 61u8, 243u8, 41u8, 51u8, 102u8, 214u8, 137u8, 100u8,
                            243u8, 185u8, 122u8, 174u8, 187u8, 117u8, 86u8, 189u8, 63u8, 135u8,
                            101u8, 218u8, 203u8, 201u8, 237u8, 254u8, 128u8, 183u8, 169u8, 221u8,
                            242u8, 65u8,
                        ],
                    )
                }
                pub fn block_hash_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::block_hash::BlockHash,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "BlockHash",
                        vec![],
                        [
                            217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
                            103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
                            164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
                            202u8, 118u8,
                        ],
                    )
                }
                pub fn block_hash(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::block_hash::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::block_hash::BlockHash,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "BlockHash",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
                            103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
                            164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
                            202u8, 118u8,
                        ],
                    )
                }
                pub fn extrinsic_data_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::extrinsic_data::ExtrinsicData,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ExtrinsicData",
                        vec![],
                        [
                            160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
                            220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
                            128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
                        ],
                    )
                }
                pub fn extrinsic_data(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::extrinsic_data::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::extrinsic_data::ExtrinsicData,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ExtrinsicData",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
                            220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
                            128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
                        ],
                    )
                }
                pub fn number(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::number::Number,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Number",
                        vec![],
                        [
                            30u8, 194u8, 177u8, 90u8, 194u8, 232u8, 46u8, 180u8, 85u8, 129u8, 14u8,
                            9u8, 8u8, 8u8, 23u8, 95u8, 230u8, 5u8, 13u8, 105u8, 125u8, 2u8, 22u8,
                            200u8, 78u8, 93u8, 115u8, 28u8, 150u8, 113u8, 48u8, 53u8,
                        ],
                    )
                }
                pub fn parent_hash(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::parent_hash::ParentHash,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ParentHash",
                        vec![],
                        [
                            26u8, 130u8, 11u8, 216u8, 155u8, 71u8, 128u8, 170u8, 30u8, 153u8, 21u8,
                            192u8, 62u8, 93u8, 137u8, 80u8, 120u8, 81u8, 202u8, 94u8, 248u8, 125u8,
                            71u8, 82u8, 141u8, 229u8, 32u8, 56u8, 73u8, 50u8, 101u8, 78u8,
                        ],
                    )
                }
                pub fn digest(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::digest::Digest,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Digest",
                        vec![],
                        [
                            61u8, 64u8, 237u8, 91u8, 145u8, 232u8, 17u8, 254u8, 181u8, 16u8, 234u8,
                            91u8, 51u8, 140u8, 254u8, 131u8, 98u8, 135u8, 21u8, 37u8, 251u8, 20u8,
                            58u8, 92u8, 123u8, 141u8, 14u8, 227u8, 146u8, 46u8, 222u8, 117u8,
                        ],
                    )
                }
                pub fn events(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::events::Events,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Events",
                        vec![],
                        [
                            227u8, 137u8, 42u8, 198u8, 5u8, 77u8, 97u8, 179u8, 75u8, 205u8, 177u8,
                            246u8, 150u8, 227u8, 38u8, 88u8, 91u8, 59u8, 236u8, 29u8, 254u8, 199u8,
                            170u8, 146u8, 57u8, 82u8, 185u8, 176u8, 40u8, 233u8, 166u8, 46u8,
                        ],
                    )
                }
                pub fn event_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::event_count::EventCount,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "EventCount",
                        vec![],
                        [
                            175u8, 24u8, 252u8, 184u8, 210u8, 167u8, 146u8, 143u8, 164u8, 80u8,
                            151u8, 205u8, 189u8, 189u8, 55u8, 220u8, 47u8, 101u8, 181u8, 33u8,
                            254u8, 131u8, 13u8, 143u8, 3u8, 244u8, 245u8, 45u8, 2u8, 210u8, 79u8,
                            133u8,
                        ],
                    )
                }
                pub fn event_topics_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::event_topics::EventTopics,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "EventTopics",
                        vec![],
                        [
                            40u8, 225u8, 14u8, 75u8, 44u8, 176u8, 76u8, 34u8, 143u8, 107u8, 69u8,
                            133u8, 114u8, 13u8, 172u8, 250u8, 141u8, 73u8, 12u8, 65u8, 217u8, 63u8,
                            120u8, 241u8, 48u8, 106u8, 143u8, 161u8, 128u8, 100u8, 166u8, 59u8,
                        ],
                    )
                }
                pub fn event_topics(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::event_topics::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::event_topics::EventTopics,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "EventTopics",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            40u8, 225u8, 14u8, 75u8, 44u8, 176u8, 76u8, 34u8, 143u8, 107u8, 69u8,
                            133u8, 114u8, 13u8, 172u8, 250u8, 141u8, 73u8, 12u8, 65u8, 217u8, 63u8,
                            120u8, 241u8, 48u8, 106u8, 143u8, 161u8, 128u8, 100u8, 166u8, 59u8,
                        ],
                    )
                }
                pub fn last_runtime_upgrade(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::last_runtime_upgrade::LastRuntimeUpgrade,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "LastRuntimeUpgrade",
                        vec![],
                        [
                            137u8, 29u8, 175u8, 75u8, 197u8, 208u8, 91u8, 207u8, 156u8, 87u8,
                            148u8, 68u8, 91u8, 140u8, 22u8, 233u8, 1u8, 229u8, 56u8, 34u8, 40u8,
                            194u8, 253u8, 30u8, 163u8, 39u8, 54u8, 209u8, 13u8, 27u8, 139u8, 184u8,
                        ],
                    )
                }
                pub fn upgraded_to_u32_ref_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::upgraded_to_u32_ref_count::UpgradedToU32RefCount,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "UpgradedToU32RefCount",
                        vec![],
                        [
                            229u8, 73u8, 9u8, 132u8, 186u8, 116u8, 151u8, 171u8, 145u8, 29u8, 34u8,
                            130u8, 52u8, 146u8, 124u8, 175u8, 79u8, 189u8, 147u8, 230u8, 234u8,
                            107u8, 124u8, 31u8, 2u8, 22u8, 86u8, 190u8, 4u8, 147u8, 50u8, 245u8,
                        ],
                    )
                }
                pub fn upgraded_to_triple_ref_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::upgraded_to_triple_ref_count::UpgradedToTripleRefCount,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "UpgradedToTripleRefCount",
                        vec![],
                        [
                            97u8, 66u8, 124u8, 243u8, 27u8, 167u8, 147u8, 81u8, 254u8, 201u8,
                            101u8, 24u8, 40u8, 231u8, 14u8, 179u8, 154u8, 163u8, 71u8, 81u8, 185u8,
                            167u8, 82u8, 254u8, 189u8, 3u8, 101u8, 207u8, 206u8, 194u8, 155u8,
                            151u8,
                        ],
                    )
                }
                pub fn execution_phase(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::execution_phase::ExecutionPhase,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ExecutionPhase",
                        vec![],
                        [
                            191u8, 129u8, 100u8, 134u8, 126u8, 116u8, 154u8, 203u8, 220u8, 200u8,
                            0u8, 26u8, 161u8, 250u8, 133u8, 205u8, 146u8, 24u8, 5u8, 156u8, 158u8,
                            35u8, 36u8, 253u8, 52u8, 235u8, 86u8, 167u8, 35u8, 100u8, 119u8, 27u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn block_weights(
                    &self,
                ) -> ::subxt::constants::Address<runtime_types::frame_system::limits::BlockWeights>
                {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "BlockWeights",
                        [
                            176u8, 124u8, 225u8, 136u8, 25u8, 73u8, 247u8, 33u8, 82u8, 206u8, 85u8,
                            190u8, 127u8, 102u8, 71u8, 11u8, 185u8, 8u8, 58u8, 0u8, 94u8, 55u8,
                            163u8, 177u8, 104u8, 59u8, 60u8, 136u8, 246u8, 116u8, 0u8, 239u8,
                        ],
                    )
                }
                pub fn block_length(
                    &self,
                ) -> ::subxt::constants::Address<runtime_types::frame_system::limits::BlockLength>
                {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "BlockLength",
                        [
                            23u8, 242u8, 225u8, 39u8, 225u8, 67u8, 152u8, 41u8, 155u8, 104u8, 68u8,
                            229u8, 185u8, 133u8, 10u8, 143u8, 184u8, 152u8, 234u8, 44u8, 140u8,
                            96u8, 166u8, 235u8, 162u8, 160u8, 72u8, 7u8, 35u8, 194u8, 3u8, 37u8,
                        ],
                    )
                }
                pub fn block_hash_count(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "BlockHashCount",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn db_weight(
                    &self,
                ) -> ::subxt::constants::Address<runtime_types::sp_weights::RuntimeDbWeight>
                {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "DbWeight",
                        [
                            42u8, 43u8, 178u8, 142u8, 243u8, 203u8, 60u8, 173u8, 118u8, 111u8,
                            200u8, 170u8, 102u8, 70u8, 237u8, 187u8, 198u8, 120u8, 153u8, 232u8,
                            183u8, 76u8, 74u8, 10u8, 70u8, 243u8, 14u8, 218u8, 213u8, 126u8, 29u8,
                            177u8,
                        ],
                    )
                }
                pub fn version(
                    &self,
                ) -> ::subxt::constants::Address<runtime_types::sp_version::RuntimeVersion>
                {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "Version",
                        [
                            219u8, 45u8, 162u8, 245u8, 177u8, 246u8, 48u8, 126u8, 191u8, 157u8,
                            228u8, 83u8, 111u8, 133u8, 183u8, 13u8, 148u8, 108u8, 92u8, 102u8,
                            72u8, 205u8, 74u8, 242u8, 233u8, 79u8, 20u8, 170u8, 72u8, 202u8, 158u8,
                            165u8,
                        ],
                    )
                }
                pub fn ss58_prefix(&self) -> ::subxt::constants::Address<::core::primitive::u16> {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "SS58Prefix",
                        [
                            116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
                            41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
                            90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod balances {
        use super::root_mod;
        use super::runtime_types;
        pub type Error = runtime_types::pallet_balances::pallet::Error;
        pub type Call = runtime_types::pallet_balances::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct TransferAllowDeath {
                    pub dest: transfer_allow_death::Dest,
                    #[codec(compact)]
                    pub value: transfer_allow_death::Value,
                }
                pub mod transfer_allow_death {
                    use super::runtime_types;
                    pub type Dest = ::subxt::utils::MultiAddress<
                        ::subxt::utils::AccountId32,
                        ::core::primitive::u32,
                    >;
                    pub type Value = ::core::primitive::u128;
                }
                impl ::subxt::blocks::StaticExtrinsic for TransferAllowDeath {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_allow_death";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetBalanceDeprecated {
                    pub who: set_balance_deprecated::Who,
                    #[codec(compact)]
                    pub new_free: set_balance_deprecated::NewFree,
                    #[codec(compact)]
                    pub old_reserved: set_balance_deprecated::OldReserved,
                }
                pub mod set_balance_deprecated {
                    use super::runtime_types;
                    pub type Who = ::subxt::utils::MultiAddress<
                        ::subxt::utils::AccountId32,
                        ::core::primitive::u32,
                    >;
                    pub type NewFree = ::core::primitive::u128;
                    pub type OldReserved = ::core::primitive::u128;
                }
                impl ::subxt::blocks::StaticExtrinsic for SetBalanceDeprecated {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "set_balance_deprecated";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ForceTransfer {
                    pub source: force_transfer::Source,
                    pub dest: force_transfer::Dest,
                    #[codec(compact)]
                    pub value: force_transfer::Value,
                }
                pub mod force_transfer {
                    use super::runtime_types;
                    pub type Source = ::subxt::utils::MultiAddress<
                        ::subxt::utils::AccountId32,
                        ::core::primitive::u32,
                    >;
                    pub type Dest = ::subxt::utils::MultiAddress<
                        ::subxt::utils::AccountId32,
                        ::core::primitive::u32,
                    >;
                    pub type Value = ::core::primitive::u128;
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceTransfer {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_transfer";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct TransferKeepAlive {
                    pub dest: transfer_keep_alive::Dest,
                    #[codec(compact)]
                    pub value: transfer_keep_alive::Value,
                }
                pub mod transfer_keep_alive {
                    use super::runtime_types;
                    pub type Dest = ::subxt::utils::MultiAddress<
                        ::subxt::utils::AccountId32,
                        ::core::primitive::u32,
                    >;
                    pub type Value = ::core::primitive::u128;
                }
                impl ::subxt::blocks::StaticExtrinsic for TransferKeepAlive {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_keep_alive";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct TransferAll {
                    pub dest: transfer_all::Dest,
                    pub keep_alive: transfer_all::KeepAlive,
                }
                pub mod transfer_all {
                    use super::runtime_types;
                    pub type Dest = ::subxt::utils::MultiAddress<
                        ::subxt::utils::AccountId32,
                        ::core::primitive::u32,
                    >;
                    pub type KeepAlive = ::core::primitive::bool;
                }
                impl ::subxt::blocks::StaticExtrinsic for TransferAll {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_all";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ForceUnreserve {
                    pub who: force_unreserve::Who,
                    pub amount: force_unreserve::Amount,
                }
                pub mod force_unreserve {
                    use super::runtime_types;
                    pub type Who = ::subxt::utils::MultiAddress<
                        ::subxt::utils::AccountId32,
                        ::core::primitive::u32,
                    >;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceUnreserve {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_unreserve";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UpgradeAccounts {
                    pub who: upgrade_accounts::Who,
                }
                pub mod upgrade_accounts {
                    use super::runtime_types;
                    pub type Who = ::std::vec::Vec<::subxt::utils::AccountId32>;
                }
                impl ::subxt::blocks::StaticExtrinsic for UpgradeAccounts {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "upgrade_accounts";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Transfer {
                    pub dest: transfer::Dest,
                    #[codec(compact)]
                    pub value: transfer::Value,
                }
                pub mod transfer {
                    use super::runtime_types;
                    pub type Dest = ::subxt::utils::MultiAddress<
                        ::subxt::utils::AccountId32,
                        ::core::primitive::u32,
                    >;
                    pub type Value = ::core::primitive::u128;
                }
                impl ::subxt::blocks::StaticExtrinsic for Transfer {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ForceSetBalance {
                    pub who: force_set_balance::Who,
                    #[codec(compact)]
                    pub new_free: force_set_balance::NewFree,
                }
                pub mod force_set_balance {
                    use super::runtime_types;
                    pub type Who = ::subxt::utils::MultiAddress<
                        ::subxt::utils::AccountId32,
                        ::core::primitive::u32,
                    >;
                    pub type NewFree = ::core::primitive::u128;
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceSetBalance {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_set_balance";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn transfer_allow_death(
                    &self,
                    dest: types::transfer_allow_death::Dest,
                    value: types::transfer_allow_death::Value,
                ) -> ::subxt::tx::Payload<types::TransferAllowDeath> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "transfer_allow_death",
                        types::TransferAllowDeath { dest, value },
                        [
                            24u8, 176u8, 111u8, 60u8, 103u8, 161u8, 139u8, 10u8, 197u8, 207u8,
                            140u8, 212u8, 166u8, 50u8, 47u8, 150u8, 83u8, 180u8, 86u8, 4u8, 159u8,
                            84u8, 195u8, 71u8, 204u8, 109u8, 233u8, 23u8, 10u8, 156u8, 209u8,
                            153u8,
                        ],
                    )
                }
                pub fn set_balance_deprecated(
                    &self,
                    who: types::set_balance_deprecated::Who,
                    new_free: types::set_balance_deprecated::NewFree,
                    old_reserved: types::set_balance_deprecated::OldReserved,
                ) -> ::subxt::tx::Payload<types::SetBalanceDeprecated> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "set_balance_deprecated",
                        types::SetBalanceDeprecated {
                            who,
                            new_free,
                            old_reserved,
                        },
                        [
                            37u8, 252u8, 153u8, 117u8, 175u8, 86u8, 206u8, 115u8, 114u8, 75u8,
                            226u8, 183u8, 222u8, 208u8, 208u8, 16u8, 90u8, 82u8, 101u8, 241u8,
                            82u8, 161u8, 116u8, 175u8, 200u8, 113u8, 154u8, 220u8, 224u8, 245u8,
                            112u8, 215u8,
                        ],
                    )
                }
                pub fn force_transfer(
                    &self,
                    source: types::force_transfer::Source,
                    dest: types::force_transfer::Dest,
                    value: types::force_transfer::Value,
                ) -> ::subxt::tx::Payload<types::ForceTransfer> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "force_transfer",
                        types::ForceTransfer {
                            source,
                            dest,
                            value,
                        },
                        [
                            23u8, 7u8, 44u8, 138u8, 180u8, 140u8, 216u8, 52u8, 198u8, 3u8, 225u8,
                            116u8, 47u8, 26u8, 61u8, 163u8, 55u8, 64u8, 113u8, 250u8, 192u8, 16u8,
                            228u8, 228u8, 85u8, 255u8, 100u8, 128u8, 245u8, 132u8, 84u8, 186u8,
                        ],
                    )
                }
                pub fn transfer_keep_alive(
                    &self,
                    dest: types::transfer_keep_alive::Dest,
                    value: types::transfer_keep_alive::Value,
                ) -> ::subxt::tx::Payload<types::TransferKeepAlive> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "transfer_keep_alive",
                        types::TransferKeepAlive { dest, value },
                        [
                            196u8, 51u8, 121u8, 239u8, 68u8, 97u8, 174u8, 26u8, 21u8, 9u8, 111u8,
                            224u8, 189u8, 35u8, 106u8, 30u8, 83u8, 184u8, 234u8, 174u8, 27u8,
                            197u8, 40u8, 126u8, 197u8, 92u8, 201u8, 253u8, 144u8, 175u8, 8u8,
                            215u8,
                        ],
                    )
                }
                pub fn transfer_all(
                    &self,
                    dest: types::transfer_all::Dest,
                    keep_alive: types::transfer_all::KeepAlive,
                ) -> ::subxt::tx::Payload<types::TransferAll> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "transfer_all",
                        types::TransferAll { dest, keep_alive },
                        [
                            13u8, 46u8, 127u8, 231u8, 179u8, 61u8, 45u8, 188u8, 195u8, 251u8,
                            146u8, 25u8, 138u8, 19u8, 52u8, 112u8, 148u8, 241u8, 134u8, 145u8,
                            97u8, 9u8, 199u8, 172u8, 229u8, 239u8, 67u8, 185u8, 128u8, 36u8, 134u8,
                            122u8,
                        ],
                    )
                }
                pub fn force_unreserve(
                    &self,
                    who: types::force_unreserve::Who,
                    amount: types::force_unreserve::Amount,
                ) -> ::subxt::tx::Payload<types::ForceUnreserve> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "force_unreserve",
                        types::ForceUnreserve { who, amount },
                        [
                            176u8, 105u8, 20u8, 111u8, 49u8, 253u8, 22u8, 225u8, 0u8, 81u8, 221u8,
                            39u8, 62u8, 22u8, 95u8, 12u8, 21u8, 251u8, 179u8, 31u8, 104u8, 23u8,
                            34u8, 216u8, 119u8, 205u8, 133u8, 196u8, 182u8, 113u8, 36u8, 93u8,
                        ],
                    )
                }
                pub fn upgrade_accounts(
                    &self,
                    who: types::upgrade_accounts::Who,
                ) -> ::subxt::tx::Payload<types::UpgradeAccounts> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "upgrade_accounts",
                        types::UpgradeAccounts { who },
                        [
                            66u8, 200u8, 179u8, 104u8, 65u8, 2u8, 101u8, 56u8, 130u8, 161u8, 224u8,
                            233u8, 255u8, 124u8, 70u8, 122u8, 8u8, 49u8, 103u8, 178u8, 68u8, 47u8,
                            214u8, 166u8, 217u8, 116u8, 178u8, 50u8, 212u8, 164u8, 98u8, 226u8,
                        ],
                    )
                }
                pub fn transfer(
                    &self,
                    dest: types::transfer::Dest,
                    value: types::transfer::Value,
                ) -> ::subxt::tx::Payload<types::Transfer> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "transfer",
                        types::Transfer { dest, value },
                        [
                            228u8, 253u8, 44u8, 208u8, 33u8, 44u8, 33u8, 42u8, 114u8, 57u8, 107u8,
                            6u8, 127u8, 116u8, 15u8, 205u8, 122u8, 172u8, 64u8, 108u8, 169u8,
                            241u8, 190u8, 221u8, 248u8, 171u8, 236u8, 129u8, 120u8, 147u8, 49u8,
                            95u8,
                        ],
                    )
                }
                pub fn force_set_balance(
                    &self,
                    who: types::force_set_balance::Who,
                    new_free: types::force_set_balance::NewFree,
                ) -> ::subxt::tx::Payload<types::ForceSetBalance> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "force_set_balance",
                        types::ForceSetBalance { who, new_free },
                        [
                            101u8, 181u8, 86u8, 32u8, 61u8, 75u8, 34u8, 164u8, 142u8, 250u8, 7u8,
                            218u8, 125u8, 57u8, 98u8, 222u8, 147u8, 26u8, 115u8, 185u8, 190u8,
                            172u8, 12u8, 212u8, 132u8, 80u8, 253u8, 69u8, 26u8, 116u8, 197u8,
                            203u8,
                        ],
                    )
                }
            }
        }
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Endowed {
                pub account: endowed::Account,
                pub free_balance: endowed::FreeBalance,
            }
            pub mod endowed {
                use super::runtime_types;
                pub type Account = ::subxt::utils::AccountId32;
                pub type FreeBalance = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct DustLost {
                pub account: dust_lost::Account,
                pub amount: dust_lost::Amount,
            }
            pub mod dust_lost {
                use super::runtime_types;
                pub type Account = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Transfer {
                pub from: transfer::From,
                pub to: transfer::To,
                pub amount: transfer::Amount,
            }
            pub mod transfer {
                use super::runtime_types;
                pub type From = ::subxt::utils::AccountId32;
                pub type To = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct BalanceSet {
                pub who: balance_set::Who,
                pub free: balance_set::Free,
            }
            pub mod balance_set {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Free = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Reserved {
                pub who: reserved::Who,
                pub amount: reserved::Amount,
            }
            pub mod reserved {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Unreserved {
                pub who: unreserved::Who,
                pub amount: unreserved::Amount,
            }
            pub mod unreserved {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ReserveRepatriated {
                pub from: reserve_repatriated::From,
                pub to: reserve_repatriated::To,
                pub amount: reserve_repatriated::Amount,
                pub destination_status: reserve_repatriated::DestinationStatus,
            }
            pub mod reserve_repatriated {
                use super::runtime_types;
                pub type From = ::subxt::utils::AccountId32;
                pub type To = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
                pub type DestinationStatus =
                    runtime_types::frame_support::traits::tokens::misc::BalanceStatus;
            }
            impl ::subxt::events::StaticEvent for ReserveRepatriated {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "ReserveRepatriated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Deposit {
                pub who: deposit::Who,
                pub amount: deposit::Amount,
            }
            pub mod deposit {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Withdraw {
                pub who: withdraw::Who,
                pub amount: withdraw::Amount,
            }
            pub mod withdraw {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Withdraw {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Withdraw";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Slashed {
                pub who: slashed::Who,
                pub amount: slashed::Amount,
            }
            pub mod slashed {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Slashed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Slashed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Minted {
                pub who: minted::Who,
                pub amount: minted::Amount,
            }
            pub mod minted {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Minted {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Minted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Burned {
                pub who: burned::Who,
                pub amount: burned::Amount,
            }
            pub mod burned {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Burned {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Burned";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Suspended {
                pub who: suspended::Who,
                pub amount: suspended::Amount,
            }
            pub mod suspended {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Suspended {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Suspended";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Restored {
                pub who: restored::Who,
                pub amount: restored::Amount,
            }
            pub mod restored {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Restored {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Restored";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Upgraded {
                pub who: upgraded::Who,
            }
            pub mod upgraded {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for Upgraded {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Upgraded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Issued {
                pub amount: issued::Amount,
            }
            pub mod issued {
                use super::runtime_types;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Issued {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Issued";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Rescinded {
                pub amount: rescinded::Amount,
            }
            pub mod rescinded {
                use super::runtime_types;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Rescinded {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Rescinded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Locked {
                pub who: locked::Who,
                pub amount: locked::Amount,
            }
            pub mod locked {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Locked {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Locked";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Unlocked {
                pub who: unlocked::Who,
                pub amount: unlocked::Amount,
            }
            pub mod unlocked {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Unlocked {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unlocked";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Frozen {
                pub who: frozen::Who,
                pub amount: frozen::Amount,
            }
            pub mod frozen {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Frozen {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Frozen";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Thawed {
                pub who: thawed::Who,
                pub amount: thawed::Amount,
            }
            pub mod thawed {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Thawed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Thawed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod total_issuance {
                    use super::runtime_types;
                    pub type TotalIssuance = ::core::primitive::u128;
                }
                pub mod inactive_issuance {
                    use super::runtime_types;
                    pub type InactiveIssuance = ::core::primitive::u128;
                }
                pub mod account {
                    use super::runtime_types;
                    pub type Account =
                        runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>;
                    pub type Param0 = ::subxt::utils::AccountId32;
                }
                pub mod locks {
                    use super::runtime_types;
                    pub type Locks =
                        runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                            runtime_types::pallet_balances::types::BalanceLock<
                                ::core::primitive::u128,
                            >,
                        >;
                    pub type Param0 = ::subxt::utils::AccountId32;
                }
                pub mod reserves {
                    use super::runtime_types;
                    pub type Reserves = runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::ReserveData<
                            [::core::primitive::u8; 8usize],
                            ::core::primitive::u128,
                        >,
                    >;
                    pub type Param0 = ::subxt::utils::AccountId32;
                }
                pub mod holds {
                    use super::runtime_types;
                    pub type Holds = runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::IdAmount<
                            (),
                            ::core::primitive::u128,
                        >,
                    >;
                    pub type Param0 = ::subxt::utils::AccountId32;
                }
                pub mod freezes {
                    use super::runtime_types;
                    pub type Freezes = runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::IdAmount<
                            (),
                            ::core::primitive::u128,
                        >,
                    >;
                    pub type Param0 = ::subxt::utils::AccountId32;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn total_issuance(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::total_issuance::TotalIssuance,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "TotalIssuance",
                        vec![],
                        [
                            116u8, 70u8, 119u8, 194u8, 69u8, 37u8, 116u8, 206u8, 171u8, 70u8,
                            171u8, 210u8, 226u8, 111u8, 184u8, 204u8, 206u8, 11u8, 68u8, 72u8,
                            255u8, 19u8, 194u8, 11u8, 27u8, 194u8, 81u8, 204u8, 59u8, 224u8, 202u8,
                            185u8,
                        ],
                    )
                }
                pub fn inactive_issuance(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::inactive_issuance::InactiveIssuance,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "InactiveIssuance",
                        vec![],
                        [
                            212u8, 185u8, 19u8, 50u8, 250u8, 72u8, 173u8, 50u8, 4u8, 104u8, 161u8,
                            249u8, 77u8, 247u8, 204u8, 248u8, 11u8, 18u8, 57u8, 4u8, 82u8, 110u8,
                            30u8, 216u8, 16u8, 37u8, 87u8, 67u8, 189u8, 235u8, 214u8, 155u8,
                        ],
                    )
                }
                pub fn account_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::account::Account,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Account",
                        vec![],
                        [
                            213u8, 38u8, 200u8, 69u8, 218u8, 0u8, 112u8, 181u8, 160u8, 23u8, 96u8,
                            90u8, 3u8, 88u8, 126u8, 22u8, 103u8, 74u8, 64u8, 69u8, 29u8, 247u8,
                            18u8, 17u8, 234u8, 143u8, 189u8, 22u8, 247u8, 194u8, 154u8, 249u8,
                        ],
                    )
                }
                pub fn account(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::account::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::account::Account,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Account",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            213u8, 38u8, 200u8, 69u8, 218u8, 0u8, 112u8, 181u8, 160u8, 23u8, 96u8,
                            90u8, 3u8, 88u8, 126u8, 22u8, 103u8, 74u8, 64u8, 69u8, 29u8, 247u8,
                            18u8, 17u8, 234u8, 143u8, 189u8, 22u8, 247u8, 194u8, 154u8, 249u8,
                        ],
                    )
                }
                pub fn locks_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::locks::Locks,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Locks",
                        vec![],
                        [
                            10u8, 223u8, 55u8, 0u8, 249u8, 69u8, 168u8, 41u8, 75u8, 35u8, 120u8,
                            167u8, 18u8, 132u8, 9u8, 20u8, 91u8, 51u8, 27u8, 69u8, 136u8, 187u8,
                            13u8, 220u8, 163u8, 122u8, 26u8, 141u8, 174u8, 249u8, 85u8, 37u8,
                        ],
                    )
                }
                pub fn locks(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::locks::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::locks::Locks,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Locks",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            10u8, 223u8, 55u8, 0u8, 249u8, 69u8, 168u8, 41u8, 75u8, 35u8, 120u8,
                            167u8, 18u8, 132u8, 9u8, 20u8, 91u8, 51u8, 27u8, 69u8, 136u8, 187u8,
                            13u8, 220u8, 163u8, 122u8, 26u8, 141u8, 174u8, 249u8, 85u8, 37u8,
                        ],
                    )
                }
                pub fn reserves_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::reserves::Reserves,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Reserves",
                        vec![],
                        [
                            112u8, 10u8, 241u8, 77u8, 64u8, 187u8, 106u8, 159u8, 13u8, 153u8,
                            140u8, 178u8, 182u8, 50u8, 1u8, 55u8, 149u8, 92u8, 196u8, 229u8, 170u8,
                            106u8, 193u8, 88u8, 255u8, 244u8, 2u8, 193u8, 62u8, 235u8, 204u8, 91u8,
                        ],
                    )
                }
                pub fn reserves(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::reserves::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::reserves::Reserves,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Reserves",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            112u8, 10u8, 241u8, 77u8, 64u8, 187u8, 106u8, 159u8, 13u8, 153u8,
                            140u8, 178u8, 182u8, 50u8, 1u8, 55u8, 149u8, 92u8, 196u8, 229u8, 170u8,
                            106u8, 193u8, 88u8, 255u8, 244u8, 2u8, 193u8, 62u8, 235u8, 204u8, 91u8,
                        ],
                    )
                }
                pub fn holds_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::holds::Holds,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Holds",
                        vec![],
                        [
                            53u8, 126u8, 215u8, 237u8, 42u8, 223u8, 188u8, 150u8, 230u8, 107u8,
                            95u8, 24u8, 26u8, 235u8, 158u8, 149u8, 193u8, 191u8, 10u8, 194u8,
                            231u8, 59u8, 35u8, 167u8, 186u8, 89u8, 43u8, 126u8, 215u8, 117u8, 1u8,
                            202u8,
                        ],
                    )
                }
                pub fn holds(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::holds::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::holds::Holds,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Holds",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            53u8, 126u8, 215u8, 237u8, 42u8, 223u8, 188u8, 150u8, 230u8, 107u8,
                            95u8, 24u8, 26u8, 235u8, 158u8, 149u8, 193u8, 191u8, 10u8, 194u8,
                            231u8, 59u8, 35u8, 167u8, 186u8, 89u8, 43u8, 126u8, 215u8, 117u8, 1u8,
                            202u8,
                        ],
                    )
                }
                pub fn freezes_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::freezes::Freezes,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Freezes",
                        vec![],
                        [
                            69u8, 49u8, 165u8, 76u8, 135u8, 142u8, 179u8, 118u8, 50u8, 109u8, 53u8,
                            112u8, 110u8, 94u8, 30u8, 93u8, 173u8, 38u8, 27u8, 142u8, 19u8, 5u8,
                            163u8, 4u8, 68u8, 218u8, 179u8, 224u8, 118u8, 218u8, 115u8, 64u8,
                        ],
                    )
                }
                pub fn freezes(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::freezes::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::freezes::Freezes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Freezes",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            69u8, 49u8, 165u8, 76u8, 135u8, 142u8, 179u8, 118u8, 50u8, 109u8, 53u8,
                            112u8, 110u8, 94u8, 30u8, 93u8, 173u8, 38u8, 27u8, 142u8, 19u8, 5u8,
                            163u8, 4u8, 68u8, 218u8, 179u8, 224u8, 118u8, 218u8, 115u8, 64u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn existential_deposit(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "ExistentialDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                pub fn max_locks(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "MaxLocks",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn max_reserves(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "MaxReserves",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn max_holds(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "MaxHolds",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn max_freezes(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "MaxFreezes",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod peaq_did {
        use super::root_mod;
        use super::runtime_types;
        pub type Error = runtime_types::peaq_pallet_did::pallet::Error;
        pub type Call = runtime_types::peaq_pallet_did::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AddAttribute {
                    pub did_account: add_attribute::DidAccount,
                    pub name: add_attribute::Name,
                    pub value: add_attribute::Value,
                    pub valid_for: add_attribute::ValidFor,
                }
                pub mod add_attribute {
                    use super::runtime_types;
                    pub type DidAccount = ::subxt::utils::AccountId32;
                    pub type Name = ::std::vec::Vec<::core::primitive::u8>;
                    pub type Value = ::std::vec::Vec<::core::primitive::u8>;
                    pub type ValidFor = ::core::option::Option<::core::primitive::u32>;
                }
                impl ::subxt::blocks::StaticExtrinsic for AddAttribute {
                    const PALLET: &'static str = "PeaqDid";
                    const CALL: &'static str = "add_attribute";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UpdateAttribute {
                    pub did_account: update_attribute::DidAccount,
                    pub name: update_attribute::Name,
                    pub value: update_attribute::Value,
                    pub valid_for: update_attribute::ValidFor,
                }
                pub mod update_attribute {
                    use super::runtime_types;
                    pub type DidAccount = ::subxt::utils::AccountId32;
                    pub type Name = ::std::vec::Vec<::core::primitive::u8>;
                    pub type Value = ::std::vec::Vec<::core::primitive::u8>;
                    pub type ValidFor = ::core::option::Option<::core::primitive::u32>;
                }
                impl ::subxt::blocks::StaticExtrinsic for UpdateAttribute {
                    const PALLET: &'static str = "PeaqDid";
                    const CALL: &'static str = "update_attribute";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ReadAttribute {
                    pub did_account: read_attribute::DidAccount,
                    pub name: read_attribute::Name,
                }
                pub mod read_attribute {
                    use super::runtime_types;
                    pub type DidAccount = ::subxt::utils::AccountId32;
                    pub type Name = ::std::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::blocks::StaticExtrinsic for ReadAttribute {
                    const PALLET: &'static str = "PeaqDid";
                    const CALL: &'static str = "read_attribute";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct RemoveAttribute {
                    pub did_account: remove_attribute::DidAccount,
                    pub name: remove_attribute::Name,
                }
                pub mod remove_attribute {
                    use super::runtime_types;
                    pub type DidAccount = ::subxt::utils::AccountId32;
                    pub type Name = ::std::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::blocks::StaticExtrinsic for RemoveAttribute {
                    const PALLET: &'static str = "PeaqDid";
                    const CALL: &'static str = "remove_attribute";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn add_attribute(
                    &self,
                    did_account: types::add_attribute::DidAccount,
                    name: types::add_attribute::Name,
                    value: types::add_attribute::Value,
                    valid_for: types::add_attribute::ValidFor,
                ) -> ::subxt::tx::Payload<types::AddAttribute> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqDid",
                        "add_attribute",
                        types::AddAttribute {
                            did_account,
                            name,
                            value,
                            valid_for,
                        },
                        [
                            181u8, 126u8, 148u8, 43u8, 209u8, 208u8, 202u8, 117u8, 42u8, 248u8,
                            57u8, 177u8, 86u8, 163u8, 86u8, 121u8, 84u8, 236u8, 244u8, 38u8, 3u8,
                            118u8, 36u8, 222u8, 169u8, 212u8, 158u8, 77u8, 240u8, 227u8, 172u8,
                            248u8,
                        ],
                    )
                }
                pub fn update_attribute(
                    &self,
                    did_account: types::update_attribute::DidAccount,
                    name: types::update_attribute::Name,
                    value: types::update_attribute::Value,
                    valid_for: types::update_attribute::ValidFor,
                ) -> ::subxt::tx::Payload<types::UpdateAttribute> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqDid",
                        "update_attribute",
                        types::UpdateAttribute {
                            did_account,
                            name,
                            value,
                            valid_for,
                        },
                        [
                            179u8, 192u8, 207u8, 149u8, 157u8, 124u8, 24u8, 129u8, 146u8, 35u8,
                            137u8, 153u8, 203u8, 20u8, 196u8, 235u8, 175u8, 156u8, 168u8, 64u8,
                            59u8, 72u8, 247u8, 151u8, 120u8, 253u8, 136u8, 220u8, 46u8, 127u8,
                            23u8, 105u8,
                        ],
                    )
                }
                pub fn read_attribute(
                    &self,
                    did_account: types::read_attribute::DidAccount,
                    name: types::read_attribute::Name,
                ) -> ::subxt::tx::Payload<types::ReadAttribute> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqDid",
                        "read_attribute",
                        types::ReadAttribute { did_account, name },
                        [
                            93u8, 140u8, 103u8, 219u8, 174u8, 246u8, 171u8, 187u8, 225u8, 25u8,
                            178u8, 203u8, 169u8, 249u8, 175u8, 113u8, 164u8, 84u8, 52u8, 124u8,
                            165u8, 12u8, 42u8, 42u8, 248u8, 156u8, 129u8, 69u8, 109u8, 56u8, 17u8,
                            222u8,
                        ],
                    )
                }
                pub fn remove_attribute(
                    &self,
                    did_account: types::remove_attribute::DidAccount,
                    name: types::remove_attribute::Name,
                ) -> ::subxt::tx::Payload<types::RemoveAttribute> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqDid",
                        "remove_attribute",
                        types::RemoveAttribute { did_account, name },
                        [
                            63u8, 2u8, 115u8, 203u8, 158u8, 168u8, 179u8, 55u8, 152u8, 157u8, 11u8,
                            58u8, 224u8, 138u8, 105u8, 94u8, 157u8, 248u8, 60u8, 239u8, 140u8, 7u8,
                            30u8, 252u8, 189u8, 210u8, 168u8, 64u8, 195u8, 1u8, 224u8, 79u8,
                        ],
                    )
                }
            }
        }
        pub type Event = runtime_types::peaq_pallet_did::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct AttributeAdded(
                pub attribute_added::Field0,
                pub attribute_added::Field1,
                pub attribute_added::Field2,
                pub attribute_added::Field3,
                pub attribute_added::Field4,
            );
            pub mod attribute_added {
                use super::runtime_types;
                pub type Field0 = ::subxt::utils::AccountId32;
                pub type Field1 = ::subxt::utils::AccountId32;
                pub type Field2 = ::std::vec::Vec<::core::primitive::u8>;
                pub type Field3 = ::std::vec::Vec<::core::primitive::u8>;
                pub type Field4 = ::core::option::Option<::core::primitive::u32>;
            }
            impl ::subxt::events::StaticEvent for AttributeAdded {
                const PALLET: &'static str = "PeaqDid";
                const EVENT: &'static str = "AttributeAdded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct AttributeRead(pub attribute_read::Field0);
            pub mod attribute_read {
                use super::runtime_types;
                pub type Field0 = runtime_types::peaq_pallet_did::structs::Attribute<
                    ::core::primitive::u32,
                    ::core::primitive::u64,
                >;
            }
            impl ::subxt::events::StaticEvent for AttributeRead {
                const PALLET: &'static str = "PeaqDid";
                const EVENT: &'static str = "AttributeRead";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct AttributeUpdated(
                pub attribute_updated::Field0,
                pub attribute_updated::Field1,
                pub attribute_updated::Field2,
                pub attribute_updated::Field3,
                pub attribute_updated::Field4,
            );
            pub mod attribute_updated {
                use super::runtime_types;
                pub type Field0 = ::subxt::utils::AccountId32;
                pub type Field1 = ::subxt::utils::AccountId32;
                pub type Field2 = ::std::vec::Vec<::core::primitive::u8>;
                pub type Field3 = ::std::vec::Vec<::core::primitive::u8>;
                pub type Field4 = ::core::option::Option<::core::primitive::u32>;
            }
            impl ::subxt::events::StaticEvent for AttributeUpdated {
                const PALLET: &'static str = "PeaqDid";
                const EVENT: &'static str = "AttributeUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct AttributeRemoved(
                pub attribute_removed::Field0,
                pub attribute_removed::Field1,
                pub attribute_removed::Field2,
            );
            pub mod attribute_removed {
                use super::runtime_types;
                pub type Field0 = ::subxt::utils::AccountId32;
                pub type Field1 = ::subxt::utils::AccountId32;
                pub type Field2 = ::std::vec::Vec<::core::primitive::u8>;
            }
            impl ::subxt::events::StaticEvent for AttributeRemoved {
                const PALLET: &'static str = "PeaqDid";
                const EVENT: &'static str = "AttributeRemoved";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod attribute_store {
                    use super::runtime_types;
                    pub type AttributeStore = runtime_types::peaq_pallet_did::structs::Attribute<
                        ::core::primitive::u32,
                        ::core::primitive::u64,
                    >;
                    pub type Param0 = [::core::primitive::u8; 32usize];
                }
                pub mod owner_store {
                    use super::runtime_types;
                    pub type OwnerStore = ::subxt::utils::AccountId32;
                    pub type Param0 = ::subxt::utils::AccountId32;
                    pub type Param1 = [::core::primitive::u8; 32usize];
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn attribute_store_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::attribute_store::AttributeStore,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PeaqDid",
                        "AttributeStore",
                        vec![],
                        [
                            123u8, 176u8, 156u8, 33u8, 193u8, 98u8, 43u8, 106u8, 200u8, 53u8,
                            127u8, 206u8, 139u8, 232u8, 168u8, 26u8, 230u8, 184u8, 130u8, 238u8,
                            129u8, 201u8, 86u8, 62u8, 240u8, 228u8, 102u8, 116u8, 250u8, 77u8,
                            81u8, 155u8,
                        ],
                    )
                }
                pub fn attribute_store(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::attribute_store::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::attribute_store::AttributeStore,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PeaqDid",
                        "AttributeStore",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            123u8, 176u8, 156u8, 33u8, 193u8, 98u8, 43u8, 106u8, 200u8, 53u8,
                            127u8, 206u8, 139u8, 232u8, 168u8, 26u8, 230u8, 184u8, 130u8, 238u8,
                            129u8, 201u8, 86u8, 62u8, 240u8, 228u8, 102u8, 116u8, 250u8, 77u8,
                            81u8, 155u8,
                        ],
                    )
                }
                pub fn owner_store_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::owner_store::OwnerStore,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PeaqDid",
                        "OwnerStore",
                        vec![],
                        [
                            129u8, 67u8, 69u8, 112u8, 36u8, 19u8, 200u8, 189u8, 18u8, 21u8, 216u8,
                            101u8, 59u8, 251u8, 47u8, 45u8, 250u8, 180u8, 100u8, 131u8, 93u8, 99u8,
                            219u8, 63u8, 240u8, 160u8, 164u8, 92u8, 80u8, 194u8, 213u8, 67u8,
                        ],
                    )
                }
                pub fn owner_store_iter1(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::owner_store::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::owner_store::OwnerStore,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PeaqDid",
                        "OwnerStore",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            129u8, 67u8, 69u8, 112u8, 36u8, 19u8, 200u8, 189u8, 18u8, 21u8, 216u8,
                            101u8, 59u8, 251u8, 47u8, 45u8, 250u8, 180u8, 100u8, 131u8, 93u8, 99u8,
                            219u8, 63u8, 240u8, 160u8, 164u8, 92u8, 80u8, 194u8, 213u8, 67u8,
                        ],
                    )
                }
                pub fn owner_store(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::owner_store::Param0>,
                    _1: impl ::std::borrow::Borrow<types::owner_store::Param1>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::owner_store::OwnerStore,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PeaqDid",
                        "OwnerStore",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            129u8, 67u8, 69u8, 112u8, 36u8, 19u8, 200u8, 189u8, 18u8, 21u8, 216u8,
                            101u8, 59u8, 251u8, 47u8, 45u8, 250u8, 180u8, 100u8, 131u8, 93u8, 99u8,
                            219u8, 63u8, 240u8, 160u8, 164u8, 92u8, 80u8, 194u8, 213u8, 67u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod peaq_rbac {
        use super::root_mod;
        use super::runtime_types;
        pub type Error = runtime_types::peaq_pallet_rbac::pallet::Error;
        pub type Call = runtime_types::peaq_pallet_rbac::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct FetchRole {
                    pub owner: fetch_role::Owner,
                    pub entity: fetch_role::Entity,
                }
                pub mod fetch_role {
                    use super::runtime_types;
                    pub type Owner = ::subxt::utils::AccountId32;
                    pub type Entity = [::core::primitive::u8; 32usize];
                }
                impl ::subxt::blocks::StaticExtrinsic for FetchRole {
                    const PALLET: &'static str = "PeaqRbac";
                    const CALL: &'static str = "fetch_role";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct FetchRoles {
                    pub owner: fetch_roles::Owner,
                }
                pub mod fetch_roles {
                    use super::runtime_types;
                    pub type Owner = ::subxt::utils::AccountId32;
                }
                impl ::subxt::blocks::StaticExtrinsic for FetchRoles {
                    const PALLET: &'static str = "PeaqRbac";
                    const CALL: &'static str = "fetch_roles";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AddRole {
                    pub role_id: add_role::RoleId,
                    pub name: add_role::Name,
                }
                pub mod add_role {
                    use super::runtime_types;
                    pub type RoleId = [::core::primitive::u8; 32usize];
                    pub type Name = ::std::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::blocks::StaticExtrinsic for AddRole {
                    const PALLET: &'static str = "PeaqRbac";
                    const CALL: &'static str = "add_role";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UpdateRole {
                    pub role_id: update_role::RoleId,
                    pub name: update_role::Name,
                }
                pub mod update_role {
                    use super::runtime_types;
                    pub type RoleId = [::core::primitive::u8; 32usize];
                    pub type Name = ::std::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::blocks::StaticExtrinsic for UpdateRole {
                    const PALLET: &'static str = "PeaqRbac";
                    const CALL: &'static str = "update_role";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct DisableRole {
                    pub role_id: disable_role::RoleId,
                }
                pub mod disable_role {
                    use super::runtime_types;
                    pub type RoleId = [::core::primitive::u8; 32usize];
                }
                impl ::subxt::blocks::StaticExtrinsic for DisableRole {
                    const PALLET: &'static str = "PeaqRbac";
                    const CALL: &'static str = "disable_role";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct FetchUserRoles {
                    pub owner: fetch_user_roles::Owner,
                    pub user_id: fetch_user_roles::UserId,
                }
                pub mod fetch_user_roles {
                    use super::runtime_types;
                    pub type Owner = ::subxt::utils::AccountId32;
                    pub type UserId = [::core::primitive::u8; 32usize];
                }
                impl ::subxt::blocks::StaticExtrinsic for FetchUserRoles {
                    const PALLET: &'static str = "PeaqRbac";
                    const CALL: &'static str = "fetch_user_roles";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AssignRoleToUser {
                    pub role_id: assign_role_to_user::RoleId,
                    pub user_id: assign_role_to_user::UserId,
                }
                pub mod assign_role_to_user {
                    use super::runtime_types;
                    pub type RoleId = [::core::primitive::u8; 32usize];
                    pub type UserId = [::core::primitive::u8; 32usize];
                }
                impl ::subxt::blocks::StaticExtrinsic for AssignRoleToUser {
                    const PALLET: &'static str = "PeaqRbac";
                    const CALL: &'static str = "assign_role_to_user";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UnassignRoleToUser {
                    pub role_id: unassign_role_to_user::RoleId,
                    pub user_id: unassign_role_to_user::UserId,
                }
                pub mod unassign_role_to_user {
                    use super::runtime_types;
                    pub type RoleId = [::core::primitive::u8; 32usize];
                    pub type UserId = [::core::primitive::u8; 32usize];
                }
                impl ::subxt::blocks::StaticExtrinsic for UnassignRoleToUser {
                    const PALLET: &'static str = "PeaqRbac";
                    const CALL: &'static str = "unassign_role_to_user";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct FetchPermission {
                    pub owner: fetch_permission::Owner,
                    pub permission_id: fetch_permission::PermissionId,
                }
                pub mod fetch_permission {
                    use super::runtime_types;
                    pub type Owner = ::subxt::utils::AccountId32;
                    pub type PermissionId = [::core::primitive::u8; 32usize];
                }
                impl ::subxt::blocks::StaticExtrinsic for FetchPermission {
                    const PALLET: &'static str = "PeaqRbac";
                    const CALL: &'static str = "fetch_permission";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct FetchPermissions {
                    pub owner: fetch_permissions::Owner,
                }
                pub mod fetch_permissions {
                    use super::runtime_types;
                    pub type Owner = ::subxt::utils::AccountId32;
                }
                impl ::subxt::blocks::StaticExtrinsic for FetchPermissions {
                    const PALLET: &'static str = "PeaqRbac";
                    const CALL: &'static str = "fetch_permissions";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AddPermission {
                    pub permission_id: add_permission::PermissionId,
                    pub name: add_permission::Name,
                }
                pub mod add_permission {
                    use super::runtime_types;
                    pub type PermissionId = [::core::primitive::u8; 32usize];
                    pub type Name = ::std::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::blocks::StaticExtrinsic for AddPermission {
                    const PALLET: &'static str = "PeaqRbac";
                    const CALL: &'static str = "add_permission";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UpdatePermission {
                    pub permission_id: update_permission::PermissionId,
                    pub name: update_permission::Name,
                }
                pub mod update_permission {
                    use super::runtime_types;
                    pub type PermissionId = [::core::primitive::u8; 32usize];
                    pub type Name = ::std::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::blocks::StaticExtrinsic for UpdatePermission {
                    const PALLET: &'static str = "PeaqRbac";
                    const CALL: &'static str = "update_permission";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct DisablePermission {
                    pub permission_id: disable_permission::PermissionId,
                }
                pub mod disable_permission {
                    use super::runtime_types;
                    pub type PermissionId = [::core::primitive::u8; 32usize];
                }
                impl ::subxt::blocks::StaticExtrinsic for DisablePermission {
                    const PALLET: &'static str = "PeaqRbac";
                    const CALL: &'static str = "disable_permission";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct FetchRolePermissions {
                    pub owner: fetch_role_permissions::Owner,
                    pub role_id: fetch_role_permissions::RoleId,
                }
                pub mod fetch_role_permissions {
                    use super::runtime_types;
                    pub type Owner = ::subxt::utils::AccountId32;
                    pub type RoleId = [::core::primitive::u8; 32usize];
                }
                impl ::subxt::blocks::StaticExtrinsic for FetchRolePermissions {
                    const PALLET: &'static str = "PeaqRbac";
                    const CALL: &'static str = "fetch_role_permissions";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AssignPermissionToRole {
                    pub permission_id: assign_permission_to_role::PermissionId,
                    pub role_id: assign_permission_to_role::RoleId,
                }
                pub mod assign_permission_to_role {
                    use super::runtime_types;
                    pub type PermissionId = [::core::primitive::u8; 32usize];
                    pub type RoleId = [::core::primitive::u8; 32usize];
                }
                impl ::subxt::blocks::StaticExtrinsic for AssignPermissionToRole {
                    const PALLET: &'static str = "PeaqRbac";
                    const CALL: &'static str = "assign_permission_to_role";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UnassignPermissionToRole {
                    pub permission_id: unassign_permission_to_role::PermissionId,
                    pub role_id: unassign_permission_to_role::RoleId,
                }
                pub mod unassign_permission_to_role {
                    use super::runtime_types;
                    pub type PermissionId = [::core::primitive::u8; 32usize];
                    pub type RoleId = [::core::primitive::u8; 32usize];
                }
                impl ::subxt::blocks::StaticExtrinsic for UnassignPermissionToRole {
                    const PALLET: &'static str = "PeaqRbac";
                    const CALL: &'static str = "unassign_permission_to_role";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct FetchGroup {
                    pub owner: fetch_group::Owner,
                    pub group_id: fetch_group::GroupId,
                }
                pub mod fetch_group {
                    use super::runtime_types;
                    pub type Owner = ::subxt::utils::AccountId32;
                    pub type GroupId = [::core::primitive::u8; 32usize];
                }
                impl ::subxt::blocks::StaticExtrinsic for FetchGroup {
                    const PALLET: &'static str = "PeaqRbac";
                    const CALL: &'static str = "fetch_group";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct FetchGroups {
                    pub owner: fetch_groups::Owner,
                }
                pub mod fetch_groups {
                    use super::runtime_types;
                    pub type Owner = ::subxt::utils::AccountId32;
                }
                impl ::subxt::blocks::StaticExtrinsic for FetchGroups {
                    const PALLET: &'static str = "PeaqRbac";
                    const CALL: &'static str = "fetch_groups";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AddGroup {
                    pub group_id: add_group::GroupId,
                    pub name: add_group::Name,
                }
                pub mod add_group {
                    use super::runtime_types;
                    pub type GroupId = [::core::primitive::u8; 32usize];
                    pub type Name = ::std::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::blocks::StaticExtrinsic for AddGroup {
                    const PALLET: &'static str = "PeaqRbac";
                    const CALL: &'static str = "add_group";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UpdateGroup {
                    pub group_id: update_group::GroupId,
                    pub name: update_group::Name,
                }
                pub mod update_group {
                    use super::runtime_types;
                    pub type GroupId = [::core::primitive::u8; 32usize];
                    pub type Name = ::std::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::blocks::StaticExtrinsic for UpdateGroup {
                    const PALLET: &'static str = "PeaqRbac";
                    const CALL: &'static str = "update_group";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct DisableGroup {
                    pub group_id: disable_group::GroupId,
                }
                pub mod disable_group {
                    use super::runtime_types;
                    pub type GroupId = [::core::primitive::u8; 32usize];
                }
                impl ::subxt::blocks::StaticExtrinsic for DisableGroup {
                    const PALLET: &'static str = "PeaqRbac";
                    const CALL: &'static str = "disable_group";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AssignRoleToGroup {
                    pub role_id: assign_role_to_group::RoleId,
                    pub group_id: assign_role_to_group::GroupId,
                }
                pub mod assign_role_to_group {
                    use super::runtime_types;
                    pub type RoleId = [::core::primitive::u8; 32usize];
                    pub type GroupId = [::core::primitive::u8; 32usize];
                }
                impl ::subxt::blocks::StaticExtrinsic for AssignRoleToGroup {
                    const PALLET: &'static str = "PeaqRbac";
                    const CALL: &'static str = "assign_role_to_group";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UnassignRoleToGroup {
                    pub role_id: unassign_role_to_group::RoleId,
                    pub group_id: unassign_role_to_group::GroupId,
                }
                pub mod unassign_role_to_group {
                    use super::runtime_types;
                    pub type RoleId = [::core::primitive::u8; 32usize];
                    pub type GroupId = [::core::primitive::u8; 32usize];
                }
                impl ::subxt::blocks::StaticExtrinsic for UnassignRoleToGroup {
                    const PALLET: &'static str = "PeaqRbac";
                    const CALL: &'static str = "unassign_role_to_group";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct FetchGroupRoles {
                    pub owner: fetch_group_roles::Owner,
                    pub group_id: fetch_group_roles::GroupId,
                }
                pub mod fetch_group_roles {
                    use super::runtime_types;
                    pub type Owner = ::subxt::utils::AccountId32;
                    pub type GroupId = [::core::primitive::u8; 32usize];
                }
                impl ::subxt::blocks::StaticExtrinsic for FetchGroupRoles {
                    const PALLET: &'static str = "PeaqRbac";
                    const CALL: &'static str = "fetch_group_roles";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AssignUserToGroup {
                    pub user_id: assign_user_to_group::UserId,
                    pub group_id: assign_user_to_group::GroupId,
                }
                pub mod assign_user_to_group {
                    use super::runtime_types;
                    pub type UserId = [::core::primitive::u8; 32usize];
                    pub type GroupId = [::core::primitive::u8; 32usize];
                }
                impl ::subxt::blocks::StaticExtrinsic for AssignUserToGroup {
                    const PALLET: &'static str = "PeaqRbac";
                    const CALL: &'static str = "assign_user_to_group";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UnassignUserToGroup {
                    pub user_id: unassign_user_to_group::UserId,
                    pub group_id: unassign_user_to_group::GroupId,
                }
                pub mod unassign_user_to_group {
                    use super::runtime_types;
                    pub type UserId = [::core::primitive::u8; 32usize];
                    pub type GroupId = [::core::primitive::u8; 32usize];
                }
                impl ::subxt::blocks::StaticExtrinsic for UnassignUserToGroup {
                    const PALLET: &'static str = "PeaqRbac";
                    const CALL: &'static str = "unassign_user_to_group";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct FetchUserGroups {
                    pub owner: fetch_user_groups::Owner,
                    pub user_id: fetch_user_groups::UserId,
                }
                pub mod fetch_user_groups {
                    use super::runtime_types;
                    pub type Owner = ::subxt::utils::AccountId32;
                    pub type UserId = [::core::primitive::u8; 32usize];
                }
                impl ::subxt::blocks::StaticExtrinsic for FetchUserGroups {
                    const PALLET: &'static str = "PeaqRbac";
                    const CALL: &'static str = "fetch_user_groups";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct FetchUserPermissions {
                    pub owner: fetch_user_permissions::Owner,
                    pub user_id: fetch_user_permissions::UserId,
                }
                pub mod fetch_user_permissions {
                    use super::runtime_types;
                    pub type Owner = ::subxt::utils::AccountId32;
                    pub type UserId = [::core::primitive::u8; 32usize];
                }
                impl ::subxt::blocks::StaticExtrinsic for FetchUserPermissions {
                    const PALLET: &'static str = "PeaqRbac";
                    const CALL: &'static str = "fetch_user_permissions";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct FetchGroupPermissions {
                    pub owner: fetch_group_permissions::Owner,
                    pub group_id: fetch_group_permissions::GroupId,
                }
                pub mod fetch_group_permissions {
                    use super::runtime_types;
                    pub type Owner = ::subxt::utils::AccountId32;
                    pub type GroupId = [::core::primitive::u8; 32usize];
                }
                impl ::subxt::blocks::StaticExtrinsic for FetchGroupPermissions {
                    const PALLET: &'static str = "PeaqRbac";
                    const CALL: &'static str = "fetch_group_permissions";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn fetch_role(
                    &self,
                    owner: types::fetch_role::Owner,
                    entity: types::fetch_role::Entity,
                ) -> ::subxt::tx::Payload<types::FetchRole> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqRbac",
                        "fetch_role",
                        types::FetchRole { owner, entity },
                        [
                            2u8, 79u8, 48u8, 229u8, 143u8, 88u8, 149u8, 20u8, 11u8, 168u8, 180u8,
                            178u8, 96u8, 113u8, 155u8, 157u8, 92u8, 240u8, 48u8, 185u8, 203u8, 1u8,
                            0u8, 12u8, 7u8, 86u8, 193u8, 24u8, 214u8, 64u8, 132u8, 53u8,
                        ],
                    )
                }
                pub fn fetch_roles(
                    &self,
                    owner: types::fetch_roles::Owner,
                ) -> ::subxt::tx::Payload<types::FetchRoles> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqRbac",
                        "fetch_roles",
                        types::FetchRoles { owner },
                        [
                            181u8, 71u8, 25u8, 79u8, 145u8, 190u8, 33u8, 100u8, 231u8, 176u8,
                            239u8, 132u8, 53u8, 102u8, 243u8, 92u8, 118u8, 179u8, 124u8, 7u8,
                            201u8, 172u8, 191u8, 100u8, 95u8, 98u8, 81u8, 91u8, 203u8, 3u8, 184u8,
                            146u8,
                        ],
                    )
                }
                pub fn add_role(
                    &self,
                    role_id: types::add_role::RoleId,
                    name: types::add_role::Name,
                ) -> ::subxt::tx::Payload<types::AddRole> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqRbac",
                        "add_role",
                        types::AddRole { role_id, name },
                        [
                            31u8, 233u8, 38u8, 129u8, 99u8, 53u8, 217u8, 151u8, 80u8, 152u8, 124u8,
                            159u8, 183u8, 124u8, 86u8, 222u8, 201u8, 185u8, 205u8, 138u8, 239u8,
                            56u8, 30u8, 140u8, 91u8, 3u8, 139u8, 87u8, 213u8, 202u8, 207u8, 117u8,
                        ],
                    )
                }
                pub fn update_role(
                    &self,
                    role_id: types::update_role::RoleId,
                    name: types::update_role::Name,
                ) -> ::subxt::tx::Payload<types::UpdateRole> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqRbac",
                        "update_role",
                        types::UpdateRole { role_id, name },
                        [
                            8u8, 100u8, 52u8, 161u8, 169u8, 251u8, 219u8, 87u8, 127u8, 129u8, 40u8,
                            234u8, 198u8, 131u8, 158u8, 206u8, 148u8, 85u8, 68u8, 167u8, 8u8,
                            154u8, 212u8, 87u8, 120u8, 234u8, 30u8, 221u8, 186u8, 44u8, 129u8,
                            174u8,
                        ],
                    )
                }
                pub fn disable_role(
                    &self,
                    role_id: types::disable_role::RoleId,
                ) -> ::subxt::tx::Payload<types::DisableRole> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqRbac",
                        "disable_role",
                        types::DisableRole { role_id },
                        [
                            40u8, 116u8, 160u8, 60u8, 194u8, 110u8, 106u8, 190u8, 123u8, 211u8,
                            125u8, 254u8, 254u8, 22u8, 24u8, 65u8, 98u8, 174u8, 150u8, 156u8,
                            168u8, 62u8, 2u8, 17u8, 241u8, 221u8, 1u8, 29u8, 135u8, 255u8, 139u8,
                            70u8,
                        ],
                    )
                }
                pub fn fetch_user_roles(
                    &self,
                    owner: types::fetch_user_roles::Owner,
                    user_id: types::fetch_user_roles::UserId,
                ) -> ::subxt::tx::Payload<types::FetchUserRoles> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqRbac",
                        "fetch_user_roles",
                        types::FetchUserRoles { owner, user_id },
                        [
                            83u8, 51u8, 211u8, 101u8, 59u8, 61u8, 230u8, 38u8, 69u8, 214u8, 82u8,
                            242u8, 245u8, 17u8, 83u8, 188u8, 114u8, 8u8, 118u8, 144u8, 52u8, 11u8,
                            147u8, 69u8, 253u8, 74u8, 153u8, 59u8, 191u8, 233u8, 66u8, 112u8,
                        ],
                    )
                }
                pub fn assign_role_to_user(
                    &self,
                    role_id: types::assign_role_to_user::RoleId,
                    user_id: types::assign_role_to_user::UserId,
                ) -> ::subxt::tx::Payload<types::AssignRoleToUser> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqRbac",
                        "assign_role_to_user",
                        types::AssignRoleToUser { role_id, user_id },
                        [
                            242u8, 250u8, 144u8, 112u8, 221u8, 143u8, 163u8, 0u8, 100u8, 64u8,
                            251u8, 15u8, 66u8, 217u8, 49u8, 122u8, 105u8, 203u8, 186u8, 225u8,
                            210u8, 131u8, 240u8, 63u8, 139u8, 25u8, 55u8, 142u8, 118u8, 248u8,
                            24u8, 231u8,
                        ],
                    )
                }
                pub fn unassign_role_to_user(
                    &self,
                    role_id: types::unassign_role_to_user::RoleId,
                    user_id: types::unassign_role_to_user::UserId,
                ) -> ::subxt::tx::Payload<types::UnassignRoleToUser> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqRbac",
                        "unassign_role_to_user",
                        types::UnassignRoleToUser { role_id, user_id },
                        [
                            215u8, 94u8, 195u8, 91u8, 149u8, 37u8, 128u8, 37u8, 164u8, 109u8, 14u8,
                            175u8, 145u8, 138u8, 69u8, 121u8, 44u8, 247u8, 30u8, 98u8, 250u8,
                            172u8, 198u8, 59u8, 87u8, 47u8, 2u8, 88u8, 103u8, 99u8, 196u8, 191u8,
                        ],
                    )
                }
                pub fn fetch_permission(
                    &self,
                    owner: types::fetch_permission::Owner,
                    permission_id: types::fetch_permission::PermissionId,
                ) -> ::subxt::tx::Payload<types::FetchPermission> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqRbac",
                        "fetch_permission",
                        types::FetchPermission {
                            owner,
                            permission_id,
                        },
                        [
                            76u8, 36u8, 180u8, 235u8, 35u8, 131u8, 115u8, 250u8, 131u8, 174u8,
                            12u8, 133u8, 36u8, 189u8, 12u8, 2u8, 203u8, 252u8, 49u8, 11u8, 91u8,
                            10u8, 250u8, 73u8, 16u8, 121u8, 178u8, 13u8, 195u8, 93u8, 126u8, 82u8,
                        ],
                    )
                }
                pub fn fetch_permissions(
                    &self,
                    owner: types::fetch_permissions::Owner,
                ) -> ::subxt::tx::Payload<types::FetchPermissions> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqRbac",
                        "fetch_permissions",
                        types::FetchPermissions { owner },
                        [
                            90u8, 44u8, 201u8, 116u8, 188u8, 128u8, 226u8, 41u8, 131u8, 61u8,
                            125u8, 253u8, 224u8, 179u8, 249u8, 7u8, 167u8, 33u8, 252u8, 78u8,
                            228u8, 14u8, 117u8, 246u8, 95u8, 118u8, 198u8, 109u8, 74u8, 145u8, 4u8,
                            153u8,
                        ],
                    )
                }
                pub fn add_permission(
                    &self,
                    permission_id: types::add_permission::PermissionId,
                    name: types::add_permission::Name,
                ) -> ::subxt::tx::Payload<types::AddPermission> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqRbac",
                        "add_permission",
                        types::AddPermission {
                            permission_id,
                            name,
                        },
                        [
                            186u8, 248u8, 200u8, 107u8, 114u8, 65u8, 138u8, 140u8, 99u8, 240u8,
                            184u8, 203u8, 125u8, 219u8, 29u8, 190u8, 212u8, 163u8, 121u8, 39u8,
                            26u8, 156u8, 217u8, 209u8, 247u8, 72u8, 148u8, 73u8, 196u8, 186u8,
                            77u8, 36u8,
                        ],
                    )
                }
                pub fn update_permission(
                    &self,
                    permission_id: types::update_permission::PermissionId,
                    name: types::update_permission::Name,
                ) -> ::subxt::tx::Payload<types::UpdatePermission> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqRbac",
                        "update_permission",
                        types::UpdatePermission {
                            permission_id,
                            name,
                        },
                        [
                            20u8, 12u8, 134u8, 254u8, 55u8, 227u8, 149u8, 144u8, 19u8, 40u8, 64u8,
                            30u8, 63u8, 163u8, 77u8, 96u8, 152u8, 226u8, 120u8, 168u8, 0u8, 29u8,
                            53u8, 135u8, 166u8, 109u8, 76u8, 5u8, 116u8, 50u8, 228u8, 236u8,
                        ],
                    )
                }
                pub fn disable_permission(
                    &self,
                    permission_id: types::disable_permission::PermissionId,
                ) -> ::subxt::tx::Payload<types::DisablePermission> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqRbac",
                        "disable_permission",
                        types::DisablePermission { permission_id },
                        [
                            89u8, 167u8, 149u8, 4u8, 74u8, 92u8, 18u8, 49u8, 97u8, 147u8, 227u8,
                            253u8, 97u8, 216u8, 63u8, 243u8, 210u8, 57u8, 17u8, 59u8, 191u8, 145u8,
                            142u8, 123u8, 95u8, 146u8, 199u8, 41u8, 246u8, 41u8, 184u8, 12u8,
                        ],
                    )
                }
                pub fn fetch_role_permissions(
                    &self,
                    owner: types::fetch_role_permissions::Owner,
                    role_id: types::fetch_role_permissions::RoleId,
                ) -> ::subxt::tx::Payload<types::FetchRolePermissions> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqRbac",
                        "fetch_role_permissions",
                        types::FetchRolePermissions { owner, role_id },
                        [
                            67u8, 248u8, 80u8, 155u8, 63u8, 21u8, 80u8, 194u8, 4u8, 172u8, 66u8,
                            124u8, 108u8, 80u8, 97u8, 100u8, 252u8, 132u8, 235u8, 112u8, 170u8,
                            199u8, 86u8, 100u8, 207u8, 142u8, 14u8, 57u8, 121u8, 42u8, 191u8,
                            163u8,
                        ],
                    )
                }
                pub fn assign_permission_to_role(
                    &self,
                    permission_id: types::assign_permission_to_role::PermissionId,
                    role_id: types::assign_permission_to_role::RoleId,
                ) -> ::subxt::tx::Payload<types::AssignPermissionToRole> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqRbac",
                        "assign_permission_to_role",
                        types::AssignPermissionToRole {
                            permission_id,
                            role_id,
                        },
                        [
                            179u8, 159u8, 101u8, 88u8, 224u8, 80u8, 82u8, 133u8, 70u8, 140u8, 47u8,
                            176u8, 87u8, 97u8, 10u8, 88u8, 139u8, 80u8, 177u8, 159u8, 253u8, 82u8,
                            19u8, 199u8, 59u8, 182u8, 138u8, 121u8, 104u8, 186u8, 191u8, 71u8,
                        ],
                    )
                }
                pub fn unassign_permission_to_role(
                    &self,
                    permission_id: types::unassign_permission_to_role::PermissionId,
                    role_id: types::unassign_permission_to_role::RoleId,
                ) -> ::subxt::tx::Payload<types::UnassignPermissionToRole> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqRbac",
                        "unassign_permission_to_role",
                        types::UnassignPermissionToRole {
                            permission_id,
                            role_id,
                        },
                        [
                            181u8, 229u8, 226u8, 127u8, 60u8, 248u8, 5u8, 243u8, 170u8, 98u8,
                            180u8, 249u8, 64u8, 203u8, 170u8, 172u8, 183u8, 121u8, 57u8, 32u8,
                            72u8, 14u8, 20u8, 81u8, 60u8, 27u8, 158u8, 46u8, 165u8, 98u8, 246u8,
                            129u8,
                        ],
                    )
                }
                pub fn fetch_group(
                    &self,
                    owner: types::fetch_group::Owner,
                    group_id: types::fetch_group::GroupId,
                ) -> ::subxt::tx::Payload<types::FetchGroup> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqRbac",
                        "fetch_group",
                        types::FetchGroup { owner, group_id },
                        [
                            138u8, 36u8, 169u8, 60u8, 164u8, 217u8, 159u8, 46u8, 155u8, 14u8, 41u8,
                            41u8, 142u8, 241u8, 103u8, 8u8, 179u8, 125u8, 226u8, 254u8, 15u8,
                            214u8, 3u8, 57u8, 55u8, 162u8, 246u8, 218u8, 225u8, 58u8, 51u8, 175u8,
                        ],
                    )
                }
                pub fn fetch_groups(
                    &self,
                    owner: types::fetch_groups::Owner,
                ) -> ::subxt::tx::Payload<types::FetchGroups> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqRbac",
                        "fetch_groups",
                        types::FetchGroups { owner },
                        [
                            106u8, 133u8, 44u8, 97u8, 103u8, 203u8, 255u8, 116u8, 44u8, 188u8,
                            15u8, 170u8, 40u8, 59u8, 45u8, 180u8, 239u8, 6u8, 83u8, 211u8, 34u8,
                            91u8, 93u8, 129u8, 103u8, 95u8, 13u8, 15u8, 250u8, 38u8, 137u8, 90u8,
                        ],
                    )
                }
                pub fn add_group(
                    &self,
                    group_id: types::add_group::GroupId,
                    name: types::add_group::Name,
                ) -> ::subxt::tx::Payload<types::AddGroup> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqRbac",
                        "add_group",
                        types::AddGroup { group_id, name },
                        [
                            246u8, 233u8, 55u8, 76u8, 172u8, 0u8, 193u8, 237u8, 60u8, 59u8, 16u8,
                            97u8, 115u8, 80u8, 75u8, 44u8, 116u8, 118u8, 252u8, 109u8, 19u8, 125u8,
                            123u8, 238u8, 41u8, 107u8, 220u8, 65u8, 97u8, 46u8, 168u8, 115u8,
                        ],
                    )
                }
                pub fn update_group(
                    &self,
                    group_id: types::update_group::GroupId,
                    name: types::update_group::Name,
                ) -> ::subxt::tx::Payload<types::UpdateGroup> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqRbac",
                        "update_group",
                        types::UpdateGroup { group_id, name },
                        [
                            247u8, 159u8, 129u8, 15u8, 150u8, 204u8, 24u8, 238u8, 110u8, 15u8,
                            138u8, 108u8, 0u8, 90u8, 252u8, 174u8, 144u8, 193u8, 219u8, 137u8,
                            66u8, 202u8, 22u8, 157u8, 17u8, 175u8, 239u8, 98u8, 192u8, 95u8, 139u8,
                            7u8,
                        ],
                    )
                }
                pub fn disable_group(
                    &self,
                    group_id: types::disable_group::GroupId,
                ) -> ::subxt::tx::Payload<types::DisableGroup> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqRbac",
                        "disable_group",
                        types::DisableGroup { group_id },
                        [
                            114u8, 90u8, 105u8, 203u8, 227u8, 204u8, 0u8, 167u8, 214u8, 68u8, 26u8,
                            223u8, 109u8, 3u8, 217u8, 24u8, 215u8, 28u8, 70u8, 182u8, 240u8, 252u8,
                            135u8, 230u8, 3u8, 216u8, 178u8, 245u8, 23u8, 33u8, 150u8, 167u8,
                        ],
                    )
                }
                pub fn assign_role_to_group(
                    &self,
                    role_id: types::assign_role_to_group::RoleId,
                    group_id: types::assign_role_to_group::GroupId,
                ) -> ::subxt::tx::Payload<types::AssignRoleToGroup> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqRbac",
                        "assign_role_to_group",
                        types::AssignRoleToGroup { role_id, group_id },
                        [
                            167u8, 255u8, 17u8, 212u8, 199u8, 124u8, 213u8, 172u8, 122u8, 216u8,
                            116u8, 100u8, 254u8, 214u8, 176u8, 167u8, 245u8, 30u8, 161u8, 240u8,
                            30u8, 177u8, 1u8, 10u8, 62u8, 101u8, 234u8, 94u8, 168u8, 255u8, 166u8,
                            164u8,
                        ],
                    )
                }
                pub fn unassign_role_to_group(
                    &self,
                    role_id: types::unassign_role_to_group::RoleId,
                    group_id: types::unassign_role_to_group::GroupId,
                ) -> ::subxt::tx::Payload<types::UnassignRoleToGroup> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqRbac",
                        "unassign_role_to_group",
                        types::UnassignRoleToGroup { role_id, group_id },
                        [
                            242u8, 180u8, 95u8, 80u8, 61u8, 211u8, 34u8, 65u8, 24u8, 176u8, 213u8,
                            50u8, 171u8, 67u8, 210u8, 62u8, 198u8, 108u8, 10u8, 129u8, 138u8, 77u8,
                            240u8, 137u8, 130u8, 49u8, 195u8, 174u8, 16u8, 133u8, 96u8, 23u8,
                        ],
                    )
                }
                pub fn fetch_group_roles(
                    &self,
                    owner: types::fetch_group_roles::Owner,
                    group_id: types::fetch_group_roles::GroupId,
                ) -> ::subxt::tx::Payload<types::FetchGroupRoles> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqRbac",
                        "fetch_group_roles",
                        types::FetchGroupRoles { owner, group_id },
                        [
                            54u8, 23u8, 120u8, 115u8, 252u8, 14u8, 15u8, 245u8, 239u8, 11u8, 248u8,
                            218u8, 72u8, 172u8, 79u8, 103u8, 104u8, 195u8, 224u8, 62u8, 52u8, 1u8,
                            200u8, 190u8, 241u8, 177u8, 105u8, 93u8, 155u8, 162u8, 193u8, 113u8,
                        ],
                    )
                }
                pub fn assign_user_to_group(
                    &self,
                    user_id: types::assign_user_to_group::UserId,
                    group_id: types::assign_user_to_group::GroupId,
                ) -> ::subxt::tx::Payload<types::AssignUserToGroup> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqRbac",
                        "assign_user_to_group",
                        types::AssignUserToGroup { user_id, group_id },
                        [
                            44u8, 35u8, 52u8, 165u8, 168u8, 158u8, 219u8, 107u8, 53u8, 199u8, 86u8,
                            223u8, 132u8, 176u8, 194u8, 26u8, 224u8, 37u8, 155u8, 87u8, 179u8,
                            24u8, 166u8, 37u8, 250u8, 254u8, 30u8, 9u8, 210u8, 43u8, 203u8, 193u8,
                        ],
                    )
                }
                pub fn unassign_user_to_group(
                    &self,
                    user_id: types::unassign_user_to_group::UserId,
                    group_id: types::unassign_user_to_group::GroupId,
                ) -> ::subxt::tx::Payload<types::UnassignUserToGroup> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqRbac",
                        "unassign_user_to_group",
                        types::UnassignUserToGroup { user_id, group_id },
                        [
                            232u8, 62u8, 232u8, 61u8, 175u8, 205u8, 34u8, 104u8, 239u8, 70u8, 49u8,
                            32u8, 22u8, 128u8, 53u8, 232u8, 107u8, 135u8, 198u8, 20u8, 56u8, 207u8,
                            175u8, 122u8, 171u8, 248u8, 77u8, 15u8, 196u8, 241u8, 163u8, 202u8,
                        ],
                    )
                }
                pub fn fetch_user_groups(
                    &self,
                    owner: types::fetch_user_groups::Owner,
                    user_id: types::fetch_user_groups::UserId,
                ) -> ::subxt::tx::Payload<types::FetchUserGroups> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqRbac",
                        "fetch_user_groups",
                        types::FetchUserGroups { owner, user_id },
                        [
                            212u8, 118u8, 152u8, 137u8, 87u8, 212u8, 23u8, 156u8, 107u8, 75u8,
                            35u8, 86u8, 67u8, 140u8, 2u8, 134u8, 245u8, 162u8, 34u8, 9u8, 46u8,
                            188u8, 240u8, 235u8, 59u8, 117u8, 244u8, 57u8, 246u8, 141u8, 226u8,
                            148u8,
                        ],
                    )
                }
                pub fn fetch_user_permissions(
                    &self,
                    owner: types::fetch_user_permissions::Owner,
                    user_id: types::fetch_user_permissions::UserId,
                ) -> ::subxt::tx::Payload<types::FetchUserPermissions> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqRbac",
                        "fetch_user_permissions",
                        types::FetchUserPermissions { owner, user_id },
                        [
                            30u8, 119u8, 61u8, 241u8, 56u8, 57u8, 248u8, 127u8, 170u8, 70u8, 157u8,
                            225u8, 225u8, 157u8, 253u8, 25u8, 177u8, 168u8, 217u8, 253u8, 219u8,
                            74u8, 229u8, 178u8, 202u8, 13u8, 37u8, 105u8, 215u8, 22u8, 142u8,
                            208u8,
                        ],
                    )
                }
                pub fn fetch_group_permissions(
                    &self,
                    owner: types::fetch_group_permissions::Owner,
                    group_id: types::fetch_group_permissions::GroupId,
                ) -> ::subxt::tx::Payload<types::FetchGroupPermissions> {
                    ::subxt::tx::Payload::new_static(
                        "PeaqRbac",
                        "fetch_group_permissions",
                        types::FetchGroupPermissions { owner, group_id },
                        [
                            188u8, 239u8, 245u8, 104u8, 142u8, 210u8, 222u8, 128u8, 108u8, 8u8,
                            75u8, 27u8, 255u8, 14u8, 85u8, 135u8, 252u8, 117u8, 23u8, 202u8, 216u8,
                            216u8, 235u8, 197u8, 12u8, 112u8, 234u8, 92u8, 100u8, 34u8, 32u8,
                            193u8,
                        ],
                    )
                }
            }
        }
        pub type Event = runtime_types::peaq_pallet_rbac::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct RoleAdded(
                pub role_added::Field0,
                pub role_added::Field1,
                pub role_added::Field2,
            );
            pub mod role_added {
                use super::runtime_types;
                pub type Field0 = ::subxt::utils::AccountId32;
                pub type Field1 = [::core::primitive::u8; 32usize];
                pub type Field2 = ::std::vec::Vec<::core::primitive::u8>;
            }
            impl ::subxt::events::StaticEvent for RoleAdded {
                const PALLET: &'static str = "PeaqRbac";
                const EVENT: &'static str = "RoleAdded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct RoleUpdated(
                pub role_updated::Field0,
                pub role_updated::Field1,
                pub role_updated::Field2,
            );
            pub mod role_updated {
                use super::runtime_types;
                pub type Field0 = ::subxt::utils::AccountId32;
                pub type Field1 = [::core::primitive::u8; 32usize];
                pub type Field2 = ::std::vec::Vec<::core::primitive::u8>;
            }
            impl ::subxt::events::StaticEvent for RoleUpdated {
                const PALLET: &'static str = "PeaqRbac";
                const EVENT: &'static str = "RoleUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct RoleRemoved(pub role_removed::Field0, pub role_removed::Field1);
            pub mod role_removed {
                use super::runtime_types;
                pub type Field0 = ::subxt::utils::AccountId32;
                pub type Field1 = [::core::primitive::u8; 32usize];
            }
            impl ::subxt::events::StaticEvent for RoleRemoved {
                const PALLET: &'static str = "PeaqRbac";
                const EVENT: &'static str = "RoleRemoved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct RoleFetched(pub role_fetched::Field0);
            pub mod role_fetched {
                use super::runtime_types;
                pub type Field0 = runtime_types::peaq_pallet_rbac::structs::Entity<
                    [::core::primitive::u8; 32usize],
                >;
            }
            impl ::subxt::events::StaticEvent for RoleFetched {
                const PALLET: &'static str = "PeaqRbac";
                const EVENT: &'static str = "RoleFetched";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct AllRolesFetched(pub all_roles_fetched::Field0);
            pub mod all_roles_fetched {
                use super::runtime_types;
                pub type Field0 = ::std::vec::Vec<
                    runtime_types::peaq_pallet_rbac::structs::Entity<
                        [::core::primitive::u8; 32usize],
                    >,
                >;
            }
            impl ::subxt::events::StaticEvent for AllRolesFetched {
                const PALLET: &'static str = "PeaqRbac";
                const EVENT: &'static str = "AllRolesFetched";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct RoleAssignedToUser(
                pub role_assigned_to_user::Field0,
                pub role_assigned_to_user::Field1,
                pub role_assigned_to_user::Field2,
            );
            pub mod role_assigned_to_user {
                use super::runtime_types;
                pub type Field0 = ::subxt::utils::AccountId32;
                pub type Field1 = [::core::primitive::u8; 32usize];
                pub type Field2 = [::core::primitive::u8; 32usize];
            }
            impl ::subxt::events::StaticEvent for RoleAssignedToUser {
                const PALLET: &'static str = "PeaqRbac";
                const EVENT: &'static str = "RoleAssignedToUser";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct RoleUnassignedToUser(
                pub role_unassigned_to_user::Field0,
                pub role_unassigned_to_user::Field1,
                pub role_unassigned_to_user::Field2,
            );
            pub mod role_unassigned_to_user {
                use super::runtime_types;
                pub type Field0 = ::subxt::utils::AccountId32;
                pub type Field1 = [::core::primitive::u8; 32usize];
                pub type Field2 = [::core::primitive::u8; 32usize];
            }
            impl ::subxt::events::StaticEvent for RoleUnassignedToUser {
                const PALLET: &'static str = "PeaqRbac";
                const EVENT: &'static str = "RoleUnassignedToUser";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct RoleAssignedToGroup(
                pub role_assigned_to_group::Field0,
                pub role_assigned_to_group::Field1,
                pub role_assigned_to_group::Field2,
            );
            pub mod role_assigned_to_group {
                use super::runtime_types;
                pub type Field0 = ::subxt::utils::AccountId32;
                pub type Field1 = [::core::primitive::u8; 32usize];
                pub type Field2 = [::core::primitive::u8; 32usize];
            }
            impl ::subxt::events::StaticEvent for RoleAssignedToGroup {
                const PALLET: &'static str = "PeaqRbac";
                const EVENT: &'static str = "RoleAssignedToGroup";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct RoleUnassignedToGroup(
                pub role_unassigned_to_group::Field0,
                pub role_unassigned_to_group::Field1,
                pub role_unassigned_to_group::Field2,
            );
            pub mod role_unassigned_to_group {
                use super::runtime_types;
                pub type Field0 = ::subxt::utils::AccountId32;
                pub type Field1 = [::core::primitive::u8; 32usize];
                pub type Field2 = [::core::primitive::u8; 32usize];
            }
            impl ::subxt::events::StaticEvent for RoleUnassignedToGroup {
                const PALLET: &'static str = "PeaqRbac";
                const EVENT: &'static str = "RoleUnassignedToGroup";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct FetchedGroupRoles(pub fetched_group_roles::Field0);
            pub mod fetched_group_roles {
                use super::runtime_types;
                pub type Field0 = ::std::vec::Vec<
                    runtime_types::peaq_pallet_rbac::structs::Role2Group<
                        [::core::primitive::u8; 32usize],
                    >,
                >;
            }
            impl ::subxt::events::StaticEvent for FetchedGroupRoles {
                const PALLET: &'static str = "PeaqRbac";
                const EVENT: &'static str = "FetchedGroupRoles";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct FetchedUserRoles(pub fetched_user_roles::Field0);
            pub mod fetched_user_roles {
                use super::runtime_types;
                pub type Field0 = ::std::vec::Vec<
                    runtime_types::peaq_pallet_rbac::structs::Role2User<
                        [::core::primitive::u8; 32usize],
                    >,
                >;
            }
            impl ::subxt::events::StaticEvent for FetchedUserRoles {
                const PALLET: &'static str = "PeaqRbac";
                const EVENT: &'static str = "FetchedUserRoles";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct FetchedUserGroups(pub fetched_user_groups::Field0);
            pub mod fetched_user_groups {
                use super::runtime_types;
                pub type Field0 = ::std::vec::Vec<
                    runtime_types::peaq_pallet_rbac::structs::User2Group<
                        [::core::primitive::u8; 32usize],
                    >,
                >;
            }
            impl ::subxt::events::StaticEvent for FetchedUserGroups {
                const PALLET: &'static str = "PeaqRbac";
                const EVENT: &'static str = "FetchedUserGroups";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct FetchedUserPermissions(pub fetched_user_permissions::Field0);
            pub mod fetched_user_permissions {
                use super::runtime_types;
                pub type Field0 = ::std::vec::Vec<
                    runtime_types::peaq_pallet_rbac::structs::Entity<
                        [::core::primitive::u8; 32usize],
                    >,
                >;
            }
            impl ::subxt::events::StaticEvent for FetchedUserPermissions {
                const PALLET: &'static str = "PeaqRbac";
                const EVENT: &'static str = "FetchedUserPermissions";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct FetchedGroupPermissions(pub fetched_group_permissions::Field0);
            pub mod fetched_group_permissions {
                use super::runtime_types;
                pub type Field0 = ::std::vec::Vec<
                    runtime_types::peaq_pallet_rbac::structs::Entity<
                        [::core::primitive::u8; 32usize],
                    >,
                >;
            }
            impl ::subxt::events::StaticEvent for FetchedGroupPermissions {
                const PALLET: &'static str = "PeaqRbac";
                const EVENT: &'static str = "FetchedGroupPermissions";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct PermissionAdded(
                pub permission_added::Field0,
                pub permission_added::Field1,
                pub permission_added::Field2,
            );
            pub mod permission_added {
                use super::runtime_types;
                pub type Field0 = ::subxt::utils::AccountId32;
                pub type Field1 = [::core::primitive::u8; 32usize];
                pub type Field2 = ::std::vec::Vec<::core::primitive::u8>;
            }
            impl ::subxt::events::StaticEvent for PermissionAdded {
                const PALLET: &'static str = "PeaqRbac";
                const EVENT: &'static str = "PermissionAdded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct PermissionUpdated(
                pub permission_updated::Field0,
                pub permission_updated::Field1,
                pub permission_updated::Field2,
            );
            pub mod permission_updated {
                use super::runtime_types;
                pub type Field0 = ::subxt::utils::AccountId32;
                pub type Field1 = [::core::primitive::u8; 32usize];
                pub type Field2 = ::std::vec::Vec<::core::primitive::u8>;
            }
            impl ::subxt::events::StaticEvent for PermissionUpdated {
                const PALLET: &'static str = "PeaqRbac";
                const EVENT: &'static str = "PermissionUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct PermissionDisabled(
                pub permission_disabled::Field0,
                pub permission_disabled::Field1,
            );
            pub mod permission_disabled {
                use super::runtime_types;
                pub type Field0 = ::subxt::utils::AccountId32;
                pub type Field1 = [::core::primitive::u8; 32usize];
            }
            impl ::subxt::events::StaticEvent for PermissionDisabled {
                const PALLET: &'static str = "PeaqRbac";
                const EVENT: &'static str = "PermissionDisabled";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct PermissionAssigned(
                pub permission_assigned::Field0,
                pub permission_assigned::Field1,
                pub permission_assigned::Field2,
            );
            pub mod permission_assigned {
                use super::runtime_types;
                pub type Field0 = ::subxt::utils::AccountId32;
                pub type Field1 = [::core::primitive::u8; 32usize];
                pub type Field2 = [::core::primitive::u8; 32usize];
            }
            impl ::subxt::events::StaticEvent for PermissionAssigned {
                const PALLET: &'static str = "PeaqRbac";
                const EVENT: &'static str = "PermissionAssigned";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct PermissionUnassignedToRole(
                pub permission_unassigned_to_role::Field0,
                pub permission_unassigned_to_role::Field1,
                pub permission_unassigned_to_role::Field2,
            );
            pub mod permission_unassigned_to_role {
                use super::runtime_types;
                pub type Field0 = ::subxt::utils::AccountId32;
                pub type Field1 = [::core::primitive::u8; 32usize];
                pub type Field2 = [::core::primitive::u8; 32usize];
            }
            impl ::subxt::events::StaticEvent for PermissionUnassignedToRole {
                const PALLET: &'static str = "PeaqRbac";
                const EVENT: &'static str = "PermissionUnassignedToRole";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct FetchedRolePermissions(pub fetched_role_permissions::Field0);
            pub mod fetched_role_permissions {
                use super::runtime_types;
                pub type Field0 = ::std::vec::Vec<
                    runtime_types::peaq_pallet_rbac::structs::Permission2Role<
                        [::core::primitive::u8; 32usize],
                    >,
                >;
            }
            impl ::subxt::events::StaticEvent for FetchedRolePermissions {
                const PALLET: &'static str = "PeaqRbac";
                const EVENT: &'static str = "FetchedRolePermissions";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct PermissionFetched(pub permission_fetched::Field0);
            pub mod permission_fetched {
                use super::runtime_types;
                pub type Field0 = runtime_types::peaq_pallet_rbac::structs::Entity<
                    [::core::primitive::u8; 32usize],
                >;
            }
            impl ::subxt::events::StaticEvent for PermissionFetched {
                const PALLET: &'static str = "PeaqRbac";
                const EVENT: &'static str = "PermissionFetched";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct AllPermissionsFetched(pub all_permissions_fetched::Field0);
            pub mod all_permissions_fetched {
                use super::runtime_types;
                pub type Field0 = ::std::vec::Vec<
                    runtime_types::peaq_pallet_rbac::structs::Entity<
                        [::core::primitive::u8; 32usize],
                    >,
                >;
            }
            impl ::subxt::events::StaticEvent for AllPermissionsFetched {
                const PALLET: &'static str = "PeaqRbac";
                const EVENT: &'static str = "AllPermissionsFetched";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct GroupFetched(pub group_fetched::Field0);
            pub mod group_fetched {
                use super::runtime_types;
                pub type Field0 = runtime_types::peaq_pallet_rbac::structs::Entity<
                    [::core::primitive::u8; 32usize],
                >;
            }
            impl ::subxt::events::StaticEvent for GroupFetched {
                const PALLET: &'static str = "PeaqRbac";
                const EVENT: &'static str = "GroupFetched";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct AllGroupsFetched(pub all_groups_fetched::Field0);
            pub mod all_groups_fetched {
                use super::runtime_types;
                pub type Field0 = ::std::vec::Vec<
                    runtime_types::peaq_pallet_rbac::structs::Entity<
                        [::core::primitive::u8; 32usize],
                    >,
                >;
            }
            impl ::subxt::events::StaticEvent for AllGroupsFetched {
                const PALLET: &'static str = "PeaqRbac";
                const EVENT: &'static str = "AllGroupsFetched";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct GroupAdded(
                pub group_added::Field0,
                pub group_added::Field1,
                pub group_added::Field2,
            );
            pub mod group_added {
                use super::runtime_types;
                pub type Field0 = ::subxt::utils::AccountId32;
                pub type Field1 = [::core::primitive::u8; 32usize];
                pub type Field2 = ::std::vec::Vec<::core::primitive::u8>;
            }
            impl ::subxt::events::StaticEvent for GroupAdded {
                const PALLET: &'static str = "PeaqRbac";
                const EVENT: &'static str = "GroupAdded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct GroupUpdated(
                pub group_updated::Field0,
                pub group_updated::Field1,
                pub group_updated::Field2,
            );
            pub mod group_updated {
                use super::runtime_types;
                pub type Field0 = ::subxt::utils::AccountId32;
                pub type Field1 = [::core::primitive::u8; 32usize];
                pub type Field2 = ::std::vec::Vec<::core::primitive::u8>;
            }
            impl ::subxt::events::StaticEvent for GroupUpdated {
                const PALLET: &'static str = "PeaqRbac";
                const EVENT: &'static str = "GroupUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct GroupDisabled(pub group_disabled::Field0, pub group_disabled::Field1);
            pub mod group_disabled {
                use super::runtime_types;
                pub type Field0 = ::subxt::utils::AccountId32;
                pub type Field1 = [::core::primitive::u8; 32usize];
            }
            impl ::subxt::events::StaticEvent for GroupDisabled {
                const PALLET: &'static str = "PeaqRbac";
                const EVENT: &'static str = "GroupDisabled";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct UserAssignedToGroup(
                pub user_assigned_to_group::Field0,
                pub user_assigned_to_group::Field1,
                pub user_assigned_to_group::Field2,
            );
            pub mod user_assigned_to_group {
                use super::runtime_types;
                pub type Field0 = ::subxt::utils::AccountId32;
                pub type Field1 = [::core::primitive::u8; 32usize];
                pub type Field2 = [::core::primitive::u8; 32usize];
            }
            impl ::subxt::events::StaticEvent for UserAssignedToGroup {
                const PALLET: &'static str = "PeaqRbac";
                const EVENT: &'static str = "UserAssignedToGroup";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct UserUnAssignedToGroup(
                pub user_un_assigned_to_group::Field0,
                pub user_un_assigned_to_group::Field1,
                pub user_un_assigned_to_group::Field2,
            );
            pub mod user_un_assigned_to_group {
                use super::runtime_types;
                pub type Field0 = ::subxt::utils::AccountId32;
                pub type Field1 = [::core::primitive::u8; 32usize];
                pub type Field2 = [::core::primitive::u8; 32usize];
            }
            impl ::subxt::events::StaticEvent for UserUnAssignedToGroup {
                const PALLET: &'static str = "PeaqRbac";
                const EVENT: &'static str = "UserUnAssignedToGroup";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod role_store {
                    use super::runtime_types;
                    pub type RoleStore = ::std::vec::Vec<
                        runtime_types::peaq_pallet_rbac::structs::Entity<
                            [::core::primitive::u8; 32usize],
                        >,
                    >;
                    pub type Param0 = ::subxt::utils::AccountId32;
                }
                pub mod role2_user_store {
                    use super::runtime_types;
                    pub type Role2UserStore = ::std::vec::Vec<
                        runtime_types::peaq_pallet_rbac::structs::Role2User<
                            [::core::primitive::u8; 32usize],
                        >,
                    >;
                    pub type Param0 = [::core::primitive::u8; 32usize];
                }
                pub mod permission_store {
                    use super::runtime_types;
                    pub type PermissionStore = ::std::vec::Vec<
                        runtime_types::peaq_pallet_rbac::structs::Entity<
                            [::core::primitive::u8; 32usize],
                        >,
                    >;
                    pub type Param0 = ::subxt::utils::AccountId32;
                }
                pub mod permission2_role_store {
                    use super::runtime_types;
                    pub type Permission2RoleStore = ::std::vec::Vec<
                        runtime_types::peaq_pallet_rbac::structs::Permission2Role<
                            [::core::primitive::u8; 32usize],
                        >,
                    >;
                    pub type Param0 = [::core::primitive::u8; 32usize];
                }
                pub mod group_store {
                    use super::runtime_types;
                    pub type GroupStore = ::std::vec::Vec<
                        runtime_types::peaq_pallet_rbac::structs::Entity<
                            [::core::primitive::u8; 32usize],
                        >,
                    >;
                    pub type Param0 = ::subxt::utils::AccountId32;
                }
                pub mod role2_group_store {
                    use super::runtime_types;
                    pub type Role2GroupStore = ::std::vec::Vec<
                        runtime_types::peaq_pallet_rbac::structs::Role2Group<
                            [::core::primitive::u8; 32usize],
                        >,
                    >;
                    pub type Param0 = [::core::primitive::u8; 32usize];
                }
                pub mod user2_group_store {
                    use super::runtime_types;
                    pub type User2GroupStore = ::std::vec::Vec<
                        runtime_types::peaq_pallet_rbac::structs::User2Group<
                            [::core::primitive::u8; 32usize],
                        >,
                    >;
                    pub type Param0 = [::core::primitive::u8; 32usize];
                }
                pub mod keys_look_up_store {
                    use super::runtime_types;
                    pub type KeysLookUpStore = runtime_types::peaq_pallet_rbac::structs::Entity<
                        [::core::primitive::u8; 32usize],
                    >;
                    pub type Param0 = [::core::primitive::u8; 32usize];
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn role_store_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::role_store::RoleStore,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PeaqRbac",
                        "RoleStore",
                        vec![],
                        [
                            214u8, 103u8, 180u8, 5u8, 212u8, 149u8, 225u8, 15u8, 192u8, 231u8,
                            73u8, 219u8, 131u8, 244u8, 78u8, 78u8, 243u8, 66u8, 119u8, 99u8, 52u8,
                            20u8, 55u8, 250u8, 255u8, 232u8, 38u8, 249u8, 187u8, 72u8, 126u8,
                            127u8,
                        ],
                    )
                }
                pub fn role_store(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::role_store::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::role_store::RoleStore,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PeaqRbac",
                        "RoleStore",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            214u8, 103u8, 180u8, 5u8, 212u8, 149u8, 225u8, 15u8, 192u8, 231u8,
                            73u8, 219u8, 131u8, 244u8, 78u8, 78u8, 243u8, 66u8, 119u8, 99u8, 52u8,
                            20u8, 55u8, 250u8, 255u8, 232u8, 38u8, 249u8, 187u8, 72u8, 126u8,
                            127u8,
                        ],
                    )
                }
                pub fn role2_user_store_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::role2_user_store::Role2UserStore,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PeaqRbac",
                        "Role2UserStore",
                        vec![],
                        [
                            176u8, 105u8, 152u8, 118u8, 5u8, 121u8, 238u8, 201u8, 82u8, 115u8,
                            90u8, 184u8, 198u8, 246u8, 169u8, 144u8, 125u8, 162u8, 25u8, 188u8,
                            144u8, 14u8, 198u8, 119u8, 89u8, 172u8, 63u8, 119u8, 7u8, 73u8, 231u8,
                            1u8,
                        ],
                    )
                }
                pub fn role2_user_store(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::role2_user_store::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::role2_user_store::Role2UserStore,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PeaqRbac",
                        "Role2UserStore",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            176u8, 105u8, 152u8, 118u8, 5u8, 121u8, 238u8, 201u8, 82u8, 115u8,
                            90u8, 184u8, 198u8, 246u8, 169u8, 144u8, 125u8, 162u8, 25u8, 188u8,
                            144u8, 14u8, 198u8, 119u8, 89u8, 172u8, 63u8, 119u8, 7u8, 73u8, 231u8,
                            1u8,
                        ],
                    )
                }
                pub fn permission_store_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::permission_store::PermissionStore,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PeaqRbac",
                        "PermissionStore",
                        vec![],
                        [
                            12u8, 54u8, 183u8, 22u8, 122u8, 96u8, 89u8, 180u8, 166u8, 101u8, 115u8,
                            7u8, 240u8, 150u8, 32u8, 153u8, 158u8, 221u8, 224u8, 204u8, 214u8, 1u8,
                            184u8, 84u8, 11u8, 186u8, 68u8, 188u8, 138u8, 227u8, 207u8, 55u8,
                        ],
                    )
                }
                pub fn permission_store(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::permission_store::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::permission_store::PermissionStore,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PeaqRbac",
                        "PermissionStore",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            12u8, 54u8, 183u8, 22u8, 122u8, 96u8, 89u8, 180u8, 166u8, 101u8, 115u8,
                            7u8, 240u8, 150u8, 32u8, 153u8, 158u8, 221u8, 224u8, 204u8, 214u8, 1u8,
                            184u8, 84u8, 11u8, 186u8, 68u8, 188u8, 138u8, 227u8, 207u8, 55u8,
                        ],
                    )
                }
                pub fn permission2_role_store_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::permission2_role_store::Permission2RoleStore,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PeaqRbac",
                        "Permission2RoleStore",
                        vec![],
                        [
                            181u8, 232u8, 98u8, 171u8, 240u8, 127u8, 33u8, 213u8, 222u8, 248u8,
                            113u8, 201u8, 53u8, 101u8, 22u8, 13u8, 88u8, 91u8, 212u8, 10u8, 17u8,
                            52u8, 50u8, 252u8, 246u8, 248u8, 147u8, 37u8, 239u8, 135u8, 223u8,
                            153u8,
                        ],
                    )
                }
                pub fn permission2_role_store(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::permission2_role_store::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::permission2_role_store::Permission2RoleStore,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PeaqRbac",
                        "Permission2RoleStore",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            181u8, 232u8, 98u8, 171u8, 240u8, 127u8, 33u8, 213u8, 222u8, 248u8,
                            113u8, 201u8, 53u8, 101u8, 22u8, 13u8, 88u8, 91u8, 212u8, 10u8, 17u8,
                            52u8, 50u8, 252u8, 246u8, 248u8, 147u8, 37u8, 239u8, 135u8, 223u8,
                            153u8,
                        ],
                    )
                }
                pub fn group_store_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::group_store::GroupStore,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PeaqRbac",
                        "GroupStore",
                        vec![],
                        [
                            16u8, 167u8, 255u8, 114u8, 171u8, 255u8, 36u8, 194u8, 64u8, 5u8, 126u8,
                            92u8, 85u8, 238u8, 241u8, 234u8, 236u8, 95u8, 45u8, 56u8, 127u8, 121u8,
                            21u8, 133u8, 207u8, 79u8, 93u8, 163u8, 225u8, 63u8, 245u8, 100u8,
                        ],
                    )
                }
                pub fn group_store(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::group_store::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::group_store::GroupStore,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PeaqRbac",
                        "GroupStore",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            16u8, 167u8, 255u8, 114u8, 171u8, 255u8, 36u8, 194u8, 64u8, 5u8, 126u8,
                            92u8, 85u8, 238u8, 241u8, 234u8, 236u8, 95u8, 45u8, 56u8, 127u8, 121u8,
                            21u8, 133u8, 207u8, 79u8, 93u8, 163u8, 225u8, 63u8, 245u8, 100u8,
                        ],
                    )
                }
                pub fn role2_group_store_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::role2_group_store::Role2GroupStore,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PeaqRbac",
                        "Role2GroupStore",
                        vec![],
                        [
                            81u8, 128u8, 119u8, 164u8, 154u8, 48u8, 110u8, 108u8, 92u8, 229u8,
                            195u8, 201u8, 0u8, 57u8, 173u8, 147u8, 157u8, 98u8, 198u8, 100u8,
                            253u8, 108u8, 222u8, 75u8, 255u8, 38u8, 239u8, 196u8, 89u8, 197u8,
                            184u8, 3u8,
                        ],
                    )
                }
                pub fn role2_group_store(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::role2_group_store::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::role2_group_store::Role2GroupStore,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PeaqRbac",
                        "Role2GroupStore",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            81u8, 128u8, 119u8, 164u8, 154u8, 48u8, 110u8, 108u8, 92u8, 229u8,
                            195u8, 201u8, 0u8, 57u8, 173u8, 147u8, 157u8, 98u8, 198u8, 100u8,
                            253u8, 108u8, 222u8, 75u8, 255u8, 38u8, 239u8, 196u8, 89u8, 197u8,
                            184u8, 3u8,
                        ],
                    )
                }
                pub fn user2_group_store_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::user2_group_store::User2GroupStore,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PeaqRbac",
                        "User2GroupStore",
                        vec![],
                        [
                            223u8, 199u8, 19u8, 69u8, 114u8, 109u8, 174u8, 254u8, 108u8, 68u8,
                            226u8, 191u8, 35u8, 207u8, 46u8, 170u8, 106u8, 32u8, 205u8, 199u8,
                            47u8, 25u8, 204u8, 241u8, 196u8, 59u8, 56u8, 57u8, 206u8, 133u8, 19u8,
                            192u8,
                        ],
                    )
                }
                pub fn user2_group_store(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::user2_group_store::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::user2_group_store::User2GroupStore,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PeaqRbac",
                        "User2GroupStore",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            223u8, 199u8, 19u8, 69u8, 114u8, 109u8, 174u8, 254u8, 108u8, 68u8,
                            226u8, 191u8, 35u8, 207u8, 46u8, 170u8, 106u8, 32u8, 205u8, 199u8,
                            47u8, 25u8, 204u8, 241u8, 196u8, 59u8, 56u8, 57u8, 206u8, 133u8, 19u8,
                            192u8,
                        ],
                    )
                }
                pub fn keys_look_up_store_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::keys_look_up_store::KeysLookUpStore,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PeaqRbac",
                        "KeysLookUpStore",
                        vec![],
                        [
                            66u8, 252u8, 100u8, 63u8, 102u8, 65u8, 155u8, 178u8, 91u8, 212u8,
                            177u8, 115u8, 69u8, 252u8, 201u8, 125u8, 20u8, 138u8, 245u8, 105u8,
                            202u8, 25u8, 207u8, 105u8, 228u8, 55u8, 16u8, 118u8, 227u8, 101u8,
                            99u8, 115u8,
                        ],
                    )
                }
                pub fn keys_look_up_store(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::keys_look_up_store::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::keys_look_up_store::KeysLookUpStore,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "PeaqRbac",
                        "KeysLookUpStore",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            66u8, 252u8, 100u8, 63u8, 102u8, 65u8, 155u8, 178u8, 91u8, 212u8,
                            177u8, 115u8, 69u8, 252u8, 201u8, 125u8, 20u8, 138u8, 245u8, 105u8,
                            202u8, 25u8, 207u8, 105u8, 228u8, 55u8, 16u8, 118u8, 227u8, 101u8,
                            99u8, 115u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod bounded_collections {
            use super::runtime_types;
            pub mod bounded_vec {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
            }
            pub mod weak_bounded_vec {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
            }
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod dispatch {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum DispatchClass {
                    #[codec(index = 0)]
                    Normal,
                    #[codec(index = 1)]
                    Operational,
                    #[codec(index = 2)]
                    Mandatory,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct DispatchInfo {
                    pub weight: runtime_types::sp_weights::weight_v2::Weight,
                    pub class: runtime_types::frame_support::dispatch::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::dispatch::Pays,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Pays {
                    #[codec(index = 0)]
                    Yes,
                    #[codec(index = 1)]
                    No,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: ext :: codec :: Decode,
                            :: subxt :: ext :: codec :: Encode,
                            :: subxt :: ext :: scale_decode :: DecodeAsType,
                            :: subxt :: ext :: scale_encode :: EncodeAsType,
                            Debug,
                        )]
                        # [codec (crate = :: subxt :: ext :: codec)]
                        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                        pub enum BalanceStatus {
                            #[codec(index = 0)]
                            Free,
                            #[codec(index = 1)]
                            Reserved,
                        }
                    }
                }
            }
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckGenesis;
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckSpecVersion;
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckTxVersion;
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckWeight;
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BlockLength {
                    pub max: runtime_types::frame_support::dispatch::PerDispatchClass<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BlockWeights {
                    pub base_block: runtime_types::sp_weights::weight_v2::Weight,
                    pub max_block: runtime_types::sp_weights::weight_v2::Weight,
                    pub per_class: runtime_types::frame_support::dispatch::PerDispatchClass<
                        runtime_types::frame_system::limits::WeightsPerClass,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct WeightsPerClass {
                    pub base_extrinsic: runtime_types::sp_weights::weight_v2::Weight,
                    pub max_extrinsic:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                    pub max_total:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                    pub reserved:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Call {
                    #[codec(index = 0)]
                    remark {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    set_heap_pages { pages: ::core::primitive::u64 },
                    #[codec(index = 2)]
                    set_code {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    set_code_without_checks {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    set_storage {
                        items: ::std::vec::Vec<(
                            ::std::vec::Vec<::core::primitive::u8>,
                            ::std::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 5)]
                    kill_storage {
                        keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    },
                    #[codec(index = 6)]
                    kill_prefix {
                        prefix: ::std::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 7)]
                    remark_with_event {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidSpecName,
                    #[codec(index = 1)]
                    SpecVersionNeedsToIncrease,
                    #[codec(index = 2)]
                    FailedToExtractRuntimeVersion,
                    #[codec(index = 3)]
                    NonDefaultComposite,
                    #[codec(index = 4)]
                    NonZeroRefCount,
                    #[codec(index = 5)]
                    CallFiltered,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Event {
                    #[codec(index = 0)]
                    ExtrinsicSuccess {
                        dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
                    },
                    #[codec(index = 1)]
                    ExtrinsicFailed {
                        dispatch_error: runtime_types::sp_runtime::DispatchError,
                        dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
                    },
                    #[codec(index = 2)]
                    CodeUpdated,
                    #[codec(index = 3)]
                    NewAccount {
                        account: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 4)]
                    KilledAccount {
                        account: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 5)]
                    Remarked {
                        sender: ::subxt::utils::AccountId32,
                        hash: ::subxt::utils::H256,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: ::core::primitive::u32,
                pub providers: ::core::primitive::u32,
                pub sufficients: ::core::primitive::u32,
                pub data: _1,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::std::vec::Vec<_1>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::std::string::String,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Phase {
                #[codec(index = 0)]
                ApplyExtrinsic(::core::primitive::u32),
                #[codec(index = 1)]
                Finalization,
                #[codec(index = 2)]
                Initialization,
            }
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Call {
                    #[codec(index = 0)]
                    transfer_allow_death {
                        dest: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    set_balance_deprecated {
                        who: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                        #[codec(compact)]
                        old_reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    force_transfer {
                        source: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        dest: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    transfer_keep_alive {
                        dest: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    transfer_all {
                        dest: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    force_unreserve {
                        who: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    upgrade_accounts {
                        who: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    },
                    #[codec(index = 7)]
                    transfer {
                        dest: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    force_set_balance {
                        who: ::subxt::utils::MultiAddress<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Error {
                    #[codec(index = 0)]
                    VestingBalance,
                    #[codec(index = 1)]
                    LiquidityRestrictions,
                    #[codec(index = 2)]
                    InsufficientBalance,
                    #[codec(index = 3)]
                    ExistentialDeposit,
                    #[codec(index = 4)]
                    Expendability,
                    #[codec(index = 5)]
                    ExistingVestingSchedule,
                    #[codec(index = 6)]
                    DeadAccount,
                    #[codec(index = 7)]
                    TooManyReserves,
                    #[codec(index = 8)]
                    TooManyHolds,
                    #[codec(index = 9)]
                    TooManyFreezes,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Event {
                    #[codec(index = 0)]
                    Endowed {
                        account: ::subxt::utils::AccountId32,
                        free_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    DustLost {
                        account: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    Transfer {
                        from: ::subxt::utils::AccountId32,
                        to: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    BalanceSet {
                        who: ::subxt::utils::AccountId32,
                        free: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    Reserved {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    Unreserved {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    ReserveRepatriated {
                        from: ::subxt::utils::AccountId32,
                        to: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                        destination_status:
                            runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    },
                    #[codec(index = 7)]
                    Deposit {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    Withdraw {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    Slashed {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 10)]
                    Minted {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 11)]
                    Burned {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 12)]
                    Suspended {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 13)]
                    Restored {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 14)]
                    Upgraded { who: ::subxt::utils::AccountId32 },
                    #[codec(index = 15)]
                    Issued { amount: ::core::primitive::u128 },
                    #[codec(index = 16)]
                    Rescinded { amount: ::core::primitive::u128 },
                    #[codec(index = 17)]
                    Locked {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 18)]
                    Unlocked {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 19)]
                    Frozen {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 20)]
                    Thawed {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AccountData<_0> {
                    pub free: _0,
                    pub reserved: _0,
                    pub frozen: _0,
                    pub flags: runtime_types::pallet_balances::types::ExtraFlags,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BalanceLock<_0> {
                    pub id: [::core::primitive::u8; 8usize],
                    pub amount: _0,
                    pub reasons: runtime_types::pallet_balances::types::Reasons,
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ExtraFlags(pub ::core::primitive::u128);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct IdAmount<_0, _1> {
                    pub id: _0,
                    pub amount: _1,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Reasons {
                    #[codec(index = 0)]
                    Fee,
                    #[codec(index = 1)]
                    Misc,
                    #[codec(index = 2)]
                    All,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ReserveData<_0, _1> {
                    pub id: _0,
                    pub amount: _1,
                }
            }
        }
        pub mod pallet_transaction_payment {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
        }
        pub mod peaq_dev_runtime {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Runtime;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum RuntimeCall {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Call),
                #[codec(index = 4)]
                Balances(runtime_types::pallet_balances::pallet::Call),
                #[codec(index = 100)]
                PeaqDid(runtime_types::peaq_pallet_did::pallet::Call),
                #[codec(index = 103)]
                PeaqRbac(runtime_types::peaq_pallet_rbac::pallet::Call),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum RuntimeError {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Error),
                #[codec(index = 4)]
                Balances(runtime_types::pallet_balances::pallet::Error),
                #[codec(index = 100)]
                PeaqDid(runtime_types::peaq_pallet_did::pallet::Error),
                #[codec(index = 103)]
                PeaqRbac(runtime_types::peaq_pallet_rbac::pallet::Error),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum RuntimeEvent {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Event),
                #[codec(index = 4)]
                Balances(runtime_types::pallet_balances::pallet::Event),
                #[codec(index = 100)]
                PeaqDid(runtime_types::peaq_pallet_did::pallet::Event),
                #[codec(index = 103)]
                PeaqRbac(runtime_types::peaq_pallet_rbac::pallet::Event),
            }
        }
        pub mod peaq_pallet_did {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Call {
                    #[codec(index = 0)]
                    add_attribute {
                        did_account: ::subxt::utils::AccountId32,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        value: ::std::vec::Vec<::core::primitive::u8>,
                        valid_for: ::core::option::Option<::core::primitive::u32>,
                    },
                    #[codec(index = 1)]
                    update_attribute {
                        did_account: ::subxt::utils::AccountId32,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        value: ::std::vec::Vec<::core::primitive::u8>,
                        valid_for: ::core::option::Option<::core::primitive::u32>,
                    },
                    #[codec(index = 2)]
                    read_attribute {
                        did_account: ::subxt::utils::AccountId32,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    remove_attribute {
                        did_account: ::subxt::utils::AccountId32,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Error {
                    #[codec(index = 0)]
                    AttributeNameExceedMax64,
                    #[codec(index = 1)]
                    AttributeAlreadyExist,
                    #[codec(index = 2)]
                    AttributeCreationFailed,
                    #[codec(index = 3)]
                    AttributeUpdateFailed,
                    #[codec(index = 4)]
                    AttributeNotFound,
                    #[codec(index = 5)]
                    AttributeAuthorizationFailed,
                    #[codec(index = 6)]
                    MaxBlockNumberExceeded,
                    #[codec(index = 7)]
                    InvalidSuppliedValue,
                    #[codec(index = 8)]
                    ParseError,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Event {
                    #[codec(index = 0)]
                    AttributeAdded(
                        ::subxt::utils::AccountId32,
                        ::subxt::utils::AccountId32,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::core::option::Option<::core::primitive::u32>,
                    ),
                    #[codec(index = 1)]
                    AttributeRead(
                        runtime_types::peaq_pallet_did::structs::Attribute<
                            ::core::primitive::u32,
                            ::core::primitive::u64,
                        >,
                    ),
                    #[codec(index = 2)]
                    AttributeUpdated(
                        ::subxt::utils::AccountId32,
                        ::subxt::utils::AccountId32,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::core::option::Option<::core::primitive::u32>,
                    ),
                    #[codec(index = 3)]
                    AttributeRemoved(
                        ::subxt::utils::AccountId32,
                        ::subxt::utils::AccountId32,
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                }
            }
            pub mod structs {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Attribute<_0, _1> {
                    pub name: ::std::vec::Vec<::core::primitive::u8>,
                    pub value: ::std::vec::Vec<::core::primitive::u8>,
                    pub validity: _0,
                    pub created: _1,
                }
            }
        }
        pub mod peaq_pallet_rbac {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Call {
                    #[codec(index = 0)]
                    fetch_role {
                        owner: ::subxt::utils::AccountId32,
                        entity: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 1)]
                    fetch_roles { owner: ::subxt::utils::AccountId32 },
                    #[codec(index = 2)]
                    add_role {
                        role_id: [::core::primitive::u8; 32usize],
                        name: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    update_role {
                        role_id: [::core::primitive::u8; 32usize],
                        name: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    disable_role {
                        role_id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 5)]
                    fetch_user_roles {
                        owner: ::subxt::utils::AccountId32,
                        user_id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 6)]
                    assign_role_to_user {
                        role_id: [::core::primitive::u8; 32usize],
                        user_id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 7)]
                    unassign_role_to_user {
                        role_id: [::core::primitive::u8; 32usize],
                        user_id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 8)]
                    fetch_permission {
                        owner: ::subxt::utils::AccountId32,
                        permission_id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 9)]
                    fetch_permissions { owner: ::subxt::utils::AccountId32 },
                    #[codec(index = 10)]
                    add_permission {
                        permission_id: [::core::primitive::u8; 32usize],
                        name: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 11)]
                    update_permission {
                        permission_id: [::core::primitive::u8; 32usize],
                        name: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 12)]
                    disable_permission {
                        permission_id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 13)]
                    fetch_role_permissions {
                        owner: ::subxt::utils::AccountId32,
                        role_id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 14)]
                    assign_permission_to_role {
                        permission_id: [::core::primitive::u8; 32usize],
                        role_id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 15)]
                    unassign_permission_to_role {
                        permission_id: [::core::primitive::u8; 32usize],
                        role_id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 16)]
                    fetch_group {
                        owner: ::subxt::utils::AccountId32,
                        group_id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 17)]
                    fetch_groups { owner: ::subxt::utils::AccountId32 },
                    #[codec(index = 18)]
                    add_group {
                        group_id: [::core::primitive::u8; 32usize],
                        name: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 19)]
                    update_group {
                        group_id: [::core::primitive::u8; 32usize],
                        name: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 20)]
                    disable_group {
                        group_id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 21)]
                    assign_role_to_group {
                        role_id: [::core::primitive::u8; 32usize],
                        group_id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 22)]
                    unassign_role_to_group {
                        role_id: [::core::primitive::u8; 32usize],
                        group_id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 23)]
                    fetch_group_roles {
                        owner: ::subxt::utils::AccountId32,
                        group_id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 24)]
                    assign_user_to_group {
                        user_id: [::core::primitive::u8; 32usize],
                        group_id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 25)]
                    unassign_user_to_group {
                        user_id: [::core::primitive::u8; 32usize],
                        group_id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 26)]
                    fetch_user_groups {
                        owner: ::subxt::utils::AccountId32,
                        user_id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 27)]
                    fetch_user_permissions {
                        owner: ::subxt::utils::AccountId32,
                        user_id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 28)]
                    fetch_group_permissions {
                        owner: ::subxt::utils::AccountId32,
                        group_id: [::core::primitive::u8; 32usize],
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Error {
                    #[codec(index = 0)]
                    EntityNameExceedMax64,
                    #[codec(index = 1)]
                    EntityAlreadyExist,
                    #[codec(index = 2)]
                    EntityDoesNotExist,
                    #[codec(index = 3)]
                    EntityDisabled,
                    #[codec(index = 4)]
                    EntityAuthorizationFailed,
                    #[codec(index = 5)]
                    AssignmentAlreadyExist,
                    #[codec(index = 6)]
                    AssignmentDoesNotExist,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Event {
                    #[codec(index = 0)]
                    RoleAdded(
                        ::subxt::utils::AccountId32,
                        [::core::primitive::u8; 32usize],
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    #[codec(index = 1)]
                    RoleUpdated(
                        ::subxt::utils::AccountId32,
                        [::core::primitive::u8; 32usize],
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    #[codec(index = 2)]
                    RoleRemoved(::subxt::utils::AccountId32, [::core::primitive::u8; 32usize]),
                    #[codec(index = 3)]
                    RoleFetched(
                        runtime_types::peaq_pallet_rbac::structs::Entity<
                            [::core::primitive::u8; 32usize],
                        >,
                    ),
                    #[codec(index = 4)]
                    AllRolesFetched(
                        ::std::vec::Vec<
                            runtime_types::peaq_pallet_rbac::structs::Entity<
                                [::core::primitive::u8; 32usize],
                            >,
                        >,
                    ),
                    #[codec(index = 5)]
                    RoleAssignedToUser(
                        ::subxt::utils::AccountId32,
                        [::core::primitive::u8; 32usize],
                        [::core::primitive::u8; 32usize],
                    ),
                    #[codec(index = 6)]
                    RoleUnassignedToUser(
                        ::subxt::utils::AccountId32,
                        [::core::primitive::u8; 32usize],
                        [::core::primitive::u8; 32usize],
                    ),
                    #[codec(index = 7)]
                    RoleAssignedToGroup(
                        ::subxt::utils::AccountId32,
                        [::core::primitive::u8; 32usize],
                        [::core::primitive::u8; 32usize],
                    ),
                    #[codec(index = 8)]
                    RoleUnassignedToGroup(
                        ::subxt::utils::AccountId32,
                        [::core::primitive::u8; 32usize],
                        [::core::primitive::u8; 32usize],
                    ),
                    #[codec(index = 9)]
                    FetchedGroupRoles(
                        ::std::vec::Vec<
                            runtime_types::peaq_pallet_rbac::structs::Role2Group<
                                [::core::primitive::u8; 32usize],
                            >,
                        >,
                    ),
                    #[codec(index = 10)]
                    FetchedUserRoles(
                        ::std::vec::Vec<
                            runtime_types::peaq_pallet_rbac::structs::Role2User<
                                [::core::primitive::u8; 32usize],
                            >,
                        >,
                    ),
                    #[codec(index = 11)]
                    FetchedUserGroups(
                        ::std::vec::Vec<
                            runtime_types::peaq_pallet_rbac::structs::User2Group<
                                [::core::primitive::u8; 32usize],
                            >,
                        >,
                    ),
                    #[codec(index = 12)]
                    FetchedUserPermissions(
                        ::std::vec::Vec<
                            runtime_types::peaq_pallet_rbac::structs::Entity<
                                [::core::primitive::u8; 32usize],
                            >,
                        >,
                    ),
                    #[codec(index = 13)]
                    FetchedGroupPermissions(
                        ::std::vec::Vec<
                            runtime_types::peaq_pallet_rbac::structs::Entity<
                                [::core::primitive::u8; 32usize],
                            >,
                        >,
                    ),
                    #[codec(index = 14)]
                    PermissionAdded(
                        ::subxt::utils::AccountId32,
                        [::core::primitive::u8; 32usize],
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    #[codec(index = 15)]
                    PermissionUpdated(
                        ::subxt::utils::AccountId32,
                        [::core::primitive::u8; 32usize],
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    #[codec(index = 16)]
                    PermissionDisabled(
                        ::subxt::utils::AccountId32,
                        [::core::primitive::u8; 32usize],
                    ),
                    #[codec(index = 17)]
                    PermissionAssigned(
                        ::subxt::utils::AccountId32,
                        [::core::primitive::u8; 32usize],
                        [::core::primitive::u8; 32usize],
                    ),
                    #[codec(index = 18)]
                    PermissionUnassignedToRole(
                        ::subxt::utils::AccountId32,
                        [::core::primitive::u8; 32usize],
                        [::core::primitive::u8; 32usize],
                    ),
                    #[codec(index = 19)]
                    FetchedRolePermissions(
                        ::std::vec::Vec<
                            runtime_types::peaq_pallet_rbac::structs::Permission2Role<
                                [::core::primitive::u8; 32usize],
                            >,
                        >,
                    ),
                    #[codec(index = 20)]
                    PermissionFetched(
                        runtime_types::peaq_pallet_rbac::structs::Entity<
                            [::core::primitive::u8; 32usize],
                        >,
                    ),
                    #[codec(index = 21)]
                    AllPermissionsFetched(
                        ::std::vec::Vec<
                            runtime_types::peaq_pallet_rbac::structs::Entity<
                                [::core::primitive::u8; 32usize],
                            >,
                        >,
                    ),
                    #[codec(index = 22)]
                    GroupFetched(
                        runtime_types::peaq_pallet_rbac::structs::Entity<
                            [::core::primitive::u8; 32usize],
                        >,
                    ),
                    #[codec(index = 23)]
                    AllGroupsFetched(
                        ::std::vec::Vec<
                            runtime_types::peaq_pallet_rbac::structs::Entity<
                                [::core::primitive::u8; 32usize],
                            >,
                        >,
                    ),
                    #[codec(index = 24)]
                    GroupAdded(
                        ::subxt::utils::AccountId32,
                        [::core::primitive::u8; 32usize],
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    #[codec(index = 25)]
                    GroupUpdated(
                        ::subxt::utils::AccountId32,
                        [::core::primitive::u8; 32usize],
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    #[codec(index = 26)]
                    GroupDisabled(::subxt::utils::AccountId32, [::core::primitive::u8; 32usize]),
                    #[codec(index = 27)]
                    UserAssignedToGroup(
                        ::subxt::utils::AccountId32,
                        [::core::primitive::u8; 32usize],
                        [::core::primitive::u8; 32usize],
                    ),
                    #[codec(index = 28)]
                    UserUnAssignedToGroup(
                        ::subxt::utils::AccountId32,
                        [::core::primitive::u8; 32usize],
                        [::core::primitive::u8; 32usize],
                    ),
                }
            }
            pub mod structs {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Entity<_0> {
                    pub id: _0,
                    pub name: ::std::vec::Vec<::core::primitive::u8>,
                    pub enabled: ::core::primitive::bool,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Permission2Role<_0> {
                    pub permission: _0,
                    pub role: _0,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Role2Group<_0> {
                    pub role: _0,
                    pub group: _0,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Role2User<_0> {
                    pub role: _0,
                    pub user: _0,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct User2Group<_0> {
                    pub user: _0,
                    pub group: _0,
                }
            }
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum ArithmeticError {
                #[codec(index = 0)]
                Underflow,
                #[codec(index = 1)]
                Overflow,
                #[codec(index = 2)]
                DivisionByZero,
            }
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod ecdsa {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Signature(pub [::core::primitive::u8; 65usize]);
            }
            pub mod ed25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            pub mod sr25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
        }
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod generic {
                use super::runtime_types;
                pub mod digest {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Digest {
                        pub logs:
                            ::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum DigestItem {
                        #[codec(index = 6)]
                        PreRuntime(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 4)]
                        Consensus(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 5)]
                        Seal(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 0)]
                        Other(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 8)]
                        RuntimeEnvironmentUpdated,
                    }
                }
                pub mod era {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum Era {
                        #[codec(index = 0)]
                        Immortal,
                        #[codec(index = 1)]
                        Mortal1(::core::primitive::u8),
                        #[codec(index = 2)]
                        Mortal2(::core::primitive::u8),
                        #[codec(index = 3)]
                        Mortal3(::core::primitive::u8),
                        #[codec(index = 4)]
                        Mortal4(::core::primitive::u8),
                        #[codec(index = 5)]
                        Mortal5(::core::primitive::u8),
                        #[codec(index = 6)]
                        Mortal6(::core::primitive::u8),
                        #[codec(index = 7)]
                        Mortal7(::core::primitive::u8),
                        #[codec(index = 8)]
                        Mortal8(::core::primitive::u8),
                        #[codec(index = 9)]
                        Mortal9(::core::primitive::u8),
                        #[codec(index = 10)]
                        Mortal10(::core::primitive::u8),
                        #[codec(index = 11)]
                        Mortal11(::core::primitive::u8),
                        #[codec(index = 12)]
                        Mortal12(::core::primitive::u8),
                        #[codec(index = 13)]
                        Mortal13(::core::primitive::u8),
                        #[codec(index = 14)]
                        Mortal14(::core::primitive::u8),
                        #[codec(index = 15)]
                        Mortal15(::core::primitive::u8),
                        #[codec(index = 16)]
                        Mortal16(::core::primitive::u8),
                        #[codec(index = 17)]
                        Mortal17(::core::primitive::u8),
                        #[codec(index = 18)]
                        Mortal18(::core::primitive::u8),
                        #[codec(index = 19)]
                        Mortal19(::core::primitive::u8),
                        #[codec(index = 20)]
                        Mortal20(::core::primitive::u8),
                        #[codec(index = 21)]
                        Mortal21(::core::primitive::u8),
                        #[codec(index = 22)]
                        Mortal22(::core::primitive::u8),
                        #[codec(index = 23)]
                        Mortal23(::core::primitive::u8),
                        #[codec(index = 24)]
                        Mortal24(::core::primitive::u8),
                        #[codec(index = 25)]
                        Mortal25(::core::primitive::u8),
                        #[codec(index = 26)]
                        Mortal26(::core::primitive::u8),
                        #[codec(index = 27)]
                        Mortal27(::core::primitive::u8),
                        #[codec(index = 28)]
                        Mortal28(::core::primitive::u8),
                        #[codec(index = 29)]
                        Mortal29(::core::primitive::u8),
                        #[codec(index = 30)]
                        Mortal30(::core::primitive::u8),
                        #[codec(index = 31)]
                        Mortal31(::core::primitive::u8),
                        #[codec(index = 32)]
                        Mortal32(::core::primitive::u8),
                        #[codec(index = 33)]
                        Mortal33(::core::primitive::u8),
                        #[codec(index = 34)]
                        Mortal34(::core::primitive::u8),
                        #[codec(index = 35)]
                        Mortal35(::core::primitive::u8),
                        #[codec(index = 36)]
                        Mortal36(::core::primitive::u8),
                        #[codec(index = 37)]
                        Mortal37(::core::primitive::u8),
                        #[codec(index = 38)]
                        Mortal38(::core::primitive::u8),
                        #[codec(index = 39)]
                        Mortal39(::core::primitive::u8),
                        #[codec(index = 40)]
                        Mortal40(::core::primitive::u8),
                        #[codec(index = 41)]
                        Mortal41(::core::primitive::u8),
                        #[codec(index = 42)]
                        Mortal42(::core::primitive::u8),
                        #[codec(index = 43)]
                        Mortal43(::core::primitive::u8),
                        #[codec(index = 44)]
                        Mortal44(::core::primitive::u8),
                        #[codec(index = 45)]
                        Mortal45(::core::primitive::u8),
                        #[codec(index = 46)]
                        Mortal46(::core::primitive::u8),
                        #[codec(index = 47)]
                        Mortal47(::core::primitive::u8),
                        #[codec(index = 48)]
                        Mortal48(::core::primitive::u8),
                        #[codec(index = 49)]
                        Mortal49(::core::primitive::u8),
                        #[codec(index = 50)]
                        Mortal50(::core::primitive::u8),
                        #[codec(index = 51)]
                        Mortal51(::core::primitive::u8),
                        #[codec(index = 52)]
                        Mortal52(::core::primitive::u8),
                        #[codec(index = 53)]
                        Mortal53(::core::primitive::u8),
                        #[codec(index = 54)]
                        Mortal54(::core::primitive::u8),
                        #[codec(index = 55)]
                        Mortal55(::core::primitive::u8),
                        #[codec(index = 56)]
                        Mortal56(::core::primitive::u8),
                        #[codec(index = 57)]
                        Mortal57(::core::primitive::u8),
                        #[codec(index = 58)]
                        Mortal58(::core::primitive::u8),
                        #[codec(index = 59)]
                        Mortal59(::core::primitive::u8),
                        #[codec(index = 60)]
                        Mortal60(::core::primitive::u8),
                        #[codec(index = 61)]
                        Mortal61(::core::primitive::u8),
                        #[codec(index = 62)]
                        Mortal62(::core::primitive::u8),
                        #[codec(index = 63)]
                        Mortal63(::core::primitive::u8),
                        #[codec(index = 64)]
                        Mortal64(::core::primitive::u8),
                        #[codec(index = 65)]
                        Mortal65(::core::primitive::u8),
                        #[codec(index = 66)]
                        Mortal66(::core::primitive::u8),
                        #[codec(index = 67)]
                        Mortal67(::core::primitive::u8),
                        #[codec(index = 68)]
                        Mortal68(::core::primitive::u8),
                        #[codec(index = 69)]
                        Mortal69(::core::primitive::u8),
                        #[codec(index = 70)]
                        Mortal70(::core::primitive::u8),
                        #[codec(index = 71)]
                        Mortal71(::core::primitive::u8),
                        #[codec(index = 72)]
                        Mortal72(::core::primitive::u8),
                        #[codec(index = 73)]
                        Mortal73(::core::primitive::u8),
                        #[codec(index = 74)]
                        Mortal74(::core::primitive::u8),
                        #[codec(index = 75)]
                        Mortal75(::core::primitive::u8),
                        #[codec(index = 76)]
                        Mortal76(::core::primitive::u8),
                        #[codec(index = 77)]
                        Mortal77(::core::primitive::u8),
                        #[codec(index = 78)]
                        Mortal78(::core::primitive::u8),
                        #[codec(index = 79)]
                        Mortal79(::core::primitive::u8),
                        #[codec(index = 80)]
                        Mortal80(::core::primitive::u8),
                        #[codec(index = 81)]
                        Mortal81(::core::primitive::u8),
                        #[codec(index = 82)]
                        Mortal82(::core::primitive::u8),
                        #[codec(index = 83)]
                        Mortal83(::core::primitive::u8),
                        #[codec(index = 84)]
                        Mortal84(::core::primitive::u8),
                        #[codec(index = 85)]
                        Mortal85(::core::primitive::u8),
                        #[codec(index = 86)]
                        Mortal86(::core::primitive::u8),
                        #[codec(index = 87)]
                        Mortal87(::core::primitive::u8),
                        #[codec(index = 88)]
                        Mortal88(::core::primitive::u8),
                        #[codec(index = 89)]
                        Mortal89(::core::primitive::u8),
                        #[codec(index = 90)]
                        Mortal90(::core::primitive::u8),
                        #[codec(index = 91)]
                        Mortal91(::core::primitive::u8),
                        #[codec(index = 92)]
                        Mortal92(::core::primitive::u8),
                        #[codec(index = 93)]
                        Mortal93(::core::primitive::u8),
                        #[codec(index = 94)]
                        Mortal94(::core::primitive::u8),
                        #[codec(index = 95)]
                        Mortal95(::core::primitive::u8),
                        #[codec(index = 96)]
                        Mortal96(::core::primitive::u8),
                        #[codec(index = 97)]
                        Mortal97(::core::primitive::u8),
                        #[codec(index = 98)]
                        Mortal98(::core::primitive::u8),
                        #[codec(index = 99)]
                        Mortal99(::core::primitive::u8),
                        #[codec(index = 100)]
                        Mortal100(::core::primitive::u8),
                        #[codec(index = 101)]
                        Mortal101(::core::primitive::u8),
                        #[codec(index = 102)]
                        Mortal102(::core::primitive::u8),
                        #[codec(index = 103)]
                        Mortal103(::core::primitive::u8),
                        #[codec(index = 104)]
                        Mortal104(::core::primitive::u8),
                        #[codec(index = 105)]
                        Mortal105(::core::primitive::u8),
                        #[codec(index = 106)]
                        Mortal106(::core::primitive::u8),
                        #[codec(index = 107)]
                        Mortal107(::core::primitive::u8),
                        #[codec(index = 108)]
                        Mortal108(::core::primitive::u8),
                        #[codec(index = 109)]
                        Mortal109(::core::primitive::u8),
                        #[codec(index = 110)]
                        Mortal110(::core::primitive::u8),
                        #[codec(index = 111)]
                        Mortal111(::core::primitive::u8),
                        #[codec(index = 112)]
                        Mortal112(::core::primitive::u8),
                        #[codec(index = 113)]
                        Mortal113(::core::primitive::u8),
                        #[codec(index = 114)]
                        Mortal114(::core::primitive::u8),
                        #[codec(index = 115)]
                        Mortal115(::core::primitive::u8),
                        #[codec(index = 116)]
                        Mortal116(::core::primitive::u8),
                        #[codec(index = 117)]
                        Mortal117(::core::primitive::u8),
                        #[codec(index = 118)]
                        Mortal118(::core::primitive::u8),
                        #[codec(index = 119)]
                        Mortal119(::core::primitive::u8),
                        #[codec(index = 120)]
                        Mortal120(::core::primitive::u8),
                        #[codec(index = 121)]
                        Mortal121(::core::primitive::u8),
                        #[codec(index = 122)]
                        Mortal122(::core::primitive::u8),
                        #[codec(index = 123)]
                        Mortal123(::core::primitive::u8),
                        #[codec(index = 124)]
                        Mortal124(::core::primitive::u8),
                        #[codec(index = 125)]
                        Mortal125(::core::primitive::u8),
                        #[codec(index = 126)]
                        Mortal126(::core::primitive::u8),
                        #[codec(index = 127)]
                        Mortal127(::core::primitive::u8),
                        #[codec(index = 128)]
                        Mortal128(::core::primitive::u8),
                        #[codec(index = 129)]
                        Mortal129(::core::primitive::u8),
                        #[codec(index = 130)]
                        Mortal130(::core::primitive::u8),
                        #[codec(index = 131)]
                        Mortal131(::core::primitive::u8),
                        #[codec(index = 132)]
                        Mortal132(::core::primitive::u8),
                        #[codec(index = 133)]
                        Mortal133(::core::primitive::u8),
                        #[codec(index = 134)]
                        Mortal134(::core::primitive::u8),
                        #[codec(index = 135)]
                        Mortal135(::core::primitive::u8),
                        #[codec(index = 136)]
                        Mortal136(::core::primitive::u8),
                        #[codec(index = 137)]
                        Mortal137(::core::primitive::u8),
                        #[codec(index = 138)]
                        Mortal138(::core::primitive::u8),
                        #[codec(index = 139)]
                        Mortal139(::core::primitive::u8),
                        #[codec(index = 140)]
                        Mortal140(::core::primitive::u8),
                        #[codec(index = 141)]
                        Mortal141(::core::primitive::u8),
                        #[codec(index = 142)]
                        Mortal142(::core::primitive::u8),
                        #[codec(index = 143)]
                        Mortal143(::core::primitive::u8),
                        #[codec(index = 144)]
                        Mortal144(::core::primitive::u8),
                        #[codec(index = 145)]
                        Mortal145(::core::primitive::u8),
                        #[codec(index = 146)]
                        Mortal146(::core::primitive::u8),
                        #[codec(index = 147)]
                        Mortal147(::core::primitive::u8),
                        #[codec(index = 148)]
                        Mortal148(::core::primitive::u8),
                        #[codec(index = 149)]
                        Mortal149(::core::primitive::u8),
                        #[codec(index = 150)]
                        Mortal150(::core::primitive::u8),
                        #[codec(index = 151)]
                        Mortal151(::core::primitive::u8),
                        #[codec(index = 152)]
                        Mortal152(::core::primitive::u8),
                        #[codec(index = 153)]
                        Mortal153(::core::primitive::u8),
                        #[codec(index = 154)]
                        Mortal154(::core::primitive::u8),
                        #[codec(index = 155)]
                        Mortal155(::core::primitive::u8),
                        #[codec(index = 156)]
                        Mortal156(::core::primitive::u8),
                        #[codec(index = 157)]
                        Mortal157(::core::primitive::u8),
                        #[codec(index = 158)]
                        Mortal158(::core::primitive::u8),
                        #[codec(index = 159)]
                        Mortal159(::core::primitive::u8),
                        #[codec(index = 160)]
                        Mortal160(::core::primitive::u8),
                        #[codec(index = 161)]
                        Mortal161(::core::primitive::u8),
                        #[codec(index = 162)]
                        Mortal162(::core::primitive::u8),
                        #[codec(index = 163)]
                        Mortal163(::core::primitive::u8),
                        #[codec(index = 164)]
                        Mortal164(::core::primitive::u8),
                        #[codec(index = 165)]
                        Mortal165(::core::primitive::u8),
                        #[codec(index = 166)]
                        Mortal166(::core::primitive::u8),
                        #[codec(index = 167)]
                        Mortal167(::core::primitive::u8),
                        #[codec(index = 168)]
                        Mortal168(::core::primitive::u8),
                        #[codec(index = 169)]
                        Mortal169(::core::primitive::u8),
                        #[codec(index = 170)]
                        Mortal170(::core::primitive::u8),
                        #[codec(index = 171)]
                        Mortal171(::core::primitive::u8),
                        #[codec(index = 172)]
                        Mortal172(::core::primitive::u8),
                        #[codec(index = 173)]
                        Mortal173(::core::primitive::u8),
                        #[codec(index = 174)]
                        Mortal174(::core::primitive::u8),
                        #[codec(index = 175)]
                        Mortal175(::core::primitive::u8),
                        #[codec(index = 176)]
                        Mortal176(::core::primitive::u8),
                        #[codec(index = 177)]
                        Mortal177(::core::primitive::u8),
                        #[codec(index = 178)]
                        Mortal178(::core::primitive::u8),
                        #[codec(index = 179)]
                        Mortal179(::core::primitive::u8),
                        #[codec(index = 180)]
                        Mortal180(::core::primitive::u8),
                        #[codec(index = 181)]
                        Mortal181(::core::primitive::u8),
                        #[codec(index = 182)]
                        Mortal182(::core::primitive::u8),
                        #[codec(index = 183)]
                        Mortal183(::core::primitive::u8),
                        #[codec(index = 184)]
                        Mortal184(::core::primitive::u8),
                        #[codec(index = 185)]
                        Mortal185(::core::primitive::u8),
                        #[codec(index = 186)]
                        Mortal186(::core::primitive::u8),
                        #[codec(index = 187)]
                        Mortal187(::core::primitive::u8),
                        #[codec(index = 188)]
                        Mortal188(::core::primitive::u8),
                        #[codec(index = 189)]
                        Mortal189(::core::primitive::u8),
                        #[codec(index = 190)]
                        Mortal190(::core::primitive::u8),
                        #[codec(index = 191)]
                        Mortal191(::core::primitive::u8),
                        #[codec(index = 192)]
                        Mortal192(::core::primitive::u8),
                        #[codec(index = 193)]
                        Mortal193(::core::primitive::u8),
                        #[codec(index = 194)]
                        Mortal194(::core::primitive::u8),
                        #[codec(index = 195)]
                        Mortal195(::core::primitive::u8),
                        #[codec(index = 196)]
                        Mortal196(::core::primitive::u8),
                        #[codec(index = 197)]
                        Mortal197(::core::primitive::u8),
                        #[codec(index = 198)]
                        Mortal198(::core::primitive::u8),
                        #[codec(index = 199)]
                        Mortal199(::core::primitive::u8),
                        #[codec(index = 200)]
                        Mortal200(::core::primitive::u8),
                        #[codec(index = 201)]
                        Mortal201(::core::primitive::u8),
                        #[codec(index = 202)]
                        Mortal202(::core::primitive::u8),
                        #[codec(index = 203)]
                        Mortal203(::core::primitive::u8),
                        #[codec(index = 204)]
                        Mortal204(::core::primitive::u8),
                        #[codec(index = 205)]
                        Mortal205(::core::primitive::u8),
                        #[codec(index = 206)]
                        Mortal206(::core::primitive::u8),
                        #[codec(index = 207)]
                        Mortal207(::core::primitive::u8),
                        #[codec(index = 208)]
                        Mortal208(::core::primitive::u8),
                        #[codec(index = 209)]
                        Mortal209(::core::primitive::u8),
                        #[codec(index = 210)]
                        Mortal210(::core::primitive::u8),
                        #[codec(index = 211)]
                        Mortal211(::core::primitive::u8),
                        #[codec(index = 212)]
                        Mortal212(::core::primitive::u8),
                        #[codec(index = 213)]
                        Mortal213(::core::primitive::u8),
                        #[codec(index = 214)]
                        Mortal214(::core::primitive::u8),
                        #[codec(index = 215)]
                        Mortal215(::core::primitive::u8),
                        #[codec(index = 216)]
                        Mortal216(::core::primitive::u8),
                        #[codec(index = 217)]
                        Mortal217(::core::primitive::u8),
                        #[codec(index = 218)]
                        Mortal218(::core::primitive::u8),
                        #[codec(index = 219)]
                        Mortal219(::core::primitive::u8),
                        #[codec(index = 220)]
                        Mortal220(::core::primitive::u8),
                        #[codec(index = 221)]
                        Mortal221(::core::primitive::u8),
                        #[codec(index = 222)]
                        Mortal222(::core::primitive::u8),
                        #[codec(index = 223)]
                        Mortal223(::core::primitive::u8),
                        #[codec(index = 224)]
                        Mortal224(::core::primitive::u8),
                        #[codec(index = 225)]
                        Mortal225(::core::primitive::u8),
                        #[codec(index = 226)]
                        Mortal226(::core::primitive::u8),
                        #[codec(index = 227)]
                        Mortal227(::core::primitive::u8),
                        #[codec(index = 228)]
                        Mortal228(::core::primitive::u8),
                        #[codec(index = 229)]
                        Mortal229(::core::primitive::u8),
                        #[codec(index = 230)]
                        Mortal230(::core::primitive::u8),
                        #[codec(index = 231)]
                        Mortal231(::core::primitive::u8),
                        #[codec(index = 232)]
                        Mortal232(::core::primitive::u8),
                        #[codec(index = 233)]
                        Mortal233(::core::primitive::u8),
                        #[codec(index = 234)]
                        Mortal234(::core::primitive::u8),
                        #[codec(index = 235)]
                        Mortal235(::core::primitive::u8),
                        #[codec(index = 236)]
                        Mortal236(::core::primitive::u8),
                        #[codec(index = 237)]
                        Mortal237(::core::primitive::u8),
                        #[codec(index = 238)]
                        Mortal238(::core::primitive::u8),
                        #[codec(index = 239)]
                        Mortal239(::core::primitive::u8),
                        #[codec(index = 240)]
                        Mortal240(::core::primitive::u8),
                        #[codec(index = 241)]
                        Mortal241(::core::primitive::u8),
                        #[codec(index = 242)]
                        Mortal242(::core::primitive::u8),
                        #[codec(index = 243)]
                        Mortal243(::core::primitive::u8),
                        #[codec(index = 244)]
                        Mortal244(::core::primitive::u8),
                        #[codec(index = 245)]
                        Mortal245(::core::primitive::u8),
                        #[codec(index = 246)]
                        Mortal246(::core::primitive::u8),
                        #[codec(index = 247)]
                        Mortal247(::core::primitive::u8),
                        #[codec(index = 248)]
                        Mortal248(::core::primitive::u8),
                        #[codec(index = 249)]
                        Mortal249(::core::primitive::u8),
                        #[codec(index = 250)]
                        Mortal250(::core::primitive::u8),
                        #[codec(index = 251)]
                        Mortal251(::core::primitive::u8),
                        #[codec(index = 252)]
                        Mortal252(::core::primitive::u8),
                        #[codec(index = 253)]
                        Mortal253(::core::primitive::u8),
                        #[codec(index = 254)]
                        Mortal254(::core::primitive::u8),
                        #[codec(index = 255)]
                        Mortal255(::core::primitive::u8),
                    }
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum DispatchError {
                #[codec(index = 0)]
                Other,
                #[codec(index = 1)]
                CannotLookup,
                #[codec(index = 2)]
                BadOrigin,
                #[codec(index = 3)]
                Module(runtime_types::sp_runtime::ModuleError),
                #[codec(index = 4)]
                ConsumerRemaining,
                #[codec(index = 5)]
                NoProviders,
                #[codec(index = 6)]
                TooManyConsumers,
                #[codec(index = 7)]
                Token(runtime_types::sp_runtime::TokenError),
                #[codec(index = 8)]
                Arithmetic(runtime_types::sp_arithmetic::ArithmeticError),
                #[codec(index = 9)]
                Transactional(runtime_types::sp_runtime::TransactionalError),
                #[codec(index = 10)]
                Exhausted,
                #[codec(index = 11)]
                Corruption,
                #[codec(index = 12)]
                Unavailable,
                #[codec(index = 13)]
                RootNotAllowed,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ModuleError {
                pub index: ::core::primitive::u8,
                pub error: [::core::primitive::u8; 4usize],
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum MultiSignature {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Signature),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Signature),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Signature),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum TokenError {
                #[codec(index = 0)]
                FundsUnavailable,
                #[codec(index = 1)]
                OnlyProvider,
                #[codec(index = 2)]
                BelowMinimum,
                #[codec(index = 3)]
                CannotCreate,
                #[codec(index = 4)]
                UnknownAsset,
                #[codec(index = 5)]
                Frozen,
                #[codec(index = 6)]
                Unsupported,
                #[codec(index = 7)]
                CannotCreateHold,
                #[codec(index = 8)]
                NotExpendable,
                #[codec(index = 9)]
                Blocked,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum TransactionalError {
                #[codec(index = 0)]
                LimitReached,
                #[codec(index = 1)]
                NoLayer,
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct RuntimeVersion {
                pub spec_name: ::std::string::String,
                pub impl_name: ::std::string::String,
                pub authoring_version: ::core::primitive::u32,
                pub spec_version: ::core::primitive::u32,
                pub impl_version: ::core::primitive::u32,
                pub apis:
                    ::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
                pub transaction_version: ::core::primitive::u32,
                pub state_version: ::core::primitive::u8,
            }
        }
        pub mod sp_weights {
            use super::runtime_types;
            pub mod weight_v2 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Weight {
                    #[codec(compact)]
                    pub ref_time: ::core::primitive::u64,
                    #[codec(compact)]
                    pub proof_size: ::core::primitive::u64,
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct RuntimeDbWeight {
                pub read: ::core::primitive::u64,
                pub write: ::core::primitive::u64,
            }
        }
    }
}
