//! This module contains all tauri event channels.

pub const ENTITY_ACTION_CREATED_INGREDIENT: &str = "ENTITY_ACTION_CREATED_INGREDIENT";
pub const ENTITY_ACTION_UPDATED_INGREDIENT: &str = "ENTITY_ACTION_UPDATED_INGREDIENT";
pub const ENTITY_ACTION_DELETED_INGREDIENT: &str = "ENTITY_ACTION_DELETED_INGREDIENT";

pub const ENTITY_ACTION_CREATED_RECIPE: &str = "ENTITY_ACTION_CREATED_RECIPE";
pub const ENTITY_ACTION_UPDATED_RECIPE: &str = "ENTITY_ACTION_UPDATED_RECIPE";
pub const ENTITY_ACTION_DELETED_RECIPE: &str = "ENTITY_ACTION_DELETED_RECIPE";

pub const ENTITY_ACTION_CREATED_RECIPE_FILE: &str = "ENTITY_ACTION_CREATED_RECIPE_FILE";
pub const ENTITY_ACTION_UPDATED_RECIPE_FILE: &str = "ENTITY_ACTION_UPDATED_RECIPE_FILE";
pub const ENTITY_ACTION_DELETED_RECIPE_FILE: &str = "ENTITY_ACTION_DELETED_RECIPE_FILE";

pub const ENTITY_ACTION_CREATED_RECIPE_STEP_INGREDIENT: &str =
    "ENTITY_ACTION_CREATED_RECIPE_STEP_INGREDIENT";
pub const ENTITY_ACTION_UPDATED_RECIPE_STEP_INGREDIENT: &str =
    "ENTITY_ACTION_UPDATED_RECIPE_STEP_INGREDIENT";
pub const ENTITY_ACTION_DELETED_RECIPE_STEP_INGREDIENT: &str =
    "ENTITY_ACTION_DELETED_RECIPE_STEP_INGREDIENT";

pub const ENTITY_ACTION_CREATED_RECIPE_STEP_INGREDIENT_DRAFT: &str =
    "ENTITY_ACTION_CREATED_RECIPE_STEP_INGREDIENT_DRAFT";
pub const ENTITY_ACTION_UPDATED_RECIPE_STEP_INGREDIENT_DRAFT: &str =
    "ENTITY_ACTION_UPDATED_RECIPE_STEP_INGREDIENT_DRAFT";
pub const ENTITY_ACTION_DELETED_RECIPE_STEP_INGREDIENT_DRAFT: &str =
    "ENTITY_ACTION_DELETED_RECIPE_STEP_INGREDIENT_DRAFT";

pub const ENTITY_ACTION_CREATED_RECIPE_STEP: &str = "ENTITY_ACTION_CREATED_RECIPE_STEP";
pub const ENTITY_ACTION_UPDATED_RECIPE_STEP: &str = "ENTITY_ACTION_UPDATED_RECIPE_STEP";
pub const ENTITY_ACTION_DELETED_RECIPE_STEP: &str = "ENTITY_ACTION_DELETED_RECIPE_STEP";

pub const ENTITY_ACTION_CREATED_UNIT_NAME: &str = "ENTITY_ACTION_CREATED_UNIT_NAME";
pub const ENTITY_ACTION_UPDATED_UNIT_NAME: &str = "ENTITY_ACTION_UPDATED_UNIT_NAME";
pub const ENTITY_ACTION_DELETED_UNIT_NAME: &str = "ENTITY_ACTION_DELETED_UNIT_NAME";

pub const SCRAPER_DOM_DROP: &str = "SCRAPER_DOM_DROP";
pub const SCRAPER_ELEMENT_DROP: &str = "SCRAPER_ELEMENT_DROP";
