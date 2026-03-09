#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)] // 增加 Hash 和 Eq 以支持 HashMap
enum ShirtColor {
    Red,
    Green,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // 使用 unwrap_or_else 是正确的，保持懒加载
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        if self.shirts.is_empty() {
            return ShirtColor::Red; // 默认保底颜色
        }

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        // 优化点 1：使用更简洁的计数方式
        for color in &self.shirts {
            match color {
                ShirtColor::Red => red += 1,
                ShirtColor::Green => green += 1,
                ShirtColor::Blue => blue += 1,
            }
        }

        // 优化点 2：比较逻辑优化
        // 找到 red, green, blue 中的最大值并返回对应枚举
        let max = red.max(green).max(blue);

        if max == blue { ShirtColor::Blue }
        else if max == green { ShirtColor::Green }
        else { ShirtColor::Red }
    }
}
fn main() {
   let store  = Inventory{
       shirts: vec![ShirtColor::Red, ShirtColor::Green, ShirtColor::Blue],
   };
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(Some(ShirtColor::Red));

    println!(
        "这个用户最喜欢{:?} gets {:?}",user_pref1,giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!(
        "这个用户最喜欢{:?} gets {:?}",user_pref2,giveaway2
    );
}
