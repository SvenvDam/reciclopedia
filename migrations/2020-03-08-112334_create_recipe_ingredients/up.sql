CREATE TABLE recipe_ingredients (
  id SERIAL PRIMARY KEY,
  recipe_id SERIAL NOT NULL,
  ingredient_id SERIAL NOT NULL,
  qty VARCHAR,
  FOREIGN KEY (recipe_id) REFERENCES recipes (id),
  FOREIGN KEY (ingredient_id) REFERENCES ingredients (id)
)