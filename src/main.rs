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
    fn test_10_pepperoni_costs_95() {
        let mut order = PizzaOrder::new_dateless_order();
        order.add(PizzaType::Pepperoni, 10);
        assert_eq!(order.total(), 95.0);
    }

    #[test]
    fn test_three_brie_costs_45() {
        let mut order = PizzaOrder::new_dateless_order();
        order.add(PizzaType::BrieChickenAndMushroom, 3);
        assert_eq!(order.total(), 45.0);
    }

    #[test]
    fn test_three_brie_added_separately_costs_45() {
        let mut order = PizzaOrder::new_dateless_order();
        order.add(PizzaType::BrieChickenAndMushroom, 1);
        order.add(PizzaType::BrieChickenAndMushroom, 1);
        order.add(PizzaType::BrieChickenAndMushroom, 1);
        assert_eq!(order.total(), 45.0);
    }

    #[test]
    fn test_one_mighty_veg_costs_12() {
        let mut order = PizzaOrder::new_dateless_order();
        order.add(PizzaType::MightyVeg, 1);
        assert_eq!(order.total(), 12.0);
    }

    #[test]
    fn test_two_pepperoni_costs_18_on_mondays() {
        let mut order = PizzaOrder::new_order();
        order.set_day(chrono::Weekday::Mon);
        order.add(PizzaType::Pepperoni, 2);
        assert_eq!(order.total(), 18.0);
    }

    #[test]
    fn test_two_pepperoni_costs_20_on_tuesdays() {
        let mut order = PizzaOrder::new_order();
        order.set_day(chrono::Weekday::Tue);
        order.add(PizzaType::Pepperoni, 2);
        assert_eq!(order.total(), 20.0);
    }

    #[test]
    fn test_one_pepperoni_costs_10_point_5_on_sundays() {
        let mut order = PizzaOrder::new_order();
        order.set_day(chrono::Weekday::Sun);
        order.add(PizzaType::Pepperoni, 1);
        assert_eq!(order.total(), 10.5);
    }

    #[test]
    fn test_one_brie_costs_15_on_mondays() {
        let mut order = PizzaOrder::new_order();
        order.set_day(chrono::Weekday::Mon);
        order.add(PizzaType::BrieChickenAndMushroom, 1);
        assert_eq!(order.total(), 15.0);
    }
}

fn main() {
    let current_time = chrono::offset::Local::now();
    println!("{}", current_time.date().weekday());
}

#[derive(PartialEq, Clone, Copy)]
enum PizzaType {
    Pepperoni,
    BrieChickenAndMushroom,
    MightyVeg,
}

struct PizzaOrder {
    running_total: f64,
    current_day: chrono::Weekday,
    total_pizzas: i64,
}

fn get_cost(pizza_type: PizzaType) -> f64 {
    match pizza_type {
        PizzaType::Pepperoni => 10.0,
        PizzaType::BrieChickenAndMushroom => 15.0,
        PizzaType::MightyVeg => 12.0,
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
        let today = current_time.date().weekday();
        return PizzaOrder {
            running_total: 0.0,
            current_day: today,
            total_pizzas: 0,
        };
    }

    fn new_dateless_order() -> PizzaOrder {
        return PizzaOrder {
            running_total: 0.0,
            current_day: chrono::Weekday::Tue,
            total_pizzas: 0,
        };
    }

    fn set_day(&mut self, new_current_day: chrono::Weekday) -> () {
        self.current_day = new_current_day
    }

    fn bonus_discount_multiplier(&self) -> f64 {
        if self.total_pizzas > 5 {
            return 0.95;
        }
        return 1.0;
    }

    fn total(&self) -> f64 {
        return self.running_total * self.bonus_discount_multiplier();
    }

    fn get_order_line_cost(&self, quantity: f64, pizza_type: PizzaType) -> f64 {
        let pizza_cost = get_cost(pizza_type);
        let mut discount_multiplier = 1.0;
        if self.current_day == chrono::Weekday::Mon {
            match pizza_type {
                PizzaType::BrieChickenAndMushroom => {
                    discount_multiplier = 1.0;
                }
                _ => {
                    discount_multiplier = get_daily_discount(self.current_day);
                }
            }
        } else {
            discount_multiplier = get_daily_discount(self.current_day);
        }
        return pizza_cost * discount_multiplier * quantity;
    }

    // Another associated function, taking two arguments:
    fn add(&mut self, pizza_type: PizzaType, quantity: i64) -> () {
        self.running_total += self.get_order_line_cost(quantity as f64, pizza_type);
        self.total_pizzas += quantity;
    }
}
