//! 商品マスタ。
//!
//! 年齢確認の要否 (`age_restricted`) と FF（カウンター商品）かどうか
//! (`fast_food`) は商品マスタの静的な属性として一か所で持つ。
//! 問題7（年確トリガーの不一致）は「判定に使う情報の取得経路が複数ある」
//! ことが原因と推測されるため、ここでは取得経路を一つに固定する。

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxRate {
    /// 軽減税率 8%
    Reduced8,
    /// 標準税率 10%
    Standard10,
}

impl TaxRate {
    pub fn percent(self) -> u32 {
        match self {
            TaxRate::Reduced8 => 8,
            TaxRate::Standard10 => 10,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    Food,
    Drink,
    Alcohol,
    Tobacco,
    Daily,
    FastFood,
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Product {
    pub sku: &'static str,
    pub name: &'static str,
    /// 税込価格（円）
    pub price: u32,
    pub tax: TaxRate,
    pub category: Category,
    pub age_restricted: bool,
    pub fast_food: bool,
}

const fn item(
    sku: &'static str,
    name: &'static str,
    price: u32,
    tax: TaxRate,
    category: Category,
) -> Product {
    let age_restricted = matches!(category, Category::Alcohol | Category::Tobacco);
    let fast_food = matches!(category, Category::FastFood);
    Product {
        sku,
        name,
        price,
        tax,
        category,
        age_restricted,
        fast_food,
    }
}

use Category::*;
use TaxRate::*;

pub const CATALOG: &[Product] = &[
    item("4900001", "おにぎり 鮭", 160, Reduced8, Food),
    item("4900002", "ミックスサンド", 320, Reduced8, Food),
    item("4900003", "緑茶 600ml", 150, Reduced8, Drink),
    item("4900004", "ボトルコーヒー", 180, Reduced8, Drink),
    item("4900005", "ボールペン 黒", 120, Standard10, Daily),
    item("4900010", "缶ビール 350ml", 248, Standard10, Alcohol),
    item("4900011", "レモンサワー 500ml", 228, Standard10, Alcohol),
    item("4900012", "たばこ A", 600, Standard10, Tobacco),
    item("FF01", "フライドチキン", 220, Reduced8, FastFood),
    item("FF02", "からあげ 5個", 260, Reduced8, FastFood),
    item("FF03", "アメリカンドッグ", 140, Reduced8, FastFood),
    item("FF04", "コロッケ", 110, Reduced8, FastFood),
    item("FF05", "肉まん", 180, Reduced8, FastFood),
];

pub fn find(sku: &str) -> Option<&'static Product> {
    CATALOG.iter().find(|p| p.sku == sku)
}

/// FF の初期在庫。コロッケは最初から品切れにしておき、
/// 「品切れ商品はそもそも注文できない」ことを確認できるようにする。
pub const INITIAL_FF_STOCK: &[(&str, u32)] = &[
    ("FF01", 3),
    ("FF02", 2),
    ("FF03", 4),
    ("FF04", 0),
    ("FF05", 5),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn age_restriction_is_derived_from_category_only() {
        for p in CATALOG {
            let expected = matches!(p.category, Category::Alcohol | Category::Tobacco);
            assert_eq!(p.age_restricted, expected, "{}", p.name);
        }
    }

    #[test]
    fn every_ff_has_initial_stock_entry() {
        for p in CATALOG.iter().filter(|p| p.fast_food) {
            assert!(
                INITIAL_FF_STOCK.iter().any(|(s, _)| *s == p.sku),
                "{}",
                p.sku
            );
        }
    }
}
