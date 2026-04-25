use aet_shared::models::calculations::{CraftingContext, CraftingResult};
use aet_shared::models::specializations::{Category, SpecId};

fn is_item_with_no_rr(unique_name: &str) -> bool {
    ["QUESTITEM_TOKEN_AVALON", "SEAWEED", "FISHCHOPS"]
        .iter()
        .any(|&pattern| unique_name.contains(pattern))
}


fn calculate_total_focus_points(categories: &[Category], target_id: &SpecId) -> u32 {
    categories
        .iter()
        .find(|cat| cat.specs.iter().any(|s| &s.id == target_id))
        .map(|cat| {
            let mastery_bonus = cat.mastery_level * 30;
            let specs_bonus: u32 = cat.specs.iter().map(|s| {
                let mut sum = 0;
                if &s.id == target_id {
                    sum += s.focus_efficiency();
                }
                sum += s.passive_efficiency();
                sum
            }).sum();
            mastery_bonus + specs_bonus
        })
        .unwrap_or(0)
}

pub fn crafting_calculations(context: &CraftingContext) -> Option<CraftingResult> {
    let recipe = context.item.recipes.as_ref()?.first()?;

    let mut total_ingredients_cost = 0.0;
    let mut returnable_cost  = 0.0;

    for ingredient in &recipe.ingredients {
        let price = context
            .prices
            .get(&ingredient.unique_name)
            .map(|p| p.current as f64)
            .unwrap_or(0.0);

        let ingredient_cost = price * ingredient.count as f64;
        total_ingredients_cost += ingredient_cost;

        if !is_item_with_no_rr(&ingredient.unique_name) {
            returnable_cost += ingredient_cost;
        }
    }

    let rrr = context.location.get_rr(context.use_focus);
    let returned_value = returnable_cost * rrr;
    let effective_cost = total_ingredients_cost - returned_value;

    let station_tax = (((context.item.value as f64 * 0.1125) * context.usage_fee as f64)/100.0) * recipe.output_count as f64;

    let actual_cost = effective_cost + station_tax;

    let item_price = context
        .prices
        .get(&context.item.unique_name)
        .map(|p| p.current as f64)
        .unwrap_or(0.0);

    let total_gross_revenue = item_price * recipe.output_count as f64;
    let market_tax_rate = if context.is_premium { 0.065 } else { 0.0105 };
    let total_tax = total_gross_revenue * market_tax_rate;
    let total_net_revenue = total_gross_revenue - total_tax;
    let profit = total_net_revenue - actual_cost;
    let profit_margin = if total_gross_revenue > 0.0 { (profit / total_gross_revenue) * 100.0 } else { 0.0 };
    let focus_cost = 0.0;


    let (silver_per_focus,focus_cost) = if context.use_focus {
        match (context.item.base_focus, &context.item.specialization) {
            (Some(base_focus), Some(spec_id)) => {
                let spec_points = calculate_total_focus_points(context.user_specs, spec_id);
                let factor      = 0.5_f64.powf(spec_points as f64 / 10000.0);
                print!("{}", factor);
                let focus_cost  = (base_focus as f64 * factor).floor();
                if focus_cost > 0.0 { (profit / focus_cost,focus_cost) } else { (0.0,0.0) }

            }
            _ => (0.0,0.0),
        }
    } else {
        (0.0,0.0)
    };



    Some(CraftingResult {
        raw_item_cost: total_ingredients_cost,
        profit: profit/recipe.output_count as f64,
        profit_margin,
        focus_cost,
        silver_per_focus: silver_per_focus/recipe.output_count as f64,
        tax: total_tax/recipe.output_count as f64,
        station_tax,
    })
}

