// This is a part of rust-encoding.
// Copyright (c) 2013, Kang Seonghoon.
// See README.md and LICENSE.txt for details.

//! An interface for getting an encoding from given label.

use all;
use types;

/// Returns an encoding from given label if any. Follows WHATWG Encoding Standard "get an encoding"
/// algorithm.
pub fn get_encoding(label: &str) -> Option<~types::Encoding> {
    match label.trim().to_ascii().to_lower().to_str_ascii() {
        ~"unicode-1-1-utf-8" |
        ~"utf-8" |
        ~"utf8" =>
            Some(~all::UTF_8.clone() as ~types::Encoding),
        ~"866" |
        ~"cp866" |
        ~"csibm866" |
        ~"ibm866" =>
            Some(~all::IBM866.clone() as ~types::Encoding),
        ~"csisolatin2" |
        ~"iso-8859-2" |
        ~"iso-ir-101" |
        ~"iso8859-2" |
        ~"iso88592" |
        ~"iso_8859-2" |
        ~"iso_8859-2:1987" |
        ~"l2" |
        ~"latin2" =>
            Some(~all::ISO_8859_2.clone() as ~types::Encoding),
        ~"csisolatin3" |
        ~"iso-8859-3" |
        ~"iso-ir-109" |
        ~"iso8859-3" |
        ~"iso88593" |
        ~"iso_8859-3" |
        ~"iso_8859-3:1988" |
        ~"l3" |
        ~"latin3" =>
            Some(~all::ISO_8859_3.clone() as ~types::Encoding),
        ~"csisolatin4" |
        ~"iso-8859-4" |
        ~"iso-ir-110" |
        ~"iso8859-4" |
        ~"iso88594" |
        ~"iso_8859-4" |
        ~"iso_8859-4:1988" |
        ~"l4" |
        ~"latin4" =>
            Some(~all::ISO_8859_4.clone() as ~types::Encoding),
        ~"csisolatincyrillic" |
        ~"cyrillic" |
        ~"iso-8859-5" |
        ~"iso-ir-144" |
        ~"iso8859-5" |
        ~"iso88595" |
        ~"iso_8859-5" |
        ~"iso_8859-5:1988" =>
            Some(~all::ISO_8859_5.clone() as ~types::Encoding),
        ~"arabic" |
        ~"asmo-708" |
        ~"csiso88596e" |
        ~"csiso88596i" |
        ~"csisolatinarabic" |
        ~"ecma-114" |
        ~"iso-8859-6" |
        ~"iso-8859-6-e" |
        ~"iso-8859-6-i" |
        ~"iso-ir-127" |
        ~"iso8859-6" |
        ~"iso88596" |
        ~"iso_8859-6" |
        ~"iso_8859-6:1987" =>
            Some(~all::ISO_8859_6.clone() as ~types::Encoding),
        ~"csisolatingreek" |
        ~"ecma-118" |
        ~"elot_928" |
        ~"greek" |
        ~"greek8" |
        ~"iso-8859-7" |
        ~"iso-ir-126" |
        ~"iso8859-7" |
        ~"iso88597" |
        ~"iso_8859-7" |
        ~"iso_8859-7:1987" |
        ~"sun_eu_greek" =>
            Some(~all::ISO_8859_7.clone() as ~types::Encoding),
        ~"csiso88598e" |
        ~"csisolatinhebrew" |
        ~"hebrew" |
        ~"iso-8859-8" |
        ~"iso-8859-8-e" |
        ~"iso-ir-138" |
        ~"iso8859-8" |
        ~"iso88598" |
        ~"iso_8859-8" |
        ~"iso_8859-8:1988" |
        ~"visual" =>
            Some(~all::ISO_8859_8.clone() as ~types::Encoding),
        ~"csiso88598i" |
        ~"iso-8859-8-i" |
        ~"logical" =>
            Some(~all::ISO_8859_8_I.clone() as ~types::Encoding),
        ~"csisolatin6" |
        ~"iso-8859-10" |
        ~"iso-ir-157" |
        ~"iso8859-10" |
        ~"iso885910" |
        ~"l6" |
        ~"latin6" =>
            Some(~all::ISO_8859_10.clone() as ~types::Encoding),
        ~"iso-8859-13" |
        ~"iso8859-13" |
        ~"iso885913" =>
            Some(~all::ISO_8859_13.clone() as ~types::Encoding),
        ~"iso-8859-14" |
        ~"iso8859-14" |
        ~"iso885914" =>
            Some(~all::ISO_8859_14.clone() as ~types::Encoding),
        ~"csisolatin9" |
        ~"iso-8859-15" |
        ~"iso8859-15" |
        ~"iso885915" |
        ~"iso_8859-15" |
        ~"l9" =>
            Some(~all::ISO_8859_15.clone() as ~types::Encoding),
        ~"iso-8859-16" =>
            Some(~all::ISO_8859_16.clone() as ~types::Encoding),
        ~"cskoi8r" |
        ~"koi" |
        ~"koi8" |
        ~"koi8-r" |
        ~"koi8_r" =>
            Some(~all::KOI8_R.clone() as ~types::Encoding),
        ~"koi8-u" =>
            Some(~all::KOI8_U.clone() as ~types::Encoding),
        ~"csmacintosh" |
        ~"mac" |
        ~"macintosh" |
        ~"x-mac-roman" =>
            Some(~all::MACINTOSH.clone() as ~types::Encoding),
        ~"dos-874" |
        ~"iso-8859-11" |
        ~"iso8859-11" |
        ~"iso885911" |
        ~"tis-620" |
        ~"windows-874" =>
            Some(~all::WINDOWS_874.clone() as ~types::Encoding),
        ~"cp1250" |
        ~"windows-1250" |
        ~"x-cp1250" =>
            Some(~all::WINDOWS_1250.clone() as ~types::Encoding),
        ~"cp1251" |
        ~"windows-1251" |
        ~"x-cp1251" =>
            Some(~all::WINDOWS_1251.clone() as ~types::Encoding),
        ~"ansi_x3.4-1968" |
        ~"ascii" |
        ~"cp1252" |
        ~"cp819" |
        ~"csisolatin1" |
        ~"ibm819" |
        ~"iso-8859-1" |
        ~"iso-ir-100" |
        ~"iso8859-1" |
        ~"iso88591" |
        ~"iso_8859-1" |
        ~"iso_8859-1:1987" |
        ~"l1" |
        ~"latin1" |
        ~"us-ascii" |
        ~"windows-1252" |
        ~"x-cp1252" =>
            Some(~all::WINDOWS_1252.clone() as ~types::Encoding),
        ~"cp1253" |
        ~"windows-1253" |
        ~"x-cp1253" =>
            Some(~all::WINDOWS_1253.clone() as ~types::Encoding),
        ~"cp1254" |
        ~"csisolatin5" |
        ~"iso-8859-9" |
        ~"iso-ir-148" |
        ~"iso8859-9" |
        ~"iso88599" |
        ~"iso_8859-9" |
        ~"iso_8859-9:1989" |
        ~"l5" |
        ~"latin5" |
        ~"windows-1254" |
        ~"x-cp1254" =>
            Some(~all::WINDOWS_1254.clone() as ~types::Encoding),
        ~"cp1255" |
        ~"windows-1255" |
        ~"x-cp1255" =>
            Some(~all::WINDOWS_1255.clone() as ~types::Encoding),
        ~"cp1256" |
        ~"windows-1256" |
        ~"x-cp1256" =>
            Some(~all::WINDOWS_1256.clone() as ~types::Encoding),
        ~"cp1257" |
        ~"windows-1257" |
        ~"x-cp1257" =>
            Some(~all::WINDOWS_1257.clone() as ~types::Encoding),
        ~"cp1258" |
        ~"windows-1258" |
        ~"x-cp1258" =>
            Some(~all::WINDOWS_1258.clone() as ~types::Encoding),
        ~"x-mac-cyrillic" |
        ~"x-mac-ukrainian" =>
            Some(~all::X_MAC_CYRILLIC.clone() as ~types::Encoding),
        /*
        ~"chinese" |
        ~"csgb2312" |
        ~"csiso58gb231280" |
        ~"gb2312" |
        ~"gb_2312" |
        ~"gb_2312-80" |
        ~"gbk" |
        ~"iso-ir-58" |
        ~"x-gbk" =>
            Some(~all::GBK.clone() as ~types::Encoding),
        ~"gb18030" =>
            Some(~all::GB18030.clone() as ~types::Encoding),
        ~"hz-gb-2312" =>
            Some(~all::HZ_GB_2312.clone() as ~types::Encoding),
        ~"big5" |
        ~"big5-hkscs" |
        ~"cn-big5" |
        ~"csbig5" |
        ~"x-x-big5" =>
            Some(~all::BIG5.clone() as ~types::Encoding),
        ~"cseucpkdfmtjapanese" |
        ~"euc-jp" |
        ~"x-euc-jp" =>
            Some(~all::EUC_JP.clone() as ~types::Encoding),
        ~"csiso2022jp" |
        ~"iso-2022-jp" =>
            Some(~all::ISO_2022_JP.clone() as ~types::Encoding),
        ~"csshiftjis" |
        ~"ms_kanji" |
        ~"shift-jis" |
        ~"shift_jis" |
        ~"sjis" |
        ~"windows-31j" |
        ~"x-sjis" =>
            Some(~all::SHIFT_JIS.clone() as ~types::Encoding),
        */
        ~"cseuckr" |
        ~"csksc56011987" |
        ~"euc-kr" |
        ~"iso-ir-149" |
        ~"korean" |
        ~"ks_c_5601-1987" |
        ~"ks_c_5601-1989" |
        ~"ksc5601" |
        ~"ksc_5601" |
        ~"windows-949" =>
            Some(~all::WINDOWS_949.clone() as ~types::Encoding),
        /*
        ~"csiso2022kr" |
        ~"iso-2022-kr" =>
            Some(~all::ISO_2022_KR.clone() as ~types::Encoding),
        ~"iso-2022-cn" |
        ~"iso-2022-cn-ext" =>
            Some(~all::REPLACEMENT.clone() as ~types::Encoding),
        ~"utf-16be" =>
            Some(~all::UTF_16BE.clone() as ~types::Encoding),
        ~"utf-16" |
        ~"utf-16le" =>
            Some(~all::UTF_16LE.clone() as ~types::Encoding),
        ~"x-user-defined"  =>
            Some(~all::X_USER_DEFINED.clone() as ~types::Encoding),
        */
        _ => None
    }
}

