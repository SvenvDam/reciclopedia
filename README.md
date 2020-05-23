# Reciclopedia

[![Build Status](https://travis-ci.org/SvenvDam/reciclopedia.svg?branch=master)](https://travis-ci.org/SvenvDam/reciclopedia)

A simple app to store and retrieve recipes.

## Check it out
[reciclopedia.herokuapp.com](https://reciclopedia.herokuapp.com)

## Example queries

### Create recipe
```
mutation {
  createRecipe(
    recipe: {
      name: "Panzanella",
      ingredients: [
        { name: "tomato" },
        { name: "bread" },
        { name: "olive oil" },
        { name: "garlic" },
        { name: "basil" },
        { name: "vinegar" },
        { name: "mustard" },
      ]
    }
  ) {
    name,
    ingredients {
      name
    }
  }
}
```

### Query by one ingredient
```
query {
  recipesByIngredient(name: "tomato") {
    name
  }
}
```

### Query by multiple ingredients
```
query {
  recipesByIngredients(names: ["tomato", "bread"]) {
    name
  }
}
```

### Delete recipe
```
mutation {
  deleteRecipe(name: "panzanella")
}
```