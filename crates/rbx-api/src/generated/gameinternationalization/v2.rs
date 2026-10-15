//! `gameinternationalization.roblox.com` v2 — generated from `vendor/roblox-api-tracker/gameinternationalization/v2.json`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "gameinternationalization.roblox.com";

/// `Roblox.GameInternationalization.Api.Language`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Language {
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nativeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_name: Option<String>,
}

/// `Roblox.Localization.Client.LanguageFamily`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LanguageFamily {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "isRightToLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_right_to_left: Option<bool>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nativeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_name: Option<String>,
}

crate::api_enum! {
    str
    SupportedLocaleLocale {
    EnUs = 0 => "en_us",
    EsEs = 1 => "es_es",
    FrFr = 2 => "fr_fr",
    IdId = 3 => "id_id",
    ItIt = 4 => "it_it",
    JaJp = 5 => "ja_jp",
    KoKr = 6 => "ko_kr",
    RuRu = 7 => "ru_ru",
    ThTh = 8 => "th_th",
    TrTr = 9 => "tr_tr",
    ViVn = 10 => "vi_vn",
    PtBr = 11 => "pt_br",
    DeDe = 12 => "de_de",
    ZhCn = 13 => "zh_cn",
    ZhTw = 14 => "zh_tw",
    BgBg = 15 => "bg_bg",
    BnBd = 16 => "bn_bd",
    CsCz = 17 => "cs_cz",
    DaDk = 18 => "da_dk",
    ElGr = 19 => "el_gr",
    EtEe = 20 => "et_ee",
    FiFi = 21 => "fi_fi",
    HiIn = 22 => "hi_in",
    HrHr = 23 => "hr_hr",
    HuHu = 24 => "hu_hu",
    KaGe = 25 => "ka_ge",
    KkKz = 26 => "kk_kz",
    KmKh = 27 => "km_kh",
    LtLt = 28 => "lt_lt",
    LvLv = 29 => "lv_lv",
    MsMy = 30 => "ms_my",
    MyMm = 31 => "my_mm",
    NbNo = 32 => "nb_no",
    NlNl = 33 => "nl_nl",
    FilPh = 34 => "fil_ph",
    PlPl = 35 => "pl_pl",
    RoRo = 36 => "ro_ro",
    UkUa = 37 => "uk_ua",
    SiLk = 38 => "si_lk",
    SkSk = 39 => "sk_sk",
    SlSl = 40 => "sl_sl",
    SqAl = 41 => "sq_al",
    BsBa = 42 => "bs_ba",
    SrRs = 43 => "sr_rs",
    SvSe = 44 => "sv_se",
    ZhCjv = 45 => "zh_cjv",
    Ar001 = 46 => "ar_001",
    EnGb = 47 => "en_gb",
    PtPt = 48 => "pt_pt",
    EsMx = 49 => "es_mx",
    FrCa = 50 => "fr_ca",
    }
}

/// `Roblox.Localization.Client.SupportedLocale`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SupportedLocale {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<LanguageFamily>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<SupportedLocaleLocale>,
    #[serde(rename = "localeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nativeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_name: Option<String>,
}

/// `Roblox.GameInternationalization.Api.LanguageWithLocales`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LanguageWithLocales {
    #[serde(rename = "childLocales")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_locales: Option<Vec<SupportedLocale>>,
    #[serde(rename = "languageFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_family: Option<Language>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.GameInternationalization.Api.LanguageWithLocales\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseLanguageWithLocales {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<LanguageWithLocales>>,
}

/// `GET /v2/supported-languages/games/{gameId}`
///
/// Get the supported languages for a game.
pub fn get_supported_languages_games_by_game_id_request(game_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v2/supported-languages/games/{}",
            crate::rt::path_value(&game_id)
        ),
    );
    Ok(r)
}

/// `GET /v2/supported-languages/games/{gameId}`
///
/// Get the supported languages for a game.
pub async fn get_supported_languages_games_by_game_id(
    client: &Client,
    game_id: i64,
) -> Result<ApiArrayResponseLanguageWithLocales> {
    client
        .send(get_supported_languages_games_by_game_id_request(game_id)?)
        .await?
        .json()
}
