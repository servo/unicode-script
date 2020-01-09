// Copyright 2014 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub mod tables;
pub use tables::Script;

/// The version of [Unicode](http://www.unicode.org/)
/// that this version of unicode-script is based on.
pub const UNICODE_VERSION: (u64, u64, u64) = (12, 1, 0);

#[cfg(feature = "harfbuzz")]
extern crate harfbuzz_sys;

fn bsearch_range_value_table(c: char, r: &'static [(u32, u32, Script)]) -> Script {
    use ::std::cmp::Ordering::{Equal, Less, Greater};
    let c = c as u32;
    match r.binary_search_by(|&(lo, hi, _)| {
        if lo <= c && c <= hi { Equal }
        else if hi < c { Less }
        else { Greater }
    }) {
        Ok(idx) => {
            let (_, _, cat) = r[idx];
            cat
        }
        Err(_) => Script::Unknown
    }
}

/// Find the script of a single char.
pub fn get_script(c: char) -> Script {
    bsearch_range_value_table(c, tables::SCRIPT)
}

impl Script {
    #[cfg(feature = "harfbuzz")]
    pub fn to_hb_script(self) -> harfbuzz_sys::hb_script_t {
        use harfbuzz_sys::*;
        use Script::*;
        match self {
            Adlam => HB_SCRIPT_ADLAM,
            Ahom => HB_SCRIPT_AHOM,
            AnatolianHieroglyphs => HB_SCRIPT_ANATOLIAN_HIEROGLYPHS,
            Arabic => HB_SCRIPT_ARABIC,
            Armenian => HB_SCRIPT_ARMENIAN,
            Avestan => HB_SCRIPT_AVESTAN,
            Balinese => HB_SCRIPT_BALINESE,
            Bamum => HB_SCRIPT_BAMUM,
            BassaVah => HB_SCRIPT_BASSA_VAH,
            Batak => HB_SCRIPT_BATAK,
            Bengali => HB_SCRIPT_BENGALI,
            Bhaiksuki => HB_SCRIPT_BHAIKSUKI,
            Bopomofo => HB_SCRIPT_BOPOMOFO,
            Brahmi => HB_SCRIPT_BRAHMI,
            Braille => HB_SCRIPT_BRAILLE,
            Buginese => HB_SCRIPT_BUGINESE,
            Buhid => HB_SCRIPT_BUHID,
            CanadianAboriginal => HB_SCRIPT_CANADIAN_SYLLABICS,
            Carian => HB_SCRIPT_CARIAN,
            CaucasianAlbanian => HB_SCRIPT_CAUCASIAN_ALBANIAN,
            Chakma => HB_SCRIPT_CHAKMA,
            Cham => HB_SCRIPT_CHAM,
            Cherokee => HB_SCRIPT_CHEROKEE,
            Common => HB_SCRIPT_COMMON,
            Coptic => HB_SCRIPT_COPTIC,
            Cuneiform => HB_SCRIPT_CUNEIFORM,
            Cypriot => HB_SCRIPT_CYPRIOT,
            Cyrillic => HB_SCRIPT_CYRILLIC,
            Deseret => HB_SCRIPT_DESERET,
            Devanagari => HB_SCRIPT_DEVANAGARI,
            Dogra => HB_SCRIPT_DOGRA,
            Duployan => HB_SCRIPT_DUPLOYAN,
            EgyptianHieroglyphs => HB_SCRIPT_EGYPTIAN_HIEROGLYPHS,
            Elbasan => HB_SCRIPT_ELBASAN,
            Elymaic => HB_SCRIPT_ELYMAIC,
            Ethiopic => HB_SCRIPT_ETHIOPIC,
            Georgian => HB_SCRIPT_GEORGIAN,
            Glagolitic => HB_SCRIPT_GLAGOLITIC,
            Gothic => HB_SCRIPT_GOTHIC,
            Grantha => HB_SCRIPT_GRANTHA,
            Greek => HB_SCRIPT_GREEK,
            Gujarati => HB_SCRIPT_GUJARATI,
            GunjalaGondi => HB_SCRIPT_GUNJALA_GONDI,
            Gurmukhi => HB_SCRIPT_GURMUKHI,
            Han => HB_SCRIPT_HAN,
            Hangul => HB_SCRIPT_HANGUL,
            HanifiRohingya => HB_SCRIPT_HANIFI_ROHINGYA,
            Hanunoo => HB_SCRIPT_HANUNOO,
            Hatran => HB_SCRIPT_HATRAN,
            Hebrew => HB_SCRIPT_HEBREW,
            Hiragana => HB_SCRIPT_HIRAGANA,
            ImperialAramaic => HB_SCRIPT_IMPERIAL_ARAMAIC,
            Inherited => HB_SCRIPT_INHERITED,
            InscriptionalPahlavi => HB_SCRIPT_INSCRIPTIONAL_PAHLAVI,
            InscriptionalParthian => HB_SCRIPT_INSCRIPTIONAL_PARTHIAN,
            Javanese => HB_SCRIPT_JAVANESE,
            Kaithi => HB_SCRIPT_KAITHI,
            Kannada => HB_SCRIPT_KANNADA,
            Katakana => HB_SCRIPT_KATAKANA,
            KayahLi => HB_SCRIPT_KAYAH_LI,
            Kharoshthi => HB_SCRIPT_KHAROSHTHI,
            Khmer => HB_SCRIPT_KHMER,
            Khojki => HB_SCRIPT_KHOJKI,
            Khudawadi => HB_SCRIPT_KHUDAWADI,
            Lao => HB_SCRIPT_LAO,
            Latin => HB_SCRIPT_LATIN,
            Lepcha => HB_SCRIPT_LEPCHA,
            Limbu => HB_SCRIPT_LIMBU,
            LinearA => HB_SCRIPT_LINEAR_A,
            LinearB => HB_SCRIPT_LINEAR_B,
            Lisu => HB_SCRIPT_LISU,
            Lycian => HB_SCRIPT_LYCIAN,
            Lydian => HB_SCRIPT_LYDIAN,
            Mahajani => HB_SCRIPT_MAHAJANI,
            Makasar => HB_SCRIPT_MAKASAR,
            Malayalam => HB_SCRIPT_MALAYALAM,
            Mandaic => HB_SCRIPT_MANDAIC,
            Manichaean => HB_SCRIPT_MANICHAEAN,
            Marchen => HB_SCRIPT_MARCHEN,
            MasaramGondi => HB_SCRIPT_MASARAM_GONDI,
            Medefaidrin => HB_SCRIPT_MEDEFAIDRIN,
            MeeteiMayek => HB_SCRIPT_MEETEI_MAYEK,
            MendeKikakui => HB_SCRIPT_MENDE_KIKAKUI,
            MeroiticCursive => HB_SCRIPT_MEROITIC_CURSIVE,
            MeroiticHieroglyphs => HB_SCRIPT_MEROITIC_HIEROGLYPHS,
            Miao => HB_SCRIPT_MIAO,
            Modi => HB_SCRIPT_MODI,
            Mongolian => HB_SCRIPT_MONGOLIAN,
            Mro => HB_SCRIPT_MRO,
            Multani => HB_SCRIPT_MULTANI,
            Myanmar => HB_SCRIPT_MYANMAR,
            Nabataean => HB_SCRIPT_NABATAEAN,
            Nandinagari => HB_SCRIPT_NANDINAGARI,
            NewTaiLue => HB_SCRIPT_NEW_TAI_LUE,
            Newa => HB_SCRIPT_NEWA,
            Nko => HB_SCRIPT_NKO,
            Nushu => HB_SCRIPT_NUSHU,
            NyiakengPuachueHmong => HB_SCRIPT_NYIAKENG_PUACHUE_HMONG,
            Ogham => HB_SCRIPT_OGHAM,
            OlChiki => HB_SCRIPT_OL_CHIKI,
            OldHungarian => HB_SCRIPT_OLD_HUNGARIAN,
            OldItalic => HB_SCRIPT_OLD_ITALIC,
            OldNorthArabian => HB_SCRIPT_OLD_NORTH_ARABIAN,
            OldPermic => HB_SCRIPT_OLD_PERMIC,
            OldPersian => HB_SCRIPT_OLD_PERSIAN,
            OldSogdian => HB_SCRIPT_OLD_SOGDIAN,
            OldSouthArabian => HB_SCRIPT_OLD_SOUTH_ARABIAN,
            OldTurkic => HB_SCRIPT_OLD_TURKIC,
            Oriya => HB_SCRIPT_ORIYA,
            Osage => HB_SCRIPT_OSAGE,
            Osmanya => HB_SCRIPT_OSMANYA,
            PahawhHmong => HB_SCRIPT_PAHAWH_HMONG,
            Palmyrene => HB_SCRIPT_PALMYRENE,
            PauCinHau => HB_SCRIPT_PAU_CIN_HAU,
            PhagsPa => HB_SCRIPT_PHAGS_PA,
            Phoenician => HB_SCRIPT_PHOENICIAN,
            PsalterPahlavi => HB_SCRIPT_PSALTER_PAHLAVI,
            Rejang => HB_SCRIPT_REJANG,
            Runic => HB_SCRIPT_RUNIC,
            Samaritan => HB_SCRIPT_SAMARITAN,
            Saurashtra => HB_SCRIPT_SAURASHTRA,
            Sharada => HB_SCRIPT_SHARADA,
            Shavian => HB_SCRIPT_SHAVIAN,
            Siddham => HB_SCRIPT_SIDDHAM,
            Signwriting => HB_SCRIPT_SIGNWRITING,
            Sinhala => HB_SCRIPT_SINHALA,
            Sogdian => HB_SCRIPT_SOGDIAN,
            SoraSompeng => HB_SCRIPT_SORA_SOMPENG,
            Soyombo => HB_SCRIPT_SOYOMBO,
            Sundanese => HB_SCRIPT_SUNDANESE,
            SylotiNagri => HB_SCRIPT_SYLOTI_NAGRI,
            Syriac => HB_SCRIPT_SYRIAC,
            Tagalog => HB_SCRIPT_TAGALOG,
            Tagbanwa => HB_SCRIPT_TAGBANWA,
            TaiLe => HB_SCRIPT_TAI_LE,
            TaiTham => HB_SCRIPT_TAI_THAM,
            TaiViet => HB_SCRIPT_TAI_VIET,
            Takri => HB_SCRIPT_TAKRI,
            Tamil => HB_SCRIPT_TAMIL,
            Tangut => HB_SCRIPT_TANGUT,
            Telugu => HB_SCRIPT_TELUGU,
            Thaana => HB_SCRIPT_THAANA,
            Thai => HB_SCRIPT_THAI,
            Tibetan => HB_SCRIPT_TIBETAN,
            Tifinagh => HB_SCRIPT_TIFINAGH,
            Tirhuta => HB_SCRIPT_TIRHUTA,
            Ugaritic => HB_SCRIPT_UGARITIC,
            Unknown => HB_SCRIPT_UNKNOWN,
            Vai => HB_SCRIPT_VAI,
            Wancho => HB_SCRIPT_WANCHO,
            WarangCiti => HB_SCRIPT_WARANG_CITI,
            Yi => HB_SCRIPT_YI,
            ZanabazarSquare => HB_SCRIPT_ZANABAZAR_SQUARE,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{get_script, Script};

    #[test]
    fn test_get_script() {
        assert_eq!(get_script('a'), Script::Latin);
        assert_eq!(get_script('.'), Script::Common);
        assert_eq!(get_script('カ'), Script::Katakana);
        assert_eq!(get_script('🦳'), Script::Common);
    }
}
