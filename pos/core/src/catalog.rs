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
    Fireworks,
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
            Category::Fireworks => "花火",
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
        Category::Fireworks,
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
    p(
        "4900000000165",
        "手持ち花火セット",
        598,
        Standard10,
        Fireworks,
    ),
    p("4900000000172", "線香花火", 220, Standard10, Fireworks),
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
    // FF（バーコードなし。レジ画面下のタブから登録する）
    p("FF01", "フライドチキン", 220, Reduced8, FastFood),
    p("FF02", "からあげ 5個", 260, Reduced8, FastFood),
    p("FF03", "アメリカンドッグ", 140, Reduced8, FastFood),
    p("FF04", "コロッケ", 110, Reduced8, FastFood),
    p("FF05", "肉まん", 180, Reduced8, FastFood),
    p("FF06", "ポテト S", 180, Reduced8, FastFood),
    p("FF07", "メンチカツ", 160, Reduced8, FastFood),
    p("FF08", "ハッシュポテト", 120, Reduced8, FastFood),
    p("FF09", "ナゲット 5個", 230, Reduced8, FastFood),
    p("FF10", "春巻き", 150, Reduced8, FastFood),
    p("FF11", "チーズドッグ", 180, Reduced8, FastFood),
    p("FF21", "焼きいも", 200, Reduced8, FastFood),
    p("FF22", "いなり寿司 2個", 190, Reduced8, FastFood),
    p("FF23", "大学いも", 250, Reduced8, FastFood),
    p("FF24", "おはぎ 2個", 220, Reduced8, FastFood),
    p("FF31", "ホットコーヒー R", 120, Reduced8, FastFood),
    p("FF32", "ホットコーヒー L", 180, Reduced8, FastFood),
    p("FF33", "カフェラテ R", 190, Reduced8, FastFood),
    p("FF34", "ホットティー", 150, Reduced8, FastFood),
    p("FF35", "ココア", 200, Reduced8, FastFood),
    p("FF41", "アイスコーヒー R", 120, Reduced8, FastFood),
    p("FF42", "アイスコーヒー L", 180, Reduced8, FastFood),
    p("FF43", "アイスカフェラテ", 210, Reduced8, FastFood),
    p("FF44", "フラッペ", 360, Reduced8, FastFood),
    p("FF45", "ソフトクリーム", 300, Reduced8, FastFood),
    p("FF51", "あんまん", 160, Reduced8, FastFood),
    p("FF52", "ピザまん", 180, Reduced8, FastFood),
    p("FF53", "おでん 大根", 100, Reduced8, FastFood),
    p("FF54", "おでん たまご", 100, Reduced8, FastFood),
];

/// FF のタブ（レジ画面の下 1/3）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FfTab {
    Fried,
    Deli,
    Drink,
    Cold,
    Other,
}

impl FfTab {
    pub fn label(self) -> &'static str {
        match self {
            FfTab::Fried => "揚げ物",
            FfTab::Deli => "常温総菜",
            FfTab::Drink => "ドリンク",
            FfTab::Cold => "コールド",
            FfTab::Other => "その他",
        }
    }
}

/// FF のタブ分けと並び順。各タブの中は売れ筋順で、先頭ページの最上部に売れ筋が来る。
pub const FF_MENU: &[(&str, FfTab)] = &[
    ("FF02", FfTab::Fried),
    ("FF01", FfTab::Fried),
    ("FF03", FfTab::Fried),
    ("FF06", FfTab::Fried),
    ("FF04", FfTab::Fried),
    ("FF07", FfTab::Fried),
    ("FF09", FfTab::Fried),
    ("FF08", FfTab::Fried),
    ("FF11", FfTab::Fried),
    ("FF10", FfTab::Fried),
    ("FF21", FfTab::Deli),
    ("FF22", FfTab::Deli),
    ("FF23", FfTab::Deli),
    ("FF24", FfTab::Deli),
    ("FF31", FfTab::Drink),
    ("FF33", FfTab::Drink),
    ("FF32", FfTab::Drink),
    ("FF35", FfTab::Drink),
    ("FF34", FfTab::Drink),
    ("FF41", FfTab::Cold),
    ("FF43", FfTab::Cold),
    ("FF42", FfTab::Cold),
    ("FF44", FfTab::Cold),
    ("FF45", FfTab::Cold),
    ("FF05", FfTab::Other),
    ("FF51", FfTab::Other),
    ("FF52", FfTab::Other),
    ("FF53", FfTab::Other),
    ("FF54", FfTab::Other),
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
    ("FF06", 6),
    ("FF07", 3),
    ("FF08", 5),
    ("FF09", 4),
    ("FF10", 2),
    ("FF11", 3),
    ("FF21", 6),
    ("FF22", 4),
    ("FF23", 3),
    ("FF24", 2),
    // カウンターのドリンクはカップがある限り作れる
    ("FF31", 99),
    ("FF32", 99),
    ("FF33", 99),
    ("FF34", 99),
    ("FF35", 99),
    ("FF41", 99),
    ("FF42", 99),
    ("FF43", 99),
    ("FF44", 20),
    ("FF45", 20),
    ("FF51", 4),
    ("FF52", 3),
    ("FF53", 10),
    ("FF54", 8),
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
        // FF はどれもタブに入り、在庫を持つ
        for p in CATALOG.iter().filter(|p| p.category == Category::FastFood) {
            assert!(FF_MENU.iter().any(|(c, _)| *c == p.code), "{}", p.code);
            assert!(
                INITIAL_FF_STOCK.iter().any(|(c, _)| *c == p.code),
                "{}",
                p.code
            );
        }
        assert_eq!(classify("9999"), Scanned::Unknown);
    }
}
