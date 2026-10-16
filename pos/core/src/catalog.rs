//! 商品マスタとバーコードの振り分け。
//!
//! 会計画面になるまでは、どのバーコードもいつでも受け付ける。読んだ
//! バーコードの種類（商品・ポイントカード・スタッフ・レジ袋・サービス商品）
//! を [`classify`] で一か所で判定し、ふさわしい処理へつなげる。
//! 「先に支払方法を選べ」「先にポイントカードを出せ」といった順番の縛りは持たない。
//!
//! 年齢確認の要否は商品マスタには持たせない。商品は「分類」だけを持ち、
//! 年確の対象かどうかは [`crate::age_rules`] の設定で決まる。

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxRate {
    /// 軽減税率 8%
    Reduced8,
    /// 標準税率 10%
    Standard10,
}

/// 商品の分類。年確ルールは分類単位でも指定できる。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    Food,
    Drink,
    Daily,
    Newspaper,
    Magazine,
    Alcohol,
    Tobacco,
    SmokingGoods,
    AdultBook,
    AdultGoods,
    Lottery,
    FastFood,
    Bag,
    Service,
}

impl Category {
    pub fn label(self) -> &'static str {
        match self {
            Category::Food => "食品",
            Category::Drink => "飲料",
            Category::Daily => "日用品",
            Category::Newspaper => "新聞",
            Category::Magazine => "雑誌",
            Category::Alcohol => "酒類",
            Category::Tobacco => "たばこ",
            Category::SmokingGoods => "喫煙具・ライター",
            Category::AdultBook => "18禁書籍",
            Category::AdultGoods => "アダルトグッズ",
            Category::Lottery => "宝くじ",
            Category::FastFood => "FF",
            Category::Bag => "レジ袋",
            Category::Service => "サービス",
        }
    }

    pub const ALL: &'static [Category] = &[
        Category::Food,
        Category::Drink,
        Category::Daily,
        Category::Newspaper,
        Category::Magazine,
        Category::Alcohol,
        Category::Tobacco,
        Category::SmokingGoods,
        Category::AdultBook,
        Category::AdultGoods,
        Category::Lottery,
        Category::FastFood,
        Category::Bag,
        Category::Service,
    ];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Product {
    /// バーコード（FF はバーコードを持たず、店員画面のメニューから登録する）
    pub code: &'static str,
    pub name: &'static str,
    /// 税込価格（円）。サービス商品は受付時に金額が決まるため 0
    pub price: u32,
    pub tax: TaxRate,
    pub category: Category,
    /// 「要店員」：お客様側で読まれたら店員呼び出しをかけ、店員の受付操作を待つ
    pub needs_staff: bool,
}

const fn p(
    code: &'static str,
    name: &'static str,
    price: u32,
    tax: TaxRate,
    category: Category,
) -> Product {
    Product {
        code,
        name,
        price,
        tax,
        category,
        needs_staff: matches!(category, Category::Service),
    }
}

use Category::*;
use TaxRate::*;

pub const CATALOG: &[Product] = &[
    p("4900000000011", "おにぎり 鮭", 160, Reduced8, Food),
    p("4900000000028", "ミックスサンド", 320, Reduced8, Food),
    p("4900000000035", "緑茶 600ml", 150, Reduced8, Drink),
    p("4900000000042", "ボトルコーヒー", 180, Reduced8, Drink),
    p("4900000000066", "カップ麺 しょうゆ", 214, Reduced8, Food),
    p("4900000000073", "スポーツドリンク", 160, Reduced8, Drink),
    p("4900000000059", "ボールペン 黒", 120, Standard10, Daily),
    p(
        "4910000000014",
        "朝刊（全国紙）",
        160,
        Standard10,
        Newspaper,
    ),
    p("4910000000021", "スポーツ新聞", 160, Standard10, Newspaper),
    p("4910000000038", "週刊誌", 480, Standard10, Magazine),
    p("4910000000045", "マンガ雑誌", 300, Standard10, Magazine),
    p("4900000000103", "缶ビール 350ml", 248, Standard10, Alcohol),
    p(
        "4900000000110",
        "レモンサワー 500ml",
        228,
        Standard10,
        Alcohol,
    ),
    p("4900000000127", "たばこ A", 600, Standard10, Tobacco),
    p(
        "4900000000134",
        "使い捨てライター",
        110,
        Standard10,
        SmokingGoods,
    ),
    p("4900000000141", "成人向け雑誌", 980, Standard10, AdultBook),
    p("4900000000158", "スクラッチくじ", 200, Standard10, Lottery),
    p("2000000000015", "レジ袋 S", 3, Standard10, Bag),
    p("2000000000022", "レジ袋 L", 5, Standard10, Bag),
    p("2100000000014", "宅急便 発送受付", 0, Standard10, Service),
    p(
        "2100000000021",
        "代行収納（公共料金）",
        0,
        Standard10,
        Service,
    ),
    p("FF01", "フライドチキン", 220, Reduced8, FastFood),
    p("FF02", "からあげ 5個", 260, Reduced8, FastFood),
    p("FF03", "アメリカンドッグ", 140, Reduced8, FastFood),
    p("FF04", "コロッケ", 110, Reduced8, FastFood),
    p("FF05", "肉まん", 180, Reduced8, FastFood),
];

pub fn find(code: &str) -> Option<&'static Product> {
    CATALOG.iter().find(|p| p.code == code)
}

/// FF の初期在庫。コロッケは最初から品切れ（店員画面で登録できないことを確認できる）。
pub const INITIAL_FF_STOCK: &[(&str, u32)] = &[
    ("FF01", 3),
    ("FF02", 2),
    ("FF03", 4),
    ("FF04", 0),
    ("FF05", 5),
];

/// 読み取ったバーコードの種類。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Scanned {
    Product(&'static Product),
    PointCard(String),
    Staff(String),
    Unknown,
}

pub const POINT_CARD_PREFIX: &str = "PC";
pub const STAFF_PREFIX: &str = "ST";

pub fn classify(code: &str) -> Scanned {
    let code = code.trim();
    if let Some(no) = code.strip_prefix(POINT_CARD_PREFIX) {
        return Scanned::PointCard(no.to_string());
    }
    if let Some(id) = code.strip_prefix(STAFF_PREFIX) {
        return Scanned::Staff(id.to_string());
    }
    match find(code) {
        // FF はバーコードでは登録しない（店員操作モードのメニューからのみ）
        Some(p) if p.category == FastFood => Scanned::Unknown,
        Some(p) => Scanned::Product(p),
        None => Scanned::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_every_kind_of_barcode() {
        assert!(matches!(classify("4900000000011"), Scanned::Product(p) if p.category == Food));
        assert_eq!(
            classify("PC12345678"),
            Scanned::PointCard("12345678".into())
        );
        assert_eq!(classify("ST1001"), Scanned::Staff("1001".into()));
        assert!(matches!(classify("2000000000015"), Scanned::Product(p) if p.category == Bag));
        assert!(matches!(classify("2100000000014"), Scanned::Product(p) if p.needs_staff));
        assert_eq!(classify("FF01"), Scanned::Unknown);
        assert_eq!(classify("9999"), Scanned::Unknown);
    }
}
