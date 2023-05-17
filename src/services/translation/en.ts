import type { TranslationStrings } from "./translations.ts";
import { constructMessageProxy } from "./translations.ts";

const translationStrings: TranslationStrings = {
  labels: {
    actions: {
      create: "Create",
      edit: "Edit",
      update: "Update",
      delete: "Delete",
      cancel: "Cancel",
      confirm: "Confirm",
      add: "Add",
      remove: "remove",
      file: {
        open: "Open",
      },
      ocr: "OCR",
    },
    entityFields: {
      ingredient: {
        name: "Ingredient Name",
      },
      recipe: {
        name: "Recipe Name",
      },
      recipeFile: {
        name: "Name",
        path: "File",
      },
      recipeIngredient: {
        quantity: "Quantity",
        unit: "Unit",
        ingredient: "Ingredient",
        quality: "Quality",
      },
      recipeStep: {
        description: "Description",
      },
    },
    descriptions: {
      ocrOutput: "OCR output",
    },
  },
  validity: {
    autocomplete: {
      max: "At most {max} {max,plural,one{element needs}other{elements need}} to be selected.",
      min: "At least {min} {min,plural,one{element needs}other{elements need}} to be selected.",
      includesExcluded: "The current selection includes an excluded value.",
    },
  },
  questions: {
    confirmation: "Are you sure?",
  },
  imperatives: {
    selectPlaceholder: "Select {label}",
  },
  headings: {
    recipeStep: "Step {number}",
    ingredients: "Ingredients",
    description: "Description",
    files: "Files",
  },
};

export const messages = constructMessageProxy(translationStrings);
