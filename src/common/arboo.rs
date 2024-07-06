pub use arboo::*;
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
pub mod arboo {
    const _: () = {
        ::core::include_bytes!(
            "/Users/alex/development/crypto/arboo_2/contracts/out/arboo.sol/UniswapV3FlashSwap.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("flashSwap_V2_to_V3"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("flashSwap_V2_to_V3"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pool0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fee1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint24"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
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
                    ::std::borrow::ToOwned::to_owned("flashSwap_V3_to_V2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("flashSwap_V3_to_V2"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pool0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fee1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint24"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
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
                    ::std::borrow::ToOwned::to_owned("uniswapV2Call"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("uniswapV2Call"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount1"),
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
                    ::std::borrow::ToOwned::to_owned("uniswapV3SwapCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "uniswapV3SwapCallback",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ARBOO_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x16Wa\x0E\xA6\x90\x81a\0\x1C\x829\xF3[`\0\x80\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x13W`\0\x80\xFD[`\0\x90\x815`\xE0\x1C\x90\x81c\x10\xD1\xE8\\\x14a\tZWP\x80c{\xD0Ae\x14a\x080W\x80c\xF0\xCCh\xC5\x14a\x051Wc\xFAF\x1E3\x14a\0MW`\0\x80\xFD[4a\x05.W``6`\x03\x19\x01\x12a\x05.W`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x05*Wa\0\x80`\xE0\x916\x90`\x04\x01a\x0C\xC2V[\x90\x80\x92\x91\x81\x01\x03\x12a\x05*Wa\0\x95\x81a\x0C\xAEV[\x90a\0\xA2` \x82\x01a\x0C\xAEV[\x91a\0\xAF`@\x83\x01a\x0C\xF0V[Pa\0\xBC``\x83\x01a\x0C\xAEV[a\0\xC8`\x80\x84\x01a\x0C\xAEV[a\0\xD4`\xC0\x85\x01a\rRV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x86\x163\x03a\x04\xF8W\x15a\x04\xE7Wa\0\xF8`$5a\x0EOV[\x90[`@Qc\t^\xA7\xB3`\xE0\x1B\x81Rsz%\rV0\xB4\xCFS\x979\xDF,]\xAC\xB4\xC6Y\xF2H\x8D`\x04\x82\x01R`$\x81\x01\x83\x90R` \x81`D\x81\x8B\x86Z\xF1\x80\x15a\x04\xDCWa\x04\xBDW[P`@Q\x91``\x83\x01\x83\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x04YW`@R`\x02\x83R` \x83\x01\x91`@6\x847\x83Q\x15a\x04\xA7W\x82R\x90\x91\x87\x90`\x01`\x01`\xA0\x1B\x03\x85\x16a\x01\x8A\x82a\x0E`V[R`@Q\x93\x84\x91c8\xED\x179`\xE0\x1B\x83R`\xA4\x83\x01\x94`\x04\x84\x01R`\x01`$\x84\x01R`\xA0`D\x84\x01RQ\x80\x94R`\xC4\x82\x01\x90\x93\x83[\x81\x81\x10a\x04\x82WPP\x81\x92\x93P0`d\x83\x01RB`\x84\x83\x01R\x03\x81\x83sz%\rV0\xB4\xCFS\x979\xDF,]\xAC\xB4\xC6Y\xF2H\x8DZ\xF1\x80\x15a\x04wW\x86\x90a\x03\xB7W[a\x02\x18\x91Pa\x02\x12`\xA0\x86\x015\x91a\x0E`V[Qa\r_V[\x91\x82\x15a\x03\x85W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x86\x90` \x81`$\x81`\x01`\x01`\xA0\x1B\x03\x88\x16Z\xFA\x80\x15a\x03\x1EWa\x03SW[P`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x80\x82R`\x01`\x01`\xA0\x1B\x03\x90\x97\x16`\x04\x82\x01R`\xA0\x95\x90\x95\x015`$\x86\x01R\x93\x94\x93` \x86\x80`D\x81\x01[\x03\x81\x84`\x01`\x01`\xA0\x1B\x03\x88\x16Z\xF1\x94\x85\x15a\x03FWa\x02\xD7\x96` \x96a\x03)W[P`@Q\x90\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x93\x90\x93R\x93\x84\x92\x83\x91\x82\x90`D\x82\x01\x90V[\x03\x92`\x01`\x01`\xA0\x1B\x03\x16Z\xF1\x80\x15a\x03\x1EWa\x02\xF2WP\x80\xF3[a\x03\x13\x90` =` \x11a\x03\x17W[a\x03\x0B\x81\x83a\r\x82V[\x81\x01\x90a\r\xA4V[P\x80\xF3[P=a\x03\x01V[`@Q=\x84\x82>=\x90\xFD[a\x03?\x90\x87=\x89\x11a\x03\x17Wa\x03\x0B\x81\x83a\r\x82V[P8a\x02\xAAV[P`@Q\x90=\x90\x82>=\x90\xFD[` \x80\x92P=\x83\x11a\x03~W[a\x03j\x81\x83a\r\x82V[\x81\x01\x03\x12a\x03yW\x858a\x02QV[`\0\x80\xFD[P=a\x03`V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01Ri\x07\x07&\xF6f\x97B\x03\xD2\x03`\xB4\x1B`D\x82\x01R`d\x90\xFD[P=\x80\x87\x83>a\x03\xC7\x81\x83a\r\x82V[\x81\x01\x90` \x81\x83\x03\x12a\x04oW\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x04sW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x04oW\x81Q\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x04YW\x82`\x05\x1B\x90`@Q\x93a\x04\x1B` \x84\x01\x86a\r\x82V[\x84R` \x80\x85\x01\x92\x82\x01\x01\x92\x83\x11a\x04UW` \x01\x90[\x82\x82\x10a\x04EWPPPa\x02\x18\x90a\x01\xFFV[\x81Q\x81R` \x91\x82\x01\x91\x01a\x042V[\x88\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x86\x80\xFD[\x87\x80\xFD[`@Q=\x88\x82>=\x90\xFD[\x85Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x95\x86\x01\x95\x8C\x95P\x87\x94P\x90\x92\x01\x91`\x01\x01a\x01\xBFV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[a\x04\xD5\x90` =` \x11a\x03\x17Wa\x03\x0B\x81\x83a\r\x82V[P8a\x01=V[`@Q=\x8A\x82>=\x90\xFD[a\x04\xF2`\x045a\x0EOV[\x90a\0\xFAV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01Ri77\xBA\x109\xB2\xB722\xB9`\xB1\x1B`D\x82\x01R`d\x90\xFD[P\x80\xFD[\x80\xFD[P4a\x05.Wa\x05@6a\r\0V[`@Qc\xE6\xA49\x05`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01\x81\x90R\x92\x84\x16`$\x82\x01\x81\x90R\x93\x95\x94` \x94\x92\x93\x90\x91\x85\x81`D\x81s\\i\xBE\xE7\x01\xEF\x81J+j>\xDDK\x16R\xCB\x9C\xC5\xAAoZ\xFA\x80\x15a\x08%W\x88\x91\x8A\x91a\x07\xEBW[P\x16\x92`@Q\x92c\x02@\xBCk`\xE2\x1B\x84R``\x84`\x04\x81\x88Z\xFA\x94\x85\x15a\x07\xE0W\x8A\x94\x8B\x96a\x07~W[P`@Q\x94c+XW{`\xE2\x1B\x86R\x87`\x04\x87\x01R`\x01`\x01`p\x1B\x03\x80\x91\x16\x96\x87`$\x88\x01R\x16\x80`D\x87\x01R\x88\x86`d\x81sz%\rV0\xB4\xCFS\x979\xDF,]\xAC\xB4\xC6Y\xF2H\x8DZ\xFA\x95\x86\x15a\x07sW\x8C\x96a\x07:W[Pa\x06@\x90a\x068b\xFF\xFF\xFF\x95\x96\x97\x98\x8A\x11\x15a\x0E\x10V[\x87\x11\x15a\x0E\x10V[`@Q\x973\x90\x89\x01R`@\x88\x01R\x16``\x86\x01R\x81`\x80\x86\x01R\x80`\xA0\x86\x01R\x83`\xC0\x86\x01R\x82`\xE0\x86\x01Ra\x01\0\x91\x10\x81\x85\x01R\x83Ra\x01 \x83\x01\x94g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x11\x94\x84\x87\x10\x86\x17a\x07&W\x90\x87\x94\x93\x92\x91\x87`@R\x16\x91\x82;\x15a\x07\"Wc\x02,\r\x9F`\xE0\x1B\x87Ra\x01$\x84\x01Ra\x01D\x83\x01R0a\x01d\x83\x01R`\x80a\x01\x84\x83\x01R\x84\x91\x82\x90\x84\x90a\x01\x1F\x19\x90a\x06\xE3a\x01\xA4\x82\x01\x82a\r\xBCV[\x03\x01\x92Z\xF1\x80\x15a\x07\x17Wa\x06\xF6W\x82\x80\xF3[a\x07\x03W`@R8\x80\x82\x80\xF3[cNH{q`\xE0\x1B\x82R`A`\x04R`$\x82\xFD[`@Q=\x85\x82>=\x90\xFD[\x84\x80\xFD[cNH{q`\xE0\x1B\x88R`A`\x04R`$\x88\xFD[\x90\x93\x94\x95P\x88\x81\x81=\x83\x11a\x07lW[a\x07T\x81\x83a\r\x82V[\x81\x01\x03\x12a\x07hWQ\x94\x93\x92a\x06@a\x06 V[\x8B\x80\xFD[P=a\x07JV[`@Q=\x8E\x82>=\x90\xFD[\x94P\x94P``\x84=``\x11a\x07\xD8W[\x81a\x07\x9B``\x93\x83a\r\x82V[\x81\x01\x03\x12a\x07\xD4Wa\x07\xAC\x84a\r\xFCV[`@a\x07\xB9\x89\x87\x01a\r\xFCV[\x95\x01Qc\xFF\xFF\xFF\xFF\x81\x16\x03a\x07\xD0W\x93\x948a\x05\xC8V[\x8A\x80\xFD[\x89\x80\xFD[=\x91Pa\x07\x8EV[`@Q=\x8C\x82>=\x90\xFD[\x80\x92P\x87\x80\x92P=\x83\x11a\x08\x1EW[a\x08\x04\x81\x83a\r\x82V[\x81\x01\x03\x12a\x04UWQ\x87\x81\x16\x81\x03a\x04UW\x87\x908a\x05\x9EV[P=a\x07\xFAV[`@Q=\x8B\x82>=\x90\xFD[P4a\x05.Wa\x08?6a\r\0V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x82\x16\x83\x81\x10\x95\x94\x93\x90\x91\x90\x86\x15a\t?Wd\x01\0\x02v\xA4\x92[b\xFF\xFF\xFF\x85`@Q\x983` \x8B\x01R\x16\x96\x87`@\x8A\x01R\x16``\x88\x01R`\x80\x87\x01R`\xA0\x86\x01R\x80`\xC0\x86\x01R\x85`\xE0\x86\x01R`\xE0\x85Ra\x01\0\x85\x01\x95\x85\x87\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x11\x17a\x07&W\x91\x86\x93\x91`@\x95\x93\x85\x87Rc\x02QYa`\xE3\x1B\x86R0a\x01\x04\x89\x01Ra\x01$\x88\x01Ra\x01D\x87\x01R\x16a\x01d\x85\x01R`\xA0a\x01\x84\x85\x01R\x81\x86`\xFF\x19\x86a\x08\xFFa\x01\xA4\x82\x01\x82a\r\xBCV[\x03\x01\x92Z\xF1\x80\x15a\x07\x17Wa\t\x12W\x82\x80\xF3[`@\x91\x82\x90=\x84\x11a\t7W[\x81a\t)\x91a\r\x82V[\x81\x01\x03\x12a\x05.W8\x80\x82\x80\xF3[=\x91Pa\t\x1FV[s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%\x92a\x08dV[\x90P4a\x05*W`\x806`\x03\x19\x01\x12a\x05*W`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x05*W`d5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x0C\xAAWa\t\xA3a\x01\0\x926\x90`\x04\x01a\x0C\xC2V[\x90\x80\x93\x91\x81\x01\x03\x12a\x0C\xAAWa\t\xB8\x82a\x0C\xAEV[\x90a\t\xC5` \x84\x01a\x0C\xAEV[\x91a\t\xD2`@\x85\x01a\x0C\xF0V[\x90a\t\xDF``\x86\x01a\x0C\xAEV[\x91a\t\xEC`\x80\x87\x01a\x0C\xAEV[\x93a\t\xF9`\xE0\x88\x01a\rRV[Pc\t^\xA7\xB3`\xE0\x1B\x81Rsh\xB3FX3\xFBr\xA7\x0E\xCD\xF4\x85\xE0\xE4\xC7\xBD\x86e\xFCE`\x04\x82\x01R`\xC0\x87\x015`$\x82\x01R` \x81`D\x81\x8B`\x01`\x01`\xA0\x1B\x03\x8A\x16Z\xF1\x80\x15a\x04\xDCWa\x0C\x8BW[P`@Q\x93\x84`\xE0\x81\x01\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xE0\x87\x01\x11\x17a\x07&W`\xE0\x85\x01`@\x90\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x86R\x84\x82\x16` \x80\x88\x01\x91\x82Rb\xFF\xFF\xFF\x94\x85\x16\x83\x89\x01\x90\x81R0``\x8A\x01\x90\x81R`\xC0\x8C\x81\x015`\x80\x8C\x01\x90\x81R`\xA0\x8E\x81\x015\x90\x8D\x01\x90\x81R\x91\x8C\x01\x8F\x81R\x96Qc\x04\xE4Z\xAF`\xE0\x1B\x81R\x9BQ\x88\x16`\x04\x8D\x01R\x94Q\x87\x16`$\x8C\x01R\x91Q\x90\x96\x16`D\x8A\x01R\x94Q\x84\x16`d\x89\x01R\x90Q`\x84\x88\x01R\x92Q`\xA4\x87\x01RQ\x16`\xC4\x85\x01R\x83`\xE4\x81\x89sh\xB3FX3\xFBr\xA7\x0E\xCD\xF4\x85\xE0\xE4\xC7\xBD\x86e\xFCEZ\xF1\x92\x83\x15a\x04wW\x86\x93a\x0CSW[P`\x03`$5\x02`$5\x81\x04`\x03\x14`$5\x15\x17\x15a\x0C?Wa\x03\xE5\x90\x04`$5\x01\x94\x85`$5\x11a\x0C?Wg\r\xE0\xB6\xB3\xA7d\0\0\x86\x01\x80\x96\x11a\x0C?Wa\x0Bv`\xA0\x87\x92\x015\x85a\r_V[\x93\x11\x15a\x0C\x0EW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x86\x90` \x81`$\x81`\x01`\x01`\xA0\x1B\x03\x88\x16Z\xFA\x80\x15a\x03\x1EWa\x0B\xE3W[P`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x80\x82R`\x01`\x01`\xA0\x1B\x03\x90\x96\x16`\x04\x82\x01R`$\x81\x01\x96\x90\x96R` \x86\x80`D\x81\x01a\x02\x88V[` \x90\x81=\x83\x11a\x0C\x07W[a\x0B\xF9\x81\x83a\r\x82V[\x81\x01\x03\x12a\x05.W8a\x0B\xAFV[P=a\x0B\xEFV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x13\x9B\xC8\x14\x1C\x9B\xD9\x9A]`\xBA\x1B`D\x82\x01R`d\x90\xFD[cNH{q`\xE0\x1B\x87R`\x11`\x04R`$\x87\xFD[\x90\x92P` \x81=` \x11a\x0C\x83W[\x81a\x0Co` \x93\x83a\r\x82V[\x81\x01\x03\x12a\x0C\x7FWQ\x918a\x0B)V[\x85\x80\xFD[=\x91Pa\x0CbV[a\x0C\xA3\x90` =` \x11a\x03\x17Wa\x03\x0B\x81\x83a\r\x82V[P8a\nFV[\x82\x80\xFD[5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x03yWV[\x91\x81`\x1F\x84\x01\x12\x15a\x03yW\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x03yW` \x83\x81\x86\x01\x95\x01\x01\x11a\x03yWV[5\x90b\xFF\xFF\xFF\x82\x16\x82\x03a\x03yWV[`\xA0\x90`\x03\x19\x01\x12a\x03yW`\x01`\x01`\xA0\x1B\x03\x90`\x045\x82\x81\x16\x81\x03a\x03yW\x91`$5b\xFF\xFF\xFF\x81\x16\x81\x03a\x03yW\x91`D5\x82\x81\x16\x81\x03a\x03yW\x91`d5\x90\x81\x16\x81\x03a\x03yW\x90`\x845\x90V[5\x90\x81\x15\x15\x82\x03a\x03yWV[\x91\x90\x82\x03\x91\x82\x11a\rlWV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x04YW`@RV[\x90\x81` \x91\x03\x12a\x03yWQ\x80\x15\x15\x81\x03a\x03yW\x90V[\x91\x90\x82Q\x92\x83\x82R`\0[\x84\x81\x10a\r\xE8WPP\x82`\0` \x80\x94\x95\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[` \x81\x83\x01\x81\x01Q\x84\x83\x01\x82\x01R\x01a\r\xC7V[Q\x90`\x01`\x01`p\x1B\x03\x82\x16\x82\x03a\x03yWV[\x15a\x0E\x17WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RoReserves Too Low`\x80\x1B`D\x82\x01R`d\x90\xFD[`\x01`\xFF\x1B\x81\x14a\rlW`\0\x03\x90V[\x80Q`\x01\x10\x15a\x04\xA7W`@\x01\x90V\xFE\xA2dipfsX\"\x12 \x0C\xD2\xF5\x06N\xD94\xA1\x17\xBFL\xEC\xCC<AK)\xA3ga\x17+~\xBFQr:Pv\xED\x9B\xCAdsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static ARBOO_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80\x80`@R`\x046\x10\x15a\0\x13W`\0\x80\xFD[`\0\x90\x815`\xE0\x1C\x90\x81c\x10\xD1\xE8\\\x14a\tZWP\x80c{\xD0Ae\x14a\x080W\x80c\xF0\xCCh\xC5\x14a\x051Wc\xFAF\x1E3\x14a\0MW`\0\x80\xFD[4a\x05.W``6`\x03\x19\x01\x12a\x05.W`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x05*Wa\0\x80`\xE0\x916\x90`\x04\x01a\x0C\xC2V[\x90\x80\x92\x91\x81\x01\x03\x12a\x05*Wa\0\x95\x81a\x0C\xAEV[\x90a\0\xA2` \x82\x01a\x0C\xAEV[\x91a\0\xAF`@\x83\x01a\x0C\xF0V[Pa\0\xBC``\x83\x01a\x0C\xAEV[a\0\xC8`\x80\x84\x01a\x0C\xAEV[a\0\xD4`\xC0\x85\x01a\rRV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x86\x163\x03a\x04\xF8W\x15a\x04\xE7Wa\0\xF8`$5a\x0EOV[\x90[`@Qc\t^\xA7\xB3`\xE0\x1B\x81Rsz%\rV0\xB4\xCFS\x979\xDF,]\xAC\xB4\xC6Y\xF2H\x8D`\x04\x82\x01R`$\x81\x01\x83\x90R` \x81`D\x81\x8B\x86Z\xF1\x80\x15a\x04\xDCWa\x04\xBDW[P`@Q\x91``\x83\x01\x83\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x04YW`@R`\x02\x83R` \x83\x01\x91`@6\x847\x83Q\x15a\x04\xA7W\x82R\x90\x91\x87\x90`\x01`\x01`\xA0\x1B\x03\x85\x16a\x01\x8A\x82a\x0E`V[R`@Q\x93\x84\x91c8\xED\x179`\xE0\x1B\x83R`\xA4\x83\x01\x94`\x04\x84\x01R`\x01`$\x84\x01R`\xA0`D\x84\x01RQ\x80\x94R`\xC4\x82\x01\x90\x93\x83[\x81\x81\x10a\x04\x82WPP\x81\x92\x93P0`d\x83\x01RB`\x84\x83\x01R\x03\x81\x83sz%\rV0\xB4\xCFS\x979\xDF,]\xAC\xB4\xC6Y\xF2H\x8DZ\xF1\x80\x15a\x04wW\x86\x90a\x03\xB7W[a\x02\x18\x91Pa\x02\x12`\xA0\x86\x015\x91a\x0E`V[Qa\r_V[\x91\x82\x15a\x03\x85W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x86\x90` \x81`$\x81`\x01`\x01`\xA0\x1B\x03\x88\x16Z\xFA\x80\x15a\x03\x1EWa\x03SW[P`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x80\x82R`\x01`\x01`\xA0\x1B\x03\x90\x97\x16`\x04\x82\x01R`\xA0\x95\x90\x95\x015`$\x86\x01R\x93\x94\x93` \x86\x80`D\x81\x01[\x03\x81\x84`\x01`\x01`\xA0\x1B\x03\x88\x16Z\xF1\x94\x85\x15a\x03FWa\x02\xD7\x96` \x96a\x03)W[P`@Q\x90\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x93\x90\x93R\x93\x84\x92\x83\x91\x82\x90`D\x82\x01\x90V[\x03\x92`\x01`\x01`\xA0\x1B\x03\x16Z\xF1\x80\x15a\x03\x1EWa\x02\xF2WP\x80\xF3[a\x03\x13\x90` =` \x11a\x03\x17W[a\x03\x0B\x81\x83a\r\x82V[\x81\x01\x90a\r\xA4V[P\x80\xF3[P=a\x03\x01V[`@Q=\x84\x82>=\x90\xFD[a\x03?\x90\x87=\x89\x11a\x03\x17Wa\x03\x0B\x81\x83a\r\x82V[P8a\x02\xAAV[P`@Q\x90=\x90\x82>=\x90\xFD[` \x80\x92P=\x83\x11a\x03~W[a\x03j\x81\x83a\r\x82V[\x81\x01\x03\x12a\x03yW\x858a\x02QV[`\0\x80\xFD[P=a\x03`V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01Ri\x07\x07&\xF6f\x97B\x03\xD2\x03`\xB4\x1B`D\x82\x01R`d\x90\xFD[P=\x80\x87\x83>a\x03\xC7\x81\x83a\r\x82V[\x81\x01\x90` \x81\x83\x03\x12a\x04oW\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x04sW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x04oW\x81Q\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x04YW\x82`\x05\x1B\x90`@Q\x93a\x04\x1B` \x84\x01\x86a\r\x82V[\x84R` \x80\x85\x01\x92\x82\x01\x01\x92\x83\x11a\x04UW` \x01\x90[\x82\x82\x10a\x04EWPPPa\x02\x18\x90a\x01\xFFV[\x81Q\x81R` \x91\x82\x01\x91\x01a\x042V[\x88\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x86\x80\xFD[\x87\x80\xFD[`@Q=\x88\x82>=\x90\xFD[\x85Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x95\x86\x01\x95\x8C\x95P\x87\x94P\x90\x92\x01\x91`\x01\x01a\x01\xBFV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[a\x04\xD5\x90` =` \x11a\x03\x17Wa\x03\x0B\x81\x83a\r\x82V[P8a\x01=V[`@Q=\x8A\x82>=\x90\xFD[a\x04\xF2`\x045a\x0EOV[\x90a\0\xFAV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01Ri77\xBA\x109\xB2\xB722\xB9`\xB1\x1B`D\x82\x01R`d\x90\xFD[P\x80\xFD[\x80\xFD[P4a\x05.Wa\x05@6a\r\0V[`@Qc\xE6\xA49\x05`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01\x81\x90R\x92\x84\x16`$\x82\x01\x81\x90R\x93\x95\x94` \x94\x92\x93\x90\x91\x85\x81`D\x81s\\i\xBE\xE7\x01\xEF\x81J+j>\xDDK\x16R\xCB\x9C\xC5\xAAoZ\xFA\x80\x15a\x08%W\x88\x91\x8A\x91a\x07\xEBW[P\x16\x92`@Q\x92c\x02@\xBCk`\xE2\x1B\x84R``\x84`\x04\x81\x88Z\xFA\x94\x85\x15a\x07\xE0W\x8A\x94\x8B\x96a\x07~W[P`@Q\x94c+XW{`\xE2\x1B\x86R\x87`\x04\x87\x01R`\x01`\x01`p\x1B\x03\x80\x91\x16\x96\x87`$\x88\x01R\x16\x80`D\x87\x01R\x88\x86`d\x81sz%\rV0\xB4\xCFS\x979\xDF,]\xAC\xB4\xC6Y\xF2H\x8DZ\xFA\x95\x86\x15a\x07sW\x8C\x96a\x07:W[Pa\x06@\x90a\x068b\xFF\xFF\xFF\x95\x96\x97\x98\x8A\x11\x15a\x0E\x10V[\x87\x11\x15a\x0E\x10V[`@Q\x973\x90\x89\x01R`@\x88\x01R\x16``\x86\x01R\x81`\x80\x86\x01R\x80`\xA0\x86\x01R\x83`\xC0\x86\x01R\x82`\xE0\x86\x01Ra\x01\0\x91\x10\x81\x85\x01R\x83Ra\x01 \x83\x01\x94g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x11\x94\x84\x87\x10\x86\x17a\x07&W\x90\x87\x94\x93\x92\x91\x87`@R\x16\x91\x82;\x15a\x07\"Wc\x02,\r\x9F`\xE0\x1B\x87Ra\x01$\x84\x01Ra\x01D\x83\x01R0a\x01d\x83\x01R`\x80a\x01\x84\x83\x01R\x84\x91\x82\x90\x84\x90a\x01\x1F\x19\x90a\x06\xE3a\x01\xA4\x82\x01\x82a\r\xBCV[\x03\x01\x92Z\xF1\x80\x15a\x07\x17Wa\x06\xF6W\x82\x80\xF3[a\x07\x03W`@R8\x80\x82\x80\xF3[cNH{q`\xE0\x1B\x82R`A`\x04R`$\x82\xFD[`@Q=\x85\x82>=\x90\xFD[\x84\x80\xFD[cNH{q`\xE0\x1B\x88R`A`\x04R`$\x88\xFD[\x90\x93\x94\x95P\x88\x81\x81=\x83\x11a\x07lW[a\x07T\x81\x83a\r\x82V[\x81\x01\x03\x12a\x07hWQ\x94\x93\x92a\x06@a\x06 V[\x8B\x80\xFD[P=a\x07JV[`@Q=\x8E\x82>=\x90\xFD[\x94P\x94P``\x84=``\x11a\x07\xD8W[\x81a\x07\x9B``\x93\x83a\r\x82V[\x81\x01\x03\x12a\x07\xD4Wa\x07\xAC\x84a\r\xFCV[`@a\x07\xB9\x89\x87\x01a\r\xFCV[\x95\x01Qc\xFF\xFF\xFF\xFF\x81\x16\x03a\x07\xD0W\x93\x948a\x05\xC8V[\x8A\x80\xFD[\x89\x80\xFD[=\x91Pa\x07\x8EV[`@Q=\x8C\x82>=\x90\xFD[\x80\x92P\x87\x80\x92P=\x83\x11a\x08\x1EW[a\x08\x04\x81\x83a\r\x82V[\x81\x01\x03\x12a\x04UWQ\x87\x81\x16\x81\x03a\x04UW\x87\x908a\x05\x9EV[P=a\x07\xFAV[`@Q=\x8B\x82>=\x90\xFD[P4a\x05.Wa\x08?6a\r\0V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x82\x16\x83\x81\x10\x95\x94\x93\x90\x91\x90\x86\x15a\t?Wd\x01\0\x02v\xA4\x92[b\xFF\xFF\xFF\x85`@Q\x983` \x8B\x01R\x16\x96\x87`@\x8A\x01R\x16``\x88\x01R`\x80\x87\x01R`\xA0\x86\x01R\x80`\xC0\x86\x01R\x85`\xE0\x86\x01R`\xE0\x85Ra\x01\0\x85\x01\x95\x85\x87\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x11\x17a\x07&W\x91\x86\x93\x91`@\x95\x93\x85\x87Rc\x02QYa`\xE3\x1B\x86R0a\x01\x04\x89\x01Ra\x01$\x88\x01Ra\x01D\x87\x01R\x16a\x01d\x85\x01R`\xA0a\x01\x84\x85\x01R\x81\x86`\xFF\x19\x86a\x08\xFFa\x01\xA4\x82\x01\x82a\r\xBCV[\x03\x01\x92Z\xF1\x80\x15a\x07\x17Wa\t\x12W\x82\x80\xF3[`@\x91\x82\x90=\x84\x11a\t7W[\x81a\t)\x91a\r\x82V[\x81\x01\x03\x12a\x05.W8\x80\x82\x80\xF3[=\x91Pa\t\x1FV[s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%\x92a\x08dV[\x90P4a\x05*W`\x806`\x03\x19\x01\x12a\x05*W`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x05*W`d5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x0C\xAAWa\t\xA3a\x01\0\x926\x90`\x04\x01a\x0C\xC2V[\x90\x80\x93\x91\x81\x01\x03\x12a\x0C\xAAWa\t\xB8\x82a\x0C\xAEV[\x90a\t\xC5` \x84\x01a\x0C\xAEV[\x91a\t\xD2`@\x85\x01a\x0C\xF0V[\x90a\t\xDF``\x86\x01a\x0C\xAEV[\x91a\t\xEC`\x80\x87\x01a\x0C\xAEV[\x93a\t\xF9`\xE0\x88\x01a\rRV[Pc\t^\xA7\xB3`\xE0\x1B\x81Rsh\xB3FX3\xFBr\xA7\x0E\xCD\xF4\x85\xE0\xE4\xC7\xBD\x86e\xFCE`\x04\x82\x01R`\xC0\x87\x015`$\x82\x01R` \x81`D\x81\x8B`\x01`\x01`\xA0\x1B\x03\x8A\x16Z\xF1\x80\x15a\x04\xDCWa\x0C\x8BW[P`@Q\x93\x84`\xE0\x81\x01\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xE0\x87\x01\x11\x17a\x07&W`\xE0\x85\x01`@\x90\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x86R\x84\x82\x16` \x80\x88\x01\x91\x82Rb\xFF\xFF\xFF\x94\x85\x16\x83\x89\x01\x90\x81R0``\x8A\x01\x90\x81R`\xC0\x8C\x81\x015`\x80\x8C\x01\x90\x81R`\xA0\x8E\x81\x015\x90\x8D\x01\x90\x81R\x91\x8C\x01\x8F\x81R\x96Qc\x04\xE4Z\xAF`\xE0\x1B\x81R\x9BQ\x88\x16`\x04\x8D\x01R\x94Q\x87\x16`$\x8C\x01R\x91Q\x90\x96\x16`D\x8A\x01R\x94Q\x84\x16`d\x89\x01R\x90Q`\x84\x88\x01R\x92Q`\xA4\x87\x01RQ\x16`\xC4\x85\x01R\x83`\xE4\x81\x89sh\xB3FX3\xFBr\xA7\x0E\xCD\xF4\x85\xE0\xE4\xC7\xBD\x86e\xFCEZ\xF1\x92\x83\x15a\x04wW\x86\x93a\x0CSW[P`\x03`$5\x02`$5\x81\x04`\x03\x14`$5\x15\x17\x15a\x0C?Wa\x03\xE5\x90\x04`$5\x01\x94\x85`$5\x11a\x0C?Wg\r\xE0\xB6\xB3\xA7d\0\0\x86\x01\x80\x96\x11a\x0C?Wa\x0Bv`\xA0\x87\x92\x015\x85a\r_V[\x93\x11\x15a\x0C\x0EW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x86\x90` \x81`$\x81`\x01`\x01`\xA0\x1B\x03\x88\x16Z\xFA\x80\x15a\x03\x1EWa\x0B\xE3W[P`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x80\x82R`\x01`\x01`\xA0\x1B\x03\x90\x96\x16`\x04\x82\x01R`$\x81\x01\x96\x90\x96R` \x86\x80`D\x81\x01a\x02\x88V[` \x90\x81=\x83\x11a\x0C\x07W[a\x0B\xF9\x81\x83a\r\x82V[\x81\x01\x03\x12a\x05.W8a\x0B\xAFV[P=a\x0B\xEFV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x13\x9B\xC8\x14\x1C\x9B\xD9\x9A]`\xBA\x1B`D\x82\x01R`d\x90\xFD[cNH{q`\xE0\x1B\x87R`\x11`\x04R`$\x87\xFD[\x90\x92P` \x81=` \x11a\x0C\x83W[\x81a\x0Co` \x93\x83a\r\x82V[\x81\x01\x03\x12a\x0C\x7FWQ\x918a\x0B)V[\x85\x80\xFD[=\x91Pa\x0CbV[a\x0C\xA3\x90` =` \x11a\x03\x17Wa\x03\x0B\x81\x83a\r\x82V[P8a\nFV[\x82\x80\xFD[5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x03yWV[\x91\x81`\x1F\x84\x01\x12\x15a\x03yW\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x03yW` \x83\x81\x86\x01\x95\x01\x01\x11a\x03yWV[5\x90b\xFF\xFF\xFF\x82\x16\x82\x03a\x03yWV[`\xA0\x90`\x03\x19\x01\x12a\x03yW`\x01`\x01`\xA0\x1B\x03\x90`\x045\x82\x81\x16\x81\x03a\x03yW\x91`$5b\xFF\xFF\xFF\x81\x16\x81\x03a\x03yW\x91`D5\x82\x81\x16\x81\x03a\x03yW\x91`d5\x90\x81\x16\x81\x03a\x03yW\x90`\x845\x90V[5\x90\x81\x15\x15\x82\x03a\x03yWV[\x91\x90\x82\x03\x91\x82\x11a\rlWV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x04YW`@RV[\x90\x81` \x91\x03\x12a\x03yWQ\x80\x15\x15\x81\x03a\x03yW\x90V[\x91\x90\x82Q\x92\x83\x82R`\0[\x84\x81\x10a\r\xE8WPP\x82`\0` \x80\x94\x95\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[` \x81\x83\x01\x81\x01Q\x84\x83\x01\x82\x01R\x01a\r\xC7V[Q\x90`\x01`\x01`p\x1B\x03\x82\x16\x82\x03a\x03yWV[\x15a\x0E\x17WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RoReserves Too Low`\x80\x1B`D\x82\x01R`d\x90\xFD[`\x01`\xFF\x1B\x81\x14a\rlW`\0\x03\x90V[\x80Q`\x01\x10\x15a\x04\xA7W`@\x01\x90V\xFE\xA2dipfsX\"\x12 \x0C\xD2\xF5\x06N\xD94\xA1\x17\xBFL\xEC\xCC<AK)\xA3ga\x17+~\xBFQr:Pv\xED\x9B\xCAdsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static ARBOO_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct arboo<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for arboo<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for arboo<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for arboo<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for arboo<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(arboo)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> arboo<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ARBOO_ABI.clone(),
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
                ARBOO_ABI.clone(),
                ARBOO_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `flashSwap_V2_to_V3` (0xf0cc68c5) function
        pub fn flash_swap_v2_to_v3(
            &self,
            pool_0: ::ethers::core::types::Address,
            fee_1: u32,
            token_in: ::ethers::core::types::Address,
            token_out: ::ethers::core::types::Address,
            amount_in: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [240, 204, 104, 197],
                    (pool_0, fee_1, token_in, token_out, amount_in),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `flashSwap_V3_to_V2` (0x7bd04165) function
        pub fn flash_swap_v3_to_v2(
            &self,
            pool_0: ::ethers::core::types::Address,
            fee_1: u32,
            token_in: ::ethers::core::types::Address,
            token_out: ::ethers::core::types::Address,
            amount_in: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [123, 208, 65, 101],
                    (pool_0, fee_1, token_in, token_out, amount_in),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uniswapV2Call` (0x10d1e85c) function
        pub fn uniswap_v2_call(
            &self,
            sender: ::ethers::core::types::Address,
            amount_0: ::ethers::core::types::U256,
            amount_1: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 209, 232, 92], (sender, amount_0, amount_1, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uniswapV3SwapCallback` (0xfa461e33) function
        pub fn uniswap_v3_swap_callback(
            &self,
            amount_0: ::ethers::core::types::I256,
            amount_1: ::ethers::core::types::I256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 70, 30, 51], (amount_0, amount_1, data))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for arboo<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `flashSwap_V2_to_V3` function with signature `flashSwap_V2_to_V3(address,uint24,address,address,uint256)` and selector `0xf0cc68c5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "flashSwap_V2_to_V3",
        abi = "flashSwap_V2_to_V3(address,uint24,address,address,uint256)"
    )]
    pub struct FlashSwapV2ToV3Call {
        pub pool_0: ::ethers::core::types::Address,
        pub fee_1: u32,
        pub token_in: ::ethers::core::types::Address,
        pub token_out: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `flashSwap_V3_to_V2` function with signature `flashSwap_V3_to_V2(address,uint24,address,address,uint256)` and selector `0x7bd04165`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "flashSwap_V3_to_V2",
        abi = "flashSwap_V3_to_V2(address,uint24,address,address,uint256)"
    )]
    pub struct FlashSwapV3ToV2Call {
        pub pool_0: ::ethers::core::types::Address,
        pub fee_1: u32,
        pub token_in: ::ethers::core::types::Address,
        pub token_out: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `uniswapV2Call` function with signature `uniswapV2Call(address,uint256,uint256,bytes)` and selector `0x10d1e85c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "uniswapV2Call",
        abi = "uniswapV2Call(address,uint256,uint256,bytes)"
    )]
    pub struct UniswapV2CallCall {
        pub sender: ::ethers::core::types::Address,
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `uniswapV3SwapCallback` function with signature `uniswapV3SwapCallback(int256,int256,bytes)` and selector `0xfa461e33`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "uniswapV3SwapCallback",
        abi = "uniswapV3SwapCallback(int256,int256,bytes)"
    )]
    pub struct UniswapV3SwapCallbackCall {
        pub amount_0: ::ethers::core::types::I256,
        pub amount_1: ::ethers::core::types::I256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum arbooCalls {
        FlashSwapV2ToV3(FlashSwapV2ToV3Call),
        FlashSwapV3ToV2(FlashSwapV3ToV2Call),
        UniswapV2Call(UniswapV2CallCall),
        UniswapV3SwapCallback(UniswapV3SwapCallbackCall),
    }
    impl ::ethers::core::abi::AbiDecode for arbooCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <FlashSwapV2ToV3Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FlashSwapV2ToV3(decoded));
            }
            if let Ok(decoded) = <FlashSwapV3ToV2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FlashSwapV3ToV2(decoded));
            }
            if let Ok(decoded) = <UniswapV2CallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UniswapV2Call(decoded));
            }
            if let Ok(decoded) = <UniswapV3SwapCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UniswapV3SwapCallback(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for arbooCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::FlashSwapV2ToV3(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FlashSwapV3ToV2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UniswapV2Call(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UniswapV3SwapCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for arbooCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FlashSwapV2ToV3(element) => ::core::fmt::Display::fmt(element, f),
                Self::FlashSwapV3ToV2(element) => ::core::fmt::Display::fmt(element, f),
                Self::UniswapV2Call(element) => ::core::fmt::Display::fmt(element, f),
                Self::UniswapV3SwapCallback(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<FlashSwapV2ToV3Call> for arbooCalls {
        fn from(value: FlashSwapV2ToV3Call) -> Self {
            Self::FlashSwapV2ToV3(value)
        }
    }
    impl ::core::convert::From<FlashSwapV3ToV2Call> for arbooCalls {
        fn from(value: FlashSwapV3ToV2Call) -> Self {
            Self::FlashSwapV3ToV2(value)
        }
    }
    impl ::core::convert::From<UniswapV2CallCall> for arbooCalls {
        fn from(value: UniswapV2CallCall) -> Self {
            Self::UniswapV2Call(value)
        }
    }
    impl ::core::convert::From<UniswapV3SwapCallbackCall> for arbooCalls {
        fn from(value: UniswapV3SwapCallbackCall) -> Self {
            Self::UniswapV3SwapCallback(value)
        }
    }
}
