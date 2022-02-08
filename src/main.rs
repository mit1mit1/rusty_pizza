use chrono::Datelike;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_cost_is_0() {
        assert_eq!(PizzaOrder::new_order().total(), 0.0);
    }

    #[test]
    fn test_two_pepperoni_costs_20() {
        let mut order = PizzaOrder::new_dateless_order();
        order.add(PizzaType::Pepperoni, 2);
        assert_eq!(order.total(), 20.0);
    }

    #[test]
    fn test_zero_pepperoni_costs_0() {
        let mut order = PizzaOrder::new_dateless_order();
        order.add(PizzaType::Pepperoni, 0);
        assert_eq!(order.total(), 0.0);
    }

    #[test]
    fn test_three_brie_costs_45() {
        let mut order = PizzaOrder::new_dateless_order();
        order.add(PizzaType::Brie_Chicken_And_Mushroom, 3);
        assert_eq!(order.total(), 45.0);
    }

    #[test]
    fn test_three_brie_added_separately_costs_45() {
        let mut order = PizzaOrder::new_dateless_order();
        order.add(PizzaType::Brie_Chicken_And_Mushroom, 1);
        order.add(PizzaType::Brie_Chicken_And_Mushroom, 1);
        order.add(PizzaType::Brie_Chicken_And_Mushroom, 1);
        assert_eq!(order.total(), 45.0);
    }

    #[test]
    fn test_one_mighty_veg_costs_12() {
        let mut order = PizzaOrder::new_dateless_order();
        order.add(PizzaType::Mighty_Veg, 1);
        assert_eq!(order.total(), 12.0);
    }

    #[test]
    fn test_two_pepperoni_costs_18_on_mondays() {
        let mut order = PizzaOrder::new_order();
        order.set_daily_discount(chrono::Weekday::Mon);
        order.add(PizzaType::Pepperoni, 2);
        assert_eq!(order.total(), 18.0);
    }

    #[test]
    fn test_two_pepperoni_costs_20_on_tuesdays() {
        let mut order = PizzaOrder::new_order();
        order.set_daily_discount(chrono::Weekday::Tue);
        order.add(PizzaType::Pepperoni, 2);
        assert_eq!(order.total(), 20.0);
    }

    #[test]
    fn test_one_pepperoni_costs_10_point_5_on_sundays() {
        let mut order = PizzaOrder::new_order();
        order.set_daily_discount(chrono::Weekday::Sun);
        order.add(PizzaType::Pepperoni, 1);
        assert_eq!(order.total(), 10.5);
    }
}

fn main() {
    let current_time = chrono::offset::Local::now();
    println!("{}", current_time.date().weekday());
}

enum PizzaType {
    Pepperoni,
    Brie_Chicken_And_Mushroom,
    Mighty_Veg,
}

struct PizzaOrder {
    running_total: f64,
    daily_discount_multiplier: f64,
    total_pizzas: i64,
}

fn get_cost(pizza_type: PizzaType) -> f64 {
    match pizza_type {
        PizzaType::Pepperoni => 10.0,
        PizzaType::Brie_Chicken_And_Mushroom => 15.0,
        PizzaType::Mighty_Veg => 12.0,
    }
}

fn get_daily_discount(current_day: chrono::Weekday) -> f64 {
    match current_day {
        chrono::Weekday::Mon => 0.9,
        chrono::Weekday::Tue => 1.0,
        chrono::Weekday::Wed => 1.0,
        chrono::Weekday::Thu => 1.0,
        chrono::Weekday::Fri => 1.0,
        chrono::Weekday::Sat => 1.0,
        chrono::Weekday::Sun => 1.05,
    }
}

// Implementation block, all `PizzaOrder` associated functions & methods go in here
impl PizzaOrder {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, PizzaOrder.
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    fn new_order() -> PizzaOrder {
        let current_time = chrono::offset::Local::now();
        let current_day = current_time.date().weekday();
        return PizzaOrder {
            running_total: 0.0,
            daily_discount_multiplier: get_daily_discount(current_day),
            total_pizzas: 0,
        };
    }

    fn new_dateless_order() -> PizzaOrder {
        return PizzaOrder {
            running_total: 0.0,
            daily_discount_multiplier: 1.0,
            total_pizzas: 0,
        };
    }

    fn set_daily_discount(&mut self, new_current_day: chrono::Weekday) -> () {
        self.daily_discount_multiplier = get_daily_discount(new_current_day)
    }

    fn total(&self) -> f64 {
        return self.running_total;
    }

    // Another associated function, taking two arguments:
    fn add(&mut self, pizza_type: PizzaType, quantity: i64) -> () {
        self.running_total += quantity as f64 * get_cost(pizza_type) * self.daily_discount_multiplier;
    }
}
