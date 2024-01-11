pub use plo_hub_nft::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod plo_hub_nft {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getApproved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getApproved"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mintNFT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mintNFT"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenURI"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("serverAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ownerOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ownerOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("approved"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setToken"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokenURI"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenURI"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Approval"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("approved"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("approved"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BatchMetadataUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BatchMetadataUpdate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_fromTokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_toTokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MetadataUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MetadataUpdate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ERC721IncorrectOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC721IncorrectOwner",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC721InsufficientApproval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC721InsufficientApproval",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC721InvalidApprover"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC721InvalidApprover",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("approver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC721InvalidOperator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC721InvalidOperator",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC721InvalidOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ERC721InvalidOwner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC721InvalidReceiver"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC721InvalidReceiver",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC721InvalidSender"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC721InvalidSender",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC721NonexistentToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC721NonexistentToken",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnableInvalidOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnableInvalidOwner",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnableUnauthorizedAccount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnableUnauthorizedAccount",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static PLOHUBNFT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P3`@Q\x80`@\x01`@R\x80`\n\x81R` \x01iPloHubNFTs`\xB0\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x14\x12\x13\x91\x95`\xDA\x1B\x81RP\x81`\0\x90\x81b\0\0e\x91\x90b\0\x01\xB6V[P`\x01b\0\0t\x82\x82b\0\x01\xB6V[PPP`\x01`\x01`\xA0\x1B\x03\x81\x16b\0\0\xA6W`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01`@Q\x80\x91\x03\x90\xFD[b\0\0\xB1\x81b\0\0\xBDV[P`\x14`\nUb\0\x02\x82V[`\x07\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x01:W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x01[WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x01\xB1W`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x01\x8CWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x01\xADW\x82\x81U`\x01\x01b\0\x01\x98V[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x01\xD2Wb\0\x01\xD2b\0\x01\x0FV[b\0\x01\xEA\x81b\0\x01\xE3\x84Tb\0\x01%V[\x84b\0\x01aV[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x02\"W`\0\x84\x15b\0\x02\tWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x01\xADV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x02SW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x022V[P\x85\x82\x10\x15b\0\x02rW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[a\x15\x93\x80b\0\x02\x92`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x16W`\x005`\xE0\x1C\x80cqP\x18\xA6\x11a\0\xA2W\x80c\xB8\x8DO\xDE\x11a\0qW\x80c\xB8\x8DO\xDE\x14a\x029W\x80c\xC8{V\xDD\x14a\x02LW\x80c\xD0\x1Fz\x87\x14a\x02_W\x80c\xE9\x85\xE9\xC5\x14a\x02rW\x80c\xF2\xFD\xE3\x8B\x14a\x02\x85W`\0\x80\xFD[\x80cqP\x18\xA6\x14a\x02\x05W\x80c\x8D\xA5\xCB[\x14a\x02\rW\x80c\x95\xD8\x9BA\x14a\x02\x1EW\x80c\xA2,\xB4e\x14a\x02&W`\0\x80\xFD[\x80c\x14O\xA6\xD7\x11a\0\xE9W\x80c\x14O\xA6\xD7\x14a\x01\x98W\x80c#\xB8r\xDD\x14a\x01\xABW\x80cB\x84.\x0E\x14a\x01\xBEW\x80ccR!\x1E\x14a\x01\xD1W\x80cp\xA0\x821\x14a\x01\xE4W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\x1BW\x80c\x06\xFD\xDE\x03\x14a\x01CW\x80c\x08\x18\x12\xFC\x14a\x01XW\x80c\t^\xA7\xB3\x14a\x01\x83W[`\0\x80\xFD[a\x01.a\x01)6`\x04a\x10\x10V[a\x02\x98V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01Ka\x02\xC3V[`@Qa\x01:\x91\x90a\x10}V[a\x01ka\x01f6`\x04a\x10\x90V[a\x03UV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01:V[a\x01\x96a\x01\x916`\x04a\x10\xC0V[a\x03~V[\0[a\x01.a\x01\xA66`\x04a\x10\xEAV[a\x03\x8DV[a\x01\x96a\x01\xB96`\x04a\x11\x05V[a\x03\xCEV[a\x01\x96a\x01\xCC6`\x04a\x11\x05V[a\x04^V[a\x01ka\x01\xDF6`\x04a\x10\x90V[a\x04~V[a\x01\xF7a\x01\xF26`\x04a\x10\xEAV[a\x04\x89V[`@Q\x90\x81R` \x01a\x01:V[a\x01\x96a\x04\xD1V[`\x07T`\x01`\x01`\xA0\x1B\x03\x16a\x01kV[a\x01Ka\x04\xE5V[a\x01\x96a\x0246`\x04a\x11OV[a\x04\xF4V[a\x01\x96a\x02G6`\x04a\x12\x12V[a\x04\xFFV[a\x01Ka\x02Z6`\x04a\x10\x90V[a\x05\x16V[a\x01\xF7a\x02m6`\x04a\x12\x8EV[a\x06'V[a\x01.a\x02\x806`\x04a\x13\0V[a\x07VV[a\x01\x96a\x02\x936`\x04a\x10\xEAV[a\x07\x84V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c$\x83$\x83`\xE1\x1B\x14\x80a\x02\xBDWPa\x02\xBD\x82a\x07\xC2V[\x92\x91PPV[```\0\x80Ta\x02\xD2\x90a\x133V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xFE\x90a\x133V[\x80\x15a\x03KW\x80`\x1F\x10a\x03 Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03KV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03.W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0a\x03`\x82a\x08\x12V[P`\0\x82\x81R`\x04` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x02\xBDV[a\x03\x89\x82\x823a\x08KV[PPV[`\0a\x03\x97a\x08XV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x03\xAAW`\0\x80\xFD[P`\t\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90U`\x01[\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x03\xFDW`@Qc2PWI`\xE1\x1B\x81R`\0`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\0a\x04\n\x83\x833a\x08\x85V[\x90P\x83`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04XW`@Qcd(={`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x82\x16`D\x82\x01R`d\x01a\x03\xF4V[PPPPV[a\x04y\x83\x83\x83`@Q\x80` \x01`@R\x80`\0\x81RPa\x04\xFFV[PPPV[`\0a\x02\xBD\x82a\x08\x12V[`\0`\x01`\x01`\xA0\x1B\x03\x82\x16a\x04\xB5W`@Qc\"q\x8A\xD9`\xE2\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x03\xF4V[P`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[a\x04\xD9a\x08XV[a\x04\xE3`\0a\t~V[V[```\x01\x80Ta\x02\xD2\x90a\x133V[a\x03\x893\x83\x83a\t\xD0V[a\x05\n\x84\x84\x84a\x03\xCEV[a\x04X\x84\x84\x84\x84a\noV[``a\x05!\x82a\x08\x12V[P`\0\x82\x81R`\x06` R`@\x81 \x80Ta\x05;\x90a\x133V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05g\x90a\x133V[\x80\x15a\x05\xB4W\x80`\x1F\x10a\x05\x89Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xB4V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\x97W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P`\0a\x05\xD2`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[\x90P\x80Q`\0\x03a\x05\xE4WP\x92\x91PPV[\x81Q\x15a\x06\x16W\x80\x82`@Q` \x01a\x05\xFE\x92\x91\x90a\x13mV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPP\x91\x90PV[a\x06\x1F\x84a\x0B\x98V[\x94\x93PPPPV[`\nT`\tT`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R`\0\x93\x92\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06vW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x9A\x91\x90a\x13\x9CV[\x11a\x06\xA4W`\0\x80\xFD[`\tT`\nT`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x85\x81\x16`$\x83\x01R`D\x82\x01\x92\x90\x92R\x91\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07&\x91\x90a\x13\xB5V[P`\x01`\x08`\0\x82\x82Ta\x07:\x91\x90a\x13\xD2V[\x90\x91UPP`\x08Ta\x07L\x85\x82a\x0C\rV[a\x06\x1F\x81\x85a\x0CrV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T`\xFF\x16\x90V[a\x07\x8Ca\x08XV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x07\xB6W`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x03\xF4V[a\x07\xBF\x81a\t~V[PV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x80\xACX\xCD`\xE0\x1B\x14\x80a\x07\xF3WP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c[^\x13\x9F`\xE0\x1B\x14[\x80a\x02\xBDWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x02\xBDV[`\0\x81\x81R`\x02` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x80a\x02\xBDW`@Qc~'2\x89`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x03\xF4V[a\x04y\x83\x83\x83`\x01a\x0C\xC2V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\xE3W`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x03\xF4V[`\0\x82\x81R`\x02` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x83\x16\x15a\x08\xB2Wa\x08\xB2\x81\x84\x86a\r\xC8V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x08\xF0Wa\x08\xCF`\0\x85`\0\x80a\x0C\xC2V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x03` R`@\x90 \x80T`\0\x19\x01\x90U[`\x01`\x01`\xA0\x1B\x03\x85\x16\x15a\t\x1FW`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x03` R`@\x90 \x80T`\x01\x01\x90U[`\0\x84\x81R`\x02` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x89\x81\x16\x91\x82\x17\x90\x92U\x91Q\x87\x93\x91\x85\x16\x91\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\xA4\x94\x93PPPPV[`\x07\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\n\x02W`@Qc\x0Ba\x17C`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x03\xF4V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x80T`\xFF\x19\x16\x86\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15a\x04XW`@Qc\n\x85\xBD\x01`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\x15\x0Bz\x02\x90a\n\xB1\x903\x90\x88\x90\x87\x90\x87\x90`\x04\x01a\x13\xF3V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\n\xECWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\n\xE9\x91\x81\x01\x90a\x140V[`\x01[a\x0BUW=\x80\x80\x15a\x0B\x1AW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0B\x1FV[``\x91P[P\x80Q`\0\x03a\x0BMW`@Qc2PWI`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x03\xF4V[\x80Q\x81` \x01\xFD[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16c\n\x85\xBD\x01`\xE1\x1B\x14a\x0B\x91W`@Qc2PWI`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x03\xF4V[PPPPPV[``a\x0B\xA3\x82a\x08\x12V[P`\0a\x0B\xBB`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[\x90P`\0\x81Q\x11a\x0B\xDBW`@Q\x80` \x01`@R\x80`\0\x81RPa\x0C\x06V[\x80a\x0B\xE5\x84a\x0E,V[`@Q` \x01a\x0B\xF6\x92\x91\x90a\x13mV[`@Q` \x81\x83\x03\x03\x81R\x90`@R[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0C7W`@Qc2PWI`\xE1\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x03\xF4V[`\0a\x0CE\x83\x83`\0a\x08\x85V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x04yW`@Qc9\xE3V7`\xE1\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x03\xF4V[`\0\x82\x81R`\x06` R`@\x90 a\x0C\x8A\x82\x82a\x14\x9DV[P`@Q\x82\x81R\x7F\xF8\xE1\xA1Z\xBA\x93\x98\xE0\x19\xF0\xB4\x9D\xF1\xA4\xFD\xE9\x8E\xE1z\xE3E\xCB_k^,'\xF5\x03>\x8C\xE7\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[\x80\x80a\x0C\xD6WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[\x15a\r\x98W`\0a\x0C\xE6\x84a\x08\x12V[\x90P`\x01`\x01`\xA0\x1B\x03\x83\x16\x15\x80\x15\x90a\r\x12WP\x82`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x80\x15a\r%WPa\r#\x81\x84a\x07VV[\x15[\x15a\rNW`@Qc\xA9\xFB\xF5\x1F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x03\xF4V[\x81\x15a\r\x96W\x83\x85`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%`@Q`@Q\x80\x91\x03\x90\xA4[P[PP`\0\x90\x81R`\x04` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\r\xD3\x83\x83\x83a\x0E\xBFV[a\x04yW`\x01`\x01`\xA0\x1B\x03\x83\x16a\x0E\x01W`@Qc~'2\x89`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x03\xF4V[`@Qc\x17~\x80/`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x01a\x03\xF4V[```\0a\x0E9\x83a\x0F\"V[`\x01\x01\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0EYWa\x0EYa\x11\x86V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0E\x83W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[`\0\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x0E\x8DWP\x93\x92PPPV[`\0`\x01`\x01`\xA0\x1B\x03\x83\x16\x15\x80\x15\x90a\x06\x1FWP\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a\x0E\xF9WPa\x0E\xF9\x84\x84a\x07VV[\x80a\x06\x1FWPP`\0\x90\x81R`\x04` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14\x91\x90PV[`\0\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a\x0FaWr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x0F\x8DWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x0F\xABWf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x0F\xC3Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x0F\xD7Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x0F\xE9W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x02\xBDW`\x01\x01\x92\x91PPV[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x07\xBFW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x10\"W`\0\x80\xFD[\x815a\x0C\x06\x81a\x0F\xFAV[`\0[\x83\x81\x10\x15a\x10HW\x81\x81\x01Q\x83\x82\x01R` \x01a\x100V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x10i\x81` \x86\x01` \x86\x01a\x10-V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x0C\x06` \x83\x01\x84a\x10QV[`\0` \x82\x84\x03\x12\x15a\x10\xA2W`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\xC9W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x10\xD3W`\0\x80\xFD[a\x10\xDC\x83a\x10\xA9V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\x10\xFCW`\0\x80\xFD[a\x0C\x06\x82a\x10\xA9V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x11\x1AW`\0\x80\xFD[a\x11#\x84a\x10\xA9V[\x92Pa\x111` \x85\x01a\x10\xA9V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[\x80\x15\x15\x81\x14a\x07\xBFW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x11bW`\0\x80\xFD[a\x11k\x83a\x10\xA9V[\x91P` \x83\x015a\x11{\x81a\x11AV[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x11\x15a\x11\xB7Wa\x11\xB7a\x11\x86V[`@Q`\x1F\x85\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x11\xDFWa\x11\xDFa\x11\x86V[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a\x11\xF8W`\0\x80\xFD[\x85\x85` \x83\x017`\0` \x87\x83\x01\x01RPPP\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x12(W`\0\x80\xFD[a\x121\x85a\x10\xA9V[\x93Pa\x12?` \x86\x01a\x10\xA9V[\x92P`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12bW`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a\x12sW`\0\x80\xFD[a\x12\x82\x87\x825` \x84\x01a\x11\x9CV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x12\xA3W`\0\x80\xFD[a\x12\xAC\x84a\x10\xA9V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\xC8W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x12\xD9W`\0\x80\xFD[a\x12\xE8\x86\x825` \x84\x01a\x11\x9CV[\x92PPa\x12\xF7`@\x85\x01a\x10\xA9V[\x90P\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a\x13\x13W`\0\x80\xFD[a\x13\x1C\x83a\x10\xA9V[\x91Pa\x13*` \x84\x01a\x10\xA9V[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x13GW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x13gWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x83Qa\x13\x7F\x81\x84` \x88\x01a\x10-V[\x83Q\x90\x83\x01\x90a\x13\x93\x81\x83` \x88\x01a\x10-V[\x01\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x13\xAEW`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x13\xC7W`\0\x80\xFD[\x81Qa\x0C\x06\x81a\x11AV[\x80\x82\x01\x80\x82\x11\x15a\x02\xBDWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R\x84\x16` \x82\x01R`@\x81\x01\x83\x90R`\x80``\x82\x01\x81\x90R`\0\x90a\x14&\x90\x83\x01\x84a\x10QV[\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x14BW`\0\x80\xFD[\x81Qa\x0C\x06\x81a\x0F\xFAV[`\x1F\x82\x11\x15a\x04yW`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x14vWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x14\x95W\x82\x81U`\x01\x01a\x14\x82V[PPPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xB7Wa\x14\xB7a\x11\x86V[a\x14\xCB\x81a\x14\xC5\x84Ta\x133V[\x84a\x14MV[` \x80`\x1F\x83\x11`\x01\x81\x14a\x15\0W`\0\x84\x15a\x14\xE8WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x14\x95V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x15/W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x15\x10V[P\x85\x82\x10\x15a\x15MW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV\xFE\xA2dipfsX\"\x12 \xF5\xB1\x19Qv\xEC\x05\tZ65s&_\xC6\xFE=2\xC07]\x07T%\xCF\x89\x04\x01{r'\tdsolcC\0\x08\x17\x003";
    /// The bytecode of the contract.
    pub static PLOHUBNFT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x16W`\x005`\xE0\x1C\x80cqP\x18\xA6\x11a\0\xA2W\x80c\xB8\x8DO\xDE\x11a\0qW\x80c\xB8\x8DO\xDE\x14a\x029W\x80c\xC8{V\xDD\x14a\x02LW\x80c\xD0\x1Fz\x87\x14a\x02_W\x80c\xE9\x85\xE9\xC5\x14a\x02rW\x80c\xF2\xFD\xE3\x8B\x14a\x02\x85W`\0\x80\xFD[\x80cqP\x18\xA6\x14a\x02\x05W\x80c\x8D\xA5\xCB[\x14a\x02\rW\x80c\x95\xD8\x9BA\x14a\x02\x1EW\x80c\xA2,\xB4e\x14a\x02&W`\0\x80\xFD[\x80c\x14O\xA6\xD7\x11a\0\xE9W\x80c\x14O\xA6\xD7\x14a\x01\x98W\x80c#\xB8r\xDD\x14a\x01\xABW\x80cB\x84.\x0E\x14a\x01\xBEW\x80ccR!\x1E\x14a\x01\xD1W\x80cp\xA0\x821\x14a\x01\xE4W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\x1BW\x80c\x06\xFD\xDE\x03\x14a\x01CW\x80c\x08\x18\x12\xFC\x14a\x01XW\x80c\t^\xA7\xB3\x14a\x01\x83W[`\0\x80\xFD[a\x01.a\x01)6`\x04a\x10\x10V[a\x02\x98V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01Ka\x02\xC3V[`@Qa\x01:\x91\x90a\x10}V[a\x01ka\x01f6`\x04a\x10\x90V[a\x03UV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01:V[a\x01\x96a\x01\x916`\x04a\x10\xC0V[a\x03~V[\0[a\x01.a\x01\xA66`\x04a\x10\xEAV[a\x03\x8DV[a\x01\x96a\x01\xB96`\x04a\x11\x05V[a\x03\xCEV[a\x01\x96a\x01\xCC6`\x04a\x11\x05V[a\x04^V[a\x01ka\x01\xDF6`\x04a\x10\x90V[a\x04~V[a\x01\xF7a\x01\xF26`\x04a\x10\xEAV[a\x04\x89V[`@Q\x90\x81R` \x01a\x01:V[a\x01\x96a\x04\xD1V[`\x07T`\x01`\x01`\xA0\x1B\x03\x16a\x01kV[a\x01Ka\x04\xE5V[a\x01\x96a\x0246`\x04a\x11OV[a\x04\xF4V[a\x01\x96a\x02G6`\x04a\x12\x12V[a\x04\xFFV[a\x01Ka\x02Z6`\x04a\x10\x90V[a\x05\x16V[a\x01\xF7a\x02m6`\x04a\x12\x8EV[a\x06'V[a\x01.a\x02\x806`\x04a\x13\0V[a\x07VV[a\x01\x96a\x02\x936`\x04a\x10\xEAV[a\x07\x84V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c$\x83$\x83`\xE1\x1B\x14\x80a\x02\xBDWPa\x02\xBD\x82a\x07\xC2V[\x92\x91PPV[```\0\x80Ta\x02\xD2\x90a\x133V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xFE\x90a\x133V[\x80\x15a\x03KW\x80`\x1F\x10a\x03 Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03KV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03.W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0a\x03`\x82a\x08\x12V[P`\0\x82\x81R`\x04` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x02\xBDV[a\x03\x89\x82\x823a\x08KV[PPV[`\0a\x03\x97a\x08XV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x03\xAAW`\0\x80\xFD[P`\t\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90U`\x01[\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x03\xFDW`@Qc2PWI`\xE1\x1B\x81R`\0`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\0a\x04\n\x83\x833a\x08\x85V[\x90P\x83`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04XW`@Qcd(={`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x82\x16`D\x82\x01R`d\x01a\x03\xF4V[PPPPV[a\x04y\x83\x83\x83`@Q\x80` \x01`@R\x80`\0\x81RPa\x04\xFFV[PPPV[`\0a\x02\xBD\x82a\x08\x12V[`\0`\x01`\x01`\xA0\x1B\x03\x82\x16a\x04\xB5W`@Qc\"q\x8A\xD9`\xE2\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x03\xF4V[P`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[a\x04\xD9a\x08XV[a\x04\xE3`\0a\t~V[V[```\x01\x80Ta\x02\xD2\x90a\x133V[a\x03\x893\x83\x83a\t\xD0V[a\x05\n\x84\x84\x84a\x03\xCEV[a\x04X\x84\x84\x84\x84a\noV[``a\x05!\x82a\x08\x12V[P`\0\x82\x81R`\x06` R`@\x81 \x80Ta\x05;\x90a\x133V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05g\x90a\x133V[\x80\x15a\x05\xB4W\x80`\x1F\x10a\x05\x89Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xB4V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\x97W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P`\0a\x05\xD2`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[\x90P\x80Q`\0\x03a\x05\xE4WP\x92\x91PPV[\x81Q\x15a\x06\x16W\x80\x82`@Q` \x01a\x05\xFE\x92\x91\x90a\x13mV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPP\x91\x90PV[a\x06\x1F\x84a\x0B\x98V[\x94\x93PPPPV[`\nT`\tT`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R`\0\x93\x92\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06vW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x9A\x91\x90a\x13\x9CV[\x11a\x06\xA4W`\0\x80\xFD[`\tT`\nT`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x85\x81\x16`$\x83\x01R`D\x82\x01\x92\x90\x92R\x91\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07&\x91\x90a\x13\xB5V[P`\x01`\x08`\0\x82\x82Ta\x07:\x91\x90a\x13\xD2V[\x90\x91UPP`\x08Ta\x07L\x85\x82a\x0C\rV[a\x06\x1F\x81\x85a\x0CrV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T`\xFF\x16\x90V[a\x07\x8Ca\x08XV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x07\xB6W`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x03\xF4V[a\x07\xBF\x81a\t~V[PV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x80\xACX\xCD`\xE0\x1B\x14\x80a\x07\xF3WP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c[^\x13\x9F`\xE0\x1B\x14[\x80a\x02\xBDWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x02\xBDV[`\0\x81\x81R`\x02` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x80a\x02\xBDW`@Qc~'2\x89`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x03\xF4V[a\x04y\x83\x83\x83`\x01a\x0C\xC2V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\xE3W`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x03\xF4V[`\0\x82\x81R`\x02` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x83\x16\x15a\x08\xB2Wa\x08\xB2\x81\x84\x86a\r\xC8V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x08\xF0Wa\x08\xCF`\0\x85`\0\x80a\x0C\xC2V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x03` R`@\x90 \x80T`\0\x19\x01\x90U[`\x01`\x01`\xA0\x1B\x03\x85\x16\x15a\t\x1FW`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x03` R`@\x90 \x80T`\x01\x01\x90U[`\0\x84\x81R`\x02` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x89\x81\x16\x91\x82\x17\x90\x92U\x91Q\x87\x93\x91\x85\x16\x91\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\xA4\x94\x93PPPPV[`\x07\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\n\x02W`@Qc\x0Ba\x17C`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x03\xF4V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x80T`\xFF\x19\x16\x86\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15a\x04XW`@Qc\n\x85\xBD\x01`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\x15\x0Bz\x02\x90a\n\xB1\x903\x90\x88\x90\x87\x90\x87\x90`\x04\x01a\x13\xF3V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\n\xECWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\n\xE9\x91\x81\x01\x90a\x140V[`\x01[a\x0BUW=\x80\x80\x15a\x0B\x1AW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0B\x1FV[``\x91P[P\x80Q`\0\x03a\x0BMW`@Qc2PWI`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x03\xF4V[\x80Q\x81` \x01\xFD[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16c\n\x85\xBD\x01`\xE1\x1B\x14a\x0B\x91W`@Qc2PWI`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x03\xF4V[PPPPPV[``a\x0B\xA3\x82a\x08\x12V[P`\0a\x0B\xBB`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[\x90P`\0\x81Q\x11a\x0B\xDBW`@Q\x80` \x01`@R\x80`\0\x81RPa\x0C\x06V[\x80a\x0B\xE5\x84a\x0E,V[`@Q` \x01a\x0B\xF6\x92\x91\x90a\x13mV[`@Q` \x81\x83\x03\x03\x81R\x90`@R[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0C7W`@Qc2PWI`\xE1\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x03\xF4V[`\0a\x0CE\x83\x83`\0a\x08\x85V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x04yW`@Qc9\xE3V7`\xE1\x1B\x81R`\0`\x04\x82\x01R`$\x01a\x03\xF4V[`\0\x82\x81R`\x06` R`@\x90 a\x0C\x8A\x82\x82a\x14\x9DV[P`@Q\x82\x81R\x7F\xF8\xE1\xA1Z\xBA\x93\x98\xE0\x19\xF0\xB4\x9D\xF1\xA4\xFD\xE9\x8E\xE1z\xE3E\xCB_k^,'\xF5\x03>\x8C\xE7\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[\x80\x80a\x0C\xD6WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[\x15a\r\x98W`\0a\x0C\xE6\x84a\x08\x12V[\x90P`\x01`\x01`\xA0\x1B\x03\x83\x16\x15\x80\x15\x90a\r\x12WP\x82`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x80\x15a\r%WPa\r#\x81\x84a\x07VV[\x15[\x15a\rNW`@Qc\xA9\xFB\xF5\x1F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x03\xF4V[\x81\x15a\r\x96W\x83\x85`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%`@Q`@Q\x80\x91\x03\x90\xA4[P[PP`\0\x90\x81R`\x04` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\r\xD3\x83\x83\x83a\x0E\xBFV[a\x04yW`\x01`\x01`\xA0\x1B\x03\x83\x16a\x0E\x01W`@Qc~'2\x89`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x03\xF4V[`@Qc\x17~\x80/`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x01a\x03\xF4V[```\0a\x0E9\x83a\x0F\"V[`\x01\x01\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0EYWa\x0EYa\x11\x86V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0E\x83W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[`\0\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x0E\x8DWP\x93\x92PPPV[`\0`\x01`\x01`\xA0\x1B\x03\x83\x16\x15\x80\x15\x90a\x06\x1FWP\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a\x0E\xF9WPa\x0E\xF9\x84\x84a\x07VV[\x80a\x06\x1FWPP`\0\x90\x81R`\x04` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14\x91\x90PV[`\0\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a\x0FaWr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x0F\x8DWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x0F\xABWf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x0F\xC3Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x0F\xD7Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x0F\xE9W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x02\xBDW`\x01\x01\x92\x91PPV[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x07\xBFW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x10\"W`\0\x80\xFD[\x815a\x0C\x06\x81a\x0F\xFAV[`\0[\x83\x81\x10\x15a\x10HW\x81\x81\x01Q\x83\x82\x01R` \x01a\x100V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x10i\x81` \x86\x01` \x86\x01a\x10-V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x0C\x06` \x83\x01\x84a\x10QV[`\0` \x82\x84\x03\x12\x15a\x10\xA2W`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\xC9W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x10\xD3W`\0\x80\xFD[a\x10\xDC\x83a\x10\xA9V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\x10\xFCW`\0\x80\xFD[a\x0C\x06\x82a\x10\xA9V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x11\x1AW`\0\x80\xFD[a\x11#\x84a\x10\xA9V[\x92Pa\x111` \x85\x01a\x10\xA9V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[\x80\x15\x15\x81\x14a\x07\xBFW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x11bW`\0\x80\xFD[a\x11k\x83a\x10\xA9V[\x91P` \x83\x015a\x11{\x81a\x11AV[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x11\x15a\x11\xB7Wa\x11\xB7a\x11\x86V[`@Q`\x1F\x85\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x11\xDFWa\x11\xDFa\x11\x86V[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a\x11\xF8W`\0\x80\xFD[\x85\x85` \x83\x017`\0` \x87\x83\x01\x01RPPP\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x12(W`\0\x80\xFD[a\x121\x85a\x10\xA9V[\x93Pa\x12?` \x86\x01a\x10\xA9V[\x92P`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12bW`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a\x12sW`\0\x80\xFD[a\x12\x82\x87\x825` \x84\x01a\x11\x9CV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x12\xA3W`\0\x80\xFD[a\x12\xAC\x84a\x10\xA9V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\xC8W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x12\xD9W`\0\x80\xFD[a\x12\xE8\x86\x825` \x84\x01a\x11\x9CV[\x92PPa\x12\xF7`@\x85\x01a\x10\xA9V[\x90P\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a\x13\x13W`\0\x80\xFD[a\x13\x1C\x83a\x10\xA9V[\x91Pa\x13*` \x84\x01a\x10\xA9V[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x13GW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x13gWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x83Qa\x13\x7F\x81\x84` \x88\x01a\x10-V[\x83Q\x90\x83\x01\x90a\x13\x93\x81\x83` \x88\x01a\x10-V[\x01\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x13\xAEW`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x13\xC7W`\0\x80\xFD[\x81Qa\x0C\x06\x81a\x11AV[\x80\x82\x01\x80\x82\x11\x15a\x02\xBDWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R\x84\x16` \x82\x01R`@\x81\x01\x83\x90R`\x80``\x82\x01\x81\x90R`\0\x90a\x14&\x90\x83\x01\x84a\x10QV[\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x14BW`\0\x80\xFD[\x81Qa\x0C\x06\x81a\x0F\xFAV[`\x1F\x82\x11\x15a\x04yW`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x14vWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x14\x95W\x82\x81U`\x01\x01a\x14\x82V[PPPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xB7Wa\x14\xB7a\x11\x86V[a\x14\xCB\x81a\x14\xC5\x84Ta\x133V[\x84a\x14MV[` \x80`\x1F\x83\x11`\x01\x81\x14a\x15\0W`\0\x84\x15a\x14\xE8WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x14\x95V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x15/W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x15\x10V[P\x85\x82\x10\x15a\x15MW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV\xFE\xA2dipfsX\"\x12 \xF5\xB1\x19Qv\xEC\x05\tZ65s&_\xC6\xFE=2\xC07]\x07T%\xCF\x89\x04\x01{r'\tdsolcC\0\x08\x17\x003";
    /// The deployed bytecode of the contract.
    pub static PLOHUBNFT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct PloHubNFT<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PloHubNFT<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PloHubNFT<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PloHubNFT<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PloHubNFT<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PloHubNFT)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PloHubNFT<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    PLOHUBNFT_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                PLOHUBNFT_ABI.clone(),
                PLOHUBNFT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 94, 167, 179], (to, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getApproved` (0x081812fc) function
        pub fn get_approved(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([8, 24, 18, 252], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isApprovedForAll` (0xe985e9c5) function
        pub fn is_approved_for_all(
            &self,
            owner: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 133, 233, 197], (owner, operator))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mintNFT` (0xd01f7a87) function
        pub fn mint_nft(
            &self,
            recipient: ::ethers::core::types::Address,
            token_uri: ::std::string::String,
            server_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([208, 31, 122, 135], (recipient, token_uri, server_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ownerOf` (0x6352211e) function
        pub fn owner_of(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([99, 82, 33, 30], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeTransferFrom` (0x42842e0e) function
        pub fn safe_transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 132, 46, 14], (from, to, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeTransferFrom` (0xb88d4fde) function
        pub fn safe_transfer_from_with_from_and_to_and_data(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 141, 79, 222], (from, to, token_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setApprovalForAll` (0xa22cb465) function
        pub fn set_approval_for_all(
            &self,
            operator: ::ethers::core::types::Address,
            approved: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 44, 180, 101], (operator, approved))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setToken` (0x144fa6d7) function
        pub fn set_token(
            &self,
            token_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([20, 79, 166, 215], token_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenURI` (0xc87b56dd) function
        pub fn token_uri(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([200, 123, 86, 221], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ApprovalForAll` event
        pub fn approval_for_all_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalForAllFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `BatchMetadataUpdate` event
        pub fn batch_metadata_update_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BatchMetadataUpdateFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MetadataUpdate` event
        pub fn metadata_update_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MetadataUpdateFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PloHubNFTEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for PloHubNFT<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ERC721IncorrectOwner` with signature `ERC721IncorrectOwner(address,uint256,address)` and selector `0x64283d7b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "ERC721IncorrectOwner",
        abi = "ERC721IncorrectOwner(address,uint256,address)"
    )]
    pub struct ERC721IncorrectOwner {
        pub sender: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub owner: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC721InsufficientApproval` with signature `ERC721InsufficientApproval(address,uint256)` and selector `0x177e802f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "ERC721InsufficientApproval",
        abi = "ERC721InsufficientApproval(address,uint256)"
    )]
    pub struct ERC721InsufficientApproval {
        pub operator: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Custom Error type `ERC721InvalidApprover` with signature `ERC721InvalidApprover(address)` and selector `0xa9fbf51f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ERC721InvalidApprover", abi = "ERC721InvalidApprover(address)")]
    pub struct ERC721InvalidApprover {
        pub approver: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC721InvalidOperator` with signature `ERC721InvalidOperator(address)` and selector `0x5b08ba18`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ERC721InvalidOperator", abi = "ERC721InvalidOperator(address)")]
    pub struct ERC721InvalidOperator {
        pub operator: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC721InvalidOwner` with signature `ERC721InvalidOwner(address)` and selector `0x89c62b64`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ERC721InvalidOwner", abi = "ERC721InvalidOwner(address)")]
    pub struct ERC721InvalidOwner {
        pub owner: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC721InvalidReceiver` with signature `ERC721InvalidReceiver(address)` and selector `0x64a0ae92`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ERC721InvalidReceiver", abi = "ERC721InvalidReceiver(address)")]
    pub struct ERC721InvalidReceiver {
        pub receiver: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC721InvalidSender` with signature `ERC721InvalidSender(address)` and selector `0x73c6ac6e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ERC721InvalidSender", abi = "ERC721InvalidSender(address)")]
    pub struct ERC721InvalidSender {
        pub sender: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC721NonexistentToken` with signature `ERC721NonexistentToken(uint256)` and selector `0x7e273289`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ERC721NonexistentToken", abi = "ERC721NonexistentToken(uint256)")]
    pub struct ERC721NonexistentToken {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Custom Error type `OwnableInvalidOwner` with signature `OwnableInvalidOwner(address)` and selector `0x1e4fbdf7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OwnableInvalidOwner", abi = "OwnableInvalidOwner(address)")]
    pub struct OwnableInvalidOwner {
        pub owner: ::ethers::core::types::Address,
    }
    ///Custom Error type `OwnableUnauthorizedAccount` with signature `OwnableUnauthorizedAccount(address)` and selector `0x118cdaa7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "OwnableUnauthorizedAccount",
        abi = "OwnableUnauthorizedAccount(address)"
    )]
    pub struct OwnableUnauthorizedAccount {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's custom errors
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum PloHubNFTErrors {
        ERC721IncorrectOwner(ERC721IncorrectOwner),
        ERC721InsufficientApproval(ERC721InsufficientApproval),
        ERC721InvalidApprover(ERC721InvalidApprover),
        ERC721InvalidOperator(ERC721InvalidOperator),
        ERC721InvalidOwner(ERC721InvalidOwner),
        ERC721InvalidReceiver(ERC721InvalidReceiver),
        ERC721InvalidSender(ERC721InvalidSender),
        ERC721NonexistentToken(ERC721NonexistentToken),
        OwnableInvalidOwner(OwnableInvalidOwner),
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for PloHubNFTErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <ERC721IncorrectOwner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC721IncorrectOwner(decoded));
            }
            if let Ok(decoded) = <ERC721InsufficientApproval as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC721InsufficientApproval(decoded));
            }
            if let Ok(decoded) = <ERC721InvalidApprover as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC721InvalidApprover(decoded));
            }
            if let Ok(decoded) = <ERC721InvalidOperator as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC721InvalidOperator(decoded));
            }
            if let Ok(decoded) = <ERC721InvalidOwner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC721InvalidOwner(decoded));
            }
            if let Ok(decoded) = <ERC721InvalidReceiver as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC721InvalidReceiver(decoded));
            }
            if let Ok(decoded) = <ERC721InvalidSender as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC721InvalidSender(decoded));
            }
            if let Ok(decoded) = <ERC721NonexistentToken as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC721NonexistentToken(decoded));
            }
            if let Ok(decoded) = <OwnableInvalidOwner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnableInvalidOwner(decoded));
            }
            if let Ok(decoded) = <OwnableUnauthorizedAccount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnableUnauthorizedAccount(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PloHubNFTErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ERC721IncorrectOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC721InsufficientApproval(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC721InvalidApprover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC721InvalidOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC721InvalidOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC721InvalidReceiver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC721InvalidSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC721NonexistentToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableInvalidOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for PloHubNFTErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ERC721IncorrectOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC721InsufficientApproval as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC721InvalidApprover as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC721InvalidOperator as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC721InvalidOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC721InvalidReceiver as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC721InvalidSender as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC721NonexistentToken as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OwnableInvalidOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OwnableUnauthorizedAccount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for PloHubNFTErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ERC721IncorrectOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC721InsufficientApproval(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC721InvalidApprover(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC721InvalidOperator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC721InvalidOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC721InvalidReceiver(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC721InvalidSender(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC721NonexistentToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnableInvalidOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for PloHubNFTErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ERC721IncorrectOwner> for PloHubNFTErrors {
        fn from(value: ERC721IncorrectOwner) -> Self {
            Self::ERC721IncorrectOwner(value)
        }
    }
    impl ::core::convert::From<ERC721InsufficientApproval> for PloHubNFTErrors {
        fn from(value: ERC721InsufficientApproval) -> Self {
            Self::ERC721InsufficientApproval(value)
        }
    }
    impl ::core::convert::From<ERC721InvalidApprover> for PloHubNFTErrors {
        fn from(value: ERC721InvalidApprover) -> Self {
            Self::ERC721InvalidApprover(value)
        }
    }
    impl ::core::convert::From<ERC721InvalidOperator> for PloHubNFTErrors {
        fn from(value: ERC721InvalidOperator) -> Self {
            Self::ERC721InvalidOperator(value)
        }
    }
    impl ::core::convert::From<ERC721InvalidOwner> for PloHubNFTErrors {
        fn from(value: ERC721InvalidOwner) -> Self {
            Self::ERC721InvalidOwner(value)
        }
    }
    impl ::core::convert::From<ERC721InvalidReceiver> for PloHubNFTErrors {
        fn from(value: ERC721InvalidReceiver) -> Self {
            Self::ERC721InvalidReceiver(value)
        }
    }
    impl ::core::convert::From<ERC721InvalidSender> for PloHubNFTErrors {
        fn from(value: ERC721InvalidSender) -> Self {
            Self::ERC721InvalidSender(value)
        }
    }
    impl ::core::convert::From<ERC721NonexistentToken> for PloHubNFTErrors {
        fn from(value: ERC721NonexistentToken) -> Self {
            Self::ERC721NonexistentToken(value)
        }
    }
    impl ::core::convert::From<OwnableInvalidOwner> for PloHubNFTErrors {
        fn from(value: OwnableInvalidOwner) -> Self {
            Self::OwnableInvalidOwner(value)
        }
    }
    impl ::core::convert::From<OwnableUnauthorizedAccount> for PloHubNFTErrors {
        fn from(value: OwnableUnauthorizedAccount) -> Self {
            Self::OwnableUnauthorizedAccount(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub approved: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_id: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "BatchMetadataUpdate",
        abi = "BatchMetadataUpdate(uint256,uint256)"
    )]
    pub struct BatchMetadataUpdateFilter {
        pub from_token_id: ::ethers::core::types::U256,
        pub to_token_id: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "MetadataUpdate", abi = "MetadataUpdate(uint256)")]
    pub struct MetadataUpdateFilter {
        pub token_id: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum PloHubNFTEvents {
        ApprovalFilter(ApprovalFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        BatchMetadataUpdateFilter(BatchMetadataUpdateFilter),
        MetadataUpdateFilter(MetadataUpdateFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for PloHubNFTEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(PloHubNFTEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(PloHubNFTEvents::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = BatchMetadataUpdateFilter::decode_log(log) {
                return Ok(PloHubNFTEvents::BatchMetadataUpdateFilter(decoded));
            }
            if let Ok(decoded) = MetadataUpdateFilter::decode_log(log) {
                return Ok(PloHubNFTEvents::MetadataUpdateFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(PloHubNFTEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(PloHubNFTEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PloHubNFTEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApprovalForAllFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BatchMetadataUpdateFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MetadataUpdateFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for PloHubNFTEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalForAllFilter> for PloHubNFTEvents {
        fn from(value: ApprovalForAllFilter) -> Self {
            Self::ApprovalForAllFilter(value)
        }
    }
    impl ::core::convert::From<BatchMetadataUpdateFilter> for PloHubNFTEvents {
        fn from(value: BatchMetadataUpdateFilter) -> Self {
            Self::BatchMetadataUpdateFilter(value)
        }
    }
    impl ::core::convert::From<MetadataUpdateFilter> for PloHubNFTEvents {
        fn from(value: MetadataUpdateFilter) -> Self {
            Self::MetadataUpdateFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for PloHubNFTEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for PloHubNFTEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getApproved` function with signature `getApproved(uint256)` and selector `0x081812fc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getApproved", abi = "getApproved(uint256)")]
    pub struct GetApprovedCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isApprovedForAll", abi = "isApprovedForAll(address,address)")]
    pub struct IsApprovedForAllCall {
        pub owner: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `mintNFT` function with signature `mintNFT(address,string,address)` and selector `0xd01f7a87`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "mintNFT", abi = "mintNFT(address,string,address)")]
    pub struct MintNFTCall {
        pub recipient: ::ethers::core::types::Address,
        pub token_uri: ::std::string::String,
        pub server_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `ownerOf` function with signature `ownerOf(uint256)` and selector `0x6352211e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "ownerOf", abi = "ownerOf(uint256)")]
    pub struct OwnerOfCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256)` and selector `0x42842e0e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256)"
    )]
    pub struct SafeTransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256,bytes)` and selector `0xb88d4fde`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256,bytes)"
    )]
    pub struct SafeTransferFromWithFromAndToAndDataCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setApprovalForAll` function with signature `setApprovalForAll(address,bool)` and selector `0xa22cb465`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setApprovalForAll", abi = "setApprovalForAll(address,bool)")]
    pub struct SetApprovalForAllCall {
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
    }
    ///Container type for all input parameters for the `setToken` function with signature `setToken(address)` and selector `0x144fa6d7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setToken", abi = "setToken(address)")]
    pub struct SetTokenCall {
        pub token_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `tokenURI` function with signature `tokenURI(uint256)` and selector `0xc87b56dd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "tokenURI", abi = "tokenURI(uint256)")]
    pub struct TokenURICall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum PloHubNFTCalls {
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        GetApproved(GetApprovedCall),
        IsApprovedForAll(IsApprovedForAllCall),
        MintNFT(MintNFTCall),
        Name(NameCall),
        Owner(OwnerCall),
        OwnerOf(OwnerOfCall),
        RenounceOwnership(RenounceOwnershipCall),
        SafeTransferFrom(SafeTransferFromCall),
        SafeTransferFromWithFromAndToAndData(SafeTransferFromWithFromAndToAndDataCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SetToken(SetTokenCall),
        SupportsInterface(SupportsInterfaceCall),
        Symbol(SymbolCall),
        TokenURI(TokenURICall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for PloHubNFTCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <GetApprovedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetApproved(decoded));
            }
            if let Ok(decoded) = <IsApprovedForAllCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsApprovedForAll(decoded));
            }
            if let Ok(decoded) = <MintNFTCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MintNFT(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <OwnerOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnerOf(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SafeTransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeTransferFrom(decoded));
            }
            if let Ok(decoded) = <SafeTransferFromWithFromAndToAndDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeTransferFromWithFromAndToAndData(decoded));
            }
            if let Ok(decoded) = <SetApprovalForAllCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetApprovalForAll(decoded));
            }
            if let Ok(decoded) = <SetTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetToken(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) = <TokenURICall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenURI(decoded));
            }
            if let Ok(decoded) = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PloHubNFTCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetApproved(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsApprovedForAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MintNFT(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OwnerOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeTransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeTransferFromWithFromAndToAndData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetApprovalForAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenURI(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PloHubNFTCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetApproved(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsApprovedForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintNFT(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnerOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFromWithFromAndToAndData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetApprovalForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenURI(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApproveCall> for PloHubNFTCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for PloHubNFTCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<GetApprovedCall> for PloHubNFTCalls {
        fn from(value: GetApprovedCall) -> Self {
            Self::GetApproved(value)
        }
    }
    impl ::core::convert::From<IsApprovedForAllCall> for PloHubNFTCalls {
        fn from(value: IsApprovedForAllCall) -> Self {
            Self::IsApprovedForAll(value)
        }
    }
    impl ::core::convert::From<MintNFTCall> for PloHubNFTCalls {
        fn from(value: MintNFTCall) -> Self {
            Self::MintNFT(value)
        }
    }
    impl ::core::convert::From<NameCall> for PloHubNFTCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for PloHubNFTCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<OwnerOfCall> for PloHubNFTCalls {
        fn from(value: OwnerOfCall) -> Self {
            Self::OwnerOf(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for PloHubNFTCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromCall> for PloHubNFTCalls {
        fn from(value: SafeTransferFromCall) -> Self {
            Self::SafeTransferFrom(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromWithFromAndToAndDataCall>
    for PloHubNFTCalls {
        fn from(value: SafeTransferFromWithFromAndToAndDataCall) -> Self {
            Self::SafeTransferFromWithFromAndToAndData(value)
        }
    }
    impl ::core::convert::From<SetApprovalForAllCall> for PloHubNFTCalls {
        fn from(value: SetApprovalForAllCall) -> Self {
            Self::SetApprovalForAll(value)
        }
    }
    impl ::core::convert::From<SetTokenCall> for PloHubNFTCalls {
        fn from(value: SetTokenCall) -> Self {
            Self::SetToken(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for PloHubNFTCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for PloHubNFTCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TokenURICall> for PloHubNFTCalls {
        fn from(value: TokenURICall) -> Self {
            Self::TokenURI(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for PloHubNFTCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for PloHubNFTCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getApproved` function with signature `getApproved(uint256)` and selector `0x081812fc`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetApprovedReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsApprovedForAllReturn(pub bool);
    ///Container type for all return fields from the `mintNFT` function with signature `mintNFT(address,string,address)` and selector `0xd01f7a87`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MintNFTReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `ownerOf` function with signature `ownerOf(uint256)` and selector `0x6352211e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OwnerOfReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `setToken` function with signature `setToken(address)` and selector `0x144fa6d7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SetTokenReturn(pub bool);
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SupportsInterfaceReturn(pub bool);
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `tokenURI` function with signature `tokenURI(uint256)` and selector `0xc87b56dd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TokenURIReturn(pub ::std::string::String);
}
