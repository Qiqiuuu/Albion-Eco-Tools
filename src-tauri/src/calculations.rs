use aet_shared::models::calculations::{CraftingContext, CraftingResult};
use aet_shared::models::specializations::{Category, SpecId};

fn is_item_with_no_rr(unique_name: &str) -> bool {
    ["QUESTITEM_TOKEN_AVALON", "SEAWEED", "FISHCHOPS"]
        .iter()
        .any(|&pattern| unique_name.contains(pattern))
}


fn calculate_total_focus_points(categories: &[Category], target_id: &SpecId) -> u32 {
    categories.iter()
        .find(|cat| cat.specs.iter().any(|s| &s.id == target_id))
        .map(|cat| {
            let mastery_bonus = cat.mastery_level * 30;
            let specs_bonus: u32 = cat.specs.iter().map(|s| {
                if &s.id == target_id {
                    s.focus_efficiency()
                } else {
                    s.passive_efficiency()
                }
            }).sum();

            mastery_bonus + specs_bonus
        })
        .unwrap_or(0)
}

pub fn crafting_calculations(context: &CraftingContext) -> Option<CraftingResult> {
    let recipe = context.item.recipes.as_ref()?.first()?;

    let mut total_ingredients_cost = 0.0;
    let mut returnable_value_cost  = 0.0;

    for ingredient in &recipe.ingredients {
        let price = context.prices
            .get(&ingredient.unique_name)
            .map(|p| p.current as f64)
            .unwrap_or(0.0);

        let line_cost = price * ingredient.count as f64;
        total_ingredients_cost += line_cost;

        if !is_item_with_no_rr(&ingredient.unique_name) {
            returnable_value_cost += line_cost;
        }
    }

    let rrr = context.location.get_rr(context.use_focus);

    let effective_cost = total_ingredients_cost - (returnable_value_cost * rrr);

    let station_tax = (context.item.value as f64 / 20.0)
        * (context.usage_fee as f64 / 44.444);

    let actual_cost = effective_cost + station_tax;

    let item_price = context.prices
        .get(&context.item.unique_name)
        .map(|p| p.current as f64)
        .unwrap_or(0.0);

    let market_value    = item_price * recipe.output_count as f64;
    let tax_rate        = if context.is_premium { 0.065 } else { 0.105 };
    let market_tax      = market_value * tax_rate;
    let revenue         = market_value - market_tax;
    let net_profit      = revenue - actual_cost;
    let profit_margin   = if market_value > 0.0 { net_profit / market_value * 100.0 } else { 0.0 };


    let focus_efficiency = if context.use_focus {
        match (context.item.base_focus, &context.item.specialization) {
            (Some(base_focus), Some(spec_id)) => {
                let spec_points = calculate_total_focus_points(context.user_specs, spec_id);
                let factor      = 0.5_f64.powf(spec_points as f64 / 10000.0);
                let focus_cost  = (base_focus as f64 * factor).floor();
                if focus_cost > 0.0 { net_profit / focus_cost } else { 0.0 }
            }
            _ => 0.0,
        }
    } else {
        0.0
    };

    Some(CraftingResult {
        raw_item_cost: total_ingredients_cost,
        actual_cost,
        crafting_tax: station_tax,
        market_value,
        market_tax_total: market_tax,
        net_profit,
        profit_margin,
        focus_efficiency,
    })
}

